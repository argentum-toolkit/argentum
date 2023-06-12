#!/usr/bin/env bash

# run cargo fmt
for f in ./*/Cargo.toml ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  cargo fmt --all --manifest-path $(dirname "$f")/Cargo.toml -v
done
