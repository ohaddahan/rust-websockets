#!/usr/bin/env bash
set -e
function docker_pull() {
  docker pull ohaddahan/base-rust-websockets
  docker pull ohaddahan/rust-rust-websockets
  docker pull ohaddahan/rust-websockets
}

function build_base() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/base-rust-websockets -f docker/ws-base.Dockerfile . --push
}

function build_rust() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-rust-websockets -f docker/ws-rust.Dockerfile . --push
}

function build_ws() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-websockets -f docker/ws.Dockerfile . --push
}

if [ -z "${1}" ] ; then
  echo "Missing argument"
else
  set -x
  "${1}"
fi
