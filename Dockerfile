# syntax=docker/dockerfile:1.6

FROM rust:1.86-slim-bullseye AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# minimal target so `cargo metadata` works
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.86-slim-bullseye AS builder
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      ca-certificates clang lld pkg-config libssl-dev \
      build-essential autoconf automake libtool m4 zlib1g-dev \
 && rm -rf /var/lib/apt/lists/*
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
RUN cargo install sccache --locked
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV PATH="/usr/local/cargo/bin:${PATH}"

# bring cargo-chef into this stage
COPY --from=chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef

WORKDIR /app
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release

# real sources
COPY . .
# build all bins (or list them explicitly with --bin if you prefer)
RUN cargo build --release --bins

# ---- runtime images ----
FROM debian:bullseye-slim AS runtime-base
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

FROM runtime-base AS subscriber
COPY --from=builder /app/target/release/subscriber /usr/local/bin/subscriber
ENTRYPOINT ["subscriber"]

FROM runtime-base AS publisher
COPY --from=builder /app/target/release/publisher /usr/local/bin/publisher
ENTRYPOINT ["publisher"]

FROM runtime-base AS provider
COPY --from=builder /app/target/release/provider /usr/local/bin/provider
ENTRYPOINT ["provider"]
