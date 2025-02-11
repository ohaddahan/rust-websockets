#!/usr/bin/env bash
set -e
function docker_pull() {
  docker pull ohaddahan/base-rust-websockets
  docker pull ohaddahan/rust-rust-websockets
  docker pull ohaddahan/rust-websockets
  docker pull ohaddahan/base-alpine-rust-websockets
  docker pull ohaddahan/rust-rust-alpine-websockets
  docker pull ohaddahan/rust-alpine-websockets
}

function build_alpine_base() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/base-alpine-rust-websockets -f docker/alpine/base.Dockerfile . --push
}

function build_alpine_rust() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-rust-alpine-websockets -f docker/alpine/rust.Dockerfile . --push
}

function build_alpine_ws() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-alpine-websockets -f docker/alpine/ws.Dockerfile . --push
}


function build_base() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/base-rust-websockets -f docker/ubuntu/ws-base.Dockerfile . --push
}

function build_rust() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-rust-websockets -f docker/ubuntu/ws-rust.Dockerfile . --push
}

function build_ws() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-websockets -f docker/ubuntu/ws.Dockerfile . --push
}

function build_server() {
  docker_pull
  docker buildx build --no-cache --platform=linux/arm64,linux/amd64 -t ohaddahan/server-websockets -f docker/server.Dockerfile . --push
}

if [ -z "${1}" ] ; then
  echo "Missing argument"
else
  set -x
  "${1}"
fi
