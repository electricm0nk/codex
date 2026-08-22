#!/usr/bin/env python3
"""Build a per-unit coverage ledger against `docs/work-inventory.json`, and
prove -- structurally, not by assertion -- whether every not-done unit was
examined by some named GROUP.

Why this exists
----------------
`docs/release/SD-31-corpus-closure-grind/artifacts/THE-BOX.md` (wave 28,
addended wave 29) found 295 not-done units that NO census lane had examined
at all, and a separate 1,212 examined by two lanes redundantly. Both facts
were discovered by hand, days after the census that should have caught them
shipped as "complete". The reconciliation arithmetic in THE-BOX.md §1 --
sum every lane's pile, subtract the overlap, compare to the population --
is exactly the operation this script automates, so that no future wave has
to re-derive it by hand from six lane reports and get it wrong twice before
a third pass catches it.

This script does not decide what a GROUP is. It reads a **classification
table** -- plain data, not code -- naming each group's matching predicate
and its `todo/*.md` entry, applies it to every not-done unit in the live
inventory, and reports:

1. A per-unit ledger row: id, kind, book, verdict, wiring_class, status,
   evidence, and the list of group ids that claim it.
2. A per-group rollup: group id, label, count, todo_entry -- flagged if the
   group has no todo_entry, because an unfiled group is a finding that will
   be lost (the operator's explicit requirement, `README.md`'s "why it
   exists" recap of the Monk case).
3. The uncovered count -- units matched by ZERO group. Must be able to be
   ZERO; when it is not, this script names every uncovered unit rather than
   just the count, because THE-BOX.md's own worked example ("the box named
   the hole and nobody went and looked") is exactly the failure a bare
   number invites.
4. The overlap count -- units matched by MORE than one group -- surfaced
   the same way THE-BOX.md §1.3 had to hand-derive it once.

Classification table shape (data, not code)
--------------------------------------------
A JSON document:

    {
      "groups": [
        {
          "id": "S6",                          # required, unique
          "label": "free text",                # required
          "todo_entry": "todo/sweeps.md#S6",    # required (empty string is
                                                 # allowed and IS flagged --
                                                 # see --strict)
          "match": {                            # required, see below
            "kind": ["class_feature"],
            "book": ["adventurers_guide"],
            "status": ["not-started"],
            "wiring_class": ["derived"],
            "verdict": ["not-started", "held"],
            "unit_ids": ["book:kind:key", ...],
            "id_regex": "python-re pattern, matched with re.search",
            "name_regex": "...",
            "corpus_key_regex": "...",
            "type_facet_regex": "...",
            "source_file_regex": "..."
          }
        },
        ...
      ]
    }

Every key inside `match` is optional; a group with an empty (or omitted)
`match` matches nothing (fails closed, not open -- an empty predicate
matching everything would silently manufacture 100% coverage). Every
PRESENT key is ANDed against every other present key. Within one key, a
list value is an "any of" match; a `*_regex` value is `re.search` against
the corresponding unit field (missing field treated as `""`).

No key accepts an expression, a lambda, or a code string. This is
deliberate: the table is meant to be produced by census lanes and read by
this tool without either side needing to trust the other's Python.

Usage
-----
    python3 scripts/coverage_ledger.py --groups path/to/groups.json
    python3 scripts/coverage_ledger.py --groups path/to/groups.json --strict
    python3 scripts/coverage_ledger.py --groups path/to/groups.json \
        --out /tmp/ledger.json --uncovered-out /tmp/uncovered.json

Exit status is always 0 unless `--strict` is given AND either an uncovered
unit exists or a group is missing its `todo_entry` -- proving the tool can
fail is part of its own test suite (`scripts/tests/test_coverage_ledger.py`).
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts", "observer"))
import pf1e_dashboard_producer as P  # noqa: E402  (path set above)

# Every field a `match` clause is allowed to test with a plain "any of"
# list. Kept as an explicit tuple (not "whatever the unit dict happens to
# have") so an unrecognized key in a classification table is a loud error
# instead of a silently-ignored typo.
LIST_MATCH_FIELDS = ("kind", "book", "status", "wiring_class", "verdict", "visible", "origin")

# Fields a `match` clause may test with a regex, via `re.search`. Same
# closed-set reasoning as LIST_MATCH_FIELDS.
#
# `evidence_regex` added wave 30 integration: the wave-30 adversarial review
# (worktree wf_b4fe44c9-141-7) fed this tool a lane-6 group keyed on
# `evidence` (the book-onboarding-gate finding, `no_compiled_rule_set_for_book`)
# and got `ClassificationTableError: unknown key(s) ['evidence']` -- the tool
# could READ `evidence` onto every ledger row but had no way to MATCH on it,
# which blocked 4 of that lane's 6 corpus-wide populations from ever being
# expressed as a real group. `evidence` is the single most load-bearing field
# for this kind of census (it is literally what THE-BOX.md's own G1/G3/G6
# groups are keyed on), so it gets the same regex-match treatment as the
# other identity-shaped fields below, not the coarser list-match `kind` and
# `verdict` already have. `visible` and `origin` (list match, not regex --
# both are small closed-ish value sets on the unit: `True`/`False`/`None`
# and `"copy"`/`"native"`/...) were the two other match keys that blocked
# lane 6 (B.3, B.4) and are added the same way, for the same reason.
REGEX_MATCH_FIELDS = {
    "id_regex": "id",
    "name_regex": "name",
    "corpus_key_regex": "corpus_key",
    "type_facet_regex": "type_facet",
    "source_file_regex": "source_file",
    "evidence_regex": "evidence",
}

KNOWN_MATCH_KEYS = frozenset(LIST_MATCH_FIELDS) | frozenset(REGEX_MATCH_FIELDS) | {"unit_ids"}


class ClassificationTableError(ValueError):
    """The classification table is malformed -- not a coverage finding."""


def load_inventory(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def load_classification_table(path: str) -> list[dict]:
    """Load and validate the groups list. Raises ClassificationTableError
    on any structural problem -- this is a fail-fast on the DATA file, not
    a coverage result, and must never be silently swallowed into "0
    groups"."""
    with open(path, "r", encoding="utf-8") as fh:
        doc = json.load(fh)
    if not isinstance(doc, dict) or "groups" not in doc:
        raise ClassificationTableError(f"{path}: top level must be an object with a 'groups' key")
    groups = doc["groups"]
    if not isinstance(groups, list):
        raise ClassificationTableError(f"{path}: 'groups' must be a list")
    seen_ids: set[str] = set()
    for i, g in enumerate(groups):
        if not isinstance(g, dict):
            raise ClassificationTableError(f"{path}: groups[{i}] is not an object")
        for required in ("id", "label", "todo_entry", "match"):
            if required not in g:
                raise ClassificationTableError(f"{path}: groups[{i}] missing required key '{required}'")
        gid = g["id"]
        if not isinstance(gid, str) or not gid:
            raise ClassificationTableError(f"{path}: groups[{i}].id must be a non-empty string")
        if gid in seen_ids:
            raise ClassificationTableError(f"{path}: duplicate group id '{gid}'")
        seen_ids.add(gid)
        match = g["match"]
        if not isinstance(match, dict):
            raise ClassificationTableError(f"{path}: group '{gid}'.match must be an object")
        unknown = set(match) - KNOWN_MATCH_KEYS
        if unknown:
            raise ClassificationTableError(
                f"{path}: group '{gid}'.match has unknown key(s) {sorted(unknown)}; "
                f"known keys are {sorted(KNOWN_MATCH_KEYS)}"
            )
        for field in LIST_MATCH_FIELDS:
            if field in match and not isinstance(match[field], list):
                raise ClassificationTableError(f"{path}: group '{gid}'.match.{field} must be a list")
        if "unit_ids" in match and not isinstance(match["unit_ids"], list):
            raise ClassificationTableError(f"{path}: group '{gid}'.match.unit_ids must be a list")
        for field in REGEX_MATCH_FIELDS:
            if field in match:
                if not isinstance(match[field], str):
                    raise ClassificationTableError(f"{path}: group '{gid}'.match.{field} must be a string")
                try:
                    re.compile(match[field])
                except re.error as exc:
                    raise ClassificationTableError(
                        f"{path}: group '{gid}'.match.{field} is not a valid regex: {exc}"
                    ) from exc
    return groups


