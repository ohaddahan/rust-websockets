FROM ohaddahan/base-alpine-rust-websockets:latest
ARG BUILDPLATFORM
ARG TARGETPLATFORM
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"