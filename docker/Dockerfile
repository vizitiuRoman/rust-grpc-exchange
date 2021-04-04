# select build image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin rust-grpc-exchange

WORKDIR /rust-grpc-exchange

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

RUN cargo clean

# build for release
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /rust-grpc-exchange/target/release/rust-grpc-exchange .

# set the startup command to run your binary
CMD ["./rust-grpc-exchange"]
EXPOSE 8001