def unit_verdict(unit: dict) -> str:
    return P.doneness_verdict(unit.get("wiring_class"), unit.get("status"), unit.get("kind"))


def not_done_population(
    inventory: dict,
    excluded_books: frozenset[str] = frozenset(P.EXCLUDED_BOOKS),
) -> list[dict]:
    """The exact population THE-BOX.md's §0/§1 reconcile against: every unit
    outside EXCLUDED_BOOKS whose verdict is not 'done'."""
    out = []
    for unit in inventory.get("units") or []:
        if (unit.get("book") or "unknown") in excluded_books:
            continue
        if unit_verdict(unit) == P.DONENESS_DONE:
            continue
        out.append(unit)
    return out


def _matches(unit: dict, verdict: str, match: dict) -> bool:
    for field in LIST_MATCH_FIELDS:
        if field not in match:
            continue
        actual = verdict if field == "verdict" else unit.get(field)
        if actual not in match[field]:
            return False
    if "unit_ids" in match:
        if unit.get("id") not in match["unit_ids"]:
            return False
    for regex_key, unit_field in REGEX_MATCH_FIELDS.items():
        if regex_key not in match:
            continue
        haystack = unit.get(unit_field) or ""
        if not re.search(match[regex_key], haystack):
            return False
    return True


def build_ledger(units: list[dict], groups: list[dict]) -> dict:
    """Returns a dict with:
      - 'rows': per-unit ledger rows (id, kind, book, verdict, wiring_class,
        status, evidence, groups)
      - 'group_rollup': per-group {id, label, todo_entry, count,
        has_todo_entry}
      - 'uncovered': list of unit ids matched by zero groups
      - 'overlap': list of {id, groups} for units matched by >1 group
      - 'population': total not-done units considered
      - 'covered_distinct': units matched by >=1 group
      - 'groups_without_todo_entry': group ids whose todo_entry is empty
    """
    # Fail closed: a group with an empty match object matches nothing,
    # explicitly, rather than falling through to "no constraints checked
    # so everything matches" -- an empty predicate manufacturing 100%
    # coverage is exactly the anti-gaming shape Decision 1a forbids.
    group_counts: collections.Counter[str] = collections.Counter()
    rows = []
    uncovered = []
    overlap = []
    for unit in units:
        verdict = unit_verdict(unit)
        matched_ids = []
        for g in groups:
            if not g["match"]:
                continue
            if _matches(unit, verdict, g["match"]):
                matched_ids.append(g["id"])
        for gid in matched_ids:
            group_counts[gid] += 1
        rows.append(
            {
                "id": unit.get("id"),
                "kind": unit.get("kind"),
                "book": unit.get("book"),
                "verdict": verdict,
                "wiring_class": unit.get("wiring_class"),
                "status": unit.get("status"),
                "evidence": unit.get("evidence"),
                "groups": matched_ids,
            }
        )
        if not matched_ids:
            uncovered.append(unit.get("id"))
        elif len(matched_ids) > 1:
            overlap.append({"id": unit.get("id"), "groups": matched_ids})

    group_rollup = []
    groups_without_todo_entry = []
    for g in groups:
        has_entry = bool(g.get("todo_entry"))
        if not has_entry:
            groups_without_todo_entry.append(g["id"])
        group_rollup.append(
            {
                "id": g["id"],
                "label": g["label"],
                "todo_entry": g.get("todo_entry") or "",
                "count": group_counts.get(g["id"], 0),
                "has_todo_entry": has_entry,
            }
        )

    return {
        "population": len(units),
        "rows": rows,
        "group_rollup": group_rollup,
        "uncovered": sorted(u for u in uncovered if u is not None),
        "uncovered_count": len(uncovered),
        "overlap": overlap,
        "overlap_count": len(overlap),
        "covered_distinct": len(units) - len(uncovered),
        "groups_without_todo_entry": groups_without_todo_entry,
    }


