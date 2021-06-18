#!/usr/bin/env bash

set -ex

packages=(
  'argentum_encryption_business'
  'argentum_encryption_infrastructure'
  'argentum_standard_business'
  'argentum_standard_infrastructure'
  'argentum_user_business'
  'argentum_user_account_business'
  'argentum_user_account_infrastructure'
  'demo-app'
)

SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
scriptDir="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"


for package in ${packages[@]}; do
  echo $package
  source "${scriptDir}/check-item.sh" $package
done
