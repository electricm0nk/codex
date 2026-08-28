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
    relay_ownership,
    resolve_owner,
    resolve_source_file,
    row_shape,
    special_ability_refs,
    species_index,
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
    "CompanionClassRecord",
    "CompanionDescriptionVariant",
    "CompanionRecord",
    "NaturalAttack",
    "NaturalAttackDamageBonus",
    "SkillAbilityDiffBonus",
    "Speed",
    "StatAdjustment",
)


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def rust_opt(value: str | None) -> str:
    return f"Some({rust_str(value)})" if value is not None else "None"


def rust_pair_slice(pairs: list[tuple[str, str]]) -> str:
    """Emit `&[(&str, &str)]` for Shape 8 cross-book ownership grants."""
    if not pairs:
        return "&[]"
    return "&[" + ", ".join(f"({rust_str(a)}, {rust_str(b)})" for a, b in pairs) + "]"


def rust_slice(values: list[str]) -> str:
    return "&[" + ", ".join(rust_str(v) for v in values) + "]"


PI_MARKER_RS = "src/rules_core/shape_b_v1.rs"


def redacted_pi_marker() -> str:
    """`shape_b_v1::REDACTED_PI_MARKER`, parsed out of the Rust source.

    **Derived, never re-typed** -- `transcribe_monster_tables.redacted_pi_marker`'s
    identical function, for the identical reason: a hand-copied literal
    drifts silently the first time the const changes, and this is the exact
    string a `DESCISPI:YES` ability's redacted `description` field ships
    instead of its declared-PI prose (`pi_screening::classify_optional_
    field_declared`'s own redaction value, `decisions.md §39.4`/`§53`,
    applied by hand here because this transcriber emits a Rust literal table
    rather than a JSON record with a `license`/`pi_field`/`pi_marker` trio to
    route through the shared reader).
    """
    text = open(PI_MARKER_RS, encoding="utf-8").read()
    match = re.search(r'REDACTED_PI_MARKER:\s*&str\s*=\s*"([^"]*)"', text)
    if not match:
        raise SystemExit(
            f"{PI_MARKER_RS}: could not find `REDACTED_PI_MARKER` -- the const's "
            "shape changed and this parser must be updated with it"
        )
    return match.group(1)


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


def parse_natural_attack_damage_bonuses(row: list[str]) -> list[tuple[str, str]]:
    """`BONUS:WEAPONPROF=Bite|DAMAGE|max(0,(STR/2))` -> [("Bite", "max(0,(STR/2))")].

    The token, verbatim — the attack selector and the formula half, neither
    normalised nor evaluated. Interpreting it is
    `derived_evaluator_fixture_check::parse_companion_strength_damage`'s job;
    transcribing it is this function's, exactly as `parse_stat_adjustments`
    transcribes `BONUS:STAT` without computing a score.

    Only the `|DAMAGE|` sub-token is read. A row's other `BONUS:WEAPONPROF=`
    tokens state different quantities (`|DAMAGESIZE|1` is a damage-DIE size
    step, not a damage bonus, and `bestiary_3`'s companion rows carry it where
    they carry no `|DAMAGE|` at all) and folding them together would report one
    quantity under another's name.
    """
    out: list[tuple[str, str]] = []
    for field in row:
        if not field.startswith("BONUS:WEAPONPROF="):
            continue
        parts = field.split("|")
        if len(parts) < 3 or parts[1].strip() != "DAMAGE":
            continue
        attack = parts[0][len("BONUS:WEAPONPROF=") :].strip()
        formula = "|".join(parts[2:]).strip()
        if not attack or not formula:
            continue
        out.append((attack, formula))
    return out


def parse_skill_ability_diff_bonuses(row: list[str]) -> list[tuple[list[str], str]]:
    """`BONUS:SKILL|Climb,Swim|DEX-STR` -> [(["Climb", "Swim"], "DEX-STR")].

    Only the ARITHMETIC shape is read -- a formula half containing a `-`
    (an ability-score difference). A row's other `BONUS:SKILL` tokens state a
    flat `TYPE=Racial` number (`BONUS:SKILL|Perception|4|TYPE=Racial`), a
    different and already-static quantity this seam does not model; folding
    the two together under one field would report one quantity under
    another's name, exactly the discipline
    `parse_natural_attack_damage_bonuses` states for `|DAMAGE|` vs
    `|DAMAGESIZE|`.

    Corpus-wide (`grep BONUS:SKILL` over every registered book's races file,
    2026-08-19) the arithmetic shape is `DEX-STR` in all 136 occurrences and
    always paired with the identical skill list `Climb,Swim` -- re-derived,
    not assumed, and the reason the parser accepts a bare `<ABBR>-<ABBR>`
    shape rather than hard-coding the one string seen.
    """
    out: list[tuple[list[str], str]] = []
    for field in row:
        if not field.startswith("BONUS:SKILL|"):
            continue
        parts = field.split("|")
        if len(parts) < 3:
            continue
        skills = [s.strip() for s in parts[1].split(",") if s.strip()]
        formula = parts[2].strip()
        if not skills or not formula or "-" not in formula:
            # No arithmetic term -- a flat `TYPE=Racial` bonus, out of scope.
            continue
        out.append((skills, formula))
    return out


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


def row_states_modelled_content(row: list[str]) -> bool:
    """Does this ability row state anything `CompanionAbilityRecord` can hold?

    The three tokens that become content on the card: `TYPE:` (which is also
    where `facet` and `delivery` are read from), `DESC:` (the rules text, gated
    or not) and `BONUS:` (the stat adjustments).  `SOURCEPAGE:` is deliberately
    NOT one of them -- a citation says where to read the rule, not what it says
    -- and neither is `ASPECT:`, which says a great deal but which no chassis in
    this program models yet.  See the screen in `transcribe` for both.
    """
    return any(
        field.startswith(("TYPE:", "DESC:", "BONUS:")) for field in row
    )


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


def split_desc(raw: str) -> tuple[str, list[str], list[str]]:
    """One `DESC:` token -> (text, `%N` variables, `PRE…` conditions)."""
    parts = raw.split("|")
    variables = [p for p in parts[1:] if p and not is_prerequisite(p)]
    conditions = [p for p in parts[1:] if p and is_prerequisite(p)]
    return parts[0], variables, conditions


