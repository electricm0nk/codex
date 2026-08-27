#!/usr/bin/env python3
"""AT-33-E5-finalize-wave3 merge.

Merges wave-3's three new equipment shape lanes (VAR, COMBAT,
stat/save/situation/tail) into the canonical `literal-verified.oracle-results.json`,
resolving the 15 real unit_id overlaps this cycle found (a single equipment
unit carrying multiple magnitude tokens/bonus-chain shapes -- e.g. an armor
item with both a `VAR|ArmorCheckPenalty` chain and a `COMBAT|AC` chain -- was
independently examined by more than one shape lane for a DIFFERENT dimension
of the same unit, producing more than one row for the same unit_id).

Merge rule for a duplicate unit_id (never last-writer-wins):
  - if ANY source row for the unit is 'disagree', the merged row is
    'disagree' (a unit with any wrong computed dimension is not verified
    correct as a whole)
  - else if ANY source row is 'unverifiable', the merged row is
    'unverifiable' (we cannot claim full agreement when one dimension was
    not checkable)
  - else (all sources 'agree') the merged row is 'agree'
The merged row's own ours/oracle/reason come from the first row matching the
winning verdict; ALL source rows are preserved verbatim under
`multi_shape_sources` so no information is discarded.
"""
import json
import sys

BASE = "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification"

VERDICT_RANK = {"disagree": 0, "unverifiable": 1, "agree": 2}


def load(fname):
    with open(f"{BASE}/{fname}") as f:
        return json.load(f)["results"]


def main():
    sources = [
        ("literal-verified", load("literal-verified.oracle-results.json")),
        ("var-bonus-shape", load("equipment-shape-var.oracle-results.json")),
        ("combat-weapon-shape", load("equipment-shape-combat.oracle-results.json")),
        ("stat-save-situation-tail", load("equipment-shape-stat-save-tail.oracle-results.json")),
    ]

    by_unit = {}
    for lane, rows in sources:
        for r in rows:
            by_unit.setdefault(r["unit_id"], []).append((lane, r))

    merged = []
    duplicate_report = []
    for unit_id, entries in by_unit.items():
        if len(entries) == 1:
            lane, row = entries[0]
            merged.append(row)
            continue

        # Real duplicate: more than one lane produced a row for this unit_id.
        duplicate_report.append({
            "unit_id": unit_id,
            "sources": [{"lane": lane, **row} for lane, row in entries],
        })
        best = min(entries, key=lambda le: VERDICT_RANK[le[1]["verdict"]])
        best_lane, best_row = best
        merged_row = dict(best_row)
        merged_row["multi_shape_sources"] = [
            {"lane": lane, "ours": row.get("ours"), "oracle": row.get("oracle"),
             "verdict": row["verdict"], "reason": row.get("reason")}
            for lane, row in entries
        ]
        merged_row["multi_shape_note"] = (
            f"unit carries {len(entries)} independently-examined magnitude/bonus-chain "
            f"shapes (multi-token equipment record); merged verdict is the worst of the "
            f"{len(entries)} per-shape verdicts, per AT-33-E5-finalize-wave3's duplicate-unit_id "
            f"root-cause rule (never last-writer-wins)"
        )
        merged.append(merged_row)

    with open(f"{BASE}/literal-verified.oracle-results.json", "w") as f:
        json.dump({"results": merged}, f, indent=2, sort_keys=True)
        f.write("\n")

    with open(f"{BASE}/finalize-wave3-duplicate-unit-ids.json", "w") as f:
        json.dump({"duplicates": duplicate_report}, f, indent=2, sort_keys=True)
        f.write("\n")

    print("merged rows:", len(merged))
    print("duplicate unit_ids found and resolved:", len(duplicate_report))
    from collections import Counter
    print(Counter(r["verdict"] for r in merged))


if __name__ == "__main__":
    main()
