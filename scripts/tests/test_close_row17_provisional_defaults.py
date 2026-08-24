#!/usr/bin/env python3
"""Tests for `scripts/close_row17_provisional_defaults.py` (`kanban.md` row
17, `decisions.md §27a`/§27b). Uses a small synthetic `tempfile` corpus
tree, never the live corpus."""

import json
import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import close_row17_provisional_defaults as CRP  # noqa: E402


def _write(root: str, book: str, name: str, corpus_key: str, facet: str, provisional: bool) -> str:
    ability_dir = os.path.join(root, book, "monster_ability")
    os.makedirs(ability_dir, exist_ok=True)
    path = os.path.join(ability_dir, f"{name}.json")
    data = {"corpus_key": corpus_key, "facet": facet, "key": f"{book}:monster_ability:{name}"}
    if provisional:
        data["shape_provisional_default"] = True
        data["shape_provisional_reason"] = "some reason"
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"data": data}, handle)
    return path


class CloseCorpusTest(unittest.TestCase):
    def test_reclassifies_a_named_record_and_clears_the_marker(self):
        # "Aurumvorax ~ Rake" is a real entry in
        # `_MONSTER_ABILITY_FACET_OVERRIDES` mapped to `SpecialAttack`.
        with tempfile.TemporaryDirectory() as root:
            path = _write(
                root, "bestiary_2", "aurumvorax_rake", "Aurumvorax ~ Rake", "SpecialQuality", True
            )
            touched = CRP.close_corpus(root)
            self.assertEqual(len(touched), 1)
            self.assertEqual(touched[0]["old_facet"], "SpecialQuality")
            self.assertEqual(touched[0]["new_facet"], "SpecialAttack")
            self.assertTrue(touched[0]["was_provisional"])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertEqual(record["data"]["facet"], "SpecialAttack")
            self.assertNotIn("shape_provisional_default", record["data"])
            self.assertNotIn("shape_provisional_reason", record["data"])

    def test_confirms_a_named_record_whose_default_was_already_correct(self):
        # "Morlock ~ Sneak Attack" maps to `SpecialQuality`, the same value
        # it already shipped under -- only the marker should clear.
        with tempfile.TemporaryDirectory() as root:
            path = _write(
                root, "beastiary", "morlock_sneak_attack", "Morlock ~ Sneak Attack", "SpecialQuality", True
            )
            touched = CRP.close_corpus(root)
            self.assertEqual(len(touched), 1)
            self.assertEqual(touched[0]["old_facet"], "SpecialQuality")
            self.assertEqual(touched[0]["new_facet"], "SpecialQuality")
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertNotIn("shape_provisional_default", record["data"])

    def test_a_record_not_named_by_the_override_table_is_untouched(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write(
                root, "occult_adventures", "unrelated", "Some Unrelated Record", "SpecialQuality", True
            )
            touched = CRP.close_corpus(root)
            self.assertEqual(touched, [])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            # Untouched: still carries the marker, since this script never
            # guesses a resolution for a record its table does not name.
            self.assertTrue(record["data"]["shape_provisional_default"])

    def test_idempotent_second_run_touches_nothing(self):
        with tempfile.TemporaryDirectory() as root:
            _write(root, "bestiary_2", "aurumvorax_rake", "Aurumvorax ~ Rake", "SpecialQuality", True)
            first = CRP.close_corpus(root)
            self.assertEqual(len(first), 1)
            second = CRP.close_corpus(root)
            self.assertEqual(second, [])

    def test_dry_run_reports_but_does_not_write(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write(
                root, "bestiary_2", "aurumvorax_rake", "Aurumvorax ~ Rake", "SpecialQuality", True
            )
            touched = CRP.close_corpus(root, dry_run=True)
            self.assertEqual(len(touched), 1)
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertEqual(record["data"]["facet"], "SpecialQuality")
            self.assertTrue(record["data"]["shape_provisional_default"])

    def test_non_monster_ability_kinds_are_never_scanned(self):
        with tempfile.TemporaryDirectory() as root:
            class_feature_dir = os.path.join(root, "occult_adventures", "class_feature")
            os.makedirs(class_feature_dir, exist_ok=True)
            path = os.path.join(class_feature_dir, "phrenic_pool.json")
            data = {
                "corpus_key": "Psychic ~ Phrenic Pool",
                "shape_provisional_default": True,
                "shape_provisional_reason": "one of several possible readings",
            }
            with open(path, "w", encoding="utf-8") as handle:
                json.dump({"data": data}, handle)
            touched = CRP.close_corpus(root)
            self.assertEqual(touched, [])


def _write_class_feature(root: str, book: str, subdir: str, name: str, key: str, provisional: bool) -> str:
    cf_dir = os.path.join(root, book, "class_feature", subdir)
    os.makedirs(cf_dir, exist_ok=True)
    path = os.path.join(cf_dir, f"{name}.json")
    data = {"key": key}
    if provisional:
        data["shape_provisional_default"] = True
        data["shape_provisional_reason"] = "some reason"
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"data": data}, handle)
    return path


