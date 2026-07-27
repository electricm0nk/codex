#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# architecture-truth-up.sh — SD-27's entry point to the vendored
# architecture truth-up gate (scripts/architecture_truth_up.py).
#
# The vendored Python gate requires --bundle and --receipts-md. This wrapper
# supplies the SD-27 defaults so the bundle's cited verification command
# (`bash scripts/architecture-truth-up.sh`) is runnable with no arguments,
# which is the form acceptance-and-verification.md §2.6 asserts.
#
# Usage:
#   bash scripts/architecture-truth-up.sh                 # SD-27 defaults
#   bash scripts/architecture-truth-up.sh --dry-run       # no edits/appends
#   BUNDLE=SD-28 bash scripts/architecture-truth-up.sh    # another bundle
#
# Environment:
#   BUNDLE              — bundle ID (default: SD-27)
#   RECEIPTS_MD         — append-only receipts ledger for the bundle
#   INTEGRATION_TARGET  — branch to diff against (default: develop)
#
# Any extra arguments are forwarded to the Python gate verbatim.
# ---------------------------------------------------------------------------
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

BUNDLE="${BUNDLE:-SD-27}"
BUNDLE_DIR="${BUNDLE_DIR:-docs/release/SD-27-future-state-book-content-ingestion}"
RECEIPTS_MD="${RECEIPTS_MD:-${REPO_ROOT}/${BUNDLE_DIR}/artifacts/receipts.md}"
INTEGRATION_TARGET="${INTEGRATION_TARGET:-develop}"

# The gate appends to the ledger; create it on first run rather than failing.
mkdir -p "$(dirname "$RECEIPTS_MD")"
[[ -f "$RECEIPTS_MD" ]] || printf '# %s — Architecture truth-up receipts\n\n' "$BUNDLE" > "$RECEIPTS_MD"

exec python3 "${REPO_ROOT}/scripts/architecture_truth_up.py" \
  --bundle "$BUNDLE" \
  --receipts-md "$RECEIPTS_MD" \
  --integration-target "$INTEGRATION_TARGET" \
  "$@"
