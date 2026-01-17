# Distribution Setup - Option 1: Bundled Libraries

This project now uses **bundled pre-compiled libraries** for the Lua FFI. This means users get the FFI performance benefits without needing to compile Rust code themselves.

## Current Status

✅ **macOS arm64** - Pre-built and included
⏳ **macOS x86_64** - Needs to be built on Intel Mac or via CI/CD
⏳ **Linux x86_64** - Needs to be built on Linux or via CI/CD
⏳ **Windows x86_64** - Needs to be built on Windows or via CI/CD

## Quick Start for Developers

### macOS arm64 (Apple Silicon) - Your Current Platform

The library is already built and in place at:
```
lib/macos-arm64/libtestsmith_nvim.dylib
```

### Other Platforms

Use the automated GitHub Actions workflow. Just push a git tag:

```bash
git add lib/macos-arm64/libtestsmith_nvim.dylib
git commit -m "Add pre-built dylib for macOS arm64"
git tag v0.1.0
git push origin main --tags
```

This automatically:
1. Builds for all platforms (macOS arm64/x86_64, Linux x86_64, Windows x86_64)
2. Creates artifacts
3. Commits them back to the repository

## Directory Structure

```
testsmith.nvim/
├── lua/
│   └── testsmith/
│       ├── init.lua           # Main plugin
│       └── ffi.lua            # FFI loader (now smart platform detection)
├── lib/                       # Pre-compiled binaries (INCLUDED IN DISTRIBUTION)
│   ├── macos-arm64/
│   │   └── libtestsmith_nvim.dylib
│   ├── macos-x86_64/
│   │   └── libtestsmith_nvim.dylib
│   ├── linux-x86_64/
│   │   └── libtestsmith_nvim.so
│   └── windows-x86_64/
│       └── testsmith_nvim.dll
├── scripts/
│   └── build-all.sh           # Cross-compile all platforms locally
├── .github/workflows/
│   └── build-libraries.yml    # Automated CI/CD
├── Cargo.toml
├── src/
└── BUILD.md                   # Build instructions
```

## How It Works

1. **User installs plugin** via plugin manager (vim-plug, lazy.nvim, etc.)
2. **Plugin includes lib/ directory** with all pre-built binaries
3. **FFI loader detects platform:**
   ```lua
   -- lua/testsmith/ffi.lua
   local platform, lib_name = get_platform_and_arch()
   local lib_path = plugin_dir .. "/lib/" .. platform .. "/" .. lib_name
   ```
4. **Loads appropriate library** - zero compilation needed
5. **Falls back to CLI** if library not available

## Distribution Checklist

- [x] Bundled libraries for quick setup
- [x] Smart platform detection in Lua
- [x] CLI fallback for unsupported platforms
- [x] GitHub Actions automated builds
- [x] Build script for manual compilation
- [ ] Other platforms built and committed to `lib/`

## Next Steps

1. **For macOS x86_64**:
   - Build on Intel Mac: `scripts/build-all.sh`
   - Or wait for GitHub Actions when we tag a release

2. **For Linux/Windows**:
   - Use GitHub Actions (automatic when tagged)
   - Or use `cross` tool locally

3. **Update Homebrew formula** (when ready to distribute):
   ```ruby
   # Formula/testsmith-nvim.rb
   def install
     system "cargo", "build", "--release"
     bin.install "target/release/testsmith-nvim"
     # Also install the dylib for FFI
     lib.install "lib/macos-arm64/libtestsmith_nvim.dylib"
   end
   ```

## Testing

Users can verify FFI is working:

```vim
:lua print(require("testsmith.ffi").is_available())
```

If it returns `false`, the CLI subprocess is being used instead (still functional, just no FFI performance).

## Advantages of This Approach

✅ Works with any plugin manager
✅ No compilation needed for end users
✅ FFI performance on supported platforms
✅ Automatic fallback to CLI
✅ Clean distribution package
✅ Supports multiple architectures

## Fallback Behavior

If library loading fails, the plugin automatically uses the CLI subprocess:
```lua
-- Falls back to vim.fn.jobstart(binary_path)
-- Same functionality, no FFI performance boost
```
