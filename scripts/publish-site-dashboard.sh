#!/usr/bin/env bash
# Refresh the public status feed under site/dashboard/ from the live producer.
#
# WHY THIS EXISTS: pf1e_dashboard_producer.py writes to $PF1E_JSON_PATH
# (default ~/swarm-observer/PF1e-dashboard.json) -- OUTSIDE the repo. Nothing
# was versioned, nothing was reviewable, and the file died with the box. This
# script produces the same JSON straight into the repo so the public site under
# site/ can serve it.
#
# The viewer fetches "PF1e-dashboard.json" as a RELATIVE url (the JSON_URL const
# in the page), so the data file must sit beside the page that serves it.
#
# Usage:
#   ./scripts/publish-site-dashboard.sh            regenerate the feed in place
#   ./scripts/publish-site-dashboard.sh --check    fail if the committed copy is
#                                                  stale (for a verify.sh stage)
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$REPO_ROOT/site/dashboard/PF1e-dashboard.json"
PRODUCER="$REPO_ROOT/scripts/observer/pf1e_dashboard_producer.py"

if [ ! -f "$PRODUCER" ]; then
    echo "producer missing at $PRODUCER" >&2
    exit 2
fi

if [ "${1:-}" = "--check" ]; then
    TMP="$(mktemp -t pf1e-dashboard-check-XXXXXX.json)"
    trap 'rm -f "$TMP"' EXIT
    python3 "$PRODUCER" --out "$TMP" >/dev/null

    # Compare everything except the stamps, which move on every run by design.
    if python3 - "$OUT" "$TMP" <<'PY'
import json
import sys


def scrub(path):
    with open(path) as handle:
        doc = json.load(handle)
    for key in ("generated_at", "generated_by"):
        doc.pop(key, None)
    return json.dumps(doc, sort_keys=True)


sys.exit(0 if scrub(sys.argv[1]) == scrub(sys.argv[2]) else 1)
PY
    then
        echo "site/dashboard/PF1e-dashboard.json is current"
    else
        echo "site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh" >&2
        exit 1
    fi
else
    python3 "$PRODUCER" --out "$OUT" >/dev/null
    echo "wrote $OUT"
fi
