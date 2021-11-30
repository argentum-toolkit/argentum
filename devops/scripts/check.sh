#!/usr/bin/env bash

set -ex

packages=(
  'argentum_encryption/business'
  'argentum_encryption/infrastructure'
  'argentum_event/business'
  'argentum_log/business'
  'argentum_log/infrastructure'
  'argentum_notification/business'
  'argentum_notification/infrastructure'
  'argentum_standard/business'
  'argentum_standard/infrastructure'
  'argentum_user/business'
  'argentum_user/infrastructure'
  'argentum_user_account/business'
  'argentum_user_account/infrastructure'
  'demo-app'
  'demo-web-app'
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
