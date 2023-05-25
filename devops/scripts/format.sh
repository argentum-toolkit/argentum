#!/usr/bin/env bash

#set -ex

CURRENT_DIR=$(pwd)

for f in ./*/Cargo.toml ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  cargo fmt --all --manifest-path $(dirname "$f")/Cargo.toml -v
done
