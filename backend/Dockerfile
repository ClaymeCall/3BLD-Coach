# Dev image: build context is the repo root (see docker-compose.yml)
FROM rust:latest
RUN cargo install cargo-watch
WORKDIR /app
# Pre-copy manifests/sources so the first start is warm; volumes override these at runtime
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY backend ./backend
ENV CARGO_TARGET_DIR=/target
EXPOSE 8080
