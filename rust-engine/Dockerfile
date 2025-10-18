# rust-engine/Dockerfile
# --- Stage 1: Builder ---
FROM rust:1.82-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy ONLY Cargo.toml to resolve dependencies
COPY Cargo.toml ./

# KEY CHANGE: Generate lock file and fetch dependencies inside the container
RUN cargo update && cargo fetch

# Now copy the rest of your source code
COPY src ./src

# Build the project using the pre-fetched dependencies
RUN cargo build --release

# --- Stage 2: Final Image ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine
EXPOSE 8000
CMD ["rust-engine"]