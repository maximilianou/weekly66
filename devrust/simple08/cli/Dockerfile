# cli/Dockerfile
# docker build -t myorg/cli .
# cli/Dockerfile
# docker run -p 8888:3000 --rm --name cli myorg/cli
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
#FROM debian:bookworm-slim
FROM debian:bookworm-slim
# copy the build artifact from the build stage
COPY --from=build /cli/target/release/cli . 
RUN chmod +x ./cli
RUN useradd -ms /bin/bash devuser
USER devuser
# set the startup command to run your binary
CMD ["./cli"]