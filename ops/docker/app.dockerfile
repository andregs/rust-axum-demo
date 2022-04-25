# Leveraging the pre-built Docker images with cargo-chef and the Rust toolchain
# see https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM lukemathwalker/cargo-chef:latest-rust-1.60.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin axum_demo

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/axum_demo /usr/local/bin
ENTRYPOINT ["/usr/local/bin/axum_demo"]
