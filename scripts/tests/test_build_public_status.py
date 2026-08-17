"""Self-test for `scripts/site/build_public_status.py` (SITE-PUBSTATUS-001,
2026-08-17): PI screening (Decision 12), the public three-bucket doneness
mapping, and the standing/denominator wiring (Decision 14).

Every fixture below builds a SCRATCH pcgen-shaped tree (never the real
pinned oracle -- these tests must pass on a machine with no oracle checkout
at all) and a scratch site/dashboard/units-shaped ledger, and exercises the
real production functions against them, same posture
`test_pi_redaction.py`/`test_provenance.py` already take.

Run: python3 -m unittest scripts.tests.test_build_public_status
"""
from __future__ import annotations

import importlib.util
import os
import pathlib
import shutil
import sys
import tempfile
import unittest

_OBSERVER_DIR = pathlib.Path(__file__).resolve().parent.parent / "observer"
sys.path.insert(0, str(_OBSERVER_DIR))

_MODULE_PATH = pathlib.Path(__file__).resolve().parent.parent / "site" / "build_public_status.py"
_spec = importlib.util.spec_from_file_location("build_public_status", _MODULE_PATH)
bps = importlib.util.module_from_spec(_spec)
sys.modules["build_public_status"] = bps
_spec.loader.exec_module(bps)

import pi_redaction  # noqa: E402
import provenance  # noqa: E402


class Scratch:
    """A throwaway PCGen-shaped oracle tree, same pattern as
    test_pi_redaction.py's own Scratch fixture."""

    def __init__(self, name: str):
        self.root = pathlib.Path(tempfile.gettempdir()) / f"codex_bps_test_{name}_{os.getpid()}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


def item(kind="feat", book="core_rulebook", name="Ordinary Feat", doneness_raw="done", type_facet=None):
    return {"kind": kind, "book": book, "name": name, "doneness_raw": doneness_raw, "type_facet": type_facet}


class DonenessBucketTests(unittest.TestCase):
    """Public bucket table: done->done, held/in-progress->partial,
    not-started/unmeasurable/deferred->not-started."""

    def test_every_internal_value_maps_to_one_of_three_public_buckets(self):
        import pf1e_dashboard_producer as producer

        expected = {
            producer.DONENESS_DONE: "done",
            producer.DONENESS_HELD: "partial",
            producer.DONENESS_IN_PROGRESS: "partial",
            producer.DONENESS_NOT_STARTED: "not-started",
            producer.DONENESS_UNMEASURABLE: "not-started",
            producer.DONENESS_DEFERRED: "not-started",
        }
        self.assertEqual(bps.DONENESS_TO_PUBLIC, expected)

    def test_attach_public_doneness_replaces_the_raw_verdict(self):
        it = item(doneness_raw="held")
        standing_by = {(bps.object_id(it), it["book"]): provenance.ORIGIN}
        bps.attach_standing_and_public_doneness([it], standing_by)
        self.assertEqual(it["doneness"], "partial")
        self.assertNotIn("doneness_raw", it)


class StandingTests(unittest.TestCase):
    """Standing reuses provenance.classify_unambiguous rather than
    reimplementing it."""

    def test_single_book_object_is_origin(self):
        items = [item(name="Solo Feat", book="core_rulebook")]
        standing = bps.compute_standing(items)
        self.assertEqual(standing[(bps.object_id(items[0]), "core_rulebook")], provenance.ORIGIN)

    def test_multi_book_object_is_unclassified_not_guessed(self):
        items = [
            item(name="Reprinted Feat", book="core_rulebook"),
            item(name="Reprinted Feat", book="advanced_players_guide"),
        ]
        standing = bps.compute_standing(items)
        for it in items:
            self.assertEqual(standing[(bps.object_id(it), it["book"])], provenance.UNCLASSIFIED)

    def test_core_essentials_is_packaging_artifact(self):
        items = [item(name="Ghost Feat", book="core_essentials")]
        standing = bps.compute_standing(items)
        self.assertEqual(
            standing[(bps.object_id(items[0]), "core_essentials")],
            provenance.PACKAGING_ARTIFACT,
        )

    def test_denominator_standings_come_from_provenance_module(self):
        # Never hand-listed: must be the exact tuple provenance.py itself
        # defines as "counts toward the denominator."
        self.assertIs(bps.DENOMINATOR_STANDINGS, provenance.DENOMINATOR_STATUSES)


