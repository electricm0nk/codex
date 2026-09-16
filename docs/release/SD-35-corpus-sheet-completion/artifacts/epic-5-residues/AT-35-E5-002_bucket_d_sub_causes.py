#!/usr/bin/env python3
"""Re-derive bucket D's sub-cause census at every inventory-moving commit of SD-35.

AT-35-E5-002's Evidence sentence is "D at 0; every sub-cause named with its mechanism
and count" (`epic-breakdown.md ### AT-35-E5-002`), and
`acceptance-and-verification.md` row AT-35-E5-002 names the artifact as "receipts".
This script is the command behind every figure in
`AT-35-E5-002_cycle1_receipt.md`: it reads each historical
`docs/work-inventory.json` out of git and partitions it with the LIVE
`scripts/completion_atlas.py` bucket rule, so every column is one lens applied to
every state (a historical atlas would be a different lens per column).

Usage:
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/AT-35-E5-002_bucket_d_sub_causes.py
    ... --json          machine-readable
    ... --at <sha>      one state only

Exit 0 when bucket D is 0 at HEAD; exit 1 otherwise.
"""

import argparse
import collections
import json
import os
import subprocess
import sys
import tempfile

REPO_ROOT = os.path.abspath(
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..")
)
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import completion_atlas as atlas  # noqa: E402

# The inventory-moving commits of SD-35, oldest first, re-derivable with:
#   git log --format='%h %s' 4c6c57eb9f..HEAD -- docs/work-inventory.json
STATES = [
    ("4c6c57eb9f", "tranche/15 cut — bundle launch state"),
    ("51f91bba11", "AT-35-E2-005 — first corpus-wide conversion"),
    ("406003afc3", "AT-35-E3-001 cycle 2 — term-level degradation replaces record refusal"),
    ("26bdfa8d5b", "AT-35-E3-002 — the whole remainder to DONE"),
    ("9bae2cfa1f", "AT-35-E4-001 — 24 mapping rows + 1 head alias"),
    ("HEAD", "HEAD"),
]


def units_at(sha):
    blob = subprocess.run(
        ["git", "-C", REPO_ROOT, "show", f"{sha}:docs/work-inventory.json"],
        capture_output=True, text=True, check=True,
    ).stdout
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
        fh.write(blob)
        path = fh.name
    try:
        return atlas._load_inventory(path)["units"]
    finally:
        os.unlink(path)


def census(sha):
    units = units_at(sha)
    counts = atlas.partition(units)["counts"]
    d_units = [u for u in units if atlas._bucket_of(u) == "D"]
    return {
        "sha": sha,
        "population": len(units),
        "buckets": {k: counts.get(k, 0) for k in
                    ["DONE", "A", "B", "C", "D", "M", "V", "U", "X", "Z"]},
        "d_total": len(d_units),
        "d_by_kind": dict(collections.Counter(u.get("kind") for u in d_units).most_common()),
        "d_by_evidence": dict(
            collections.Counter(u.get("evidence") for u in d_units).most_common()),
    }


def _family(evidence):
    """Collapse the 58 per-class `...:<class>` strings into one named family."""
    if (evidence or "").startswith("class_feature_of_unmodelled_corpus_class:"):
        return "class_feature_of_unmodelled_corpus_class:*"
    return evidence


def transitions(from_sha="4c6c57eb9f", to_sha="HEAD"):
    """Per bucket-D sub-cause family at `from_sha`, where each unit stands at `to_sha`.

    This is the mechanism half of AT-35-E5-002's Evidence sentence: a sub-cause is
    not "gone" because its evidence string stopped being emitted, it is gone because
    each of its units carries a rendered sheet line at HEAD.
    """
    src = units_at(from_sha)
    dst = {u.get("id"): u for u in units_at(to_sha)}
    d_units = [u for u in src if atlas._bucket_of(u) == "D"]
    totals = collections.Counter()
    dests = collections.defaultdict(collections.Counter)
    for u in d_units:
        fam = _family(u.get("evidence"))
        totals[fam] += 1
        h = dst.get(u.get("id"))
        key = (h.get("status"), h.get("evidence")) if h else ("MISSING_AT_HEAD", None)
        dests[fam][key] += 1
    return totals, dests


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--at")
    ap.add_argument("--transitions", action="store_true",
                    help="cut-state bucket-D sub-causes -> their HEAD status/evidence")
    args = ap.parse_args(argv)

    if args.transitions:
        totals, dests = transitions()
        print(f"families={len(totals)} d_units={sum(totals.values())}")
        for fam, n in totals.most_common():
            print(f"{n:>5}  {fam}")
            for (status, ev), k in dests[fam].most_common():
                print(f"        -> {k:>5}  {status} / {ev}")
        lost = sum(k for fam in dests for (st, _), k in dests[fam].items()
                   if st != "sheet-complete")
        print(f"not_sheet_complete_at_HEAD={lost} "
              f"verdict={'PASS' if lost == 0 else 'FAIL'}")
        return 0 if lost == 0 else 1

    states = [(args.at, "requested")] if args.at else STATES
    rows = [census(sha) for sha, _ in states]

    if args.json:
        print(json.dumps({"states": rows}, indent=2, sort_keys=True))
    else:
        for (sha, label), row in zip(states, rows):
            print(f"== {sha}  {label}")
            print(f"   population={row['population']} D={row['d_total']}  "
                  + " ".join(f"{k}={v}" for k, v in row["buckets"].items()))
            for kind, n in row["d_by_kind"].items():
                print(f"     kind {kind:<16} {n}")
            for ev, n in row["d_by_evidence"].items():
                print(f"     evidence {n:>5}  {ev}")
            print()

    head_d = rows[-1]["d_total"]
    print(f"bucket_D_at_HEAD={head_d} verdict={'PASS' if head_d == 0 else 'FAIL'}")
    return 0 if head_d == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
