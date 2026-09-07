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
import pi_scrub as PS  # noqa: E402


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

    def test_bonus_skill_percent_list_with_no_magnitude_field_is_implicit_flat_one(self):
        """T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21):
        `ultimate_campaign:trait:trait_harvester`'s `BONUS:SKILL|%LIST` --
        oracle-verified (`uca_abilities_traits.lst:198`) to be the ONLY
        occurrence of this exact 2-field shorthand anywhere in the pinned
        corpus. PCGen omits the magnitude field entirely for a
        CHOOSE:SKILL-linked flat trait bonus; the record's own `ASPECT`
        token ("+1 trait bonus...") and `DESC` confirm the omitted
        magnitude is a flat +1, matching Pathfinder's universal trait
        skill-bonus convention this shorthand always implies.

        Scoped narrowly to `%LIST` as the 2nd field specifically -- a bare
        `SKILL|<real skill name>` with no magnitude (the pre-existing
        `test_bonus_too_short_returns_none` case above, a genuinely
        malformed/incomplete token) must still return `None`, never `1`."""
        self.assertEqual(SL.extract_formula_segment("BONUS", "SKILL|%LIST"), "1")

    def test_bonus_skill_named_target_with_no_magnitude_still_returns_none(self):
        """Sibling guard: the implicit-1 rule is scoped to the `%LIST`
        shorthand only -- a 2-field `SKILL|<name>` bonus (any name other
        than `%LIST`) is a genuinely malformed/short token, not this
        shorthand, and must stay `None`."""
        self.assertIsNone(SL.extract_formula_segment("BONUS", "SKILL|Stealth"))


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

    def test_equipment_modifier_records_nested_under_equipment_equipmods_index_as_equipment_modifier(self):
        """`decisions.md §20` t9-onboarding straggler wave, discovery-forward
        from the concurrent `epic-6-kind-trait_cycle-2` kind-aware-join fix:
        EVERY `equipment_modifier` record in this corpus lives at
        `<book>/equipment/equipmods/*.json` (`equipment_gap.rs::generate()`'s
        own write path, `book_out.join("equipmods")` where `book_out =
        <book>/equipment` -- confirmed corpus-wide, zero bare
        `equipment_modifier/` directories exist anywhere). The kind-aware
        join's `parts[0]`-only derivation reads this as kind `"equipment"`,
        not `"equipment_modifier"` -- a real, corpus-wide regression (1,003
        `equipment_modifier` units, essentially the WHOLE kind, went
        `no_record` the moment the join became kind-aware), not the genuine
        `equipment_modifier`->`equipment` COLLISION shape the kind-aware fix
        was built to catch (a record ACTUALLY filed directly under
        `equipment/*.json` with no `equipmods` subdirectory, wrongly
        satisfying an `equipment_modifier` unit's old kind-blind join).
        `equipmods` is the ONLY `equipment/<X>/` subdirectory that means a
        different kind -- `arms_armor`/`general`/`magic_items` are ordinary
        `equipment`-kind sub-groupings (confirmed corpus-wide: exactly these
        4 subdirectory names exist under any book's `equipment/`), so a
        directory name check, not a blanket two-level rule, is required."""
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment", "equipmods")
            os.makedirs(book_dir)
            record = {
                "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                "source": {"path": "test_equipmods.lst", "line": 5},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)

            index = SL.build_corpus_index(tmp, books={"test_book"})
            key = ("test_book", "equipment_modifier", "test_equipmods.lst", 5)
            self.assertIn(key, index, f"expected key not in index: {list(index.keys())}")

    def test_equipment_records_directly_under_equipment_still_index_as_equipment(self):
        """A record filed directly under `equipment/*.json` (no `equipmods`
        subdirectory) still indexes as plain `equipment` -- the fix is
        specific to the `equipmods` directory name, not a blanket "always
        descend one more level" rule that would misclassify ordinary
        `equipment` sub-groupings like `arms_armor`/`general`/`magic_items`."""
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment")
            os.makedirs(book_dir)
            record = {
                "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                "source": {"path": "test_equip.lst", "line": 5},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)

            index = SL.build_corpus_index(tmp, books={"test_book"})
            key = ("test_book", "equipment", "test_equip.lst", 5)
            self.assertIn(key, index, f"expected key not in index: {list(index.keys())}")

    def test_equipment_sub_grouping_directory_still_indexes_as_equipment(self):
        """A real ordinary `equipment`-kind sub-grouping directory (e.g.
        `arms_armor`) must NOT be treated as denoting a different kind --
        only the specific `equipmods` name does."""
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment", "arms_armor")
            os.makedirs(book_dir)
            record = {
                "data": {"raw_tokens": [{"key": "BONUS", "value": "VAR|Foo|WIS"}]},
                "source": {"path": "test_arms.lst", "line": 5},
            }
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump(record, fh)

            index = SL.build_corpus_index(tmp, books={"test_book"})
            key = ("test_book", "equipment", "test_arms.lst", 5)
            self.assertIn(key, index, f"expected key not in index: {list(index.keys())}")

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

    # decisions.md §27a / kanban.md row 17: `f0_reached_by` must tell a
    # genuinely-derived F0 apart from a placeholder wearing F0's label.
    def test_no_record_f0_reached_by_is_engine_does_not_hold(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1)
        row = SL.classify_unit(unit, corpus_index={})
        self.assertEqual(row["f0_reached_by"], "engine_does_not_hold")
        self.assertFalse(row["pi_redacted_formula"])

    def test_no_formula_tokens_f0_reached_by_is_measured_empty(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "spell", "f.lst", 1): []}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["f0_reached_by"], "measured_empty")
        self.assertFalse(row["pi_redacted_formula"])

    def test_matched_real_family_has_no_f0_reached_by(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "spell", "f.lst", 1): [{"key": "BONUS", "value": "VAR|Foo|2"}]}  # F1
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], "F1")
        self.assertIsNone(row["f0_reached_by"])

    def test_matched_with_unparseable_token_is_fallthrough_not_measured_empty(self):
        """A record CAN be found (join succeeds) and still carry a
        DEFINE/BONUS token this classifier cannot extract a segment from
        (too few `|`-fields). That is a placeholder F0 -- `nothing else
        matched` -- never `measured_empty`, which is reserved for a record
        that genuinely carries zero formula tokens at all."""
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "spell", "f.lst", 1): [{"key": "DEFINE", "value": "OnlyOneField"}]}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["join_status"], "matched")
        self.assertEqual(row["f0_reached_by"], "fallthrough")
        self.assertFalse(row["pi_redacted_formula"])

    def test_pi_redacted_formula_value_is_measured_pi_redacted_and_flagged(self):
        """T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21):
        a record whose BONUS/DEFINE VALUE is the redaction marker itself
        (`decisions.md §24b`: the record was renamed, its mechanical value
        blanket-redacted alongside NAME/DESC) is a genuinely-measured
        answer -- "PI, cannot be shipped as formula" -- NOT a fallthrough
        placeholder (`decisions.md §27a`: "if the value genuinely carries
        PI, it stays redacted -- but then it is not a fallthrough
        placeholder, it is a correctly-measured redacted value"). This
        distinguishes it from an ordinary parse failure, which stays
        `fallthrough` (see the next test)."""
        unit = _unit("b:trait:x", "trait", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "trait", "f.lst", 1): [{"key": "BONUS", "value": PS.REDACTED_PI_MARKER}]}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["family"], SL.FAMILY_F0_NO_FORMULA)
        self.assertEqual(row["f0_reached_by"], "measured_pi_redacted")
        self.assertTrue(row["pi_redacted_formula"])

    def test_non_pi_parse_failure_stays_fallthrough_not_measured_pi_redacted(self):
        """Sanity/mutation guard for the split above: a genuine parse
        failure (malformed token, no redaction marker) must stay
        `fallthrough` -- row 17's real, actionable population -- and must
        NEVER be swept into `measured_pi_redacted` merely because it is
        also F0."""
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "f.lst", 1)
        index = {("b", "spell", "f.lst", 1): [{"key": "DEFINE", "value": "OnlyOneField"}]}
        row = SL.classify_unit(unit, index)
        self.assertEqual(row["f0_reached_by"], "fallthrough")
        self.assertFalse(row["pi_redacted_formula"])

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

    def test_cross_book_fallback_matches_when_book_scoped_fallback_also_misses(self):
        """SD-32 t9-onboarding straggler wave (`decisions.md §20`): a book's
        `.MOD`/widening row deliberately does not re-declare a spell/trait/
        equipment-modifier that ALREADY exists under a DIFFERENT book's own
        citation -- `ingest_spells.rs`'s own `already_ingested_oa`/
        `already_ingested_uc` is the documented example ("a handful of rows
        exist only to widen an existing spell's class access"). Real case:
        `occult_adventures:spell:repulsion` cites `oa_spells.lst:464`, but
        "Repulsion" is never (re-)ingested for `occult_adventures` -- the
        real, already-shipped record lives under `crb`'s own citation. Both
        the primary (book, source_file, source_line) join AND the same-book
        (book, kind, key) fallback miss (different book); the THIRD tier,
        keyed on (kind, key) alone across every book, recovers it."""
        unit = _unit(
            "occult_adventures:spell:repulsion",
            "spell",
            "occult_adventures",
            "not-started",
            "static",
            "oa_spells.lst",
            464,
            corpus_key="Repulsion",
        )
        cross_book_key_index = {("spell", "Repulsion"): ("crb", [{"key": "DEFINE", "value": "Foo|0"}])}
        row = SL.classify_unit(unit, corpus_index={}, key_index={}, cross_book_key_index=cross_book_key_index)
        self.assertEqual(row["join_status"], "matched")
        self.assertEqual(row["family"], "F1")

    def test_cross_book_fallback_never_fires_when_the_same_book_key_index_already_covers_it(self):
        """The cross-book tier is a LAST resort -- when the same-book
        `key_index` fallback already resolves it, the coarser book-agnostic
        tier is never consulted."""
        unit = _unit(
            "b:equipment:x", "equipment", "b", "not-started", "static", "overlay.lst", 1, corpus_key="Widget"
        )
        key_index = {("b", "equipment", "Widget"): []}
        cross_book_key_index = {("equipment", "Widget"): ("other_book", [{"key": "DEFINE", "value": "Foo|0"}])}
        row = SL.classify_unit(
            unit, corpus_index={}, key_index=key_index, cross_book_key_index=cross_book_key_index
        )
        self.assertEqual(row["join_status"], "no_formula_tokens")

    def test_cross_book_fallback_declines_an_ambiguous_key_marked_none(self):
        """`build_cross_book_key_index` marks a (kind, key) pair `None` when
        two DIFFERENT books' records collide on the identical key with
        different content -- an ambiguous cross-book match must never guess
        which book a third book's reference means (`decisions.md §1a`:
        under-include rather than invent)."""
        unit = _unit(
            "third_book:spell:x", "spell", "third_book", "not-started", "static", "f.lst", 1, corpus_key="Widget"
        )
        cross_book_key_index = {("spell", "Widget"): None}
        row = SL.classify_unit(unit, corpus_index={}, key_index={}, cross_book_key_index=cross_book_key_index)
        self.assertEqual(row["join_status"], "no_record")

    def test_cross_book_fallback_never_fires_across_a_different_kind(self):
        unit = _unit(
            "third_book:spell:x", "spell", "third_book", "not-started", "static", "f.lst", 1, corpus_key="Widget"
        )
        cross_book_key_index = {("equipment", "Widget"): ("other_book", [{"key": "DEFINE", "value": "Foo|0"}])}
        row = SL.classify_unit(unit, corpus_index={}, key_index={}, cross_book_key_index=cross_book_key_index)
        self.assertEqual(row["join_status"], "no_record")

    def test_no_cross_book_key_index_argument_is_backward_compatible(self):
        unit = _unit("b:spell:x", "spell", "b", "not-started", "static", "missing.lst", 1, corpus_key="Widget")
        row = SL.classify_unit(unit, corpus_index={}, key_index={})
        self.assertEqual(row["join_status"], "no_record")


