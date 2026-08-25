#!/usr/bin/env python3
"""Re-derives the REAL corpus group name(s) behind each registered pool in
`src/bin/v06_work_inventory.rs::CLASS_FEATURE_POOLS`, generically (SD-32 T12
Epic 8 row 18, cycle 4, `decisions.md §17a`).

Cycle 2 found ONE registered pool name (`"Spirit"`, Shaman) matching zero
corpus records. Cycle 3 checked the rest by hand and found the pattern is
general -- most registered names are the bare pool word (`"Mystery"`,
`"Domain"`, `"Blessing"`) while the real corpus shape is
`"<Adjective> <PoolWord>"` (`"Battle Mystery"`, `"Air Domain"`, `"Warpriest"`
with no `"Blessing"` suffix at all), and a further, distinct drift exists
even where the adjective IS the owner class name: `"Slayer Talent"` (the
correct MEMBER-key prefix) versus `"Slayer Talents"` (the real HEADER
record's own suffix) -- singular/plural, not adjective drift.

This script is the ONE generic mechanism `decisions.md §17`/dispatch demand
(no per-pool lookup table): for each `(registered_name, owner_class)` pair,
it scans every real `" ~ "`-group-qualified `class_feature` corpus group
whose OWNER (`data.class`, read from the group's own records, never assumed)
is the registered pool's owner class, and matches the group's own name
against `registered_name` by:

  1. exact byte match, or
  2. a word-boundary SUFFIX match (`group` ends with `" " + registered`), or
  3. the same suffix match after normalizing BOTH the registered name and
     the group's own suffix by stripping one trailing `s` (singular/plural
     insensitive) -- the mechanism `pool_header_record_by_normalized_suffix`
     (`src/rules_core/pilot_compute/mod.rs`, this cycle) also uses for the
     header-record lookup, so census and compute logic share one rule.

No group name is ever hand-listed as a special case; every row in the
table below is produced by that one rule running over the live corpus.

Usage:
    python3 scripts/census_class_feature_pool_group_names.py [--json]
"""
from __future__ import annotations

import argparse
import glob
import json
import sys
from collections import defaultdict

# Mirrors `src/bin/v06_work_inventory.rs::CLASS_FEATURE_POOLS` exactly
# (registered name, owner class, choice_set_id, namespace) -- read here only
# to know WHICH (name, owner) pairs to re-derive, never to trust the name
# itself as a corpus group.
CLASS_FEATURE_POOLS: list[tuple[str, str]] = [
    ("Rage Power", "Barbarian"),
    ("Unchained Rage Power", "Unchained Barbarian"),
    ("Discovery", "Alchemist"),
    ("Grand Discovery", "Alchemist"),
    ("Rogue Talent", "Rogue"),
    ("Advanced Talents", "Rogue"),
    ("Hex", "Witch"),
    ("Revelation", "Oracle"),
    ("Mercy", "Paladin"),
    ("Investigator Talent", "Investigator"),
    ("Slayer Talent", "Slayer"),
    ("Judgment", "Inquisitor"),
    ("Inquisition", "Inquisitor"),
    ("Blessing", "Warpriest"),
    ("Evolution", "Summoner"),
    ("Bloodline", "Sorcerer"),
    ("Bloodrager Bloodline", "Bloodrager"),
    ("Domain", "Cleric"),
    ("Order", "Cavalier"),
    ("Mystery", "Oracle"),
    ("Curse", "Oracle"),
    ("Spirit", "Shaman"),
    ("Animal Focus", "Hunter"),
    ("Favored Enemy", "Ranger"),
    ("Favored Terrain", "Ranger"),
    ("Versatile Performance", "Bard"),
    ("Arcane School", "Wizard"),
    ("Focused Arcane School", "Wizard"),
]


def normalize(word: str) -> str:
    return word.rstrip("s")


def group_matches(registered: str, group: str) -> str | None:
    """Returns the match kind ("exact", "suffix", "suffix-normalized") or
    None. Word-boundary only -- never a bare substring."""
    if group == registered:
        return "exact"
    suffix_exact = group[: -len(registered)] if group.endswith(" " + registered) else None
    if suffix_exact is not None:
        return "suffix"
    if group.split(" ")[-1] and normalize(group.rsplit(" ", 1)[-1]) == normalize(registered.rsplit(" ", 1)[-1]):
        prefix = group.rsplit(" ", 1)[0] if " " in group else ""
        reg_prefix = registered.rsplit(" ", 1)[0] if " " in registered else ""
        if prefix == reg_prefix:
            return "suffix-normalized"
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    files = sorted(glob.glob("data/corpus/*/class_feature/**/*.json", recursive=True))

    # group name -> {"owners": {class_name: count}, "total": int}
    groups: dict[str, dict] = defaultdict(lambda: {"owners": defaultdict(int), "total": 0})
    for f in files:
        try:
            doc = json.load(open(f, encoding="utf-8"))
        except Exception:
            continue
        data = doc.get("data", {})
        key = data.get("key")
        if not isinstance(key, str) or " ~ " not in key:
            continue
        group = key.split(" ~ ", 1)[0]
        owner = data.get("class")
        g = groups[group]
        g["total"] += 1
        if isinstance(owner, str):
            g["owners"][owner] += 1

    rows = []
    for registered, owner_class in CLASS_FEATURE_POOLS:
        matches = []
        for group, g in groups.items():
            # a group belongs to this pool's owner only if the OWNER CLASS
            # majority of its own records says so -- read from the corpus,
            # never assumed from the group's text.
            owner_here = max(g["owners"].items(), key=lambda kv: kv[1])[0] if g["owners"] else None
            if owner_here != owner_class:
                continue
            kind = group_matches(registered, group)
            if kind:
                matches.append((group, g["total"], kind))
        matches.sort(key=lambda m: -m[1])
        rows.append(
            {
                "registered_name": registered,
                "owner_class": owner_class,
                "real_groups": [{"group": m[0], "records": m[1], "match": m[2]} for m in matches],
                "real_record_total": sum(m[1] for m in matches),
            }
        )

    if args.json:
        print(json.dumps(rows, indent=2))
        return 0

    print("Pool registry name -> real corpus group name(s) (decisions.md §17a, T12 cycle 4)")
    print(f"  command: python3 {sys.argv[0]}")
    for row in rows:
        real = row["real_groups"]
        label = f"{row['registered_name']!r} ({row['owner_class']})"
        if not real:
            print(f"  {label:<45} -> NO MATCH (0 real corpus groups)")
            continue
        for m in real:
            print(
                f"  {label:<45} -> {m['group']!r:<28} {m['records']:>5} records  [{m['match']}]"
            )
        if len(real) > 1 or real[0]["records"] == 0:
            print(f"      total across {len(real)} group(s): {row['real_record_total']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
