#!/usr/bin/env bash
set -e
cargo build
function build_variations() {
  local target=$1
  cargo build -p "${target}"
  cargo build -p "${target}" --features libc
  cargo build -p "${target}" --features mimalloc
  cargo build -p "${target}" --features jemalloc
}
build_variations axum-tws-example
build_variations tungstenite-example
build_variations axum-example