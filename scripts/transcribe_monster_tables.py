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
    # The first two campaign-setting books in this lane, and the only two
    # remaining books with ZERO orphan abilities -- every one of their ability
    # rows is named by a monster row in the same book. Derived, not assumed:
    # `python3 scripts/classify_monster_ability_rows.py book_of_the_damned_volume_1
    # book_of_the_damned_volume_2`.
    "book_of_the_damned_volume_1": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    # SD-29 Epic 5 extend, round 3, and the book that made this script grow two
    # screens. It is the first in the lane that is NOT orphan-free; the first
    # whose monsters live in TWO races files with COLLIDING line numbers
    # (`iswg_races.lst` 7 rows, `iswg_races_bestiary.lst` 7); and the first
    # whose corpus rows carry `NAMEISPI:YES`, PCGen's own per-record Product
    # Identity declaration. Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py inner_sea_world_guide`
    # and `grep -c NAMEISPI:YES iswg_races.lst iswg_races_bestiary.lst`.
    "inner_sea_world_guide": "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    # SD-29 Epic 5 extend, round 4, and the largest book in the lane by an order
    # of magnitude: 316 monster rows + 466 ability rows, of which 402 are owned
    # by a monster row of this book. The first `roleplaying_game/` bestiary the
    # lane has taken since the Bonus Bestiary pilot, and the first book with
    # ZERO Product Identity rows in either signal -- derived, never assumed:
    # `grep -c NAMEISPI:YES b2_races.lst b2_abilities_race.lst` -> 0, 0, and
    # `python3 scripts/classify_monster_ability_rows.py bestiary_2` -> PI 0.
    # `ogl-pi-blacklist.md` §2 predicts it: classic SRD monster names are
    # presumptively Open Game Content.
    "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2",
    # SD-29 Epic 5 extend, round 5, and the CLEANEST book the lane has taken --
    # 261 monster rows + 40 ability rows with only 13 orphans, no Product
    # Identity row and no `.COPY=` delta at all. Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py bestiary_3` ->
    # `bestiary_3  261  40  0  27  13  0  0`, and
    # `grep -c NAMEISPI:YES b3_races.lst b3_abilities_race.lst` -> 0, 0, which
    # is what `ogl-pi-blacklist.md` §2 predicts for a `roleplaying_game/`
    # bestiary. It is also the first book in the lane whose ability rows are
    # reached ENTIRELY by the namespaced-prefix shape (27 prefix, 0 row-named).
    #
    # `row-named` is 0 even though `b3_races.lst` carries 100
    # `ABILITY:Special Ability|AUTOMATIC|` tokens
    # (`grep -c 'ABILITY:Special Ability|AUTOMATIC|' b3_races.lst` -> 100).
    # Those tokens name rows this book files under a DIFFERENT kind, so they
    # are not in this book's `monster_ability` key set and the row-named pass
    # correctly finds none of them. See `rules_tables::bestiary_3` for the
    # derivation and the 341-unit scope finding it carries -- the tokens are
    # real, the abilities are real, and `v06_work_inventory::file_kind` reads
    # only the FIRST `TYPE:` segment, so `TYPE:AghashRacialAbility.
    # SpecialQuality.Supernatural` lands in `race_trait` while
    # `TYPE:SpecialQuality.Extraordinary.AdaroRacial` lands here.
    "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3",
    # SD-29 Epic 5 extend, round 6, and the largest REACHABLE book left in the
    # lane: 220 monster rows + 768 ability rows, 749 of which are reachable.
    # Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py bestiary_4` ->
    # `bestiary_4  220  768  0  543  225  14  0`.
    #
    # It is the first `roleplaying_game/` bestiary in this lane that carries
    # Product Identity rows at all, and that REFINES rather than contradicts
    # `ogl-pi-blacklist.md` §2. Rounds 4 and 5 each recorded the prediction as
    # a BOOK-location predicate ("a `roleplaying_game/` bestiary carries zero
    # `NAMEISPI:YES`"), and both were right about their own book by luck of its
    # contents. The blacklist's actual predicate is PER-RECORD (§2.1): a
    # generic SRD species name is presumptively Open Game Content, a unique
    # named persona is not. `grep -c NAMEISPI:YES b4_races.lst
    # b4_abilities_race.lst` -> 14, 0, and all 14 are unique named personas --
    # three Demon Lords (Dagon, Kostchtchie, Pazuzu), three Empyreal Lords
    # (Cernunnos, Korada, Vildeis), three Great Old Ones (Bokrug, Cthulhu,
    # Hastur), three Kaiju (Agyra, Bezravnis, Mogaru), Spawn of Yog-Sothoth
    # and Star-Spawn of Cthulhu -- not one generic species among them. The
    # book-location form of the rule would have shipped all 14.
    "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4",
    # SD-29 Epic 5 extend, round 7, and the first `campaign_setting/` bestiary
    # in the lane. Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py inner_sea_bestiary` ->
    # `inner_sea_bestiary  40  190  157  0  26  7  0`, i.e. 197 reachable.
    #
    # Its shape is the ROW-NAMED one (157 of 190 abilities are named by an
    # `ABILITY:Special Ability|AUTOMATIC|` token on a monster row; 0 reach
    # through the namespaced prefix), the same shape as the Bonus Bestiary
    # pilot and the opposite of `bestiary_3`'s. It carries 7 `NAMEISPI:YES`
    # rows -- and that is what `ogl-pi-blacklist.md` §2's PER-RECORD predicate
    # (`decisions.md §57.1`) predicts for a `campaign_setting/` book, whose
    # creatures are Golarion-specific personae rather than generic SRD species.
    "inner_sea_bestiary": "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    # SD-29 Epic 5 extend, round 8. Bestiary 1 -- the book `decisions.md §58.3`
    # ruled on and deliberately did not execute in the round that ruled. It is
    # the first book in this lane whose monster rows are ALREADY served, in
    # part, by a DIFFERENT compiled table: SD-22's `rules_tables::beastiary1`
    # holds 46 of the book's 330 rows. The ruling is that this chassis sits
    # ALONGSIDE that table and takes the book's COMPLEMENT -- see
    # `CROSS_TABLE_MONSTER_RECORDS` below for the mechanism and
    # `rules_tables::bestiary` for the derivation.
    #
    # Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py bestiary` ->
    # `bestiary  284  523  375  2  146  0  0`, i.e. 661 classifier-reachable
    # over a monster set that includes all 330 corpus rows while counting only
    # the 284 remaining ones. 607 ship; the 54-unit residue is the cross-table
    # class this screen names.
    "bestiary": "pathfinder/paizo/roleplaying_game/bestiary",
}

