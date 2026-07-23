FROM rust:1.94

RUN rustup component add clippy rustfmt
RUN cargo install --locked cargo-deny

RUN cargo install flamegraph rustfilt

RUN apt-get update \
    && apt-get install -y \
        iputils-ping \
        linux-perf \
        llvm \
        lldb \
    && rm -rf /var/lib/apt/lists/*


# Add additional perf privileges
RUN echo "kernel.perf_event_paranoid = -1" >>/etc/sysctl.conf
