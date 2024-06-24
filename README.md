# GBA Playground

## Building

```bash
rustup component add rust-src
cargo build --release
```

## Run

Requirements:
 - mGBA: Install mGBA with your package manager or download it from https://mgba.io/downloads.html

```
cargo run
```
## Running tests

```bash
cargo test --lib --target=x86_64-unknown-linux-gnu
```

## Logger
messages can be logged with function `log()`:
```rust
log(MgbaMessageLevel::Debug, "Hello world!")
```

The logs can be displayed in the mGBA emulator in the log window: `Tools > View Logs...`
