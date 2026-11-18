#!/usr/bin/env bash
# Maps the container user to the host user (HOST_UID/HOST_GID, see .env.local / devops/scripts/setup.sh)
# so files created in bind mounts (target/, var/) are owned by the host user.
set -euo pipefail

HOST_UID="${HOST_UID:-1000}"
HOST_GID="${HOST_GID:-1000}"

# Already running as a non-root user (rootless docker, `user:` override).
if [ "$(id -u)" != "0" ]; then
    exec "$@"
fi

# Unprivileged perf profiling (no-op without --privileged).
sysctl -w kernel.perf_event_paranoid=-1 >/dev/null 2>&1 || true

if ! getent group "$HOST_GID" >/dev/null; then
    groupadd -g "$HOST_GID" hostgroup
fi

if getent passwd "$HOST_UID" >/dev/null; then
    DEV_USER="$(getent passwd "$HOST_UID" | cut -d: -f1)"
else
    useradd --create-home --uid "$HOST_UID" --gid "$HOST_GID" hostuser
    DEV_USER="hostuser"
fi

if [ "$(id -g "$DEV_USER")" != "$HOST_GID" ]; then
    usermod -g "$HOST_GID" "$DEV_USER"
fi

# Toolchain homes are pre-chowned to rust (1000:1000) at build time;
# fix them only when the host uid/gid differs. The registry mount point is
# a host bind mount that dockerd may have created as root (when the host
# path didn't exist yet), so it needs fixing regardless of the toolchain
# home ownership.
if [ "$(stat -c '%u:%g' /usr/local/cargo/registry)" != "$HOST_UID:$HOST_GID" ]; then
    chown -R "$HOST_UID:$HOST_GID" /usr/local/cargo/registry
fi
if [ "$(stat -c '%u:%g' /usr/local/cargo)" != "$HOST_UID:$HOST_GID" ]; then
    chown -R "$HOST_UID:$HOST_GID" /usr/local/cargo /usr/local/rustup
fi

DEV_HOME="$(getent passwd "$DEV_USER" | cut -d: -f6)"

if [ "$#" -eq 0 ]; then
    set -- bash
fi

exec gosu "$DEV_USER" env HOME="$DEV_HOME" USER="$DEV_USER" LOGNAME="$DEV_USER" "$@"
