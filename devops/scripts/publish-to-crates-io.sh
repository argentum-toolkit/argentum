#!/usr/bin/env bash

#TODO: build dependency tree and order publishing publish by dependency tree
for f in ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  cargo publish --manifest-path $(dirname "$f")/Cargo.toml

  #avoid 409 status
  sleep 10
done
