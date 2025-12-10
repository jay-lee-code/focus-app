Frontend:
Latest version of Vue.js and bootstrap.js

Backend:
Rust + axum

JWT to handle all authentication
SQLite for database (but I need to make it easy to replace with PostgreSQL later)

Backend is an API that can be potentially used with multiple frontends later on. Backend also serves all static files

My main app idea as of now:
Time tracking app frontend. User can start or stop focus sessions and also record or edit what they have done during the day. Every session is tagged with some category (some of which may be considered productive) that the user can color code

Per user, the backend needs to securely store user focus sessions, tags, and user specific settings.

TODO and concerns:

- Add .gitignore file
- Add install and setup instructions so I can replicate on any Linux (Ubuntu) computer.
- How can I use SQLite to start but make the transition to PostgreSQL later on easily?
- How should I separate the front and backend?
- How can I setup JWT authentication and use short time to live tokens? I do not want to use a blacklist because I do not want to deal with the cache and complexity that involves.
- Setup all of the initial files that I will need.
