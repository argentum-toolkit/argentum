FROM rust:1.100 AS rs

RUN rustup component add clippy rustfmt
RUN cargo install cargo-binstall
RUN cargo binstall cargo-deny dioxus-cli flamegraph rustfilt

RUN apt-get update \
    && apt-get install -y \
        gosu \
        iputils-ping \
        linux-perf \
        llvm \
        lldb \
    && rm -rf /var/lib/apt/lists/*

# Add additional perf privileges
RUN echo "kernel.perf_event_paranoid = -1" >>/etc/sysctl.conf

RUN chown -R rust:rust /usr/local/cargo /usr/local/rustup

COPY devops/docker/rust/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
