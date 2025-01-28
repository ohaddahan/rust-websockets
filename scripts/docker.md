# Build

docker buildx build --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-websockets -f docker/ws.Dockerfile .
docker buildx build --platform=linux/arm64,linux/amd64 -t ohaddahan/rust-websockets -f docker/ws.Dockerfile . --push

# Interactive

docker run -it -t ohaddahan/rust-websockets /bin/bash

# Run

docker run -p 8000:8000 -e NAME="axum-tws-example" -t ohaddahan/rust-websockets
docker run -p 8000:8000 -e NAME="client" -t ohaddahan/rust-websockets
