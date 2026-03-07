#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERIFY=1
OFFLINE=0
LIST_ONLY=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    --no-verify)
      VERIFY=0
      shift
      ;;
    --offline)
      OFFLINE=1
      shift
      ;;
    --list-only)
      LIST_ONLY=1
      shift
      ;;
    *)
      break
      ;;
  esac
done

if [ "$#" -gt 0 ]; then
  PACKAGES=("$@")
else
  PACKAGES=(
    univis_ui_style
    univis_ui_engine
    univis_ui_interaction
    univis_ui_widgets
    univis_ui
  )
fi

COUNT="${#PACKAGES[@]}"
INDEX=0
for PACKAGE in "${PACKAGES[@]}"; do
  INDEX=$((INDEX + 1))
  echo "[$INDEX/$COUNT] cargo package -p $PACKAGE"
  EXTRA_ARGS=()
  if [ "$OFFLINE" -eq 1 ]; then
    EXTRA_ARGS+=(--offline)
  fi
  if [ "$LIST_ONLY" -eq 1 ]; then
    cargo package -p "$PACKAGE" --list --allow-dirty "${EXTRA_ARGS[@]}"
  elif [ "$VERIFY" -eq 1 ]; then
    cargo package -p "$PACKAGE" --allow-dirty "${EXTRA_ARGS[@]}"
  else
    cargo package -p "$PACKAGE" --allow-dirty --no-verify "${EXTRA_ARGS[@]}"
  fi
done

if [ "$LIST_ONLY" -eq 1 ]; then
  echo "All package file lists were generated successfully."
else
  echo "All packages were created successfully."
fi
