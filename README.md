# GBA Playground

## Building

```bash
rustup component add rust-src
cargo build --release
```

## Running tests

```bash
cargo test --lib --target=x86_64-unknown-linux-gnu
```
