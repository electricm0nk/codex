#!/usr/bin/env python3
"""SD-33 remediation wave 5 (AT-33-E5-003) -- re-runs the FULL 66-unit
population `AT-33-E5-shape-combat`'s whole-character `AC.TOTAL` diff
already judged (40 agree + 26 disagree of that lane's own 82-item
manifest; the other 16 never got a numeric oracle value from the diff at
all, so were never "judged" by it), through the absolute-method isolator
(`ac-isolate.txt.ftl`'s `AC.ISOLATED` token: `BONUS.COMBAT.AC.TOTAL.
!BASE.!Ability.!Size`, live against the SAME already-committed single-
item `.pcg` fixtures under `combat-shape-work/ac-pcg/` -- no baseline
character needed at all).

Real, live PCGen invocations against the pinned oracle
(PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6, repo-local
checkout at $PCGEN_REPO_DIR, defaulting to $HOME/workspace/repos/pcgen).

Usage:
  ac_isolate_run.py <repo_root> <ac_manifest.json> <equipment-shape-combat.oracle-results.json>
                     <e5_ac_isolator.output.json> <out_dir> <full_out.json> <disagreement_fixes_out.json>
                     [--workers N]
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO_ROOT_MARKER = "scripts/oracle_harness/charbuild_remainder_run_one.sh"


def read_ac_isolated(path):
    if not os.path.exists(path):
        return None
    for line in open(path, encoding="utf-8"):
        line = line.strip()
        if line.startswith("AC.ISOLATED="):
            return line.split("=", 1)[1].strip()
    return None


def run_one(repo_root, ftl_path, settings_dir, pcg_path, out_path):
    runner = os.path.join(repo_root, REPO_ROOT_MARKER)
    proc = subprocess.run(
        ["bash", runner, pcg_path, ftl_path, out_path, settings_dir],
        cwd=repo_root,
        capture_output=True,
        text=True,
        timeout=120,
    )
    return proc.returncode, proc.stdout, proc.stderr


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("repo_root")
    ap.add_argument("ac_manifest")
    ap.add_argument("shape_combat_results")
    ap.add_argument("isolator_output")
    ap.add_argument("out_dir")
    ap.add_argument("full_out")
    ap.add_argument("disagreement_fixes_out")
    ap.add_argument("--workers", type=int, default=6)
    ap.add_argument("--only", nargs="*", default=None, help="restrict to these unit_ids (debug)")
    args = ap.parse_args()

    repo_root = os.path.abspath(args.repo_root)
    sys.path.insert(0, os.path.join(repo_root, "scripts"))
    from oracle_harness import compare as OC  # noqa: E402

    manifest = json.load(open(args.ac_manifest))
    items_by_id = {it["unit_id"]: it for it in manifest["items"]}

    shape_combat = json.load(open(args.shape_combat_results))
    verdict_by_id = {r["unit_id"]: r["verdict"] for r in shape_combat["results"]}
    already_judged = sorted(uid for uid, v in verdict_by_id.items() if v in ("agree", "disagree"))

    isolator = json.load(open(args.isolator_output))
    ours_by_id = {it["unit_id"]: it["ours"] for it in isolator["items"]}

    if args.only:
        already_judged = [u for u in already_judged if u in args.only]

    ftl_path = os.path.join(
        repo_root,
        "docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/ac-isolate.txt.ftl",
    )
    os.makedirs(args.out_dir, exist_ok=True)
    settings_dir = "/tmp/pcgen-run-settings-sd33-r5-disagreements"
    os.makedirs(settings_dir, exist_ok=True)

    jobs = []
    for uid in already_judged:
        item = items_by_id[uid]
        pcg_path = os.path.join(repo_root, item["pcg_path"])
        out_path = os.path.join(args.out_dir, f"{item['slug']}.txt")
        jobs.append((uid, item, pcg_path, out_path))

    results_meta = {}
    failed = []
    with ThreadPoolExecutor(max_workers=args.workers) as ex:
        futs = {
            ex.submit(run_one, repo_root, ftl_path, settings_dir, pcg_path, out_path): uid
            for (uid, item, pcg_path, out_path) in jobs
        }
        for fut in as_completed(futs):
            uid = futs[fut]
            try:
                rc, out, err = fut.result()
            except Exception as e:  # noqa: BLE001
                failed.append((uid, str(e)))
                continue
            if rc != 0:
                failed.append((uid, f"exit {rc}: {err[-500:]}"))

    full_results = []
    disagreement_fix_ids = {
        "advanced_class_guide:equipment:full_plate_of_the_corpse",
        "inner_sea_world_guide:equipment:field_plate",
        "inner_sea_world_guide:equipment:stoneplate",
        "ultimate_equipment:equipment:snakeskin_tunic",
    }
    disagreement_fix_rows = []
    unresolved_isolated = []
    prior_oracle_by_id = {r["unit_id"]: r.get("oracle") for r in shape_combat["results"]}

    for uid, item, pcg_path, out_path in jobs:
        isolated_str = read_ac_isolated(out_path)
        if isolated_str is None:
            unresolved_isolated.append(uid)
            continue
        oracle_value = int(isolated_str)
        ours_val = ours_by_id.get(uid)
        # Same convention `disagreement-fixes-manifest.json`'s own
        # `allow_none_ac` flag already established (AT-33-E5-003 wave 4):
        # `armor_class_bonus_from_bonus_chains` returns `None` when a
        # record's ONLY `COMBAT|AC` chain is `TYPE=Circumstance`
        # (excluded), which means "no chain applies" -- a real, honest
        # zero contribution, not an unresolved/unknown value. Comparing
        # a bare `None` against a numeric oracle would otherwise fall
        # into `compare_unit`'s string-equality branch (`"None" !=
        # "0"`) and manufacture a false `disagree` that has nothing to
        # do with this cycle's harness fix (confirmed this cycle:
        # `advanced_race_guide:equipment:sea_knife`, already agreed at
        # 0/0 by wave 4's own committed row).
        if ours_val is None:
            ours_val = 0
        rec = OC.compare_unit(uid, ours_val, oracle_value)
        rec["prior_diff_oracle"] = prior_oracle_by_id.get(uid)
        rec["method"] = "ac_isolated_bonus_by_type"
        full_results.append(rec)
        if uid in disagreement_fix_ids:
            disagreement_fix_rows.append(
                {"unit_id": uid, "ours": rec["ours"], "oracle": rec["oracle"], "verdict": rec["verdict"]}
            )

    json.dump({"results": full_results}, open(args.full_out, "w"), indent=2)
    open(args.full_out, "a").write("\n")
    json.dump({"results": disagreement_fix_rows}, open(args.disagreement_fixes_out, "w"), indent=2)
    open(args.disagreement_fixes_out, "a").write("\n")

    counts = {"agree": 0, "disagree": 0, "unverifiable": 0}
    for r in full_results:
        counts[r["verdict"]] += 1
    moved = [
        r["unit_id"]
        for r in full_results
        if r.get("prior_diff_oracle") is not None and r["oracle"] != r["prior_diff_oracle"]
    ]

    print(f"ac_isolate_run: {len(already_judged)} already-judged units, {len(failed)} run failures, "
          f"{len(unresolved_isolated)} unresolved isolated tokens")
    print(f"full re-run: {len(full_results)} rows -- agree={counts['agree']} disagree={counts['disagree']} "
          f"unverifiable={counts['unverifiable']}")
    print(f"oracle value moved vs prior diff-based oracle: {len(moved)} of {len(full_results)}")
    print(f"moved unit_ids: {moved}")
    print(f"disagreement-fix rows written: {len(disagreement_fix_rows)}")
    if failed:
        print(f"FAILED RUNS: {failed}")
    if unresolved_isolated:
        print(f"UNRESOLVED ISOLATED: {unresolved_isolated}")


if __name__ == "__main__":
    main()
