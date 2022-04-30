FROM lukemathwalker/cargo-chef:latest-rust-1.60.0 AS chef
WORKDIR app

# Generate a manifest of workspace (cargo) dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build only the dependencies -- for caching
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --workspace --recipe-path recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --workspace --bins
# TODO in prod, RUN cargo build --release --workspace --bins
