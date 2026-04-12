#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

REPRESENTATIVE_EXAMPLES=(
  "univis_ui hello_world"
  "univis_ui root_screen_hud"
  "univis_ui root_world_scale"
  "univis_ui root_fit_content"
  "univis_ui_engine border_light_3d"
  "univis_ui_interaction interaction"
  "univis_ui_widgets text_field"
  "univis_ui_widgets panel_window"
)

COUNT="${#REPRESENTATIVE_EXAMPLES[@]}"
echo "Checking $COUNT representative examples sequentially (release mode)..."

for i in "${!REPRESENTATIVE_EXAMPLES[@]}"; do
  read -r package example <<< "${REPRESENTATIVE_EXAMPLES[$i]}"
  index=$((i + 1))
  echo "[$index/$COUNT] $package :: $example"
  ./scripts/check_examples_serial_release.sh -p "$package" "$example"
done

echo "Representative examples passed cargo check in release mode."
