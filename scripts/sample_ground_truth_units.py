#!/usr/bin/env python3
"""scripts/sample_ground_truth_units.py -- a real, runnable, seeded,
stratified sampler for FUTURE ground-truth-sample draws (Epic 2, SD31-E2-F1,
OPEN-ISSUES.md row 4).

WHY THIS EXISTS: neither the sampling script nor the evidence-extraction
scripts the SD31-E2-F1-ground-truth-methodology.md describes for the v1
draw were ever committed (`git diff --stat` vs. merge-base showed 6 files,
all `docs/`, zero `.py`/`.rs`). The v1 draw (`random.seed(31)`) is therefore
NOT REPRODUCIBLE and this script does not attempt to reconstruct it --a
reconstruction could not prove the original draw was unbiased, and
presenting one as the original would be worse than admitting the gap. This
script is for every draw FROM THIS CYCLE FORWARD: run it, commit its exact
invocation and output, and the draw it produces is auditable and
re-runnable by anyone.

WHAT IT DOES: draws additional candidate units from `docs/work-inventory.json`
(or a compatible corpus_literal_sweep-shaped document), stratified by
`(wiring_class, kind)`, targeting cells a caller names as under-covered,
excluding any id already present in an existing sample. It emits STUBS
ONLY -- id/kind/book/name/engine_wiring_class/engine_wiring_class_reason/
source_file/source_line/population -- never a `hand_wiring_class`, a
`token_evidence`, or any other verdict field. A human (or an agent acting
as one) reads each stub's real corpus record and fills in the hand-label
fields afterward; this script performs NO classification of its own,
consistent with Decision 1(e) item 1's bar on classifier code before F2.

Usage:
    python3 scripts/sample_ground_truth_units.py \\
        --inventory docs/work-inventory.json \\
        --exclude-ids-from EXISTING_SAMPLE.json \\
        --current-cell-counts CELL_COUNTS.json \\
        --target-per-cell 3 \\
        --seed 31 \\
        --out DRAW.json

`--current-cell-counts` is a JSON object `{"<wiring_class>:<kind>": <int>, ...}`
naming how many units the CALLER already has hand-labelled into each cell
today (computed from the existing sample's `hand_wiring_class`, since that
axis does not exist pre-draw) -- the script tops up the gap against
`--target-per-cell` using each unit's ENGINE `wiring_class` as the only
stratification signal available before a human reads the record (the same
constraint the original v1 draw necessarily worked under too).
"""
from __future__ import annotations

import argparse
import collections
import json
import random
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_INVENTORY = REPO_ROOT / "docs" / "work-inventory.json"


def load_inventory_units(path: str) -> list:
    with open(path, "r", encoding="utf-8") as fh:
        doc = json.load(fh)
    units = doc["units"] if isinstance(doc, dict) and "units" in doc else doc
    return [u for u in units if u.get("book") != "beginner_box"]


def load_excluded_ids(paths: list) -> set:
    excluded = set()
    for path in paths:
        with open(path, "r", encoding="utf-8") as fh:
            data = json.load(fh)
        if isinstance(data, list):
            for rec in data:
                if isinstance(rec, dict) and rec.get("id"):
                    excluded.add(rec["id"])
                elif isinstance(rec, str):
                    excluded.add(rec)
        elif isinstance(data, dict) and "units" in data:
            for rec in data["units"]:
                if rec.get("id"):
                    excluded.add(rec["id"])
    return excluded


def draw(
    units: list,
    current_cell_counts: dict,
    target_per_cell: int,
    excluded_ids: set,
    seed: int,
    zero_magnitude_only: bool = False,
) -> list:
    """Deterministic given (units, current_cell_counts, target_per_cell,
    excluded_ids, seed, zero_magnitude_only): groups eligible units by
    `(wiring_class, kind)`, shuffles each cell's candidate pool with a seeded
    RNG, and takes enough from the front of each shuffled pool to close the
    gap between the caller's current count and `target_per_cell` -- never
    more than the cell's real population supports.

    `zero_magnitude_only` (SD31-D7-PROSE-001, Decision 7's PROXY WARNING):
    restricts the candidate pool to `magnitude_token_count == 0` units before
    stratifying -- the exact population the `magnitude_token_count == 0`
    proxy decides, and the only population a sample validating that proxy
    may honestly draw from. `magnitude_token_count` is always carried on the
    emitted record (regardless of this flag) so a reader can see the raw
    proxy value that put a unit in the draw, independent of the engine's own
    `wiring_class` verdict.
    """
    rng = random.Random(seed)

    by_cell = collections.defaultdict(list)
    for u in units:
        if u.get("id") in excluded_ids:
            continue
        if zero_magnitude_only and u.get("magnitude_token_count") != 0:
            continue
        cell = f"{u.get('wiring_class')}:{u.get('kind')}"
        by_cell[cell].append(u)

    drawn = []
    for cell, pool in sorted(by_cell.items()):
        have = current_cell_counts.get(cell, 0)
        need = max(0, target_per_cell - have)
        if need == 0:
            continue
        pool_sorted = sorted(pool, key=lambda u: u["id"])  # deterministic pre-shuffle order
        rng.shuffle(pool_sorted)
        take = pool_sorted[:need]
        for u in take:
            drawn.append(
                {
                    "id": u["id"],
                    "kind": u.get("kind"),
                    "book": u.get("book"),
                    "name": u.get("name"),
                    "engine_wiring_class": u.get("wiring_class"),
                    "engine_wiring_class_reason": u.get("wiring_class_reason"),
                    "engine_status": u.get("status"),
                    "magnitude_token_count": u.get("magnitude_token_count"),
                    "corpus_key": u.get("corpus_key"),
                    "source_file": u.get("source_file"),
                    "source_line": u.get("source_line"),
                    "population": "widening_batch_v2",
                    "cell": cell,
                }
            )

    return drawn


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--inventory", default=str(DEFAULT_INVENTORY))
    parser.add_argument(
        "--exclude-ids-from",
        action="append",
        default=[],
        help="a ground-truth-sample-shaped JSON file (or several, repeat the flag) "
        "whose ids are excluded from the draw",
    )
    parser.add_argument(
        "--current-cell-counts",
        required=True,
        help="JSON object {'<wiring_class>:<kind>': <int>} of the caller's "
        "current hand-labelled coverage per cell",
    )
    parser.add_argument("--target-per-cell", type=int, default=3)
    parser.add_argument("--seed", type=int, default=31)
    parser.add_argument(
        "--zero-magnitude-only",
        action="store_true",
        help="restrict the draw to magnitude_token_count==0 units (Decision 7 proxy validation)",
    )
    parser.add_argument("--out", required=True)
    args = parser.parse_args(argv)

    units = load_inventory_units(args.inventory)
    excluded = load_excluded_ids(args.exclude_ids_from)
    with open(args.current_cell_counts, "r", encoding="utf-8") as fh:
        current_cell_counts = json.load(fh)

    result = draw(
        units,
        current_cell_counts,
        args.target_per_cell,
        excluded,
        args.seed,
        zero_magnitude_only=args.zero_magnitude_only,
    )

    with open(args.out, "w", encoding="utf-8") as fh:
        json.dump(result, fh, indent=2)

    by_cell = collections.Counter(r["cell"] for r in result)
    print(f"sample_ground_truth_units: seed={args.seed} target_per_cell={args.target_per_cell}")
    print(f"  drew {len(result)} unit(s) across {len(by_cell)} cell(s):")
    for cell, n in sorted(by_cell.items()):
        print(f"    {cell}: {n}")
    print(f"  written to {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
