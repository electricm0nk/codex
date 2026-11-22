#!/usr/bin/env python3
"""SD-32 `generic-ledger-rerun` cycle — re-derive script (measurement only).

WHY THIS EXISTS
----------------
The dispatch brief for this cycle (`docs/release/SD-32-compute-library-and-cause-closure/
decisions.md §17` item 3: "re-run the shape ledger over everything") asked for the honest current
state of the shape ledger, card 15's reconciliation, and card 11's five sub-populations
(T2b/T9/T12/T2a-residual/T4-L9), each with a re-derivable command — never a bare, hand-copied
total (`decisions.md §12c`). This script is that command: it re-runs `shape_ledger.py` and
`card15_reconcile.py` live and re-derives T2a/T12/their overlap directly against
`data/corpus/**/class_feature/**/*.json` and `docs/work-inventory.json`, exactly the same joins
`epic-2-t2a-t12_cycle-1_cycle_receipt.md` used, so a later cycle can re-run this file verbatim
instead of re-typing the ad hoc one-liners this memo's authoring session ran by hand.

It changes nothing: no corpus data, no `docs/work-inventory.json`, no pinned count, no engine code.
It only reads and prints.

USAGE
-----
    PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/generic_pass_state_rederive.py [--json OUT]

Requires `docs/work-inventory.json` and `data/corpus/**` to already be populated by the real
producer (`v06_work_inventory`) and generators — this script does not run them.
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import subprocess
import sys
from collections import Counter

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DISPATCHED_CLASSES = [
    "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue",
    "Sorcerer", "Wizard", "Arcanist", "Bloodrager", "Brawler", "Hunter", "Investigator", "Shaman",
    "Skald", "Slayer", "Swashbuckler", "Warpriest", "Alchemist", "Cavalier", "Inquisitor",
    "Oracle", "Summoner", "Witch", "Gunslinger", "Ninja", "Samurai", "Unchained Barbarian",
    "Unchained Monk", "Unchained Rogue", "Unchained Summoner",
]
_DISPATCHED_LOWER = [d.lower() for d in DISPATCHED_CLASSES]


def _is_dispatched(value: str) -> bool:
    v = value.strip().lower()
    return any(v == d or v.startswith(d + " ") or v.endswith(" " + d) for d in _DISPATCHED_LOWER)


def derive_t2a_t12(inventory_path: str, corpus_root: str) -> dict:
    """Re-derive |T2a|, |T12|, |T2a n T12|, |T2a u T12| exactly as
    `epic-2-t2a-t12_cycle-1_cycle_receipt.md` did, against whatever `docs/work-inventory.json` and
    `data/corpus/**/class_feature/**` currently contain."""
    with open(inventory_path) as f:
        wi = json.load(f)

    t12_keys = {
        u["corpus_key"]
        for u in wi["units"]
        if (u.get("evidence") or "").startswith("class_feature_of_unmodelled_corpus_class")
    }
    t12_count = sum(
        1
        for u in wi["units"]
        if (u.get("evidence") or "").startswith("class_feature_of_unmodelled_corpus_class")
    )

    total = nn = disp = 0
    class_by_key: dict[str, str | None] = {}
    for p in glob.glob(os.path.join(corpus_root, "*/class_feature/**/*.json"), recursive=True):
        if os.path.basename(p).startswith("manifest"):
            continue
        try:
            with open(p) as f:
                d = json.load(f)
        except (OSError, json.JSONDecodeError):
            continue
        data = d.get("data")
        if not isinstance(data, dict):
            continue
        total += 1
        c = data.get("class")
        if data.get("key"):
            class_by_key[data["key"]] = c
        if c is None:
            continue
        nn += 1
        if _is_dispatched(c):
            disp += 1
    t2a = nn - disp

    joined = non_disp = 0
    for k in t12_keys:
        if k in class_by_key:
            joined += 1
            c = class_by_key[k]
            if c is None or not _is_dispatched(c):
                non_disp += 1

    return {
        "class_feature_corpus_records_total": total,
        "class_feature_non_null_class": nn,
        "class_feature_dispatched_class": disp,
        "T2a (non-dispatched data.class)": t2a,
        "T12 (evidence=class_feature_of_unmodelled_corpus_class, docs/work-inventory.json)": t12_count,
        "T12_distinct_corpus_keys": len(t12_keys),
        "T12_keys_joined_to_corpus": joined,
        "T2a_n_T12 (T12 keys whose current data.class is non-dispatched)": non_disp,
        "T2a_residual (T2a - T2a_n_T12)": t2a - non_disp,
        "T2a_u_T12": t2a + t12_count - non_disp,
    }


def run_shape_ledger(inventory_path: str, corpus_root: str, out_path: str) -> dict:
    subprocess.run(
        [
            sys.executable,
            os.path.join(REPO_ROOT, "scripts", "shape_ledger.py"),
            "--inventory", inventory_path,
            "--corpus-root", corpus_root,
            "--output", out_path,
        ],
        check=True,
        cwd=REPO_ROOT,
    )
    with open(out_path) as f:
        ledger = json.load(f)
    rows = ledger["rows"]
    join_status = Counter(r["join_status"] for r in rows)
    families = Counter(r["family"] for r in rows)
    return {
        "population": len(rows),
        "unclassified_count": sum(1 for r in rows if r["family"] is None),
        "join_status_counts": dict(join_status),
        "family_counts": dict(sorted(families.items())),
    }


def run_gate3(inventory_path: str, corpus_root: str) -> dict:
    proc = subprocess.run(
        [
            sys.executable,
            os.path.join(REPO_ROOT, "scripts", "shape_coverage_standing_gate.py"),
            "--inventory", inventory_path,
            "--corpus-root", corpus_root,
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    return {
        "exit_code": proc.returncode,
        "PASS": proc.returncode == 0,
        "stdout": proc.stdout.strip(),
        "stderr_tail": proc.stderr.strip().splitlines()[-1] if proc.stderr.strip() else "",
    }


def run_card15_reconcile(corpus_root: str, inventory_path: str, out_path: str) -> dict:
    subprocess.run(
        [
            sys.executable,
            os.path.join(REPO_ROOT, "scripts", "card15_reconcile.py"),
            "--pcgen-root", corpus_root,
            "--inventory", inventory_path,
            "--output", out_path,
        ],
        check=True,
        cwd=REPO_ROOT,
    )
    with open(out_path) as f:
        rec = json.load(f)
    pops = rec["populations"]
    disp = rec["disposition_status_of_kind_unenumerable"]
    return {
        "census_tracked_kind_population": pops["census_tracked_kind_population"]["value"],
        "census_kind_unenumerable_population": pops["census_kind_unenumerable_population"]["value"],
        "inventory_all_units_population": pops["inventory_all_units_population"]["value"],
        "ledger_not_done_population": pops["ledger_not_done_population"]["value"],
        "total_this_run": disp["total_this_run"],
        "remaining_undisposed": disp["arithmetic_check"]["remaining_undisposed"],
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--inventory", default=os.path.join(REPO_ROOT, "docs/work-inventory.json"))
    ap.add_argument(
        "--corpus-root",
        default=os.path.join(REPO_ROOT, "data/corpus"),
        help="the repo's OWN generated corpus (data/corpus), NOT the PCGen oracle -- shape_ledger.py/"
        "shape_coverage_standing_gate.py/card15_reconcile.py join against generated records here. "
        "Deliberately does not default from $PCGEN_CORPUS_ROOT (that's the oracle, a different tree).",
    )
    ap.add_argument("--pcgen-root", default=os.environ.get("PCGEN_CORPUS_ROOT"), help="oracle root for shape_ledger/gate/card15 (defaults to $PCGEN_CORPUS_ROOT)")
    ap.add_argument("--json", default=None, help="write the full combined report here")
    ap.add_argument("--scratch-dir", default="/tmp")
    args = ap.parse_args()

    pcgen_root = args.pcgen_root or os.environ.get("PCGEN_CORPUS_ROOT")
    if not pcgen_root:
        print("ERROR: no PCGEN_CORPUS_ROOT / --pcgen-root given; shape_ledger/gate3/card15 need the pinned oracle.", file=sys.stderr)
        return 2

    report: dict = {}
    report["t2a_t12"] = derive_t2a_t12(args.inventory, args.corpus_root)
    report["shape_ledger"] = run_shape_ledger(
        args.inventory, args.corpus_root, os.path.join(args.scratch_dir, "generic_pass_ledger_now.json")
    )
    report["gate_3"] = run_gate3(args.inventory, args.corpus_root)
    report["card15_reconcile"] = run_card15_reconcile(
        pcgen_root, args.inventory, os.path.join(args.scratch_dir, "generic_pass_15_reconcile_now.json")
    )

    print(json.dumps(report, indent=2))
    if args.json:
        with open(args.json, "w") as f:
            json.dump(report, f, indent=2)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
