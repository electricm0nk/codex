#!/usr/bin/env python3
"""SD-35 operator ruling B18 (`decisions.md §21`): what the widened
`has_classifying_token` predicate admits, measured against the pinned corpus
before the fix is trusted.

The predicate `src/bin/v06_work_inventory.rs::has_classifying_token` used a
single classifying token per kind (`TYPE:` for a feat, `SCHOOL:`/`CLASSES:` for
a spell) as a proxy for "this row declares a record of its own". Ruling B18
widens it: the token **or** the row's own printable rule prose (a non-empty,
non-`.CLEAR` `DESC:`/`BENEFIT:`), per `row_carries_rule_prose`.

This script walks every `.lst` file in the pinned oracle checkout, applies the
OLD and the NEW predicate to each plain row, and reports the difference, so the
answer to "does this admit ten rows or ten thousand" is a measured number and
not a hope. It deliberately walks EVERY publisher, not only the in-scope books,
so an out-of-scope blow-up would still be visible.

    PCGEN_CORPUS_ROOT=... python3 .../widened_predicate_census.py

Writes `b18-widened-predicate-census.json` beside itself. Also writes
`b18-admitted-units.json` (the unit ids the regeneration actually added to
`docs/work-inventory.json`) when the regenerated inventory is on disk, which is
the input `b18_ten_render_proof.py` reads.
"""

from __future__ import annotations

import collections
import json
import os
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[4]


def file_kind(basename: str) -> str | None:
    """The two kinds whose arm ruling B18 changed, in `file_kind`'s own order.

    Every earlier arm of `src/bin/v06_work_inventory.rs::file_kind` that could
    claim one of these basenames first is reproduced as a `None`, so this walker
    never credits a row to `feat`/`spell` that the binary would classify
    otherwise.
    """
    if "_abilities_class" in basename or "_abilities_race" in basename:
        return None
    if "_abilities_companion" in basename or "_abilities_familiar" in basename:
        return None
    if "abilit" in basename:
        return None
    if "_races" in basename or "_classes" in basename:
        return None
    if "_feats" in basename:
        return "feat"
    if "_spells" in basename:
        return "spell"
    return None


def carries_rule_prose(fields: list[str]) -> bool:
    """`row_carries_rule_prose`, in Python. Kept byte-for-byte in step with the
    Rust original: a non-empty, non-`.CLEAR` `DESC:` or `BENEFIT:` field."""
    for field in fields:
        for prefix in ("DESC:", "BENEFIT:"):
            if field.startswith(prefix):
                value = field[len(prefix) :].strip()
                if value and not value.startswith(".CLEAR"):
                    return True
    return False


def main() -> int:
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if not root:
        print("PCGEN_CORPUS_ROOT is unset -- bootstrap it with scripts/fetch-pcgen-oracle.sh", file=sys.stderr)
        return 2
    base = pathlib.Path(root) / "pathfinder"

    admitted: list[dict] = []
    lost: list[dict] = []
    counts: collections.Counter[str] = collections.Counter()

    for path in sorted(base.rglob("*.lst")):
        kind = file_kind(path.name)
        if kind is None:
            continue
        with path.open(encoding="utf-8", errors="replace") as handle:
            for number, raw in enumerate(handle, 1):
                line = raw.rstrip("\n")
                if not line.strip() or line.lstrip().startswith("#"):
                    continue
                fields = [f for f in line.split("\t") if f.strip()]
                if not fields:
                    continue
                first = fields[0].strip()
                # `.FORGET` and `.MOD` rows are dispatched by `enumerate_file`
                # BEFORE `has_classifying_token` is consulted, so neither
                # predicate ever sees them and widening cannot admit one.
                if first.endswith(".FORGET") or ".MOD" in first:
                    continue
                if kind == "feat":
                    old = any(f.startswith("TYPE:") for f in fields)
                else:
                    old = any(f.startswith(("SCHOOL:", "CLASSES:")) for f in fields)
                new = old or carries_rule_prose(fields)
                counts[f"{kind}_rows"] += 1
                counts[f"{kind}_old_pass"] += int(old)
                counts[f"{kind}_new_pass"] += int(new)
                row = {
                    "kind": kind,
                    "file": str(path.relative_to(base)),
                    "line": number,
                    "name": first[:80],
                }
                if new and not old:
                    admitted.append(row)
                if old and not new:
                    lost.append(row)

    report = {
        "ruling": "SD-35 operator ruling B18 (decisions.md §21)",
        "criterion": "AT-35-E7-000-POPULATION-FIX",
        "corpus": str(base),
        "predicate_old": "feat: TYPE:  |  spell: SCHOOL: or CLASSES:",
        "predicate_new": "the same, OR a non-empty non-.CLEAR DESC:/BENEFIT: (row_carries_rule_prose)",
        "scope": "every publisher in the pinned checkout, not only the in-scope books",
        "counts": dict(sorted(counts.items())),
        "admitted_rows": len(admitted),
        "rows_no_longer_admitted": len(lost),
        "admitted_by_book": dict(
            collections.Counter(r["file"].split("/")[-2] for r in admitted).most_common()
        ),
        "admitted": admitted,
        "lost": lost,
    }
    (HERE / "b18-widened-predicate-census.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8"
    )

    print(json.dumps(report["counts"], indent=1))
    print(f"admitted_rows={len(admitted)} rows_no_longer_admitted={len(lost)}")
    for row in admitted:
        print(f"  + {row['kind']:5s} {row['file']}:{row['line']}  {row['name']}")

    # The unit ids the regeneration actually added, for the render proof. The
    # rows above are `.lst` rows; a row becomes a unit only after refine_kind,
    # book attribution and the duplicate-identity trap, so this is derived from
    # the inventories, never assumed to equal `admitted_rows`.
    before = os.environ.get("INVENTORY_BEFORE")
    inventory = REPO / "docs/work-inventory.json"
    if before and pathlib.Path(before).is_file() and inventory.is_file():
        old_ids = {u["id"] for u in json.loads(pathlib.Path(before).read_text(encoding="utf-8"))["units"]}
        new_ids = [u["id"] for u in json.loads(inventory.read_text(encoding="utf-8"))["units"]]
        added = sorted(set(new_ids) - old_ids)
        (HERE / "b18-admitted-units.json").write_text(
            json.dumps(
                {
                    "ruling": "SD-35 operator ruling B18 (decisions.md §21)",
                    "derived_from": "set(docs/work-inventory.json ids) - set($INVENTORY_BEFORE ids)",
                    "count": len(added),
                    "ids": added,
                },
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )
        print(f"admitted_units={len(added)} (written to b18-admitted-units.json)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