def _print_summary(ledger: dict) -> None:
    print(f"population (not-done units considered): {ledger['population']}")
    print(f"covered by >=1 group:                    {ledger['covered_distinct']}")
    print(f"uncovered (0 groups):                     {ledger['uncovered_count']}")
    print(f"overlap (>1 group):                        {ledger['overlap_count']}")
    print()
    print("group rollup:")
    for row in ledger["group_rollup"]:
        flag = "" if row["has_todo_entry"] else "  <-- NO TODO ENTRY"
        print(f"  {row['id']:<12} {row['count']:>7}  {row['todo_entry']}{flag}")
    if ledger["uncovered_count"]:
        sample = ledger["uncovered"][:20]
        print()
        print(f"uncovered units ({ledger['uncovered_count']} total, first {len(sample)} shown):")
        for uid in sample:
            print(f"  {uid}")
    if ledger["groups_without_todo_entry"]:
        print()
        print(f"groups with NO todo_entry: {ledger['groups_without_todo_entry']}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY, help="path to docs/work-inventory.json")
    parser.add_argument("--groups", required=True, help="path to the classification table (JSON)")
    parser.add_argument("--out", help="write the full ledger (rows + rollup) as JSON to this path")
    parser.add_argument("--uncovered-out", help="write just the uncovered unit-id list as JSON to this path")
    parser.add_argument(
        "--strict",
        action="store_true",
        help="exit 1 if any unit is uncovered, or any group has no todo_entry",
    )
    args = parser.parse_args(argv)

    inventory = load_inventory(args.inventory)
    groups = load_classification_table(args.groups)
    units = not_done_population(inventory)
    ledger = build_ledger(units, groups)

    _print_summary(ledger)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            json.dump(ledger, fh, indent=2)
            fh.write("\n")
    if args.uncovered_out:
        with open(args.uncovered_out, "w", encoding="utf-8") as fh:
            json.dump(ledger["uncovered"], fh, indent=2)
            fh.write("\n")

    if args.strict and (ledger["uncovered_count"] > 0 or ledger["groups_without_todo_entry"]):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
