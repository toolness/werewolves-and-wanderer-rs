#! /bin/bash

set -e

echo "Building for current platform..."

cargo build

export WASM=wasm32-unknown-emscripten

echo "Building for ${WASM}..."

cargo build --target=${WASM}
