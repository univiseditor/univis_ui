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

echo "== Quality checks =="
./scripts/check_quality.sh

echo
echo "== Release-mode library tests =="
for PACKAGE in "${PACKAGES[@]}"; do
  ./scripts/test_lib_serial_release.sh -p "$PACKAGE"
done

echo
echo "== Representative example checks =="
./scripts/check_representative_examples.sh

echo
echo "== Example checks =="
for PACKAGE in "${PACKAGES[@]}"; do
  ./scripts/check_examples_serial_release.sh -p "$PACKAGE"
done

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
