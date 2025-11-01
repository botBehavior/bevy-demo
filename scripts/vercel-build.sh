#!/usr/bin/env bash
set -euo pipefail

if ! command -v rustup >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable
fi

source "$HOME/.cargo/env"

rustup target add wasm32-unknown-unknown

if ! command -v trunk >/dev/null 2>&1; then
  cargo install --locked trunk
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  cargo install --locked wasm-bindgen-cli
fi

trunk build --release

mkdir -p dist/assets
if command -v rsync >/dev/null 2>&1; then
  rsync -a assets/ dist/assets/
else
  cp -a assets/. dist/assets/
fi
