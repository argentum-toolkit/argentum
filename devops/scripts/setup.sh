#!/usr/bin/env bash

echo "Set up pre-commit"
ln -s -f ../../devops/scripts/check.sh .git/hooks/pre-commit

echo "Create folders"
mkdir -p var/db/user_account/data/

echo "Initialization of Data Base"
docker compose stop
docker compose up -d service.user.db service.user_account.db

#todo: wait-for-it.sh
sleep 10

#TODO: run migrations
#cd argentum_user/infrastructure
#DATABASE_URL=postgres://dev:dev@*:54321/argentum_user diesel migration run
#cd ../..
#
#cd argentum_user_account/infrastructure
#DATABASE_URL=postgres://dev:dev@*:54322/argentum_user_account diesel migration run
#cd ../..

docker compose stop

echo "Database is initialized"

echo "Setup finished"
