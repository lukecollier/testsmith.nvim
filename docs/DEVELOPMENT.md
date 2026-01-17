# Development Guide

## Quick Development Workflow

### 1. Build for Testing
```bash
cargo build --release
```

The FFI loader automatically finds the library in `target/release/` and uses it for testing.

### 2. Test in Neovim
```bash
nvim
# Press <leader>tf to test testsmith
```

### 3. No Extra Steps Needed
The plugin automatically picks up your latest build from `target/release/`. Just rebuild when you make changes.

## Library Loading Order

The FFI loader tries to find the library in this order:

1. **System library paths** (best for installed versions)
   - `libtestsmith_nvim.dylib` on macOS
   - `libtestsmith_nvim.so` on Linux
   - `testsmith_nvim.dll` on Windows

2. **Development build directory** (during active development)
   - `target/release/libtestsmith_nvim.dylib`
   - `target/release/libtestsmith_nvim.so`
   - etc.

3. **Bundled lib directory** (for distribution)
   - `lib/macos-arm64/libtestsmith_nvim.dylib`
   - `lib/linux-x86_64/libtestsmith_nvim.so`
   - etc.

4. **Falls back to CLI subprocess** if no library found

## Development vs Distribution

### During Development
```bash
cargo build --release
nvim  # Uses target/release/libtestsmith_nvim.dylib
```
- No extra copy steps needed
- Changes take effect after rebuild
- Fastest workflow

### For Distribution
```bash
make build-release           # Build + copy to lib/
git add lib/
git push                     # Users get bundled library
```
- Libraries copied to `lib/{platform}/`
- Users get pre-built, zero-compilation plugin
- Portable across plugin managers

## File Organization

```
testsmith.nvim/
├── src/                              # Rust source code
├── target/
│   └── release/
│       ├── libtestsmith_nvim.dylib   ← Used during development
│       ├── testsmith-nvim            ← CLI binary
│       └── ...
├── lib/
│   └── macos-arm64/
│       └── libtestsmith_nvim.dylib   ← Used for distribution
└── lua/testsmith/
    └── ffi.lua                       ← Tries all locations
```

## Testing Changes

```bash
# Make Rust code changes
vim src/generator.rs

# Rebuild
cargo build --release

# Test immediately (uses target/release/)
nvim
```

No intermediate steps needed. The plugin automatically uses your latest build.

## Common Tasks

### Debug Build (Development)
```bash
cargo build                # Faster, but no FFI
nvim                       # Falls back to CLI
```

### Release Build (Testing FFI)
```bash
cargo build --release      # Slower, optimized
nvim                       # Uses target/release/
```

### Copy to Distribution
```bash
make build-release         # Build + copy to lib/
git add lib/
```

### Clean Everything
```bash
cargo clean                # Removes target/
make clean                 # Also removes lib/
```

### See Build Commands
```bash
make help
```

## Troubleshooting

**Plugin not using my changes**
- Did you run `cargo build --release`?
- Neovim might be caching - try `:lua package.loaded.testsmith = nil` then reopen

**FFI not loading**
- Run `:lua print(require("testsmith.ffi").is_available())`
- Should print `true` if library loaded
- Plugin falls back to CLI if `false`

**Wrong library being used**
- FFI loader tries: system → `target/release/` → `lib/` → CLI
- Check which one exists: `ls -la target/release/libtestsmith_nvim.dylib lib/*/`

**Symbol not found error**
- Make sure you're building for your architecture
- `cargo build --release` should auto-detect
- If cross-compiling, specify: `cargo build --release --target aarch64-apple-darwin`

## Development Tips

- **Live reload**: After `cargo build --release`, just reload the plugin in Neovim
- **Debug logging**: Add `println!()` in Rust, they show in Neovim messages
- **Binary available too**: `target/release/testsmith-nvim` can be used as CLI
- **Fast iteration**: Release build takes ~12s first time, ~0.1s for incremental

## Next: Distribution

When ready to distribute:
```bash
make build-release                # Prepare release
git add lib/
git commit -m "Release v0.1.0"
git tag v0.1.0
git push --tags                   # GitHub Actions builds all platforms
```

See `QUICK_START.md` for one-command building.
