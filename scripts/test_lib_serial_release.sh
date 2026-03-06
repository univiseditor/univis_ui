#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

JOBS="${CARGO_BUILD_JOBS:-1}"
TMP_LIST="$(mktemp)"
trap 'rm -f "$TMP_LIST"' EXIT

echo "Listing lib tests (release mode)..."
CARGO_BUILD_JOBS="$JOBS" cargo test --release --lib -- --list > "$TMP_LIST"

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
  CARGO_BUILD_JOBS="$JOBS" cargo test --release --lib "$TEST_NAME" -- --exact --test-threads=1
done

echo "All lib tests passed in release mode."
