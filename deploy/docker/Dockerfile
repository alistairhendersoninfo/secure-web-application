# syntax=docker/dockerfile:1

FROM rust:1-bullseye AS builder
WORKDIR /src
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.toml
COPY crates/controller/Cargo.toml crates/controller/Cargo.toml
COPY crates/agent/Cargo.toml crates/agent/Cargo.toml
COPY crates/shared/Cargo.toml crates/shared/Cargo.toml
COPY proto proto
RUN mkdir -p crates/controller/src crates/agent/src crates/shared/src && \
    echo "fn main(){}" > crates/controller/src/main.rs && \
    echo "fn main(){}" > crates/agent/src/main.rs && \
    echo "" > crates/shared/src/lib.rs && \
    cargo build --release --target x86_64-unknown-linux-musl || true
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl -p controller

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/controller /app/controller
USER nonroot
EXPOSE 10443
ENTRYPOINT ["/app/controller"]
