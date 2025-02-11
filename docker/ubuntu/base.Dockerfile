FROM ubuntu:24.04
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install curl gzip git-all -y
RUN apt-get install build-essential -y
RUN apt-get install libc6 -y
RUN apt-get install -y pkg-config openssl libssl-dev
RUN apt-get install -y --no-install-recommends openssl ca-certificates