#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"

cargo run -q -p vpp-conformance -- \
  --c-fixtures conformance/fixtures/c \
  --rust-fixtures conformance/fixtures/rust \
  --report-out echel-core/docs/development/06-evidence/conformance/report.json
