#! /bin/bash

set -e

echo "Building for current platform..."

cargo build

bash build-emscripten.sh
