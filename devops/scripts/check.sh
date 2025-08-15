#!/usr/bin/env bash

set -ex

rustc -V && cargo -V

SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
scriptDir="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"


cargo deny check licenses
cargo deny check bans
# cargo deny check advisories
cargo deny check sources

cargo check
cargo clippy
cargo fmt --all -- --check
cargo test
#cargo publish --dry-run

