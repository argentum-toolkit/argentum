#!/usr/bin/env bash

set -e

echo "Set up pre-commit"
ln -s -f ../../devops/scripts/check.sh .git/hooks/pre-commit

echo "Create folders"
mkdir -p var/db/user_account/data/

echo "Initialization of Data Base"
docker compose stop
docker compose up -d service.user.db service.user_account.db

#todo: wait-for-it.sh
sleep 10

### Migrations
cd demo-web-app
cargo run --bin migration
cd ..

docker compose stop

echo "Database is initialized"

echo "Setup finished"
