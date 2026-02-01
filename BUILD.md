# Build Instructions

## Prerequisites

- Rust toolchain (with `cargo`)
- `wasm32-wasip2` target installed

## Install WASM Target

```bash
rustup target add wasm32-wasip2
```

## Build Commands

### Development Build

```bash
cargo build --target wasm32-wasip2
# Output: target/wasm32-wasip2/debug/zed_github_actions.wasm
```

### Release Build

```bash
cargo build --target wasm32-wasip2 --release
# Output: target/wasm32-wasip2/release/zed_github_actions.wasm
```

### Check Without Building

```bash
cargo check
```

### Format Code

```bash
cargo fmt
```

## Output

The compiled extension will be at:
- Debug: `target/wasm32-wasip2/debug/zed_github_actions.wasm`
- Release: `target/wasm32-wasip2/release/zed_github_actions.wasm`

## Testing in Zed

1. Build the extension in release mode
2. In Zed, go to Extensions panel
3. Click "Install Dev Extension"
4. Select this repository directory
5. Zed will use the built `extension.wasm` file

## CI/CD

For automated builds, use:

```bash
cargo build --target wasm32-wasip2 --release
```

The WASM file will be ready for distribution.
