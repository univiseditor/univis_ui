#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

JOBS="${CARGO_BUILD_JOBS:-1}"
TMP_LIST="$(mktemp)"
trap 'rm -f "$TMP_LIST"' EXIT

PKG_ARGS=()
if [ "${1:-}" = "-p" ]; then
  if [ -z "${2:-}" ]; then
    echo "Usage: $0 [-p <package>]"
    exit 1
  fi
  PKG_ARGS=(-p "$2")
  shift 2
fi

echo "Listing lib tests (release mode)..."
CARGO_BUILD_JOBS="$JOBS" cargo test "${PKG_ARGS[@]}" --release --lib -- --list > "$TMP_LIST"

mapfile -t TESTS < <(sed -n 's/: test$//p' "$TMP_LIST")

COUNT="${#TESTS[@]}"
if [ "$COUNT" -eq 0 ]; then
  echo "No lib tests found."
  exit 0
fi

echo "Running $COUNT lib tests sequentially (release mode)..."
INDEX=0
for TEST_NAME in "${TESTS[@]}"; do
  INDEX=$((INDEX + 1))
  echo "[$INDEX/$COUNT] $TEST_NAME"
  CARGO_BUILD_JOBS="$JOBS" cargo test "${PKG_ARGS[@]}" --release --lib "$TEST_NAME" -- --exact --test-threads=1
done

echo "All lib tests passed in release mode."
