#!/bin/bash

# Build script for WASM module
# Usage: ./build-wasm.sh [--dev|--release]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RUST_DIR="$PROJECT_ROOT/rust/engine"
OUT_DIR="$PROJECT_ROOT/web/src/wasm/pkg"

# Default to dev build
MODE="${1:---dev}"

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Determine build target
if [ "$MODE" = "--release" ]; then
    TARGET="--release"
    echo "Building WASM in release mode..."
else
    TARGET="--dev"
    echo "Building WASM in dev mode..."
fi

cd "$RUST_DIR"

# Build with wasm-pack
wasm-pack build --target web --out-dir "$OUT_DIR" $TARGET

echo "WASM build complete! Output: $OUT_DIR"
