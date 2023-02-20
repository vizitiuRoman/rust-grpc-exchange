FROM debian:buster-slim AS base
RUN apt-get update \
    && apt-get install curl openssl ca-certificates -y --no-install-recommends \
        libpq5 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM rust:1.58.0-buster AS build
WORKDIR /usr/src/app
RUN USER=root cargo init
RUN rustup install nightly
RUN apt-get update \
    && apt-get install cmake -y
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN rustup run nightly cargo build --release --bin server
COPY ./src ./src
# https://users.rust-lang.org/t/dockerfile-with-cached-dependencies-does-not-recompile-the-main-rs-file/21577
RUN touch src/main.rs && rustup run nightly cargo build --release --bin server

FROM base AS final
WORKDIR /app
COPY --from=build /usr/src/app/target/release/server .
ENV RUST_BACKTRACE=full
ENTRYPOINT ["/app/server"]
