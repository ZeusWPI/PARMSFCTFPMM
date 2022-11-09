# BUILD

FROM rust:latest AS build

RUN rustup default nightly

RUN USER=root cargo new --vcs none --bin prolog
WORKDIR /prolog

COPY ./.cargo ./.cargo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo +nightly build --release --bin prolog
RUN rm ./src/*
RUN rm ./target/release/deps/prolog*

COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo +nightly build --release --bin prolog

# RUN

FROM debian:stable-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update \
	&& apt install -y libsqlite3-0 \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /prolog

COPY --from=build /prolog/target/release/prolog .
COPY ./static ./static
COPY ./templates ./templates

CMD [ "./prolog" ]
