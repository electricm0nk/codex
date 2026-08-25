#!/usr/bin/env python3
"""Check `THE-BOX.md`'s partition of `docs/work-inventory.json` against the
live inventory: prove -- structurally, not by assertion -- that every unit
belongs to exactly one named group.

Why this exists
----------------
`decisions.md` §1 (SD-33): SD-32 inherited a *citation* to SD-31's
`THE-BOX.md` and never rebuilt it -- five backward-pointing references, one
of them an anecdote inside a lessons list. The bundle's goal stopped being a
thing anyone had to update. This script is the mechanism that makes
`THE-BOX.md` a living partition rather than a document: it reads the
group definitions straight out of the committed markdown (a fenced
` ```json ledger ` block) and the full 49,438-unit population straight out
of `docs/work-inventory.json`, and fails closed if the two disagree.

`AT-33-E1-001` is the only criterion this cycle implements. It requires:

    uncovered == 0   -- no unit belongs to zero groups
    overlap == 0     -- no unit belongs to more than one group
    population stated by execution against docs/work-inventory.json,
        never trusted from the file's own `totals.units` field

`AT-33-E1-002` (a later cycle, same file) adds the remaining four fail-closed
conditions named in `decisions.md` §1 (oracle disagreement, an
`unverifiable` unit dispositioned `done`, and the `derived_at` staleness
gate). This script deliberately does not implement those yet -- there is
nothing here pretending to check them.

`THE-BOX.md` shape this script depends on
------------------------------------------
Exactly one fenced code block opened with the literal marker
` ```json ledger ` (not plain ` ```json `, so a document that also wants to
show *other* JSON snippets in prose does not collide) containing:

    {"groups": [
        {"id": ..., "disposition": ..., "count": ..., "match": {...},
         "command": "..."},
        ...
    ]}

`match` is a dict of `unit field -> required value`; a unit belongs to a
group when every key in `match` equals the unit's value for that key
(AND semantics). Every group in `THE-BOX.md` today matches on `status`
alone (the inventory's `status` field is already a clean 9-way partition of
the full population -- confirmed by execution, see `THE-BOX.md`'s own
per-group re-derive commands), but the matcher is not hardcoded to
`status` so a later cycle can add a group that discriminates on more than
one field without changing this parser.
"""

import argparse
import json
import os
import re
import sys
from collections import Counter, namedtuple

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_BOX = os.path.join(
    REPO_ROOT, "docs", "release", "SD-33-computed-value-verification", "THE-BOX.md"
)

LEDGER_BLOCK_RE = re.compile(r"```json ledger\s*\n(.*?)\n```", re.DOTALL)

PartitionResult = namedtuple("PartitionResult", ["uncovered", "overlap", "population", "membership"])


def load_inventory(path):
    """Return (units, population). `population` is `len(units)`, computed
    live -- never read from the file's own `totals.units` field, which is a
    stated claim, not a measurement."""
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    units = data["units"]
    return units, len(units)


def load_box(path):
    """Return the list of group dicts parsed out of THE-BOX.md's fenced
    ```json ledger block. Raises ValueError if the block is absent or the
    JSON it contains has no top-level "groups" list."""
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    m = LEDGER_BLOCK_RE.search(text)
    if not m:
        raise ValueError(
            f"{path}: no ```json ledger fenced block found -- THE-BOX.md "
            "must carry exactly one machine-readable ledger block"
        )
    block = json.loads(m.group(1))
    groups = block.get("groups")
    if not isinstance(groups, list) or not groups:
        raise ValueError(f"{path}: ledger block has no non-empty 'groups' list")
    return groups


def _matches(unit, match):
    return all(unit.get(field) == value for field, value in match.items())


def partition(units, groups):
    """Compute uncovered/overlap against the live units, and the live
    per-group membership count (independent of any count each group
    *states* it has)."""
    uncovered = []
    overlap = []
    membership = Counter()
    for unit in units:
        hits = [g["id"] for g in groups if _matches(unit, g.get("match", {}))]
        if len(hits) == 0:
            uncovered.append(unit["id"])
        elif len(hits) > 1:
            overlap.append(unit["id"])
        for gid in hits:
            membership[gid] += 1
    return PartitionResult(uncovered=uncovered, overlap=overlap, population=len(units), membership=membership)


def stated_count_mismatches(groups, membership):
    """Groups whose THE-BOX.md-stated `count` disagrees with the live
    recomputed membership -- reported, not gated, in this cycle (see module
    docstring: staleness/disagreement gating is AT-33-E1-002)."""
    mismatches = []
    for g in groups:
        stated = g.get("count")
        live = membership.get(g["id"], 0)
        if stated is not None and stated != live:
            mismatches.append((g["id"], stated, live))
    return mismatches


def run_check(inventory_path, box_path, out=sys.stdout):
    units, population = load_inventory(inventory_path)
    groups = load_box(box_path)
    result = partition(units, groups)

    mismatches = stated_count_mismatches(groups, result.membership)
    for gid, stated, live in mismatches:
        print(
            f"WARNING: group '{gid}' states count={stated} but live "
            f"recomputation from {inventory_path} finds {live} "
            "(THE-BOX.md needs re-deriving)",
            file=out,
        )

    print(
        f"uncovered={len(result.uncovered)} overlap={len(result.overlap)} "
        f"population={result.population}",
        file=out,
    )

    if result.uncovered:
        sample = ", ".join(result.uncovered[:10])
        more = f" (+{len(result.uncovered) - 10} more)" if len(result.uncovered) > 10 else ""
        print(f"UNCOVERED: {sample}{more}", file=out)
    if result.overlap:
        sample = ", ".join(result.overlap[:10])
        more = f" (+{len(result.overlap) - 10} more)" if len(result.overlap) > 10 else ""
        print(f"OVERLAP: {sample}{more}", file=out)

    return 0 if (not result.uncovered and not result.overlap) else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    # `--check` is accepted (it is the invocation AT-33-E1-001 specifies) but
    # is currently the only mode this tool has, so it does not branch
    # behaviour -- there is no second mode to select between yet.
    parser.add_argument("--check", action="store_true", help="run the partition check (the only mode this tool has)")
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY, help="path to docs/work-inventory.json")
    parser.add_argument("--box", default=DEFAULT_BOX, help="path to THE-BOX.md")
    args = parser.parse_args(argv)

    return run_check(args.inventory, args.box)


if __name__ == "__main__":
    sys.exit(main())
