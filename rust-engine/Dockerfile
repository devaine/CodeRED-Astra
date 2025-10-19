# syntax=docker/dockerfile:1.7
# rust-engine/Dockerfile

# --- Stage 1: Builder ---
# (No changes needed in this stage)
FROM rust:slim AS builder
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG RUSTUP_TOOLCHAIN=
ENV PATH="/usr/local/cargo/bin:${PATH}"

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

RUN set -eux; \
    if [ -n "${RUSTUP_TOOLCHAIN}" ]; then \
        if ! rustup toolchain list | grep -q "^${RUSTUP_TOOLCHAIN}"; then \
            rustup toolchain install "${RUSTUP_TOOLCHAIN}"; \
        fi; \
        rustup default "${RUSTUP_TOOLCHAIN}"; \
    else \
        if [ -f rust-toolchain.toml ]; then \
            TOOLCHAIN=$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml | head -n1); \
            if [ -n "$TOOLCHAIN" ]; then \
                if ! rustup toolchain list | grep -q "^$TOOLCHAIN"; then \
                    rustup toolchain install "$TOOLCHAIN"; \
                fi; \
                rustup default "$TOOLCHAIN"; \
            fi; \
        fi; \
    fi; \
    rustup show active-toolchain || true

RUN mkdir -p src && echo "fn main() { println!(\"cargo cache build\"); }" > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo fetch

RUN rm -f src/main.rs
COPY src ./src
RUN cargo build --release --locked

# --- Stage 2: Final, small image ---

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin appuser

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine

# --- THIS IS THE FIX ---
# **1. Copy the demo data files from your local machine into the image.**
COPY demo-data /app/demo-data

# **2. Create other directories and set permissions on everything.**
RUN chown appuser:appuser /usr/local/bin/rust-engine \
    && mkdir -p /var/log /app/storage \
    && touch /var/log/astra-errors.log \
    && chown -R appuser:appuser /var/log /app

WORKDIR /app
USER appuser

EXPOSE 8000
ENTRYPOINT ["/bin/sh", "-c", "/usr/local/bin/rust-engine >> /var/log/astra-errors.log 2>&1"]