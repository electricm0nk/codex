#!/usr/bin/env python3
"""Transcribe one book's `companion` rows into a Rust table.

The companion analogue of ``scripts/transcribe_monster_tables.py``, written to
that file's rules because they are the ones that made the monster lane's output
trustworthy rather than merely plausible:

* **The unit set comes from ``docs/work-inventory.json``, not from the ``.lst``
  file.**  Every emitted record is one inventory unit, keyed by that unit's own
  ``corpus_key`` and ``name``, so the table reconciles with the inventory's
  predicate by construction rather than by a line count that would ship phantom
  records for ``.MOD``/``.COPY`` overlays.
* **Every emitted value is a substring of the cited row.**  Nothing is computed,
  defaulted or inferred; a token the row does not carry becomes ``None``.  In
  particular ``BONUS:STAT`` tokens are transcribed as *adjustments* and never
  summed into a score, and ``MONSTERCLASS:`` is carried verbatim rather than
  expanded into hit points — PCGen computes both at runtime and this program
  does not have that engine.

Usage::

    python3 scripts/transcribe_companion_tables.py inner_sea_combat

``PCGEN_CORPUS_ROOT`` may point at a local PCGen ``data/`` checkout; it defaults
to ``$HOME/workspace/repos/pcgen/data``.
"""

from __future__ import annotations

import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from classify_companion_rows import (  # noqa: E402
    bare_species,
    book_dirs,
    gated_on_an_uningested_campaign,
    precampaign_gates,
    prerace_owners,
    read_row,
    resolve_source_file,
    row_shape,
    special_ability_refs,
    token,
)

# The `TYPE:` segments this chassis models. Everything else on a row's `TYPE:`
# is kept verbatim in `type_segments` rather than dropped -- see
# `companion_chassis::CompanionAbilityRecord::type_segments`.
FACETS = {
    "CompanionAdvancement": "CompanionAdvancement",
    "SpecialQuality": "SpecialQuality",
    "SpecialAttack": "SpecialAttack",
}
DELIVERIES = {
    "Supernatural": "Supernatural",
    "Extraordinary": "Extraordinary",
    "SpellLike": "SpellLike",
}

FULL_ABILITY_RULE = "DisplayFullAbility"

# The emitted `use` line is derived from the symbols the emitted rows actually
# name, never written as a fixed line: Horror Adventures' single ability row
# carries no delivery segment, so importing `CompanionAbilityDelivery` there is
# an unused import, and `./scripts/verify.sh`'s clippy stage denies warnings.
IMPORT_PLACEHOLDER = "// __COMPANION_CHASSIS_IMPORTS__"
IMPORTABLE = (
    "CompanionAbilityDelivery",
    "CompanionAbilityFacet",
    "CompanionAbilityRecord",
    "CompanionRecord",
    "NaturalAttack",
    "Speed",
    "StatAdjustment",
)


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def rust_opt(value: str | None) -> str:
    return f"Some({rust_str(value)})" if value is not None else "None"


def rust_slice(values: list[str]) -> str:
    return "&[" + ", ".join(rust_str(v) for v in values) + "]"


def parse_speeds(row: list[str]) -> list[tuple[str, int]]:
    """`MOVE:Walk,30,Fly,40` -> [("Walk", 30), ("Fly", 40)]."""
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


def parse_size(row: list[str]) -> str | None:
    """The row's size letter, from either token shape the corpus uses.

    Inner Sea Combat spells it `SIZE:L`; Monster Codex's companion rows carry no
    `SIZE:` token at all and state the same fact as `FACT:BaseSize|M`. Reading
    only the first shape serves an empty size chip for 8 of the 15 registered
    creature rows while the corpus plainly states one -- the identical defect
    `transcribe_monster_tables.parse_size` was widened for.
    """
    direct = token(row, "SIZE:")
    if direct:
        return direct
    for field in row:
        if field.startswith("FACT:BaseSize|"):
            return field[len("FACT:BaseSize|") :] or None
    return None


def parse_reach(row: list[str]) -> int | None:
    raw = token(row, "REACH:")
    if raw is None:
        return None
    try:
        return int(raw.strip())
    except ValueError:
        return None


def is_die_expression(value: str) -> bool:
    return bool(re.fullmatch(r"0|\d*d\d+(?:[+-]\d+)?", value))


