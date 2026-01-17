# Build & Distribution Quick Start

## TL;DR - One Command Build

```bash
make build-release
```

This builds the Rust code and automatically copies the FFI library to `lib/` for distribution.

## What Happened?

```
cargo build --release           (1) Compiles testsmith for your platform
                                ↓
./scripts/copy-libs.sh          (2) Copies the dylib/so to lib/{platform}/
                                ↓
lib/macos-arm64/                ✅ Library is now ready for distribution
libtestsmith_nvim.dylib
```

## For Different Platforms

| Platform | Command | Output |
|----------|---------|--------|
| macOS arm64 | `make build-release` | `lib/macos-arm64/libtestsmith_nvim.dylib` |
| macOS x86_64 | `make build-release` | `lib/macos-x86_64/libtestsmith_nvim.dylib` |
| Linux x86_64 | `make build-release` | `lib/linux-x86_64/libtestsmith_nvim.so` |
| Windows x86_64 | `make build-release` | `lib/windows-x86_64/testsmith_nvim.dll` |

## Common Tasks

### Development (Debug Build)
```bash
cargo build                 # No library copy (saves time)
nvim                        # Falls back to CLI subprocess
```

### Release (Build + Distribute)
```bash
make build-release          # Build + copy library
git add lib/
git commit -m "Update library"
git push
```

### Just Copy (After Manual Build)
```bash
cargo build --release       # Just the build
make copy-libs              # Just the copy
```

### Clean Everything
```bash
make clean                  # Removes all build artifacts
```

### See All Commands
```bash
make help
```

## Lua Plugin Behavior

1. **FFI tries to load library from `lib/` first**
   - If found: Fast FFI calls ⚡
   - If not found: Falls back to CLI subprocess ✅

2. **Plugin works either way**
   - With library: Direct Rust calls (fastest)
   - Without library: CLI subprocess (still works)

## Distribution

Users installing testsmith.nvim get:
- ✅ Lua plugin code
- ✅ Pre-built FFI library (for their platform)
- ✅ Zero compilation needed!

The `lib/` directory is included in the plugin, so everything "just works."

## Why Two Steps?

Cargo's `build.rs` runs BEFORE linking completes, so we can't use it for post-build operations. The explicit two-step approach is:
- ✅ Reliable (runs after build finishes)
- ✅ Fast (only copies current platform)
- ✅ Clear (no hidden magic)

See `BUILD_DESIGN.md` for details.

## Automated CI/CD

Want all platforms built automatically? Push a tag:

```bash
git tag v0.1.0
git push --tags
# GitHub Actions builds for all platforms
# Libraries are auto-committed back to repo
```

See `.github/workflows/build-libraries.yml`

---

**That's it!** Use `make build-release` and you're done. 🚀
