#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PACKAGES=(
  univis_ui_style
  univis_ui_engine
  univis_ui_interaction
  univis_ui_widgets
  univis_ui
)

echo "== Cargo check =="
cargo check --workspace

echo
echo "== Library tests =="
for PACKAGE in "${PACKAGES[@]}"; do
  ./scripts/test_lib_serial_release.sh -p "$PACKAGE"
done

echo
echo "== Example checks =="
./scripts/check_examples_serial_release.sh -p univis_ui

echo
echo "== Package rehearsal =="
cargo package -p univis_ui_style --allow-dirty --offline --no-verify
./scripts/package_alpha_serial.sh --list-only --offline \
  univis_ui_engine \
  univis_ui_interaction \
  univis_ui_widgets \
  univis_ui

echo
echo "Alpha release verification completed successfully."
