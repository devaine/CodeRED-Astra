# rust-engine/Dockerfile

# --- Stage 1: Builder ---
# Use a stable Rust version
FROM rust:stable-bookworm AS builder
WORKDIR /usr/src/app

# Install build dependencies needed for sqlx
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest files first to leverage Docker layer caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to allow cargo to download dependencies into the cache layer
RUN mkdir -p src && echo "fn main() { println!(\"cargo cache build\"); }" > src/main.rs

# Fetch and build dependencies (this will be cached until Cargo.toml changes)
RUN cargo build --release || true

# Now copy the real source and build the final binary
COPY src ./src
RUN rm -f src/main.rs || true
RUN cargo build --release

# --- Stage 2: Final, small image ---
FROM debian:bookworm-slim
# Install only necessary runtime dependencies
RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine

EXPOSE 8000
CMD ["rust-engine"]