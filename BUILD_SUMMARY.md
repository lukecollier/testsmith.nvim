# Build System Summary

## Quick Start

```bash
# Build release and automatically copy library
make build-release
```

That's it! 🎉

## Files

- **Makefile** - Simple commands for building
- **scripts/copy-libs.sh** - Detects platform and copies library
- **scripts/build-release.sh** - Builds and shows next steps
- **BUILD_DESIGN.md** - Technical explanation

## How It Works

1. **`make build-release`** runs:
   ```
   cargo build --release     # Compiles for current platform
   ./scripts/copy-libs.sh    # Copies dylib/so to lib/
   ```

2. **The library ends up at:**
   - macOS arm64: `lib/macos-arm64/libtestsmith_nvim.dylib`
   - macOS x86_64: `lib/macos-x86_64/libtestsmith_nvim.dylib`
   - Linux: `lib/linux-x86_64/libtestsmith_nvim.so`
   - Windows: `lib/windows-x86_64/testsmith_nvim.dll`

## Manual Commands

If you prefer not to use Make:

```bash
# Build + copy
cargo build --release && ./scripts/copy-libs.sh

# Just build (library stays in target/)
cargo build --release

# Just copy (after building)
./scripts/copy-libs.sh
```

## Multi-Platform Distribution

To build for multiple platforms:

### Option 1: GitHub Actions (Automated) ✅
```bash
git tag v0.1.0
git push --tags
# GitHub Actions builds for all platforms automatically
```

### Option 2: Manual (Build on each machine)
```bash
# On Apple Silicon Mac
make build-release

# On Intel Mac
make build-release

# On Linux
make build-release

# On Windows
make build-release
```

Then commit all libraries:
```bash
git add lib/
git commit -m "Add pre-built FFI libraries for all platforms"
git push
```

## What Users Get

When someone installs testsmith.nvim:
- ✅ Lua plugin code
- ✅ Pre-built FFI library for their platform
- ✅ No compilation needed!

## Development Workflow

```bash
# Make changes
vim src/generator.rs

# Build and test
make build-release

# Test in Neovim
nvim

# Ready to distribute
git add lib/
git commit -m "Update FFI library"
git push
```

## Troubleshooting

**"❌ Library not found"**
- Did you run `make build-release` (not just `cargo build`)?
- The script looks in `target/release/`

**"Build succeeds but library missing"**
- Run: `./scripts/copy-libs.sh`
- It needs to run AFTER cargo build completes

**Wrong platform library**
- `copy-libs.sh` auto-detects your platform via `uname`
- Run on the correct machine for each platform

## See Also

- `BUILD_AUTOMATION.md` - Detailed copy script info
- `BUILD_DESIGN.md` - Why this design?
- `DISTRIBUTION.md` - Distribution strategy
