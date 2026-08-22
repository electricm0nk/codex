"""Self-test for `scripts/site_public_status_pi_gate.py` (SITE-PI-ALLOWLIST-001,
2026-08-17): the safety-net gate over the committed `site/status-data*`
artifacts, exercised against SCRATCH fixtures rather than the real
committed files, so a change to the gate's own matching logic is caught
here regardless of what happens to be committed at the time.

NOT currently wired as its own `scripts/verify.sh` stage -- verify.sh is
outside this package's write scope. It is still real coverage: run it
directly (`python3 -m unittest scripts.tests.test_site_public_status_pi_gate`)
or via the `root-lib`/full test sweep. `scripts/verify.sh`'s own
`site-public-status-pi-gate` stage runs the gate FOR REAL against whatever
is actually committed, which is the primary proof for the shipped data;
this file is the primary proof for the gate's *logic*, with the real
oracle checkout replaced by a scratch fixture so it also runs on a
box with no PCGen checkout at all.

Run: python3 -m unittest scripts.tests.test_site_public_status_pi_gate
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

_GATE = _REPO_ROOT / "scripts" / "site_public_status_pi_gate.py"
_spec2 = importlib.util.spec_from_file_location("site_public_status_pi_gate", _GATE)
gate = importlib.util.module_from_spec(_spec2)
_spec2.loader.exec_module(gate)

_ALLOWLIST = _REPO_ROOT / "scripts" / "site" / "pi_substring_allowlist.py"
_spec3 = importlib.util.spec_from_file_location("pi_substring_allowlist", _ALLOWLIST)
pi_allowlist = importlib.util.module_from_spec(_spec3)
_spec3.loader.exec_module(pi_allowlist)


class Scratch:
    def __init__(self, name: str):
        self.root = pathlib.Path(tempfile.gettempdir()) / f"codex_gate_test_{name}_{os.getpid()}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


def book_detail(book_id: str, items: list[dict]) -> dict:
    return {
        "id": book_id,
        "title": book_id,
        "kinds": [{"kind": "feat", "label": "Feats", "items": items}],
    }


def status_item(name: str, type_facet=None) -> dict:
    return {"name": name, "doneness": "not-started", "type_facet": type_facet, "standing": "origin"}


class FindStatusItemPiLeaksTests(unittest.TestCase):
    def setUp(self):
        self.scratch = Scratch("find_leaks")
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
        self.declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        self.declared_by_length = sorted(self.declared_names, key=len, reverse=True)
        self.book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        self.allowlist_index = pi_allowlist.build_allowlist_index()

    def test_a_brand_new_unlisted_word_match_is_a_hit(self):
        # Mutation-proof (a): seed a leak the allow-list has never seen.
        doc = book_detail("advanced_players_guide", [status_item("Death (Pharasma)")])
        hits = gate.find_status_item_pi_leaks(
            doc, self.declared_by_length, self.book_declared, self.allowlist_index
        )
        self.assertEqual(len(hits), 1)
        self.assertIn("Pharasma", hits[0][1])

    def test_the_real_allowlisted_names_are_clean(self):
        # Mutation-proof (b): every entry actually on the real, shipped
        # allow-list must be clean in its own reviewed book(s).
        for name, entry in self.allowlist_index.items():
            for book in entry["books"]:
                with self.subTest(name=name, book=book):
                    doc = book_detail(book, [status_item(name)])
                    hits = gate.find_status_item_pi_leaks(
                        doc, self.declared_by_length, self.book_declared, self.allowlist_index
                    )
                    self.assertEqual(hits, [], f"{name!r} in {book!r} unexpectedly flagged: {hits}")

    def test_an_allowlisted_name_in_a_DIFFERENT_book_is_still_a_hit(self):
        # The allow-list is keyed on (name, book) together -- publishing
        # "Dimensional Shackles" under a book the review never covered
        # must not inherit clearance from the core_rulebook entry.
        doc = book_detail("ultimate_magic", [status_item("Dimensional Shackles")])
        hits = gate.find_status_item_pi_leaks(
            doc, self.declared_by_length, self.book_declared, self.allowlist_index
        )
        self.assertEqual(len(hits), 1)

    def test_a_redacted_marker_is_never_flagged(self):
        doc = book_detail(
            "advanced_players_guide", [status_item(pi_redaction.REDACTED_PI_MARKER)]
        )
        hits = gate.find_status_item_pi_leaks(
            doc, self.declared_by_length, self.book_declared, self.allowlist_index
        )
        self.assertEqual(hits, [])

    def test_a_fused_grammatical_derivative_is_never_flagged(self):
        # "Razmiri"/"Numerian"/"Druman"-shaped names never even reach the
        # allow-list question -- word-boundary matching excludes them
        # up front (see test_pi_redaction.py's own coverage of the
        # primitive itself).
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/regions.lst",
            "Nex\tNAMEISPI:YES\tTYPE:Region\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        declared_by_length = sorted(declared_names, key=len, reverse=True)
        book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
        doc = book_detail("advanced_class_guide", [status_item("Discern Next of Kin")])
        hits = gate.find_status_item_pi_leaks(doc, declared_by_length, book_declared, self.allowlist_index)
        self.assertEqual(hits, [])

    def test_type_facet_still_uses_plain_substring_not_word_boundary(self):
        # type_facet screening is unchanged by this cycle's `name` fix --
        # still a global plain-substring scan (see build_public_status.py's
        # own module-level comment for why this field stays plain-substring).
        doc = book_detail(
            "advanced_players_guide",
            [status_item("Ordinary Feat", type_facet="PharasmaClassFeatures.SpecialQuality")],
        )
        hits = gate.find_status_item_pi_leaks(
            doc, self.declared_by_length, self.book_declared, self.allowlist_index
        )
        self.assertEqual(len(hits), 1)
        self.assertIn("type_facet", hits[0][0])

    def test_a_shape_that_is_not_a_book_detail_document_is_a_no_op(self):
        self.assertEqual(
            gate.find_status_item_pi_leaks({"unrelated": True}, self.declared_by_length, self.book_declared, self.allowlist_index),
            [],
        )


if __name__ == "__main__":
    unittest.main()
