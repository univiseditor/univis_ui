#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

JOBS="${CARGO_BUILD_JOBS:-1}"
PACKAGES=(
  univis_ui_style
  univis_ui_engine
  univis_ui_interaction
  univis_ui_widgets
  univis_ui
)

example_dir_for_package() {
  local package="$1"
  if [ "$package" = "univis_ui" ]; then
    printf '%s\n' "examples"
  else
    printf '%s\n' "crates/$package/examples"
  fi
}

feature_args_for_example() {
  local package="$1"
  local example="$2"
  case "$package:$example" in
    univis_ui_engine:border_light_3d | univis_ui:card_profile | univis_ui:sci_fi)
      printf '%s\n' "--features example_bloom"
      ;;
  esac
}

run_example_check() {
  local package="$1"
  local example="$2"
  local -a cargo_args=(-p "$package" --release --example "$example")
  local feature_args
  feature_args="$(feature_args_for_example "$package" "$example")"
  if [ -n "$feature_args" ]; then
    # shellcheck disable=SC2206
    cargo_args=($feature_args "${cargo_args[@]}")
  fi

  echo "cargo check ${cargo_args[*]}"
  CARGO_BUILD_JOBS="$JOBS" cargo check "${cargo_args[@]}"
}

resolve_package_for_example() {
  local example="$1"
  local matches=()

  if [ -f "examples/$example.rs" ]; then
    matches+=("univis_ui")
  fi

  for package in "${PACKAGES[@]}"; do
    if [ "$package" = "univis_ui" ]; then
      continue
    fi
    if [ -f "crates/$package/examples/$example.rs" ]; then
      matches+=("$package")
    fi
  done

  if [ "${#matches[@]}" -eq 0 ]; then
    echo "Example '$example' was not found in any package." >&2
    exit 1
  fi

  if [ "${#matches[@]}" -gt 1 ]; then
    echo "Example '$example' is ambiguous across packages: ${matches[*]}" >&2
    echo "Use -p <package> to disambiguate." >&2
    exit 1
  fi

  printf '%s\n' "${matches[0]}"
}

PKG=""
if [ "${1:-}" = "-p" ]; then
  if [ -z "${2:-}" ]; then
    echo "Usage: $0 [-p <package>] [example_name ...]"
    exit 1
  fi
  PKG="$2"
  shift 2
fi

declare -a RESOLVED_PACKAGES=()
declare -a RESOLVED_EXAMPLES=()

if [ -n "$PKG" ]; then
  EXAMPLES_DIR="$(example_dir_for_package "$PKG")"
  if [ "$#" -gt 0 ]; then
    for example in "$@"; do
      RESOLVED_PACKAGES+=("$PKG")
      RESOLVED_EXAMPLES+=("$example")
    done
  elif [ -d "$EXAMPLES_DIR" ]; then
    while IFS= read -r example; do
      RESOLVED_PACKAGES+=("$PKG")
      RESOLVED_EXAMPLES+=("$example")
    done < <(find "$EXAMPLES_DIR" -maxdepth 1 -type f -name '*.rs' -printf '%f\n' | sed 's/\.rs$//' | sort)
  fi
elif [ "$#" -gt 0 ]; then
  for example in "$@"; do
    RESOLVED_PACKAGES+=("$(resolve_package_for_example "$example")")
    RESOLVED_EXAMPLES+=("$example")
  done
else
  for package in "${PACKAGES[@]}"; do
    EXAMPLES_DIR="$(example_dir_for_package "$package")"
    if [ ! -d "$EXAMPLES_DIR" ]; then
      continue
    fi
    while IFS= read -r example; do
      RESOLVED_PACKAGES+=("$package")
      RESOLVED_EXAMPLES+=("$example")
    done < <(find "$EXAMPLES_DIR" -maxdepth 1 -type f -name '*.rs' -printf '%f\n' | sed 's/\.rs$//' | sort)
  done
fi

COUNT="${#RESOLVED_EXAMPLES[@]}"
if [ "$COUNT" -eq 0 ]; then
  echo "No examples found."
  exit 0
fi

echo "Checking $COUNT examples sequentially (release mode)..."
for i in "${!RESOLVED_EXAMPLES[@]}"; do
  index=$((i + 1))
  package="${RESOLVED_PACKAGES[$i]}"
  example="${RESOLVED_EXAMPLES[$i]}"
  echo "[$index/$COUNT] $package :: $example"
  run_example_check "$package" "$example"
done

echo "All examples passed cargo check in release mode."
