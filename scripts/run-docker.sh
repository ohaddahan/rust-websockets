#!/usr/bin/env bash
set -e

if [ -z "${1}" ] ; then
  echo "Missing arguments PORT | NAME"
  exit 1
fi
if [ -z "${2}" ] ; then
  echo "Missing arguments PORT | NAME"
  exit 1
fi
PORT="${1}"
NAME="${2}"

set -x
docker run -p "${PORT}:${PORT}" -e NAME="${NAME}" -t ohaddahan/rust-websockets