# Automated Library Building

The project provides a simple workflow to build and copy FFI libraries to the `lib/` directory.

## How It Works

1. **Build the release binary:** `cargo build --release`
2. **Copy the library:** `./scripts/copy-libs.sh`

The copy script:
- Detects your platform (macOS arm64/x86_64, Linux, Windows)
- Finds the compiled library in `target/{target}/release/`
- Copies it to `lib/{platform}/` with the correct platform-specific name:
  - `lib/macos-arm64/libtestsmith_nvim.dylib`
  - `lib/macos-x86_64/libtestsmith_nvim.dylib`
  - `lib/linux-x86_64/libtestsmith_nvim.so`
  - `lib/windows-x86_64/testsmith_nvim.dll`
  - `lib/linux-arm64/libtestsmith_nvim.so`

## Usage

### Quick Release Build + Copy ✅
```bash
cargo build --release && ./scripts/copy-libs.sh
# Output:
# 🔨 Copying libtestsmith_nvim.dylib for macos-arm64...
# ✅ Copied target/release/libtestsmith_nvim.dylib → lib/macos-arm64/libtestsmith_nvim.dylib
# ✅ Verified! Size: 2235392 bytes
```

### Or Step by Step
```bash
# Step 1: Build release
cargo build --release

# Step 2: Copy library
./scripts/copy-libs.sh
```

### Debug Build (Skip Copy)
```bash
cargo build
# Library is not copied (saves time during development)
```

## Cross-Compilation

When building for a different target:

```bash
# Build for Intel Mac from Apple Silicon
cargo build --release --target x86_64-apple-darwin
./scripts/copy-libs.sh  # Will copy to lib/macos-x86_64/

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu
./scripts/copy-libs.sh  # Will copy to lib/linux-x86_64/
```

## Supported Targets

| Target | Platform | Output |
|--------|----------|--------|
| `aarch64-apple-darwin` | macOS arm64 | `lib/macos-arm64/libtestsmith_nvim.dylib` |
| `x86_64-apple-darwin` | macOS x86_64 | `lib/macos-x86_64/libtestsmith_nvim.dylib` |
| `x86_64-unknown-linux-gnu` | Linux x86_64 | `lib/linux-x86_64/libtestsmith_nvim.so` |
| `x86_64-pc-windows-msvc` | Windows x86_64 | `lib/windows-x86_64/testsmith_nvim.dll` |
| `aarch64-unknown-linux-gnu` | Linux ARM64 | `lib/linux-arm64/libtestsmith_nvim.so` |

## Advantages

✅ **Simple** - Just run two commands: `cargo build --release && ./scripts/copy-libs.sh`
✅ **Clear** - No build.rs magic, explicit copy operation
✅ **Cross-platform** - Works with all supported architectures
✅ **Distribution ready** - Libraries are always in `lib/` for distribution
✅ **Reliable** - Happens after build completes (not before linking)

## Implementation Details

The `scripts/copy-libs.sh` script:
- Auto-detects platform and architecture using `uname`
- Creates `lib/{platform}/` directories automatically
- Handles both native and cross-compilation scenarios
- Provides clear output with emoji indicators (🔨, ✅, ❌)
- Verifies the copy succeeded and shows file size

## GitHub Actions Integration

The `.github/workflows/build-libraries.yml` workflow:
1. Builds for all supported platforms when you push a tag
2. Each platform build runs `scripts/copy-libs.sh` automatically
3. All libraries are committed back to the repository
4. Users download complete plugin with all pre-built libraries

## Example Workflow

```bash
# Make changes to Rust code
vim src/generator.rs

# Build and copy library
cargo build --release && ./scripts/copy-libs.sh
# ✅ lib/macos-arm64/libtestsmith_nvim.dylib is updated

# Test in Neovim
nvim

# Ready to distribute!
git add lib/
git commit -m "Update FFI library"
git push
```
