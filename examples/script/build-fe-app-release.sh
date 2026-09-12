#!/usr/bin/env bash

set -ex

docker compose run --rm --build --workdir /code/examples/fe-app builder dx bundle --web --release
# docker compose run --rm --build --workdir /code/examples/fe-app builder dx bundle --web

docker compose --env-file .env --env-file .env.local build release.fe
