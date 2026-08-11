#!/usr/bin/env python3
"""Transcribe one book's `monster` / `monster_ability` rows into a Rust table.

The Bonus Bestiary pilot (SD-29 Epic 5) produced its `monster_data.rs` with a
*throwaway* parser that was described in a receipt but never checked in, so the
next book in the lane had nothing to run.  This is that parser, made
reproducible and book-generic.  Adding a book is a row in ``BOOKS`` plus a run
of this script; nothing here knows anything book-specific.

Two properties make the output trustworthy rather than merely plausible:

* **The unit set comes from ``docs/work-inventory.json``, not from the ``.lst``
  file.**  A raw line count over ``isb_races.lst`` reads 45 where the inventory
  counts 40 — the difference is ``.MOD``/``.COPY`` overlays and the trap
  filters, and a transcriber that counted lines itself would ship phantom
  records.  Every emitted record is one inventory unit, keyed by that unit's own
  ``corpus_key`` and ``name``, so the table reconciles with the inventory's
  predicate by construction.
* **Every emitted value is a substring of the cited row.**  Nothing is computed,
  defaulted, or inferred; a token the row does not carry becomes ``None``.  The
  generator (`gen_book_cache.rs`) re-reads each cited line and asserts its first
  column still matches before citing it.

Usage::

    python3 scripts/transcribe_monster_tables.py monster_codex

``PCGEN_CORPUS_ROOT`` may point at a local PCGen ``data/`` checkout; it defaults
to ``$HOME/workspace/repos/pcgen/data``.
"""

from __future__ import annotations

import json
import os
import re
import sys

# Book id -> path of the book's directory relative to the PCGen `data/` root.
# The two `.lst` file names are read from the inventory units themselves, so
# they are deliberately NOT repeated here.
BOOKS = {
    "bonus_bestiary": "pathfinder/paizo/roleplaying_game/bonus_bestiary",
    "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex",
}

# The `TYPE:` first segment that names which facet of `monster_ability` a row
# is. Spelled exactly as the corpus spells it.
FACETS = {"SpecialAttack": "SpecialAttack", "SpecialQuality": "SpecialQuality"}
# The `TYPE:` segment naming how the ability is delivered, when the row says.
DELIVERIES = {
    "Supernatural": "Supernatural",
    "Extraordinary": "Extraordinary",
    "SpellLike": "SpellLike",
}


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def read_row(path: str, line_no: int) -> list[str]:
    """The 1-based line at `line_no`, split into its tab-separated tokens."""
    with open(path, encoding="utf-8", errors="replace") as handle:
        line = handle.read().split("\n")[line_no - 1]
    return [token.strip() for token in line.split("\t") if token.strip()]


def token(row: list[str], prefix: str) -> str | None:
    """The first token with this prefix, with the prefix stripped."""
    for field in row:
        if field.startswith(prefix):
            return field[len(prefix) :]
    return None


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def rust_opt(value: str | None) -> str:
    return f"Some({rust_str(value)})" if value is not None else "None"


def rust_slice(values: list[str]) -> str:
    return "&[" + ", ".join(rust_str(v) for v in values) + "]"


def parse_speeds(row: list[str]) -> list[tuple[str, int]]:
    """`MOVE:Walk,30,Burrow,10` -> [("Walk", 30), ("Burrow", 10)]."""
    raw = token(row, "MOVE:")
    if not raw:
        return []
    parts = [p.strip() for p in raw.split(",") if p.strip()]
    speeds = []
    for i in range(0, len(parts) - 1, 2):
        try:
            speeds.append((parts[i], int(parts[i + 1])))
        except ValueError:
            continue
    return speeds


def is_die_expression(value: str) -> bool:
    """Whether a `NATURALATTACKS:` damage field is a die expression at all.

    Monster Codex's Seru carries `NATURALATTACKS:Venom,…,*1,Poison` — the
    corpus puts the word *Poison* where Bonus Bestiary puts `1d6`. Writing that
    into `damage_dice` would print "Venom Poison" on the sheet in the slot a
    player reads as damage. The field is accepted only when it is a die
    expression or the literal `0` (a real no-damage attack, which Bonus
    Bestiary's Allip genuinely has); anything else leaves `damage_dice` `None`
    and the attack prints as a name, which is what this lane already does for
    the 13 Bonus Bestiary attacks the corpus never prices.
    """
    return bool(re.fullmatch(r"0|\d*d\d+(?:[+-]\d+)?", value))