# Books part of whose monster rows another compiled table of THIS repo already
# serves, keyed by the `data/corpus/` directory that table's records live in.
#
# Bestiary 1 is the only such book and `decisions.md §58.3` is the ruling that
# makes it one: SD-22's `rules_tables::beastiary1` already serves 46 of the
# book's 330 monster rows, under the `beastiary1:monster:<slug>` key space, out
# of the same `data/corpus/beastiary/monster/` directory this chassis writes to.
# Absorbing them would mean emitting 46 records the catalog already serves under
# the same wire code -- a duplicate a player can see -- so this chassis takes the
# complement instead.
#
# The exclusion is derived from the OTHER table's own shipped records rather
# than from a hand-written name list or from the work inventory's `status`
# field, for two reasons. It is the same denominator `reach_gate::monsters_reach`
# reads, so the two can never disagree; and it is stable under this generator's
# own output, because the two record shapes are distinguishable -- SD-22's
# records carry their identity as `data.id` (they predate the `key` convention),
# every chassis record carries `data.key`. Re-running the transcriber after
# `gen_book_cache` has written the chassis half therefore excludes the same 46
# rows, not all 330.
CROSS_TABLE_MONSTER_RECORDS = {"bestiary": "beastiary"}

# The `TYPE:` first segment that names which facet of `monster_ability` a row
# is. Spelled exactly as the corpus spells it.
FACETS = {"SpecialAttack": "SpecialAttack", "SpecialQuality": "SpecialQuality"}
# The `TYPE:` segment naming how the ability is delivered, when the row says.
DELIVERIES = {
    "Supernatural": "Supernatural",
    "Extraordinary": "Extraordinary",
    "SpellLike": "SpellLike",
}


PI_SCREEN_RS = "src/rules_core/pi_screening.rs"


def pi_blacklist_terms() -> list[str]:
    """`pi_screening::PI_BLACKLIST_TERMS`, parsed out of the Rust source.

    **Derived, never re-typed.** `gen_book_cache`'s monster generator screens
    every serialized record against that list and treats a hit as a HARD STOP
    -- it writes nothing further and exits. A transcription that did not know
    the list would therefore produce a table that cannot be generated from, and
    a copy of the list here would drift the first time a per-book override adds
    a term (`docs/governance/ogl-pi-blacklist.md` §3 exists to add them).

    Reading the Rust is what keeps one list authoritative.
    """
    text = open(PI_SCREEN_RS, encoding="utf-8").read()
    start = text.index("pub const PI_BLACKLIST_TERMS")
    body = text[start : text.index("\n];", start)]
    terms = re.findall(r'"([^"]+)"', body)
    if len(terms) < 20:
        raise SystemExit(
            f"{PI_SCREEN_RS}: parsed only {len(terms)} PI terms, which cannot be right -- "
            "the const's shape changed and this parser must be updated with it"
        )
    return terms


def pi_hits(terms: list[str], *values: str | None) -> list[str]:
    """Every blacklist term appearing in any of these emitted values."""
    joined = " ".join(v for v in values if v)
    return [t for t in terms if t in joined]


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


