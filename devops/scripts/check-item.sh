#!/usr/bin/env bash

set -ex

ITEM_DIR=$1
CURRENT_DIR=$(pwd)

cd $ITEM_DIR

cargo build
cargo clippy
cargo fmt --all
#cargo fmt --all -- --check
cargo test
#cargo publish --dry-run

cd $CURRENT_DIR