def parse_desc(row: list[str]) -> tuple[str | None, list[str], list[tuple[str, list[str], list[str]]]]:
    """The `DESC:` text a player should read, its `%N` variables, its variants.

    A row carrying two `DESC:` tokens gated on PCGen's own
    `PRERULE:1,DisplayFullAbility` serves the **full rules text**, never the
    summary -- the defect `transcribe_monster_tables.parse_desc` was widened for
    when Book of the Damned Volume 2 landed.

    Any OTHER row carrying several was, until round 6, a hard refusal: the
    transcriber would not pick one by position and had nowhere else to put the
    rest.  Ultimate Wilderness is the book that made that refusal load-bearing
    -- **22** of its ability rows carry between 2 and 9 `DESC:` tokens, each
    gated on a different `PREVARGTEQ:`/`PREVARLT:`/`PREALIGN:` predicate, and
    they are the rows that carry `Poison`, `Constrict`, `Breath Weapon` and
    `Camouflage`.  Dropping them would have shipped creature cards whose
    abilities have no text; picking one would have shipped the wrong text for
    every character on the other side of the gate (`decisions.md §61.1`).

    So all of them are carried, in row order, each with its own gate verbatim,
    and NONE is evaluated here.  The first return value stays the row's single
    UNGATED token when it has exactly one -- so the ordinary row, and every
    record the lane already shipped, is byte-identical -- and is ``None`` when
    every token is conditional, which is the honest state for a row that states
    no unconditional rules text.
    """
    descs = [f[len("DESC:") :] for f in row if f.startswith("DESC:")]
    if not descs:
        return None, [], []
    if len(descs) > 1:
        full = [
            d
            for d in descs
            if any(
                entry.startswith("PRERULE") and FULL_ABILITY_RULE in entry
                for entry in d.split("|")[1:]
            )
        ]
        if len(full) == 1:
            text, variables, _ = split_desc(full[0])
            return text, variables, []
        variants = [split_desc(d) for d in descs]
        ungated = [v for v in variants if not v[2]]
        if len(ungated) == 1:
            return ungated[0][0], ungated[0][1], variants
        return None, [], variants
    text, variables, _ = split_desc(descs[0])
    return text, variables, []


def tokens_all(row: list[str], prefix: str) -> list[str]:
    """Every field's payload starting with `prefix`, in row order.

    `classify_companion_rows.token` returns only the FIRST match -- right for
    every single-valued token this transcriber reads, wrong for `ABILITY:`,
    which a `*_classes_companion.lst` row states more than once (`Vermin
    Companion` carries two: `Special Ability|...` and `Internal|...`).
    """
    return [f[len(prefix) :] for f in row if f.startswith(prefix)]


def parse_hit_dice(row: list[str]) -> int | None:
    raw = token(row, "HD:")
    return int(raw) if raw is not None and raw.isdigit() else None


