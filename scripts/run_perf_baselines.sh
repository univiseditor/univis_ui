#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

cat <<'EOF' >&2
The benchmark harness examples are archived in the current branch and their source
files are not shipped anymore.

Use the committed reports under `perf_baselines/current_max/2026-04-05/` instead:
- `solver_benchmarks.txt`
- `runtime_benchmarks.txt`
- `manifest.json`

If the harness sources are restored later, this script can be reconnected to the
example-backed workflow.
EOF

exit 1
