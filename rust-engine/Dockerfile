# syntax=docker/dockerfile:1.7
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

# Use rustup and cargo from the official Rust image location
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Ensure the selected toolchain is installed and set as default (fixes edition mismatches in CI)
RUN /usr/local/cargo/bin/rustup toolchain install ${RUSTUP_TOOLCHAIN} \
    && /usr/local/cargo/bin/rustup default ${RUSTUP_TOOLCHAIN}

# Copy manifest files first to leverage Docker layer caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to allow cargo to download dependencies into the cache layer
RUN mkdir -p src && echo "fn main() { println!(\"cargo cache build\"); }" > src/main.rs

# Fetch and build dependencies (this will be cached until Cargo.toml changes)
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/usr/src/app/target,sharing=locked \
    cargo build --release || true


# Now copy the real source and build the final binary
COPY src ./src
# Only remove the dummy main.rs if it exists and is not the real one
RUN if grep -q 'cargo cache build' src/main.rs 2>/dev/null; then rm src/main.rs; fi
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/usr/src/app/target,sharing=locked \
    cargo build --release

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