# U+00AD SOFT HYPHEN, a PDF-extraction artifact that reaches the corpus as an
# INVISIBLE character inside a word. Bestiary 4 is the first book in this lane
# to carry any -- 5 occurrences in `b4_abilities_race.lst`, 0 in Bestiary 2's
# and Bestiary 3's equivalents -- and `clippy::invisible_characters` is
# deny-by-default, so transcribing them verbatim does not merely look wrong, it
# fails the build.
#
# Every occurrence stands where a REAL hyphen belongs, mangled by the
# line-breaking in the source PDF:
#
#     pod<U+00AD>spawned   10-foot<U+00AD>radius
#     free<U+00AD>willed   cone<U+00AD>shaped
#
# Replaced with `-` rather than DELETED: deleting yields "10-footradius" and
# "coneshaped", which are wrong. The book corroborates the hyphen itself --
# its own ability row is keyed `Pod-Spawned ~ Loss of Magic`, spelled with a
# plain hyphen, for the same creature whose DESC: text carries the soft one.
#
# This is a character-encoding normalisation of a known extraction artifact,
# not a rewrite of rules text: no word, number or token changes. It is applied
# in `read_row` so that it lands before EVERY downstream reader -- the Product
# Identity screen included -- rather than only on the `DESC:` path.
SOFT_HYPHEN = "­"


def read_row(path: str, line_no: int) -> list[str]:
    """The 1-based line at `line_no`, split into its tab-separated tokens."""
    with open(path, encoding="utf-8", errors="replace") as handle:
        line = handle.read().split("\n")[line_no - 1]
    line = line.replace(SOFT_HYPHEN, "-")
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


def is_prerequisite(entry: str) -> bool:
    """Whether a `DESC:` trailing entry is a prerequisite rather than a variable.

    PCGen writes prerequisites in both polarities and the negated spelling is
    the one this parser originally missed: ``PRERULE:1,DisplayFullAbility`` was
    filtered, ``!PRERULE:1,DisplayFullAbility`` was not, so it landed in
    ``description_variables`` as though it were a formula variable the row's
    ``%N`` refer to. Book of the Damned Volume 2 is the first ingested book to
    carry the shape (11 of its 17 ability rows); Bonus Bestiary and Monster
    Codex carry none, which is why the defect was latent through two books.
    Corpus-wide the shape occurs on **650** `DESC:` tokens across the
    `*_abilities_race*.lst` files
    (``grep -rhoE 'DESC:[^\\t]*\\|![A-Z]+[A-Z:]*' --include='*_abilities_race*.lst' .``
    from the PCGen `data/` root), so it would have recurred in the lane's every
    remaining book.
    """
    return entry.lstrip("!").startswith("PRE")


FULL_ABILITY_RULE = "DisplayFullAbility"


class UnmodelledDesc(Exception):
    """A row states several `DESC:` texts under a gate `parse_desc` cannot read.

    Raised rather than exiting so the caller can decide whether the row matters.
    **A row that is going to be DROPPED does not matter**, and until Bestiary 3
    every unmodelled row happened also to be one that would ship, so exiting
    from inside the parser and exiting from the transcription were the same
    thing. They are not: `ability_pi_reason` parses EVERY ability row, including
    the orphans that the pass below it discards, so an unmodelled shape on a row
    no monster owns stopped a transcription over a record that was never going
    to be emitted.

    `b3_abilities_race.lst:1663` (`Jiang-Shi Vampire`) is the first instance --
    11 `DESC:` tokens, none gated on `DisplayFullAbility`, describing an
    acquired template in 11 sections. It is an orphan: no `b3_races.lst` monster
    row names it, and the base creature row it templates is commented out at
    `b3_races.lst:293`.

    The refusal itself is unchanged and still hard for any row that SHIPS -- the
    emission pass re-parses and lets this propagate, so a shape the parser
    cannot read can never reach a player. Only the drop path swallows it.
    """


