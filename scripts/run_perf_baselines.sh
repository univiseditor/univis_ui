#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export UNIVIS_PERF_WARMUP="${UNIVIS_PERF_WARMUP:-48}"
export UNIVIS_PERF_ITERATIONS="${UNIVIS_PERF_ITERATIONS:-240}"

echo "== Solver Benchmarks =="
cargo run -p univis_ui_engine --release --example solver_benchmarks -- "$@"

echo
echo "== Runtime Benchmarks =="
cargo run -p univis_ui --release --example runtime_benchmarks -- "$@"
