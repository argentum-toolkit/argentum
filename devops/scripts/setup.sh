#!/usr/bin/env bash

set -e

echo "Set up pre-commit"
ln -s -f ../../devops/scripts/check.sh .git/hooks/pre-commit

echo "Create certificates for HTTP/2"
mkdir -p var/cert/api
cd var/cert/api
mkcert -install
mkcert localhost 127.0.0.1 ::1 api
cd -

echo "Create folders"
docker compose stop
mkdir -p var/db/user/
mkdir -p var/db/user_account/
mkdir -p var/db/keycloak/

echo "Initialization of Data Base"
docker compose up -d service.user.db service.user_account.db service.keycloak.db

#todo: wait-for-it.sh
sleep 10

### Migrations
docker compose run --rm api cargo run --bin migration

docker compose stop

echo "Database is initialized"

echo "Setup finished"
