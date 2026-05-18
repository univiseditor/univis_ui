#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "== Cargo fmt =="
cargo fmt --all --check

echo
echo "== Cargo clippy =="
cargo clippy --workspace --all-targets -- -D warnings \
  -A clippy::collapsible_if \
  -A clippy::default_constructed_unit_structs \
  -A clippy::derivable_impls \
  -A clippy::empty_line_after_doc_comments \
  -A clippy::explicit_auto_deref \
  -A clippy::field_reassign_with_default \
  -A clippy::if_same_then_else \
  -A clippy::manual_div_ceil \
  -A clippy::needless_option_as_deref \
  -A clippy::needless_range_loop \
  -A clippy::needless_update \
  -A clippy::ptr_arg \
  -A clippy::redundant_field_names \
  -A clippy::too_many_arguments \
  -A clippy::type_complexity \
  -A clippy::unnecessary_map_or \
  -A clippy::unwrap_or_default \
  -A clippy::useless_conversion

echo
echo "== Cargo check =="
cargo check --workspace --all-targets

echo
echo "== API docs =="
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

echo
echo "== Public API surface =="
./scripts/check_public_api_surface.sh

echo
echo "== Structure guardrails =="
./scripts/check_structure_guardrails.sh

echo
echo "== Engine boundary guardrails =="
./scripts/check_engine_boundary_guardrails.sh

echo
echo "== Library tests =="
cargo test --workspace --lib

echo
echo "Quality checks completed successfully."