def parse_desc(row: list[str]) -> tuple[str | None, list[str]]:
    """The `DESC:` text a player should read, plus the variables its `%N` name.

    **A row may carry more than one `DESC:` token**, and Book of the Damned
    Volume 2 is the first ingested book that does: 15 of its 17 ability rows
    carry two, one gated ``!PRERULE:1,DisplayFullAbility`` (a one-line summary,
    shown when PCGen's full-ability rule is off) and one gated
    ``PRERULE:1,DisplayFullAbility`` (the complete rules text). Taking the first
    match — which is what a single-`DESC:` book let this parser do — serves the
    **summary** and silently drops the mechanics: `Seraptis ~ Gaze of Despair`
    would reach the catalog as *"fills the minds of those within %1 feet with
    …despair"* and never mention the save, the Charisma drain or the duration.

    So when a row states both, the full-text one is selected. This is a choice
    between two verbatim corpus texts on a criterion the corpus itself states,
    never a composition of one. A row carrying several `DESC:` tokens under some
    *other* gate is an unmodelled shape and stops the transcription rather than
    being resolved by position.

    **The CONTINUATION shape, widened deliberately in round 7 (Inner Sea
    Bestiary).** A third shape exists and this book is the first whose rows
    carrying it actually ship: several `DESC:` tokens, *none* of which carries a
    pipe-delimited entry at all -- no gate, no `%N` variable -- and every token
    after the first beginning with a space. That is one description the corpus
    split across tokens, and PCGen renders them in row order; `Moxix ~ Gush`
    states its trigger in the first and its effect in the second. Taking the
    first alone would serve *"blood and pus spews forth from the wound."* and
    silently drop the 20-foot radius, the DC 28 Reflex save and the duration --
    the same class of loss `decisions.md §46`'s summary-vs-full finding
    recorded, arrived at from the other direction.

    Joining them is a concatenation of verbatim corpus texts in the corpus's own
    order using the corpus's own separator (the leading space each continuation
    carries), never a composition. The predicate is deliberately narrow: a row
    whose several tokens carry ANY pipe entry is a variant/gated shape and still
    refuses. `isb_abilities_race.lst:203`/`:204`/`:206` are exactly that -- they
    carry `%N` variables and state alternatives rather than a continuation -- and
    they are still refused by this parser rather than joined.

    **Two further shapes, widened in round 8 (Bestiary 1), one row each.** Both
    are the round-3 summary-vs-full pair in a row that carries NO gate, and both
    are selections between two verbatim corpus texts on a criterion the corpus
    itself states -- never a composition, never a positional guess:

    * **superset** -- one token's text literally begins with every other token's,
      so the long one contains the short one whole. `b1_abilities_race.lst:1183`.
    * **variable-bearing** -- exactly one token carries a pipe entry, and every
      entry it carries names a variable this row's own `DEFINE:` declares, so the
      row's `DEFINE:`/`BONUS:VAR` machinery exists to fill that token's `%N` and
      no other's. `b1_abilities_race.lst:1068`, whose ungated summary drops the
      severing AC, the Fortitude DC and the Strength damage.

    Scope derived, not assumed: over every book in `BOOKS`, 54 ability rows carry
    several `DESC:` tokens -- 34 gated-full, 4 continuation, 1 superset, 1
    variable-bearing, and 14 that remain refused. Not one of the 14 is a row any
    book ships; every one is an orphan or a Product Identity row.
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
        if len(full) == 1:
            descs = full
        else:
            if all("|" not in d for d in descs) and all(
                d.startswith(" ") for d in descs[1:]
            ):
                return "".join(descs), []
            texts = [d.split("|")[0] for d in descs]
            longest = max(texts, key=len)
            if all(longest.startswith(t) for t in texts):
                # SUPERSET shape. One token's text literally BEGINS with every
                # other token's, so the long one contains the short one whole
                # and selecting it drops not one corpus word. No composition, no
                # positional guess, and no criterion of this script's invention
                # -- the containment is the corpus's own statement.
                descs = [descs[texts.index(longest)]]
            else:
                piped = [d for d in descs if "|" in d]
                defines = {
                    f[len("DEFINE:") :].split("|")[0]
                    for f in row
                    if f.startswith("DEFINE:")
                }
                entries = (
                    [e for e in piped[0].split("|")[1:] if e] if len(piped) == 1 else []
                )
                if entries and all(e in defines for e in entries):
                    # VARIABLE-BEARING shape. Exactly one token carries a pipe
                    # entry, and every entry it carries names a variable this
                    # row's own `DEFINE:` declares -- the row's `DEFINE:` /
                    # `BONUS:VAR` machinery exists to fill that token's `%N`
                    # placeholders and nothing else's. So the corpus states which
                    # of the two texts is the complete one, exactly as the
                    # `DisplayFullAbility` gate does above, in a row that carries
                    # no gate. `b1_abilities_race.lst:1068` is the whole of this
                    # shape corpus-wide; its ungated summary stops at "as ranged
                    # touch attacks" and drops the severing AC, the save and the
                    # Strength damage -- `decisions.md §46`'s loss again.
                    descs = piped
                else:
                    raise UnmodelledDesc(
                        f"row carries {len(descs)} DESC: tokens, none gated on "
                        f"{FULL_ABILITY_RULE}, no continuation, no superset and no single "
                        "token bearing this row's own DEFINEd variables; the transcriber "
                        f"refuses to pick one by position. Tokens: {descs!r}"
                    )
        assert len(descs) == 1, "every branch above narrows to exactly one token"
    parts = descs[0].split("|")
    return parts[0], [p for p in parts[1:] if p and not is_prerequisite(p)]


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


def cross_table_served_monster_keys(corpus_dir: str) -> set[str]:
    """Monster corpus keys another compiled table of the same book already ships.

    Reads `data/corpus/<corpus_dir>/monster/` and returns the `source.record_key`
    of every record written in the pre-`key` Shape B v1 shape -- `data.id` and no
    `data.key`. That is precisely SD-22's `beastiary1` output and precisely not
    this generator's, so the set is stable under re-running the chassis pass over
    the same directory.

    An empty result is a hard stop rather than an empty exclusion: an exclusion
    that silently becomes a no-op would ship 46 duplicate stat blocks, and a
    duplicate under one wire code is a defect a player can see.
    """
    root = os.path.join("data/corpus", corpus_dir, "monster")
    served: set[str] = set()
    for name in sorted(os.listdir(root)):
        if not name.endswith(".json"):
            continue
        with open(os.path.join(root, name), encoding="utf-8") as handle:
            record = json.load(handle)
        data = record.get("data", {})
        if "id" in data and "key" not in data:
            served.add(record["source"]["record_key"])
    if not served:
        raise SystemExit(
            f"{root} holds no `data.id`-shaped records, so the cross-table "
            "exclusion would be a no-op -- refusing to ship duplicates"
        )
    return served


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

    # ---- Product Identity screen, applied BEFORE the orphan pass ----
    #
    # Inner Sea World Guide is the first book in this lane to carry a PI term
    # inside a record's own KEY: `Daughter of Urgathoa` and its three abilities
    # name a Golarion deity. `decisions.md §46.4` predicted the opposite -- both
    # Book of the Damned volumes read `records_redacted: 0` and the derived
    # reason was that a monster row is a stat block, not setting prose. That
    # holds for a *description*; it does not hold for a NAME, and a name is the
    # one field redaction cannot touch. `[redacted PI]` as a monster's key would
    # be a record no one can look up, which is worse than not shipping it.
    #
    # So a PI-carrying record is DROPPED, not redacted, and dropping a monster
    # cascades: its abilities lose their only owner and fall out through the
    # orphan pass below. Nothing here reclassifies a term -- reclassification is
    # `docs/governance/ogl-pi-blacklist.md` §3's per-book override and an
    # operator decision, not a transcriber's.
    terms = pi_blacklist_terms()

    def monster_pi_reason(unit: dict) -> str | None:
        row = monster_rows[unit["corpus_key"]]
        if token(row, "NAMEISPI:") == "YES":
            return "NAMEISPI:YES"
        # Only the values this transcription EMITS are screened, because those
        # are the values `gen_book_cache` serializes and screens in turn. An
        # earlier draft screened every token of the row and dropped the
        # Sandpoint Devil for `AUTO:LANG|Abyssal|Varisian` -- a language grant
        # that never reaches a record, matching the blacklist's `Varisia` as a
        # substring of `Varisian`. Over-exclusion is a real cost: it silently
        # drops corpus content nothing was going to publish.
        hits = pi_hits(
            terms,
            unit["corpus_key"],
            unit["name"],
            parse_size(row),
            token(row, "RACETYPE:"),
            token(row, "RACESUBTYPE:"),
            token(row, "CR:"),
            token(row, "MONSTERCLASS:"),
            token(row, "SOURCEPAGE:"),
            *(m for m, _ in parse_speeds(row)),
            *(n for n, _ in parse_natural_attacks(row)),
            *monster_ability_keys[unit["corpus_key"]],
            *external[unit["corpus_key"]],
        )
        # The reason names the SCREEN, never the term: the reason string is
        # written into a checked-in generated file, and `pi_table_sweep` rejects
        # a Product Identity term anywhere under `rules_tables/` regardless of
        # the sentence around it. `len(hits)` is enough to tell a reader whether
        # one term matched or several.
        return f"{len(hits)} PI_BLACKLIST_TERMS hit(s) in emitted values" if hits else None

    # Ability rows whose `DESC:` shape the parser refuses. Populated by the PI
    # screen, checked after the orphan pass: any that SURVIVED must stop the
    # transcription, because they are about to be emitted.
    unscreenable: set[str] = set()

    def ability_pi_reason(unit: dict) -> str | None:
        row = read_row(os.path.join(root, unit["source_file"]), unit["source_line"])
        if token(row, "NAMEISPI:") == "YES":
            return "NAMEISPI:YES"
        _facet, _delivery, traits = parse_type(row)
        try:
            description, variables = parse_desc(row)
        except UnmodelledDesc:
            # This row cannot be screened, so it is not reported as Product
            # Identity -- an unscreenable row is not a clean row, and saying
            # "no PI" about one would be an unearned claim. Nothing is waived:
            # if the row survives the orphan pass it is EMITTED, and the
            # emission pass re-parses it and lets the refusal propagate, so an
            # unscreened row can never reach the generated table. If it does not
            # survive, the reason it is absent is that nothing owns it, which is
            # the reason the header should state.
            unscreenable.add(unit["corpus_key"])
            return None
        hits = pi_hits(
            terms,
            unit["corpus_key"],
            unit["name"],
            description,
            token(row, "SOURCEPAGE:"),
            *traits,
            *variables,
            *owners[unit["corpus_key"]],
        )
        return f"{len(hits)} PI_BLACKLIST_TERMS hit(s) in emitted values" if hits else None

    # Monsters first, and the ability screen runs only AFTER their owners are
    # withdrawn. `owners` is an emitted field, so an ability whose owner is a
    # PI-dropped monster would otherwise be reported as a PI hit on the owner's
    # name when the true reason is that it has become an orphan. Screening in
    # this order reports each row under the reason that actually applies to it.
    pi_monsters = [(u, r) for u in monsters if (r := monster_pi_reason(u))]
    dropped_keys = {u["corpus_key"] for u, _ in pi_monsters}
    if dropped_keys:
        monsters = [u for u in monsters if u["corpus_key"] not in dropped_keys]
        for key in dropped_keys:
            monster_ability_keys.pop(key, None)
            external.pop(key, None)
        for ability_key in owners:
            owners[ability_key] = [o for o in owners[ability_key] if o not in dropped_keys]

    pi_abilities = [(u, r) for u in abilities if (r := ability_pi_reason(u))]
    if pi_monsters or pi_abilities:
        dropped_ability_keys = {u["corpus_key"] for u, _ in pi_abilities}
        abilities = [u for u in abilities if u["corpus_key"] not in dropped_ability_keys]
        for key in monster_ability_keys:
            monster_ability_keys[key] = [
                a for a in monster_ability_keys[key] if a not in dropped_ability_keys
            ]
        # stderr may name the keys: it is a console message, not a checked-in
        # file, and an operator ruling on the exclusion needs to know what was
        # excluded.
        print(
            f"{book}: PI screen dropped {len(pi_monsters)} monster row(s) and "
            f"{len(pi_abilities)} ability row(s): "
            + ", ".join(
                f"{u['corpus_key']} ({reason})" for u, reason in pi_monsters + pi_abilities
            ),
            file=sys.stderr,
        )

    # ---- `.COPY=` screen, between the PI screen and the orphan pass ----
    #
    # A `<Base>.COPY=<Variant>` row does not state a stat block. It states a
    # DELTA on one: PCGen copies the base record whole and then applies the few
    # tokens the copy row carries. Bestiary 2 is the first book in this lane to
    # carry any (`b2_races.lst:454` and `:594`, the only two in the whole
    # corpus -- derived, not assumed:
    #
    #   python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    #   print(sum(1 for u in d['units'] if u['kind'] in ('monster','monster_ability')
    #   and u.get('origin')=='copy'))"   -> 2
    #
    # Transcribing one verbatim -- which is all this script does -- produces a
    # record with a challenge rating and NOTHING else: no size, no speed, no
    # type, no page. That is a card a player opens to find blank, the stub class
    # `docs/governance/no-stub-mvp-doctrine.md` forbids, and `gen_book_cache`'s
    # `verified_citation_line` refuses it outright anyway, because the row's
    # first column reads `<Base>.COPY=<Variant>` and not the record's name.
    #
    # Resolving the delta is not a transcription. It means composing values
    # across two rows while `MonsterStatBlock` carries ONE `source_file` /
    # `source_line` pair, so every inherited field would ship under a citation
    # that does not contain it -- precisely the stale-citation defect
    # `verified_citation_line` and `v06_corpus_trap_report --audit` exist to
    # catch. A chassis that models inheritance needs a second citation, and that
    # is a deliberate widening, not something to slip into an ingest round.
    #
    # So a `.COPY=` row is DROPPED, exactly as a PI row is, and it cascades the
    # same way: an ability owned only by a dropped variant falls out through the
    # orphan pass below.
    copy_monsters = [
        u
        for u in monsters
        if ".COPY=" in (monster_rows[u["corpus_key"]][0] if monster_rows[u["corpus_key"]] else "")
    ]
    if copy_monsters:
        copy_keys = {u["corpus_key"] for u in copy_monsters}
        monsters = [u for u in monsters if u["corpus_key"] not in copy_keys]
        for key in copy_keys:
            monster_ability_keys.pop(key, None)
            external.pop(key, None)
        for ability_key in owners:
            owners[ability_key] = [o for o in owners[ability_key] if o not in copy_keys]
        print(
            f"{book}: {len(copy_monsters)} `.COPY=` derived monster row(s) NOT transcribed "
            "(a copy row states a delta on another record, not a stat block): "
            + ", ".join(
                f"{u['source_file']}:{u['source_line']}" for u in copy_monsters
            ),
            file=sys.stderr,
        )

    # ---- `.MOD`-only screen, beside the `.COPY=` screen and for its reason ----
    #
    # A `<Record>.MOD` row does not state a stat block either. It states a DELTA
    # on a record defined elsewhere, and the work inventory has always said so in
    # its own `origin` field: `declared` for a row that defines a record,
    # `mod_only` for a unit whose every corpus row is an overlay. This screen
    # reads that field rather than re-deriving it, because the inventory's parser
    # is what decided the unit existed at all.
    #
    # Transcribing one verbatim yields the delta's few tokens under the record's
    # name -- the same blank card `.COPY=` produces -- and
    # `gen_book_cache::verified_citation_line` refuses it outright anyway,
    # because the row's first column reads `<Record>.MOD` and not the record's
    # name. Resolving the delta means composing across the base row and every
    # overlay, under ONE `source_file`/`source_line` pair, which is the
    # stale-citation defect that function exists to catch.
    #
    # Scope derived, not assumed. Over every book in the inventory:
    #   python3 -c "import json, collections; d=json.load(open('docs/work-inventory.json'));
    #   print(collections.Counter(u.get('origin') for u in d['units']
    #   if u['kind'] in ('monster','monster_ability')))"
    # -> `declared 4371, mod_only 4, copy 2`. All 4 `mod_only` units are this
    # book's monster rows; not one ability row in any book carries the shape.
    mod_monsters = [u for u in monsters if u.get("origin") == "mod_only"]
    if mod_monsters:
        mod_keys = {u["corpus_key"] for u in mod_monsters}
        monsters = [u for u in monsters if u["corpus_key"] not in mod_keys]
        for key in mod_keys:
            monster_ability_keys.pop(key, None)
            external.pop(key, None)
        for ability_key in owners:
            owners[ability_key] = [o for o in owners[ability_key] if o not in mod_keys]
        print(
            f"{book}: {len(mod_monsters)} `.MOD`-only monster row(s) NOT transcribed "
            "(an overlay row states a delta on a record defined elsewhere): "
            + ", ".join(f"{u['source_file']}:{u['source_line']}" for u in mod_monsters),
            file=sys.stderr,
        )

    # ---- cross-table-owner screen, between the `.COPY=` screen and the orphan
    # pass ----
    #
    # `decisions.md §58.3`'s ruling, executed. A monster row this repo ALREADY
    # ships out of a different compiled table is not this chassis's to emit: two
    # records for one creature, under one wire code, in one catalog, is a
    # duplicate the player sees. So the row is dropped here -- and the drop
    # cascades exactly as the Product Identity and `.COPY=` drops above it do,
    # except that the abilities it strands are NOT orphans and must not be
    # reported as such.
    #
    # An orphan is a row nothing in the book owns. A CROSS-TABLE OWNER row is
    # well-formed and owned; it is unreachable from here only because its owner
    # lives in the other table, which has no ability family at all. That is a
    # different remedy (widen the other table, or migrate it), so it is a
    # different class, counted and cited separately in the header below.
    cross_table_monsters: list[dict] = []
    cross_table_abilities: list[dict] = []
    other_table_dir = CROSS_TABLE_MONSTER_RECORDS.get(book)
    if other_table_dir:
        served = cross_table_served_monster_keys(other_table_dir)
        cross_table_monsters = [u for u in monsters if u["corpus_key"] in served]
        cross_keys = {u["corpus_key"] for u in cross_table_monsters}
        monsters = [u for u in monsters if u["corpus_key"] not in cross_keys]
        for key in cross_keys:
            monster_ability_keys.pop(key, None)
            external.pop(key, None)
        stranded: set[str] = set()
        for ability_key in owners:
            before = owners[ability_key]
            after = [o for o in before if o not in cross_keys]
            if before and not after:
                stranded.add(ability_key)
            owners[ability_key] = after
        cross_table_abilities = [u for u in abilities if u["corpus_key"] in stranded]
        abilities = [u for u in abilities if u["corpus_key"] not in stranded]
        print(
            f"{book}: cross-table screen withheld {len(cross_table_monsters)} monster row(s) "
            f"already served by `data/corpus/{other_table_dir}/monster` and "
            f"{len(cross_table_abilities)} ability row(s) owned only by them",
            file=sys.stderr,
        )

    # An ability row no monster row of this book claims is an ORPHAN: the
    # catalog renders an ability underneath its owning monster, so a record with
    # no owner would load and never be shown -- the stub class `decisions.md
    # §44.2` was written about. Round 2 dodged the question by taking the only
    # two remaining orphan-free books; from round 3 on, every candidate book has
    # orphans, and the rule is `kanban.md`'s: transcribe the LINKED subset and
    # carry the orphans as an `OPEN_FINDINGS` entry naming their remedy. They
    # stay `not-ingested` in the work inventory, which is the honest status --
    # not `grounded`, and not silently emitted as unreachable rows.
    orphans = [u for u in abilities if not owners[u["corpus_key"]]]
    abilities = [u for u in abilities if owners[u["corpus_key"]]]
    if orphans:
        print(
            f"{book}: {len(orphans)} orphan ability row(s) NOT transcribed "
            "(no monster row of this book owns them): "
            + ", ".join(u["corpus_key"] for u in orphans),
            file=sys.stderr,
        )

    # The deferred half of `UnmodelledDesc`. A row the parser refused is fine
    # only if something else already dropped it; one that reached this point is
    # on its way into the generated table, and the refusal is now this
    # transcription's refusal.
    shipping_unscreenable = [u for u in abilities if u["corpus_key"] in unscreenable]
    if shipping_unscreenable:
        raise SystemExit(
            f"{book}: "
            + "; ".join(
                f"{u['source_file']}:{u['source_line']} ({u['corpus_key']})"
                for u in shipping_unscreenable
            )
            + " would be transcribed but carry a `DESC:` shape parse_desc refuses. "
            "Widen it deliberately."
        )

    def source_files(units: list[dict]) -> list[str]:
        """Distinct `.lst` files these units were read from, in first-seen order.

        A book is not guaranteed one file per kind: Inner Sea World Guide's 14
        monsters are split 7/7 across `iswg_races.lst` and
        `iswg_races_bestiary.lst`. Naming `units[0]['source_file']` in the header
        -- which this script did until round 3 -- silently mis-cites half of them.
        """
        seen: list[str] = []
        for unit in units:
            if unit["source_file"] not in seen:
                seen.append(unit["source_file"])
        return seen

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
    for name in source_files(monsters):
        count = sum(1 for u in monsters if u["source_file"] == name)
        out.append(f"//!   * `{name}` -- {count} monster rows")
    for name in source_files(abilities):
        count = sum(1 for u in abilities if u["source_file"] == name)
        out.append(f"//!   * `{name}` -- {count} monster-ability rows")
    if pi_monsters or pi_abilities:
        out.append("//!")
        out.append(
            f"//! {len(pi_monsters)} monster row(s) and {len(pi_abilities)} ability row(s) of this"
        )
        out.append(
            "//! book are Product Identity and are NOT transcribed -- either because the corpus"
        )
        out.append(
            "//! row DECLARES it (`NAMEISPI:YES`, PCGen's own per-record marker) or because an"
        )
        out.append(
            "//! emitted value carries a `pi_screening::PI_BLACKLIST_TERMS` term. Both land in"
        )
        out.append(
            "//! the name or key, which is the one field redaction cannot touch. Reclassifying"
        )
        out.append(
            "//! is `docs/governance/ogl-pi-blacklist.md` §3's per-book override, an operator"
        )
        out.append("//! decision, not a transcriber's:")
        # The row is cited by FILE:LINE and never by its key. `pi_table_sweep`
        # rejects a Product Identity term anywhere under `rules_tables/`,
        # including a comment explaining why the term was excluded -- the sweep
        # does not read intent, and a generated header has no need to
        # instantiate the very name it is recording the removal of. The
        # citation is also the better identifier: a reader checks it against
        # the corpus, where the name legitimately lives.
        for unit, reason in pi_monsters + pi_abilities:
            out.append(
                f"//!   * `{unit['source_file']}:{unit['source_line']}` "
                f"({'monster' if unit['kind'] == 'monster' else 'ability'} row, {reason})"
            )
    if copy_monsters:
        out.append("//!")
        out.append(
            f"//! {len(copy_monsters)} monster row(s) of this book are `<Base>.COPY=<Variant>`"
        )
        out.append(
            "//! derived rows and are NOT transcribed. A copy row states a DELTA on another"
        )
        out.append(
            "//! record, not a stat block -- transcribing one verbatim yields a card with a"
        )
        out.append(
            "//! challenge rating and no size, speed, type or page. Resolving the delta means"
        )
        out.append(
            "//! composing two rows under one `source_line`, which is the stale-citation defect"
        )
        out.append(
            "//! `gen_book_cache::verified_citation_line` refuses. It needs a second citation on"
        )
        out.append("//! the chassis, deliberately widened, not an ingest round's side effect:")
        for unit in copy_monsters:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}`")
    if mod_monsters:
        out.append("//!")
        out.append(
            f"//! {len(mod_monsters)} monster row(s) of this book are `<Record>.MOD` OVERLAY"
        )
        out.append(
            "//! rows and are NOT transcribed, for the reason above: an overlay states a delta"
        )
        out.append(
            "//! on a record defined elsewhere, not a stat block. The work inventory classes"
        )
        out.append(
            "//! them `origin: mod_only` itself, and this screen reads that field rather than"
        )
        out.append("//! re-deriving it:")
        for unit in mod_monsters:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}`")
    if cross_table_monsters or cross_table_abilities:
        out.append("//!")
        out.append(
            f"//! {len(cross_table_monsters)} monster row(s) of this book are already shipped by"
        )
        out.append(
            "//! ANOTHER compiled table of this repo and are deliberately NOT transcribed here"
        )
        out.append(
            "//! (`decisions.md §58.3`: this chassis sits ALONGSIDE that table and takes the"
        )
        out.append(
            "//! book's complement -- emitting them too would put two records for one creature"
        )
        out.append(
            f"//! under one wire code). {len(cross_table_abilities)} further ability row(s) are"
        )
        out.append(
            "//! CROSS-TABLE OWNER rows: well-formed and owned, unreachable from here only"
        )
        out.append(
            "//! because every monster that names them is one of those rows. They are NOT"
        )
        out.append(
            "//! orphans and their remedy is not the orphans' remedy. Cited by corpus line:"
        )
        for unit in cross_table_monsters:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}` (monster row)")
        for unit in cross_table_abilities:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}` (ability row)")
    if orphans:
        out.append("//!")
        out.append(
            f"//! {len(orphans)} further ability row(s) in this book are ORPHANS -- no monster"
        )
        out.append(
            "//! row here claims them, so they are deliberately NOT transcribed (a record"
        )
        out.append(
            "//! with no owner loads and is never shown). `not-ingested` is their honest status"
        )
        out.append(
            "//! in the work inventory, and the round's receipt records them by key:"
        )
        # Cited by FILE:LINE, not by key, for the same reason the PI block above
        # is: an orphan created by a PI drop carries the dropped row's declared
        # Product Identity name in its own namespaced key.
        for unit in orphans:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}`")
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
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
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
