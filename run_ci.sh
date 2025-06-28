#!/bin/sh
set -e

# Ensure required components are installed (ignore errors if no network)
rustup component add rust-src || true
rustup component add rustfmt || true

# Build GBA binaries
cargo build --release --out-dir=out -Z unstable-options

# Run tests on host
cargo test --lib --target=x86_64-unknown-linux-gnu --release

# Check formatting if rustfmt is available
if rustup component list --installed | grep -q '^rustfmt'; then
  cargo fmt --check
else
  echo "Skipping cargo fmt -- rustfmt not installed"
fi
