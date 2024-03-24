FROM lukemathwalker/cargo-chef:latest-rust-1.77.0 AS chef
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
# openssl - it is dynamically linked by some dependencies
# ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
        && apt-get install -y --no-install-recommends openssl ca-certificates \
        # Clean up
        && apt-get autoremove -y \
        && apt-get clean -y \
        && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY config config
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
