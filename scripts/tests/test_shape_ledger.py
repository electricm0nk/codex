#!/usr/bin/env python3
"""Tests for `scripts/shape_ledger.py` (SD-32 Gate 1, card `gate-1-shape-closure`).

Proves the two load-bearing claims AT-32-G1-001/002 make:

1. Every not-done unit is classified into exactly one family -- including
   the honest F0 (no formula content) and F8 (residual) extension families
   -- so `unclassified_count` is structurally 0, not merely 0 by luck on
   today's inventory.
2. The tool fails closed on an empty/unreadable inventory (AT-32-G1-002):
   it must refuse to report "0 unclassified" when there was nothing to
   classify, mirroring `test_coverage_ledger.py`'s negative-case
   discipline for the same shape of claim.

Uses small synthetic inventory/corpus fixtures rather than the live 38k-unit
corpus, so these tests stay fast and are not subject to corpus drift
(same discipline as `test_coverage_ledger.py`'s own docstring).
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_ledger as SL  # noqa: E402


def _unit(id_, kind, book, status, wiring_class, source_file, source_line, **extra):
    u = {
        "id": id_,
        "kind": kind,
        "book": book,
        "status": status,
        "wiring_class": wiring_class,
        "source_file": source_file,
        "source_line": source_line,
    }
    u.update(extra)
    return u


class ExtractFormulaSegmentTest(unittest.TestCase):
    def test_define_takes_second_field(self):
        self.assertEqual(SL.extract_formula_segment("DEFINE", "MesmeristPool|0"), "0")

    def test_define_missing_second_field_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("DEFINE", "MesmeristPool"))

    def test_bonus_var_takes_third_field(self):
        self.assertEqual(
            SL.extract_formula_segment("BONUS", "VAR|TDeftFingersBonus|max(MesmeristLVL/2,1)"),
            "max(MesmeristLVL/2,1)",
        )

    def test_bonus_skill_takes_third_field_ignores_trailing_type(self):
        self.assertEqual(
            SL.extract_formula_segment("BONUS", "SKILL|Perception|8|TYPE=Racial"), "8"
        )

    def test_bonus_too_short_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("BONUS", "SKILL|Perception"))

    def test_non_define_non_bonus_returns_none(self):
        self.assertIsNone(SL.extract_formula_segment("ABILITY", "Special Ability|X"))


class ClassifyFormulaTest(unittest.TestCase):
    def test_flat_constant(self):
        self.assertEqual(SL.classify_formula("2"), "F1")
        self.assertEqual(SL.classify_formula("-4"), "F1")
        self.assertEqual(SL.classify_formula("10%"), "F1")

    def test_per_level_scaling(self):
        self.assertEqual(SL.classify_formula("MesmeristLVL/2"), "F2")
        self.assertEqual(SL.classify_formula("WizardLVL"), "F2")

    def test_ability_modifier(self):
        self.assertEqual(SL.classify_formula("WIS"), "F3")
        self.assertEqual(SL.classify_formula("-CON"), "F3")

    def test_named_counter_plain_identifier(self):
        self.assertEqual(SL.classify_formula("BloodlineLVL_UNRELATED_NAME_no_lvl_suffix"), "F4")
        self.assertEqual(SL.classify_formula("TInjectionsUses"), "F4")

    def test_clamped_per_level(self):
        self.assertEqual(SL.classify_formula("max(MesmeristLVL/2,1)"), "F5")
        self.assertEqual(SL.classify_formula("min(floor((Sorcerer_Psychic_BloodlinePower3LVL+3)/6*2),4)"), "F5")

    def test_classlevel(self):
        self.assertEqual(SL.classify_formula('classlevel("Wizard")'), "F6")

    def test_conditional_step(self):
        self.assertEqual(
            SL.classify_formula("if(Sorcerer_CF_BloodlineArcana==0,1,0)"), "F7"
        )

    def test_skill_rank_derived(self):
        self.assertEqual(SL.classify_formula('skillinfo("RANK","Bluff")'), "F9")
        self.assertEqual(SL.classify_formula("TOTALRANK"), "F9")
        # skillinfo wins priority over a co-occurring LVL term
        self.assertEqual(
            SL.classify_formula('if(skillinfo("RANK","Bluff")>=WizardLVL,1,0)'), "F9"
        )

    def test_level_threshold_step_count(self):
        self.assertEqual(
            SL.classify_formula("if(WizardLVL>=5,1,0)+if(WizardLVL>=10,1,0)+if(WizardLVL>=15,1,0)"),
            "F10",
        )

    def test_residual_falls_to_f8(self):
        # Multi-term arithmetic mixing identifiers this classifier does
        # not recognise -- named F8 per the module docstring, not silently
        # dropped into F0/F1.
        self.assertEqual(SL.classify_formula('var("COUNT[EQTYPE.ARMOR.EQUIPPED]")*2'), "F8")

    def test_empty_string_is_f0_not_f8(self):
        self.assertEqual(SL.classify_formula(""), SL.FAMILY_F0_NO_FORMULA)

    def test_every_family_id_referenced_has_metadata(self):
        # Every id this function can return must have a label + proof_width
        # in the family metadata table -- AT-32-G1-003's requirement that
        # every family states its proof width, checked structurally.
        meta = SL._family_metadata()
        possible_ids = {fid for fid, *_ in SL.FAMILIES} | {SL.FAMILY_F0_NO_FORMULA, SL.FAMILY_F8_OTHER}
        for fid in possible_ids:
            self.assertIn(fid, meta, f"family {fid} has no metadata entry")
            self.assertTrue(meta[fid]["label"])
            self.assertTrue(meta[fid]["proof_width"])


class BuildCorpusIndexTest(unittest.TestCase):
    def test_indexes_by_book_basename_line_and_filters_define_bonus(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "spell")
            os.makedirs(book_dir)
            record = {
                "data": {
                    "raw_tokens": [
                        {"key": "DESC", "value": "irrelevant, not a formula token"},
                        {"key": "BONUS", "value": "VAR|Foo|WIS"},
                        {"key": "DEFINE", "value": "Bar|0"},
                    ]
                },
                "source": {"path": "some/path/test_spells.lst", "line": 42},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)
            # LICENSE.json must be skipped even if present
            with open(os.path.join(book_dir, "LICENSE.json"), "w") as fh:
                json.dump({"data": {"raw_tokens": [{"key": "BONUS", "value": "SHOULD|NOT|APPEAR"}]}}, fh)

            index = SL.build_corpus_index(tmp, books={"test_book"})
            key = ("test_book", "spell", "test_spells.lst", 42)
            self.assertIn(key, index)
            toks = index[key]
            self.assertEqual(len(toks), 2)
            self.assertTrue(all(t["key"] in ("BONUS", "DEFINE") for t in toks))

    def test_bestiary_book_walks_the_beastiary_corpus_directory(self):
        """`data/corpus/`'s core Bestiary output lives under the historical
        `beastiary` spelling (see `scripts/transcribe_monster_tables.py`'s
        `CROSS_TABLE_MONSTER_RECORDS = {"bestiary": "beastiary"}` and
        `src/bin/gen_book_cache.rs`'s `corpus_book: "beastiary"`), but every
        `docs/work-inventory.json` unit for this book carries `book:
        "bestiary"` (no trailing `a`). Without an alias, `build_corpus_index`
        asked for book `"bestiary"` walks a near-empty directory and every
        such unit's join reports `no_record` even though its record exists.
        This reproduced for real: 260 `bestiary` `monster_ability` units
        alone (`python3 scripts/shape_ledger.py --inventory
        docs/work-inventory.json --output /tmp/l.json` then filter
        `join_status == "no_record" and book == "bestiary"`)."""
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "beastiary", "monster_ability")
            os.makedirs(book_dir)
            record = {
                "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                "source": {"path": "ce_abilities_race.lst", "line": 1280},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)

            index = SL.build_corpus_index(tmp, books={"bestiary"})
            key = ("bestiary", "monster_ability", "ce_abilities_race.lst", 1280)
            self.assertIn(key, index)
            self.assertEqual(len(index[key]), 1)

    def test_bestiary_alias_does_not_hide_the_correctly_spelled_directory(self):
        """`BOOK_CORPUS_DIR_ALIASES` routes book `"bestiary"` to the legacy
        `beastiary` directory (see the test above). But not every `bestiary`
        kind lives under that legacy spelling -- `spell` (and `equipment`)
        records this bundle ingested land under the CORRECTLY-spelled
        `data/corpus/bestiary/` directory, which the alias-only walk never
        visits. Before this fix, 109 real `bestiary` `spell` records (already
        on disk at `data/corpus/bestiary/spell/*.json`, e.g.
        `blur_self_only.json`, ce_spells.lst:62) were invisible to the join
        and reported `no_record` even though their corpus record exists
        (`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
        --output /tmp/l.json` then filter `join_status == "no_record" and
        book == "bestiary" and kind == "spell"`). The index must contain
        records from BOTH the aliased legacy directory AND the book's own
        correctly-spelled directory."""
        with tempfile.TemporaryDirectory() as tmp:
            legacy_dir = os.path.join(tmp, "beastiary", "monster_ability")
            os.makedirs(legacy_dir)
            with open(os.path.join(legacy_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                        "source": {"path": "ce_abilities_race.lst", "line": 1280},
                    },
                    fh,
                )
            real_dir = os.path.join(tmp, "bestiary", "spell")
            os.makedirs(real_dir)
            with open(os.path.join(real_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": []},
                        "source": {"path": "ce_spells.lst", "line": 62},
                    },
                    fh,
                )

            index = SL.build_corpus_index(tmp, books={"bestiary"})
            legacy_key = ("bestiary", "monster_ability", "ce_abilities_race.lst", 1280)
            real_key = ("bestiary", "spell", "ce_spells.lst", 62)
            self.assertIn(legacy_key, index)
            self.assertIn(real_key, index)

    def test_join_is_kind_aware_two_kinds_at_the_identical_coordinate_index_separately(self):
        """The real defect this fix closes (`epic-6-kind-trait_cycle-2_cycle_
        receipt.md` §4/`decisions.md §25` discovery-forward): a pre-`Kind::
        Trait` generic-ingest pass wrote a `kind: ability`-shaped record at
        the SAME `(book, source_file, source_line)` coordinate a `kind:
        trait` census unit cites (e.g. `inner_sea_races/ability/loner_of_
        the_rocks.json` carries `TYPE:Trait.RaceTrait.Oread Race Trait` at
        `isr_abilities.lst`). Before this fix, `build_corpus_index` keyed
        only on `(book, basename, line)`, so the second kind written to that
        coordinate silently overwrote (or was silently read as) the first --
        a `trait` unit's join would land on the `ability` record and never
        report `no_record`, even though no `trait` record exists anywhere.
        The index must carry BOTH kinds' tokens under their own keys."""
        with tempfile.TemporaryDirectory() as tmp:
            ability_dir = os.path.join(tmp, "b", "ability")
            trait_dir = os.path.join(tmp, "b", "trait")
            os.makedirs(ability_dir)
            os.makedirs(trait_dir)
            with open(os.path.join(ability_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                        "source": {"path": "isr_abilities.lst", "line": 78},
                    },
                    fh,
                )
            # No record actually exists under kind "trait" at this coordinate
            # -- that is the whole point: the bug was that the "ability"
            # record answered for it anyway.

            index = SL.build_corpus_index(tmp, books={"b"})
            ability_key = ("b", "ability", "isr_abilities.lst", 78)
            trait_key = ("b", "trait", "isr_abilities.lst", 78)
            self.assertIn(ability_key, index)
            self.assertNotIn(trait_key, index)

    def test_generic_sibling_directory_normalizes_to_its_base_kind(self):
        """`ingest_generic_kind.py`/`ingest_race_trait_generic.py` write
        `<kind>_generic/` as a deliberate SIBLING to `<kind>/` (never inside
        it -- their own docstrings name the reason: existing curated
        consumers glob `<kind>/*.json` directly and would misinterpret the
        flatter generic shape). Their whole design depended on the OLD
        kind-BLIND join to count as a real answer for a `<kind>` census
        unit ("`shape_ledger.py::build_corpus_index` walks ... with no
        subdirectory-name filter, so a sibling directory is exactly as
        measurable for Gate-1 purposes"). Making the join kind-AWARE must
        not silently break that intentional design -- a `<kind>_generic`
        record must still satisfy a `<kind>` unit's join. Only a
        genuinely-different kind (not a `<X>_generic` sibling of the
        unit's own kind) must be refused."""
        with tempfile.TemporaryDirectory() as tmp:
            generic_dir = os.path.join(tmp, "b", "trait_generic")
            os.makedirs(generic_dir)
            with open(os.path.join(generic_dir, "unit.json"), "w") as fh:
                json.dump(
                    {
                        "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                        "source": {"path": "apg_abilities.lst", "line": 109},
                    },
                    fh,
                )
            index = SL.build_corpus_index(tmp, books={"b"})
            self.assertIn(("b", "trait", "apg_abilities.lst", 109), index)

            unit = _unit("b:trait:x", "trait", "b", "not-started", "static", "apg_abilities.lst", 109)
            row = SL.classify_unit(unit, index)
            self.assertEqual(row["join_status"], "matched")
            self.assertEqual(row["family"], "F3")


class ClassifyUnitTest(unittest.TestCase):
    def test_no_join_match_is_f0_no_record(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1)
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_record")

    def test_join_match_no_formula_tokens_is_f0(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "spell", "f.lst", 1): []}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_formula_tokens")

    def test_join_match_picks_highest_priority_family(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {
            ("b", "spell", "f.lst", 1): [
                {"key": "BONUS", "value": "VAR|Foo|WIS"},  # F3
                {"key": "BONUS", "value": 'VAR|Bar|skillinfo("RANK","Bluff")'},  # F9, higher priority
            ]
        }
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], "F9")
        self.assertEqual(row["join_status"], "matched")

    def test_join_is_kind_aware_a_different_kinds_record_at_same_coordinate_is_no_record(self):
        """The exact real-world defect (`epic-6-kind-trait_cycle-2_cycle_
        receipt.md` §4): a `trait` unit's `(book, source_file, source_line)`
        coordinate has a real record on disk, but that record is `kind:
        ability`, not `kind: trait`. The join must not credit the wrong
        kind's record -- this must report `no_record`, not `matched`."""
        unit = _unit(
            "inner_sea_races:trait:loner_of_the_rocks",
            "trait",
            "inner_sea_races",
            "not-started",
            "static",
            "isr_abilities.lst",
            78,
        )
        # Only an "ability"-kind record exists at this coordinate -- no
        # "trait"-kind record was ever indexed for it.
        index = {("inner_sea_races", "ability", "isr_abilities.lst", 78): [{"key": "BONUS", "value": "VAR|Foo|WIS"}]}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["join_status"], "no_record")
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)

    def test_missing_source_fields_is_no_record(self):
        unit = {"id": "b:spell:x", "kind": "spell", "book": "b"}
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "no_record")

    def test_citation_redirect_fallback_matches_by_book_kind_key_when_primary_join_misses(self):
        """`ultimate_magic`'s 11 `equipment` units this cycle traced
        (`decisions.md §20`/§17a): `docs/work-inventory.json`'s own equipment
        enumeration mints exactly one unit per corpus_key, and for a
        `.COPY=`-aliased spellbook whose key is ALSO restated (with no new
        content) by `ultimate_magic/_pfs/pfs_um_equip_general.lst`'s PFS
        legality overlay, the surviving unit's `source_file`/`source_line`
        cite the OVERLAY row -- while the real, already-ingested corpus
        record (e.g. `data/corpus/ultimate_magic/equipment/book_of_harms.json`)
        was generated from and cites the BASE `um_equip_general.lst` row.
        The primary (book, source_file, source_line) join therefore misses a
        record that genuinely exists. `key_index`, keyed on the corpus
        record's own `(book, kind, data.key)` identity -- never on `name`
        alone, matching `equipment_gap.rs`'s own documented `held`-map
        name-collision hazard -- recovers it as `matched`/`no_formula_tokens`
        rather than a false `no_record`."""
        unit = _unit(
            "ultimate_magic:equipment:book_of_harms",
            "equipment",
            "ultimate_magic",
            "not-started",
            "static",
            "pfs_um_equip_general.lst",
            8,
            corpus_key="Book of Harms",
        )
        key_index = {("ultimate_magic", "equipment", "Book of Harms"): []}
        row = SL.classify_unit(unit, corpus_index={}, key_index=key_index)
        self.assertEqual(row["join_status"], "no_formula_tokens")
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)

    def test_citation_redirect_fallback_still_classifies_real_formula_tokens(self):
        unit = _unit(
            "b:equipment:x",
            "equipment",
            "b",
            "not-started",
            "static",
            "overlay.lst",
            1,
            corpus_key="Widget",
        )
        key_index = {("b", "equipment", "Widget"): [{"key": "BONUS", "value": "VAR|Foo|WIS"}]}
        row = SL.classify_unit(unit, corpus_index={}, key_index=key_index)
        self.assertEqual(row["join_status"], "matched")
        self.assertEqual(row["family"], "F3")

    def test_citation_redirect_fallback_never_fires_across_a_different_kind(self):
        """The fallback is keyed on `(book, kind, key)`, never `(book, key)`
        alone -- a same-named key in a DIFFERENT kind directory (e.g. an
        `equipment_modifier` record) must never satisfy an `equipment`
        unit's join, the same discipline `equipment_gap.rs`'s own `held`
        map already applies per-book."""
        unit = _unit(
            "b:equipment:x", "equipment", "b", "not-started", "static", "overlay.lst", 1, corpus_key="Widget"
        )
        key_index = {("b", "equipment_modifier", "Widget"): [{"key": "BONUS", "value": "VAR|Foo|WIS"}]}
        row = SL.classify_unit(unit, corpus_index={}, key_index=key_index)
        self.assertEqual(row["join_status"], "no_record")

    def test_primary_join_wins_over_key_index_fallback_when_both_present(self):
        """The fallback is a LAST resort -- when the primary (book,
        source_file, source_line) join already finds the record, the
        fallback path (and its coarser identity) is never consulted."""
        unit = _unit(
            "b:equipment:x", "equipment", "b", "not-started", "static", "f.lst", 1, corpus_key="Widget"
        )
        corpus_index = {("b", "equipment", "f.lst", 1): [{"key": "DEFINE", "value": "Foo|0"}]}
        key_index = {("b", "equipment", "Widget"): []}
        row = SL.classify_unit(unit, corpus_index, key_index=key_index)
        self.assertEqual(row["join_status"], "matched")
        self.assertEqual(row["family"], "F1")

    def test_no_key_index_argument_is_backward_compatible(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1)
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["join_status"], "no_record")