def parse_class_row(row: list[str]) -> dict:
    """One `*_classes_companion.lst` row -> `CompanionClassRecord` fields.

    Handles BOTH shapes this file carries: a `CLASS:<name>` header row (most
    fields populated) and a bare `###Block: Level Advancement` line (`key` is
    a level number, every field but `ability_grants` empty) -- see
    `companion_chassis::CompanionClassRecord`'s own doc for why the tokenizer
    treats the second as a record in its own right.
    """
    return {
        "output_name": token(row, "OUTPUTNAME:"),
        "hit_dice": parse_hit_dice(row),
        "max_level": token(row, "MAXLEVEL:"),
        "type_segments": parse_type_segments(row),
        "visible_no": token(row, "VISIBLE:") == "NO",
        "source_page": token(row, "SOURCEPAGE:"),
        "ability_grants": tokens_all(row, "ABILITY:"),
        "fact_class_type": next(
            (f[len("FACT:ClassType|") :] for f in row if f.startswith("FACT:ClassType|")),
            None,
        ),
    }


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
    # ---- class-row build (`decisions.md §65.1`, `§17`) ----
    #
    # Through `AT-34-E3-001` cycle 4 this was DROP-AND-NAME: a `raise
    # SystemExit` until round 8, then a screen that named the rows and left
    # them `engine-does-not-hold` rather than modelling them. Round 8's own
    # comment (kept below, now historical) declared a genuine level-
    # progression record type as the eventual fix and deliberately did not
    # build it. This cycle builds it: `CompanionClassRecord` (`companion_
    # chassis.rs`) is neither a creature (no `SIZE:`, no `MOVE:`, no natural
    # attacks) nor an ability (no `DESC:`) — it carries `HD:`/`MAXLEVEL:`/
    # `ABILITY:` grants verbatim and computes nothing, the same discipline
    # `CompanionRecord::monster_class`'s own doc states for the identical
    # shape read from the creature side.
    #
    # `row_shape` sorts a `_classes_` file into neither `creatures` nor
    # `abilities` above, so these units were always out of both tables; they
    # are gathered here into their own list instead, sorted by source line
    # exactly like the two lists above.
    class_units = sorted(
        (u for u in units if row_shape(u["source_file"]) == "class"),
        key=lambda u: u["source_line"],
    )
    classes = [u["corpus_key"] for u in class_units]
    if classes:
        print(
            f"{book}: {len(classes)} `*_classes_companion.lst` CLASS row(s) transcribed as "
            "CompanionClassRecord (a level progression -- not a creature, not an ability): "
            + ", ".join(classes),
            file=sys.stderr,
        )
    if not creatures:
        raise SystemExit(f"{book} carries no companion creature rows")

    # ---- delta-row screen, CREATURE half (`decisions.md §63.1`) ----
    #
    # `§59.2` built this screen for ABILITY rows and ran it over `abilities`
    # alone, because Bestiary 4 -- the book that forced it -- carries `.COPY=`
    # only there. `core_essentials` is the first companion book whose CREATURE
    # rows carry it, 22 of them: `ce_races_familiar_cr.lst:33` reads
    # `Bat.COPY=Bat (Celestial)` and carries `OUTPUTNAME:`, `TEMPLATE:` and
    # `KIT:` -- no `SIZE:`, no `MOVE:`, no `MONSTERCLASS:`. Transcribed verbatim
    # it ships a creature card with a name and nothing else, which is the exact
    # stub class the ability half exists to prevent.
    #
    # Screened HERE rather than beside the ability screen below, because every
    # ownership index downstream is derived from `creatures`, and two of them
    # are actively wrong if a delta row is in the set:
    #
    # * `creature_display` (shape 5) keys on `OUTPUTNAME:`, and all 22 of these
    #   rows carry the BASE creature's display name -- `Bat (Celestial)` and
    #   `Bat (Fiendish)` both say `OUTPUTNAME:Bat`. Left in, they overwrite each
    #   other in a dict and an ability keyed `Bat ~ …` would be attributed to
    #   whichever delta row was read last.
    # * `creature_species` (shape 3) maps `bare_species` -> every claimant, so a
    #   delta row would stand as an owner of record for rows it does not define.
    #
    # An ability reachable ONLY from a dropped creature therefore loses its last
    # owner here and falls through to the orphan pass below, which is the right
    # disposition: it is a row this book's shipped creatures do not reach.
    creature_deltas = sorted(
        u["corpus_key"] for u in creatures if u.get("origin") in ("copy", "mod_only")
    )
    creature_delta_kinds = {
        u["corpus_key"]: u["origin"]
        for u in creatures
        if u.get("origin") in ("copy", "mod_only")
    }
    if creature_deltas:
        dropped = set(creature_deltas)
        creatures = [u for u in creatures if u["corpus_key"] not in dropped]
        if not creatures:
            raise SystemExit(
                f"{book}: every companion creature row is a `.COPY=`/`.MOD` delta; "
                "there is no record for this chassis to transcribe"
            )
        print(
            f"{book}: {len(creature_deltas)} delta CREATURE row(s) NOT transcribed "
            "(a `.COPY=`/`.MOD` row states a delta on another record, not a record): "
            + ", ".join(creature_deltas),
            file=sys.stderr,
        )

    # ---- Product Identity screen, CREATURE half, before the ownership
    # indices below are built ----
    #
    # Adopted from `transcribe_monster_tables`'s Product Identity screen
    # (`decisions.md §39`/`§53`, `SD30-E3-F3-001`), applied at the SAME early
    # point the `.COPY=`/`.MOD` delta screen immediately above states its own
    # placement by: `creature_species`/`creature_display`, built below, are
    # both derived from `creatures`, so a row that should not ship must be
    # gone before either index sees it. `NAMEISPI:YES` DROPS the row rather
    # than redacting it -- a name cannot be redacted without breaking the
    # record's own identity and every `<Creature> ~ <Ability>` key that
    # references it, PCGen's own rule `decisions.md §50.3`/`§53.2`
    # independently converged on. `CompanionRecord`'s own fields
    # (`companion_chassis::CompanionRecord`) are all structural -- no
    # free-text description at all -- so there is no `DESCISPI:YES` field to
    # redact at this half; only the name-drop rule applies here. This book's
    # own re-derived source exposure at time of writing is zero
    # (`decisions.md §39.2` corpus-wide sweep, re-run this cycle).
    pi_dropped_creatures = sorted(
        u["corpus_key"]
        for u in creatures
        if token(
            read_row(resolve_source_file(directory, u["source_file"]), u["source_line"]),
            "NAMEISPI:",
        )
        == "YES"
    )
    if pi_dropped_creatures:
        dropped = set(pi_dropped_creatures)
        creatures = [u for u in creatures if u["corpus_key"] not in dropped]
        if not creatures:
            raise SystemExit(
                f"{book}: every companion creature row declares NAMEISPI:YES; "
                "there is no record for this chassis to transcribe"
            )
        print(
            f"{book}: {len(pi_dropped_creatures)} creature row(s) NOT transcribed "
            "(NAMEISPI:YES -- a declared name cannot be redacted, mirrors "
            "transcribe_monster_tables's rule): " + ", ".join(pi_dropped_creatures),
            file=sys.stderr,
        )

    creature_keys = {u["corpus_key"] for u in creatures}
    # `<species>` -> EVERY creature row claiming it, in row order. A list rather
    # than one key, and derived from the ORDERED creature list rather than from
    # `creature_keys` (a set) -- see `classify_companion_rows.species_index` and
    # `decisions.md §59.3`. Bestiary 4 ships `Almiraj` AND `Familiar (Almiraj)`,
    # and the old comprehension picked between them by set iteration order,
    # producing a different table on every run.
    creature_species = species_index(creatures)

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
            candidates.extend(resolve_owner(owner, creature_display, creature_species))
        if " ~ " in key:
            prefix = key.split(" ~ ")[0]
            candidates.extend(resolve_owner(prefix, creature_display, creature_species))
        for candidate in candidates:
            if candidate in creature_keys and candidate not in owners[key]:
                owners[key].append(candidate)
                creature_ability_keys[candidate].append(key)

    # Shape 6, relay rows (`classify_companion_rows`, `decisions.md §59.1`).
    # The owner is stated across a corpus row that is NOT an inventory unit:
    # Bestiary 4's `Familiar (Giant Flea)` names `Racial Traits ~ Flea (Giant)`
    # (a `CATEGORY:Internal` row) and THAT row names `Flea (Giant) ~ Disease`.
    # Applied BEFORE the shape-4 fixpoint below, so a relay-owned row can go on
    # to grant through shape 4 like any other owned row.
    #
    # The attribution is the same as shape 4's: the CREATURE that reaches the
    # relay, because `companion_chassis`'s both-directions link test types
    # `owners` as creature keys. The relay itself is never emitted -- it is not
    # a unit, so it has no record to be emitted as, and inventing one would put
    # a row on the wire that `docs/work-inventory.json` does not count.
    for key, relay_owners in relay_ownership(
        directory, creatures, abilities, creature_rows
    ).items():
        for creature in relay_owners:
            if creature in creature_keys and creature not in owners[key]:
                owners[key].append(creature)
                creature_ability_keys[creature].append(key)

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

    # Shape 7, BOOK-WIDE grant (`AT-34-E3-001`, `decisions.md §66`). Shapes 1-6
    # all attribute ownership from something a CREATURE ROW itself states
    # (`ABILITY:`, `PRERACE:`, a namespaced `KEY:` prefix, a granting row, a
    # relay row, an `OUTPUTNAME:`). Core Rulebook's own "Animal Companion"
    # progression table -- the generic feat pool (`Animal Companion Feat ~
    # …`), trick/training pool (`Animal Trick ~ …`, `Animal Training ~ …`) and
    # by-level bonus table (`Animal Companion ~ …`, `Companion ~ …`,
    # `Companion Stat ~ …`) -- is never claimed by any ONE creature row,
    # because the corpus states it exactly once, generically, for the whole
    # `CLASS:Companion` chassis (`cr_classes_companion.lst`) every one of this
    # book's registered creatures shares (every row here carries
    # `MONSTERCLASS:Companion:…`). That is a true, corpus-backed fact about
    # ALL of them, not an invented link to one: PF1's own Animal Companion
    # rules (CRB p.52-55) grant this identical table to every companion,
    # regardless of species. `BOOK_WIDE_GRANTS` is an exact, closed key set —
    # never a prefix heuristic — so a future unrelated orphan can never
    # silently ride this shape.
    BOOK_WIDE_GRANTS: dict[str, set[str]] = {
        "core_rulebook": {
            "+2 to Dexterity and Constitution",
            "Animal Companion Feat ~ Acrobatic",
            "Animal Companion Feat ~ Agile Maneuvers",
            "Animal Companion Feat ~ Armor Proficiency (Heavy)",
            "Animal Companion Feat ~ Armor Proficiency (Light)",
            "Animal Companion Feat ~ Armor Proficiency (Medium)",
            "Animal Companion Feat ~ Athletic",
            "Animal Companion Feat ~ Blind-Fight",
            "Animal Companion Feat ~ Combat Reflexes",
            "Animal Companion Feat ~ Diehard",
            "Animal Companion Feat ~ Dodge",
            "Animal Companion Feat ~ Endurance",
            "Animal Companion Feat ~ Feat",
            "Animal Companion Feat ~ GM Feat",
            "Animal Companion Feat ~ Great Fortitude",
            "Animal Companion Feat ~ Improved Bull Rush",
            "Animal Companion Feat ~ Improved Initiative",
            "Animal Companion Feat ~ Improved Natural Armor",
            "Animal Companion Feat ~ Improved Natural Attack",
            "Animal Companion Feat ~ Improved Overrun",
            "Animal Companion Feat ~ Intimidating Prowess",
            "Animal Companion Feat ~ Iron Will",
            "Animal Companion Feat ~ Lightning Reflexes",
            "Animal Companion Feat ~ Mobility",
            "Animal Companion Feat ~ Power Attack",
            "Animal Companion Feat ~ Run",
            "Animal Companion Feat ~ Skill Focus",
            "Animal Companion Feat ~ Spring Attack",
            "Animal Companion Feat ~ Stealthy",
            "Animal Companion Feat ~ Toughness",
            "Animal Companion Feat ~ Weapon Finesse",
            "Animal Companion Feat ~ Weapon Focus",
            "Animal Companion ~ AC Bonus",
            "Animal Companion ~ Ability Score Increase",
            "Animal Companion ~ Bonus Tricks",
            "Animal Companion ~ Devotion",
            "Animal Companion ~ Evasion",
            "Animal Companion ~ Improved Evasion",
            "Animal Companion ~ Link",
            "Animal Companion ~ Multiattack",
            "Animal Companion ~ Share Spells",
            "Animal Companion ~ Spell Resistance",
            "Animal Companion ~ Stat Bonus",
            "Animal Training ~ Combat Training",
            "Animal Training ~ Fighting",
            "Animal Training ~ Guarding",
            "Animal Training ~ Heavy Labor",
            "Animal Training ~ Hunting",
            "Animal Training ~ Performance",
            "Animal Training ~ Riding",
            "Animal Trick ~ Air Walk",
            "Animal Trick ~ Attack",
            "Animal Trick ~ Attack II",
            "Animal Trick ~ Come",
            "Animal Trick ~ Defend",
            "Animal Trick ~ Down",
            "Animal Trick ~ Fetch",
            "Animal Trick ~ Guard",
            "Animal Trick ~ Heel",
            "Animal Trick ~ Perform",
            "Animal Trick ~ Seek",
            "Animal Trick ~ Stay",
            "Animal Trick ~ Track",
            "Animal Trick ~ Work",
            "Base Companion ~ Animal Companion",
            "Base Companion ~ Special Mount",
            "Companion Advancement",
            "Companion Skills",
            "Companion Stat ~ CHA",
            "Companion Stat ~ CON",
            "Companion Stat ~ DEX",
            "Companion Stat ~ INT",
            "Companion Stat ~ STR",
            "Companion Stat ~ WIS",
            "Companion ~ Ability Score Increase",
            "Companion ~ Bonus Tricks",
            "Companion ~ Devotion",
            "Companion ~ Evasion",
            "Companion ~ Improved Evasion",
            "Companion ~ Link",
            "Companion ~ Multiattack",
            "Companion ~ Share Spells",
            "Companion ~ Spell Resistance (AC)",
            "Companion ~ Spell Resistance (SM)",
        },
    }
    book_wide_applied = 0
    # `sorted(...)`, not raw `set` iteration: CPython randomizes `str` hash
    # seeds per process, so an un-sorted set walk here made every `ability_keys`
    # list this shape touches (and every `creature_ability_keys[creature]` list
    # downstream) reorder run to run with no corpus reason -- found this cycle
    # by diffing two back-to-back regenerations of the SAME unmodified book and
    # getting a non-empty diff (`git diff` after a second run showed 76-element
    # `ability_keys` lists reshuffled, same elements). `owners[key]` (assembled
    # via `sorted(creature_keys)` two lines below) was already immune; this key
    # loop was the one unsorted set walk left in the whole pass.
    for key in sorted(BOOK_WIDE_GRANTS.get(book, set())):
        if key not in owners or owners[key]:
            continue
        for creature in sorted(creature_keys):
            owners[key].append(creature)
            creature_ability_keys[creature].append(key)
        book_wide_applied += 1
    if book_wide_applied:
        print(
            f"{book}: {book_wide_applied} ability row(s) attributed to ALL "
            f"{len(creature_keys)} registered creatures (Shape 7, book-wide grant)",
            file=sys.stderr,
        )

    # Shape 8, CROSS-BOOK ownership (`AT-34-E3-001`, `decisions.md §67`). Every
    # shape above (1-7) attributes an ability to a creature registered under
    # THIS SAME book. Core Rulebook's `ce_abilities_familiar_cr.lst` pool is a
    # real exception the source material itself creates: the ability rules
    # (Magic chapter) are stated in Core Rulebook while the 11 familiar
    # creature stat blocks are stated in Bestiary (`ce_races_familiar_cr.lst`
    # declares `SOURCELONG:Bestiary`) -- two real halves of one PF1 mechanic,
    # split across two real books, neither row misattributed.
    #
    # `CROSS_BOOK_GRANTS` is an exact, closed key set exactly like
    # `BOOK_WIDE_GRANTS` above -- never a prefix or shape heuristic -- and each
    # entry additionally names its owner book and the EXACT closed set of
    # owner creature keys, so a future unrelated orphan can never silently
    # ride this shape. Applied only to keys still unowned after shapes 1-7 (an
    # ability already owned some other way is never re-attributed here).
    CROSS_BOOK_GRANTS: dict[str, dict[str, tuple[str, tuple[str, ...]]]] = {
        "core_rulebook": {
            key: (
                "beastiary",
                (
                    "Bat", "Cat", "Hawk", "Lizard", "Monkey", "Owl", "Rat", "Raven",
                    "Toad", "Viper", "Weasel",
                ),
            )
            for key in (
                "Familiar Alertness Choice ~ Alertness Active",
                "Familiar Alertness Choice ~ Alertness Inactive",
                "Familiar ~ Alertness",
                "Familiar ~ Deliver Touch Spells",
                "Familiar ~ Empathic Link",
                "Familiar ~ Improved Evasion",
                "Familiar ~ Intelligence Score",
                "Familiar ~ Natural Armor Bonus",
                "Familiar ~ Scry on Familiar",
                "Familiar ~ Share Spells",
                "Familiar ~ Speak One Language",
                "Familiar ~ Speak with Animals of Its Kind",
                "Familiar ~ Speak with Master",
                "Familiar ~ Spell Resistance",
            )
        },
    }
    # Keyed the same as `owners` -- every ability key, most mapping to `[]`.
    cross_book_owners: dict[str, list[tuple[str, str]]] = {u["corpus_key"]: [] for u in abilities}
    cross_book_applied = 0
    for key, (owner_book, owner_creatures) in CROSS_BOOK_GRANTS.get(book, {}).items():
        if key not in owners or owners[key]:
            continue
        # Verify each named creature is a REAL, currently-registered creature
        # of the owner book -- read directly from that book's own ingested
        # corpus, never assumed from this closed list alone. A grant naming a
        # creature the owner book does not register is a defect in this
        # table, not a fact about the corpus, and must fail closed.
        owner_dir = os.path.join("data", "corpus", owner_book, "companion")
        registered: set[str] = set()
        if os.path.isdir(owner_dir):
            for fname in os.listdir(owner_dir):
                if not fname.endswith(".json"):
                    continue
                with open(os.path.join(owner_dir, fname), encoding="utf-8") as fh:
                    doc = json.load(fh)
                corpus_key = doc.get("data", {}).get("corpus_key")
                if corpus_key:
                    registered.add(corpus_key)
        missing = [c for c in owner_creatures if c not in registered]
        if missing:
            raise SystemExit(
                f"{book}: Shape 8 cross-book grant for {key!r} names {missing} which "
                f"{owner_book} does not register at {owner_dir} -- widen or fix the grant"
            )
        for creature in owner_creatures:
            cross_book_owners[key].append((owner_book, creature))
        cross_book_applied += 1
    if cross_book_applied:
        print(
            f"{book}: {cross_book_applied} ability row(s) attributed via Shape 8 "
            f"cross-book ownership to {len(next(iter(CROSS_BOOK_GRANTS.get(book, {}).values()))[1])} "
            "creature(s) of another registered book",
            file=sys.stderr,
        )

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
    # their honest `engine-does-not-hold` status in the work inventory — this function
    # never touches that — so the shortfall stays a stated claim a reader can
    # check rather than a silent omission.

    # ---- Product Identity screen, ABILITY half ----
    #
    # `pi_screening::{declared_product_identity, classify_optional_field_declared}`'s
    # two rulings, hand-applied here because this transcriber emits a Rust
    # literal table rather than a JSON record with `license`/`pi_field`/
    # `pi_marker` to route through the shared reader (`decisions.md §39.4`,
    # `SD30-E3-F3-001`): `NAMEISPI:YES` DROPS the row -- the identity field,
    # referenced by every `<Creature> ~ <Ability>` key that owns it, so
    # redaction would break the reference, exactly `transcribe_monster_
    # tables`'s own rule. `DESCISPI:YES` REDACTS the description text (its
    # `%N` variables, and every gated variant -- `description_variants` are
    # alternate renderings of the SAME declared-PI prose, not independent
    # text) to `shape_b_v1::REDACTED_PI_MARKER` and ships the row anyway,
    # mirroring `ingest_race_traits.rs`/`ingest_pu_classes.rs`'s
    # name-drop/description-redact split. A row carrying BOTH tokens is
    # dropped, not redacted -- the name-drop always wins, because a dropped
    # row has no description left to redact.
    #
    # This transcriber (unlike `transcribe_monster_tables`) does not also run
    # a term-blacklist scan over emitted values -- `scripts/verify.sh`'s
    # `pi-sweep` stage (`pi_sweep_rules_tables`) already screens EVERY
    # generated file under `src/rules_core/rules_tables/`, this book's
    # `companion_data.rs` included, against `pi_screening::PI_BLACKLIST_TERMS`
    # downstream of this script; adding a second copy of that scan here would
    # duplicate a check that already runs, not add coverage. Declared-PI
    # reading is a DIFFERENT question from term-blacklist scanning
    # (`decisions.md §39.4`'s "union, never a merge"), and this screen answers
    # only that question.
    #
    # This book's own re-derived source exposure at time of writing is zero
    # for every book this transcriber's `book_dirs()` currently registers
    # (`decisions.md §39.2` corpus-wide sweep over the 17-book companion
    # scope, re-run this cycle); `decisions.md §39`'s own 1-row
    # `dtt_races_companion.lst` (`dirty_tactics_toolbox`) finding is real but
    # is a book this transcriber does not yet register at all -- out of
    # `docs/work-inventory.json`'s `corpus_root`/`additional_book_dirs`, so
    # out of THIS script's current scope, owned by book onboarding when that
    # book is added (`SD-31-corpus-closure-grind`).
    pi_dropped_abilities: list[str] = []
    desc_redacted: set[str] = set()
    for unit in abilities:
        key = unit["corpus_key"]
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        if token(row, "NAMEISPI:") == "YES":
            pi_dropped_abilities.append(key)
        elif token(row, "DESCISPI:") == "YES":
            desc_redacted.add(key)
    if pi_dropped_abilities:
        dropped = set(pi_dropped_abilities)
        abilities = [u for u in abilities if u["corpus_key"] not in dropped]
        for key in dropped:
            owners.pop(key, None)
            cross_book_owners.pop(key, None)
        for creature_key, keys in creature_ability_keys.items():
            creature_ability_keys[creature_key] = [k for k in keys if k not in dropped]
        print(
            f"{book}: {len(pi_dropped_abilities)} ability row(s) NOT transcribed "
            "(NAMEISPI:YES -- a declared name cannot be redacted): "
            + ", ".join(sorted(pi_dropped_abilities)),
            file=sys.stderr,
        )
    # `desc_redacted`'s stderr line and module-doc entry are emitted below,
    # AFTER the `.COPY=`/`.MOD` and empty-payload screens and the orphan pass
    # have all run -- any of them can still remove a row this set was
    # computed before (an orphaned or delta DESCISPI:YES row has no
    # description left to redact either, because it has no row left).

    # ---- delta-row screen (`.COPY=` / `.MOD`), before the orphan pass ----
    #
    # Adopted verbatim from `transcribe_monster_tables`'s `.COPY=` screen, for
    # the reason stated there: a `<Base>.COPY=<Variant>` row does not state a
    # record, it states a DELTA on one.  PCGen copies the base record whole and
    # then applies the few tokens the copy row carries, so transcribing the row
    # verbatim -- which is all this script does -- yields a record with an
    # `ASPECT` and nothing else: no `TYPE:`, no `DESC:`, no page. That is a card
    # a player opens to find blank, the stub class
    # `docs/governance/no-stub-mvp-doctrine.md` forbids. `gen_book_cache`'s
    # `verified_citation_line` refuses it outright anyway, because the row's
    # first column reads `CATEGORY=Special Ability|Change Shape.COPY=Pooka ~
    # Change Shape` rather than the record's name -- which is exactly how this
    # book's two were found (round 5, `decisions.md §59.2`).
    #
    # Resolving the delta is not a transcription: it composes values across two
    # rows while `CompanionAbilityRecord` carries ONE `source_file`/`source_line`
    # pair, so every inherited field would ship under a citation that does not
    # contain it. A chassis modelling inheritance needs a second citation, and
    # that is a deliberate widening, not something to slip into an ingest round.
    #
    # `mod_only` is the same class of row and screened by the same pass -- a
    # `.MOD` overlay updates a record declared elsewhere. **No book registered
    # through round 5 carries one**, so that half is stated, not exercised
    # (`decisions.md §56.3`'s discipline); `core_essentials` (4) and
    # `ultimate_wilderness` (1) are where it will first bite.
    #
    # The inventory already states which rows these are -- `origin` is its own
    # field, so this reads a classification rather than re-deriving one:
    #   python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json'));
    #   print(collections.Counter(u['origin'] for u in d['units'] if u['kind']=='companion'))"
    #   -> Counter({'declared': 1666, 'copy': 25, 'mod_only': 5})
    deltas = sorted(
        u["corpus_key"] for u in abilities if u.get("origin") in ("copy", "mod_only")
    )
    delta_kinds = {
        u["corpus_key"]: u["origin"]
        for u in abilities
        if u.get("origin") in ("copy", "mod_only")
    }
    if deltas:
        delta_keys = set(deltas)
        abilities = [u for u in abilities if u["corpus_key"] not in delta_keys]
        for key in delta_keys:
            owners.pop(key, None)
            cross_book_owners.pop(key, None)
        # A creature must never name a record this table does not define --
        # `companion_chassis`'s `the_chassis_link_resolves_in_both_directions_
        # for_every_book` asserts exactly that, in both directions.
        for creature_key, keys in creature_ability_keys.items():
            creature_ability_keys[creature_key] = [k for k in keys if k not in delta_keys]
        print(
            f"{book}: {len(deltas)} delta row(s) NOT transcribed (a `.COPY=`/`.MOD` row "
            "states a delta on another record, not a record): " + ", ".join(deltas),
            file=sys.stderr,
        )

    orphans = sorted(k for k, v in owners.items() if not v and not cross_book_owners.get(k))
    orphan_keys = set(orphans)
    if orphans:
        abilities = [u for u in abilities if u["corpus_key"] not in orphan_keys]

    # ---- empty-payload screen (`decisions.md §63.3`) ----
    #
    # A row can be perfectly OWNED and still state nothing this chassis is able
    # to hold.  `core_essentials`' `Pseudodragon ~ Tail`
    # (`ce_abilities_familiar_race_cr.lst:215`) is the first, and across all
    # twelve registered books it is the ONLY one: it carries `KEY:`,
    # `CATEGORY:Special Ability`, `SOURCEPAGE:p.229` and
    # `ASPECT:ReachAttack|5 ft.` -- no `TYPE:`, no `DESC:`, no `BONUS:`.
    # Transcribed, every modelled field comes out empty and the card a player
    # opens reads "Tail" over a page number.
    #
    # `ASPECT:` is the one token that says what the row DOES, and no chassis in
    # this program models it -- not this one and not `monster_chassis`
    # (`grep -rn aspect src/rules_core/rules_tables/monster_chassis.rs
    # scripts/transcribe_monster_tables.py` -> nothing).  Modelling it is a real
    # widening and worth doing: 27 of the 394 grounded ability rows across the
    # twelve registered books carry an `ASPECT:` that is being dropped today.
    # The other 26 also carry a `TYPE:`, so they are diminished by the omission
    # rather than emptied by it, which is why this round states the measurement
    # and takes the narrow disposition instead of widening the record type on
    # the way past.
    #
    # The disposition is `§61.2`'s, already settled one round earlier for
    # Ultimate Wilderness's archetype rows: a row this chassis is the wrong
    # SHAPE for is dropped, named here and in the module doc, and left honestly
    # `engine-does-not-hold` in `docs/work-inventory.json` -- never shipped as a card
    # with nothing on it.
    #
    # The predicate is `reach_gate::companions_reach`'s own ability payload rule
    # with `source_page` REMOVED, and that difference is the finding rather than
    # an inconsistency: a page citation tells a player where to read the rule,
    # not what it is.  The gate counts it, so this row would have passed the
    # reach gate while showing nothing -- a gate agreeing with a stub is exactly
    # the twin problem `AGENTS.md` names, and screening at the generator is the
    # fix at the source rather than at the instrument.
    empty = sorted(
        u["corpus_key"]
        for u in abilities
        if not row_states_modelled_content(
            read_row(resolve_source_file(directory, u["source_file"]), u["source_line"])
        )
    )
    if empty:
        empty_keys = set(empty)
        abilities = [u for u in abilities if u["corpus_key"] not in empty_keys]
        for key in empty_keys:
            owners.pop(key, None)
            cross_book_owners.pop(key, None)
        # Same both-directions obligation the delta screen carries: a creature
        # must never name a record this table does not define.
        for creature_key, keys in creature_ability_keys.items():
            creature_ability_keys[creature_key] = [k for k in keys if k not in empty_keys]
        print(
            f"{book}: {len(empty)} owned ability row(s) NOT transcribed (the row states "
            "nothing this chassis models -- no TYPE:, no DESC:, no BONUS:): " + ", ".join(empty),
            file=sys.stderr,
        )

    # Finalized against whatever `abilities` actually ships after every
    # screen above -- an ability no longer shipping (orphan, `.COPY=`/`.MOD`
    # delta, or empty-payload) has no description left to redact either.
    desc_redacted &= {u["corpus_key"] for u in abilities}
    if desc_redacted:
        print(
            f"{book}: {len(desc_redacted)} ability row(s) description redacted "
            f"(DESCISPI:YES): " + ", ".join(sorted(desc_redacted)),
            file=sys.stderr,
        )

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
    for shape_name, rows in (("creature", creatures), ("ability", abilities), ("class", class_units)):
        for source_file in sorted({u["source_file"] for u in rows}):
            n = sum(1 for u in rows if u["source_file"] == source_file)
            out.append(f"//!   * `{source_file}` -- {n} companion {shape_name} rows")
    if pi_dropped_creatures or pi_dropped_abilities:
        out.append("//!")
        out.append(
            f"//! {len(pi_dropped_creatures)} creature row(s) and {len(pi_dropped_abilities)}"
        )
        out.append(
            "//! ability row(s) of this book DECLARE `NAMEISPI:YES` (PCGen's own per-record"
        )
        out.append(
            "//! Product Identity marker) and are NOT transcribed -- a name cannot be"
        )
        out.append(
            "//! redacted without breaking the record's own identity and every ability key"
        )
        out.append(
            "//! that references it. Reclassifying is `docs/governance/ogl-pi-blacklist.md`"
        )
        out.append("//! §3's per-book override, an operator decision, not a transcriber's:")
        # `units` (built at the top of this function, never reassigned after
        # the `gated` filter) still carries every dropped row's own unit
        # dict -- `creatures`/`abilities` have already had these keys removed
        # by this point, so they are looked up here rather than there.
        pi_dropped_units = {u["corpus_key"]: u for u in units}
        for key in sorted(pi_dropped_creatures) + sorted(pi_dropped_abilities):
            unit = pi_dropped_units[key]
            out.append(
                f"//!   * `{unit['source_file']}:{unit['source_line']}` "
                f"({'creature' if key in pi_dropped_creatures else 'ability'} row, `{key}`)"
            )
    if desc_redacted:
        out.append("//!")
        out.append(
            f"//! {len(desc_redacted)} ability row(s) of this book DECLARE `DESCISPI:YES` --"
        )
        out.append(
            "//! their `description` (its `%N` variables, and every gated"
        )
        out.append(
            "//! `description_variants` entry) SHIP REDACTED to `shape_b_v1::"
        )
        out.append(
            "//! REDACTED_PI_MARKER` rather than dropped, because a description (unlike a"
        )
        out.append(
            "//! name) can be redacted and the record still works. Reclassifying is"
        )
        out.append(
            "//! `docs/governance/ogl-pi-blacklist.md` §3's per-book override, an operator"
        )
        out.append("//! decision, not a transcriber's:")
        desc_redacted_units = {u["corpus_key"]: u for u in abilities}
        for key in sorted(desc_redacted):
            unit = desc_redacted_units[key]
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}` ({key})")
    if orphans:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- ability rows no creature row of this book owns, so"
        )
        out.append(
            "//! nothing could ever reach them on screen. Dropped rather than emitted"
        )
        out.append(
            "//! unreachable (`decisions.md §50`, adopted from the monster lane; §56.1)."
        )
        # NOT "carried as an OPEN_FINDINGS entry", which is what this block said
        # from round 4 until round 6 -- while every registered book had ZERO
        # orphans, so the sentence was never checkable. It is false by
        # construction: `reach_gate::OPEN_FINDINGS` is keyed by (book, FAMILY)
        # and its consistency test fails an entry naming a family that DOES
        # reach a player, which every registered book's `companions` family
        # does. A dropped row is also not an ingested record, so it is outside
        # the reach gate's denominator entirely. The honest record of it is
        # this list plus the book's `mod.rs` (`decisions.md §61.2`).
        out.append(
            "//! These rows keep their `engine-does-not-hold` status in"
        )
        out.append(
            "//! `docs/work-inventory.json`, which is where the shortfall is counted; they"
        )
        out.append(
            "//! are NOT a `reach_gate` `OPEN_FINDINGS` entry, because that list is keyed by"
        )
        out.append(
            "//! FAMILY and this book's `companions` family does reach a player:"
        )
        for key in orphans:
            out.append(f"//!   * `{key}`")
    if deltas:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- rows that state a DELTA on another record rather than"
        )
        out.append(
            "//! a record of their own. Transcribing one verbatim ships a card with almost"
        )
        out.append(
            "//! every field empty; resolving it needs a second citation this chassis does"
        )
        out.append(
            "//! not carry (`decisions.md §59.2`, adopting the monster lane's `.COPY=`"
        )
        out.append("//! screen). Their owners no longer name them:")
        for key in deltas:
            out.append(f"//!   * `{key}` (`origin: {delta_kinds[key]}`)")
    if creature_deltas:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- CREATURE rows that state a DELTA on another record"
        )
        out.append(
            "//! (`decisions.md §63.1`). PCGen copies the base creature whole and applies"
        )
        out.append(
            "//! the few tokens the copy row carries, so the row itself states no `SIZE:`,"
        )
        out.append(
            "//! no `MOVE:` and no `MONSTERCLASS:` -- transcribed verbatim it is a creature"
        )
        out.append(
            "//! card with a name and nothing else. Any ability reachable only from one of"
        )
        out.append("//! these is carried as an orphan above, not shipped unowned:")
        for key in creature_deltas:
            out.append(f"//!   * `{key}` (`origin: {creature_delta_kinds[key]}`)")
    if empty:
        out.append("//!")
        out.append(
            "//! NOT transcribed -- OWNED rows that state nothing this chassis models"
        )
        out.append(
            "//! (`decisions.md §63.3`). The row carries no `TYPE:`, no `DESC:` and no"
        )
        out.append(
            "//! `BONUS:`, so every modelled field transcribes empty and the card reads as"
        )
        out.append(
            "//! a name over a page number. What each one DOES state is an `ASPECT:`, which"
        )
        out.append(
            "//! no chassis in this program models yet -- the disposition is `§61.2`'s:"
        )
        out.append("//! dropped, named here, left honestly `engine-does-not-hold`:")
        for key in empty:
            out.append(f"//!   * `{key}`")
    if classes:
        out.append("//!")
        out.append(
            "//! `*_classes_companion.lst` CLASS rows, transcribed as `CompanionClassRecord`"
        )
        out.append(
            "//! (`AT-34-E3-001`, `decisions.md §17`) rather than dropped. A PCGen monster"
        )
        out.append(
            "//! class is the hit-dice progression a creature row's `MONSTERCLASS:` token"
        )
        out.append(
            "//! names -- it states no `SIZE:`, no `MOVE:` and no natural attacks, so it is"
        )
        out.append(
            "//! neither a creature nor an ability; every field is carried verbatim and"
        )
        out.append("//! nothing is computed from it:")
        for key in classes:
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
        damage_bonuses = parse_natural_attack_damage_bonuses(row)
        skill_bonuses = parse_skill_ability_diff_bonuses(row)
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
            "        natural_attack_damage_bonuses: &["
            + ", ".join(
                f"NaturalAttackDamageBonus {{ attack: {rust_str(a)}, formula: {rust_str(f)} }}"
                for a, f in damage_bonuses
            )
            + "],"
        )
        out.append(
            "        skill_ability_diff_bonuses: &["
            + ", ".join(
                f"SkillAbilityDiffBonus {{ skills: {rust_slice(skills)}, "
                f"formula: {rust_str(formula)} }}"
                for skills, formula in skill_bonuses
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
        description, variables, variants = parse_desc(row)
        if unit["corpus_key"] in desc_redacted:
            # `DESCISPI:YES` -- the redaction promised by the module doc's
            # own listing above. `variables` names `%N` placeholders from the
            # ORIGINAL text, which no longer ships, so it is cleared too
            # rather than left dangling against a marker with no `%N` for
            # them to refer to; `variants` are alternate renderings of the
            # SAME declared-PI prose, so they are dropped rather than each
            # individually redacted -- one marker says everything three would.
            description = redacted_pi_marker()
            variables = []
            variants = []
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
            "        description_variants: &["
            + ", ".join(
                "CompanionDescriptionVariant { "
                f"text: {rust_str(text)}, "
                f"variables: {rust_slice(vs)}, "
                f"conditions: {rust_slice(cs)} }}"
                for text, vs, cs in variants
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
        out.append(f"        source_page: {rust_opt(token(row, 'SOURCEPAGE:'))},")
        out.append(f"        owners: {rust_slice(owners[unit['corpus_key']])},")
        out.append(
            "        cross_book_owners: "
            f"{rust_pair_slice(cross_book_owners.get(unit['corpus_key'], []))},"
        )
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")
    out.append(f"/// Every {book} `*_classes_companion.lst` row ({len(class_units)} rows).")
    out.append("pub(super) static COMPANION_CLASSES: &[CompanionClassRecord] = &[")
    for unit in class_units:
        row = read_row(resolve_source_file(directory, unit["source_file"]), unit["source_line"])
        fields = parse_class_row(row)
        out.append("    CompanionClassRecord {")
        out.append(f"        key: {rust_str(unit['corpus_key'])},")
        out.append(f"        output_name: {rust_opt(fields['output_name'])},")
        hd = fields["hit_dice"]
        out.append(f"        hit_dice: {'Some(' + str(hd) + ')' if hd is not None else 'None'},")
        out.append(f"        max_level: {rust_opt(fields['max_level'])},")
        out.append(f"        type_segments: {rust_slice(fields['type_segments'])},")
        out.append(f"        visible_no: {'true' if fields['visible_no'] else 'false'},")
        out.append(f"        source_page: {rust_opt(fields['source_page'])},")
        out.append(f"        ability_grants: {rust_slice(fields['ability_grants'])},")
        out.append(f"        fact_class_type: {rust_opt(fields['fact_class_type'])},")
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
#
# SD-29 Epic 7 round 8 added the second and third entries, and they are the same
# hazard as the first rather than a new one. `core_rulebook` and
# `advanced_players_guide` are the corpus-directory ids (`data/corpus/` spells
# both out in full), but their engine modules have carried the abbreviations
# `crb` and `apg` since long before this lane — `rules_tables/mod.rs` line 3
# names them that way. Without these rows the transcriber writes
# `rules_tables/core_rulebook/companion_data.rs`: a SECOND module for a book that
# already has one, which compiles, passes its own tests, and is reachable from
# nothing. That is the exact failure the paragraph above describes; it was found
# by reading this comment before running the tool, not by the gate, because an
# unreferenced module is invisible to it.
MODULE_DIR = {
    "bestiary": "beastiary1",
    "core_rulebook": "crb",
    "advanced_players_guide": "apg",
}


def module_dir(book: str) -> str:
    return MODULE_DIR.get(book, book)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <book>")
    book = sys.argv[1]
    # Transcribe BEFORE opening the output, never inside the `with`. Opening
    # for write creates the file, so a run that then refuses -- an unknown book
    # id, a class row, an unresolvable multi-`DESC:` -- used to leave an EMPTY
    # generated module behind, in a directory it had just created. Round 6 did
    # exactly that with a mistyped book id and left an empty
    # `rules_tables/beastiary/companion_data.rs` on the tree; nothing in the
    # gate would have caught it, because an unreferenced module compiles fine.
    contents = transcribe(book)
    directory = f"src/rules_core/rules_tables/{module_dir(book)}"
    os.makedirs(directory, exist_ok=True)
    path = os.path.join(directory, "companion_data.rs")
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(contents)
    print(f"wrote {path}")


if __name__ == "__main__":
    main()
