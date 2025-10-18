# rust-engine/Dockerfile
# --- Stage 1: Builder ---
FROM rust:1.7-slim as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# --- Stage 2: Final Image ---
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust-engine /usr/local/bin/rust-engine
EXPOSE 8000
CMD ["rust-engine"]