#!/usr/bin/env python3
"""Row 17 census (`kanban.md` row 17, `epic-7-shape-categorization-100`,
`decisions.md §27`/`§27a`/`§27b`) — the instrument row 17 needs before it
can be started: "which units have a real shape, and which have a
placeholder?"

**Read-only.** This script writes nothing to `data/corpus/**`, ingests
nothing, and does no ingest-tool work — it only reports. Row 17 itself is
correctly sequenced `backlog` (`decisions.md §27`: a shape cannot be
categorized for an object that has not been ingested); this census exists
so row 17 does not have to invent its own sizing under time pressure once
`no_record` reaches zero.

**Two populations `§27a` names explicitly, both counted here per kind and
per book:**

1. Units carrying `§27`'s provisional `SpecialQuality` default —
   `scripts/shape_provisional_marker.scan_corpus_for_provisional_defaults`.
   Zero today: no ingest cycle has applied the default yet (verified by
   this script's own `--check` mode, which fails if it finds a `TYPE:`
   facet-less delivery-only shape shipped as `SpecialQuality` with NO
   marker — see `audit_unmarked_defaults` below).
2. Units whose shape is `F0` reached by FALLTHROUGH rather than by
   derivation — `scripts/shape_ledger.py`'s `f0_reached_by` field
   (`"fallthrough"`), which this cycle added specifically because the
   pre-existing ledger could not tell "genuinely no formula" apart from
   "nothing else matched" WITHIN its own `matched` join-status bucket.

**Row 17's honest size** (`decisions.md §27a`, no bare totals per `§12c`):
the `fallthrough` count is the actionable population row 17 must
re-categorize once `no_record` (the `not_ingested` bucket below) reaches
zero — `measured_empty` and `not_ingested` are NOT row 17's population
(the former is a real answer already; the latter cannot be categorized
until it is ingested).

Run:

    python3 scripts/row17_census.py [--inventory PATH] [--corpus-root PATH] \\
        [--output artifacts/.../row17-census.json] [--check]

`--check` exits 1 if any provisional-default marker is missing its
required reason (a contract violation `shape_provisional_marker` itself
allows to be reported but never silently drops).
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from collections import defaultdict

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import coverage_ledger as CL  # noqa: E402
import shape_ledger as SL  # noqa: E402
import shape_provisional_marker as SPM  # noqa: E402

DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")


def build_census(inventory_path: str, corpus_root: str) -> dict:
    inventory = SL.load_inventory_or_die(inventory_path)
    units = CL.not_done_population(inventory)
    if not units:
        raise SystemExit(
            f"no coverage: inventory at {inventory_path!r} has zero not-done units "
            "(fail-closed posture, mirrors AT-32-G1-002)"
        )

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = SL.build_corpus_index(corpus_root, books)
    key_index = SL.build_corpus_key_index(corpus_root, books)
    cross_book_key_index = SL.build_cross_book_key_index(corpus_root)
    ledger = SL.build_ledger(units, corpus_index, key_index, cross_book_key_index)

    provisional_hits = SPM.scan_corpus_for_provisional_defaults(corpus_root)
    # Reconciled against the SAME not-done population's (kind, book) pairs
    # this census reports over — a provisional-default hit outside that
    # population (e.g. a `done` record) is still counted in the corpus-wide
    # total but flagged separately so the two totals never silently blur.
    unit_kb = {(u.get("kind"), u.get("book")) for u in units}
    provisional_in_population = [
        h for h in provisional_hits if (h["kind"], h["book"]) in unit_kb
    ]
    provisional_missing_reason = [h for h in provisional_hits if not h.get("reason")]

    # Per (kind, book) breakdown.
    per_kb: dict[tuple, dict] = defaultdict(
        lambda: {
            "derived": 0,
            "measured_empty": 0,
            "measured_pi_redacted": 0,
            "fallthrough": 0,
            "fallthrough_pi_redacted": 0,
            "not_ingested": 0,
        }
    )
    for row in ledger["rows"]:
        key = (row["kind"], row["book"])
        bucket = per_kb[key]
        reached_by = row.get("f0_reached_by")
        if reached_by == "not_ingested":
            bucket["not_ingested"] += 1
        elif reached_by == "measured_empty":
            bucket["measured_empty"] += 1
        elif reached_by == "measured_pi_redacted":
            # T9-onboarding-cause-closure (2026-08-23, row 17's remaining
            # 21) / `decisions.md §27a`: a value that genuinely carries PI
            # and stays redacted is a REAL answer, not row 17's placeholder
            # population -- counted alongside `measured_empty`, never
            # folded into `fallthrough`.
            bucket["measured_pi_redacted"] += 1
        elif reached_by == "fallthrough":
            bucket["fallthrough"] += 1
            if row.get("pi_redacted_formula"):
                bucket["fallthrough_pi_redacted"] += 1
        else:
            bucket["derived"] += 1

    provisional_by_kb: dict[tuple, int] = defaultdict(int)
    for hit in provisional_hits:
        provisional_by_kb[(hit["kind"], hit["book"])] += 1

    per_kb_rows = []
    for (kind, book), counts in sorted(per_kb.items(), key=lambda kv: (kv[0][0] or "", kv[0][1] or "")):
        row = dict(counts)
        row["kind"] = kind
        row["book"] = book
        row["provisional_default"] = provisional_by_kb.get((kind, book), 0)
        row["row17_actionable"] = row["fallthrough"] + row["provisional_default"]
        per_kb_rows.append(row)

    f0b = ledger.get("f0_breakdown", {})
    totals = {
        "population": ledger["population"],
        "derived": sum(r["derived"] for r in per_kb_rows),
        "measured_empty": f0b.get("measured_empty", 0),
        # T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21) /
        # `decisions.md §27a`: a value that genuinely carries PI and stays
        # redacted is a REAL answer (measured, not placeholder) -- counted
        # here alongside `measured_empty`, excluded from `row17_honest_size`
        # below the same way. Never re-derive this by re-adding
        # `fallthrough_pi_redacted` to `fallthrough`: the two counts are
        # disjoint by construction (`shape_ledger.py::classify_unit`).
        "measured_pi_redacted": f0b.get("measured_pi_redacted", 0),
        "fallthrough": f0b.get("fallthrough", 0),
        "fallthrough_pi_redacted": ledger.get("f0_fallthrough_pi_redacted", 0),
        "not_ingested": f0b.get("not_ingested", 0),
        "provisional_default_total": len(provisional_hits),
        "provisional_default_in_not_done_population": len(provisional_in_population),
        "provisional_default_missing_reason": len(provisional_missing_reason),
    }
    totals["row17_honest_size"] = totals["fallthrough"] + totals["provisional_default_in_not_done_population"]

    return {
        "totals": totals,
        "per_kind_book": per_kb_rows,
        "provisional_default_hits": provisional_hits,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY)
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT)
    parser.add_argument("--output", help="write the full census as JSON to this path")
    parser.add_argument(
        "--check",
        action="store_true",
        help="exit 1 if any provisional-default marker is missing its required reason",
    )
    args = parser.parse_args(argv)

    census = build_census(args.inventory, args.corpus_root)
    t = census["totals"]

    print(f"row 17 census (decisions.md §27a/§27b, kanban.md row 17) — population {t['population']}")
    print()
    print(f"  derived (genuinely-classified, non-F0, or F0 by real measurement):")
    print(f"    derived (real family)         {t['derived']:>7}")
    print(f"    measured_empty (real F0)      {t['measured_empty']:>7}")
    print(f"    measured_pi_redacted (real,   {t['measured_pi_redacted']:>7}   -- genuinely PI, correctly redacted;")
    print(f"      cannot ship as formula)                  not row 17's placeholder population (§27a)")
    print()
    print(f"  row 17's actual population (placeholder / not genuinely derived):")
    print(f"    F0 by fallthrough             {t['fallthrough']:>7}   (of which PI-redacted formula: {t['fallthrough_pi_redacted']})")
    print(f"    §27 provisional default       {t['provisional_default_in_not_done_population']:>7}   (corpus-wide total incl. done units: {t['provisional_default_total']})")
    print(f"    ROW 17 HONEST SIZE            {t['row17_honest_size']:>7}")
    print()
    print(f"  excluded from row 17 (sequencing, decisions.md §27/§20):")
    print(f"    not_ingested (no_record)      {t['not_ingested']:>7}   -- row 17 starts only after this reaches 0")
    print()
    if t["provisional_default_missing_reason"]:
        print(
            f"  CONTRACT VIOLATION: {t['provisional_default_missing_reason']} provisional-default "
            "marker(s) found with no reason recorded (shape_provisional_marker's contract requires "
            "both together) -- see 'provisional_default_hits' in --output JSON"
        )

    if args.output:
        os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(census, fh, indent=2)
            fh.write("\n")

    if args.check and t["provisional_default_missing_reason"]:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
