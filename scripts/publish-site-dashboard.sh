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
#   ./scripts/publish-site-dashboard.sh --check-pin
#                                                  fail in MILLISECONDS if the
#                                                  feed's recorded input has
#                                                  moved (for the per-cycle
#                                                  push gate) -- see below
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$REPO_ROOT/site/dashboard/PF1e-dashboard.json"
SHARD_DIR="$REPO_ROOT/site/dashboard/units"
# THE INPUT PIN -- the control for incident key
# `site-dashboard-json-stale-after-inventory-move`.
#
# `--check` is correct but costs ~15 minutes of real producer time (measured
# 904 s on the shared checkout, 2026-09-10, HEAD 00e44eee02), so it only ever
# ran at the ~90-minute epic wrap-up gate. The key fired three times, and each
# time the disposition was "regenerate it in the wrap-up correction cycle" --
# a chore, not a mechanism (AGENTS.md rule 8). Every one of those firings had
# the same cause: a cycle regenerated `docs/work-inventory.json` (the
# producer's dominant input, `PF1E_WORK_INVENTORY_DOC` below) and did not
# republish the feed derived from it.
#
# So a real regen records the sha256 of the inventory it rendered from, and
# `--check-pin` re-hashes that one file and compares. No producer, no corpus,
# no build -- cheap enough to sit in every cycle's pre-push gate
# (`workflow-instruction.md` §6 step 3), which is where the divergence is
# actually created. `--check` runs it first too, so the wrap-up gate stops
# paying 15 minutes to learn what one hash already knew.
#
# WHAT THE PIN DOES NOT COVER (AGENTS.md rule 7): it watches ONE input. A feed
# made stale by a unit-ledger edit or an owner-state manifest change hashes
# clean here and is caught only by the full `--check`. The pin narrows the
# window on the recorded cause; it does not replace the full check, and both
# stages stay in verify.sh for that reason.
PIN="$REPO_ROOT/site/dashboard/inventory-pin.json"
# Overridable only by scripts/tests/test_publish_site_dashboard.sh, which
# points this at a tiny fake producer so the seeding/comparison logic below
# can be proven without the real 4000+-line producer or a full corpus.
PRODUCER="${PF1E_DASHBOARD_PRODUCER:-$REPO_ROOT/scripts/observer/pf1e_dashboard_producer.py}"
# Overridable only by scripts/tests/test_publish_site_dashboard.sh, for the same
# reason PRODUCER is: the self-test builds a hermetic fake repo containing only
# this script, so the real projection generator (which imports the observer
# package and reads the committed unit ledgers) is not present there. The test
# points this at a tiny stand-in; every real run uses the default.
PUBLIC_STATUS_BUILDER="${PF1E_PUBLIC_STATUS_BUILDER:-$REPO_ROOT/scripts/site/build_public_status.py}"
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

# Compare the recorded input pin against the inventory on disk. Exits 0 when
# they agree, 1 when they do not OR when no pin was ever recorded -- an absent
# pin is a FAILURE, never a silent pass, because "nothing to compare" is
# exactly the always-green shape scripts/tests/test_publish_site_dashboard.sh
# exists to prevent (case 13).
check_input_pin() {
    if [ ! -f "$PF1E_WORK_INVENTORY_DOC" ]; then
        echo "work inventory missing at $PF1E_WORK_INVENTORY_DOC" >&2
        return 1
    fi
    local actual
    actual="$(sha256sum "$PF1E_WORK_INVENTORY_DOC" | awk '{print $1}')"
    if [ ! -f "$PIN" ]; then
        echo "site/dashboard/inventory-pin.json: no pin recorded -- run ./scripts/publish-site-dashboard.sh" >&2
        return 1
    fi
    local recorded
    recorded="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("work_inventory_sha256",""))' "$PIN")"
    if [ "$recorded" != "$actual" ]; then
        echo "docs/work-inventory.json has moved since the feed was published (pinned $recorded, on disk $actual) -- run ./scripts/publish-site-dashboard.sh" >&2
        return 1
    fi
    echo "site/dashboard/PF1e-dashboard.json input pin matches docs/work-inventory.json ($actual)"
    return 0
}

