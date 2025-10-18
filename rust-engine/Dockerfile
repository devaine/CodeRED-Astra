# rust-engine/Dockerfile
# --- Stage 1: Builder ---
FROM rust:1.82-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./
# Create a dummy src/main.rs for dependency build
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm src/main.rs

# Copy source code and build
COPY src ./src
RUN cargo build --release

# --- Stage 2: Final Image ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine
EXPOSE 8000
CMD ["rust-engine"]