def parse_natural_attacks(row: list[str]) -> list[tuple[str, str | None]]:
    """Named attacks, with a die expression only where the row carries one.

    Two token shapes name an attack, and this program's rule is that a name the
    corpus does not price carries `None` rather than an invented value:

    * `NATURALATTACKS:Incorporeal touch,Touch,*1,0` — the 4th comma field is the
      damage. A row may carry **several** of these tokens (Seru carries two).
    * `ABILITY:Internal|AUTOMATIC|Bite` — a bare cross-reference with no dice at
      any hop. A **namespaced** name in that list (`Racial Traits ~ Seru`) is a
      reference to another record, not an attack, and is skipped: it is how
      Monster Codex attaches a racial-trait container, and transcribing it would
      have put "Racial Traits ~ Seru" in the attack row on screen.
    """
    attacks: list[tuple[str, str | None]] = []
    seen: set[str] = set()
    for field in row:
        if not field.startswith("NATURALATTACKS:"):
            continue
        for entry in field[len("NATURALATTACKS:") :].split("|"):
            fields = [f.strip() for f in entry.split(",")]
            if not fields or not fields[0]:
                continue
            name = fields[0]
            dice = fields[3] if len(fields) > 3 and fields[3] else None
            if dice is not None and not is_die_expression(dice):
                dice = None
            if name not in seen:
                seen.add(name)
                attacks.append((name, dice))
    for field in row:
        if not field.startswith("ABILITY:Internal|AUTOMATIC|"):
            continue
        for name in field.split("|")[2:]:
            name = name.strip()
            if not name or name.startswith("PRE") or "=" in name or " ~ " in name:
                continue
            if name not in seen:
                seen.add(name)
                attacks.append((name, None))
    return attacks


def parse_size(row: list[str]) -> str | None:
    """The row's size letter, from either token shape the corpus uses.

    Bonus Bestiary spells it `SIZE:M`; Monster Codex spells the same fact
    `FACT:BaseSize|S` and carries no `SIZE:` token at all. Reading only the
    first shape would have served an empty size chip for every Monster Codex
    row while the corpus plainly states one.
    """
    direct = token(row, "SIZE:")
    if direct:
        return direct
    for field in row:
        if field.startswith("FACT:BaseSize|"):
            return field[len("FACT:BaseSize|") :] or None
    return None


def parse_special_ability_refs(row: list[str]) -> list[str]:
    """Keys named by the row's `ABILITY:Special Ability|AUTOMATIC|…` tokens."""
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


def parse_desc(row: list[str]) -> tuple[str | None, list[str]]:
    """The `DESC:` text plus the trailing variable list its `%N` refer to."""
    raw = token(row, "DESC:")
    if raw is None:
        return None, []
    parts = raw.split("|")
    return parts[0], [p for p in parts[1:] if p and not p.startswith("PRE")]


def parse_type(row: list[str]) -> tuple[str, str | None, list[str]]:
    """`TYPE:SpecialAttack.Supernatural.Aura` -> facet, delivery, traits."""
    raw = token(row, "TYPE:") or ""
    segments = [s for s in raw.split(".") if s]
    facet = None
    delivery = None
    traits: list[str] = []
    for segment in segments:
        if facet is None and segment in FACETS:
            facet = segment
        elif delivery is None and segment in DELIVERIES:
            delivery = segment
        else:
            traits.append(segment)
    if facet is None:
        raise SystemExit(
            f"row carries no `monster_ability` facet in TYPE:{raw!r} — the chassis "
            "models SpecialAttack/SpecialQuality only; widen it deliberately"
        )
    return facet, delivery, traits


