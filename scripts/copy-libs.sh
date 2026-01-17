#!/bin/bash
# Copy compiled FFI libraries to lib/ directory for distribution
# Run this after: cargo build --release

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

# Detect platform and architecture
UNAME_S=$(uname -s)
UNAME_M=$(uname -m)

# Determine target triple and library name
if [ "$UNAME_S" = "Darwin" ]; then
    if [ "$UNAME_M" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
        PLATFORM="macos-arm64"
        LIB_NAME="libtestsmith_nvim.dylib"
    else
        TARGET="x86_64-apple-darwin"
        PLATFORM="macos-x86_64"
        LIB_NAME="libtestsmith_nvim.dylib"
    fi
elif [ "$UNAME_S" = "Linux" ]; then
    if [ "$UNAME_M" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
        PLATFORM="linux-arm64"
    else
        TARGET="x86_64-unknown-linux-gnu"
        PLATFORM="linux-x86_64"
    fi
    LIB_NAME="libtestsmith_nvim.so"
elif [[ "$UNAME_S" == MINGW* ]] || [[ "$UNAME_S" == MSYS* ]]; then
    TARGET="x86_64-pc-windows-msvc"
    PLATFORM="windows-x86_64"
    LIB_NAME="testsmith_nvim.dll"
else
    echo "❌ Unsupported platform: $UNAME_S"
    exit 1
fi

echo "🔨 Copying $LIB_NAME for $PLATFORM..."

# Check if library exists in release build
LIB_SOURCE="target/release/$LIB_NAME"
LIB_ALT_SOURCE="target/$TARGET/release/$LIB_NAME"

if [ ! -f "$LIB_SOURCE" ] && [ ! -f "$LIB_ALT_SOURCE" ]; then
    echo "❌ Library not found!"
    echo "   Expected: $LIB_SOURCE or $LIB_ALT_SOURCE"
    echo "   Did you run 'cargo build --release'?"
    exit 1
fi

# Use whichever exists
if [ -f "$LIB_SOURCE" ]; then
    LIB_PATH="$LIB_SOURCE"
else
    LIB_PATH="$LIB_ALT_SOURCE"
fi

# Create lib directory
mkdir -p "lib/$PLATFORM"

# Copy library
cp "$LIB_PATH" "lib/$PLATFORM/$LIB_NAME"
echo "✅ Copied $LIB_PATH → lib/$PLATFORM/$LIB_NAME"

# Verify
if [ -f "lib/$PLATFORM/$LIB_NAME" ]; then
    SIZE=$(stat -f%z "lib/$PLATFORM/$LIB_NAME" 2>/dev/null || stat -c%s "lib/$PLATFORM/$LIB_NAME")
    echo "✅ Verified! Size: $SIZE bytes"
else
    echo "❌ Copy verification failed!"
    exit 1
fi
