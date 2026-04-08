#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "== Library tests =="
cargo test -p univis_ui_engine --lib
cargo test -p univis_ui_interaction --lib
cargo test -p univis_ui_widgets --lib

echo
echo "== Representative examples =="
./scripts/check_representative_examples.sh

echo
echo "== Perf budgets =="
./scripts/run_perf_baselines.sh --check

echo
echo "Frame-zero rollout gates passed."
