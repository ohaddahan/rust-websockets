# Build

docker buildx build --platform=linux/arm64,linux/amd64 -t rust-websockets -f docker/ws.Dockerfile .

# Interactive

docker run -it -t blabla /bin/bash

# Run

docker run -e NAME="axum-tws-example" -t blabla
