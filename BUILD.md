# Building Testsmith Libraries

The Lua plugin uses pre-compiled native libraries (`.dylib` on macOS, `.so` on Linux, `.dll` on Windows) for performance via FFI. These libraries are distributed with the plugin in the `lib/` directory.

## Using Pre-Built Libraries

If you installed testsmith.nvim via a plugin manager, the pre-built libraries are already included and you don't need to do anything.

## Building Locally

If you need to rebuild the libraries yourself:

### Prerequisites

- Rust toolchain (https://rustup.rs)
- For cross-compilation, install `cross`: `cargo install cross`

### Build All Platforms

```bash
cd testsmith.nvim
./scripts/build-all.sh
```

This builds for:
- macOS arm64 (Apple Silicon)
- macOS x86_64 (Intel)
- Linux x86_64
- Windows x86_64

The compiled libraries will be placed in `lib/{platform}/`.

### Build Single Platform

If you only need the current platform:

```bash
cd testsmith.nvim
cargo build --release
```

Then copy to the appropriate lib directory:

**macOS arm64:**
```bash
mkdir -p lib/macos-arm64
cp target/release/libtestsmith_nvim.dylib lib/macos-arm64/
```

**macOS x86_64:**
```bash
mkdir -p lib/macos-x86_64
cp target/release/libtestsmith_nvim.dylib lib/macos-x86_64/
```

**Linux x86_64:**
```bash
mkdir -p lib/linux-x86_64
cp target/release/libtestsmith_nvim.so lib/linux-x86_64/
```

## Development Workflow

When modifying the Rust code:

1. Make changes to `src/`
2. Run `cargo build --release` to compile
3. Copy the result to the appropriate `lib/` subdirectory
4. Reload Neovim (`nvim -u NONE -c "set rtp+=."`  to test)

If the FFI fails to load, it will fall back to the CLI subprocess.

## Automated Builds (CI/CD)

GitHub Actions automatically builds all platforms when you push a git tag:

```bash
git tag v0.2.0
git push --tags
```

This triggers `.github/workflows/build-libraries.yml` which:
1. Builds for all supported platforms
2. Creates artifacts for each platform
3. Commits the compiled libraries back to the repository

## Distribution

Include the entire `lib/` directory when distributing testsmith.nvim:

```
testsmith.nvim/
├── lua/
├── plugin/
├── lib/                    # Include these!
│   ├── macos-arm64/
│   ├── macos-x86_64/
│   ├── linux-x86_64/
│   └── windows-x86_64/
└── ...
```

This ensures the plugin works out-of-the-box without requiring users to build Rust code.

## Troubleshooting

### "Testsmith library not found" error

The FFI loader couldn't find the compiled library. Ensure:
1. The `lib/` directory exists and contains the correct platform subdirectory
2. You're running on a supported platform (macOS, Linux, Windows)
3. The FFI will automatically fall back to CLI subprocess if library loading fails

### Symbol not found errors on macOS

If you see linker errors, ensure you're using the correct target:
- `aarch64-apple-darwin` for Apple Silicon
- `x86_64-apple-darwin` for Intel Macs

Use `rustup target add <target>` to install the target, then rebuild.
