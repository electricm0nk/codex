#!/usr/bin/env python3
"""AT-35-E2-005-DISPOSITION -- the unit-for-unit hand-off of AT-35-E2-005's remainder.

Re-derives, from `docs/work-inventory.json` (the atlas partition of
`scripts/completion_atlas.py`), `data/sheet_rules/_refused.json` and
`artifacts/epic-2-sheet-rule/token-coverage.json` at HEAD, which SD-35
criterion owns every non-DONE unit, and fails closed (exit 1) unless the
owned cells sum to the live non-DONE total with no unit in two cells and
no unit in none.

Owner rule (the disposition, `decisions.md §16`):
  refused by the converter (id in `_refused.json`)           -> AT-35-E4-001
  not refused, bucket V (literal-verified / fixture-verified) -> AT-35-E4-002
  not refused, bucket U (unmeasurable)                        -> AT-35-E5-003
  not refused, bucket Z (not-started)                         -> AT-35-E5-003
  not refused, bucket X (deferred-with-reason)                -> AT-35-E5-004
  not refused, bucket A / B / C / D / M                       -> AT-35-E5-001 / E3-001|E3-002 / E3-003 / E5-002 / E4-001
    (the bucket's own to-zero criterion in `epic-breakdown.md`; B splits by kind:
     `class_feature` -> AT-35-E3-001, every other kind -> AT-35-E3-002)

Run:  python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_handoff.py
      [--write <path.json>]
"""
import argparse
import collections
import json
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 5)))
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas  # noqa: E402  (the partition of record; never a second status list)

INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")
REFUSED = os.path.join(REPO, "data", "sheet_rules", "_refused.json")
LEDGER = os.path.join(REPO, "docs", "release", "SD-35-corpus-sheet-completion",
                      "artifacts", "epic-2-sheet-rule", "token-coverage.json")

BUCKET_OWNER = {"V": "AT-35-E4-002", "U": "AT-35-E5-003", "Z": "AT-35-E5-003",
                "X": "AT-35-E5-004", "A": "AT-35-E5-001", "C": "AT-35-E3-003",
                "D": "AT-35-E5-002", "M": "AT-35-E4-001"}


