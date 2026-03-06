#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

JOBS="${CARGO_BUILD_JOBS:-1}"
PKG_ARGS=()
EXAMPLES_DIR="examples"

if [ "${1:-}" = "-p" ]; then
  if [ -z "${2:-}" ]; then
    echo "Usage: $0 [-p <package>] [example_name ...]"
    exit 1
  fi
  PKG_ARGS=(-p "$2")
  CANDIDATE_DIR="crates/$2/examples"
  if [ -d "$CANDIDATE_DIR" ]; then
    EXAMPLES_DIR="$CANDIDATE_DIR"
  fi
  shift 2
fi

if [ "$#" -gt 0 ]; then
  EXAMPLES=("$@")
else
  mapfile -t EXAMPLES < <(find "$EXAMPLES_DIR" -maxdepth 1 -type f -name '*.rs' -printf '%f\n' | sed 's/\.rs$//' | sort)
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
  echo "[$INDEX/$COUNT] cargo check ${PKG_ARGS[*]} --release --example $EXAMPLE_NAME"
  CARGO_BUILD_JOBS="$JOBS" cargo check "${PKG_ARGS[@]}" --release --example "$EXAMPLE_NAME"
done

echo "All examples passed cargo check in release mode."
