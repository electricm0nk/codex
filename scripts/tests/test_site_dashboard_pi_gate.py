"""Self-test for `scripts/site_dashboard_pi_gate.py` (FIX-DASHBOARD-PI,
2026-08-17): the safety-net gate over the committed `site/dashboard/**`
artifacts, exercised against SCRATCH fixtures rather than the real
committed files, so a change to the gate's own matching logic is caught
here regardless of what happens to be committed at the time.

Same pattern as the sibling `test_site_public_status_pi_gate.py` (see that
file's own module docstring for why a scratch-fixture test is the primary
proof for the gate's *logic*, with `scripts/verify.sh`'s own
`site-dashboard-pi-gate` stage remaining the primary proof for whatever is
actually committed).

Mutation-proof discipline (Decision 12 requirement #3, extended by
FIX-DASHBOARD-PI to the word-boundary layer): a declared-PI name is seeded
into (a) the top-level feed's `books[*].items` roster, (b) a shard's own
`fields`/`rows`, and (c) a `unit_index` category label -- each must be
caught. Comment out any ONE of the three `for ... in find_*` loops in
`site_dashboard_pi_gate.py`'s `main()` and the matching test below goes
red.

Run: python3 -m unittest scripts.tests.test_site_dashboard_pi_gate
"""
from __future__ import annotations

import importlib.util
import os
import pathlib
import shutil
import sys
import tempfile
import unittest

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent

_PI_REDACTION = _REPO_ROOT / "scripts" / "observer" / "pi_redaction.py"
_spec = importlib.util.spec_from_file_location("pi_redaction", _PI_REDACTION)
pi_redaction = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(pi_redaction)

_GATE = _REPO_ROOT / "scripts" / "site_dashboard_pi_gate.py"
_spec2 = importlib.util.spec_from_file_location("site_dashboard_pi_gate", _GATE)
gate = importlib.util.module_from_spec(_spec2)
_spec2.loader.exec_module(gate)

_ALLOWLIST = _REPO_ROOT / "scripts" / "site" / "pi_substring_allowlist.py"
_spec3 = importlib.util.spec_from_file_location("pi_substring_allowlist", _ALLOWLIST)
pi_allowlist = importlib.util.module_from_spec(_spec3)
_spec3.loader.exec_module(pi_allowlist)


class Scratch:
    def __init__(self, name: str):
        self.root = pathlib.Path(tempfile.gettempdir()) / f"codex_dashboard_gate_test_{name}_{os.getpid()}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


def top_level_feed(book_id: str, equipment: list) -> dict:
    return {"books": [{"id": book_id, "title": book_id, "items": {"equipment": equipment}}]}


def shard_doc(rows: list) -> dict:
    return {"kind": "equipment", "fields": ["name", "book"], "rows": rows}


def unit_index_doc(kind: str, label: str) -> dict:
    return {
        "kinds": {
            kind: {
                "categories": {"somegroup": {"label": label, "done": 1}},
            }
        }
    }