def transcribe(book: str) -> str:
    book_relative = BOOKS[book]
    root = os.path.join(corpus_root(), book_relative)
    inventory = json.load(open("docs/work-inventory.json", encoding="utf-8"))
    units = [
        u
        for u in inventory["units"]
        if u["book"] == book and u["kind"] in ("monster", "monster_ability")
    ]
    monsters = sorted(
        (u for u in units if u["kind"] == "monster"), key=lambda u: u["source_line"]
    )
    abilities = sorted(
        (u for u in units if u["kind"] == "monster_ability"),
        key=lambda u: u["source_line"],
    )
    if not monsters and not abilities:
        raise SystemExit(f"{book} carries no monster/monster_ability units")

    ability_keys = {u["corpus_key"] for u in abilities}
    monster_keys = {u["corpus_key"] for u in monsters}

    # The chassis link, derived from the corpus in the two shapes the corpus
    # actually uses. Neither is invented: a monster's `ABILITY:Special Ability`
    # token names its abilities outright, and a namespaced ability key
    # (`Seru ~ Poison`) names its owner in its own first segment.
    owners: dict[str, list[str]] = {key: [] for key in ability_keys}
    monster_ability_keys: dict[str, list[str]] = {}
    external: dict[str, list[str]] = {}
    monster_rows: dict[str, list[str]] = {}
    for unit in monsters:
        row = read_row(os.path.join(root, unit["source_file"]), unit["source_line"])
        monster_rows[unit["corpus_key"]] = row
        named = parse_special_ability_refs(row)
        mine = [n for n in named if n in ability_keys]
        monster_ability_keys[unit["corpus_key"]] = mine
        external[unit["corpus_key"]] = [n for n in named if n not in ability_keys]
        for key in mine:
            owners[key].append(unit["corpus_key"])
    # Iterated in the abilities file's own row order, never over a set: the
    # emitted `ability_keys` slice is compared verbatim by tests and by the
    # generated corpus records, so a set's iteration order would make the
    # output differ run to run for no corpus reason.
    for key in [u["corpus_key"] for u in abilities]:
        if " ~ " in key:
            prefix = key.split(" ~ ")[0]
            if prefix in monster_keys and prefix not in owners[key]:
                owners[key].append(prefix)
                monster_ability_keys[prefix].append(key)

    out: list[str] = []
    out.append(f"//! {book} monster + monster-ability tables, transcribed verbatim")
    out.append("//! from the book's own PCGen `.lst` rows.")
    out.append("//!")
    out.append("//! GENERATED FILE -- do not hand-edit. Regenerate with")
    out.append(f"//! `python3 scripts/transcribe_monster_tables.py {book}`, whose unit set is")
    out.append("//! `docs/work-inventory.json`'s own units for this book rather than a raw")
    out.append("//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the")
    out.append("//! inventory correctly excludes).")
    out.append("//!")
    out.append("//! Sources, with the line each record was read from carried per row:")
    if monsters:
        out.append(
            f"//!   * `{monsters[0]['source_file']}` -- {len(monsters)} monster rows"
        )
    if abilities:
        out.append(
            f"//!   * `{abilities[0]['source_file']}` -- {len(abilities)} monster-ability rows"
        )
    out.append("")
    out.append(
        "use crate::rules_core::rules_tables::monster_chassis::{"
        "MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, "
        "NaturalAttack, Speed};"
    )
    out.append("")
    out.append(f"/// Every {book} monster stat block ({len(monsters)} rows).")
    out.append("pub(super) static MONSTERS: &[MonsterStatBlock] = &[")
    for unit in monsters:
        key = unit["corpus_key"]
        row = monster_rows[key]
        speeds = parse_speeds(row)
        attacks = parse_natural_attacks(row)
        out.append("    MonsterStatBlock {")
        out.append(f"        key: {rust_str(key)},")
        out.append(f"        name: {rust_str(unit['name'])},")
        out.append(f"        size: {rust_opt(parse_size(row))},")
        out.append(
            "        speeds: &["
            + ", ".join(
                f'Speed {{ mode: {rust_str(m)}, feet: {f} }}' for m, f in speeds
            )
            + "],"
        )
        out.append(f"        race_type: {rust_opt(token(row, 'RACETYPE:'))},")
        out.append(f"        race_subtype: {rust_opt(token(row, 'RACESUBTYPE:'))},")
        out.append(f"        challenge_rating: {rust_opt(token(row, 'CR:'))},")
        out.append(f"        monster_class: {rust_opt(token(row, 'MONSTERCLASS:'))},")
        out.append(f"        source_page: {rust_opt(token(row, 'SOURCEPAGE:'))},")
        out.append(
            "        natural_attacks: &["
            + ", ".join(
                f"NaturalAttack {{ name: {rust_str(n)}, damage_dice: {rust_opt(d)} }}"
                for n, d in attacks
            )
            + "],"
        )
        out.append(
            f"        ability_keys: {rust_slice(monster_ability_keys[key])},"
        )
        out.append(f"        external_ability_refs: {rust_slice(external[key])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")
    out.append(f"/// Every {book} monster-ability record ({len(abilities)} rows).")
    out.append("pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[")
    for unit in abilities:
        row = read_row(os.path.join(root, unit["source_file"]), unit["source_line"])
        facet, delivery, traits = parse_type(row)
        description, variables = parse_desc(row)
        out.append("    MonsterAbilityRecord {")
        out.append(f"        key: {rust_str(unit['corpus_key'])},")
        out.append(f"        name: {rust_str(unit['name'])},")
        out.append(f"        facet: MonsterAbilityFacet::{facet},")
        out.append(
            "        delivery: "
            + (
                f"Some(MonsterAbilityDelivery::{delivery})"
                if delivery
                else "None"
            )
            + ","
        )
        out.append(f"        traits: {rust_slice(traits)},")
        out.append(f"        description: {rust_opt(description)},")
        out.append(f"        description_variables: {rust_slice(variables)},")
        out.append(f"        source_page: {rust_opt(token(row, 'SOURCEPAGE:'))},")
        out.append(f"        owners: {rust_slice(owners[unit['corpus_key']])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")
    return "\n".join(out)


def main() -> None:
    if len(sys.argv) != 2 or sys.argv[1] not in BOOKS:
        raise SystemExit(f"usage: {sys.argv[0]} <{'|'.join(sorted(BOOKS))}>")
    book = sys.argv[1]
    path = f"src/rules_core/rules_tables/{book}/monster_data.rs"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(transcribe(book))
    print(f"wrote {path}")


if __name__ == "__main__":
    main()
