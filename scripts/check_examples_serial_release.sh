#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

JOBS="${CARGO_BUILD_JOBS:-1}"

if [ "$#" -gt 0 ]; then
  EXAMPLES=("$@")
else
  mapfile -t EXAMPLES < <(find examples -maxdepth 1 -type f -name '*.rs' -printf '%f\n' | sed 's/\.rs$//' | sort)
fi

COUNT="${#EXAMPLES[@]}"
if [ "$COUNT" -eq 0 ]; then
  echo "No examples found."
  exit 0
fi

echo "Checking $COUNT examples sequentially (release mode)..."
INDEX=0
for EXAMPLE_NAME in "${EXAMPLES[@]}"; do
  INDEX=$((INDEX + 1))
  echo "[$INDEX/$COUNT] cargo check --release --example $EXAMPLE_NAME"
  CARGO_BUILD_JOBS="$JOBS" cargo check --release --example "$EXAMPLE_NAME"
done

echo "All examples passed cargo check in release mode."
