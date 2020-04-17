FROM rust:latest AS builder
WORKDIR /app/
# See .dockerignore
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update \
    && apt-get install --no-install-recommends --yes \
    apt-utils \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/dawaclient /usr/local/bin/dawaclient
