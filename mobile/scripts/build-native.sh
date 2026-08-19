#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT/mobile"

command -v cargo-ndk >/dev/null || { echo "缺少 cargo-ndk：cargo install cargo-ndk" >&2; exit 1; }
rustup target list --installed | grep -q '^aarch64-linux-android$' || {
  echo "缺少 Rust target：rustup target add aarch64-linux-android" >&2
  exit 1
}

# Always invoke helpers through bash: Git checkouts may not preserve executable bits.
bash ./scripts/copy-assets.sh

# cargo-ndk 4.x expects --manifest-path before the Cargo subcommand.
cargo ndk \
  -t arm64-v8a \
  -o app/src/main/jniLibs \
  --manifest-path core/Cargo.toml \
  build --release