class BuildCorpusKeyIndexTest(unittest.TestCase):
    def test_indexes_by_book_kind_data_key(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment")
            os.makedirs(book_dir)
            record = {
                "data": {
                    "key": "Book of Harms",
                    "raw_tokens": [
                        {"key": "DESC", "value": "irrelevant, not a formula token"},
                        {"key": "BONUS", "value": "VAR|Foo|WIS"},
                    ],
                },
                "source": {"path": "um_equip_general.lst", "line": 16},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)
            with open(os.path.join(book_dir, "LICENSE.json"), "w") as fh:
                json.dump({"data": {"key": "SHOULD_NOT_APPEAR", "raw_tokens": []}}, fh)

            index = SL.build_corpus_key_index(tmp, books={"test_book"})
            key = ("test_book", "equipment", "Book of Harms")
            self.assertIn(key, index)
            self.assertEqual(len(index[key]), 1)
            self.assertEqual(index[key][0]["key"], "BONUS")

    def test_record_with_no_data_key_is_not_indexed(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment")
            os.makedirs(book_dir)
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump({"data": {"raw_tokens": []}, "source": {"path": "f.lst", "line": 1}}, fh)
            index = SL.build_corpus_key_index(tmp, books={"test_book"})
            self.assertEqual(index, {})


class BuildLedgerTest(unittest.TestCase):
    def test_unclassified_count_is_always_zero_for_nonempty_population(self):
        units = [
            _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1),
            _unit("b:spell:y", "spell", "b", "not-started", "static", "f.lst", 1),
        ]
        index = {("b", "spell", "f.lst", 1): [{"key": "BONUS", "value": "VAR|Foo|WIS"}]}
        ledger = SL.build_ledger(units, index)
        self.assertEqual(ledger["unclassified_count"], 0)
        self.assertEqual(ledger["population"], 2)
        self.assertIn(SL.FAMILY_F0_NO_FORMULA, ledger["families"])
        self.assertIn("F3", ledger["families"])

    def test_join_status_counts_reconciles_to_population(self):
        """decisions.md §14b: a bare unclassified_count/family total hides
        that F0 conflates no_record ("join found nothing") and
        no_formula_tokens ("found a record with no DEFINE/BONUS"). This
        aggregate must be present and must sum back to the population."""
        units = [
            _unit("b:spell:no-record", "spell", "b", "not-started", "static", "missing.lst", 1),
            _unit("b:spell:no-formula", "spell", "b", "not-started", "static", "empty.lst", 1),
            _unit("b:spell:matched", "spell", "b", "not-started", "static", "f.lst", 1),
        ]
        index = {
            ("b", "spell", "empty.lst", 1): [],
            ("b", "spell", "f.lst", 1): [{"key": "BONUS", "value": "VAR|Foo|WIS"}],
        }
        ledger = SL.build_ledger(units, index)
        jsc = ledger["join_status_counts"]
        self.assertEqual(jsc.get("no_record"), 1)
        self.assertEqual(jsc.get("no_formula_tokens"), 1)
        self.assertEqual(jsc.get("matched"), 1)
        self.assertEqual(sum(jsc.values()), ledger["population"])


class FailClosedOnEmptyTest(unittest.TestCase):
    """AT-32-G1-002: the shape ledger fails closed on empty predicates."""

    def _run(self, inventory_path):
        return subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "shape_ledger.py"), "--inventory", inventory_path],
            capture_output=True,
            text=True,
        )

    def test_dev_null_reports_no_coverage_and_nonzero_exit(self):
        result = self._run("/dev/null")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no coverage", (result.stdout + result.stderr).lower())

    def test_empty_units_list_reports_no_coverage_and_nonzero_exit(self):
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump({"units": []}, fh)
            path = fh.name
        try:
            result = self._run(path)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no coverage", (result.stdout + result.stderr).lower())
        finally:
            os.unlink(path)

    def test_all_done_units_also_reports_no_coverage(self):
        # not_done_population() filters DONE units to zero -- proves the
        # fail-closed path is reachable via real filtering, not only via a
        # literally-empty units list.
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump(
                {
                    "units": [
                        _unit(
                            "b:equipment:e", "equipment", "b", "literal-verified", "static", "f.lst", 1
                        )
                    ]
                },
                fh,
            )
            path = fh.name
        try:
            result = self._run(path)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no coverage", (result.stdout + result.stderr).lower())
        finally:
            os.unlink(path)


