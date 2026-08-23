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

    def test_class_feature_internal_rows_reroute_to_ability_category_internal(self):
        # Card 15 (`decisions.md §12b`): a `_abilities_class.lst` row
        # carrying `CATEGORY:Internal` is PCGen bookkeeping, not a
        # class_feature, and should land in the same bucket the bare
        # `abilit` branch already uses for this exact marker.
        with tempfile.TemporaryDirectory() as tmp:
            root = os.path.join(tmp, "pathfinder")
            book = "paizo/roleplaying_game/core_rulebook"
            _touch(os.path.join(root, book, "core_rulebook.pcc"), "x\n")
            _touch(
                os.path.join(root, book, "cr_abilities_class.lst"),
                "Rage\tCATEGORY:Special Ability\n"
                "Damage Reduction ~ All\tCATEGORY:Internal\tDR:ClassFeatureDR_ALL/-\n",
            )
            bd = CI.BookDir(book, "core_rulebook", "paizo/roleplaying_game")
            counts = CI.count_objects(root, [bd])
            self.assertEqual(counts["kind_unenumerable"].get("class_feature"), 1)
            self.assertEqual(
                counts["kind_unenumerable"].get("ability_category:Internal"), 1
            )
            self.assertNotIn("class_feature", counts["counts_by_kind"])

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
