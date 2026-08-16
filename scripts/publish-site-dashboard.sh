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
SHARD_DIR="$REPO_ROOT/site/dashboard/units"
# Overridable only by scripts/tests/test_publish_site_dashboard.sh, which
# points this at a tiny fake producer so the seeding/comparison logic below
# can be proven without the real 4000+-line producer or a full corpus.
PRODUCER="${PF1E_DASHBOARD_PRODUCER:-$REPO_ROOT/scripts/observer/pf1e_dashboard_producer.py}"
# The real producer's `WORK_INVENTORY_FULL_DOC` default hardcodes
# `~/workspace/repos/codex/docs/work-inventory.json` -- correct for the
# shared checkout but WRONG from any other worktree (SD-31 cycles each run in
# their own `git worktree add` tree, `AGENTS.md`'s concurrency rules). Pin it
# to THIS checkout's own file explicitly so a worktree-run --check never
# silently reads a different tree's corpus snapshot.
export PF1E_WORK_INVENTORY_DOC="${PF1E_WORK_INVENTORY_DOC:-$REPO_ROOT/docs/work-inventory.json}"

if [ ! -f "$PRODUCER" ]; then
    echo "producer missing at $PRODUCER" >&2
    exit 2
fi

if [ "${1:-}" = "--check" ]; then
    TMPDIR_CHECK="$(mktemp -d -t pf1e-dashboard-check-XXXXXX)"
    trap 'rm -rf "$TMPDIR_CHECK"' EXIT
    TMP="$TMPDIR_CHECK/PF1e-dashboard.json"

    # Seed the scratch output with the CURRENT committed copy before running
    # the producer. The producer merges owner-managed state (manifests,
    # channels -- `_load_existing_owner_state`) and reuses a cached unit-shard
    # index (`build_unit_shards`) by reading whatever already lives at
    # `--out`/its sibling `units/` dir. Without this seed, `--check` renders
    # from a blank slate every time -- empty owner state, a freshly-timestamped
    # shard index -- and reports STALE unconditionally, even when the
    # committed copy is genuinely current. (Found and fixed this cycle,
    # SD31-ATTRIB-003: the unseeded version failed on an untouched tree.)
    if [ -f "$OUT" ]; then
        cp "$OUT" "$TMP"
    fi
    if [ -d "$SHARD_DIR" ]; then
        cp -r "$SHARD_DIR" "$TMPDIR_CHECK/units"
    fi

    python3 "$PRODUCER" --out "$TMP" >/dev/null

    # Compare everything except the stamps, which move on every run by
    # design. Stripped RECURSIVELY, not just at the top level: the payload
    # nests several independently-stamped sub-objects (`unit_index` from
    # `build_unit_shards`, `retrospective` from `scripts/retro.py summary`,
    # and any future one) that each carry their own `generated_at`/
    # `generated_by`, re-stamped on every run even when their own content is
    # byte-identical. A shallow, path-by-path strip missed `retrospective`
    # here on the first pass (SD31-ATTRIB-003) -- proven wrong by running
    # `--check` twice in a row against an untouched tree and seeing it fail
    # both times on that one key.
    if python3 - "$OUT" "$TMP" <<'PY'
import json
import sys


def strip_stamps(node):
    if isinstance(node, dict):
        for key in ("generated_at", "generated_by"):
            node.pop(key, None)
        for value in node.values():
            strip_stamps(value)
    elif isinstance(node, list):
        for item in node:
            strip_stamps(item)


def scrub(path):
    with open(path) as handle:
        doc = json.load(handle)
    strip_stamps(doc)
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