class WordBoundaryGateTests(unittest.TestCase):
    def setUp(self):
        self.scratch = Scratch("wb")
        self.addCleanup(self.scratch.cleanup)
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_gods/deities.lst",
            "Pharasma\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/advanced_race_guide/regions.lst",
            "Shackles\tNAMEISPI:YES\tTYPE:Region\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        self.declared_by_length = sorted(declared_names, key=len, reverse=True)
        self.book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        self.allowlist_index = pi_allowlist.build_allowlist_index()

    # (a) Top-level feed: books[*].items roster.
    def test_a_top_level_roster_word_boundary_leak_is_caught(self):
        doc = top_level_feed("advanced_players_guide", ["Death (Pharasma)"])
        hits = gate.find_book_roster_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(len(hits), 1)
        self.assertIn("Pharasma", hits[0][1])

    def test_a_top_level_roster_clean_name_is_not_flagged(self):
        doc = top_level_feed("core_rulebook", ["Longsword"])
        hits = gate.find_book_roster_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_a_top_level_roster_allowlisted_name_is_clean(self):
        doc = top_level_feed("core_rulebook", ["Dimensional Shackles"])
        hits = gate.find_book_roster_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_a_top_level_roster_dict_entry_name_field_is_checked(self):
        doc = top_level_feed("advanced_players_guide", [{"name": "Death (Pharasma)", "id": "x"}])
        hits = gate.find_book_roster_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(len(hits), 1)

    # (b) A shard's own fields/rows.
    def test_b_shard_word_boundary_leak_is_caught(self):
        doc = shard_doc([["Death (Pharasma)", "advanced_players_guide"]])
        hits = gate.find_shard_word_boundary_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(len(hits), 1)
        self.assertIn("Pharasma", hits[0][1])

    def test_b_shard_allowlisted_name_is_clean(self):
        doc = shard_doc([["Shackles of Compliance", "ultimate_equipment"]])
        hits = gate.find_shard_word_boundary_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_b_shard_allowlisted_name_in_a_different_book_is_still_a_hit(self):
        doc = shard_doc([["Dimensional Shackles", "ultimate_magic"]])
        hits = gate.find_shard_word_boundary_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(len(hits), 1)

    def test_b_shard_redacted_marker_is_never_flagged(self):
        doc = shard_doc([[pi_redaction.REDACTED_PI_MARKER, "advanced_players_guide"]])
        hits = gate.find_shard_word_boundary_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_b_shard_type_facet_substring_leak_is_caught(self):
        doc = {
            "fields": ["name", "book", "type_facet"],
            "rows": [["Ordinary Helm", "adventurers_guide", "ClassFeatures.Hellknight Signifer Class Feature.SpecialQuality"]],
        }
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/adventurers_guide/signifer.lst",
            "Signifer\tNAMEISPI:YES\tTYPE:Special Ability\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        declared_by_length = sorted(declared_names, key=len, reverse=True)
        book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        hits = gate.find_shard_word_boundary_leaks(doc, declared_by_length, book_declared, self.allowlist_index)
        self.assertEqual(len(hits), 1)
        self.assertEqual(hits[0][0], "$.rows[0][2]")

    def test_b_shard_type_facet_redacted_marker_is_never_flagged(self):
        doc = {
            "fields": ["name", "book", "type_facet"],
            "rows": [["Longsword", "core_rulebook", pi_redaction.REDACTED_PI_MARKER]],
        }
        hits = gate.find_shard_word_boundary_leaks(doc, self.declared_by_length, self.book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    # (c) unit_index category labels.
    def test_c_category_label_word_boundary_leak_is_caught(self):
        doc = unit_index_doc("class_feature", "Varisian Pilgrim Domain")
        # "Varisian Pilgrim" is not in this scratch oracle; seed it directly.
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_magic/pilgrim.lst",
            "Varisian Pilgrim\tNAMEISPI:YES\tTYPE:Archetype\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        declared_by_length = sorted(declared_names, key=len, reverse=True)
        hits = gate.find_category_label_leaks(doc, declared_by_length, self.allowlist_index)
        self.assertEqual(len(hits), 1)
        self.assertIn("Varisian Pilgrim", hits[0][1])

    def test_c_category_label_reaches_through_the_top_level_feed_shape_too(self):
        """`unit_index` embedded in the top-level PF1e-dashboard.json feed
        (not just the standalone units/index.json manifest) is scanned too
        -- `_unit_index_kinds` must resolve both shapes."""
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_magic/pilgrim.lst",
            "Varisian Pilgrim\tNAMEISPI:YES\tTYPE:Archetype\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        declared_by_length = sorted(declared_names, key=len, reverse=True)
        doc = {"generated_at": "x", "unit_index": unit_index_doc("class_feature", "Varisian Pilgrim Domain")}
        hits = gate.find_category_label_leaks(doc, declared_by_length, self.allowlist_index)
        self.assertEqual(len(hits), 1)

    def test_c_category_label_reviewed_allowlist_entry_is_clean(self):
        doc = unit_index_doc("class", "Ulfen Guard Class Feature")
        hits = gate.find_category_label_leaks(doc, self.declared_by_length, self.allowlist_index)
        self.assertEqual(hits, [], f"reviewed allow-list entry unexpectedly flagged: {hits}")

    def test_c_category_label_clean_label_is_not_flagged(self):
        doc = unit_index_doc("class_feature", "Combat Style Feature")
        hits = gate.find_category_label_leaks(doc, self.declared_by_length, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_c_a_shape_that_is_not_a_unit_index_document_is_a_no_op(self):
        self.assertEqual(gate.find_category_label_leaks({"unrelated": True}, self.declared_by_length, self.allowlist_index), [])

    # Fused grammatical derivatives never even reach the allow-list question.
    def test_a_fused_grammatical_derivative_is_never_flagged(self):
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/regions.lst",
            "Nex\tNAMEISPI:YES\tTYPE:Region\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        declared_by_length = sorted(declared_names, key=len, reverse=True)
        book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        doc = top_level_feed("advanced_class_guide", ["Discern Next of Kin"])
        hits = gate.find_book_roster_leaks(doc, declared_by_length, book_declared, self.allowlist_index)
        self.assertEqual(hits, [])


if __name__ == "__main__":
    unittest.main()
