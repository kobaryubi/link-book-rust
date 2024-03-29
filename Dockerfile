FROM rust:1.76
WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install apt-transport-https

RUN rustup component add rustfmt
RUN cargo install cargo-watch
