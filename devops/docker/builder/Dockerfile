FROM rust:1.70

RUN rustup component add clippy rustfmt

# Firstly you have to install libpq
RUN cargo install diesel_cli --no-default-features --features postgres
