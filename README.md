# Time Tracking App

This project is a Time Tracking application built with a Rust backend and a Vue.js frontend.

## Tech Stack

*   **Frontend:** Vue.js (Vite), Bootstrap 5
*   **Backend:** Rust, Axum
*   **Database:** SQLite (managed by Diesel ORM), designed to be easily migratable to PostgreSQL.
*   **Authentication:** JWT (Access + Refresh Tokens), Argon2 password hashing.

## Prerequisites

To replicate this setup on a Linux (Ubuntu) machine, you need the following installed:

### 1. Rust

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Node.js

Install Node.js (e.g., using `nvm` or system package manager). Long-Term Support (LTS) version recommended.

```bash
# Example using curl
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs
```

### 3. Diesel CLI

Install the Diesel CLI tool for managing the database. Since we are using SQLite initially, we can install it with only SQLite support to avoid needing libpq/mysql client libraries on the system for now.

```bash
# Install SQLite development libraries if not present
sudo apt-get install libsqlite3-dev

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features sqlite
```

## Project Setup

1.  **Clone the repository.**
2.  **Setup Backend:**
    ```bash
    cd backend
    # Create .env file
    echo "DATABASE_URL=database.sqlite" > .env
    echo "JWT_SECRET=your_secret_key_change_me" >> .env

    # Run database migrations
    diesel setup
    diesel migration run

    # Run server
    cargo run
    ```
3.  **Setup Frontend:**
    ```bash
    cd frontend
    npm install
    npm run dev
    ```

## Development

*   The Backend runs on `http://localhost:3000`.
*   The Frontend runs on `http://localhost:5173` (default Vite port).

## Production Build

To build the entire application for production:

1.  **Build Frontend:**
    ```bash
    cd frontend
    npm run build
    ```
    This creates a `dist/` folder.

2.  **Run Backend:**
    The backend is configured to serve static files from `../frontend/dist`.
    ```bash
    cd backend
    cargo run --release
    ```
