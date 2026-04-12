#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ALLOWLIST_FILE="$(mktemp)"
ACTUAL_FILE="$(mktemp)"
trap 'rm -f "$ALLOWLIST_FILE" "$ACTUAL_FILE"' EXIT

have_rg() {
  command -v rg >/dev/null 2>&1
}

collect_matches() {
  local kind="$1"
  local pattern="$2"

  if have_rg; then
    rg -l --glob '!crates/univis_ui_engine/**' -- "$pattern" crates src examples \
      | sed "s|^|$kind	|"
  else
    find crates src examples -type f -name '*.rs' ! -path 'crates/univis_ui_engine/*' -print0 \
      | xargs -0 grep -lE -- "$pattern" \
      | sed "s|^|$kind	|"
  fi
}

print_kind_matches() {
  local kind="$1"
  local pattern="$2"

  if have_rg; then
    rg -n --glob '!crates/univis_ui_engine/**' -- "$pattern" crates src examples \
      | sed "s|^|[$kind] |"
  else
    find crates src examples -type f -name '*.rs' ! -path 'crates/univis_ui_engine/*' -print0 \
      | xargs -0 grep -nE -- "$pattern" \
      | sed "s|^|[$kind] |"
  fi
}

cat >"$ALLOWLIST_FILE" <<'EOF'
EOF
sort -o "$ALLOWLIST_FILE" "$ALLOWLIST_FILE"

{
  collect_matches "engine_internal" '\bunivis_ui_engine::internal(::|\b)'
  collect_matches "engine_hidden_core" '\bunivis_ui_engine::layout::core::'
  collect_matches "engine_hidden_components" '\bunivis_ui_engine::layout::components::'
  collect_matches "engine_hidden_algorithms" '\bunivis_ui_engine::layout::algorithms::'
  collect_matches "engine_hidden_pipeline" '\bunivis_ui_engine::layout::pipeline::'
  collect_matches "engine_hidden_solver_types" '\bunivis_ui_engine::layout::solver_types::'
} | sort -u >"$ACTUAL_FILE" || true

echo "== Engine boundary guardrails =="
echo "Tracked legacy exceptions: $(wc -l <"$ALLOWLIST_FILE" | tr -d ' ')"
echo "Current legacy exceptions: $(wc -l <"$ACTUAL_FILE" | tr -d ' ')"

resolved_entries="$(comm -23 "$ALLOWLIST_FILE" "$ACTUAL_FILE")"
if [[ -n "$resolved_entries" ]]; then
  echo
  echo "Resolved legacy boundary exceptions detected:"
  echo "$resolved_entries"
fi

unexpected_entries="$(comm -23 "$ACTUAL_FILE" "$ALLOWLIST_FILE")"
if [[ -n "$unexpected_entries" ]]; then
  echo
  echo "Unexpected new engine-boundary violations detected:" >&2
  echo "$unexpected_entries" >&2
  echo >&2
  echo "Detailed matches:" >&2
  print_kind_matches "engine_internal" '\bunivis_ui_engine::internal(::|\b)' >&2 || true
  print_kind_matches "engine_hidden_core" '\bunivis_ui_engine::layout::core::' >&2 || true
  print_kind_matches "engine_hidden_components" '\bunivis_ui_engine::layout::components::' >&2 || true
  print_kind_matches "engine_hidden_algorithms" '\bunivis_ui_engine::layout::algorithms::' >&2 || true
  print_kind_matches "engine_hidden_pipeline" '\bunivis_ui_engine::layout::pipeline::' >&2 || true
  print_kind_matches "engine_hidden_solver_types" '\bunivis_ui_engine::layout::solver_types::' >&2 || true
  exit 1
fi

echo
echo "No new engine-boundary violations were introduced."
