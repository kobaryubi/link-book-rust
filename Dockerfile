FROM rust:1.76
WORKDIR /usr/src/app
RUN rustup component add rustfmt
RUN cargo install cargo-watch
