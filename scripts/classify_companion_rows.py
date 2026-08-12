#!/usr/bin/env python3
"""Classify a book's `companion` corpus ROWS before a round commits to it.

`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §45.1` is the reason
this file exists at all: a lane that ranks books by `docs/work-inventory.json`'s
*evidence token* is reading a statement about what the engine has compiled when
the question is what the **corpus rows** are, and the race-trait lane got its
whole successor queue exactly backwards that way.  `§46.1` applied the same
discipline to `monster_ability` and found a structural ceiling.  This is that
step for `companion`.

# The question this answers

The `companion` kind is not one kind.  A `.lst` basename carrying a
companion/familiar marker holds rows of three structurally different shapes, and
only one of them is a chassis:

* **creature** rows (`*_races_companion.lst`, `*_races_familiar.lst`) — a
  companion or familiar creature, the thing a player's sheet names.  This is the
  chassis: it carries `SIZE:`, `MOVE:`, `MONSTERCLASS:`, ability references.
* **ability** rows (`*_abilities_companion.lst`, `*_abilities_familiar.lst`,
  `*_abilities_race_*companion*.lst`) — a special quality, special attack or
  level-advancement package that reaches a player **only underneath the creature
  that owns it**, exactly as `monster_ability` does underneath `monster`.
* **class** rows (`*_classes_companion.lst`) — the PCGen `Companion` /
  `Familiar` monster *classes* the `MONSTERCLASS:` token names.  These are hit
  dice progressions, not creatures and not abilities.

An ability row no creature row claims is a record that loads and is never shown:
the stub class `§44.2` was written about.  The `ORPHAN` column below is that
count, and it is a **ceiling on the lane**, not a preference.

# The three ownership shapes, every one stated by the corpus

1. **row-named** — a creature row's `ABILITY:Special Ability|AUTOMATIC|<name>`
   token names the ability outright (by `KEY:` or by display name).  This is the
   same predicate `transcribe_monster_tables.parse_special_ability_refs` uses.
2. **prerace** — the ability row's own `PRERACE:1,<Race>` names a creature row of
   this book.  `monster_ability` has no analogue; companions carry it heavily
   (every `TYPE:CompanionAdvancement` row does), and a classifier that only knew
   shape 1 would report them as orphans.
3. **prefix** — a namespaced `KEY:<Owner> ~ <Leaf>` whose `<Owner>` is a creature
   of this book, either verbatim or as the inner name of `Companion (<Owner>)` /
   `Familiar (<Owner>)`.  `Worg ~ Mastery` owns through `Companion (Worg)`.

A looser rule would over-report reachability, which is the direction that ships
stubs; each shape above is a token the row itself carries.

Usage::

    python3 scripts/classify_companion_rows.py                 # every book
    python3 scripts/classify_companion_rows.py inner_sea_combat monster_codex

`PCGEN_CORPUS_ROOT` may point at a local PCGen ``data/`` checkout; it defaults to
``$HOME/workspace/repos/pcgen/data``.
"""

from __future__ import annotations

import json
import os
import re
import sys

INVENTORY = "docs/work-inventory.json"


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def book_dirs() -> dict[str, str]:
    """Book id -> its PCGen source directory, read from the inventory itself."""
    inv = json.load(open(INVENTORY, encoding="utf-8"))
    dirs: dict[str, str] = {}
    root = inv["corpus_root"]
    for entry in os.listdir(root):
        if os.path.isdir(os.path.join(root, entry)):
            dirs[entry] = os.path.join(root, entry)
    for extra in inv.get("additional_book_dirs", []):
        dirs[os.path.basename(extra)] = extra
    return dirs


def row_shape(source_file: str) -> str:
    """Which of the three structural shapes a companion `.lst` basename holds."""
    if "_races_" in source_file:
        return "creature"
    if "_classes_" in source_file:
        return "class"
    return "ability"


def read_row(path: str, line_no: int) -> list[str]:
    with open(path, encoding="utf-8", errors="replace") as handle:
        line = handle.read().split("\n")[line_no - 1]
    return [t.strip() for t in line.split("\t") if t.strip()]


