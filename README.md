# Twitter Backend

A minimal Rust backend for a Twitter-like app built with Axum and SQLx.

## Quickstart

This project uses Docker Compose for development. The backend runs inside a container and applies database migrations automatically at startup, so no local PostgreSQL or manual migration steps are required.

1. Ensure Docker and Docker Compose are installed.
2. Copy the example environment and edit if needed:

```bash
cp .env.example .env
# edit .env to adjust credentials if necessary
```

3. Build and start the services (database + backend). The backend will run migrations on startup:

```bash
docker compose up --build -d
```

4. The backend runs in development mode (dev image includes `sqlx-cli` and `cargo-watch`) and listens on port `3000`:

```text
http://localhost:3000
```

## Tech stack
- Axum
- SQLx
- Tokio
- PostgreSQL
