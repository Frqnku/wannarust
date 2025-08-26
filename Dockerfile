FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl \
    && apt-get update && apt-get install -y musl-tools build-essential pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY wannarust ./wannarust
WORKDIR /app/wannarust

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

COPY infection home/infection
COPY --from=builder /app/wannarust/target/x86_64-unknown-linux-musl/release/wannarust .

RUN chmod +x wannarust

