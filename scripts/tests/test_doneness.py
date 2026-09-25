#!/usr/bin/env python3
"""Self-test for `scripts/observer/doneness.py` (SD-36 D3 extraction).

Proves the one load-bearing claim this module exists to make: it is
byte-identical, for every (wiring_class, status, kind) combination either
module has a rule for, to `pf1e_dashboard_producer.doneness_verdict` --
the function it was extracted from and must never silently diverge from
until a future cycle deletes the producer outright.

Run: python3 -m unittest -v scripts/tests/test_doneness.py
"""
import importlib.util
import itertools
import pathlib
import unittest

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
_OBSERVER_DIR = _REPO_ROOT / "scripts" / "observer"


def _load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


D = _load("doneness", _OBSERVER_DIR / "doneness.py")
PRODUCER = _load("pf1e_dashboard_producer", _OBSERVER_DIR / "pf1e_dashboard_producer.py")

WIRING_CLASSES = ("ambiguous", "display", "static", "derived", "computed")
STATUSES = (
    "deferred-with-reason", "engine-does-not-hold", "not-started", "sheet-complete",
    "unknown", "unmeasurable", "grounded", "text-complete", "ingested-magnitude",
    "literal-verified", "fixture-verified", "oracle-agree", "oracle-unverifiable",
)
KINDS = (None, "companion", "spell", "feat")


class TestDonenessMatchesProducer(unittest.TestCase):
    def test_every_combination_matches_the_producer(self):
        mismatches = []
        for wc, st, kind in itertools.product(WIRING_CLASSES, STATUSES, KINDS):
            try:
                d = D.doneness_verdict(wc, st, kind)
            except ValueError as exc:
                d = ("ERR", str(exc))
            try:
                p = PRODUCER.doneness_verdict(wc, st, kind)
            except ValueError as exc:
                p = ("ERR", str(exc))
            if d != p:
                mismatches.append((wc, st, kind, d, p))
        self.assertEqual(mismatches, [], f"{len(mismatches)} mismatch(es): {mismatches}")

    def test_excluded_books_matches_the_producer(self):
        self.assertEqual(D.EXCLUDED_BOOKS, PRODUCER.EXCLUDED_BOOKS)

    def test_doneness_values_matches_the_producer(self):
        self.assertEqual(D.DONENESS_VALUES, PRODUCER.DONENESS_VALUES)

    def test_no_grounding_probe_matches_the_producer(self):
        self.assertEqual(D.NO_GROUNDING_PROBE, PRODUCER.NO_GROUNDING_PROBE)

    def test_unknown_wiring_class_raises_in_both(self):
        with self.assertRaises(ValueError):
            D.doneness_verdict("not-a-real-wiring-class", "grounded")
        with self.assertRaises(ValueError):
            PRODUCER.doneness_verdict("not-a-real-wiring-class", "grounded")


if __name__ == "__main__":
    unittest.main()
