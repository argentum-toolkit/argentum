#!/usr/bin/env bash

DIR="$(cd "$(dirname "$0")" && pwd)"

$DIR/build-api-app-release.sh
$DIR/build-fe-app-release.sh
