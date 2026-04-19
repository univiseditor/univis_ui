#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "Checking the standalone Android phone package..."
cargo check --manifest-path android/android_phone_app/Cargo.toml --all-targets

echo
echo "Checking any currently shipped workspace examples..."
./scripts/check_examples_serial_release.sh