class GateEndToEndTest(unittest.TestCase):
    """Runs the tool end to end against a small synthetic inventory +
    corpus tree, mirroring AT-32-G1-001's verification command shape."""

    def test_end_to_end_produces_zero_unclassified_and_expected_families(self):
        with tempfile.TemporaryDirectory() as tmp:
            corpus_root = os.path.join(tmp, "corpus")
            book_dir = os.path.join(corpus_root, "book_a", "spell")
            os.makedirs(book_dir)
            records = {
                "unit_flat.json": {
                    "data": {"raw_tokens": [{"key": "BONUS", "value": "SKILL|Perception|2|TYPE=Racial"}]},
                    "source": {"path": "x/book_a_spells.lst", "line": 1},
                },
                "unit_lvl.json": {
                    "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WizardLVL/2"}]},
                    "source": {"path": "x/book_a_spells.lst", "line": 2},
                },
                "unit_noformula.json": {
                    "data": {"raw_tokens": []},
                    "source": {"path": "x/book_a_spells.lst", "line": 3},
                },
            }
            for fname, rec in records.items():
                with open(os.path.join(book_dir, fname), "w") as fh:
                    json.dump(rec, fh)

            inventory = {
                "units": [
                    _unit("book_a:spell:flat", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 1),
                    _unit("book_a:spell:lvl", "spell", "book_a", "not-started", "static", "book_a_spells.lst", 2),
                    _unit(
                        "book_a:spell:noformula",
                        "spell",
                        "book_a",
                        "not-started",
                        "static",
                        "book_a_spells.lst",
                        3,
                    ),
                    _unit("book_a:spell:unreached", "spell", "book_a", "not-started", "static", "missing.lst", 99),
                ]
            }
            inv_path = os.path.join(tmp, "inventory.json")
            with open(inv_path, "w") as fh:
                json.dump(inventory, fh)
            out_path = os.path.join(tmp, "ledger.json")

            result = subprocess.run(
                [
                    sys.executable,
                    os.path.join(REPO_ROOT, "scripts", "shape_ledger.py"),
                    "--inventory",
                    inv_path,
                    "--corpus-root",
                    corpus_root,
                    "--output",
                    out_path,
                ],
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            with open(out_path) as fh:
                ledger = json.load(fh)
            self.assertEqual(ledger["unclassified_count"], 0)
            self.assertEqual(ledger["population"], 4)
            self.assertEqual(ledger["families"]["F1"]["count"], 1)
            self.assertEqual(ledger["families"]["F2"]["count"], 1)
            self.assertEqual(ledger["families"][SL.FAMILY_F0_NO_FORMULA]["count"], 2)


if __name__ == "__main__":
    unittest.main()
