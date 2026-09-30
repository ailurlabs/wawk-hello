#!/usr/bin/env bash
# Build script for wawk-hello plugin.
# Runs tests, then compiles to a WASM component (wasm32-wasip1).
#
# The ecosystem convention is `cargo component build --target wasm32-wasip1`:
# the artifact is a WASM *component* that `wac plug` can compose into
# wawk-wasi / wawk-bindgen / wawk-mcp. A plain `cargo build --target
# wasm32-unknown-unknown` core module cannot be plugged, so do not use it.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> Running tests..."
cargo test --quiet

echo "==> Building component (wasm32-wasip1, release)..."
cargo component build --release --target wasm32-wasip1

WASM="target/wasm32-wasip1/release/wawk_hello.wasm"

# Copy to plugin root
cp "$WASM" ./wawk-hello.wasm

echo "==> Built: $SCRIPT_DIR/wawk-hello.wasm"
echo "    Size: $(du -h wawk-hello.wasm | cut -f1)"

# Show the component world if wasm-tools is available
if command -v wasm-tools &>/dev/null; then
    echo "==> Component world:"
    wasm-tools component wit "$WASM" 2>/dev/null | sed 's/^/    /'
fi
