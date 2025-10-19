# rust-engine/Dockerfile

# --- Stage 1: Builder ---
# Use a stable Rust version
FROM rust:1.82-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies needed for sqlx
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the ENTIRE project at once. This ensures Cargo sees a valid project.
COPY . .

# Build the project. Cargo will resolve dependencies and compile.
RUN cargo build --release

# --- Stage 2: Final, small image ---
FROM debian:bookworm-slim
# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine

EXPOSE 8000
CMD ["rust-engine"]