def owner_of(unit, refused_ids):
    bucket = completion_atlas._bucket_of(unit)
    if bucket == "DONE":
        return None, bucket
    if unit["id"] in refused_ids:
        return "AT-35-E4-001", bucket
    if bucket == "B":
        return ("AT-35-E3-001" if unit.get("kind") == "class_feature" else "AT-35-E3-002"), bucket
    if bucket in BUCKET_OWNER:
        return BUCKET_OWNER[bucket], bucket
    return "UNOWNED", bucket


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", help="write the hand-off table as JSON to this path")
    args = ap.parse_args(argv)

    inv = json.load(open(INVENTORY))
    units = inv["units"]
    refused = json.load(open(REFUSED))
    refused_entries = {e["id"]: e for e in refused["entries"]}
    ledger = json.load(open(LEDGER))

    cells = collections.Counter()          # (owner, bucket, status, refused?) -> n
    by_owner = collections.Counter()
    by_owner_kind = collections.defaultdict(collections.Counter)
    seen = set()
    dup = 0
    unowned = []
    refused_non_done_tokens = collections.Counter()   # token string -> non-DONE refused units (multiplicity)
    refused_non_done_ids = set()
    for u in units:
        if u["id"] in seen:
            dup += 1
        seen.add(u["id"])
        owner, bucket = owner_of(u, refused_entries)
        if owner is None:
            continue
        is_ref = u["id"] in refused_entries
        cells[(owner, bucket, u.get("status"), is_ref)] += 1
        by_owner[owner] += 1
        by_owner_kind[owner][u.get("kind")] += 1
        if owner == "UNOWNED":
            unowned.append(u["id"])
        if is_ref:
            refused_non_done_ids.add(u["id"])
            for t in refused_entries[u["id"]]["token_types"]:
                refused_non_done_tokens[t] += 1

    non_done_total = sum(by_owner.values())
    atlas_non_done = ledger["denominators"]["non_done"]
    ledger_refused_non_done = ledger["denominators"]["refused_non_done"]
    owned_sum = sum(v for k, v in by_owner.items() if k != "UNOWNED")

    # The ledger's own per-shape non-DONE counts (81 shapes) -- the "by token string"
    # view the receipts quote; shapes are the converter's refusal strings.
    shapes_non_done = {k: v["non_done"] for k, v in ledger["refusal_shapes"].items() if v["non_done"]}
    shapes_sorted = sorted(shapes_non_done.items(), key=lambda kv: (-kv[1], kv[0]))

    refused_class_all = sum(1 for e in refused_entries.values() if e["kind"] == "class")
    refused_class_non_done = sum(1 for u in units
                                 if u["id"] in refused_non_done_ids and u.get("kind") == "class")

    ok = (dup == 0 and not unowned and owned_sum == non_done_total == atlas_non_done
          and len(refused_non_done_ids) == ledger_refused_non_done)

    table = {
        "schema": "at-35-e2-005-disposition-handoff.v1",
        "derived_by": "python3 " + os.path.relpath(__file__, REPO),
        "head": completion_atlas._head_sha(),
        "denominators": {
            "units": len(units),
            "non_done": non_done_total,
            "atlas_non_done": atlas_non_done,
            "refused_records": len(refused_entries),
            "refused_non_done": len(refused_non_done_ids),
            "ledger_refused_non_done": ledger_refused_non_done,
            "not_refused_non_done": non_done_total - len(refused_non_done_ids),
        },
        "owner_rule": __doc__.split("Owner rule")[1].split("Run:")[0].strip(),
        "by_owner": dict(sorted(by_owner.items())),
        "by_owner_kind": {o: dict(sorted(c.items())) for o, c in sorted(by_owner_kind.items())},
        "cells": [
            {"owner": o, "bucket": b, "status": s, "refused": r, "units": n}
            for (o, b, s, r), n in sorted(cells.items(), key=lambda kv: (kv[0][0], kv[0][1], str(kv[0][2]), kv[0][3]))
        ],
        "refused_non_done_by_shape": dict(shapes_sorted),
        "refused_non_done_by_token_type_multiplicity": dict(sorted(refused_non_done_tokens.items(), key=lambda kv: (-kv[1], kv[0]))),
        "refused_class_records": {"all": refused_class_all, "non_done": refused_class_non_done},
        "sum_checks": {
            "duplicate_ids": dup,
            "unowned": len(unowned),
            "owned_sum": owned_sum,
            "non_done_total": non_done_total,
            "atlas_non_done": atlas_non_done,
            "refused_non_done_matches_ledger": len(refused_non_done_ids) == ledger_refused_non_done,
            "ok": ok,
        },
    }
    if args.write:
        with open(args.write, "w") as fh:
            json.dump(table, fh, indent=1, sort_keys=False)
            fh.write("\n")

    for (o, b, s, r), n in sorted(cells.items(), key=lambda kv: (kv[0][0], kv[0][1], str(kv[0][2]), kv[0][3])):
        print(f"cell owner={o} bucket={b} status={s} refused={r} units={n}")
    print("by_owner " + " ".join(f"{k}={v}" for k, v in sorted(by_owner.items())))
    print("refused_class_records all=%d non_done=%d" % (refused_class_all, refused_class_non_done))
    print("refused_non_done_by_shape " + ", ".join(f"{k}={v}" for k, v in shapes_sorted))
    print(f"non_done={non_done_total} atlas_non_done={atlas_non_done} refused_non_done={len(refused_non_done_ids)} "
          f"not_refused_non_done={non_done_total - len(refused_non_done_ids)} owned_sum={owned_sum} "
          f"unowned={len(unowned)} duplicate_ids={dup} verdict={'PASS' if ok else 'FAIL'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