# `docs/work-inventory.json` records `source_file` as a BASENAME, so a `.lst`
# that PCGen loads out of a subdirectory (`support/`, `_pfs/`) is not at
# `<book>/<basename>` at all.  Bestiary 5's `b5_races_companion_oa.lst` is the
# first companion instance: two of its 57 units live in `support/`.
#
# `classify` used to `continue` past a path it could not open, which is the
# failure shape `decisions.md §47.3` warns about — a check that silently
# measures less than it claims.  Both consumers now resolve the basename and
# raise when it is nowhere under the book.
_RESOLVED: dict[tuple[str, str], str] = {}


def resolve_source_file(directory: str, source_file: str) -> str:
    """`<book>/<basename>` if it exists, else the one match anywhere below it."""
    cached = _RESOLVED.get((directory, source_file))
    if cached is not None:
        return cached
    direct = os.path.join(directory, source_file)
    if os.path.exists(direct):
        _RESOLVED[(directory, source_file)] = direct
        return direct
    hits = [
        os.path.join(parent, source_file)
        for parent, _dirs, files in os.walk(directory)
        if source_file in files
    ]
    if not hits:
        raise SystemExit(f"{source_file} is nowhere under {directory}")
    if len(hits) > 1:
        raise SystemExit(f"{source_file} is ambiguous under {directory}: {hits!r}")
    _RESOLVED[(directory, source_file)] = hits[0]
    return hits[0]


# A `.lst` whose pcc load line carries `PRECAMPAIGN:` is loaded only when that
# campaign is also loaded — and the gate is on the PCC LINE, never inside the
# `.lst` (`loop-instruction.md`, "Conditional cross-book support files"; a
# `grep PRECAMPAIGN` over the `.lst` itself returns 0).
#
# Most such gates name a book this repo HAS ingested (`INCLUDES=Bestiary 3`,
# `INCLUDES=Advanced Player's Guide`), so "gated" alone is not "out of scope".
# What is out of scope is a gate naming a campaign this repo has not ingested.
# `decisions.md §47.2` already ruled exactly this for Horror Adventures'
# `ha_abilities_race_oa.lst`, and `RuleSetId::Ha`'s doc comment records it.
UNINGESTED_CAMPAIGN_GATES = ("Occult Adventures",)

_PCC_LOAD = re.compile(r"^[A-Z]+:(?P<path>[^|\t]+\.lst)\|(?P<rest>.*)$")


def precampaign_gates(directory: str) -> dict[str, str]:
    """`.lst` basename -> the `PRECAMPAIGN:` expression its pcc load line carries."""
    gates: dict[str, str] = {}
    for parent, _dirs, files in os.walk(directory):
        for name in files:
            if not name.endswith(".pcc"):
                continue
            with open(os.path.join(parent, name), encoding="utf-8", errors="replace") as handle:
                for line in handle:
                    line = line.strip()
                    if line.startswith("#"):
                        continue
                    match = _PCC_LOAD.match(line)
                    if match is None or "PRECAMPAIGN:" not in match.group("rest"):
                        continue
                    for field in match.group("rest").split("|"):
                        if field.startswith("PRECAMPAIGN:"):
                            gates[os.path.basename(match.group("path"))] = field
                            break
    return gates


def gated_on_an_uningested_campaign(gate: str | None) -> bool:
    return gate is not None and any(name in gate for name in UNINGESTED_CAMPAIGN_GATES)


def token(row: list[str], prefix: str) -> str | None:
    for field in row:
        if field.startswith(prefix):
            return field[len(prefix) :]
    return None


def special_ability_refs(row: list[str]) -> list[str]:
    """Keys named by `ABILITY:Special Ability|AUTOMATIC|…` (shape 1)."""
    refs: list[str] = []
    for field in row:
        if not field.startswith("ABILITY:Special Ability|"):
            continue
        for name in field.split("|")[2:]:
            name = name.strip()
            if not name or name.startswith("PRE") or "=" in name:
                continue
            if name not in refs:
                refs.append(name)
    return refs


def prerace_owners(row: list[str]) -> list[str]:
    """Creature names named by the row's own `PRERACE:` token (shape 2)."""
    owners: list[str] = []
    for field in row:
        if not field.startswith("PRERACE:"):
            continue
        parts = field[len("PRERACE:") :].split(",")
        for name in parts[1:]:
            name = name.strip().lstrip("!")
            if name and name not in owners:
                owners.append(name)
    return owners


