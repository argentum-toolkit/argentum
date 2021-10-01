#!/usr/bin/env bash

echo "Set up pre-commit"
ln -s -f ../../devops/scripts/check.sh .git/hooks/pre-commit

echo "Create folders"
mkdir -p var/db/user_account/data/

echo "Setup finished"
