# Build & Testing Guide

## TL;DR

### Development (Testing)
```bash
cargo build --release && nvim
# Plugin auto-finds library in target/release/
# No copying needed!
```

### Distribution
```bash
make build-release && git add lib/ && git push
# Copies library to lib/ for users
# Users get pre-built, zero-compilation plugin
```

---

## Library Loading (Smart Priority)

The FFI loader checks these locations in order:

```
1️⃣  System library paths
    ↓ (not found)
2️⃣  target/release/                    ← Active development here!
    ↓ (not found)
3️⃣  lib/{platform}/                    ← For distribution
    ↓ (not found)
4️⃣  CLI subprocess fallback             ← Always works
```

### During Development
- You: `cargo build --release`
- Result: Library in `target/release/`
- Plugin: Finds it automatically ✅
- No copying needed!

### For Users
- You: `make build-release && git push`
- Result: Library in `lib/{platform}/`
- Plugin: Uses bundled version ✅
- Works out of the box!

---

## Workflows

### 1. Development & Testing

```bash
# Build (creates target/release/libtestsmith_nvim.dylib)
cargo build --release

# Test in Neovim (auto-finds in target/release/)
nvim
# Press <leader>tf to test

# Make changes to src/
vim src/generator.rs

# Rebuild (fast, incremental)
cargo build --release

# Test again (no extra steps!)
nvim
```

**Key: No intermediate copying needed during development!**

### 2. Prepare for Distribution

```bash
# Build + copy to lib/
make build-release

# Verify
ls -la lib/macos-arm64/

# Commit
git add lib/
git commit -m "Update library"
git push
```

### 3. Multi-Platform (GitHub Actions)

```bash
# On your machine
make build-release
git add lib/
git commit -m "Build for this platform"
git push

# Then push tag
git tag v0.1.0
git push --tags
# GitHub Actions auto-builds other platforms
```

---

## File Reference

| File | Purpose | When Used |
|------|---------|-----------|
| `target/release/libtestsmith_nvim.dylib` | Development build | During active coding |
| `lib/macos-arm64/libtestsmith_nvim.dylib` | Distribution build | For plugin release |
| `Makefile` | Build commands | `make build-release` |
| `scripts/copy-libs.sh` | Platform detection + copy | Deployment workflow |
| `lua/testsmith/ffi.lua` | Library loader | Auto-finds & loads |

---

## Commands Quick Reference

```bash
# Development
cargo build --release           # Build for testing
cargo build                     # Debug (slower testing)
cargo clean                     # Clean all builds

# Distribution
make build-release              # Build + copy to lib/
make copy-libs                  # Just copy (after build)
make help                       # See all make targets

# Testing
nvim                            # Test plugin
make clean && make build-release # Clean + full rebuild
```

---

## Supported Platforms

| Target | Detected | Library |
|--------|----------|---------|
| macOS arm64 (Apple Silicon) | ✅ Auto | `lib/macos-arm64/libtestsmith_nvim.dylib` |
| macOS x86_64 (Intel) | ✅ Auto | `lib/macos-x86_64/libtestsmith_nvim.dylib` |
| Linux x86_64 | ✅ Auto | `lib/linux-x86_64/libtestsmith_nvim.so` |
| Linux ARM64 | ✅ Auto | `lib/linux-arm64/libtestsmith_nvim.so` |
| Windows x86_64 | ✅ Auto | `lib/windows-x86_64/testsmith_nvim.dll` |

---

## FAQ

**Q: Why don't I need to copy during development?**
A: The FFI loader checks `target/release/` after building, so your latest code is always available.

**Q: When do I need to use `make build-release`?**
A: Only when preparing for distribution. It copies to `lib/` so users get pre-built libraries.

**Q: Do users need to build Rust?**
A: No! The `lib/` directory has pre-built libraries. They get zero-compilation plugin.

**Q: What if FFI fails?**
A: Plugin automatically falls back to CLI subprocess. Everything still works!

**Q: How do I test different architectures?**
A: Either:
1. Use a real machine (Apple Silicon, Intel Mac, Linux, Windows)
2. Or use GitHub Actions (push a tag)

**Q: Can I use system-installed library?**
A: Yes! FFI loader tries system paths first, then development, then bundled.

---

## Documentation Files

Start with:
1. **This file** - You're reading it! 📖
2. **TESTING_WORKFLOW.md** - Development loop
3. **DEVELOPMENT.md** - Detailed dev guide
4. **QUICK_START.md** - For distribution

Others:
- **BUILD_SUMMARY.md** - Overview
- **BUILD_DESIGN.md** - Technical details
- **BUILD_AUTOMATION.md** - Copy script info
- **DISTRIBUTION.md** - Distribution strategy

---

## Summary

✨ **Development is simple:** `cargo build --release && nvim`

📦 **Distribution is simple:** `make build-release && git push`

🚀 **Users get:** Pre-built plugin, zero compilation

---

**Ready?** Start with `cargo build --release` and test your changes! 🎉
