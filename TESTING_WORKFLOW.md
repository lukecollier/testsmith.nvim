# Testing & Development Workflow

## Super Simple Development Loop

```bash
# 1. Build
cargo build --release

# 2. Test in Neovim (plugin auto-finds library in target/release/)
nvim

# 3. Repeat: make changes, rebuild, test
```

That's it! ✨

## How It Works

The FFI loader looks for the library in this order:

```
1. System paths (libtestsmith_nvim.dylib)
   ↓
2. target/release/              ← You are here during development
   ↓
3. lib/{platform}/              ← For distribution
   ↓
4. Falls back to CLI subprocess
```

When you run `cargo build --release`, the library goes to `target/release/` and the plugin automatically finds it.

**No manual copying needed!** 🎉

## Development Checklist

- [x] Change Rust code
- [x] Run `cargo build --release`
- [x] Open Neovim and test
- [x] Repeat

## Example

```bash
# Terminal 1: Watch for changes and rebuild
$ cargo watch -x "build --release"

# Terminal 2: Test the plugin
$ nvim

# Make changes to src/generator.rs
# cargo watch automatically rebuilds
# Just reload the plugin in Neovim with :so %
# Changes take effect immediately!
```

## Testing Different Configurations

```bash
# Test with explicit structure
:lua require("testsmith").find_or_create_test({structure = "maven"})

# Test framework auto-detection
:lua require("testsmith").find_or_create_test()

# Check if FFI is available
:lua print(require("testsmith.ffi").is_available())
```

## When You're Ready to Distribute

Once you're happy with your changes:

```bash
make build-release              # Copy from target/release/ to lib/
git add lib/
git commit -m "Release v0.1.0"
git tag v0.1.0
git push --tags                 # GitHub Actions builds other platforms
```

Users then get the pre-built libraries from `lib/`.

## Library Loading Priority

### During Development
✅ `target/release/libtestsmith_nvim.dylib` (found and used)
⚪ `lib/macos-arm64/libtestsmith_nvim.dylib` (ignored, not needed)

### For Users (After Distribution)
⚪ System paths (typically not set)
⚪ `target/release/` (they don't have this)
✅ `lib/macos-arm64/libtestsmith_nvim.dylib` (found and used)
⚪ CLI fallback (available if needed)

## Pro Tips

**Fast iteration with file watcher:**
```bash
cargo watch -x "build --release" -c
```
Automatically rebuilds when you save files (with clear screen).

**Profile a specific test:**
```bash
cargo build --release && time nvim
```
Measure how long it takes to open.

**Debug FFI calls:**
Add `println!()` in Rust, they appear in Neovim messages:
```rust
println!("DEBUG: structure = {}", structure);
```

**Use CLI directly:**
```bash
cargo build --release
./target/release/testsmith-nvim ~/Projects/MyFile.java
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| FFI not loading in Neovim | Check `:lua print(require("testsmith.ffi").is_available())` |
| Changes not taking effect | Reload plugin: `:lua package.loaded.testsmith = nil` |
| Library not found | Make sure `cargo build --release` completed successfully |
| Wrong Rust code being tested | Did you rebuild? `cargo build --release` again |

## Full Workflow Summary

```
┌─────────────────────────────────────────────────┐
│ DEVELOPMENT                                     │
├─────────────────────────────────────────────────┤
│ $ cargo build --release                         │
│ → Creates target/release/libtestsmith_nvim.* │
│                                                 │
│ $ nvim                                          │
│ → FFI loader finds target/release/              │
│ → Tests run with latest Rust code              │
│                                                 │
│ Make changes → Rebuild → Test (repeat)          │
└─────────────────────────────────────────────────┘
                       ↓
┌─────────────────────────────────────────────────┐
│ DISTRIBUTION                                    │
├─────────────────────────────────────────────────┤
│ $ make build-release                            │
│ → Copies target/release/* to lib/{platform}/   │
│                                                 │
│ $ git add lib/ && git push                      │
│ → Users download plugin with bundled libraries │
└─────────────────────────────────────────────────┘
```

## See Also

- `DEVELOPMENT.md` - Full development guide
- `QUICK_START.md` - For distribution
- `MAKE` or `make help` - Build commands
