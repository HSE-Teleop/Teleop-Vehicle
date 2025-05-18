FROM rust:1.86-slim-bullseye AS builder

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates build-essential \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release --bin subscriber --bin publisher --bin provider

FROM debian:bullseye-slim AS runtime-base

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

FROM runtime-base AS subscriber
COPY --from=builder /usr/src/app/target/release/subscriber /usr/local/bin/subscriber
ENTRYPOINT ["subscriber"]

FROM runtime-base AS publisher
COPY --from=builder /usr/src/app/target/release/publisher  /usr/local/bin/publisher
ENTRYPOINT ["publisher"]

FROM runtime-base AS provider
COPY --from=builder /usr/src/app/target/release/provider  /usr/local/bin/provider
ENTRYPOINT ["provider"]
