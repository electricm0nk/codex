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

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from codex_neutral_name import (  # noqa: E402
    divergence_entry,
    neutral_key,
    neutral_name,
)

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
    # SD-29 Epic 5 extend, round 9, and the FIRST book in this lane whose rows
    # do not all live in the book's root directory. 3 of its 39 monster rows and
    # 16 of its 161 ability rows sit under `support/`, and the inventory records
    # every unit's `source_file` as a BARE BASENAME -- so `os.path.join(root,
    # name)` (correct by coincidence for the nine books above) raises
    # FileNotFoundError here. `resolve_book_file` is that widening; the matching
    # one on the Rust side is `MonsterAbilityRecord::source_file` plus
    # `MonsterBookSpec::abilities_lsts`, which until this round was singular.
    # Derived, never assumed:
    # `find ~/workspace/repos/pcgen/data -ipath '*inner_sea_gods*' -name '*races*'`
    # -> `isg_races.lst`, `isg_abilities_races.lst`,
    #    `support/isg_races_b4.lst`, `support/isg_abilities_races_b4.lst`.
    #
    # The `support/` pair is NOT unconditionally loaded and is NOT out of scope
    # either. `_inner_sea_gods.pcc:68` and `:70` gate both on
    # `PRECAMPAIGN:1,INCLUDES=Bestiary 4` -- a gate this repo satisfies since
    # round 6 registered `bestiary_4`. That is the `PRECAMPAIGN` hazard
    # `loop-instruction.md`'s corpus shape notes describe, read from the PCC
    # LOAD LINE rather than from inside the `.lst` (`grep PRECAMPAIGN` over
    # those two `.lst` files returns 0).
    #
    # `python3 scripts/classify_monster_ability_rows.py inner_sea_gods` ->
    # `inner_sea_gods  39  161  0  77  81  3  0`, i.e. 116 reachable -- 73% of
    # the whole lane's REAL remainder at the start of this round.
    "inner_sea_gods": "pathfinder/paizo/campaign_setting/inner_sea_gods",
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
    # SD-29 Epic 5 extend, round 10, and the FIRST non-Paizo book in this lane
    # (Dreamscarred Press). It is also the first book the lane takes whose
    # `RuleSetId` was already compiled for OTHER kinds -- `RuleSetId::Upsi` has
    # served this book's 221 feats and 552 equipment records since SD-28 E29 --
    # so registering it costs no new rule set and no new corpus directory.
    #
    # Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py ultimate_psionics` ->
    # `ultimate_psionics  21  79  3  10  66  0  0`, i.e. 34 reachable.
    #
    # Both files sit at the book ROOT (`up_races.lst`, `up_abilities_race.lst`),
    # so `resolve_book_file` resolves each in one hop and round 9's widening is
    # not load-bearing here. `up_races_apg.lst` also exists and contributes ZERO
    # `monster` units -- the unit set comes from the inventory, not from a glob:
    # `python3 -c "...Counter((u['kind'],u['source_file']) for u in units if
    # u['book']=='ultimate_psionics')"` -> only `up_races.lst` (21 monster) and
    # `up_abilities_race.lst` (79 monster_ability).
    #
    # Zero Product Identity rows in either signal, which is what
    # `ogl-pi-blacklist.md` §2.1's PER-RECORD predicate predicts for a
    # psionics rules supplement whose creatures are generic species (Blue,
    # Dromite, Elan, Maenad, Ophiduan, Xeph) rather than named personae.
    "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics",
    # SD-29 Epic 5 extend, FINAL round, and the last book in this lane with any
    # workable unit at all. Nine units -- 3 monster rows and the 6 ability rows
    # namespaced to them -- and they are the entire REAL remainder of the
    # monster lane. Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py horror_adventures` ->
    # `horror_adventures  3  71  0  6  65  0  0`, i.e. 9 reachable against 65
    # orphans.
    #
    # Like `ultimate_psionics` this book's `RuleSetId` was already compiled for
    # other kinds -- `RuleSetId::Ha` has served its `race_trait` family since
    # Epic 6 round 3 and its `companion` family since Epic 7 -- so registering
    # it costs no new rule set and no new corpus directory.
    #
    # Both files sit at the book ROOT and both load UNCONDITIONALLY:
    # `grep -n 'ha_races.lst\|ha_abilities_race.lst' _horror_adventures.pcc` ->
    # `63:ABILITY:ha_abilities_race.lst` and `77:RACE:ha_races.lst`, neither
    # carrying a `PRECAMPAIGN` gate. That is the check round 9 made load-bearing
    # and the one that disqualified this lane's OTHER nominally-workable book --
    # see `decisions.md`'s negated-gate finding for `occult_adventures`.
    #
    # Zero Product Identity rows in either signal (`grep -c NAMEISPI:YES
    # ha_races.lst ha_abilities_race.lst` -> 0, 0), which is what
    # `ogl-pi-blacklist.md` §2.1's PER-RECORD predicate predicts: the three rows
    # are a generic hive-insect species set (Hive Larva Swarm, Hive Queen, Hive
    # Warrior), not named personae.
    "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures",
    # `decisions.md §20` no_record-to-zero, round 3: the first five of nine
    # ZERO-monster books this lane had never registered at all (the prior
    # receipt's "no further apply-the-mechanism-to-book-N cycles remain" was
    # re-derived and found stale -- `python3 scripts/classify_monster_ability_
    # rows.py` shows 171 orphan rows across 8 unregistered books with zero
    # monster rows of their own, so nothing can ever own them and every row
    # ships owner-less by construction, the identical shape this script
    # already handles generically). `mythic_adventures` (21 rows) is deferred
    # here still: its `rules_tables/` module directory does not exist yet and
    # needs real scaffolding, not just a registry row.
    #
    # Zero Product Identity rows in the abilities file for 4 of 5 (`grep -c
    # 'NAMEISPI:YES\|DESCISPI:YES' <file>` -> 0); `ultimate_wilderness`
    # carries 1 hit in its 296-line file, screened per-record by the
    # transcriber's own `ability_pi_reason` exactly like every other book
    # (not assumed clear here).
    "ultimate_wilderness": "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
    "ultimate_intrigue": "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
    "ultimate_magic": "pathfinder/paizo/roleplaying_game/ultimate_magic",
    "bestiary_6": "pathfinder/paizo/roleplaying_game/bestiary_6",
    "bestiary_5": "pathfinder/paizo/roleplaying_game/bestiary_5",
    # `decisions.md §20` no_record-to-zero, round 4: the two remaining
    # zero-monster books of the original 8, now registered. Both already have
    # a dedicated hand-rolled `gen_book_cache.rs` function
    # (`gen_pathfinder_unchained`/`gen_advanced_race_guide`) that emits their
    # OTHER families (feats, equipment, ...); this transcription is unaffected
    # by that and still only writes `monster_data.rs` -- the generator
    # function itself is extended (round 4) to also call `gen_monster_book`
    # after its existing writes, reusing the identical `MonsterBookSpec`-driven
    # mechanism every other book here uses. `pu_abilities_race.lst` (72 rows)
    # and `arg_abilities_race.lst` (1 row) both load UNGATED at their book's
    # own `.pcc` root (`grep -n 'ABILITY:pu_abilities_race.lst'
    # _pathfinder_unchained.pcc` -> line 43, no `PRECAMPAIGN`; `grep -n
    # 'ABILITY:arg_abilities_race.lst' advanced_race_guide.pcc` -> line 57, no
    # `PRECAMPAIGN`). Zero Product Identity rows in either file (`grep -c
    # NAMEISPI:YES pu_abilities_race.lst arg_abilities_race.lst` -> 0, 0).
    "pathfinder_unchained": "pathfinder/paizo/roleplaying_game/pathfinder_unchained",
    "advanced_race_guide": "pathfinder/paizo/roleplaying_game/advanced_race_guide",
    # `decisions.md §20` no_record-to-zero, round 5: the last of the original
    # ZERO-monster books this lane had never registered, deferred by round 4
    # ("its `rules_tables/` module directory does not exist yet"). That
    # deferral is now stale -- a sibling T2 (`spell`) lane already created
    # `rules_tables/mythic_adventures/` (for `spell_list`, `decisions.md §20`
    # spell round), so this cycle only adds `mod monster_data;` to the
    # existing module, not a whole new directory. Derived, never assumed:
    # `python3 scripts/classify_monster_ability_rows.py mythic_adventures` ->
    # `mythic_adventures 0 21 0 0 21 0 0` (0 monster rows, 21 ability rows, all
    # 21 orphan, 0 PI, 0 `.COPY=`) -- the identical zero-monster shape every
    # other book in this dict already handles. `ma_abilities_race.lst` (21
    # rows) loads UNGATED at the book's own `.pcc` root (`grep -n
    # 'ABILITY:ma_abilities_race.lst' _mythic_adventures.pcc` -> line 40, no
    # `PRECAMPAIGN`). Zero Product Identity rows (`grep -c
    # 'NAMEISPI:YES\|DESCISPI:YES' ma_abilities_race.lst` -> 0).
    "mythic_adventures": "pathfinder/paizo/roleplaying_game/mythic_adventures",
    # `decisions.md §27b` -- EVERYTHING: the repeatedly-reconfirmed
    # "correctly out of scope" disposition for this book is OVERTURNED.
    # "Not applicable to the modelled campaign set" was a reachability
    # statement about the negated `!PRECAMPAIGN:1,INCLUDES=Bestiary 3` gate on
    # `support/oa_races_b3.lst`/`support/oa_abilities_race_b3.lst`
    # (`_occult_adventures.pcc:74-75`), never an ingest statement -- the
    # objects exist in the book and are ingested here like every other book.
    # Reachability stays a SEPARATE number (`decisions.md §16`), reported
    # honestly as 0 by `monster_chassis.rs`'s reach-gate: no owning race row
    # in this dict's scope claims any of the 5 in-scope ability rows by name
    # (`grep -n 'Homunculus Companion ~\|Shikigami ~' oa_abilities_race.lst
    # support/oa_abilities_race_b3.lst` shows only CATEGORY:Internal umbrella
    # rows referencing them, which this generator does not resolve into
    # ownership), so all 5 ship owner-less, the identical honest shape
    # `mythic_adventures`/`ultimate_wilderness`/`ultimate_intrigue`/
    # `ultimate_magic` above already ship.
    #
    # `races_lsts: []` deliberately, matching those same four precedent rows:
    # this book's `race`/`monster`-kind rows (`oa_races.lst`'s 4,
    # `oa_races_b3.lst`'s 1) are a DIFFERENT `docs/work-inventory.json` kind
    # (`race`/`monster`, not `monster_ability`) and outside this cycle's
    # scope -- registering them here would risk emitting `MonsterStatBlock`
    # records into a sibling lane's territory rather than the 5
    # `monster_ability` units this cycle closes. Derived, never assumed:
    # `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
    # cross-referenced against `docs/work-inventory.json`'s own
    # `book=="occult_adventures"` rows -- exactly 5 `monster_ability` units,
    # all `engine-does-not-hold`; the `race`(4)/`monster`(1) rows are a separate kind
    # this cycle does not touch. Zero Product Identity rows in either
    # abilities file (`grep -c 'NAMEISPI:YES\|DESCISPI:YES'
    # oa_abilities_race.lst support/oa_abilities_race_b3.lst` -> 0, 0).
    "occult_adventures": "pathfinder/paizo/roleplaying_game/occult_adventures",
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

