#! /bin/bash

set -e

export WASM32_TARGET=wasm32-unknown-emscripten
export WASM32_ROOT_DIR=target/${WASM32_TARGET}/debug

echo "Compiling TypeScript..."

node_modules/.bin/tsc

echo "Building for ${WASM32_TARGET}..."

cargo build --target=${WASM32_TARGET}

cp ${WASM32_ROOT_DIR}/werewolves-and-wanderer.js .
cp ${WASM32_ROOT_DIR}/deps/werewolves_and_wanderer*.wasm .

export ASMJS_TARGET=asmjs-unknown-emscripten
export ASMJS_ROOT_DIR=target/${ASMJS_TARGET}/debug

echo "Building for ${ASMJS_TARGET}..."

cargo build --target=${ASMJS_TARGET}

cp ${ASMJS_ROOT_DIR}/werewolves-and-wanderer.js werewolves-and-wanderer.asm.js
