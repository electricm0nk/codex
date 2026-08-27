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

`AT-33-E1-002` extends this same script with the remaining three fail-closed
conditions named in `decisions.md` §1:

    3. oracle disagreement -- any `(ours, oracle, verdict)` record with
       verdict "disagree" in an oracle-results file (the shape AT-33-E2-003's
       harness returns). No harness exists yet as of this cycle, so the
       check is wired but has nothing to examine until Epic 2 lands an
       `oracle-results.json`; it activates automatically once one exists,
       rather than being written twice.
    4. an `unverifiable` unit dispositioned `done` -- a ledger group that
       marks itself `"unverifiable": true` must never also carry
       `"disposition": "done"` (`decisions.md` §7's exact over-claim).
    5. the `derived_at` staleness gate -- `THE-BOX.md`'s front-matter
       `derived_at` SHA must be a real commit that is an ancestor of (or
       equal to) current `HEAD`.

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
import subprocess
import sys
from collections import Counter, namedtuple

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_BOX = os.path.join(
    REPO_ROOT, "docs", "release", "SD-33-computed-value-verification", "THE-BOX.md"
)
DEFAULT_ORACLE_RESULTS = os.path.join(
    REPO_ROOT, "docs", "release", "SD-33-computed-value-verification",
    "artifacts", "epic-2-oracle-harness", "oracle-results.json",
)

LEDGER_BLOCK_RE = re.compile(r"```json ledger\s*\n(.*?)\n```", re.DOTALL)

PartitionResult = namedtuple(
    "PartitionResult", ["uncovered", "overlap", "population", "membership", "group_members"]
)


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
    """Compute uncovered/overlap against the live units, the live per-group
    membership count (independent of any count each group *states* it
    has), and the live per-group membership id list (used by
    `unverifiable_done_violations` to name the affected units)."""
    uncovered = []
    overlap = []
    membership = Counter()
    group_members = {}
    for unit in units:
        hits = [g["id"] for g in groups if _matches(unit, g.get("match", {}))]
        if len(hits) == 0:
            uncovered.append(unit["id"])
        elif len(hits) > 1:
            overlap.append(unit["id"])
        for gid in hits:
            membership[gid] += 1
            group_members.setdefault(gid, []).append(unit["id"])
    return PartitionResult(
        uncovered=uncovered, overlap=overlap, population=len(units),
        membership=membership, group_members=group_members,
    )


def load_front_matter(path):
    """Parse the YAML-ish front matter block (between the first two `---`
    lines) out of a markdown file. Returns a dict of top-level scalar
    `key: value` pairs, or `{}` if the file carries no front matter block at
    all. Used to read `THE-BOX.md`'s `derived_at` field for the staleness
    gate (condition 5)."""
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    if not text.startswith("---\n"):
        return {}
    end = text.find("\n---", 4)
    if end == -1:
        return {}
    block = text[4:end]
    fm = {}
    for line in block.splitlines():
        if ":" not in line:
            continue
        key, _, value = line.partition(":")
        fm[key.strip()] = value.strip()
    return fm


def check_staleness(derived_at_sha, repo_root=REPO_ROOT):
    """Condition 5 (`decisions.md` §1 item 5): `THE-BOX.md`'s recorded
    `derived_at` SHA must name a real commit that is an ancestor of (or
    equal to) current `HEAD`. Returns `(ok, message)` -- never raises, so a
    missing or garbage SHA is a reported failure, not a crash."""
    if not derived_at_sha:
        return False, "THE-BOX.md front matter carries no derived_at SHA"
    proc = subprocess.run(
        ["git", "merge-base", "--is-ancestor", derived_at_sha, "HEAD"],
        cwd=repo_root, capture_output=True, text=True,
    )
    if proc.returncode == 0:
        return True, f"derived_at={derived_at_sha} is an ancestor of HEAD"
    detail = proc.stderr.strip() or f"git exit {proc.returncode}"
    return False, f"derived_at={derived_at_sha} is NOT an ancestor of HEAD ({detail})"


def unverifiable_done_violations(groups, group_members):
    """Condition 4 (`decisions.md` §1 item 4, §7): a group that marks its
    own units `"unverifiable": true` must never also carry
    `"disposition": "done"` -- the exact over-claim SD-32's
    `doneness_verdict()` made for 8,330 units blessed against artifacts we
    wrote ourselves, never against the oracle. Returns a list of
    `(group_id, member_unit_ids)` for every violating group."""
    out = []
    for g in groups:
        if g.get("unverifiable") and g.get("disposition") == "done":
            out.append((g["id"], list(group_members.get(g["id"], []))))
    return out


def load_oracle_results(path):
    """Load an oracle-results file: either a bare JSON list of records, or
    `{"results": [...]}`. Each record is the `(unit_id, ours, oracle,
    verdict)` shape `AT-33-E2-003`'s comparison harness returns."""
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    if isinstance(data, dict):
        records = data.get("results")
        if records is None:
            raise ValueError(f"{path}: oracle-results dict form must have a 'results' list")
    elif isinstance(data, list):
        records = data
    else:
        raise ValueError(f"{path}: oracle-results must be a JSON list or {{'results': [...]}} dict")
    return records


def oracle_disagreement_violations(records):
    """Condition 3 (`decisions.md` §1 item 3): any record whose verdict is
    `"disagree"`. `"unverifiable"` is a first-class, non-failing outcome
    (`AT-33-E2-003`) -- only `"disagree"` gates."""
    return [r for r in records if r.get("verdict") == "disagree"]


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


def run_check(inventory_path, box_path, out=sys.stdout, oracle_results_path=None, repo_root=REPO_ROOT):
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

    # Condition 3 -- oracle disagreement (decisions.md §1 item 3). Wired
    # unconditionally; simply has nothing to check until an oracle-results
    # file exists (Epic 2's deliverable), which it then picks up
    # automatically without a second cycle rewriting this check.
    if oracle_results_path is None:
        oracle_results_path = DEFAULT_ORACLE_RESULTS
    if os.path.exists(oracle_results_path):
        oracle_records = load_oracle_results(oracle_results_path)
    else:
        oracle_records = []
        print(
            f"INFO: no oracle-results at {oracle_results_path} -- oracle "
            "disagreement check is wired but has nothing to examine yet "
            "(Epic 2 not landed)",
            file=out,
        )
    oracle_bad = oracle_disagreement_violations(oracle_records)

    # Condition 4 -- an unverifiable unit dispositioned done (decisions.md
    # §1 item 4, §7).
    unverifiable_bad = unverifiable_done_violations(groups, result.group_members)

    # Condition 5 -- the derived_at staleness gate (decisions.md §1 item 5).
    front_matter = load_front_matter(box_path)
    stale_ok, stale_msg = check_staleness(front_matter.get("derived_at"), repo_root=repo_root)
    if not stale_ok:
        print(f"STALE: {stale_msg}", file=out)

    unverifiable_done_count = sum(len(ids) for _, ids in unverifiable_bad)
    print(
        f"uncovered={len(result.uncovered)} overlap={len(result.overlap)} "
        f"population={result.population} oracle_disagreement={len(oracle_bad)} "
        f"unverifiable_done={unverifiable_done_count} stale={not stale_ok}",
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
    if oracle_bad:
        sample = ", ".join(r.get("unit_id", "?") for r in oracle_bad[:10])
        more = f" (+{len(oracle_bad) - 10} more)" if len(oracle_bad) > 10 else ""
        print(f"ORACLE_DISAGREEMENT: {sample}{more}", file=out)
    if unverifiable_bad:
        for gid, ids in unverifiable_bad:
            sample = ", ".join(ids[:10])
            more = f" (+{len(ids) - 10} more)" if len(ids) > 10 else ""
            print(f"UNVERIFIABLE_DISPOSITIONED_DONE: group '{gid}': {sample}{more}", file=out)

    ok = (
        not result.uncovered
        and not result.overlap
        and not oracle_bad
        and not unverifiable_bad
        and stale_ok
    )
    return 0 if ok else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    # `--check` is accepted (it is the invocation AT-33-E1-001 specifies) but
    # is currently the only mode this tool has, so it does not branch
    # behaviour -- there is no second mode to select between yet.
    parser.add_argument("--check", action="store_true", help="run the partition check (the only mode this tool has)")
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY, help="path to docs/work-inventory.json")
    parser.add_argument("--box", default=DEFAULT_BOX, help="path to THE-BOX.md")
    parser.add_argument(
        "--oracle-results", default=None,
        help=f"path to oracle-results JSON (default: {DEFAULT_ORACLE_RESULTS}); "
             "skipped with an INFO line, not a failure, when absent",
    )
    args = parser.parse_args(argv)

    return run_check(args.inventory, args.box, oracle_results_path=args.oracle_results)


if __name__ == "__main__":
    sys.exit(main())
