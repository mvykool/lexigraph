# Stage 1: build the app
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Dependencies caching
COPY Cargo.toml Cargo.lock 
COPY src/ src/
RUN cargo fetch
RUN cargo build --release

# Stage 2: slim runtime
FROM debian:bullseye-slim

# Install CA certificates for HTTPS & runtime deps
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled app from builder
COPY --from=builder /app/target/release/my-axum-app /usr/local/bin/app

# Set env vars if needed
ENV RUST_LOG=info
WORKDIR /usr/local/bin

# Run the app
CMD ["./app"]
