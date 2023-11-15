FROM rust:bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin cli
WORKDIR /cli

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/cli*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /cli/target/release/cli .

# set the startup command to run your binary
CMD ["./cli"]