# syntax=docker/dockerfile:1.7
# rust-engine/Dockerfile

# --- Stage 1: Builder ---
# Use a stable Rust version
FROM rust:1.85-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies needed for sqlx
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


# Allow optional override of toolchain (e.g., nightly or a pinned version). Leave empty to use image default.
ARG RUSTUP_TOOLCHAIN=

# Use rustup and cargo from the official Rust image location
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Copy manifest files first to leverage Docker layer caching for dependencies
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Ensure the pinned toolchain from rust-toolchain.toml (or provided ARG) is installed in a cacheable layer
RUN set -eux; \
        if [ -n "${RUSTUP_TOOLCHAIN}" ]; then \
            rustup toolchain install "${RUSTUP_TOOLCHAIN}" && \
            rustup default "${RUSTUP_TOOLCHAIN}"; \
        else \
            if [ -f rust-toolchain.toml ]; then \
                TOOLCHAIN=$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml | head -n1); \
                if [ -n "$TOOLCHAIN" ]; then \
                    rustup toolchain install "$TOOLCHAIN" && \
                    rustup default "$TOOLCHAIN"; \
                fi; \
            fi; \
        fi; \
        rustup show active-toolchain || true

# Create a dummy src to allow cargo to download dependencies into the cache layer
RUN mkdir -p src && echo "fn main() { println!(\"cargo cache build\"); }" > src/main.rs

# Fetch and build dependencies (this will be cached until Cargo.toml changes)
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release || true


# Now copy the real source and build the final binary
COPY src ./src
# Only remove the dummy main.rs if it exists and is not the real one
RUN if grep -q 'cargo cache build' src/main.rs 2>/dev/null; then rm src/main.rs; fi
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release

# --- Stage 2: Final, small image ---

FROM debian:bookworm-slim
# Install only necessary runtime dependencies (no upgrade, just ca-certificates)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Add a non-root user for security
RUN useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin appuser

# Copy the compiled binary from the builder stage

COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine
RUN chown appuser:appuser /usr/local/bin/rust-engine \
    && mkdir -p /var/log \
    && touch /var/log/astra-errors.log \
    && chown appuser:appuser /var/log/astra-errors.log

EXPOSE 8000
USER appuser
# Redirect all output to /var/log/astra-errors.log for easy monitoring
ENTRYPOINT ["/bin/sh", "-c", "/usr/local/bin/rust-engine >> /var/log/astra-errors.log 2>&1"]