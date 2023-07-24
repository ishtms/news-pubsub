# Build
FROM rust:1.71.0 AS builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y && apt install libssl-dev \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/news_pubsub news_pubsub
COPY config config
ENV APP_ENVIRONMENT prod
ENTRYPOINT ["./news_pubsub"]