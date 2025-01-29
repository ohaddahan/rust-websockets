FROM --platform=$BUILDPLATFORM ubuntu:24.04 AS base
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
ARG DEBIAN_FRONTEND=noninteractive
ARG RUSTC_VERSION=1.82.0
RUN apt-get update
RUN apt-get install curl gzip git-all -y
RUN apt-get install build-essential -y
RUN apt-get install libc6 -y
RUN apt-get install -y pkg-config openssl libssl-dev
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install $RUSTC_VERSION
RUN rustup default $RUSTC_VERSION
RUN apt-get install -y --no-install-recommends openssl ca-certificates
FROM base AS builder
RUN git clone https://github.com/ohaddahan/rust-websockets.git \
    && cd rust-websockets \
    && cargo build --features jemalloc
FROM builder AS runner
#CMD ["/rust-websockets/target/axum-example"]
ENV NAME="axum-example"
CMD ["/bin/bash", "-c", "/rust-websockets/target/debug/${ARGS:-${NAME}}"]
