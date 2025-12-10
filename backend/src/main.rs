use axum::{
    routing::{get, post},
    Router, Json, Extension,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::sync::Arc;
use tower_http::{
    cors::{CorsLayer, Any},
    services::{ServeDir, ServeFile},
};
use axum::routing::get_service;
use dotenvy::dotenv;

mod schema;
mod models;
mod auth;

use self::models::{User, NewUser};
use self::schema::users::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
struct AppState {
    pool: DbPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let state = AppState { pool };

    let app = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/refresh", post(refresh))
        .route("/api/me", get(me))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .route("/vite.svg", get_service(ServeFile::new("../frontend/dist/vite.svg")))
        .fallback_service(ServeFile::new("../frontend/dist/index.html"))
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut conn = state.pool.get().expect("couldn't get db connection from pool");

    let existing_user = users
        .filter(email.eq(&payload.email))
        .first::<User>(&mut conn)
        .optional();

    match existing_user {
        Ok(Some(_)) => return (StatusCode::CONFLICT, "User already exists").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
        Ok(None) => {}
    }

    let hashed_password = match auth::hash_password(&payload.password) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed").into_response(),
    };

    let new_user = NewUser {
        email: &payload.email,
        password_hash: &hashed_password,
    };

    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn);

    match result {
        Ok(_) => (StatusCode::CREATED, "User created successfully").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let mut conn = state.pool.get().expect("couldn't get db connection from pool");

    let user_result = users
        .filter(email.eq(&payload.email))
        .first::<User>(&mut conn)
        .optional();

    let user = match user_result {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    if !auth::verify_password(&payload.password, &user.password_hash) {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    // Generate tokens
    // Access token: 15 minutes (900 seconds)
    // Refresh token: 7 days (604800 seconds) - In a real app we might store this in DB to allow revocation
    let access_token = match auth::create_jwt(&user.email, 900) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed").into_response(),
    };

    let refresh_token = match auth::create_jwt(&user.email, 604800) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed").into_response(),
    };

    Json(LoginResponse {
        access_token,
        refresh_token,
    }).into_response()
}

async fn me(claims: auth::Claims) -> impl IntoResponse {
    Json(claims)
}

#[derive(serde::Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(serde::Serialize)]
struct RefreshResponse {
    access_token: String,
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> impl IntoResponse {
    let claims = match auth::verify_jwt(&payload.refresh_token) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid refresh token").into_response(),
    };

    let mut conn = state.pool.get().expect("couldn't get db connection from pool");
    let user_exists = users
        .filter(email.eq(&claims.sub))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0) > 0;

    if !user_exists {
        return (StatusCode::UNAUTHORIZED, "User no longer exists").into_response();
    }

    let access_token = match auth::create_jwt(&claims.sub, 900) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed").into_response(),
    };

    Json(RefreshResponse { access_token }).into_response()
}
