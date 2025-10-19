# rust-engine/Dockerfile

# --- Stage 1: Builder ---
# Use a stable Rust version
FROM rust:1.82-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies needed for sqlx
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Allow override of toolchain (stable, nightly, or a pinned version)
ARG RUSTUP_TOOLCHAIN=stable

# Explicit HOME to avoid undefined variable during Docker build
ENV HOME=/root

# Install rustup and ensure the selected toolchain is installed (fixes edition mismatches in CI)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup toolchain install ${RUSTUP_TOOLCHAIN} \
    && /root/.cargo/bin/rustup default ${RUSTUP_TOOLCHAIN}

ENV PATH="/root/.cargo/bin:${PATH}"

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