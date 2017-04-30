#! /bin/bash

set -eu

export OUTPUT_DIR=static
export WASM32_TARGET=wasm32-unknown-emscripten
export WASM32_ROOT_DIR=target/${WASM32_TARGET}/debug

echo "Cleaning out ${OUTPUT_DIR}/..."

rm -f ${OUTPUT_DIR}/*.wasm
rm -f ${OUTPUT_DIR}/*.js

echo "Compiling TypeScript..."

node_modules/.bin/tsc
cp node_modules/promise-polyfill/promise.js ${OUTPUT_DIR}

echo "Building for ${WASM32_TARGET}..."

cargo build --target=${WASM32_TARGET}

cp ${WASM32_ROOT_DIR}/werewolves-and-wanderer.js ${OUTPUT_DIR}
cp ${WASM32_ROOT_DIR}/deps/werewolves_and_wanderer*.wasm ${OUTPUT_DIR}

export ASMJS_TARGET=asmjs-unknown-emscripten
export ASMJS_ROOT_DIR=target/${ASMJS_TARGET}/debug

echo "Building for ${ASMJS_TARGET}..."

cargo build --target=${ASMJS_TARGET}

cp ${ASMJS_ROOT_DIR}/werewolves-and-wanderer.js ${OUTPUT_DIR}/werewolves-and-wanderer.asm.js
