#!/usr/bin/env bash
set -ex
# need to be installed https://github.com/killercup/cargo-edit

VERSION=$1
# run cargo fmt
for f in ./*/Cargo.toml ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  cargo set-version --manifest-path $(dirname "$f")/Cargo.toml $VERSION
done
