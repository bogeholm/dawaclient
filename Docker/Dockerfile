FROM rust:latest AS builder
WORKDIR /usr/src/dawaclient/
# See .dockerignore
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install --no-install-recommends --yes apt-utils ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dawaclient /usr/local/bin/dawaclient
