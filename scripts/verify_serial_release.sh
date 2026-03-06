#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if [ "${1:-}" = "-p" ]; then
  if [ -z "${2:-}" ]; then
    echo "Usage: $0 [-p <package>] [example_name ...]"
    exit 1
  fi
  PKG="$2"
  shift 2
  ./scripts/test_lib_serial_release.sh -p "$PKG"
  ./scripts/check_examples_serial_release.sh -p "$PKG" "$@"
else
  ./scripts/test_lib_serial_release.sh
  ./scripts/check_examples_serial_release.sh "$@"
fi
