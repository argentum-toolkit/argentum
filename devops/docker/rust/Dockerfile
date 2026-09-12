FROM rust:1.96 AS rs

RUN rustup component add clippy rustfmt
RUN cargo install cargo-binstall
RUN cargo binstall cargo-deny dioxus-cli flamegraph rustfilt

RUN apt-get update \
    && apt-get install -y \
        iputils-ping \
        linux-perf \
        llvm \
        lldb \
    && rm -rf /var/lib/apt/lists/*

# Add additional perf privileges
RUN echo "kernel.perf_event_paranoid = -1" >>/etc/sysctl.conf