class CloseClassFeatureCorpusTest(unittest.TestCase):
    def test_clears_the_marker_on_a_named_and_resolved_record(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write_class_feature(
                root, "occult_adventures", "psychic", "phrenic_pool", "Psychic ~ Phrenic Pool", True
            )
            touched = CRP.close_class_feature_corpus(root)
            self.assertEqual(len(touched), 1)
            self.assertEqual(touched[0]["key"], "Psychic ~ Phrenic Pool")
            self.assertTrue(touched[0]["was_provisional"])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertNotIn("shape_provisional_default", record["data"])
            self.assertNotIn("shape_provisional_reason", record["data"])

    def test_a_record_not_named_by_the_resolution_table_is_untouched(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write_class_feature(
                root, "occult_adventures", "sorcerer", "some_other_feature", "Sorcerer ~ Some Other Feature", True
            )
            touched = CRP.close_class_feature_corpus(root)
            self.assertEqual(touched, [])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertTrue(record["data"]["shape_provisional_default"])

    def test_a_named_record_not_currently_provisional_is_untouched(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write_class_feature(
                root, "occult_adventures", "psychic", "phrenic_pool", "Psychic ~ Phrenic Pool", False
            )
            touched = CRP.close_class_feature_corpus(root)
            self.assertEqual(touched, [])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertNotIn("shape_provisional_default", record["data"])

    def test_idempotent_second_run_touches_nothing(self):
        with tempfile.TemporaryDirectory() as root:
            _write_class_feature(root, "occult_adventures", "psychic", "phrenic_pool", "Psychic ~ Phrenic Pool", True)
            first = CRP.close_class_feature_corpus(root)
            self.assertEqual(len(first), 1)
            second = CRP.close_class_feature_corpus(root)
            self.assertEqual(second, [])

    def test_dry_run_reports_but_does_not_write(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write_class_feature(
                root, "occult_adventures", "psychic", "phrenic_pool", "Psychic ~ Phrenic Pool", True
            )
            touched = CRP.close_class_feature_corpus(root, dry_run=True)
            self.assertEqual(len(touched), 1)
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertTrue(record["data"]["shape_provisional_default"])

    def test_monster_ability_kinds_are_never_scanned_by_the_class_feature_closer(self):
        with tempfile.TemporaryDirectory() as root:
            path = _write(
                root, "bestiary_2", "aurumvorax_rake", "Aurumvorax ~ Rake", "SpecialQuality", True
            )
            touched = CRP.close_class_feature_corpus(root)
            self.assertEqual(touched, [])
            with open(path, encoding="utf-8") as handle:
                record = json.load(handle)
            self.assertTrue(record["data"]["shape_provisional_default"])


if __name__ == "__main__":
    unittest.main()
