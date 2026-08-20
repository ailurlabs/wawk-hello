#!/usr/bin/env bash
# Build script for wawk-hello plugin.
# Runs tests, then compiles to WebAssembly.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> Running tests..."
cargo test --quiet

echo "==> Building wasm32-unknown-unknown (release)..."
cargo build --target wasm32-unknown-unknown --release

WASM="target/wasm32-unknown-unknown/release/wawk_hello.wasm"

# Copy to plugin root
cp "$WASM" ./wawk-hello.wasm

echo "==> Built: $SCRIPT_DIR/wawk-hello.wasm"
echo "    Size: $(du -h wawk-hello.wasm | cut -f1)"

# Show exports if wasm-tools is available
if command -v wasm-tools &>/dev/null; then
    echo "==> Module exports:"
    wasm-tools print "$WASM" 2>/dev/null | grep '(export' | grep -v 'cabi' | sed 's/^/    /'
fi
