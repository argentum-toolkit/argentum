#!/usr/bin/env bash

set -e

echo "Set up pre-commit"
ln -s -f ../../devops/scripts/check.sh .git/hooks/pre-commit

# Ensure the local (gitignored) env file exists: it selects the DEV compose file
if [ ! -f .env.local ]; then
    echo "Create .env.local"
    cat > .env.local <<'EOF'
# Local overrides and secrets. Selects the DEV compose file.
# See .env for the full list of variables.
COMPOSE_FILE=docker-compose.dev.yaml
EOF
fi

compose() {
    docker compose --env-file .env --env-file .env.local "$@"
}

echo "Create certificates for HTTP/2"
mkdir -p var/cert/api
cd var/cert/api
mkcert -install
mkcert localhost 127.0.0.1 ::1 api
cd -

echo "Create folders"
compose stop
mkdir -p var/db/user/
mkdir -p var/db/user_account/
mkdir -p var/db/oidc/

echo "Initialization of Data Base"
compose up -d service.user.db service.user_account.db service.oidc.db

#todo: wait-for-it.sh
sleep 10

### Migrations
compose run --rm api cargo run --bin migration

compose stop

echo "Database is initialized"

echo "Setup finished"
