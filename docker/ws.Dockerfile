FROM --platform=$BUILDPLATFORM ohaddahan/rust-rust-websockets:latest
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
RUN git clone https://github.com/ohaddahan/rust-websockets.git \
    && cd rust-websockets \
    && cargo build --features jemalloc
ENV NAME="axum-tws-example"
CMD ["/bin/bash", "-c", "/rust-websockets/target/debug/${ARGS:-${NAME}}"]