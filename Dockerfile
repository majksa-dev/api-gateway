# Rust base
FROM rust:1-slim-bullseye as base
RUN apt-get update && apt-get install -y \
    pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
WORKDIR /app

# Rust planner
FROM base AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# Build app
FROM base as app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY src ./src
RUN cargo build --release
RUN cp ./target/release/api-gateway ./bin/server

# Application
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /app
WORKDIR /app
COPY --from=app /app/bin/server /app/server
ENTRYPOINT [ "bash", "-c" ]
CMD "/app/server"
HEALTHCHECK --interval=5s --timeout=5s --start-period=5s --retries=5 CMD [ "wget", "-q", "-O", "-", "http://localhost:$${HEALTHCHECK_PORT:-9000}" ]