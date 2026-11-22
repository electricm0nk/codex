#!/usr/bin/env python3
"""AT-33-E5-finalize-wave4 merge.

Merges wave-4's two lanes into the three canonical Epic-5 result files:

  1. `disagreement-fixes.oracle-results.json` (22 rows, all `agree`) --
     SUPERSEDES the matching stale `disagree` row for the same `unit_id` in
     `literal-verified.oracle-results.json` and `AT-33-E5-003.combined-oracle-results.json`.
     This is the ONE sanctioned overwrite (per the dispatch brief): the
     disagreement lane re-examined these exact units and produced a corrected
     `ours` value via a real engine fix, not a relabeling.
  2. `equipment-last75.oracle-results.json` (8 rows) -- pure ADDITION. None of
     these 8 unit_ids exist in any canonical file before this merge (verified
     below; the merge aborts loudly if that ever stops being true).

`fixture-verified.combined-oracle-results.json` is untouched -- neither lane
produced a row for any unit in the fixture-verified population (verified
below).

Any OTHER duplicate unit_id this script encounters (an id appearing more than
once within a single source file, or a "new" row that turns out to already
exist) is NOT resolved by last-writer-wins -- the script raises and refuses to
write, per `AGENTS.md` git/merge discipline and the dispatch brief's explicit
instruction.
"""
import json
import sys
import collections

BASE = "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification"


def load(fname):
    with open(f"{BASE}/{fname}") as f:
        return json.load(f)


def ids(rows):
    return [r["unit_id"] for r in rows]


def assert_no_internal_dupes(name, rows):
    c = collections.Counter(ids(rows))
    dupes = {k: v for k, v in c.items() if v > 1}
    if dupes:
        raise SystemExit(f"REFUSING TO MERGE: {name} has internal duplicate unit_ids: {dupes}")


def merge_one(canonical_fname, dis_rows, dis_ids, last75_rows, last75_ids, population, label):
    doc = load(canonical_fname)
    rows = doc["results"]
    assert_no_internal_dupes(canonical_fname, rows)

    existing_ids = set(ids(rows))

    # last75 rows must be genuinely new to THIS file.
    overlap = last75_ids & existing_ids
    if overlap:
        raise SystemExit(
            f"REFUSING TO MERGE: {canonical_fname} already contains last75 unit_ids "
            f"{overlap} -- last-writer-wins is forbidden, root-cause this instead."
        )

    # dis rows supersede a matching row IFF one exists in this file.
    dis_here = [r for r in dis_rows if r["unit_id"] in existing_ids]
    dis_here_ids = {r["unit_id"] for r in dis_here}

    merged = [r for r in rows if r["unit_id"] not in dis_here_ids]
    merged.extend(dis_here)
    merged.extend(last75_rows)

    merged_ids = ids(merged)
    if len(merged_ids) != len(set(merged_ids)):
        c = collections.Counter(merged_ids)
        dupes = {k: v for k, v in c.items() if v > 1}
        raise SystemExit(f"REFUSING TO WRITE {canonical_fname}: post-merge duplicates {dupes}")

    with open(f"{BASE}/{canonical_fname}", "w") as f:
        json.dump({"results": merged}, f, indent=2, sort_keys=True)
        f.write("\n")

    verdicts = collections.Counter(r["verdict"] for r in merged)
    print(
        f"{label}: rows={len(merged)} distinct={len(set(merged_ids))} population={population} "
        f"superseded={len(dis_here_ids)} added={len(last75_rows)} verdicts={dict(verdicts)}"
    )
    return merged, dis_here_ids


def main():
    dis_rows = load("disagreement-fixes.oracle-results.json")["results"]
    last75_rows = load("equipment-last75.oracle-results.json")["results"]
    assert_no_internal_dupes("disagreement-fixes.oracle-results.json", dis_rows)
    assert_no_internal_dupes("equipment-last75.oracle-results.json", last75_rows)

    dis_ids = set(ids(dis_rows))
    last75_ids = set(ids(last75_rows))
    overlap = dis_ids & last75_ids
    if overlap:
        raise SystemExit(f"REFUSING TO MERGE: dis_ids and last75_ids overlap: {overlap}")

    # fixture-verified must be untouched by both lanes -- verify, don't assume.
    fix_rows = load("fixture-verified.combined-oracle-results.json")["results"]
    fix_ids = set(ids(fix_rows))
    fix_hit = (dis_ids | last75_ids) & fix_ids
    if fix_hit:
        raise SystemExit(f"UNEXPECTED: fixture-verified population touched by wave-4 lanes: {fix_hit}")
    print(f"fixture-verified.combined-oracle-results.json: UNTOUCHED, rows={len(fix_rows)} (verified 0 overlap)")

    merge_one(
        "literal-verified.oracle-results.json",
        dis_rows, dis_ids, last75_rows, last75_ids,
        population=6589, label="literal-verified",
    )
    merge_one(
        "AT-33-E5-003.combined-oracle-results.json",
        dis_rows, dis_ids, last75_rows, last75_ids,
        population=8330, label="combined (AT-33-E5-003)",
    )


if __name__ == "__main__":
    main()