# Record the pin from the inventory the producer just rendered from.
write_input_pin() {
    python3 - "$PIN" "$PF1E_WORK_INVENTORY_DOC" <<'PY'
import hashlib, json, os, sys, time

pin_path, inventory_path = sys.argv[1], sys.argv[2]
with open(inventory_path, "rb") as handle:
    digest = hashlib.sha256(handle.read()).hexdigest()
os.makedirs(os.path.dirname(pin_path), exist_ok=True)
with open(pin_path, "w") as handle:
    json.dump(
        {
            "_comment": (
                "Written by scripts/publish-site-dashboard.sh. The sha256 of the "
                "docs/work-inventory.json that site/dashboard/PF1e-dashboard.json "
                "was rendered from. `--check-pin` re-hashes that file and compares, "
                "so a cycle that moves the inventory without republishing the feed "
                "goes red in milliseconds at its own push gate instead of 90 minutes "
                "later at the epic wrap-up."
            ),
            "pinned_from": "docs/work-inventory.json",
            "work_inventory_sha256": digest,
            "pinned_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        },
        handle,
        indent=2,
        sort_keys=True,
    )
    handle.write("\n")
print(f"pinned docs/work-inventory.json sha256={digest}")
PY
}

if [ "${1:-}" = "--check-pin" ]; then
    check_input_pin
    exit $?
fi

if [ "${1:-}" = "--check" ]; then
    # Fast path first: if the pin already proves the feed's dominant input
    # moved, say so now rather than after ~15 minutes of producer time.
    check_input_pin || exit 1

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
        # `-p` (preserve timestamps) matters: `build_unit_shards`' cache
        # hit test is `getmtime(manifest_path) >= src_mtime` -- an
        # unpreserved `cp -r` stamps mtime=now on the copy, which makes the
        # cache hit UNCONDITIONALLY and serves the seeded copy back
        # verbatim regardless of whether docs/work-inventory.json actually
        # moved. Preserving the source mtime lets the real cache logic run
        # honestly instead of being short-circuited by the seed step itself
        # (wave-8 adversarial review, confirmed by mutation: poisoning a
        # seeded index.json with fabricated totals came back unchanged
        # under the unpreserved copy).
        cp -r -p "$SHARD_DIR" "$TMPDIR_CHECK/units"
    fi

    # PF1E_DASHBOARD_STRICT_TIMEOUT=1: --check is the ONLY caller that opts
    # into loud-failure-on-timeout. A live regeneration (the `else` branch
    # below) leaves this unset so its own stale-cache fallback keeps the
    # public site from going blank over one slow build -- see
    # `StateDumpTimeout`'s own docstring in pf1e_dashboard_producer.py for
    # why a --check that silently compared two stale-cache-derived outputs
    # and reported "current" was the actual defect (SD-34 AT-34-E6-001
    # wave-27), not the 600s cap by itself.
    PF1E_DASHBOARD_STRICT_TIMEOUT=1 python3 "$PRODUCER" --out "$TMP" >/dev/null

    # Compare everything except the stamps, which move on every run by
    # design, AND everything that is not actually derived from committed
    # repo state. Stamps are stripped RECURSIVELY, not just at the top
    # level: the payload nests several independently-stamped sub-objects
    # (`unit_index` from `build_unit_shards`, `retrospective` from
    # `scripts/retro.py summary`, and any future one) that each carry their
    # own `generated_at`/`generated_by`, re-stamped on every run even when
    # their own content is byte-identical. A shallow, path-by-path strip
    # missed `retrospective` here on the first pass (SD31-ATTRIB-003) --
    # proven wrong by running `--check` twice in a row against an untouched
    # tree and seeing it fail both times on that one key.
    #
    # Two whole subtrees are dropped from the comparison entirely, not just
    # stamp-stripped, because their VALUES genuinely drift with no commit
    # involved (wave-8 adversarial review, confirmed by running --check on
    # a several-hours-old tree and diffing committed-vs-fresh with only
    # stamps stripped): `usage` reads a live cache file OUTSIDE the repo
    # (`DEFAULT_USAGE_CACHE`, no commit can pin it), and `retrospective`
    # summarizes `docs/retro/events/*.jsonl`, which every cycle is mandated
    # to append to as part of its own normal work -- neither one is a sign
    # the DASHBOARD is stale. The `[engine content dump <timestamp>]`
    # suffix `derive_book_matrix` appends to each unresolved book/kind's
    # `open_questions` text is scrubbed the same way for the same reason: it
    # restamps on every run even when the underlying reason is unchanged.
    if python3 - "$OUT" "$TMP" <<'PY'
import json
import re
import sys

ENGINE_DUMP_SUFFIX = re.compile(r" \[engine content dump [^\]]*\]$")


def strip_stamps(node):
    if isinstance(node, dict):
        for key in ("generated_at", "generated_by"):
            node.pop(key, None)
        for value in node.values():
            strip_stamps(value)
    elif isinstance(node, list):
        for item in node:
            strip_stamps(item)


def strip_engine_dump_suffix(node):
    if isinstance(node, dict):
        for key, value in list(node.items()):
            if isinstance(value, str):
                node[key] = ENGINE_DUMP_SUFFIX.sub("", value)
            else:
                strip_engine_dump_suffix(value)
    elif isinstance(node, list):
        for item in node:
            strip_engine_dump_suffix(item)


def scrub(path):
    with open(path) as handle:
        doc = json.load(handle)
    # Not repo-derived state -- see the comment above this heredoc.
    doc.pop("usage", None)
    doc.pop("retrospective", None)
    strip_stamps(doc)
    strip_engine_dump_suffix(doc)
    return json.dumps(doc, sort_keys=True)


sys.exit(0 if scrub(sys.argv[1]) == scrub(sys.argv[2]) else 1)
PY
    then
        echo "site/dashboard/PF1e-dashboard.json is current"
    else
        echo "site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh" >&2
        exit 1
    fi

    # The public status page (site/status.html) never reads this feed or its
    # `usage`/`channels.*.agents`/session-prose content directly -- it fetches
    # site/status-data.json and site/status-data/<book>.json, a narrow
    # allow-listed projection computed straight from site/dashboard/units/*.json
    # by scripts/site/build_public_status.py (see that script's module
    # docstring, and this repo's "What is in the feed" note in
    # site/dashboard/README.md, for why the projection reads the unit ledgers
    # rather than this file). Check that projection is current too.
    python3 "$PUBLIC_STATUS_BUILDER" --check
else
    python3 "$PRODUCER" --out "$OUT" >/dev/null
    echo "wrote $OUT"

    # Record what this render was derived from, in the same run that rendered
    # it -- the two cannot diverge because nothing writes one without the other.
    write_input_pin

    echo "==> Regenerating public status projection (site/status-data.json + site/status-data/*.json)"
    python3 "$PUBLIC_STATUS_BUILDER"
fi
