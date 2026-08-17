"""Self-test for `scripts/observer/pi_redaction.py` -- Decision 12's
declared-PI oracle reader (SD31-D14-PROV-001).

Every fixture below builds a SCRATCH pcgen-shaped tree (never the real
pinned oracle -- these tests must pass on a machine with no oracle checkout
at all, same posture `test_ground_truth_evidence_guard.py` already takes)
and exercises the real production functions against it, so a change to the
reader's own logic is caught here rather than only downstream in the
producer.

Run: python3 -m unittest scripts.tests.test_pi_redaction
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import shutil
import tempfile
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "observer" / "pi_redaction.py"
)
_spec = importlib.util.spec_from_file_location("pi_redaction", _MODULE_PATH)
pi_redaction = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(pi_redaction)


class Scratch:
    def __init__(self, name: str):
        self.root = pathlib.Path(
            tempfile.gettempdir()
        ) / f"codex_pi_redaction_test_{name}_{os.getpid()}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


class DeclaredProductIdentityTests(unittest.TestCase):
    """Parity with `src/rules_core/pi_screening.rs::declared_product_identity`."""

    def test_nameispi_yes_is_a_name_hit(self):
        name, desc = pi_redaction.declared_product_identity([("NAMEISPI", "YES")])
        self.assertTrue(name)
        self.assertFalse(desc)

    def test_descispi_yes_is_a_description_hit(self):
        name, desc = pi_redaction.declared_product_identity([("DESCISPI", "YES")])
        self.assertFalse(name)
        self.assertTrue(desc)

    def test_nameispi_no_is_not_a_hit(self):
        # PCGen writes `NAMEISPI:NO` explicitly on OGL rows -- anything
        # other than YES is absence, not a hit.
        name, _ = pi_redaction.declared_product_identity([("NAMEISPI", "NO")])
        self.assertFalse(name)

    def test_keys_are_case_insensitive_and_values_are_trimmed(self):
        name, desc = pi_redaction.declared_product_identity(
            [("nameispi", " Yes "), ("descispi", "yes")]
        )
        self.assertTrue(name)
        self.assertTrue(desc)

    def test_no_tokens_is_no_declaration(self):
        name, desc = pi_redaction.declared_product_identity([])
        self.assertFalse(name)
        self.assertFalse(desc)


class ParseRowTokensTests(unittest.TestCase):
    def test_bare_flag_with_no_colon_is_skipped(self):
        tokens = pi_redaction.parse_row_tokens("Otyugh Hide\tNAMEISPI:YES\tBAREFLAG\tCOST:1415")
        keys = [k for k, _ in tokens]
        self.assertIn("NAMEISPI", keys)
        self.assertIn("COST", keys)
        self.assertNotIn("BAREFLAG", keys)


class OracleNameCheckerTests(unittest.TestCase):
    """Mutation-proof: a real NAMEISPI:YES row is caught; a clean row is not."""

    def setUp(self):
        self.s = Scratch("checker")
        self.addCleanup(self.s.cleanup)
        self.s.write(
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip.lst",
            "Sturdy Rope\tCOST:1\tWT:5\n"
            "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n",
        )
        self.checker = pi_redaction.OracleNameChecker(str(self.s.root))

    def test_available_when_the_checkout_exists(self):
        self.assertTrue(self.checker.available)

    def test_a_declared_row_is_caught(self):
        name, _ = self.checker.declared("ultimate_equipment", "ue_equip.lst", 2)
        self.assertTrue(name, "NAMEISPI:YES on line 2 must be caught")

    def test_a_clean_row_is_not_flagged(self):
        name, _ = self.checker.declared("ultimate_equipment", "ue_equip.lst", 1)
        self.assertFalse(name, "line 1 declares nothing and must not be flagged")

    def test_missing_book_degrades_to_no_declaration_not_a_crash(self):
        name, desc = self.checker.declared("nonexistent_book", "whatever.lst", 1)
        self.assertFalse(name)
        self.assertFalse(desc)

    def test_missing_coordinates_degrade_safely(self):
        self.assertEqual(self.checker.declared(None, None, None), (False, False))
        self.assertEqual(self.checker.declared("ultimate_equipment", "ue_equip.lst", None), (False, False))

    def test_out_of_range_line_degrades_safely(self):
        name, _ = self.checker.declared("ultimate_equipment", "ue_equip.lst", 9999)
        self.assertFalse(name)

    def test_unavailable_checkout_reports_unavailable(self):
        checker = pi_redaction.OracleNameChecker(str(self.s.root / "does_not_exist"))
        self.assertFalse(checker.available)
        # Still degrades safely rather than raising.
        self.assertEqual(checker.declared("ultimate_equipment", "ue_equip.lst", 2), (False, False))


class BuildDeclaredPiNameIndexTests(unittest.TestCase):
    def setUp(self):
        self.s = Scratch("index")
        self.addCleanup(self.s.cleanup)
        self.s.write(
            "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_classes.lst",
            "Aldori Swordlord\tNAMEISPI:YES\tHD:10\n"
            "Ordinary Fighter\tHD:10\n",
        )
        self.s.write(
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip.lst",
            "Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n",
        )

    def test_index_contains_every_declared_name_and_nothing_else(self):
        names = pi_redaction.build_declared_pi_name_index(str(self.s.root))
        self.assertIn("Aldori Swordlord", names)
        self.assertIn("Otyugh Hide", names)
        self.assertNotIn("Ordinary Fighter", names)

    def test_class_prefix_is_stripped(self):
        self.s.write(
            "pathfinder/paizo/roleplaying_game/some_book/classy.lst",
            "CLASS:Secret Order\tNAMEISPI:YES\tHD:10\n",
        )
        names = pi_redaction.build_declared_pi_name_index(str(self.s.root))
        self.assertIn("Secret Order", names)
        self.assertNotIn("CLASS:Secret Order", names)

    def test_a_name_shared_with_a_non_pi_record_elsewhere_is_excluded(self):
        # The real defect this cycle found: an unrelated Spycraft ritual
        # declaring NAMEISPI:YES with the bare name "Teleport" must not
        # cause the Core Rulebook's ordinary, non-PI "Teleport" spell to
        # read as a leak. "A shared name never implies a shared thing."
        self.s.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst",
            "Teleport\tTYPE:Arcane.Divine\tCLASSES:Sorcerer,Wizard=5\n",
        )
        self.s.write(
            "pathfinder/some_other_publisher/rituals.lst",
            "Teleport\tNAMEISPI:YES\tTYPE:Mystic.Ritual\n",
        )
        names = pi_redaction.build_declared_pi_name_index(str(self.s.root))
        self.assertNotIn("Teleport", names, "an ambiguous shared name must not be flagged")

    def test_a_paizo_only_ambiguous_name_is_still_excluded(self):
        # Same collision, but both inside paizo_root -- still ambiguous.
        self.s.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst",
            "Shield\tTYPE:Arcane\tCLASSES:Sorcerer,Wizard=1\n",
        )
        self.s.write(
            "pathfinder/paizo/player_companion/some_book/items.lst",
            "Shield\tNAMEISPI:YES\tCOST:5\n",
        )
        names = pi_redaction.build_declared_pi_name_index(str(self.s.root))
        self.assertNotIn("Shield", names)


class LeakScanTests(unittest.TestCase):
    """`find_declared_pi_leaks` — the verify.sh gate's own scan engine."""

    def test_finds_a_leak_nested_in_a_shard_shaped_document(self):
        patterns = pi_redaction.compile_name_patterns(["Aldori Swordlord", "Otyugh Hide"])
        doc = {
            "manifests": {
                "sd30_book_pre_build": {
                    "items": [
                        {"id": "aldori_swordlord", "name": "Aldori Swordlord", "book": "Adventurer's Guide"},
                        {"id": "safe_item", "name": "Ordinary Fighter", "book": "Adventurer's Guide"},
                    ]
                }
            }
        }
        hits = pi_redaction.find_declared_pi_leaks(doc, patterns)
        found_names = {name for _, name in hits}
        self.assertIn("Aldori Swordlord", found_names)
        self.assertNotIn("Otyugh Hide", found_names)

    def test_a_clean_document_has_no_hits(self):
        patterns = pi_redaction.compile_name_patterns(["Otyugh Hide"])
        doc = {"units": [{"name": pi_redaction.REDACTED_PI_MARKER, "book": "ultimate_equipment"}]}
        self.assertEqual(pi_redaction.find_declared_pi_leaks(doc, patterns), [])

    def test_substring_collisions_do_not_fire(self):
        # "Nex" must not fire inside "Nexus" -- row 149's own stated
        # collision example -- AND (the real defect this cycle found) a
        # declared-PI name must not fire as a mere SUBSTRING of an
        # unrelated longer name: "Shackles" (a declared-PI region
        # background) must not flag the ordinary, non-PI magic item
        # "Shackles of Compliance".
        patterns = pi_redaction.compile_name_patterns(["Nex", "Shackles"])
        doc = {"a": "Nexus Gate", "b": "Shackles of Compliance"}
        self.assertEqual(pi_redaction.find_declared_pi_leaks(doc, patterns), [])

    def test_a_shard_row_array_leak_is_also_found(self):
        # Shards store rows as bare arrays (UNIT_SHARD_FIELDS order), not
        # dicts -- confirm the walk finds a leak inside a plain list of
        # lists too.
        patterns = pi_redaction.compile_name_patterns(["Otyugh Hide"])
        doc = {"rows": [["Otyugh Hide", "ultimate_equipment", "grounded"]]}
        hits = pi_redaction.find_declared_pi_leaks(doc, patterns)
        self.assertEqual(len(hits), 1)


class RedactDeclaredPiNamesTests(unittest.TestCase):
    """`redact_declared_pi_names` -- the producer's blanket defense-in-depth
    pass over the whole assembled document."""

    def test_an_exact_match_leaf_is_replaced(self):
        names = {"Aldori Swordlord"}
        doc = {"matrix": {"prestige_classes": [{"name": "Aldori Swordlord"}]}}
        out = pi_redaction.redact_declared_pi_names(doc, names)
        self.assertEqual(out["matrix"]["prestige_classes"][0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_a_substring_occurrence_is_left_alone(self):
        names = {"Shackles"}
        doc = {"name": "Shackles of Compliance"}
        out = pi_redaction.redact_declared_pi_names(doc, names)
        self.assertEqual(out["name"], "Shackles of Compliance")

    def test_the_original_document_is_not_mutated(self):
        names = {"Aldori Swordlord"}
        doc = {"name": "Aldori Swordlord"}
        pi_redaction.redact_declared_pi_names(doc, names)
        self.assertEqual(doc["name"], "Aldori Swordlord")


if __name__ == "__main__":
    unittest.main()
