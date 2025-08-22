# Build stage
FROM rust:1.82-slim-bookworm AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf src
# Copy source code and build
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/axum-postgres-example /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
