#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

WARN_LOC_THRESHOLD=900
FAIL_LOC_THRESHOLD=1000
ALLOWLIST_FILE="$(mktemp)"
ACTUAL_FILE="$(mktemp)"
trap 'rm -f "$ALLOWLIST_FILE" "$ACTUAL_FILE"' EXIT

have_rg() {
  command -v rg >/dev/null 2>&1
}

list_matching_files() {
  local pattern="$1"
  shift

  if have_rg; then
    rg -l -- "$pattern" "$@"
  else
    grep -RIlE --include='*.rs' -- "$pattern" "$@"
  fi
}

print_matches() {
  local pattern="$1"
  shift

  if have_rg; then
    rg -n -- "$pattern" "$@"
  else
    grep -RInE --include='*.rs' -- "$pattern" "$@"
  fi
}

source_line_counts() {
  find crates src -type f -name '*.rs' ! -path '*/examples/*' -print0 \
    | xargs -0 wc -l \
    | awk '$2 != "total"' \
    | sort -nr
}

cat >"$ALLOWLIST_FILE" <<'EOF'
crates/univis_ui_engine/src/layout/algorithms/bridge.rs
crates/univis_ui_engine/src/layout/core/pass_up.rs
crates/univis_ui_engine/src/layout/core/solver/absolute.rs
crates/univis_ui_engine/src/layout/core/solver/helpers.rs
crates/univis_ui_engine/src/layout/core/solver/translate.rs
crates/univis_ui_engine/src/layout/core/solver/types.rs
crates/univis_ui_engine/src/layout/geometry.rs
crates/univis_ui_engine/src/layout/layout_system/root_stacking.rs
crates/univis_ui_engine/src/layout/layout_system/roots.rs
crates/univis_ui_engine/src/layout/layout_system/ui3d_sync.rs
crates/univis_ui_engine/src/layout/pipeline/container.rs
crates/univis_ui_engine/src/layout/render/system.rs
crates/univis_ui_engine/src/layout/tests.rs
crates/univis_ui_engine/src/layout/univis_node.rs
crates/univis_ui_widgets/src/widget/badge.rs
crates/univis_ui_widgets/src/widget/button.rs
crates/univis_ui_widgets/src/widget/checkbox.rs
crates/univis_ui_widgets/src/widget/divider.rs
crates/univis_ui_widgets/src/widget/drag_value.rs
crates/univis_ui_widgets/src/widget/icon_btn.rs
crates/univis_ui_widgets/src/widget/image.rs
crates/univis_ui_widgets/src/widget/progress.rs
crates/univis_ui_widgets/src/widget/scroll_view.rs
crates/univis_ui_widgets/src/widget/seekbar.rs
crates/univis_ui_widgets/src/widget/toggle.rs
EOF
sort -o "$ALLOWLIST_FILE" "$ALLOWLIST_FILE"

echo "== Largest src files =="
source_line_counts | sed -n '1,15p'

largest_file_entry="$(source_line_counts | sed -n '1p')"
largest_file_loc="$(awk '{print $1}' <<<"$largest_file_entry")"

if [[ -n "$largest_file_loc" && "$largest_file_loc" -gt "$FAIL_LOC_THRESHOLD" ]]; then
  echo
  echo "A source file exceeded the hard LOC guardrail of $FAIL_LOC_THRESHOLD lines." >&2
  echo "$largest_file_entry" >&2
  exit 1
fi

warning_entries="$(source_line_counts | awk -v threshold="$WARN_LOC_THRESHOLD" '$1 > threshold')"
if [[ -n "$warning_entries" ]]; then
  echo
  echo "Warning: source files above $WARN_LOC_THRESHOLD LOC were detected:" >&2
  echo "$warning_entries" >&2
fi

echo
echo "== internal_prelude wildcard usage =="
list_matching_files '^use crate::internal_prelude::\*;$' crates src | sort >"$ACTUAL_FILE" || true
cat "$ACTUAL_FILE"

unexpected_usage="$(comm -23 "$ACTUAL_FILE" "$ALLOWLIST_FILE")"
if [[ -n "$unexpected_usage" ]]; then
  echo
  echo "Unexpected new 'use crate::internal_prelude::*;' usage detected:" >&2
  echo "$unexpected_usage" >&2
  exit 1
fi

echo
echo "== Result<_, ()> guardrail =="
if print_matches 'Result<[^>]*,\s*\(\s*\)>' crates src; then
  echo "Found forbidden Result<_, ()> usage." >&2
  exit 1
fi

echo
echo "Structure guardrails passed."
