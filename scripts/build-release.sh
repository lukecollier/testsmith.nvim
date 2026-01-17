#!/bin/bash
# Build testsmith for release and automatically copy libraries for all platforms
# Usage: ./scripts/build-release.sh

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

CURRENT_TARGET=$(rustc --version --verbose | grep host | cut -d' ' -f2)
echo "🔨 Current platform: $CURRENT_TARGET"
echo ""

# Build for current platform
echo "📦 Building for $CURRENT_TARGET..."
cargo build --release

# Copy library for current platform
echo ""
echo "📋 Copying library for current platform..."
./scripts/copy-libs.sh

echo ""
echo "✅ Release build and copy complete!"
echo ""
echo "To build for other platforms, run on those systems:"
echo "  - Intel Mac:    cargo build --release && ./scripts/copy-libs.sh"
echo "  - Linux:        cargo build --release && ./scripts/copy-libs.sh"
echo "  - Windows:      cargo build --release && ./scripts/copy-libs.sh"
echo ""
echo "Or use GitHub Actions CI/CD by pushing a tag:"
echo "  git tag v0.1.0"
echo "  git push --tags"
