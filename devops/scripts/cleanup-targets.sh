#!/usr/bin/env bash

#Remove Rust targets
for f in ./*/Cargo.toml ./*/*/Cargo.toml; do
  rm -Rf $(dirname "$f")/target
done

for f in ./*/*/Cargo.toml; do
  rm -Rf $(dirname "$f")/Cargo.lock
done