class RollupDenominatorTests(unittest.TestCase):
    """denominator = origin + variant only; everything else is visibly
    excluded, not silently dropped."""

    def test_unclassified_and_packaging_artifact_are_excluded_from_pct(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.ORIGIN},
            {"kind": "feat", "book": "b", "name": "b", "doneness": "done", "standing": provenance.UNCLASSIFIED},
            {"kind": "feat", "book": "b", "name": "c", "doneness": "not-started", "standing": provenance.PACKAGING_ARTIFACT},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 1)
        self.assertEqual(roll["done"], 1)
        self.assertEqual(roll["pct"], 100.0)
        self.assertEqual(roll["excluded_from_percentage"], 2)
        self.assertEqual(
            roll["standing_breakdown"],
            {provenance.ORIGIN: 1, provenance.UNCLASSIFIED: 1, provenance.PACKAGING_ARTIFACT: 1},
        )

    def test_variant_counts_toward_denominator(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.VARIANT},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 1)
        self.assertEqual(roll["done"], 1)

    def test_empty_denominator_reports_zero_pct_not_a_crash(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.UNCLASSIFIED},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 0)
        self.assertEqual(roll["pct"], 0.0)
        self.assertEqual(roll["excluded_from_percentage"], 1)


class PiRedactionTests(unittest.TestCase):
    """Decision 12: withhold the name, keep the row."""

    def setUp(self):
        self.scratch = Scratch("pi_redact")
        self.addCleanup(self.scratch.cleanup)
        # A declared-PI feat in core_rulebook, and an ordinary non-PI feat
        # alongside it, same shape a real book directory would have.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/feats.lst",
            "Secret Feat\tNAMEISPI:YES\tTYPE:General\n"
            "Ordinary Feat\tNAMEISPI:NO\tTYPE:General\n",
        )
        self.name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        self.declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)

    def test_declared_pi_name_is_replaced_row_survives(self):
        items = [
            item(name="Secret Feat", book="core_rulebook", doneness_raw="done"),
        ]
        standing_by = bps.compute_standing(items)
        bps.attach_standing_and_public_doneness(items, standing_by)
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)
        # The row survives: doneness and standing are untouched by redaction.
        self.assertEqual(items[0]["doneness"], "done")
        self.assertEqual(items[0]["standing"], provenance.ORIGIN)

    def test_non_pi_name_is_left_alone(self):
        items = [item(name="Ordinary Feat", book="core_rulebook")]
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["name"], "Ordinary Feat")

    def test_type_facet_carrying_a_declared_pi_name_is_fully_redacted(self):
        items = [item(name="Ordinary Feat", book="core_rulebook", type_facet="SecretFeatClassFeatures.SpecialQuality")]
        # "Secret Feat" as two-word declared name won't substring-match a
        # camelCase facet; use a single-token declared name instead to
        # exercise the real leak shape (compound identifier embedding).
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/deities.lst",
            "Magaambya\tNAMEISPI:YES\tTYPE:Location\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items[0]["type_facet"] = "MagaambyanInitiateClassFeatures.SpecialQuality.Supernatural"
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["type_facet"], pi_redaction.REDACTED_PI_MARKER)
        # Name itself is untouched -- only the leaking field is withheld.
        self.assertEqual(items[0]["name"], "Ordinary Feat")

    def test_type_facet_with_no_declared_name_substring_is_untouched(self):
        items = [item(name="Ordinary Feat", type_facet="AegisClassFeatures.SpecialQuality.Supernatural")]
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["type_facet"], "AegisClassFeatures.SpecialQuality.Supernatural")

    def test_standing_uses_the_true_name_not_the_redacted_marker(self):
        # Two DIFFERENT declared-PI objects, each printed once, in
        # different books -- if provenance ran on the redacted marker
        # instead of the true name, both would collapse into one fake
        # "object" spanning two books and wrongly come back unclassified.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/advanced_players_guide/feats.lst",
            "Other Secret Feat\tNAMEISPI:YES\tTYPE:General\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [
            item(name="Secret Feat", book="core_rulebook"),
            item(name="Other Secret Feat", book="advanced_players_guide"),
        ]
        standing_by = bps.compute_standing(items)
        bps.attach_standing_and_public_doneness(items, standing_by)
        bps.redact_for_display(items, name_to_books, declared_names)
        for it in items:
            self.assertEqual(it["name"], pi_redaction.REDACTED_PI_MARKER)
            self.assertEqual(it["standing"], provenance.ORIGIN)


class LoadUnitsByKindTests(unittest.TestCase):
    def setUp(self):
        self.scratch = Scratch("units_dir")
        self.addCleanup(self.scratch.cleanup)

    def test_unknown_kind_raises_loud(self):
        import json

        path = self.scratch.write(
            "PF1e-units-mystery.json",
            json.dumps({"kind": "mystery", "fields": ["name", "book", "status", "wiring_class"], "rows": []}),
        )
        with self.assertRaises(KeyError):
            bps.load_units_by_kind(self.scratch.root)


if __name__ == "__main__":
    unittest.main()
