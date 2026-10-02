#!/usr/bin/env bash

set -ex

docker compose run --rm --build --workdir /code/examples/api-app builder cargo build --release

docker compose --env-file .env --env-file .env.local build release.api
