#!/usr/bin/env bash
set -e
set -x
docker buildx build --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-websockets -f docker/ws.Dockerfile . --push