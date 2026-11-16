#!/usr/bin/env bash

set -ex

docker compose run --rm --build --workdir /code/examples/api-app builder cargo build --release

docker compose build release.api