class BuildCrossBookKeyIndexTest(unittest.TestCase):
    def test_indexes_by_kind_key_across_books(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "crb", "spell")
            os.makedirs(book_dir)
            record = {
                "data": {"key": "Repulsion", "raw_tokens": [{"key": "DEFINE", "value": "Foo|0"}]},
                "source": {"path": "crb_spells.lst", "line": 217},
            }
            with open(os.path.join(book_dir, "repulsion.json"), "w") as fh:
                json.dump(record, fh)

            index = SL.build_cross_book_key_index(tmp)
            self.assertIn(("spell", "Repulsion"), index)
            book, tokens = index[("spell", "Repulsion")]
            self.assertEqual(book, "crb")
            self.assertEqual(tokens, [{"key": "DEFINE", "value": "Foo|0"}])

    def test_two_different_books_with_the_same_key_and_different_content_is_ambiguous(self):
        with tempfile.TemporaryDirectory() as tmp:
            for book, tokens in (("book_a", []), ("book_b", [{"key": "DEFINE", "value": "Foo|0"}])):
                book_dir = os.path.join(tmp, book, "spell")
                os.makedirs(book_dir)
                record = {
                    "data": {"key": "Collides", "raw_tokens": tokens},
                    "source": {"path": "f.lst", "line": 1},
                }
                with open(os.path.join(book_dir, "collides.json"), "w") as fh:
                    json.dump(record, fh)

            index = SL.build_cross_book_key_index(tmp)
            self.assertIsNone(index[("spell", "Collides")])

    def test_two_different_books_with_the_same_key_and_identical_content_is_not_ambiguous(self):
        """Two books legitimately restating the identical record (same
        formula tokens) is not the collision hazard this guards against --
        only content DIVERGENCE is ambiguous."""
        with tempfile.TemporaryDirectory() as tmp:
            for book in ("book_a", "book_b"):
                book_dir = os.path.join(tmp, book, "spell")
                os.makedirs(book_dir)
                record = {
                    "data": {"key": "Same", "raw_tokens": [{"key": "DEFINE", "value": "Foo|0"}]},
                    "source": {"path": "f.lst", "line": 1},
                }
                with open(os.path.join(book_dir, "same.json"), "w") as fh:
                    json.dump(record, fh)

            index = SL.build_cross_book_key_index(tmp)
            self.assertIsNotNone(index[("spell", "Same")])

    def test_record_with_no_data_key_is_not_indexed(self):
        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "test_book", "equipment")
            os.makedirs(book_dir)
            with open(os.path.join(book_dir, "unit.json"), "w") as fh:
                json.dump({"data": {"raw_tokens": []}, "source": {"path": "f.lst", "line": 1}}, fh)
            index = SL.build_cross_book_key_index(tmp)
            self.assertEqual(index, {})


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

    def test_f0_breakdown_separates_measured_from_fallthrough_and_moves_on_mutation(self):
        """decisions.md §27a / kanban.md row 17: the census this feeds must
        be able to go RED when a genuinely-derived unit is mutated to look
        PI-redacted, and back to GREEN on revert -- proving the count is
        live, not a static label.

        T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21):
        a PI-redacted formula value is `measured_pi_redacted` (a real
        answer), never `fallthrough` (a placeholder) -- so the fixture's
        one genuinely-redacted `trait` unit counts there, and the mutated
        spell (also PI-redacted) joins it, NOT `fallthrough`. A separate
        `fallthrough` unit (a genuine, non-PI parse failure) proves that
        bucket still moves independently."""
        units = [
            _unit("b:spell:no-record", "spell", "b", "not-started", "static", "missing.lst", 1),
            _unit("b:spell:empty", "spell", "b", "not-started", "static", "empty.lst", 1),
            _unit("b:spell:real", "spell", "b", "not-started", "static", "real.lst", 1),
            _unit("b:trait:redacted", "trait", "b", "not-started", "static", "red.lst", 1),
            _unit("b:feat:malformed", "feat", "b", "not-started", "static", "bad.lst", 1),
        ]
        base_index = {
            ("b", "spell", "empty.lst", 1): [],
            ("b", "spell", "real.lst", 1): [{"key": "BONUS", "value": "VAR|Foo|2"}],  # F1, genuinely derived
            ("b", "trait", "red.lst", 1): [{"key": "BONUS", "value": PS.REDACTED_PI_MARKER}],
            ("b", "feat", "bad.lst", 1): [{"key": "DEFINE", "value": "OnlyOneField"}],  # genuine parse failure
        }
        ledger = SL.build_ledger(units, base_index)
        f0b = ledger["f0_breakdown"]
        self.assertEqual(f0b.get("engine_does_not_hold"), 1)
        self.assertEqual(f0b.get("measured_empty"), 1)
        self.assertEqual(f0b.get("measured_pi_redacted"), 1)
        self.assertEqual(f0b.get("fallthrough"), 1)
        self.assertEqual(ledger["f0_fallthrough_pi_redacted"], 0)
        self.assertEqual(ledger["families"]["F1"]["count"], 1)

        # RED: mutate the genuinely-derived unit's own record so it now
        # looks PI-redacted (its BONUS value becomes the PI-redaction
        # marker, same shape as the real 20-unit finding this cycle
        # re-derived) -- the measured_pi_redacted count MUST move, and
        # `fallthrough` MUST stay untouched by it.
        mutated_index = dict(base_index)
        mutated_index[("b", "spell", "real.lst", 1)] = [{"key": "BONUS", "value": PS.REDACTED_PI_MARKER}]
        mutated_ledger = SL.build_ledger(units, mutated_index)
        self.assertEqual(mutated_ledger["f0_breakdown"].get("measured_pi_redacted"), 2)
        self.assertEqual(mutated_ledger["f0_breakdown"].get("fallthrough"), 1)
        self.assertEqual(mutated_ledger["f0_fallthrough_pi_redacted"], 0)
        self.assertNotIn("F1", mutated_ledger["families"])

        # GREEN: revert -- back to the original, un-mutated counts.
        reverted_ledger = SL.build_ledger(units, base_index)
        self.assertEqual(reverted_ledger["f0_breakdown"].get("measured_pi_redacted"), 1)
        self.assertEqual(reverted_ledger["f0_breakdown"].get("fallthrough"), 1)
        self.assertEqual(reverted_ledger["families"]["F1"]["count"], 1)


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
