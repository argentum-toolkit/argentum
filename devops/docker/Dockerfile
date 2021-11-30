FROM rust:1.56.1

RUN rustup component add clippy rustfmt

# Firstly you have to install libpq
RUN cargo install diesel_cli --no-default-features --features postgres