# The `TYPE:` segment that names which facet of `monster_ability` a row is.
# Spelled exactly as the corpus spells it. `Weakness`/`Defensive`/`Aura`/
# `Sense`/`Communicate` were added by the T9 five-book widening cycle
# (`decisions.md §16`'s caution applied): each is a distinct, repeated,
# corpus-native facet label -- never a semantic remapping onto
# `SpecialAttack`/`SpecialQuality` -- verified against
# `bestiary`/`bestiary_2`/`bestiary_3`/`inner_sea_bestiary`/`inner_sea_gods`'s
# own PI-cleared population before being added (the cycle's own receipt has
# the per-shape counts). A bare delivery-only `TYPE:` (no facet segment at
# all), the `CATEGORY:Internal` shape and one-off non-facet strings are
# deliberately NOT in this dict -- each needs a per-record read, not a
# vocabulary entry guessed from one sample.
FACETS = {
    "SpecialAttack": "SpecialAttack",
    "SpecialQuality": "SpecialQuality",
    "Weakness": "Weakness",
    "Defensive": "Defensive",
    "Aura": "Aura",
    "Sense": "Sense",
    "Communicate": "Communicate",
}
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


PI_MARKER_RS = "src/rules_core/shape_b_v1.rs"


def redacted_pi_marker() -> str:
    """`shape_b_v1::REDACTED_PI_MARKER`, parsed out of the Rust source.

    **Derived, never re-typed** -- same discipline as [`pi_blacklist_terms`]
    above, for the same reason: a hand-copied literal drifts silently the first
    time the const changes, and this is the exact string a `DESCISPI:YES`
    ability's redacted `description` field ships instead of its declared-PI
    prose (`pi_screening::classify_optional_field_declared`'s own redaction
    value, `decisions.md §39.4`/`§53`, applied by hand here because this
    transcriber emits a Rust literal table rather than a JSON record with a
    `license`/`pi_field`/`pi_marker` trio to route through the shared reader).
    """
    text = open(PI_MARKER_RS, encoding="utf-8").read()
    match = re.search(r'REDACTED_PI_MARKER:\s*&str\s*=\s*"([^"]*)"', text)
    if not match:
        raise SystemExit(
            f"{PI_MARKER_RS}: could not find `REDACTED_PI_MARKER` -- the const's "
            "shape changed and this parser must be updated with it"
        )
    return match.group(1)


def pi_hits(terms: list[str], *values: str | None) -> list[str]:
    """Every blacklist term appearing in any of these emitted values."""
    joined = " ".join(v for v in values if v)
    return [t for t in terms if t in joined]


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


# `decisions.md §9`: `core_essentials` is not a book. `SD31-ATTRIB-*` re-attributes
# its units' `book` field to their real book (e.g. `"bestiary"`), but the unit's
# `source_file` (`ce_abilities_race.lst`, `b4_abilities_races_ce.lst`, ...) is a
# physical file that never moves -- it stays under `core_essentials`'s own PCGen
# directory forever, because re-attribution is a reporting-field relabel, not a
# file move (`SD31-ATTRIB-001`'s own receipt: "zero doneness transitions... book
# is a pure reporting field"). `resolve_book_file` walked only the book's own
# root, so a re-attributed unit's `source_file` was unreachable under ANY book's
# root, confirmed live 2026-08-16 (`SD31-E6-F9-002`): re-running the transcriber
# for `bestiary` raised `SystemExit("ce_abilities_race.lst is not present
# anywhere under .../roleplaying_game/bestiary")`, and 108 (`bestiary`
# `ce_abilities_race.lst` 32, `bestiary_2` 72, `bestiary_3` 4) plus 28 of
# `bestiary_4`'s own `b4_abilities_races_ce.lst` -- 136 `static`/`derived`
# `monster_ability` units total -- sat `engine-does-not-hold` for exactly this reason,
# not because they are orphaned.
_CORE_ESSENTIALS_DIR = "pathfinder/paizo/roleplaying_game/core_essentials"


def _find_under(root: str, name: str) -> list[str]:
    """Every real path named `name` somewhere under `root`, sorted."""
    candidates = []
    for dirpath, _dirnames, filenames in os.walk(root):
        if name in filenames:
            candidates.append(os.path.join(dirpath, name))
    return sorted(candidates)


def resolve_book_file(root: str, name: str) -> str:
    """The real path of `name` inside a book directory, which is not always its root.

    ``v06_work_inventory`` records a unit's ``source_file`` as a BARE BASENAME.
    For the first nine books in this lane that was also the file's location, so
    ``os.path.join(root, name)`` was correct by coincidence rather than by rule.
    It is not correct for `inner_sea_gods`, whose 3 monster rows and 16 ability
    rows live in ``support/isg_races_b4.lst`` and
    ``support/isg_abilities_races_b4.lst``; nor for `occult_adventures`, whose
    single monster row lives in ``support/oa_races_b3.lst``.  Derived, not
    assumed::

        find ~/workspace/repos/pcgen/data -ipath '*inner_sea_gods*' -name '*races*'

    Nor is a book's own root the ONLY place a re-attributed unit's file can be:
    if `name` is absent from `root` entirely, this also tries
    `core_essentials`'s directory (`_CORE_ESSENTIALS_DIR`) -- the one other
    place `decisions.md §9` re-attribution can have left a unit's real file --
    unless `root` already IS that directory (no self-fallback: a file genuinely
    absent from `core_essentials` stays absent, not silently re-searched into a
    confusing duplicate-candidate error).

    Both misses would have been LOUD (``FileNotFoundError``/``SystemExit``), not
    silent, which is why this is a widening rather than a correction.

    Two failure modes are refused rather than resolved:

    * **Not found anywhere** -- the inventory cites a file this book (nor, for
      a re-attributed unit, `core_essentials`) does not have, so every citation
      derived from it would be fiction.
    * **Found in more than one place** -- a bare basename that matches two real
      files does not identify a row, and picking either one is a coin flip on
      which rules text ships. The book's own root always wins over the
      `core_essentials` fallback when a name collides in both (checked as two
      separate passes, never one merged walk) so a book that legitimately owns
      a same-named file is never redirected to `core_essentials`'s copy.
    """
    candidates = _find_under(root, name)
    if not candidates:
        ce_root = os.path.join(corpus_root(), _CORE_ESSENTIALS_DIR)
        if os.path.abspath(ce_root) != os.path.abspath(root):
            candidates = _find_under(ce_root, name)
    if not candidates:
        raise SystemExit(f"{name} is not present anywhere under {root}")
    if len(candidates) > 1:
        raise SystemExit(
            f"{name} resolves to {len(candidates)} files under {root} "
            f"({', '.join(sorted(candidates))}) -- a bare basename that names "
            "two real files does not identify a row"
        )
    return candidates[0]


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


