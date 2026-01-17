# Build System Design

## Why Not Use build.rs for Library Copying?

Cargo's `build.rs` script runs **before** the linker completes, which means:
- ❌ The compiled library doesn't exist yet
- ❌ We can't reliably copy it to `lib/`
- ❌ Different Cargo versions put intermediate files in different locations

## Solution: Explicit Post-Build Copy

We use explicit commands that run **after** linking completes:

```bash
cargo build --release    # Build the library
./scripts/copy-libs.sh   # Copy it to lib/ (runs after build)
```

## Recommended Workflows

### Option 1: Using Makefile (Simplest) ⭐
```bash
make build-release
# Automatically builds AND copies library
```

### Option 2: Manual Commands
```bash
cargo build --release && ./scripts/copy-libs.sh
```

### Option 3: Script Wrapper
```bash
./scripts/build-release.sh
```

## Why This Approach?

✅ **Clear** - Explicit two-step process (build → copy)
✅ **Reliable** - Runs after library is fully built
✅ **Fast** - Only copies the current platform's library
✅ **Cross-platform** - Works on macOS, Linux, Windows
✅ **Simple** - No Cargo build script magic

## For CI/CD (GitHub Actions)

The workflow automatically:
1. Builds for all platforms (on respective CI runners)
2. Runs `copy-libs.sh` for each platform
3. Commits all libraries back to repo
4. Users get pre-built libraries

See `.github/workflows/build-libraries.yml`

## Supported Targets

When you build on different machines:

| Machine | Command | Result |
|---------|---------|--------|
| Apple Silicon Mac | `make build-release` | `lib/macos-arm64/` |
| Intel Mac | `make build-release` | `lib/macos-x86_64/` |
| Linux x86_64 | `make build-release` | `lib/linux-x86_64/` |
| Windows | `make build-release` | `lib/windows-x86_64/` |

## Multi-Platform Distribution

To build for multiple platforms:

1. **On each machine**, run: `make build-release`
2. **Commit the results**: `git add lib/`
3. **Or use GitHub Actions** (automatic when you push tags)

Result: `lib/` contains dylibs/so/dlls for all platforms
