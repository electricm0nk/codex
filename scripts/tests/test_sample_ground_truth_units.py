"""scripts/tests/test_sample_ground_truth_units.py -- smoke test for
scripts/sample_ground_truth_units.py (SD31-E2-F1-002, OPEN-ISSUES.md row 4).

Proves the sampler is (a) deterministic given the same seed, (b) actually
stratifies by (wiring_class, kind), (c) honours exclusion, and (d) emits
STUBS ONLY -- no `hand_wiring_class`/`token_evidence`/other verdict field --
consistent with the no-classifier-before-F2 bar.
"""
import unittest

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import sample_ground_truth_units as sampler


def make_units():
    units = []
    for i in range(5):
        units.append(
            {
                "id": f"book_a:class_feature:display_{i}",
                "kind": "class_feature",
                "book": "book_a",
                "name": f"Display {i}",
                "wiring_class": "display",
                "wiring_class_reason": "no_magnitude_token",
                "status": "engine-does-not-hold",
                "corpus_key": f"Display {i}",
                "source_file": "a.lst",
                "source_line": i + 1,
            }
        )
    for i in range(2):
        units.append(
            {
                "id": f"book_a:spell:derived_{i}",
                "kind": "spell",
                "book": "book_a",
                "name": f"Derived {i}",
                "wiring_class": "derived",
                "wiring_class_reason": "bonus",
                "status": "engine-does-not-hold",
                "corpus_key": f"Derived {i}",
                "source_file": "a.lst",
                "source_line": 10 + i,
            }
        )
    return units


class SampleGroundTruthUnitsTest(unittest.TestCase):
    def test_deterministic_given_same_seed(self):
        units = make_units()
        d1 = sampler.draw(units, {}, target_per_cell=3, excluded_ids=set(), seed=31)
        d2 = sampler.draw(units, {}, target_per_cell=3, excluded_ids=set(), seed=31)
        self.assertEqual([r["id"] for r in d1], [r["id"] for r in d2])

    def test_different_seed_can_differ(self):
        units = make_units()
        d1 = sampler.draw(units, {}, target_per_cell=3, excluded_ids=set(), seed=31)
        d2 = sampler.draw(units, {}, target_per_cell=3, excluded_ids=set(), seed=32)
        # display:class_feature has 5 candidates, target 3 -- the drawn
        # subset is very likely to differ across seeds (not guaranteed by
        # construction, but true for this fixture/these two seeds).
        d1_ids = {r["id"] for r in d1 if r["cell"] == "display:class_feature"}
        d2_ids = {r["id"] for r in d2 if r["cell"] == "display:class_feature"}
        self.assertNotEqual(d1_ids, d2_ids)

    def test_stratifies_and_respects_target_and_gap(self):
        units = make_units()
        # Already have 2 of the 3 target display:class_feature units --
        # should draw exactly 1 more of them.
        drawn = sampler.draw(
            units, {"display:class_feature": 2, "derived:spell": 0}, target_per_cell=3, excluded_ids=set(), seed=31
        )
        by_cell = {}
        for r in drawn:
            by_cell.setdefault(r["cell"], []).append(r)
        self.assertEqual(len(by_cell.get("display:class_feature", [])), 1)
        # derived:spell only has 2 candidates total -- cannot reach target 3.
        self.assertEqual(len(by_cell.get("derived:spell", [])), 2)

    def test_excluded_ids_never_drawn(self):
        units = make_units()
        excluded = {"book_a:class_feature:display_0", "book_a:class_feature:display_1"}
        drawn = sampler.draw(units, {}, target_per_cell=5, excluded_ids=excluded, seed=31)
        drawn_ids = {r["id"] for r in drawn}
        self.assertFalse(drawn_ids & excluded)

    def test_output_carries_no_verdict_fields(self):
        units = make_units()
        drawn = sampler.draw(units, {}, target_per_cell=1, excluded_ids=set(), seed=31)
        self.assertTrue(drawn)
        for rec in drawn:
            self.assertNotIn("hand_wiring_class", rec)
            self.assertNotIn("token_evidence", rec)
            self.assertNotIn("confidence", rec)
            self.assertNotIn("agrees_with_engine", rec)

    def test_output_carries_magnitude_token_count(self):
        # SD31-D7-PROSE-001: the sample must carry the raw
        # `magnitude_token_count` field alongside the wiring_class stub so a
        # reader can see which proxy (magnitude_token_count==0 alone, vs.
        # engine `wiring_class`) put each unit in the draw -- required to
        # measure the two proxies' precision/recall separately.
        units = make_units()
        for u in units:
            u["magnitude_token_count"] = 0
        drawn = sampler.draw(units, {}, target_per_cell=1, excluded_ids=set(), seed=31)
        self.assertTrue(drawn)
        for rec in drawn:
            self.assertIn("magnitude_token_count", rec)
            self.assertEqual(rec["magnitude_token_count"], 0)

    def test_zero_magnitude_only_excludes_nonzero_units(self):
        # Decision 7's PROXY WARNING: the sample this card draws must come
        # from the `magnitude_token_count == 0` population specifically, not
        # the whole board -- a unit carrying real magnitude tokens has
        # nothing to do with the proxy under test.
        units = make_units()
        for i, u in enumerate(units):
            u["magnitude_token_count"] = 0 if i % 2 == 0 else 3
        drawn = sampler.draw(
            units, {}, target_per_cell=10, excluded_ids=set(), seed=31, zero_magnitude_only=True
        )
        self.assertTrue(drawn)
        for rec in drawn:
            self.assertEqual(rec["magnitude_token_count"], 0)

    def test_zero_magnitude_only_false_is_unfiltered_default(self):
        # Default behaviour is unchanged for every existing caller (Epic 2's
        # own draws never passed the flag).
        units = make_units()
        for i, u in enumerate(units):
            u["magnitude_token_count"] = 0 if i % 2 == 0 else 3
        drawn_default = sampler.draw(units, {}, target_per_cell=10, excluded_ids=set(), seed=31)
        counts = {r["magnitude_token_count"] for r in drawn_default}
        self.assertIn(3, counts, "unfiltered draw must still be able to pick a nonzero unit")


if __name__ == "__main__":
    unittest.main()
