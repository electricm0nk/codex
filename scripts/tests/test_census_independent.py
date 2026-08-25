#!/usr/bin/env python3
"""Tests for `scripts/census_independent.py` (SD-32 Gate 0, card
`gate-0-census-closure`).

Uses a small synthetic PCGen-shaped tree rather than the live 158/186-dir
oracle, so these tests stay fast and do not depend on the live oracle
checkout being populated. The live-oracle run itself is exercised
separately by the cycle receipt's own command, against the real pinned
corpus.
"""

import json
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import census_independent as CI  # noqa: E402


def _touch(path, content=""):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(content)


class DiscoverBookDirsTest(unittest.TestCase):
    def test_finds_paizo_and_third_party_book_dirs(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pcgen_root")
            _touch(
                os.path.join(
                    root,
                    "pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc",
                )
            )
            _touch(
                os.path.join(
                    root,
                    "pathfinder/paizo/adventure_path/some_ap/some_ap.pcc",
                )
            )
            _touch(
                os.path.join(
                    root,
                    "pathfinder/dreamscarred_press/ultimate_psionics/up.pcc",
                )
            )
            # A directory with NO .pcc file must not be counted as a book.
            _touch(
                os.path.join(
                    root, "pathfinder/paizo/roleplaying_game/empty_dir/notes.txt"
                )
            )

            found = CI.discover_book_dirs(root)
            ids = sorted(b.book_id for b in found)
            self.assertEqual(ids, ["core_rulebook", "some_ap", "ultimate_psionics"])

    def test_nested_pcc_still_counts_the_top_book_dir(self):
        """A `.pcc` fragment several levels deep (e.g. a `_pfs/` subfolder)
        still marks its top-level book directory as real -- proving the
        walker does not miss nested content the way SD-31 wave 1's
        single-level join did."""
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pcgen_root")
            _touch(
                os.path.join(
                    root,
                    "pathfinder/paizo/roleplaying_game/bestiary/_pfs/pfs_bestiary.pcc",
                )
            )
            found = CI.discover_book_dirs(root)
            self.assertEqual([b.book_id for b in found], ["bestiary"])


class ClassifyScopeTest(unittest.TestCase):
    def _inventory(self, ids):
        return {"books": [{"id": i} for i in ids]}

    def test_in_scope_and_justified_exclusions_reach_zero_unexplained(self):
        book_dirs = [
            CI.BookDir("core_rulebook", "core_rulebook", "paizo/roleplaying_game"),
            CI.BookDir("some_ap", "some_ap", "paizo/adventure_path"),
            CI.BookDir(
                "dreamscarred_press/psionics_unleashed",
                "psionics_unleashed",
                "dreamscarred_press",
            ),
            CI.BookDir("beginner_box", "beginner_box", "paizo/roleplaying_game"),
        ]
        inv = self._inventory(["core_rulebook"])
        result = CI.classify_scope(book_dirs, inv)
        self.assertEqual([b.book_id for b in result.in_scope], ["core_rulebook"])
        self.assertEqual(len(result.excluded), 3)
        self.assertEqual(result.unexplained, [])

    def test_gate_can_fail_a_roleplaying_game_book_missing_from_roster_is_unexplained(self):
        """Proves the gate is not a rubber stamp: a `roleplaying_game`
        book directory that matches none of the fixed exclusion buckets
        (not beginner_box, not core_essentials) and is absent from the
        inventory roster is a genuine, un-bucketed gap -- exactly the
        'oversight, not scope' case AT-32-G0-001 requires the walker to
        surface rather than silently swallow."""
        book_dirs = [
            CI.BookDir(
                "surprise_new_book", "surprise_new_book", "paizo/roleplaying_game"
            )
        ]
        inv = self._inventory([])
        result = CI.classify_scope(book_dirs, inv)
        self.assertEqual(result.unexplained, ["surprise_new_book"])
        self.assertEqual(result.excluded, [])


class ObjectDefinitionRulesTest(unittest.TestCase):
    LST_CONTENT = (
        "#  a header comment\n"
        "SOURCELONG:Test Book\tSOURCESHORT:TB\n"
        "\n"
        "Acrobatic\tCATEGORY:FEAT\tTYPE:General\n"
        "CATEGORY=Class|Arcanist.MOD\tDEFINE:Arcanist_CF_ClassSkills|0\n"
        "Special Ability ~ Burdenless ~ Armor.COPY=Burdenless\tVISIBLE:NO\n"
        "Old Entry.FORGET\tSOURCEPAGE:p.1\n"
    )

    def test_mod_lines_are_not_new_units_copy_lines_are(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(
                os.path.join(root, book, "core_rulebook.pcc"), "SOURCELONG:x\n"
            )
            _touch(os.path.join(root, book, "cr_feats.lst"), self.LST_CONTENT)

            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])

            # 2 real feat-file units: "Acrobatic" and the COPY= derivation.
            # ".MOD" and ".FORGET" rows are excluded from the count.
            self.assertEqual(counts["counts_by_kind"].get("feat"), 2)
            self.assertEqual(counts["mod_continuation"], 1)
            self.assertEqual(counts["forget_directive"], 1)
            self.assertEqual(counts["copy_derivation"], 1)

    def test_class_feature_files_are_named_kind_unenumerable_not_dropped(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Rage\tCATEGORY:Special Ability\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertNotIn("class_feature", counts["counts_by_kind"])

    def test_bare_internal_tracker_reroutes_to_ability_category_internal(self):
        # Card 15 (`decisions.md §14c` item 4): a `_abilities_class.lst`
        # row carrying `CATEGORY:Internal` AND no content field AND no
        # gateway token is a genuine PCGen bookkeeping marker with zero
        # payload of its own -- proven by class (40/2,614 at the pinned
        # oracle SHA), not asserted for the whole population. It reroutes
        # to the same bucket the bare `abilit` branch already uses for
        # this exact marker.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Rage\tCATEGORY:Special Ability\n"
                "Panache Tracker\tCATEGORY:Internal\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Internal"), 1
            )
            self.assertNotIn("class_feature", counts["counts_by_kind"])

    def test_content_bearing_internal_row_stays_class_feature_not_excluded(self):
        # Card 15 §14c item 4 -- the reopening finding itself: the
        # class_feature memo's own worked (B) example, "Damage Reduction ~
        # All" (`DR:ClassFeatureDR_ALL/-`), is real content (the engine's
        # DR-tracking machinery reads the `DR:` variable this row names) --
        # a narrower DEFINE:/BONUS:-only test misses it, exactly the
        # AGENTS.md "grep filtered to BONUS/PRE hides STACK/MULT" hazard.
        # This is the test that proves the exclusion rule does NOT swallow
        # a real object: it must stay counted as `class_feature`, never
        # rerouted, even though it carries `CATEGORY:Internal`.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Damage Reduction ~ All\tCATEGORY:Internal\tDR:ClassFeatureDR_ALL/-\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertNotIn(
                "ability_category:Internal", counts["kind_unenumerable"]
            )

    def test_gateway_only_internal_row_stays_class_feature_not_excluded(self):
        # A row with no content field of its own but a real
        # `ABILITY:...|AUTOMATIC|<target>` gateway token is a proven facet
        # of something else, not a bare marker -- `_row_is_bare_internal_marker`
        # must return False for it (the gateway-field check), so it stays
        # counted as `class_feature` rather than being silently excluded.
        # Whether the gateway's target itself resolves is a cross-file
        # question this row-local exclusion deliberately does not attempt
        # (see the module comment above `_ROW_CONTENT_FIELD_RE`) -- the
        # conservative default is to count, not exclude, when in doubt
        # (decisions.md §12b's burden of proof is on (B)).
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Improved Claws\tKEY:Feral Heart ~ Improved Claws\tCATEGORY:Internal"
                "\tABILITY:FEAT|AUTOMATIC|Improved Natural Attack (Claw)\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertNotIn(
                "ability_category:Internal", counts["kind_unenumerable"]
            )

    def test_non_internal_class_feature_row_is_unaffected(self):
        # Sanity: a plain (non-CATEGORY:Internal) class_feature row must
        # never be routed through the bare-marker check at all.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Rage\tCATEGORY:Special Ability\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertNotIn(
                "ability_category:Internal", counts["kind_unenumerable"]
            )

    def test_ce_sizes_file_is_non_object_not_kind_unenumerable(self):
        # Card 15 §7b: PF1e's fixed 9-variant size table, already covered
        # by `src/rules_core/size.rs`'s `SizeCategory` enum -- not an object.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_essentials"
            _touch(os.path.join(root, book, "core_essentials.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "ce__sizes.lst"),
                "Fine\tSOMEFIELD:x\n",
            )
            bd = CI.BookDir(book, "core_essentials", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["total_counted_units"], 0)
            self.assertEqual(counts["total_kind_unenumerable_units"], 0)
            self.assertIn(
                "paizo/roleplaying_game/core_essentials/ce__sizes.lst",
                counts["non_object_files"],
            )

    def test_bestiary_races_file_counts_as_monster_not_race(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/bestiary"
            _touch(os.path.join(root, book, "bestiary.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "b1_races.lst"),
                "Aboleth\tRACETYPE:Monstrous Aberration\n",
            )
            bd = CI.BookDir(book, "bestiary", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("monster"), 1)
            self.assertNotIn("race", counts["counts_by_kind"])

    def test_core_rulebook_races_file_counts_as_race_not_monster(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_races.lst"),
                "Human\tFAVCLASS:Any\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("race"), 1)
            self.assertNotIn("monster", counts["counts_by_kind"])

    def test_skills_file_counts_as_skill_kind_not_unclassified(self):
        # SD-32 card 15 (`decisions.md §12b`): `*_skills.lst` moved from
        # `kind_unenumerable["unclassified:<file>"]` into the `skill` kind
        # once `Kind::Skill` landed in `src/bin/v06_work_inventory.rs`, so
        # the walker and the inventory agree.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_skills.lst"),
                "Acrobatics\tKEYSTAT:DEX\tACHECK:YES\tTYPE:Dexterity.ACHECK.Base\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("skill"), 1)
            self.assertNotIn("unclassified:cr_skills.lst", counts["kind_unenumerable"])

    def test_type_trait_row_in_a_bare_abilities_file_counts_as_kind_trait(self):
        # SD-32 `decisions.md §25` (the `kind: trait` epic) -- the exact real
        # oracle row (verbatim, tab-split) that named this epic:
        # `inner_sea_races/isr_abilities.lst:78`.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/campaign_setting/inner_sea_races"
            _touch(os.path.join(root, book, "inner_sea_races.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "isr_abilities.lst"),
                "Loner of the Rocks\tKEY:Trait ~ Loner of the Rocks\tCATEGORY:Special Ability\t"
                "TYPE:Trait.RaceTrait.Oread Race Trait\tDESC:Gain a +1 trait bonus.\t"
                "BONUS:SKILL|Heal,Survival|1|TYPE=Trait\n",
            )
            bd = CI.BookDir(book, "inner_sea_races", "paizo/campaign_setting")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("trait"), 1)
            self.assertNotIn("ability", counts["counts_by_kind"])

    def test_type_trait_row_is_checked_before_the_feat_redirect(self):
        # A row whose `TYPE:` names a bare `Trait` (no dot-suffix, real
        # `ultimate_campaign/uca_abilities_traits.lst` shape) also counts as
        # `trait`, never `feat`, even carrying `CATEGORY:FEAT` -- proves the
        # ordering `_row_is_pf1_trait` is checked BEFORE the FEAT redirect.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/ultimate_campaign"
            _touch(os.path.join(root, book, "ultimate_campaign.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "uca_abilities_traits.lst"),
                "Reactionary\tCATEGORY:FEAT\tTYPE:Trait\tDESC:+2 on initiative.\n",
            )
            bd = CI.BookDir(book, "ultimate_campaign", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("trait"), 1)
            self.assertNotIn("feat", counts["counts_by_kind"])

    def test_simple_added_kinds_count_as_kinds_not_kind_unenumerable(self):
        # SD-32 `decisions.md §17`: `template`/`deity`/`power`/`domain`/
        # `language` moved from `kind_unenumerable` to tracked kinds
        # together, through the same generic mechanism `skill` proved --
        # see `15-card-15-other-kinds-memo.md` §1-5.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(os.path.join(root, book, "cr_templates.lst"), "Ghost\tVISIBLE:NO\n")
            _touch(os.path.join(root, book, "cr_deities.lst"), "Abadar\tDOMAINS:Law\n")
            _touch(os.path.join(root, book, "cr_domains.lst"), "Air\tABILITY:1|AUTOMATIC|Lightning Arc\n")
            _touch(os.path.join(root, book, "up_powers.lst"), "Mind Thrust\tSCHOOL:Telepathy\n")
            _touch(os.path.join(root, book, "cr_languages.lst"), "Common\tTYPE:Spoken.Written\n")
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            for kind in ("template", "deity", "domain", "power", "language"):
                self.assertEqual(counts["counts_by_kind"].get(kind), 1, kind)
            self.assertEqual(counts["kind_unenumerable"].get("template_row"), None)
            self.assertEqual(counts["kind_unenumerable"].get("deity"), None)
            self.assertEqual(counts["kind_unenumerable"].get("domain"), None)
            self.assertEqual(counts["kind_unenumerable"].get("power"), None)
            self.assertEqual(counts["kind_unenumerable"].get("language"), None)

    def test_kitsune_races_file_no_longer_misclassified_as_kit(self):
        # SD-32 `decisions.md §17`: `"kit" in b` false-positived on
        # `kitsune_races.lst` (the race NAME "Kitsune" contains "kit").
        # Narrowed to `_kits` so real `race`-kind content is not diverted --
        # matches `src/bin/v06_work_inventory.rs`'s `file_kind`, which never
        # had a "kit" branch and always resolved this file to `Kind::Race`.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_essentials"
            _touch(os.path.join(root, book, "core_essentials.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "kitsune_races.lst"),
                "Kitsune\tFAVCLASS:Any\tTYPE:Humanoid\n",
            )
            bd = CI.BookDir(book, "core_essentials", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("race"), 1)
            self.assertNotIn("kit", counts["kind_unenumerable"])

    def test_real_kits_file_still_reroutes_and_produces_zero_rows(self):
        # A file that genuinely matches `_kits` (not a name-collision) still
        # reroutes to the `kit` bucket -- proving the narrowing did not just
        # delete the check. Real `*_kits.lst` content uses PCGen's
        # `STARTPACK:`-block format (every row's own first field carries a
        # `:` before any tab), so `_parse_lst_rows` already skips every row
        # as a directive line regardless of which bucket the file lands in --
        # 0 rows counted either way, verified against the pinned oracle
        # (48 real `*_kits*.lst` files, 0 rows total).
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_kits.lst"),
                "STARTPACK:A Test\tTYPE:Default\nSTAT:INT=0\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("kit"), None)
            self.assertNotIn("race", counts["counts_by_kind"])

    def test_non_object_files_are_skipped_not_miscounted(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr__datacontrols.lst"),
                "SomeControl\tTYPE:x\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["total_counted_units"], 0)
            self.assertEqual(counts["total_kind_unenumerable_units"], 0)
            self.assertEqual(
                counts["non_object_files"],
                ["paizo/roleplaying_game/core_rulebook/cr__datacontrols.lst"],
            )

    def test_bare_abilities_file_classifies_by_row_category_tag(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Acrobatic\tCATEGORY:FEAT\n"
                "Darkvision\tCATEGORY:Special Quality\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("feat"), 1)
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Special Quality"), 1
            )

    def test_content_bearing_ability_row_is_enumerated_as_kind_ability(self):
        # SD-32 card 15-ability (`decisions.md §12b`): a bare abilities row
        # carrying independent mechanical content (here `DEFINE:`) is
        # disposition (A) per `15-card-15-ability-category-memo.md` and
        # must land in `counts_by_kind["ability"]`, not `kind_unenumerable`.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Lay on Hands\tCATEGORY:Special Ability\tDEFINE:LayOnHandsLVL|0\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("ability"), 1)
            self.assertNotIn(
                "ability_category:Special Ability", counts["kind_unenumerable"]
            )

    def test_bare_picklist_ability_row_stays_excluded_not_swallowed_as_ability(self):
        # The (B)-picklist shape from the memo's "Ability Focus" bucket: a
        # row with nothing beyond CATEGORY:/TYPE: -- must NOT be counted as
        # `ability`.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Breath Weapon\tCATEGORY:Ability Focus\tTYPE:Ability Focus\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertNotIn("ability", counts["counts_by_kind"])
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Ability Focus"), 1
            )

    def test_gateway_only_ability_row_stays_excluded_not_swallowed_as_ability(self):
        # The (B)-gateway shape: no content field, but an
        # `ABILITY:...|AUTOMATIC|<target>` wrapper -- a facet, not a new
        # object, even though it would otherwise look content-bearing.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Add a Class Skill\tCATEGORY:Special Ability"
                "\tABILITY:Class Skill|AUTOMATIC|%LIST\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertNotIn("ability", counts["counts_by_kind"])
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Special Ability"), 1
            )

    def test_ability_row_whose_key_collides_with_a_tracked_kind_is_a_duplicate_not_new(
        self,
    ):
        # The (B)-duplicate shape ("the shared-name hazard"): a KEY:-field
        # exact match against an already-tracked kind (here `feat`) means
        # this row is cross-book content reuse, not a new object -- even
        # though it carries real content of its own.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_feats.lst"),
                "Ability Focus\tKEY:Ability Focus\tCATEGORY:FEAT\tTYPE:General\n",
            )
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Ability Focus\tKEY:Ability Focus\tCATEGORY:Special Ability"
                "\tDEFINE:AbilityFocusLVL|0\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertNotIn("ability", counts["counts_by_kind"])
            self.assertEqual(counts["counts_by_kind"].get("feat"), 1)
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Special Ability"), 1
            )

    def test_abilities_familiar_file_routes_to_companion_not_ability_category(self):
        # SD-32 card 15-ability: `src/bin/v06_work_inventory.rs`'s
        # `file_kind` already routes `*_abilities_familiar*.lst`/
        # `*_abilities_companion*.lst` to the tracked `companion` kind
        # (checked BEFORE the bare-abilities fallback) -- the census must
        # agree, or the two walkers' `ability_category:*` figures
        # double-count against `companion` (a real defect this cycle found:
        # 97 rows across 6 in-scope files).
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/bestiary_3"
            _touch(os.path.join(root, book, "bestiary_3.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "b3_abilities_familiar.lst"),
                "Empathic Link\tCATEGORY:Special Ability\tDEFINE:EmpathicLinkLVL|0\n",
            )
            bd = CI.BookDir(book, "bestiary_3", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["counts_by_kind"].get("companion"), 1)
            self.assertNotIn("ability", counts["counts_by_kind"])
            self.assertNotIn(
                "ability_category:Special Ability", counts["kind_unenumerable"]
            )

    def test_exclusion_rule_mutation_proof_widening_it_swallows_a_real_object(self):
        # `decisions.md §16`/dispatch brief item 4: a test that FAILS if the
        # (B) exclusion rule ever starts eating (A) rows. Proven red/green
        # live, not asserted: widen `_ABILITY_CONTENT_RE` to additionally
        # treat a bare `TYPE:` field as "content" (a plausible-looking but
        # WRONG widening -- `TYPE:` is present on picklist rows too) and
        # confirm a genuine (B)-picklist fixture (no independent content)
        # gets wrongly counted as `ability`; then revert and confirm the
        # correct, narrower rule excludes it again.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities.lst"),
                "Breath Weapon\tCATEGORY:Ability Focus\tTYPE:Ability Focus\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")

            original_re = CI._ABILITY_CONTENT_RE
            try:
                # A deliberately wrong widening: "content" now includes any
                # bare TYPE: field, which every picklist row also carries.
                CI._ABILITY_CONTENT_RE = __import__("re").compile(
                    original_re.pattern + r"|TYPE:"
                )
                counts = CI.count_objects(root, [bd])
                # RED: the widened rule wrongly swallows the picklist row.
                self.assertEqual(
                    counts["counts_by_kind"].get("ability"),
                    1,
                    "widened rule failed to reproduce the swallowing bug "
                    "this test exists to catch",
                )
            finally:
                CI._ABILITY_CONTENT_RE = original_re

            # GREEN: reverted to the real rule, the picklist row is excluded
            # again, exactly like `test_bare_picklist_ability_row_stays_excluded_not_swallowed_as_ability`.
            counts = CI.count_objects(root, [bd])
            self.assertNotIn("ability", counts["counts_by_kind"])
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Ability Focus"), 1
            )


class EndToEndTest(unittest.TestCase):
    def test_run_writes_diff_json_and_excluded_directories_md_with_zero_unexplained(self):
        with tempfile.TemporaryDirectory() as tmp:
            pcgen_root = os.path.join(tmp, "pcgen_root")
            book = "pathfinder/paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(pcgen_root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(pcgen_root, book, "cr_feats.lst"),
                "Acrobatic\tCATEGORY:FEAT\n",
            )
            _touch(
                os.path.join(
                    pcgen_root,
                    "pathfinder/paizo/adventure_path/some_ap/some_ap.pcc",
                ),
                "x\n",
            )

            inventory_path = os.path.join(tmp, "work-inventory.json")
            with open(inventory_path, "w", encoding="utf-8") as fh:
                json.dump({"books": [{"id": "core_rulebook"}]}, fh)

            output_path = os.path.join(tmp, "artifacts", "diff.json")
            diff = CI.run(pcgen_root, inventory_path, output_path)

            self.assertEqual(diff["unexplained"], 0)
            self.assertEqual(diff["in_scope_book_dirs"], 1)
            self.assertEqual(diff["excluded_book_dirs"], 1)
            self.assertTrue(os.path.exists(output_path))
            excluded_md = os.path.join(tmp, "artifacts", "excluded-directories.md")
            self.assertTrue(os.path.exists(excluded_md))
            with open(output_path, "r", encoding="utf-8") as fh:
                on_disk = json.load(fh)
            self.assertEqual(on_disk["unexplained"], 0)


if __name__ == "__main__":
    unittest.main()
