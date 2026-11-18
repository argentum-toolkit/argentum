FROM rust:1.100 AS rs

RUN rustup component add clippy rustfmt
# Pre-install the wasm target so `dx bundle --web` doesn't reinstall it on every run
RUN rustup target add wasm32-unknown-unknown
RUN curl -sL https://github.com/cargo-bins/cargo-binstall/releases/download/v1.27.0/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xz -C /usr/local/cargo/bin
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

RUN useradd --create-home --uid 1000 --user-group rust \
    && chown -R rust:rust /usr/local/cargo /usr/local/rustup

COPY devops/docker/rust/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
