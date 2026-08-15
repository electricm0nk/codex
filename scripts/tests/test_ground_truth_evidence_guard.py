"""scripts/tests/test_ground_truth_evidence_guard.py -- the detection
self-test for scripts/ground_truth_evidence_guard.py (SD31-E2-F1-002,
OPEN-ISSUES.md row 3).

Builds a small, hermetic FAKE corpus tree and a matching FAKE ground-truth
sample under a temp directory for every case -- never touches the real
$PCGEN_CORPUS_ROOT or the live SD31-E2-F1 sample. Proves the guard can both
PASS a genuinely-evidenced record and FAIL each of the three defect shapes
it exists to catch ("this repo has shipped three gates that could not
fail" -- SD31-E2-F1-002's brief):

  * ABSENT      -- token_evidence missing/blank
  * DUPLICATED  -- token_evidence byte-identical across two records (the
                   exact shape of the original 105-unit defect)
  * UNGROUNDED  -- token_evidence quotes tokens that do not occur anywhere
                   in the record's own corpus row (a fabricated quote)

Also proves it does NOT flag a genuinely well-evidenced record (a required
companion to "can fail" -- a guard that always fails is exactly as useless
as one that never does), and proves the nested-directory nested-path case
(OPEN-ISSUES.md row 1's bug) resolves correctly via `corpus_path_verified`
without needing the buggy single-level join.
"""
import json
import os
import tempfile
import unittest

import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import ground_truth_evidence_guard as guard


class GroundTruthEvidenceGuardTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.corpus_root = self._tmp.name
        # Fake corpus shape: pathfinder/paizo/roleplaying_game/<book>/<file>.lst
        self.book_dir = os.path.join(
            self.corpus_root, "pathfinder", "paizo", "roleplaying_game", "fake_book"
        )
        os.makedirs(self.book_dir, exist_ok=True)
        self._write(
            "fb_abilities_class.lst",
            "Real Feature\tKEY:Real Feature\tCATEGORY:Special Ability\t"
            "TYPE:SpecialQuality.Extraordinary\tDESC:A wholly ordinary text-only ability.\n",
        )
        # A record reachable only via a nested subdirectory -- mirrors
        # OPEN-ISSUES.md row 1's single-level-join bug shape.
        nested_dir = os.path.join(self.book_dir, "nested", "deeper")
        os.makedirs(nested_dir, exist_ok=True)
        with open(
            os.path.join(nested_dir, "fb_nested.lst"), "w", encoding="utf-8"
        ) as fh:
            fh.write(
                "Nested Feature\tKEY:Nested Feature\tCATEGORY:Special Ability\t"
                "BONUS:VAR|NestedBonus|2\n"
            )

    def _write(self, filename, text):
        with open(os.path.join(self.book_dir, filename), "w", encoding="utf-8") as fh:
            fh.write(text)

    def _base_record(self, **overrides):
        rec = {
            "id": "fake_book:class_feature:real_feature",
            "kind": "class_feature",
            "book": "fake_book",
            "name": "Real Feature",
            "source_file": "fb_abilities_class.lst",
            "source_line": 1,
            "engine_wiring_class": "display",
            "hand_wiring_class": "display",
        }
        rec.update(overrides)
        return rec

    # -- the "can fail" cases -----------------------------------------

    def test_fabricated_canned_duplicate_is_caught(self):
        """Feeds the guard a FABRICATED canned record: two units sharing
        one byte-identical, non-record-specific evidence string -- exactly
        the shape of the original 105-unit defect. Must fail."""
        canned = "confirmed from the unit's full token closure -- matches engine's own wiring_class"
        units = [
            self._base_record(id="fake_book:class_feature:real_feature", token_evidence=canned),
            self._base_record(id="fake_book:class_feature:real_feature", name="Real Feature", token_evidence=canned),
        ]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertTrue(violations, "guard failed to catch a byte-identical duplicated evidence pair")
        self.assertTrue(any("byte-identical" in v for v in violations))

    def test_absent_evidence_is_caught(self):
        units = [self._base_record(token_evidence="")]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertTrue(violations)
        self.assertTrue(any("absent/empty" in v for v in violations))

        units2 = [self._base_record()]  # no token_evidence key at all
        violations2 = guard.check_sample(units2, self.corpus_root, "fixture")
        self.assertTrue(violations2)
        self.assertTrue(any("absent/empty" in v for v in violations2))

    def test_fabricated_ungrounded_quote_is_caught(self):
        """A quote that simply does not appear in the corpus row anywhere
        -- a fabricated/hallucinated token. Must fail."""
        fabricated = (
            "This record is genuinely computed. Quoted tokens (verbatim from the row(s) below): "
            "BONUS:VAR|ThisTokenIsMadeUpAndNowhereInTheCorpus|99"
        )
        units = [self._base_record(token_evidence=fabricated)]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertTrue(violations, "guard failed to catch a fabricated, ungrounded quote")

    def test_ungrounded_freeform_evidence_is_caught(self):
        """Free-form (non-marker) evidence with no real corpus overlap at
        all. Must fail."""
        units = [self._base_record(token_evidence="This is a wholly generic description with no real quotes in it whatsoever.")]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertTrue(violations)

    # -- the "does not cry wolf" case ----------------------------------

    def test_genuinely_evidenced_record_passes(self):
        real_evidence = (
            "Whole row read: no MAGNITUDE_TOKENS field present. Quoted tokens (verbatim "
            "from the row(s) below): KEY:Real Feature | DESC:A wholly ordinary text-only ability."
        )
        units = [self._base_record(token_evidence=real_evidence)]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertEqual(violations, [], f"genuinely-evidenced record was wrongly flagged: {violations}")

    def test_freeform_genuinely_evidenced_record_passes(self):
        real_evidence = (
            "Row carries only DESC:A wholly ordinary text-only ability. and no magnitude field -> display."
        )
        units = [self._base_record(token_evidence=real_evidence)]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertEqual(violations, [], f"genuinely-evidenced free-form record was wrongly flagged: {violations}")

    def test_nested_path_resolves_via_recursive_search(self):
        """A record whose row lives two directories below the book root
        (OPEN-ISSUES.md row 1's shape) must still be found and verified --
        proves this guard does not share the production single-level-join
        bug it exists to help catch."""
        real_evidence = (
            "Quoted tokens (verbatim from the row(s) below): BONUS:VAR|NestedBonus|2"
        )
        units = [
            self._base_record(
                id="fake_book:class_feature:nested_feature",
                name="Nested Feature",
                source_file="fb_nested.lst",
                source_line=1,
                token_evidence=real_evidence,
            )
        ]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertEqual(violations, [], f"nested-path record wrongly flagged: {violations}")

    def test_corpus_path_verified_is_honoured(self):
        """A record naming its own verified read-path (SD31-E2-F1-002's
        `corpus_path_verified` field) is checked against exactly that path
        without needing a filename search at all."""
        real_evidence = (
            "Quoted tokens (verbatim from the row(s) below): BONUS:VAR|NestedBonus|2"
        )
        units = [
            self._base_record(
                id="fake_book:class_feature:nested_feature",
                name="Nested Feature",
                source_file="fb_nested.lst",
                source_line=1,
                corpus_path_verified=["nested/deeper/fb_nested.lst"],
                token_evidence=real_evidence,
            )
        ]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertEqual(violations, [], f"corpus_path_verified record wrongly flagged: {violations}")

    def test_unknown_book_is_reported_not_silently_skipped(self):
        units = [
            self._base_record(
                book="a_book_that_does_not_exist_anywhere",
                token_evidence="Some plausible-looking evidence text of reasonable length.",
            )
        ]
        violations = guard.check_sample(units, self.corpus_root, "fixture")
        self.assertTrue(violations)
        self.assertTrue(any("no known corpus directory" in v for v in violations))


if __name__ == "__main__":
    unittest.main()
