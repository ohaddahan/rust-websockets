FROM --platform=$BUILDPLATFORM ohaddahan/base-rust-websockets:latest
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"