#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# graphify-update.sh — SD-27's entry point to the vendored graphify update
# gate (scripts/graphify-update.py), used by the E4.2 architecture-closure
# cycle.
#
# Replaces the bundle's previously-cited `graphify cluster-only` command,
# which named a binary that is not on PATH. The Python gate drives that same
# cluster-only mode and locates the repo root by walking up from
# --receipts-md, so the ledger must live inside the repo.
#
# Usage:
#   bash scripts/graphify-update.sh                # SD-27 defaults
#   bash scripts/graphify-update.sh --dry-run      # no graphify run
#   bash scripts/graphify-update.sh --graphify-cli /path/to/graphify
#
# Environment:
#   BUNDLE              — bundle ID (default: SD-27)
#   RECEIPTS_MD         — append-only receipts ledger for the bundle
#   INTEGRATION_TARGET  — branch to capture branch_tip from (default: develop)
#
# Any extra arguments are forwarded to the Python gate verbatim.
# ---------------------------------------------------------------------------
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

BUNDLE="${BUNDLE:-SD-27}"
BUNDLE_DIR="${BUNDLE_DIR:-docs/release/SD-27-future-state-book-content-ingestion}"
RECEIPTS_MD="${RECEIPTS_MD:-${REPO_ROOT}/${BUNDLE_DIR}/artifacts/receipts.md}"
INTEGRATION_TARGET="${INTEGRATION_TARGET:-develop}"

if [[ ! -f "$RECEIPTS_MD" ]]; then
  echo "FATAL: receipts ledger not found: $RECEIPTS_MD" >&2
  echo "The bundle must have a receipts.md before this gate fires." >&2
  exit 2
fi

exec python3 "${REPO_ROOT}/scripts/graphify-update.py" \
  --bundle "$BUNDLE" \
  --receipts-md "$RECEIPTS_MD" \
  --integration-target "$INTEGRATION_TARGET" \
  "$@"
