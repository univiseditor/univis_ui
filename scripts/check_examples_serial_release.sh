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

manifest_path_for_package() {
  local package="$1"
  if [ "$package" = "univis_ui" ]; then
    printf '%s\n' "Cargo.toml"
  else
    printf '%s\n' "crates/$package/Cargo.toml"
  fi
}

manifest_example_names_for_package() {
  local package="$1"
  local manifest
  manifest="$(manifest_path_for_package "$package")"

  if [ ! -f "$manifest" ]; then
    return 0
  fi

  awk '
    /^\[\[example\]\]/ { in_example = 1; next }
    /^\[/ { in_example = 0 }
    in_example && /^[[:space:]]*name[[:space:]]*=/ {
      line = $0
      sub(/^[^"]*"/, "", line)
      sub(/".*$/, "", line)
      if (line != "") {
        print line
      }
    }
  ' "$manifest"
}

autodiscovered_example_names_for_package() {
  local package="$1"
  local examples_dir
  examples_dir="$(example_dir_for_package "$package")"

  if [ -d "$examples_dir" ]; then
    find "$examples_dir" -maxdepth 1 -type f -name '*.rs' -printf '%f\n' | sed 's/\.rs$//' | sort
  fi
}

example_names_for_package() {
  local package="$1"
  {
    manifest_example_names_for_package "$package"
    autodiscovered_example_names_for_package "$package"
  } | sort -u
}

example_exists_in_package() {
  local package="$1"
  local example="$2"
  local candidate

  while IFS= read -r candidate; do
    if [ "$candidate" = "$example" ]; then
      return 0
    fi
  done < <(example_names_for_package "$package")

  return 1
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

  for package in "${PACKAGES[@]}"; do
    if example_exists_in_package "$package" "$example"; then
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

append_resolved_example() {
  local package="$1"
  local example="$2"
  local i

  for i in "${!RESOLVED_EXAMPLES[@]}"; do
    if [ "${RESOLVED_PACKAGES[$i]}" = "$package" ] && [ "${RESOLVED_EXAMPLES[$i]}" = "$example" ]; then
      return 0
    fi
  done

  RESOLVED_PACKAGES+=("$package")
  RESOLVED_EXAMPLES+=("$example")
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
  if [ "$#" -gt 0 ]; then
    for example in "$@"; do
      if ! example_exists_in_package "$PKG" "$example"; then
        echo "Example '$example' was not found in package '$PKG'." >&2
        exit 1
      fi
      append_resolved_example "$PKG" "$example"
    done
  else
    while IFS= read -r example; do
      append_resolved_example "$PKG" "$example"
    done < <(example_names_for_package "$PKG")
  fi
elif [ "$#" -gt 0 ]; then
  for example in "$@"; do
    append_resolved_example "$(resolve_package_for_example "$example")" "$example"
  done
else
  for package in "${PACKAGES[@]}"; do
    while IFS= read -r example; do
      append_resolved_example "$package" "$example"
    done < <(example_names_for_package "$package")
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