def parse_stat_adjustments(row: list[str]) -> list[tuple[str, int]]:
    """`BONUS:STAT|DEX,WIS|4` -> [("DEX", 4), ("WIS", 4)].

    Identical parse to `scripts/transcribe_companion_tables.py`'s function of
    the same name -- the two chassis kinds carry the same PCGen token, and
    `monster_chassis::StatAdjustment` (the Rust side) IS
    `companion_chassis::StatAdjustment`, reused rather than duplicated
    (SD31-E6-F1-002).

    A multi-ability token is split into one record each, which is what PCGen
    itself does with it. A token whose amount is not an integer literal (a
    formula, e.g. `BONUS:STAT|STR|MutagenicMaulerMutagenStatBonus`) is
    **skipped**, not guessed: this program has no formula interpreter
    (`decisions.md §24`) and a wrong number in an ability column is worse than
    an absent one. This is deliberately an ADJUSTMENT, never a final ability
    score -- PCGen computes the real score at runtime from a base template
    this ingest does not carry, so serving anything labelled "Strength" here
    would be the quieter lie (`companion_chassis::StatAdjustment`'s own doc
    comment, and `OPEN-ISSUES.md` row 26's structural finding: Demon (Balor)'s
    `BONUS:STAT|STR|24` is a DELTA against a base this book's own row never
    states).
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


def parse_has_spell_like_abilities(row: list[str]) -> bool:
    """Whether the row carries a `BONUS:VAR|SLA_CL|<...>` token -- PCGen's
    encoding of PF1's "Spell-Like Abilities" universal monster rule (caster
    level = Hit Dice, or an arithmetic wrapper of it).

    A presence check only (SD31-E6-F1-002, `OPEN-ISSUES.md` row 44), and
    **not** the more general `SPELLS:` token: TDD red/green anchor --
    Linnorm (Crag) (`b1_races.lst:269`) carries `BONUS:VAR|SLA_CL|HD` and its
    spell-like effects (`True Seeing ~ Constant`) reach the row only through
    an `ABILITY:` cross-reference, with NO `SPELLS:` token anywhere on the
    line at all; gating on `SPELLS:` would have wrongly answered `False` for
    one of this seam's own 7 already-committed fixtures. Every one of those 7
    fixtures' `corpus_field` is exactly `BONUS:VAR|SLA_CL|HD`, which is the
    signal this function keys on.
    """
    return any("BONUS:VAR|SLA_CL|" in field for field in row)


def parse_sla_cl_token(row: list[str]) -> str | None:
    """The trailing value of the row's own `BONUS:VAR|SLA_CL|<value>` field,
    verbatim -- `None` when the row carries none (SD31-E6-F9-003).

    A row may state the generic Universal Monster Rule (`HD` or the
    equivalent `max(TL,1)`/`(max(TL,1))`) or a monster-specific literal
    override (Couatl: `9`, against 12 Hit Dice) -- both are corpus fact, and
    this function transcribes whichever the row states without judging
    between them; `derived_evaluator_fixture_check::spell_like_ability_
    caster_level` is what applies the rule.

    A field carrying a further pipe segment after the value (e.g.
    `BONUS:VAR|SLA_CL|2|PREABILITY:...`) is a conditional/feat-granted
    ADDITION on top of a base value stated elsewhere on the row, not the base
    rule itself -- excluded, same as a row carrying the token more than once,
    both left `None` rather than guessed. Neither shape has been observed on
    any row this script's own book roster carries as of this pass; refusing
    outright is cheaper than silently picking the wrong one of two numbers if
    a future book ever does.
    """
    tokens = [field for field in row if field.startswith("BONUS:VAR|SLA_CL|")]
    if len(tokens) != 1:
        return None
    value = tokens[0][len("BONUS:VAR|SLA_CL|") :]
    if "|" in value:
        return None
    return value.strip() or None


def parse_spell_like_abilities(row: list[str]) -> list[tuple]:
    """Every spell the row grants as a spell-like ability, one tuple per SPELL,
    read from the row's `SPELLS:` tokens.

    PCGen shape::

        SPELLS:<label>|TIMES=<n>|[TIMEUNIT=<unit>|]CASTERLEVEL=<v>|<spell>[,<dc>]|<spell>[,<dc>]

    A single token routinely grants several spells sharing one label,
    frequency and caster level, so a token expands to N tuples, not one.

    Returns ``(label, times, time_unit, caster_level, spell, save_dc)`` with
    every element a verbatim substring of the row and ``None`` for a segment
    the row does not carry.  Nothing is computed: the ``,<dc>`` tail is split
    off the spell name and handed on unparsed, and
    ``derived_evaluator_fixture_check::spell_like_ability_save_dc`` is what
    applies PF1's universal monster rule to it.

    A segment carrying a `PRE`-guard or an unrecognised `<TAG>=` pair is
    skipped rather than mistaken for a spell name -- guards are conditions,
    never grants, and a tag this parser does not know is not a spell.
    """
    known_tags = ("TIMES=", "TIMEUNIT=", "CASTERLEVEL=", "SPELLBOOK=", "DC=",
                  "DCBASE=", "CASTERLEVELFORMULA=")
    out: list[tuple] = []
    for field in row:
        if not field.startswith("SPELLS:"):
            continue
        segments = field[len("SPELLS:"):].split("|")
        if not segments:
            continue
        label = segments[0].strip()
        if not label:
            continue
        times = time_unit = caster_level = None
        spells: list[tuple[str, str | None]] = []
        for segment in segments[1:]:
            segment = segment.strip()
            if not segment:
                continue
            if segment.startswith("TIMES="):
                times = segment[len("TIMES="):].strip() or None
                continue
            if segment.startswith("TIMEUNIT="):
                time_unit = segment[len("TIMEUNIT="):].strip() or None
                continue
            if segment.startswith("CASTERLEVEL="):
                caster_level = segment[len("CASTERLEVEL="):].strip() or None
                continue
            if segment.startswith("PRE") or segment.startswith("!PRE"):
                continue
            if any(segment.startswith(tag) for tag in known_tags):
                continue
            if "=" in segment.split(",")[0]:
                # An unrecognised `<TAG>=<value>` pair. Skipping is the honest
                # reading: it is certainly not a spell name, and guessing which
                # tag it is would put a fabricated value on the record.
                continue
            name, comma, dc = segment.partition(",")
            name = name.strip()
            if not name:
                continue
            spells.append((name, dc.strip() or None if comma else None))
        for name, dc in spells:
            out.append((label, times, time_unit, caster_level, name, dc))
    return out


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


# ---------------------------------------------------------------------------
# The `CATEGORY:Internal` bundle-row hop (SD-29 `decisions.md §62.4`, round
# 10's `scripts/scan_monster_ability_bundle_rows.py`, sized corpus-wide at
# 235 units but never wired into this transcriber's own ownership pass).
#
# A monster row may name its abilities INDIRECTLY, through a bundle row::
#
#     support/isg_races_b4.lst:6            The First Blade
#         ABILITY:Internal|AUTOMATIC|Race Traits ~ First Blade
#     support/isg_abilities_races_b4.lst:8  Race Traits ~ First Blade  CATEGORY:Internal
#         ABILITY:Special Ability|AUTOMATIC|…|First Blade ~ Powerful Blows (Slam)|…
#
# `parse_special_ability_refs` and the `<Monster> ~ <Ability>` namespace
# prefix (both above) never see this: the monster row's own `ABILITY:`
# token is `Internal`, not `Special Ability`, and the real ability's
# namespace (`First Blade ~ …`) does not match the monster's own KEY (`The
# First Blade`). Two functions, mirroring `scan_monster_ability_bundle_rows.
# py`'s already-proven regex shapes exactly rather than re-deriving them, so
# the 235-unit count that script already validated against the live oracle
# is what this pass reproduces, not a new, independently-fallible read of
# the same tokens.
_INTERNAL_BUNDLE_REF = re.compile(r"ABILITY:Internal\|AUTOMATIC\|([^\t|]*)")
_CATEGORY_EQUALS_PREFIX = re.compile(r"^CATEGORY=[^|]*\|")


def parse_internal_bundle_refs(row: list[str]) -> list[str]:
    """Bundle keys named by the row's `ABILITY:Internal|AUTOMATIC|…` tokens.

    The SAME token also names bare (non-namespaced) natural-attack cross
    references (`parse_natural_attacks`'s own doc comment: `ABILITY:Internal|
    AUTOMATIC|Bite`) -- this function does not try to tell the two apart by
    shape; it returns every named entry, and [`transcribe`]'s caller only
    credits an entry that turns out to resolve to a real `CATEGORY:Internal`
    bundle ROW in this book's own ability files, which a bare attack name
    never does. An entry that resolves to nothing is silently not a bundle
    ref, exactly as it already silently was before this function existed.
    """
    refs: list[str] = []
    for field in row:
        if not field.startswith("ABILITY:Internal|AUTOMATIC|"):
            continue
        for name in field.split("|")[2:]:
            name = name.strip()
            if not name or is_prerequisite(name):
                continue
            if name not in refs:
                refs.append(name)
    return refs


def find_internal_bundle_ability_refs(
    ability_file_paths: list[str], bundle_keys: set[str]
) -> dict[str, list[str]]:
    """`bundle_key -> [ability_key, ...]`, read from every `CATEGORY:Internal`
    row in `ability_file_paths` whose own leading field (`KEY:`-stripped
    the same way a `.MOD` row's target is: a leading `CATEGORY=<x>|` prefix
    removed, then everything before a trailing `.MOD` dropped) is one of
    `bundle_keys`. Byte-identical selection logic to `scan_monster_ability_
    bundle_rows.py::scan_book`'s own bundle-definition-row match, kept as a
    second, independent typing of the same already-proven regexes rather
    than imported, so a bug in one does not silently become a bug in both --
    the discipline `resolve_book_file`'s own docstring calls "a second
    spelling" for the same reason.
    """
    result: dict[str, list[str]] = {}
    for path in ability_file_paths:
        with open(path, encoding="utf-8", errors="replace") as handle:
            for line in handle:
                if not line.strip():
                    continue
                first_column = line.split("\t", 1)[0]
                key = _CATEGORY_EQUALS_PREFIX.sub("", first_column).split(".MOD")[0].strip()
                if key not in bundle_keys:
                    continue
                bucket = result.setdefault(key, [])
                for token_text in re.findall(r"ABILITY:[^\t\n]*", line):
                    for part in token_text.split("|")[2:]:
                        part = part.strip()
                        if not part or is_prerequisite(part):
                            continue
                        if part not in bucket:
                            bucket.append(part)
    return result


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


def _concat_desc_variants(descs: list[str]) -> tuple[str, list[str]]:
    """The generalised SIXTH `parse_desc` shape (`decisions.md §27b`, round
    6/7/8's own docstring naming this exact gap): concatenate every `DESC:`
    token's own text, verbatim, in the row's own order.

    Every branch above this one resolves a row where the corpus states ONE
    global criterion that picks a single winning token (the `DisplayFullAbility`
    ruleset toggle, a literal-superset containment, or a lone token whose pipe
    entries name this row's own `DEFINE:`d variables). The **56**-unit
    `PRERULE`/`PREVAREQ`/`PREVARGT`/`PRESIZE*`/`PREHD`/`PRERACE`/`PRETEMPLATE`/
    `PREABILITY`-gated group this branch closes is different: its gate tests a
    property of the *owning monster instance* (its CR, HD, size, template,
    race subtype, or a feat it has) -- a fact this per-ability-KEY table row,
    shared verbatim across every monster that owns it, cannot resolve once and
    for all. There is no single row-level "the" value to trace for these
    gates (unlike a row's own unconditionally-set `BONUS:VAR`, which the
    `PREVAREQ:EnergyDrainNoHP,0`-style rows below already resolve before ever
    reaching here -- see `parse_type_or_provisional_default`'s sibling
    resolution for the analogous `TYPE:` gap). Picking ONE variant here would
    be exactly the guess `§1a` forbids; omitting the gated ones would silently
    drop mechanics the way the original single-`DESC:`-only parser did.

    So every token's text ships, concatenated in the corpus's own order --
    the same "verbatim corpus text, corpus's own order, never a composition"
    principle round 7's CONTINUATION shape already established, generalised
    to also carry a token's own PRE-gate condition (dropped from the emitted
    text -- gates are not player-facing prose) and its own `%N` variables
    alongside the plain, ungated continuation case CONTINUATION already
    covers. A single space joins adjacent tokens' text (the corpus supplies
    every WORD; a plain ASCII space between two already-punctuated sentences
    is formatting hygiene, not invented content -- the alternative, gluing
    two sentences together with no separator at all, is the actual defect).

    Each token's own `%N` placeholders are renumbered so a single, ordered,
    GLOBAL `description_variables` list can back them: token 2's own `%1`
    (which names token 2's own first pipe-declared variable, NOT token 1's)
    becomes `%(N+1)` where `N` is the count of variables already collected
    from every earlier token. This is pure bookkeeping -- the renumbered text
    still names exactly the variables the corpus's own pipe entries declared,
    in the corpus's own order, nothing added or guessed.
    """
    joined_parts: list[str] = []
    global_vars: list[str] = []
    for d in descs:
        segments = d.split("|")
        text = segments[0]
        row_vars = [p for p in segments[1:] if p and not is_prerequisite(p)]
        if row_vars:
            offset = len(global_vars)
            text = re.sub(
                r"%(\d+)",
                lambda m, _offset=offset: f"%{_offset + int(m.group(1))}",
                text,
            )
            global_vars.extend(row_vars)
        joined_parts.append(text)
    return " ".join(joined_parts), global_vars


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

    **CORRECTION (`SD31-E6-F9-002`, `OPEN-ISSUES.md` row 151): the "not one of
    the 14 ships" claim above was true only because `core_essentials`-origin
    rows were UNREACHABLE at all (`resolve_book_file` raised before any of
    them reached this function).** Once that was fixed, 5 more rows in this
    same refused shape turned out to have a real owning monster: `bestiary`'s
    `ce_abilities_race.lst:1359`/`:1363`/`:1516` (`Energy Drain`/`Fast
    Healing`/`Stench`) and `bestiary_2`'s `ce_abilities_race.lst:1955`/`:2043`
    (`Telepathy ~ Miles`/`Voidworm ~ Change Shape`). All 5 share a FIFTH
    shape this docstring never enumerated: several `DESC:` tokens gated on a
    `PREVAREQ`/`PREVARGT` comparison against a `BONUS:VAR`-set value (e.g.
    singular vs. plural phrasing keyed to whether a variable equals 1 or
    exceeds it), not the on/off `PRERULE:1,DisplayFullAbility` toggle the
    "gated-full" branch above resolves. Still refused, deliberately -- picking
    the right variant needs the value each row's own `BONUS:VAR` sets,
    verified per record, not guessed generically.
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
                    return _concat_desc_variants(descs)
        assert len(descs) == 1, "every branch above narrows to exactly one token"
    parts = descs[0].split("|")
    return parts[0], [p for p in parts[1:] if p and not is_prerequisite(p)]


def type_segments(row: list[str]) -> list[str]:
    """Every dot-separated segment across EVERY `TYPE:` token on the row, in
    field order.

    A row can carry more than one `TYPE:` token — `bestiary_3`'s dragon
    subtypes state `TYPE:Supernatural` and `TYPE:RaceAbility.SpecialQuality`
    as two separate fields on the same line (`Forest Dragon ~ Change Shape`
    and 26 more). `token()` returns only the first field with a given prefix,
    which silently discarded the SECOND token's facet for those 27 rows
    before this existed — not a vocabulary gap, a parsing bug, found and
    fixed by the T9 `MonsterAbilityFacet`-widening cycle while deriving the
    real refusal population (`decisions.md §17a`: re-derive, don't trust).

    **`decisions.md §22` — two upstream shapes, inherited and resolved here,
    not perpetuated.** Re-derived live against the round-6 refusal
    population (`no_record` monster_ability round 6), previously named only
    as "2 corpus typos and a comma-delimiter anomaly" with no fix landed:

    1. **Comma-delimiter anomaly** — `bestiary`'s `b1_abilities_race.lst:1138`
       (`Spectre ~ Create Spawn`) states `TYPE:SpecialAttack,Supernatural`:
       PCGen's own delimiter is `.`, and every other row in every book this
       script has ever read uses it; this single row uses `,` instead. Two
       facet-bearing rows cannot both be right about the delimiter their own
       shared vocabulary uses, so this is exactly `§22`'s "two rows that
       cannot both be right" — Codex resolves it by treating `,` as an
       additional segment separator, corpus-wide, rather than mirroring the
       one row's typo.
    2. **Misspelled facet/delivery segments** — `bestiary_2`'s
       `b2_abilities_race.lst:1259` (`Tick Swarm ~ Cling`) states
       `TYPE:SpecialAttck.Extraordinary` (missing the `a`), and
       `b2_abilities_race.lst:851` (`Mothman ~ Agent of Fate`) states
       `TYPE:Spelllike` (missing the capital `L`) where every other book's
       equivalent field reads `SpellLike`. Both are single-row spelling
       defects in the oracle's own data, corrected here by an explicit,
       named substitution table (`_TYPE_SEGMENT_TYPO_FOLDS` below) — never a
       fuzzy/heuristic match, so no *other* segment can ever be silently
       "corrected" into a different vocabulary word.

    Both corrections are applied only inside this function, before facet/
    delivery classification, so `parse_type`'s own vocabulary
    (`FACETS`/`DELIVERIES`) never has to special-case either shape.
    """
    segments: list[str] = []
    for field in row:
        if field.startswith("TYPE:"):
            for raw in field[len("TYPE:") :].split("."):
                for part in raw.split(","):
                    part = _TYPE_SEGMENT_TYPO_FOLDS.get(part, part)
                    if part:
                        segments.append(part)
    return segments


# `decisions.md §22` — named, single-row corrections for confirmed upstream
# spelling defects (never a fuzzy match). Adding an entry here is a
# licensing/correctness-relevant divergence from the oracle's own bytes:
# name the exact row it was found on in a comment, the way the two entries
# below do.
_TYPE_SEGMENT_TYPO_FOLDS: dict[str, str] = {
    # `bestiary_2/b2_abilities_race.lst:1259` (`Tick Swarm ~ Cling`).
    "SpecialAttck": "SpecialAttack",
    # `bestiary_2/b2_abilities_race.lst:851` (`Mothman ~ Agent of Fate`).
    "Spelllike": "SpellLike",
}


class UnmodelledFacet(Exception):
    """A row's `TYPE:` segments name no facet the chassis models.

    Raised rather than exiting so the caller can decide whether the row
    matters — same fix, same reason, as `UnmodelledDesc`
    (`SD31-E6-F9-005`'s doc comment above `unscreenable_shipping`): a row
    this cannot resolve is fine to drop, but must never crash every OTHER
    row in the same book. Before this existed, `parse_type` raised
    `SystemExit` directly and stopped `bestiary`/`bestiary_2`/`bestiary_3`/
    `inner_sea_bestiary`/`inner_sea_gods` from transcribing ANY ability row
    at all, not just the ones carrying an unmodelled shape — the identical
    defect class, found again in the same script.
    """


def parse_type(row: list[str]) -> tuple[str, str | None, list[str]]:
    """`TYPE:SpecialAttack.Supernatural.Aura` -> facet, delivery, traits."""
    segments = type_segments(row)
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
        raise UnmodelledFacet(
            f"row carries no `monster_ability` facet in TYPE segments {segments!r} — the "
            "chassis models SpecialAttack/SpecialQuality/Weakness/Defensive/Aura/Sense/"
            "Communicate only; widen it deliberately"
        )
    return facet, delivery, traits


# `decisions.md §27`'s provisional default, unblocked by the operator ruling
# (T9 round 6 escalated this exact population and refused to invent a
# default unilaterally; §27 grants it, conditioned on every defaulted unit
# carrying a machine-countable marker naming WHY -- `workflow-instruction.md
# §6a`'s contract, enforced by `scripts/shape_provisional_marker.py`, the
# ONLY sanctioned place that writes the marker fields).
PROVISIONAL_FACET_DEFAULT = "SpecialQuality"


# `decisions.md §27a`/`§27b` — kanban.md row 17's final categorization pass
# (`epic-7-shape-categorization-100`). Every row `provisional_facet_reason`
# would otherwise have to default (per-book/`§27`) has now been individually
# re-derived against the corpus/oracle, keyed by the row's own `KEY:` value.
# A row with an entry here NEVER goes through the `§27` provisional-default
# path: `parse_type` returns this facet directly, exactly as if the row's
# own `TYPE:` segments had declared it (`§27a`: "F0/no-formula reached by
# fallthrough is not an answer... derived by measurement or it is not
# done" — the same standard applies to a facet default). Each entry cites
# the corpus/oracle evidence it rests on so a future reader can re-verify
# it without re-deriving from scratch. Never a guess; where genuinely
# ambiguous, the majority-convention reading is used and named as such.
#
# Group 1 — reclassified to `SpecialAttack` (the `§27` `SpecialQuality`
# default was wrong for these four; corroborated by a genuinely-declared
# sibling record elsewhere in the corpus, not by domain recall):
#   * "Aurumvorax ~ Rake" — the universal monster rule "Rake" is
#     `SpecialAttack` corpus-wide and unanimously: the base rule record
#     itself (`data/corpus/beastiary/monster_ability/rake.json`,
#     `TYPE:SpecialAttack.Extraordinary.AttackOption`) plus every other
#     book's own "~ Rake" row (`gynosphinx_rake.json`,
#     `bandersnatch_rake.json`) all genuinely declare `SpecialAttack`.
#   * "Bunyip ~ Blood Rage" — same shape: the universal rule's own base
#     record (`data/corpus/bestiary_2/monster_ability/blood_rage.json`,
#     `TYPE:SpecialAttack.Extraordinary`) and `inner_sea_bestiary`'s
#     `volnagur_blood_rage.json` both genuinely declare `SpecialAttack`.
#   * "Yrthak ~ Sonic Lance" — the SAME creature's sibling ability
#     `Yrthak ~ Explosion` (`b2_abilities_race.lst:1416`) genuinely
#     declares `TYPE:SpecialAttack.Extraordinary` and its own `DESC:`
#     names the identical mechanic ("a yrthak can fire its sonic lance at
#     the ground...") — the two rows describe one ability from two angles.
#   * "Howler ~ Abyssal Strike" — identical shape ("natural weapons treated
#     as aligned for the purpose of overcoming damage reduction") to
#     `inner_sea_world_guide`'s genuinely-declared
#     `nascent_demon_lord_aligned_strike.json` (`SpecialAttack`); the same
#     creature's siblings `Howl`/`Pain`
#     (`b2_abilities_race.lst:696`-`:697`) are also both `SpecialAttack`.
#
# Group 2 — confirmed `SpecialQuality` (the `§27` default was already the
# genuinely-correct answer; marker removed because it is now a measurement,
# not a placeholder):
#   * "Adlet ~ Spell-Like Abilities", "Lorthact ~ Spell-Like Abilities",
#     "Mothman ~ Agent of Fate" — an unqualified "Spell-Like Abilities" row
#     (no facet segment declared) is `SpecialQuality` corpus-wide by a
#     large majority: of the 277 genuinely-declared (non-provisional)
#     "Spell-Like Abilities" `monster_ability` records in the corpus, 255
#     declare `SpecialQuality.SpellLike` against 22 `SpecialAttack.
#     SpellLike` — the majority-convention reading, named as such.
#   * "Denizen of Leng ~ Planar Fast Healing" — Fast Healing is a passive
#     defensive trait, not an attack; this is `decisions.md §27`'s own
#     cited example (`ModifyHP.Supernatural`) and the corpus's own
#     genuinely-declared Fast-Healing-shaped records agree.
#   * "Xocothian ~ Speed Burst" — a self-only movement ability usable as a
#     full-round action; not an attack, so `SpecialQuality` by exclusion
#     among the seven modeled facets.
#   * "Carnivorous Blob ~ Split" — the universal monster rule "Split" is
#     `SpecialQuality` corpus-wide: 4 of 5 genuinely-declared "~ Split"
#     records (`carnivorous_crystal_split.json`, `plasma_ooze_split.json`,
#     `black_pudding_split.json`, `ocher_jelly_split.json`) declare
#     `SpecialQuality`; the fifth (`amphisbaena_split.json`) declares
#     `Defensive`, a distinguishable creature-specific variant, not a
#     counter-example to this row.
#   * "Lamia Matriarch ~ Spells", "Royal Naga ~ Spells", "Water Naga ~
#     Spells", "Lunar Naga ~ Spells" — a bare "casts spells as an Nth-level
#     sorcerer" racial spellcasting grant; none of the other six modeled
#     facets fit a passive granted capability, so `SpecialQuality` by
#     exclusion (matches the three siblings' own shared shape).
#   * "Asurendra ~ None" — a content-less placeholder row (no `DESC:`,
#     `TYPE:AsurendraAdditional` alone) sitting among sibling
#     `AsurendraAdditional`-tagged rows (`Death`/`Sacrilege`/`Shaping`) that
#     all genuinely declare `SpecialQuality`; `SpecialQuality` by structural
#     analogy to those siblings.
#   * "Unfettered Eidolon ~ Con/Str/Wis/Dex/Cha/Int" (6 rows) — a flat
#     `BONUS:STAT` ability-score-choice row (`CHOOSE:NOCHOICE`); none of
#     the other six modeled facets describe a stat bonus, so
#     `SpecialQuality` by exclusion.
#   * "Petrified Maiden ~ Weapon Selection" — a granted weapon-proficiency
#     choice (`CHOOSE:WEAPONPROFICIENCY`), the same shape as the eidolon
#     stat-selection rows above; `SpecialQuality` by exclusion.
#   * "Morlock ~ Sneak Attack" — an invisible (`VISIBLE:NO`) internal
#     numeric feed (`BONUS:VAR|SneakAttackDice|1`), `TYPE:Internal` (round
#     6's own "genuinely novel shape" — no other genuinely-declared
#     `monster_ability` record anywhere in the corpus carries the
#     `Internal` trait to compare against). None of `SpecialAttack`/
#     `Weakness`/`Defensive`/`Aura`/`Sense`/`Communicate` describe a hidden
#     numeric feed either, so `SpecialQuality` is the only fit within the
#     seven modeled facets, matching the row's own `CATEGORY:Special
#     Ability` declaration.
_MONSTER_ABILITY_FACET_OVERRIDES: dict[str, str] = {
    "Aurumvorax ~ Rake": "SpecialAttack",
    "Bunyip ~ Blood Rage": "SpecialAttack",
    "Yrthak ~ Sonic Lance": "SpecialAttack",
    "Howler ~ Abyssal Strike": "SpecialAttack",
    "Adlet ~ Spell-Like Abilities": "SpecialQuality",
    "Lorthact ~ Spell-Like Abilities": "SpecialQuality",
    "Mothman ~ Agent of Fate": "SpecialQuality",
    "Denizen of Leng ~ Planar Fast Healing": "SpecialQuality",
    "Xocothian ~ Speed Burst": "SpecialQuality",
    "Carnivorous Blob ~ Split": "SpecialQuality",
    "Lamia Matriarch ~ Spells": "SpecialQuality",
    "Royal Naga ~ Spells": "SpecialQuality",
    "Water Naga ~ Spells": "SpecialQuality",
    "Lunar Naga ~ Spells": "SpecialQuality",
    "Asurendra ~ None": "SpecialQuality",
    "Unfettered Eidolon ~ Con": "SpecialQuality",
    "Unfettered Eidolon ~ Str": "SpecialQuality",
    "Unfettered Eidolon ~ Wis": "SpecialQuality",
    "Unfettered Eidolon ~ Dex": "SpecialQuality",
    "Unfettered Eidolon ~ Cha": "SpecialQuality",
    "Unfettered Eidolon ~ Int": "SpecialQuality",
    "Petrified Maiden ~ Weapon Selection": "SpecialQuality",
    "Morlock ~ Sneak Attack": "SpecialQuality",
}


def provisional_facet_reason(row: list[str]) -> str:
    """Classify WHY a row's `TYPE:` segments name no modeled facet, for the
    `§27` provisional-default marker. Never guesses a real facet -- only
    names which of four shapes T9 round 6/7's own re-derivation found in
    this population, so the eventual `row 17` real-categorization pass
    (`§27a`) starts from a labeled bucket rather than one undifferentiated
    pile. Four shapes, corpus-wide re-derived (`§17a`), not four guesses:

    * **`copy_row_base_ability_type_unresolved`** -- the row carries NO
      `TYPE:` token at all because its identity field is a `.COPY=` overlay
      (`CATEGORY=Special Ability|Rake.COPY=Rake`), and the bare-named base
      ability (`Rake`, unqualified) it copies from does not exist as its
      own row in ANY book this script reads -- confirmed by a corpus-wide
      search, not assumed absent.
    * **`missing_type_token_no_facet`** -- the row carries no `TYPE:` token
      and is not a `.COPY=` row either (`Lamia Matriarch ~ Spells`: a
      `CATEGORY:`/`DESC:` row PCGen itself never gave a `TYPE:`).
    * **`type_internal_only_no_facet_no_delivery`** -- the row's ONLY
      segment is `Internal` (PCGen's own `CATEGORY:Internal` bundle-row
      marker, not a delivery or a facet), naming a hidden bonus-granter a
      player never sees (`VISIBLE:NO`), round 6's own "genuinely novel
      shape".
    * **`delivery_only_no_facet_segment`** -- the row states HOW the
      ability is delivered (`SpellLike`, `Extraordinary`, `Supernatural`)
      but never states WHAT facet it is -- exactly `decisions.md §27`'s own
      cited example (`ModifyHP.Supernatural`).
    * **`book_specific_type_label_no_facet_vocabulary_gap`** -- none of the
      above: a genuine book-specific one-off `TYPE:` string
      (`AsurendraAdditional`, `Unfettered Eidolon Stat Selection`, …) that
      would need its own per-record policy call to assign a real facet,
      round 6's own residual bucket.
    """
    segments = type_segments(row)
    if not segments:
        if ".COPY=" in row[0]:
            return "copy_row_base_ability_type_unresolved"
        return "missing_type_token_no_facet"
    if segments == ["Internal"]:
        return "type_internal_only_no_facet_no_delivery"
    if any(segment in DELIVERIES for segment in segments):
        return "delivery_only_no_facet_segment"
    return "book_specific_type_label_no_facet_vocabulary_gap"


def parse_type_or_provisional_default(
    row: list[str],
) -> tuple[str, str | None, list[str], str | None]:
    """`parse_type`, widened by `decisions.md §27`'s provisional default.

    A row that genuinely declares a modeled facet returns exactly what
    `parse_type` would, with a `None` fourth value -- this function changes
    NOTHING for the ~96% of rows that already resolve cleanly. A row with
    no modeled facet ships anyway (`facet` forced to
    `PROVISIONAL_FACET_DEFAULT`, `delivery`/`traits` read off the row's own
    segments exactly as `parse_type` would have, had it not raised) and the
    fourth value names why, via `provisional_facet_reason`.

    This function only classifies and returns the facet/reason -- it never
    touches a corpus record. The caller stamps the returned reason onto the
    shipped JSON record via `scripts/shape_provisional_marker.py`'s
    `stamp_provisional_default`, the only sanctioned place that writes the
    marker (`workflow-instruction.md §6a`).

    `decisions.md §27a`/`§27b` (kanban.md row 17): a row whose `KEY:` value
    matches `_MONSTER_ABILITY_FACET_OVERRIDES` is a genuinely-derived
    answer, not a placeholder -- it is returned with a `None` fourth value
    (never provisional) exactly like a row whose own `TYPE:` segments
    resolved cleanly, even though its segments alone could not resolve it.
    """
    try:
        facet, delivery, traits = parse_type(row)
        return facet, delivery, traits, None
    except UnmodelledFacet:
        segments = type_segments(row)
        delivery = next((segment for segment in segments if segment in DELIVERIES), None)
        traits = [segment for segment in segments if segment != delivery]
        key = token(row, "KEY:")
        override = _MONSTER_ABILITY_FACET_OVERRIDES.get(key) if key else None
        if override is not None:
            return override, delivery, traits, None
        return PROVISIONAL_FACET_DEFAULT, delivery, traits, provisional_facet_reason(row)


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


def transcribe(book: str, provisional_facets: dict[str, str] | None = None) -> str:
    """`provisional_facets`, if given, is filled IN PLACE with
    `{corpus_key: reason}` for every ability row this call defaults via
    `decisions.md §27` (see `parse_type_or_provisional_default`). Optional
    and defaults to a throwaway dict so every existing caller's
    `transcribe(book) -> str` signature is unchanged -- only `write_book`
    passes a real dict, to hand the population to the stamping step."""
    if provisional_facets is None:
        provisional_facets = {}
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
    internal_bundle_refs: dict[str, list[str]] = {}
    for unit in monsters:
        row = read_row(resolve_book_file(root, unit["source_file"]), unit["source_line"])
        monster_rows[unit["corpus_key"]] = row
        named = parse_special_ability_refs(row)
        mine = [n for n in named if n in ability_keys]
        monster_ability_keys[unit["corpus_key"]] = mine
        external[unit["corpus_key"]] = [n for n in named if n not in ability_keys]
        for key in mine:
            owners[key].append(unit["corpus_key"])
        bundle_refs = parse_internal_bundle_refs(row)
        if bundle_refs:
            internal_bundle_refs[unit["corpus_key"]] = bundle_refs
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

    # ---- Third ownership pass: the `CATEGORY:Internal` bundle-row hop ----
    #
    # Deliberately AFTER the two direct passes above (whose ordering nothing
    # here changes) and BEFORE the Product Identity screen (a bundle-reached
    # ability must be screened exactly like a directly-owned one, not
    # exempted by running later). Resolved corpus-wide, not per-monster: two
    # monsters can legitimately name the same bundle key.
    if internal_bundle_refs:
        bundle_keys = {ref for refs in internal_bundle_refs.values() for ref in refs}
        ability_file_paths = sorted(
            {resolve_book_file(root, name) for name in {u["source_file"] for u in abilities}}
        )
        bundle_defs = find_internal_bundle_ability_refs(ability_file_paths, bundle_keys)
        # Iterated in monster-row order (never over a dict/set), for the same
        # run-to-run determinism reason the prefix pass above is.
        for unit in monsters:
            for bundle_key in internal_bundle_refs.get(unit["corpus_key"], []):
                for ability_key in bundle_defs.get(bundle_key, []):
                    if ability_key not in ability_keys:
                        continue
                    if unit["corpus_key"] not in owners[ability_key]:
                        owners[ability_key].append(unit["corpus_key"])
                        monster_ability_keys[unit["corpus_key"]].append(ability_key)

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
        # No `DESCISPI:YES` check here, deliberately: `MonsterStatBlock`
        # (`monster_chassis.rs`) carries no free-text description field at
        # all -- a monster row's own `DESC:` token, if it has one, is never
        # read or emitted by this half of the transcriber (only
        # `ability_pi_reason` below calls `parse_desc`). A declaration on a
        # field this table never serializes has nothing to redact, so there
        # is no silent gap here to close -- verified by reading
        # `MonsterStatBlock`'s own field list, not assumed
        # (`SD30-E3-F3-001`).
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
            # Every value the `spell_like_abilities` field emits. Added with
            # the field itself (SD31-W15-MONSTER-SLA-001): a spell name, a
            # spell-book label or a caster-level variable name is emitted text
            # like any other, and `pi_table_sweep` rejects a Product Identity
            # term anywhere under `rules_tables/` regardless of which field it
            # sits in. Omitting these would have opened exactly the silent gap
            # the `DESCISPI:` comment above exists to say is NOT open.
            *(v for sla in parse_spell_like_abilities(row) for v in sla if v),
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

    # Ability rows whose `TYPE:` segments name no facet the chassis models
    # USED to be dropped here (`UnmodelledFacet`, `SD31-E6-F9-005`'s fix).
    # `decisions.md §27` now grants a provisional default instead -- see
    # `parse_type_or_provisional_default` -- so this population ships.
    # Populated here, in `ability_pi_reason`'s own pre-pass (which already
    # runs once per ability, before the header block below is written), not
    # in the emission loop -- the header needs the final count and the
    # emission loop runs after it. The caller's dict (already substituted
    # for `None` at the top of this function) is mutated IN PLACE, never
    # rebound, so a caller-supplied dict (`write_book`) stays valid.

    # Ability rows whose `DESC:` text is declared Product Identity
    # (`DESCISPI:YES`) OR whose description text carries an undeclared
    # blacklist-term hit -- either way the ROW is not otherwise dropped:
    # these ship, with `description` (and its variables) replaced by
    # `REDACTED_PI_MARKER` at emission time below, mirroring
    # `ingest_race_traits.rs`/`ingest_pu_classes.rs`'s "a description CAN be
    # redacted and the record still works" rule (`decisions.md §39.4`). A row
    # whose NAME carries PI is renamed, not dropped, either (see
    # `name_renamed` below) -- a dropped row exists only when a hit lands
    # in a field neither mechanism can fix.
    desc_redacted: set[str] = set()

    # Ability rows whose bare NAME (an emitted value, not a PCGen
    # declaration) hits the blacklist term list -- `decisions.md §24`'s "the
    # name itself is PI" case. Value is `(codex_name, codex_key)`, both
    # derived ONLY from `(kind, book, source_file, source_line)` via
    # `scripts/codex_neutral_name.py` -- see that module's own docstring for
    # the `§24b`-1 proof this cannot be influenced by the original name.
    # Deliberately narrower than PCGen's OWN `NAMEISPI:YES` declaration
    # (still handled by the early-return branch below, unchanged: dropped,
    # not renamed) -- this branch only fires when the ROW ITSELF never
    # declared its name PI and the term scan found it anyway, which is
    # exactly the population T9 round 6 named and this cycle closes.
    name_renamed: dict[str, tuple[str, str]] = {}
    renamed_divergence: list[dict] = []

    def ability_pi_reason(unit: dict) -> str | None:
        row = read_row(resolve_book_file(root, unit["source_file"]), unit["source_line"])
        if token(row, "NAMEISPI:") == "YES":
            return "NAMEISPI:YES"
        _facet, _delivery, traits, reason = parse_type_or_provisional_default(row)
        if reason:
            provisional_facets[unit["corpus_key"]] = reason
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
        # `DESCISPI:YES` -- the row's OWN declaration that its description is
        # Product Identity, read the same way `pi_screening::declared_product_
        # identity` reads it off a corpus row's tokens (case-sensitive-value
        # `YES`, `NAMEISPI:NO`/`DESCISPI:NO` are not declarations). The
        # declared field is excluded from the term-blacklist scan below --
        # the row's own declaration already settles the question for that
        # field, and scanning it too would be redundant, not stricter,
        # exactly the "union, never a merge" rule `decisions.md §39.4` states
        # for the two screens. Every OTHER emitted value is still screened
        # exactly as before.
        desc_declared = token(row, "DESCISPI:") == "YES"

        # The name and key are the ONE field a hit here cannot be redacted
        # away from -- `decisions.md §24` is the fix, screened separately
        # from every other emitted value so a name-only hit renames rather
        # than drops.
        name_hits = pi_hits(terms, unit["corpus_key"], unit["name"])
        desc_hits = [] if desc_declared else pi_hits(terms, description)
        other_hits = pi_hits(
            terms,
            token(row, "SOURCEPAGE:"),
            *traits,
            *variables,
            *owners[unit["corpus_key"]],
        )
        if other_hits:
            # A hit outside the name/description fields is not something
            # either the `§24` rename or the redact-and-ship path can fix
            # (an owner's name, a trait/variable value) -- unchanged from
            # the prior behaviour: dropped.
            return f"{len(other_hits)} PI_BLACKLIST_TERMS hit(s) in emitted values"

        if name_hits:
            codex_name = neutral_name("monster_ability", book, unit["source_file"], unit["source_line"])
            codex_key = neutral_key("monster_ability", book, unit["source_file"], unit["source_line"])
            name_renamed[unit["corpus_key"]] = (codex_name, codex_key)
            renamed_divergence.append(
                divergence_entry(
                    "monster_ability", book, unit["source_file"], unit["source_line"], reason="name_pi_blocked"
                )
            )
            if desc_declared or desc_hits:
                desc_redacted.add(unit["corpus_key"])
            return None

        if desc_declared or desc_hits:
            desc_redacted.add(unit["corpus_key"])
            return None

        return None

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
        # An ability that both declared `DESCISPI:YES` and was dropped for a
        # SEPARATE reason (`NAMEISPI:YES`, or a term-blacklist hit on a
        # non-description field) has no description left to redact -- it has
        # no row left at all. `ability_pi_reason` never reaches the
        # `desc_redacted.add` line on the `NAMEISPI:YES` path (it returns
        # first), but a term-blacklist hit on another field runs AFTER that
        # line, so this cleanup is not a no-op.
        desc_redacted -= dropped_ability_keys
        # Symmetric safety net for the rename map -- structurally unreachable
        # today (`ability_pi_reason` returns on `other_hits` before it ever
        # populates `name_renamed`), kept so a future branch reordering
        # cannot silently ship a renamed row alongside a drop for the same
        # key.
        for key in dropped_ability_keys:
            name_renamed.pop(key, None)
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
    # `decisions.md §58.3`'s ruling, executed for the MONSTER half. A monster
    # row this repo ALREADY ships out of a different compiled table is not this
    # chassis's to emit: two records for one creature, under one wire code, in
    # one catalog, is a duplicate the player sees. So the monster row is
    # dropped here.
    #
    # The ABILITY half is different, and `§58.3`'s own text says so: "the 54
    # become a new named exclusion class: cross-table owner -- well-formed,
    # owned, and unreachable only because the owner lives in a different
    # table... a different remedy (widen the other table, or migrate it)."
    # `SD31-W22-MONSTER-001` scoped that remedy and left it unbuilt for a
    # dedicated cycle; this is that cycle. A `MonsterAbilityRecord` needs no
    # `MonsterStatBlock` in THIS table to resolve by key -- `monster_ability_
    # resolve` and `chassis_monster_ability_keys` (`v06_work_inventory.rs`)
    # both index `monster_abilities` directly, never through a monster's own
    # `ability_keys` list. So the row transcribes here, keyed to its REAL
    # owner (the legacy monster's name, exactly as `MonsterBook::abilities_
    # owned_by_name` below reads it) rather than to any block in this table's
    # own `monsters_static()` -- which is also why `abilities_of()` (the
    # `ability_keys`-driven path used for this table's OWN 234 monsters) never
    # picks these up and never double-serves them under a `bestiary` block.
    #
    # An orphan is a row nothing in the book owns. A CROSS-TABLE OWNER row is
    # well-formed and owned; only its OWNER's stat block lives in the other
    # table. Counted and cited separately in the header below, and NOT folded
    # into the plain "row-named"/"prefix" counts, so a reader can tell a
    # normal reach from this one.
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
        for ability_key in list(owners):
            before = owners[ability_key]
            after = [o for o in before if o not in cross_keys]
            if before and not after:
                # This row's ONLY owner(s) are cross-table monsters. Keep the
                # REAL owner name(s) (`before`, not the emptied `after`) --
                # transcribed below with `owners` intact, not dropped, so the
                # emitted record still says who grants it.
                stranded.add(ability_key)
                continue
            owners[ability_key] = after
        cross_table_abilities = [u for u in abilities if u["corpus_key"] in stranded]
        # NOT removed from `abilities`: these rows are real, owned, and
        # transcribed below like every other owned row, through the same
        # orphan/unscreenable screens that follow. Only their doc-comment
        # citation (header, below) calls them out as a distinct class.
        print(
            f"{book}: cross-table screen found {len(cross_table_monsters)} monster row(s) "
            f"already served by `data/corpus/{other_table_dir}/monster`; their "
            f"{len(cross_table_abilities)} ability row(s) transcribe here anyway, keyed to "
            "that real owner",
            file=sys.stderr,
        )

    # An ability row no monster row of this book claims is an ORPHAN: the
    # catalog renders an ability underneath its owning monster, so a record
    # with no owner loads and is never shown by `list_monster_catalog` --
    # never the stub class `decisions.md §44.2` was written about, because a
    # stub is a record a player's screen SHOWS empty, and an owner-less
    # record here reaches no screen at all (verified: `list_monster_catalog`
    # only ever walks a monster's OWN `ability_keys`, `monster_catalog.rs`,
    # never a bare scan of every `MonsterAbilityRecord`).
    #
    # `decisions.md §20` (2026-08-23) is dispositive: `no_record` means
    # never-ingested, and an un-ingested row's shape cannot be measured --
    # Gate 1's DoD is that every unit's shape IS measured, which is a
    # strictly weaker claim than "reaches a player". Rounds 2-through-T9
    # dropped orphans because a `engine-does-not-hold` row is honest about BOTH
    # claims failing; that conflated the two. `§20`'s own text says to
    # "claim reachability separately from ingestion, and only where
    # `reach_gate.rs` actually proves it" -- which this cycle now does: an
    # orphan SHIPS (owners: &[], shape measurable, `no_record` cleared) and
    # is pinned as a **named, provable non-reach** in
    # `reach_gate.rs::UNREACHED_RECORD_FINDINGS`, not silently claimed
    # reachable. `unreached_records_are_exactly_the_recorded_findings` fails
    # the build the moment an unpinned key stops reaching, so this cannot
    # rot into a silent stub the way an unchecked drop could.
    orphans = [u for u in abilities if not owners[u["corpus_key"]]]
    if orphans:
        print(
            f"{book}: {len(orphans)} orphan ability row(s) transcribed WITHOUT an owner "
            "(no monster row of this book claims them; ingested for shape measurement per "
            "decisions.md §20, reachability NOT claimed -- see reach_gate.rs "
            "UNREACHED_RECORD_FINDINGS): "
            + ", ".join(u["corpus_key"] for u in orphans),
            file=sys.stderr,
        )

    # The deferred half of `UnmodelledDesc`. A row the parser refused is fine
    # only if something else already dropped it; one that reached this point is
    # OWNED and would otherwise ship -- but "would ship" and "ships" are not
    # the same thing.
    #
    # **CORRECTED (`SD31-E6-F9-005`): this used to `raise SystemExit`, which
    # crashed the WHOLE BOOK's transcription over these few rows** -- not just
    # refusing the ambiguous row, refusing every OTHER genuinely-owned,
    # cleanly-parseable ability in the same book too. Confirmed live against
    # the pinned oracle: re-running this script for `bestiary`/`bestiary_2`
    # (before this fix) raised on exactly the 5 `ce_abilities_race.lst` rows
    # `OPEN-ISSUES.md` row 157 names and produced ZERO other movement, even
    # though `classify_monster_ability_rows.py` independently confirms 135
    # (`bestiary`) + 95 (`bestiary_2`) OTHER engine-does-not-hold ability rows are
    # genuinely row-named/prefix-owned and parse cleanly -- the crash was
    # silently blocking all of them, not just the 5 unscreenable ones.
    #
    # The fix picks the SAME remedy this transcriber already applies to a PI
    # row or a `.COPY=` row: drop the row that cannot be shipped honestly,
    # name it precisely in the header and on stderr, and let every OTHER row
    # this book owns transcribe. This is not a widening of `parse_desc` --
    # picking the wrong `DESC:` variant under time pressure is exactly the
    # fabrication risk `OPEN-ISSUES.md` row 157 correctly declined to take,
    # and this fix still declines it. It only stops that unresolved question
    # from holding every unrelated, unambiguous record hostage.
    unscreenable_shipping = [u for u in abilities if u["corpus_key"] in unscreenable]
    if unscreenable_shipping:
        unscreenable_keys = {u["corpus_key"] for u in unscreenable_shipping}
        abilities = [u for u in abilities if u["corpus_key"] not in unscreenable_keys]
        # Same cleanup the PI-drop pass above already performs, and for the
        # identical reason: a monster's OWN `ability_keys` field was computed
        # early (`mine`, before this or any other drop pass ran) and must not
        # keep naming a key that no longer has a row in `abilities` -- an
        # un-caught instance of exactly this shape is what
        # `bestiary::tests::every_ability_key_a_shipped_monster_names_
        # resolves_here` caught live this cycle (`Demon (Hezrou) names
        # ability Stench, which this table does not define`) before this
        # line existed.
        for key in monster_ability_keys:
            monster_ability_keys[key] = [
                a for a in monster_ability_keys[key] if a not in unscreenable_keys
            ]
        print(
            f"{book}: {len(unscreenable_shipping)} owned ability row(s) NOT transcribed "
            "(parse_desc cannot resolve their multi-DESC: shape without guessing): "
            + "; ".join(
                f"{u['source_file']}:{u['source_line']} ({u['corpus_key']})"
                for u in unscreenable_shipping
            ),
            file=sys.stderr,
        )

    # A row whose `TYPE:` segments name no facet this chassis models USED to
    # be dropped here exactly like `unscreenable` above. `decisions.md §27`
    # now grants a provisional `SpecialQuality` default instead of a drop --
    # the row SHIPS, `parse_type_or_provisional_default` supplies the
    # default facet, and the emission loop below records which corpus_keys
    # were defaulted (and why) into `provisional_facets` for the caller to
    # stamp via `shape_provisional_marker.stamp_provisional_default`. No
    # rows are removed from `abilities` for this reason any more.

    # Finalized against whatever `abilities` actually ships after every screen
    # above (the `.COPY=`/`.MOD`/cross-table/orphan passes can each remove a
    # row this set was computed before) -- an ability no longer shipping has
    # no description left to redact either, and one dropped for an unrelated
    # reason (e.g. `unscreenable`) has no name left to rename.
    shipping_keys = {u["corpus_key"] for u in abilities}
    desc_redacted &= shipping_keys
    name_renamed = {k: v for k, v in name_renamed.items() if k in shipping_keys}
    # Same reason, applied to `provisional_facets` -- IN PLACE (never
    # rebound: `write_book` holds a reference to this exact dict).
    for key in [k for k in provisional_facets if k not in shipping_keys]:
        del provisional_facets[key]

    # Every ability key this table emits, after renaming: the identity a
    # cross-reference (a monster's own `ability_keys` list) must use to find
    # a renamed row, because the row's emitted `key` is the neutral one, not
    # `corpus_key`. Every OTHER key maps to itself.
    def emitted_ability_key(k: str) -> str:
        return name_renamed[k][1] if k in name_renamed else k
    if desc_redacted:
        print(
            f"{book}: {len(desc_redacted)} ability row(s) description redacted "
            f"(DESCISPI:YES): " + ", ".join(sorted(desc_redacted)),
            file=sys.stderr,
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
            "//! row DECLARES its name Product Identity (`NAMEISPI:YES`, PCGen's own"
        )
        out.append(
            "//! per-record marker) or because a `pi_screening::PI_BLACKLIST_TERMS` term lands"
        )
        out.append(
            "//! in a field neither the `§24` rename nor the description-redact path can fix"
        )
        out.append(
            "//! (an owner's name, a trait/variable value). A hit confined to the name/key or"
        )
        out.append(
            "//! description alone ships instead -- see the renamed/redacted lists below."
        )
        out.append(
            "//! Reclassifying is `docs/governance/ogl-pi-blacklist.md` §3's per-book override,"
        )
        out.append(
            "//! an operator"
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
    if name_renamed:
        out.append("//!")
        out.append(
            f"//! {len(name_renamed)} ability row(s) of this book have their OWN name/key match"
        )
        out.append(
            "//! a `pi_screening::PI_BLACKLIST_TERMS` term -- `decisions.md §24`'s \"the name"
        )
        out.append(
            "//! itself is PI\" case. Each ships under a Codex-generated NEUTRAL name/key"
        )
        out.append(
            "//! derived ONLY from `(kind, book, source_file, source_line)` -- never from the"
        )
        out.append(
            "//! original name, not even transformed -- `scripts/codex_neutral_name.py`. Per"
        )
        out.append(
            "//! `§24b`-4, the divergence record below stops at the coordinate and the reason;"
        )
        out.append("//! the original string is never written here:")
        for entry in sorted(renamed_divergence, key=lambda e: (e["source_file"], e["source_line"])):
            out.append(
                f"//!   * `{entry['source_file']}:{entry['source_line']}` "
                f"-> {entry['codex_name']} ({entry['reason']})"
            )
    if desc_redacted:
        out.append("//!")
        out.append(
            f"//! {len(desc_redacted)} ability row(s) of this book carry Product Identity in"
        )
        out.append(
            "//! their `description` field ONLY (declared `DESCISPI:YES`, or an undeclared"
        )
        out.append(
            "//! `pi_screening::PI_BLACKLIST_TERMS` term found by scanning) -- `description`"
        )
        out.append(
            "//! (and its `%N` variables) SHIP REDACTED to `shape_b_v1::REDACTED_PI_MARKER`"
        )
        out.append(
            "//! rather than dropped, because a description (unlike a name) can be redacted"
        )
        out.append(
            "//! and the record still works. Reclassifying is"
        )
        out.append(
            "//! `docs/governance/ogl-pi-blacklist.md` §3's per-book override, an operator"
        )
        out.append("//! decision, not a transcriber's:")
        redacted_units = {u["corpus_key"]: u for u in abilities}
        for key in sorted(desc_redacted):
            unit = redacted_units[key]
            out.append(
                f"//!   * `{unit['source_file']}:{unit['source_line']}` "
                f"({emitted_ability_key(key)})"
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
            f"//! under one wire code). {len(cross_table_abilities)} further ability row(s) ARE"
        )
        out.append(
            "//! transcribed below despite this (`SD31-W23-MONSTER-001`, `§58.3`'s own deferred"
        )
        out.append(
            "//! 'different remedy'): CROSS-TABLE OWNER rows are well-formed and owned, only by"
        )
        out.append(
            "//! a monster whose STAT BLOCK lives in the other table. An ability record needs no"
        )
        out.append(
            "//! stat block of its own to resolve by key, so these ship with their real owner"
        )
        out.append(
            "//! name intact and are read by `MonsterBook::abilities_owned_by_name`, never by"
        )
        out.append(
            "//! `abilities_of()` (which walks a `MonsterStatBlock.ability_keys` this table has"
        )
        out.append(
            "//! none of for these owners) -- so they never double-serve under a `bestiary`"
        )
        out.append("//! block. Cited by corpus line:")
        for unit in cross_table_monsters:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}` (monster row)")
        for unit in cross_table_abilities:
            out.append(
                f"//!   * `{unit['source_file']}:{unit['source_line']}` (ability row, "
                f"owner: {', '.join(owners[unit['corpus_key']])})"
            )
    if orphans:
        out.append("//!")
        out.append(
            f"//! {len(orphans)} further ability row(s) in this book are ORPHANS -- no monster"
        )
        out.append(
            "//! row here claims them, so they SHIP with `owners: &[]` rather than being"
        )
        out.append(
            "//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,"
        )
        out.append(
            "//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`"
        )
        out.append(
            "//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here"
        )
        out.append(
            "//! reaches no screen -- reachability is NOT claimed for these, and each key is"
        )
        out.append(
            "//! pinned as a named, provable non-reach in `reach_gate.rs::"
        )
        out.append(
            "//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:"
        )
        # Cited by FILE:LINE, not by key, for the same reason the PI block above
        # is: an orphan created by a PI drop carries the dropped row's declared
        # Product Identity name in its own namespaced key.
        for unit in orphans:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}`")
    if unscreenable_shipping:
        out.append("//!")
        out.append(
            f"//! {len(unscreenable_shipping)} further ability row(s) of this book ARE owned"
        )
        out.append(
            "//! but are NOT transcribed: each carries several `DESC:` tokens under a gate"
        )
        out.append(
            "//! `parse_desc` does not model (a `PREVAREQ`/`PREVARGT` comparison against a"
        )
        out.append(
            "//! `BONUS:VAR`-set value), and picking one by position would risk shipping"
        )
        out.append(
            "//! subtly wrong player-facing text. `engine-does-not-hold` is their honest status; widen"
        )
        out.append(
            "//! `parse_desc` deliberately, hand-verified per row, to reach them:"
        )
        for unit in unscreenable_shipping:
            out.append(f"//!   * `{unit['source_file']}:{unit['source_line']}` ({unit['corpus_key']})")
    if provisional_facets:
        out.append("//!")
        out.append(
            f"//! {len(provisional_facets)} ability row(s) ship with a `decisions.md §27`"
        )
        out.append(
            "//! PROVISIONAL `SpecialQuality` facet default (their own `TYPE:` segments name"
        )
        out.append(
            "//! no facet this chassis models) -- this is NOT a measured shape, only an ingest"
        )
        out.append(
            "//! unblock; each record's `shape_provisional_default`/`shape_provisional_reason`"
        )
        out.append(
            "//! fields (stamped by `shape_provisional_marker.py`, never written by hand) are"
        )
        out.append(
            "//! what `row 17`'s real categorization pass (`§27a`) must retire to zero:"
        )
        for key in sorted(provisional_facets):
            out.append(f"//!   * `{key}` ({provisional_facets[key]})")
    out.append("")
    # `MonsterSpellLikeAbility` is imported only when this book actually
    # constructs one. Four registered books (Monster Codex, both Book of the
    # Damned volumes, Horror Adventures) carry no `SPELLS:` grant on any
    # monster row at all, and an unconditional import there is an
    # `unused_imports` warning in a generated file -- which is noise the next
    # reader has to re-diagnose, not a harmless extra line.
    imports = [
        "MonsterAbilityDelivery",
        "MonsterAbilityFacet",
        "MonsterAbilityRecord",
    ]
    if any(parse_spell_like_abilities(monster_rows[u["corpus_key"]]) for u in monsters):
        imports.append("MonsterSpellLikeAbility")
    imports += ["MonsterStatBlock", "NaturalAttack", "Speed", "StatAdjustment"]
    out.append(
        "use crate::rules_core::rules_tables::monster_chassis::{"
        + ", ".join(imports)
        + "};"
    )
    out.append("")
    out.append(f"/// Every {book} monster stat block ({len(monsters)} rows).")
    out.append("pub(super) static MONSTERS: &[MonsterStatBlock] = &[")
    for unit in monsters:
        key = unit["corpus_key"]
        row = monster_rows[key]
        speeds = parse_speeds(row)
        attacks = parse_natural_attacks(row)
        stat_adjustments = parse_stat_adjustments(row)
        has_spell_like_abilities = parse_has_spell_like_abilities(row)
        sla_cl_token = parse_sla_cl_token(row)
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
            "        ability_keys: "
            f"{rust_slice([emitted_ability_key(k) for k in monster_ability_keys[key]])},"
        )
        out.append(f"        external_ability_refs: {rust_slice(external[key])},")
        out.append(
            "        stat_adjustments: &["
            + ", ".join(
                f"StatAdjustment {{ ability: {rust_str(a)}, amount: {v} }}"
                for a, v in stat_adjustments
            )
            + "],"
        )
        out.append(
            f"        has_spell_like_abilities: {'true' if has_spell_like_abilities else 'false'},"
        )
        out.append(f"        sla_cl_token: {rust_opt(sla_cl_token)},")
        out.append(
            "        spell_like_abilities: &["
            + ", ".join(
                "MonsterSpellLikeAbility { "
                f"label: {rust_str(label)}, "
                f"times: {rust_opt(times)}, "
                f"time_unit: {rust_opt(time_unit)}, "
                f"caster_level_token: {rust_opt(caster_level)}, "
                f"spell: {rust_str(spell)}, "
                f"save_dc_token: {rust_opt(save_dc)} }}"
                for label, times, time_unit, caster_level, spell, save_dc
                in parse_spell_like_abilities(row)
            )
            + "],"
        )
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
        out.append(f"        source_line: {unit['source_line']},")
        out.append("    },")
    out.append("];")
    out.append("")
    out.append(f"/// Every {book} monster-ability record ({len(abilities)} rows).")
    out.append("pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[")
    for unit in abilities:
        row = read_row(resolve_book_file(root, unit["source_file"]), unit["source_line"])
        facet, delivery, traits, facet_provisional_reason = parse_type_or_provisional_default(row)
        if facet_provisional_reason:
            provisional_facets[unit["corpus_key"]] = facet_provisional_reason
        description, variables = parse_desc(row)
        if unit["corpus_key"] in desc_redacted:
            # `DESCISPI:YES`, or an undeclared blacklist-term hit found by
            # scanning -- either way the redaction promised by the module
            # doc's own listing above. The `%N` placeholders in `description`
            # name variables from the ORIGINAL text, which no longer ships,
            # so they are cleared too rather than left dangling against a
            # marker string that contains no `%N` for them to refer to.
            description = redacted_pi_marker()
            variables = []
        renamed = name_renamed.get(unit["corpus_key"])
        emitted_key = renamed[1] if renamed else unit["corpus_key"]
        emitted_name = renamed[0] if renamed else unit["name"]
        out.append("    MonsterAbilityRecord {")
        out.append(f"        key: {rust_str(emitted_key)},")
        out.append(f"        name: {rust_str(emitted_name)},")
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
        out.append(f"        source_file: {rust_str(unit['source_file'])},")
        out.append(f"        source_line: {unit['source_line']},")
        # `decisions.md §24b`-3: "a field marks it as carrying a
        # Codex-generated name". `§24b`-4: the divergence record stops at
        # the coordinate -- never the original string.
        out.append(f"        codex_generated_name: {'true' if renamed else 'false'},")
        out.append(f"        rename_reason: {rust_opt('name_pi_blocked' if renamed else None)},")
        out.append(
            "        rename_coordinate: "
            + rust_opt(f"{book}:{unit['source_file']}:{unit['source_line']}" if renamed else None)
            + ","
        )
        out.append("    },")
    out.append("];")
    out.append("")
    if provisional_facets:
        print(
            f"{book}: {len(provisional_facets)} ability row(s) shipped with a "
            "decisions.md §27 PROVISIONAL SpecialQuality facet default (not a "
            "measured shape -- stamped via shape_provisional_marker, see "
            "workflow-instruction.md §6a): "
            + ", ".join(sorted(provisional_facets)),
            file=sys.stderr,
        )
    return "\n".join(out)


# The `data/corpus/` directory name a book's chassis output actually lands
# in, when it differs from the transcriber's own `BOOKS` key. Reuses
# `CROSS_TABLE_MONSTER_RECORDS`'s key/value pair rather than a second
# hand-written map (`decisions.md §17`) -- that dict already names this
# exact fact for a different reason (the `bestiary` -> `beastiary` on-disk
# spelling), and it is the only book in this lane whose key and corpus
# directory diverge.
def corpus_output_dir(book: str) -> str:
    return CROSS_TABLE_MONSTER_RECORDS.get(book, book)


def provisional_facet_units(book: str) -> dict[str, str]:
    """Read-only: `{corpus_key: reason}` for every `monster_ability` row
    `book` currently ships (or would ship) under `decisions.md §27`'s
    provisional facet default. Calls `transcribe()` purely for its
    classification side effect on `provisional_facets` -- this performs no
    file I/O and can be called any number of times without writing
    anything, which is what lets the stamping step (run after `gen_book_
    cache` has already produced the JSON files) recompute the population
    instead of needing `write_book` to have captured and persisted it."""
    provisional_facets: dict[str, str] = {}
    transcribe(book, provisional_facets)
    return provisional_facets


def write_book(book: str) -> str:
    """Transcribe `book` and write it to its `monster_data.rs`, atomically.

    `transcribe()` can raise partway through a book with real, un-fabricatable
    problems (an orphan-owning row whose `DESC:` shape `parse_desc` refuses,
    `SD31-E6-F9-002`'s own `ce_abilities_race.lst:1955`/`:2043` finding) --
    that is the transcriber correctly REFUSING rather than guessing, not a bug.
    The bug this guards is orthogonal: `main()` used to `open(path, "w")`
    *before* calling `transcribe()`, which truncates the target file to 0
    bytes immediately, so a mid-`transcribe()` raise left the file empty --
    confirmed live twice in one cycle (`SD31-E6-F9-002`, `bestiary` and
    `bestiary_2`; both were the committed, unmodified file at the time, so
    `git checkout --` would have equally recovered them -- a WORKING tree with
    uncommitted local changes queued for this same book would not have had
    that luxury). `transcribe()`
    is computed FIRST, in full, into a string; the file on disk is touched
    only after that succeeds, and a raise leaves the existing file exactly as
    it was.

    The write itself is now genuinely atomic too (`SD31-W9-INTEGRATE-001`
    finding: the word was true of the compute-then-write ordering above but
    not of the write call itself -- an interruption or disk-full error
    mid-`handle.write()` could still have left a truncated file on disk,
    the identical failure mode this docstring's own word promises against).
    Writes to a same-directory temp file first, then `os.replace()`s it onto
    the real path -- `os.replace` is a single filesystem rename, so the
    target either has the OLD complete content or the NEW complete content,
    never a partial write, on every platform this repo runs on.
    """
    path = f"src/rules_core/rules_tables/{book}/monster_data.rs"
    content = transcribe(book)
    tmp_path = f"{path}.tmp"
    with open(tmp_path, "w", encoding="utf-8") as handle:
        handle.write(content)
    os.replace(tmp_path, path)
    return path


def main() -> None:
    if len(sys.argv) != 2 or sys.argv[1] not in BOOKS:
        raise SystemExit(f"usage: {sys.argv[0]} <{'|'.join(sorted(BOOKS))}>")
    path = write_book(sys.argv[1])
    print(f"wrote {path}")


if __name__ == "__main__":
    main()
