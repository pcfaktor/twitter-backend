FROM rust:slim

WORKDIR /app

RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo install cargo-watch

CMD ["cargo", "watch", "--why", "-x", "run"]