def parse_natural_attacks(row: list[str]) -> list[tuple[str, str | None]]:
    """Named attacks, with a die expression only where the row carries one.

    Same two token shapes `transcribe_monster_tables.parse_natural_attacks`
    reads, and the same rule that a name the corpus does not price carries
    `None` rather than an invented value.
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


def parse_stat_adjustments(row: list[str]) -> list[tuple[str, int]]:
    """`BONUS:STAT|DEX,WIS|4` -> [("DEX", 4), ("WIS", 4)].

    A multi-ability token is split into one record each, which is what PCGen
    itself does with it. A token whose amount is not an integer literal (a
    formula) is **skipped**, not guessed: this program has no formula
    interpreter (`decisions.md §24`) and a wrong number in an ability column is
    worse than an absent one.
    """
    out: list[tuple[str, int]] = []
    for field in row:
        if not field.startswith("BONUS:STAT|"):
            continue
        parts = field.split("|")
        if len(parts) < 3:
            continue
        abilities = [a.strip() for a in parts[1].split(",") if a.strip()]
        try:
            amount = int(parts[2].strip())
        except ValueError:
            continue
        for ability in abilities:
            out.append((ability, amount))
    return out


def parse_natural_armor(row: list[str]) -> int | None:
    """`BONUS:VAR|AC_Natural_Armor|4|TYPE=Base` -> 4.

    Only the `TYPE=Base` token is read. An advancement row's
    `TYPE=Base.STACK` / `TYPE=Base.REPLACE` token states how it combines with
    the base, which is a computation this chassis does not perform, so it is
    left to the ability record's own verbatim rendering rather than folded in.
    """
    for field in row:
        if not field.startswith("BONUS:VAR|AC_Natural_Armor|"):
            continue
        parts = field.split("|")
        if len(parts) < 4 or parts[3].strip() != "TYPE=Base":
            continue
        try:
            return int(parts[2].strip())
        except ValueError:
            continue
    return None


def parse_type_segments(row: list[str]) -> list[str]:
    raw = token(row, "TYPE:") or ""
    return [s for s in raw.split(".") if s]


def read_facet_and_delivery(segments: list[str]) -> tuple[str | None, str | None]:
    facet = None
    delivery = None
    for segment in segments:
        if facet is None and segment in FACETS:
            facet = segment
        elif delivery is None and segment in DELIVERIES:
            delivery = segment
    return facet, delivery


def is_prerequisite(entry: str) -> bool:
    """A `DESC:` trailing entry that is a prerequisite rather than a variable.

    Both polarities, because the negated spelling is the one
    `transcribe_monster_tables` originally missed and corpus-wide it occurs on
    650 `DESC:` tokens.
    """
    return entry.lstrip("!").startswith("PRE")


def parse_desc(row: list[str]) -> tuple[str | None, list[str]]:
    """The `DESC:` text a player should read, plus the variables its `%N` name.

    A row carrying two `DESC:` tokens gated on PCGen's own
    `PRERULE:1,DisplayFullAbility` serves the **full rules text**, never the
    summary -- the defect `transcribe_monster_tables.parse_desc` was widened for
    when Book of the Damned Volume 2 landed. A row carrying several under some
    other gate stops the transcription rather than being resolved by position.
    """
    descs = [f[len("DESC:") :] for f in row if f.startswith("DESC:")]
    if not descs:
        return None, []
    if len(descs) > 1:
        full = [
            d
            for d in descs
            if any(
                entry.startswith("PRERULE") and FULL_ABILITY_RULE in entry
                for entry in d.split("|")[1:]
            )
        ]
        if len(full) != 1:
            raise SystemExit(
                f"row carries {len(descs)} DESC: tokens and {len(full)} of them are gated on "
                f"{FULL_ABILITY_RULE}; the transcriber refuses to pick one by position. "
                f"Widen it deliberately. Tokens: {descs!r}"
            )
        descs = full
    parts = descs[0].split("|")
    return parts[0], [p for p in parts[1:] if p and not is_prerequisite(p)]


def transcribe(book: str) -> str:
    directory = book_dirs()[book]
    inventory = json.load(open("docs/work-inventory.json", encoding="utf-8"))
    units = [u for u in inventory["units"] if u["book"] == book and u["kind"] == "companion"]

    # A `.lst` the book's pcc loads only under a campaign this repo has not
    # ingested is out of this rule set's scope BY CONSTRUCTION, not by omission
    # — `decisions.md §47.2`, ruled for Horror Adventures' Occult-Adventures-
    # gated race-trait file and applied here to Bestiary 5's
    # `support/b5_races_companion_oa.lst`.  Excluded here rather than silently
    # missing, and named in the emitted module doc so the shortfall is a stated
    # claim a reader can check.
    gates = precampaign_gates(directory)
    gated = [u for u in units if gated_on_an_uningested_campaign(gates.get(u["source_file"]))]
    gated_keys = {u["corpus_key"] for u in gated}
    units = [u for u in units if u["corpus_key"] not in gated_keys]

    creatures = sorted(
        (u for u in units if row_shape(u["source_file"]) == "creature"),
        key=lambda u: u["source_line"],
    )
    abilities = sorted(
        (u for u in units if row_shape(u["source_file"]) == "ability"),
        key=lambda u: u["source_line"],
    )
    classes = [u for u in units if row_shape(u["source_file"]) == "class"]
    if classes:
        raise SystemExit(
            f"{book} carries {len(classes)} `*_classes_companion.lst` rows; the chassis models "
            "creature and ability rows only. Widen it deliberately."
        )
    if not creatures:
        raise SystemExit(f"{book} carries no companion creature rows")

    creature_keys = {u["corpus_key"] for u in creatures}
    creature_species = {bare_species(k): k for k in creature_keys}

    # Shape 5, DISPLAY-NAME namespacing -- see the matching block in
    # `classify_companion_rows.py`, whose ORPHAN column must agree with what
    # this transcriber drops. An ability's `<X> ~ <Y>` prefix may be the
    # creature's `OUTPUTNAME:` rather than its `KEY:`; read from the row's own
    # token, never inferred by unwrapping the key's parentheses.
    creature_display: dict[str, str] = {}
    for unit in creatures:
        display = token(
            read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"]),
            "OUTPUTNAME:",
        )
        if display:
            creature_display[display] = unit["corpus_key"]
    ability_by_key = {u["corpus_key"]: u for u in abilities}
    ability_by_name = {u["name"]: u for u in abilities}

    owners: dict[str, list[str]] = {u["corpus_key"]: [] for u in abilities}
    creature_ability_keys: dict[str, list[str]] = {}
    external: dict[str, list[str]] = {}
    creature_rows: dict[str, list[str]] = {}

    # Shape 1, row-named: iterated in the creature file's own row order.
    for unit in creatures:
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        creature_rows[unit["corpus_key"]] = row
        named = special_ability_refs(row)
        mine: list[str] = []
        outside: list[str] = []
        for ref in named:
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is None:
                outside.append(ref)
            elif hit["corpus_key"] not in mine:
                mine.append(hit["corpus_key"])
        creature_ability_keys[unit["corpus_key"]] = mine
        external[unit["corpus_key"]] = outside
        for key in mine:
            owners[key].append(unit["corpus_key"])

    # Shapes 2 and 3, in the abilities file's own row order -- never over a set,
    # whose iteration order would make the output differ run to run for no
    # corpus reason.
    for unit in abilities:
        key = unit["corpus_key"]
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        candidates: list[str] = []
        for owner in prerace_owners(row):
            candidates.append(creature_display.get(owner) or creature_species.get(owner, owner))
        if " ~ " in key:
            prefix = key.split(" ~ ")[0]
            candidates.append(creature_display.get(prefix) or creature_species.get(prefix, prefix))
        for candidate in candidates:
            if candidate in creature_keys and candidate not in owners[key]:
                owners[key].append(candidate)
                creature_ability_keys[candidate].append(key)

    # Shape 4, granted-by (`classify_companion_rows` §"The four ownership
    # shapes", `decisions.md §54.1`). An ability row that is itself owned may
    # carry shape 1's own `ABILITY:Special Ability|AUTOMATIC|<name>` token, and
    # then it — not the creature row — is what names `<name>`. The grant is
    # attributed to the granting row's OWNER creatures, because
    # `companion_chassis`'s both-directions link test types `owners` as creature
    # keys and the catalog serves an ability underneath the creature that
    # reaches it. The granting row is not dropped from the chain: it is a
    # registered record of this book in its own right, and its own `owners` say
    # which creature it hangs from.
    #
    # Seeded only from rows shapes 1-3 already owned, and run to a fixpoint in
    # the abilities file's own row order, so the output cannot depend on set
    # iteration order and an orphan can never grant reachability to an orphan.
    granted_by: dict[str, list[str]] = {}
    for unit in abilities:
        key = unit["corpus_key"]
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        named = []
        for ref in special_ability_refs(row):
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is not None and hit["corpus_key"] != key:
                named.append(hit["corpus_key"])
        granted_by[key] = named

    changed = True
    while changed:
        changed = False
        for unit in abilities:
            key = unit["corpus_key"]
            if not owners[key]:
                continue
            for target in granted_by[key]:
                for creature in owners[key]:
                    if creature not in owners[target]:
                        owners[target].append(creature)
                        creature_ability_keys[creature].append(target)
                        changed = True

    # Only ability rows WITH an owner are registered.  A row no creature row of
    # this book reaches is a record that would load and never be shown, so it is
    # dropped from the emitted table and named in the module doc below.
    #
    # Until round 4 this was a hard refusal, which was right while every
    # candidate book still had an orphan-free alternative.  `bestiary` was the
    # last such book (`decisions.md §54`), so from here the rule is the monster
    # lane's, adopted verbatim (`monster_chassis.rs` module doc, `decisions.md
    # §50`): **transcribe the linked subset, and carry the orphans as an
    # `OPEN_FINDINGS` entry naming their remedy** rather than emitting
    # unreachable rows or skipping the book entirely.  The dropped rows keep
    # their honest `not-ingested` status in the work inventory — this function
    # never touches that — so the shortfall stays a stated claim a reader can
    # check rather than a silent omission.
    orphans = sorted(k for k, v in owners.items() if not v)
    orphan_keys = set(orphans)
    if orphans:
        abilities = [u for u in abilities if u["corpus_key"] not in orphan_keys]

    out: list[str] = []
    out.append(f"//! {book} companion tables, transcribed verbatim from the book's own")
    out.append("//! PCGen `.lst` rows.")
    out.append("//!")
    out.append("//! GENERATED FILE -- do not hand-edit. Regenerate with")
    out.append(f"//! `python3 scripts/transcribe_companion_tables.py {book}`, whose unit set is")
    out.append("//! `docs/work-inventory.json`'s own units for this book rather than a raw")
    out.append("//! line count over the `.lst`.")
    out.append("//!")
    out.append("//! Sources, with the file AND line each record was read from carried per row:")
    # Counted per file rather than reported off `[0]`.  Until round 4 every
    # registered book had exactly one file per shape, so naming the first row's
    # file and the whole shape's count said the same thing; Bestiary 3 is the
    # first book with a `_companion` AND a `_familiar` file per shape, where the
    # old line claimed all 31 creature rows came from the 16-row file
    # (`decisions.md §56.2`).
    for shape_name, rows in (("creature", creatures), ("ability", abilities)):
        for source_file in sorted({u["source_file"] for u in rows}):
            n = sum(1 for u in rows if u["source_file"] == source_file)
            out.append(f"//!   * `{source_file}` -- {n} companion {shape_name} rows")
    if orphans:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- ability rows no creature row of this book owns, so"
        )
        out.append(
            "//! nothing could ever reach them on screen. Dropped rather than emitted"
        )
        out.append(
            "//! unreachable, and carried as a `reach_gate` `OPEN_FINDINGS` entry naming"
        )
        out.append(
            "//! their remedy (`decisions.md §50`, adopted from the monster lane; §56.1):"
        )
        for key in orphans:
            out.append(f"//!   * `{key}`")
    if gated:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- out of this rule set's scope by construction, not by"
        )
        out.append(
            "//! omission (`decisions.md §47.2`): the book's pcc loads these rows only under"
        )
        out.append("//! a campaign this repo has not ingested.")
        for unit in gated:
            out.append(
                f"//!   * `{unit['corpus_key']}` -- `{unit['source_file']}`, "
                f"`{gates[unit['source_file']]}`"
            )
    out.append("")
    # Filled in at the end from what the emitted rows actually name -- see the
    # note beside `IMPORT_PLACEHOLDER`.
    out.append(IMPORT_PLACEHOLDER)
    out.append("")
    out.append(f"/// Every {book} companion creature ({len(creatures)} rows).")
    out.append("pub(super) static COMPANIONS: &[CompanionRecord] = &[")
    for unit in creatures:
        key = unit["corpus_key"]
        row = creature_rows[key]
        speeds = parse_speeds(row)
        attacks = parse_natural_attacks(row)
        adjustments = parse_stat_adjustments(row)
        reach = parse_reach(row)
        armor = parse_natural_armor(row)
        out.append("    CompanionRecord {")
        out.append(f"        key: {rust_str(key)},")
        out.append(f"        name: {rust_str(unit['name'])},")
        out.append(f"        size: {rust_opt(parse_size(row))},")
        out.append(
            "        speeds: &["
            + ", ".join(f"Speed {{ mode: {rust_str(m)}, feet: {f} }}" for m, f in speeds)
            + "],"
        )
        out.append(f"        reach_feet: {'Some(' + str(reach) + ')' if reach is not None else 'None'},")
        out.append(f"        race_type: {rust_opt(token(row, 'RACETYPE:'))},")
        out.append(f"        race_subtype: {rust_opt(token(row, 'RACESUBTYPE:'))},")
        out.append(f"        monster_class: {rust_opt(token(row, 'MONSTERCLASS:'))},")
        out.append(f"        type_segments: {rust_slice(parse_type_segments(row))},")
        out.append(
            "        natural_attacks: &["
            + ", ".join(
                f"NaturalAttack {{ name: {rust_str(n)}, damage_dice: {rust_opt(d)} }}"
                for n, d in attacks
            )
            + "],"
        )
        out.append(
            "        stat_adjustments: &["
            + ", ".join(
                f"StatAdjustment {{ ability: {rust_str(a)}, amount: {v} }}" for a, v in adjustments
            )
            + "],"
        )
        out.append(
            f"        natural_armor: {'Some(' + str(armor) + ')' if armor is not None else 'None'},"
        )
        out.append(f"        source_page: {rust_opt(token(row, 'SOURCEPAGE:'))},")
        out.append(f"        ability_keys: {rust_slice(creature_ability_keys[key])},")
        out.append(f"        external_ability_refs: {rust_slice(external[key])},")
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")
    out.append(f"/// Every {book} companion ability record ({len(abilities)} rows).")
    out.append("pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[")
    for unit in abilities:
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        segments = parse_type_segments(row)
        facet, delivery = read_facet_and_delivery(segments)
        description, variables = parse_desc(row)
        adjustments = parse_stat_adjustments(row)
        out.append("    CompanionAbilityRecord {")
        out.append(f"        key: {rust_str(unit['corpus_key'])},")
        out.append(f"        name: {rust_str(unit['name'])},")
        out.append(
            "        facet: "
            + (f"Some(CompanionAbilityFacet::{facet})" if facet else "None")
            + ","
        )
        out.append(
            "        delivery: "
            + (f"Some(CompanionAbilityDelivery::{delivery})" if delivery else "None")
            + ","
        )
        out.append(f"        type_segments: {rust_slice(segments)},")
        out.append(f"        description: {rust_opt(description)},")
        out.append(f"        description_variables: {rust_slice(variables)},")
        out.append(
            "        stat_adjustments: &["
            + ", ".join(
                f"StatAdjustment {{ ability: {rust_str(a)}, amount: {v} }}" for a, v in adjustments
            )
            + "],"
        )
        out.append(f"        source_page: {rust_opt(token(row, 'SOURCEPAGE:'))},")
        out.append(f"        owners: {rust_slice(owners[unit['corpus_key']])},")
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")

    index = out.index(IMPORT_PLACEHOLDER)
    body = "\n".join(out[index + 1 :])
    used = [symbol for symbol in IMPORTABLE if re.search(rf"\b{symbol}\b", body)]
    out[index] = (
        "use crate::rules_core::rules_tables::companion_chassis::{"
        + ", ".join(used)
        + "};"
    )
    return "\n".join(out)


# A corpus book id is not always the name of the Rust module that holds its
# tables. `bestiary` is `beastiary1` on the engine side — a misspelling that
# predates this lane and that `decisions.md §44` records silently under-reporting
# 108 Bestiary 1 records once already. Writing `bestiary/companion_data.rs` would
# create a SECOND module for a book that already has one, and the second would
# compile, pass its own tests and be reachable from nothing.
#
# Mapped rather than renamed: renaming `beastiary1` is a repo-wide identifier
# change with no companion-lane content in it (`AGENTS.md`, "Do not expand
# scope"). Books absent from this map use their own id.
MODULE_DIR = {"bestiary": "beastiary1"}


def module_dir(book: str) -> str:
    return MODULE_DIR.get(book, book)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <book>")
    book = sys.argv[1]
    directory = f"src/rules_core/rules_tables/{module_dir(book)}"
    os.makedirs(directory, exist_ok=True)
    path = os.path.join(directory, "companion_data.rs")
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(transcribe(book))
    print(f"wrote {path}")


if __name__ == "__main__":
    main()