def bare_species(key: str) -> str:
    """`Companion (Worg)` -> `Worg`; `Familiar (Seru)` -> `Seru`; else itself."""
    for wrapper in ("Companion (", "Familiar ("):
        if key.startswith(wrapper) and key.endswith(")"):
            return key[len(wrapper) : -1]
    return key


def classify(book: str, units: list[dict], directory: str) -> dict:
    gates = precampaign_gates(directory)
    gated = [
        u for u in units if gated_on_an_uningested_campaign(gates.get(u["source_file"]))
    ]
    gated_keys = {u["corpus_key"] for u in gated}
    units = [u for u in units if u["corpus_key"] not in gated_keys]

    creatures = [u for u in units if row_shape(u["source_file"]) == "creature"]
    abilities = [u for u in units if row_shape(u["source_file"]) == "ability"]
    classes = [u for u in units if row_shape(u["source_file"]) == "class"]

    creature_keys = {u["corpus_key"] for u in creatures}
    creature_species = {bare_species(k): k for k in creature_keys}

    ability_by_key = {u["corpus_key"]: u for u in abilities}
    ability_by_name = {u["name"]: u for u in abilities}

    owned_row_named: set[str] = set()
    for unit in creatures:
        path = resolve_source_file(directory, unit["source_file"])
        for ref in special_ability_refs(read_row(path, unit["source_line"])):
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is not None:
                owned_row_named.add(hit["corpus_key"])

    owned_prerace: set[str] = set()
    owned_prefix: set[str] = set()
    for unit in abilities:
        key = unit["corpus_key"]
        path = resolve_source_file(directory, unit["source_file"])
        row = read_row(path, unit["source_line"])
        for owner in prerace_owners(row):
            if owner in creature_keys or owner in creature_species:
                owned_prerace.add(key)
        if " ~ " in key:
            prefix = key.split(" ~ ")[0]
            if prefix in creature_keys or prefix in creature_species:
                owned_prefix.add(key)

    owned = owned_row_named | owned_prerace | owned_prefix
    orphans = [u["corpus_key"] for u in abilities if u["corpus_key"] not in owned]
    return {
        "book": book,
        "creatures": len(creatures),
        "abilities": len(abilities),
        "classes": len(classes),
        "row_named": len(owned_row_named),
        "prerace": len(owned_prerace),
        "prefix": len(owned_prefix),
        "orphans": orphans,
        "gated": [
            (u["corpus_key"], u["source_file"], gates[u["source_file"]]) for u in gated
        ],
    }


def main() -> None:
    inv = json.load(open(INVENTORY, encoding="utf-8"))
    dirs = book_dirs()
    wanted = sys.argv[1:]
    units_by_book: dict[str, list[dict]] = {}
    for unit in inv["units"]:
        if unit["kind"] == "companion":
            units_by_book.setdefault(unit["book"], []).append(unit)
    books = wanted or sorted(units_by_book)
    print(
        f"{'book':32} {'crea':>5} {'abil':>5} {'clas':>5} {'named':>6} "
        f"{'prerace':>8} {'prefix':>7} {'ORPHAN':>7}"
    )
    total_orphans = 0
    total_units = 0
    total_gated = 0
    for book in books:
        units = units_by_book.get(book, [])
        if not units:
            print(f"{book:32} carries no companion units")
            continue
        directory = dirs.get(book)
        if directory is None:
            print(f"{book:32} no source directory found")
            continue
        result = classify(book, units, directory)
        total_orphans += len(result["orphans"])
        total_gated += len(result["gated"])
        total_units += len(units)
        print(
            f"{book:32} {result['creatures']:5} {result['abilities']:5} "
            f"{result['classes']:5} {result['row_named']:6} {result['prerace']:8} "
            f"{result['prefix']:7} {len(result['orphans']):7}"
        )
        if wanted and result["orphans"]:
            for key in result["orphans"]:
                print(f"    ORPHAN {key}")
        if wanted and result["gated"]:
            for key, source_file, gate in result["gated"]:
                print(f"    GATED  {key} — {source_file} loaded under {gate}")
    print(f"\ntotal companion units in scope : {total_units}")
    print(f"orphan ability rows            : {total_orphans}")
    print(f"PRECAMPAIGN-gated on an uningested campaign : {total_gated}")
    print(f"reachable remainder            : {total_units - total_orphans - total_gated}")


if __name__ == "__main__":
    main()
