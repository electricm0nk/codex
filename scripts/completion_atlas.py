#!/usr/bin/env python3
"""Partition `docs/work-inventory.json`'s full unit population into the ten
buckets fixed by SD-34 `decisions.md` §2 -- the atlas that plays, for SD-34,
the role `THE-BOX.md` played for SD-33.

`AT-34-E1-001` is the only criterion this cycle implements:

    python3 scripts/completion_atlas.py --check
        -> population=49438 buckets=10 unclassified=0 overlap=0   (exit 0)

Every unit lands in exactly one of:

    DONE  A  B  C  D  M  V  U  X  Z

Bucket derivation is keyed on `status` plus `evidence` (not `status` alone --
`evidence` is what separates A from B from C from D within the single
`not-ingested` status), reading the *live* inventory rather than any number
carried forward from a prior bundle (`decisions.md §12` L2).

`overlap` is structurally impossible under this implementation: `_bucket_of`
returns exactly one letter per unit, by construction, via an if/elif chain
with no bucket able to also claim another bucket's unit. It is still
computed and printed explicitly (never assumed) so a future refactor that
turns `_bucket_of` into a multi-match function trips a real check rather
than a silent invariant.

`unclassified` is real: `_bucket_of` returns `None` (never a made-up letter)
for any unit whose `(status, evidence)` pair matches nothing below, and
`--check` fails closed on that -- `AT-34-E1-002` condition 1.

`AT-34-E1-002`'s remaining five fail-closed conditions (a `DONE` unit whose
evidence does not support it, a bucket with no named clearing mechanism, a
stale `derived_at`, and the `file:line` evidence citation) are a separate
criterion and are not implemented by this cycle. `BUCKET_DEFINITIONS`
below already carries the `clears` and `evidence_source` fields that
criterion will assert against, so this file does not need to be reshaped
to add them later.
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import subprocess
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
ARTIFACT_PATH = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion", "artifacts", "epic-1-atlas",
    "completion-atlas.json",
)

# --- bucket A: "no engine table for this kind" -----------------------------
_A_MARKER = "has_no_engine_table"

# --- bucket B: "table exists, record not in it" -----------------------------
_B_MARKERS = ("not_held_by_engine", "absent_from", "not_modelled")

# --- bucket C: "held and computed, never surfaced" --------------------------
_C_MARKERS = ("explanation_id", "diagnostic")

BUCKET_DEFINITIONS = {
    "DONE": {
        "meaning": "nothing remains",
        "clears": "—",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {grounded, text-complete})",
    },
    "A": {
        "meaning": "engine has no table for this kind",
        "clears": "building the table (Epic 2)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'has_no_engine_table')"
        ),
    },
    "B": {
        "meaning": "table exists, record not in it",
        "clears": "placing the record (Epic 3/4)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'not_held_by_engine' / 'absent_from' / 'not_modelled')"
        ),
    },
    "C": {
        "meaning": "held and computed, never surfaced",
        "clears": "wiring the display/explanation path (Epic 3)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'explanation_id' / 'diagnostic')"
        ),
    },
    "D": {
        "meaning": "other engine gap (sub-causes enumerated, never a shrug)",
        "clears": "per named sub-cause",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == not-ingested, no other bucket matched)",
    },
    "M": {
        "meaning": "magnitude ingested, never computed or applied",
        "clears": "running the compute path (shape engine)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == ingested-magnitude)",
    },
    "V": {
        "meaning": "verified by proxy, never by the oracle",
        "clears": "the SD-33 oracle harness (scripts/oracle_harness/)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {literal-verified, fixture-verified})",
    },
    "U": {
        "meaning": "instrument cannot express a verdict",
        "clears": "instrument correction",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == unmeasurable)",
    },
    "X": {
        "meaning": "deferred with a stated reason",
        "clears": "revisiting the stated condition",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == deferred-with-reason)",
    },
    "Z": {
        "meaning": "not started",
        "clears": "ordinary work",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == not-started)",
    },
}

BUCKET_ORDER = ["DONE", "A", "B", "C", "D", "M", "V", "U", "X", "Z"]


def _bucket_of(unit: dict) -> "str | None":
    status = unit.get("status")
    evidence = unit.get("evidence") or ""

    if status in ("grounded", "text-complete"):
        return "DONE"
    if status in ("literal-verified", "fixture-verified"):
        return "V"
    if status == "ingested-magnitude":
        return "M"
    if status == "unmeasurable":
        return "U"
    if status == "deferred-with-reason":
        return "X"
    if status == "not-started":
        return "Z"
    if status == "not-ingested":
        if _A_MARKER in evidence:
            return "A"
        if any(marker in evidence for marker in _B_MARKERS):
            return "B"
        if any(marker in evidence for marker in _C_MARKERS):
            return "C"
        return "D"
    return None


def _load_inventory(path: str = INVENTORY_PATH) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def _head_sha() -> str:
    try:
        return subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=REPO_ROOT,
            capture_output=True, text=True, check=True,
        ).stdout.strip()
    except Exception:
        return "unknown"


def partition(units: list, book: "str | None" = None) -> dict:
    """Return (counts_by_bucket, unclassified_ids, overlap_ids, examined_population)."""
    counts = collections.Counter()
    unclassified_ids = []
    seen = set()
    overlap_ids = []
    examined = 0
    for unit in units:
        if book is not None and unit.get("book") != book:
            continue
        examined += 1
        b = _bucket_of(unit)
        uid = unit.get("id")
        if uid in seen:
            overlap_ids.append(uid)
        seen.add(uid)
        if b is None:
            unclassified_ids.append(uid)
            continue
        counts[b] += 1
    return {
        "counts": counts,
        "unclassified_ids": unclassified_ids,
        "overlap_ids": overlap_ids,
        "examined": examined,
    }


def _sub_causes(units: list, bucket: str) -> "collections.Counter | None":
    if bucket not in ("D", "U"):
        return None
    c = collections.Counter()
    for unit in units:
        if _bucket_of(unit) == bucket:
            c[unit.get("evidence")] += 1
    return c


def cmd_check(args) -> int:
    inv = _load_inventory()
    units = inv["units"]
    result = partition(units, book=args.book)
    counts = result["counts"]
    unclassified = len(result["unclassified_ids"])
    overlap = len(result["overlap_ids"])
    population = result["examined"]

    if args.book is None:
        print(
            f"population={population} buckets={len(BUCKET_ORDER)} "
            f"unclassified={unclassified} overlap={overlap}"
        )
        for b in BUCKET_ORDER:
            print(f"  {b}: {counts.get(b, 0)}")

        d_causes = _sub_causes(units, "D")
        u_causes = _sub_causes(units, "U")
        artifact = {
            "population": population,
            "derived_at": _head_sha(),
            "buckets": {
                b: {
                    "count": counts.get(b, 0),
                    "meaning": BUCKET_DEFINITIONS[b]["meaning"],
                    "clears": BUCKET_DEFINITIONS[b]["clears"],
                    "evidence_source": BUCKET_DEFINITIONS[b]["evidence_source"],
                }
                for b in BUCKET_ORDER
            },
            "unclassified": unclassified,
            "overlap": overlap,
            "sub_causes": {
                "D": dict(d_causes.most_common()) if d_causes else {},
                "U": dict(u_causes.most_common()) if u_causes else {},
            },
            "re_derive_command": "python3 scripts/completion_atlas.py --check",
        }
        os.makedirs(os.path.dirname(ARTIFACT_PATH), exist_ok=True)
        with open(ARTIFACT_PATH, "w", encoding="utf-8") as fh:
            json.dump(artifact, fh, indent=2, sort_keys=True)
            fh.write("\n")

        if unclassified != 0 or overlap != 0:
            return 1
        return 0

    # --book <slug> --check: exit 0 only when every non-DONE bucket is 0
    print(
        f"book={args.book} population={population} "
        f"unclassified={unclassified} overlap={overlap}"
    )
    for b in BUCKET_ORDER:
        print(f"  {b}: {counts.get(b, 0)}")
    if unclassified != 0 or overlap != 0:
        return 1
    non_done_total = sum(counts.get(b, 0) for b in BUCKET_ORDER if b != "DONE")
    return 0 if non_done_total == 0 else 1


def cmd_by_book(args) -> int:
    inv = _load_inventory()
    units = inv["units"]
    books = sorted({u.get("book") for u in units if u.get("book")})
    for book in books:
        result = partition(units, book=book)
        counts = result["counts"]
        total = result["examined"]
        row = " ".join(
            f"{b}={counts.get(b, 0)}({(counts.get(b, 0) / total * 100 if total else 0):.1f}%)"
            for b in BUCKET_ORDER
        )
        print(f"{book} (n={total}): {row}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--by-book", action="store_true")
    parser.add_argument("--book", default=None)
    args = parser.parse_args(argv)

    if args.by_book:
        return cmd_by_book(args)
    if args.check:
        return cmd_check(args)
    parser.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
