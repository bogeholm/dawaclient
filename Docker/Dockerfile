FROM rust:latest AS builder
WORKDIR /usr/src/dawaclient/
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install --no-install-recommends --yes apt-utils ca-certificates libssl-dev
COPY --from=builder /usr/local/cargo/bin/dawaclient /usr/local/bin/dawaclient
