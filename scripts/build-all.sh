#!/bin/bash
# Build testsmith-nvim for all supported platforms

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "Building testsmith-nvim for all platforms..."

# macOS arm64 (Apple Silicon)
echo "Building for macOS arm64..."
rustup target add aarch64-apple-darwin 2>/dev/null || true
cargo build --release --target aarch64-apple-darwin
mkdir -p lib/macos-arm64
cp target/aarch64-apple-darwin/release/libtestsmith_nvim.dylib lib/macos-arm64/

# macOS x86_64 (Intel)
echo "Building for macOS x86_64..."
rustup target add x86_64-apple-darwin 2>/dev/null || true
cargo build --release --target x86_64-apple-darwin
mkdir -p lib/macos-x86_64
cp target/x86_64-apple-darwin/release/libtestsmith_nvim.dylib lib/macos-x86_64/

# Linux x86_64 (requires cross-compilation tool)
echo "Building for Linux x86_64..."
if command -v cross &> /dev/null; then
  rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true
  cross build --release --target x86_64-unknown-linux-gnu
  mkdir -p lib/linux-x86_64
  cp target/x86_64-unknown-linux-gnu/release/libtestsmith_nvim.so lib/linux-x86_64/
else
  echo "⚠️  'cross' not found. Install with: cargo install cross"
  echo "   Skipping Linux build..."
fi

echo ""
echo "✅ Build complete! Generated libraries:"
find lib -type f -name "libtestsmith*" -o -name "testsmith*"
