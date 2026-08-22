"""Self-test for `scripts/supersession_register_gate.py` (SD31-D10-REGISTER-001).

WHY THIS EXISTS
----------------
`SD-30 state-goals-and-lessons.md §3.1`: this repo has shipped three gates
that could not fail, each caught only by running it against a known-answer
case. Decision 10's own register is the FIRST authorization in this
package to shrink the mandate denominator, and it is a standing rule a
cycle may apply without a per-entry operator signature -- so this gate,
not a signature, is the only thing protecting that number. This test seeds
one BAD entry of each of the two refusal shapes the card demands, plus one
structural-violation shape, and confirms each is both reported and fails
the gate's exit code -- then confirms a genuinely-clean register (and one
variant-line entry carrying real `reprint_proof`) passes.

Hermetic: builds a small fake corpus tree under a temp dir (same pattern
`test_reachability_audit.py`/`test_ground_truth_evidence_guard.py` use) so
this runs with no dependency on the real pinned oracle checkout.

Run: python3 -m unittest scripts/tests/test_supersession_register_gate.py
Wired as the `supersession-gate-selftest` stage in `scripts/verify.sh`.
"""
from __future__ import annotations

import importlib.util
import os
import pathlib
import tempfile
import unittest

_MODULE_PATH = pathlib.Path(__file__).resolve().parent.parent / "supersession_register_gate.py"
_spec = importlib.util.spec_from_file_location("supersession_register_gate", _MODULE_PATH)
gate_mod = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(gate_mod)


def _write(path: pathlib.Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


class HermeticCorpusMixin:
    """Builds a tiny fake corpus tree with exactly the books/files/lines
    the tests below reference, and a matching FileFinder."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self._tmp.name)
        book_dirs = {
            "bookA": "fake/bookA",
            "bookB": "fake/bookB",
            "pathfinder_unchained": "fake/pathfinder_unchained",
        }
        # bookA and bookB: a genuinely identical row (same name, same
        # TYPE tags in a DIFFERENT order -- proves order-insensitivity),
        # differing only in SOURCEPAGE (stripped) and SOURCEDATE (not part
        # of the row at all, lives in the .pcc header in the real corpus).
        _write(self.root / "fake/bookA/a.lst",
               "Widget of Testing\tTYPE:Wondrous.Minor\tBONUS:AC|1|natural\tSOURCEPAGE:p.1\n")
        _write(self.root / "fake/bookB/b.lst",
               "Widget of Testing\tTYPE:Minor.Wondrous\tBONUS:AC|1|natural\tSOURCEPAGE:p.99\n")
        # a MATERIALLY different row sharing the same corpus_key -- must
        # be refused if ever presented as a proven pair (refusal 1).
        _write(self.root / "fake/bookB/b2.lst",
               "Widget of Testing\tTYPE:Minor.Wondrous\tBONUS:AC|2|natural\tSOURCEPAGE:p.5\n")
        # pathfinder_unchained: a variant, not a reprint.
        _write(self.root / "fake/pathfinder_unchained/pu.lst",
               "Widget of Testing\tTYPE:Wondrous.Minor\tBONUS:AC|1|natural\tSOURCEPAGE:p.7\n")

        self.finder = gate_mod.FileFinder(str(self.root), book_dirs=book_dirs)

    def tearDown(self):
        self._tmp.cleanup()


# Each fake book's own .lst basename -- used to resolve a caller's default
# `source_file` from the BOOK it names, not from argument position. (Fixed
# 2026-08-16, SD31-W7-INTEGRATE-001: the old fixed defaults
# `surviving_file="a.lst"`/`superseded_file="b.lst"` silently mismatched
# whenever a caller passed `bookB` as the surviving side -- e.g.
# `_entry("bookB", ..., "bookA", ...)` -- and every test in this file did.
# That mismatch was invisible under the gate's old dead-fallback bug (a
# lookup that fails on BOTH sides compared `None == None` as equal and
# passed silently); fixing the gate's real-resolution requirement exposed
# it. Deriving the default file from the book name closes it at the root
# for every caller, not just the ones this cycle touched.)
_DEFAULT_FILE_FOR_BOOK = {"bookA": "a.lst", "bookB": "b.lst", "pathfinder_unchained": "pu.lst"}


def _entry(surviving_book, surviving_date, superseded_book, superseded_date,
           surviving_file=None, surviving_line=1,
           superseded_file=None, superseded_line=1,
           reprint_proof=None):
    if surviving_file is None:
        surviving_file = _DEFAULT_FILE_FOR_BOOK.get(surviving_book, f"{surviving_book}.lst")
    if superseded_file is None:
        superseded_file = _DEFAULT_FILE_FOR_BOOK.get(superseded_book, f"{superseded_book}.lst")
    e = {
        "kind": "equipment",
        "corpus_key": "Widget of Testing",
        "surviving": {"id": f"{surviving_book}:equipment:widget_of_testing", "book": surviving_book,
                       "source_date": surviving_date, "source_file": surviving_file,
                       "source_line": surviving_line},
        "superseded": [
            {"id": f"{superseded_book}:equipment:widget_of_testing", "book": superseded_book,
             "source_date": superseded_date, "source_file": superseded_file,
             "source_line": superseded_line},
        ],
    }
    if reprint_proof is not None:
        e["reprint_proof"] = reprint_proof
    return e


class CleanEntryPassesTest(HermeticCorpusMixin, unittest.TestCase):
    """Decision 13 (2026-08-17): for an IDENTICAL pair the FIRST print owns
    it. A clean, passing entry now has the OLDER book as `surviving` and
    the NEWER book as `superseded` -- the opposite of the pre-Decision-13
    shape."""

    def test_genuinely_identical_reordered_type_tags_passes(self):
        entry = _entry("bookA", "2009-08", "bookB", "2012-01")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertEqual(violations, [], f"a genuinely identical pair must clear the gate: {violations}")

    def test_full_register_with_one_clean_entry_is_ok(self):
        register = {"objects": [_entry("bookA", "2009-08", "bookB", "2012-01")],
                    "denominator": {"count_removed": 1}}
        result = gate_mod.validate_register(register, self.finder)
        self.assertTrue(result["ok"], result["violations"])


class MaterialDifferenceRefusalTest(HermeticCorpusMixin, unittest.TestCase):
    """The gate must be proven able to fail on refusal 1: two records that
    are NOT the same object (a genuinely different BONUS magnitude) must
    never be silently accepted as a supersession pair. Direction is the
    Decision-13-correct shape (surviving = older/first print); the
    materially-different content is planted on the SUPERSEDED side
    (`b2.lst`) instead."""

    def test_materially_different_bonus_is_refused(self):
        entry = _entry("bookA", "2009-08", "bookB", "2012-01",
                        superseded_file="b2.lst", superseded_line=1)
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(
            any("do NOT carry identical" in v for v in violations),
            f"a materially different pair must be refused, got: {violations}",
        )

    def test_full_register_with_one_bad_entry_fails_the_gate(self):
        entry = _entry("bookA", "2009-08", "bookB", "2012-01", superseded_file="b2.lst")
        register = {"objects": [entry], "denominator": {"count_removed": 1}}
        result = gate_mod.validate_register(register, self.finder)
        self.assertFalse(result["ok"], "a materially-different entry must fail validate_register")
        self.assertTrue(result["violations"])


class VariantLineRefusalTest(HermeticCorpusMixin, unittest.TestCase):
    """Refusal 2: pathfinder_unchained/mythic_adventures default to VARIANT.
    An entry naming either without `reprint_proof` must be refused; one
    that carries real, non-empty `reprint_proof` must be allowed through
    (the guard is proof-gated, not an unconditional ban)."""

    def test_variant_line_without_reprint_proof_is_refused(self):
        entry = _entry("pathfinder_unchained", "2015-04", "bookA", "2009-08",
                        surviving_file="pu.lst")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(
            any("VARIANT" in v for v in violations),
            f"a variant-line entry with no reprint_proof must be refused, got: {violations}",
        )

    def test_variant_line_with_reprint_proof_is_allowed(self):
        entry = _entry("pathfinder_unchained", "2015-04", "bookA", "2009-08",
                        surviving_file="pu.lst",
                        reprint_proof=(
                            "record-level: pu.lst's own row is byte-identical to "
                            "bookA's and PCGen ships no separate 'Unchained' variant "
                            "of this specific object (hand-verified against the "
                            "pinned oracle, not merely key-matched)"
                        ))
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertEqual(
            [v for v in violations if "VARIANT" in v], [],
            f"an entry with real reprint_proof must clear the variant guard: {violations}",
        )

    def test_empty_string_reprint_proof_still_refused(self):
        """A present-but-blank `reprint_proof` field must not silently
        satisfy the guard -- mutation-proof against the trivial bypass."""
        entry = _entry("pathfinder_unchained", "2015-04", "bookA", "2009-08",
                        surviving_file="pu.lst", reprint_proof="   ")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(any("VARIANT" in v for v in violations))

    def test_mythic_adventures_on_the_superseded_side_is_also_refused(self):
        entry = _entry("bookA", "2013-08", "mythic_adventures", "2009-08")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(any("VARIANT" in v for v in violations))


class StructuralRefusalTest(HermeticCorpusMixin, unittest.TestCase):
    def test_bare_integer_corpus_key_is_refused(self):
        """The `companion` key `"1"` defect: a bare-integer corpus_key is a
        PCGen level-number continuation row, never an object identity."""
        entry = _entry("bookB", "2012-01", "bookA", "2009-08")
        entry["corpus_key"] = "1"
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(
            any("bare integer" in v for v in violations),
            f"a bare-integer corpus_key must be refused, got: {violations}",
        )

    def test_core_essentials_is_refused_on_either_side(self):
        entry = _entry("bookA", "2009-08", "core_essentials", "2009-08")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(any("core_essentials" in v for v in violations))

    def test_survivor_newer_than_superseded_is_refused(self):
        # Decision 13 (2026-08-17): for an identical pair the FIRST print
        # owns it. A "newest wins" shaped entry -- surviving is NEWER than
        # the thing it supposedly supersedes -- must now be refused.
        entry = _entry("bookB", "2012-01", "bookA", "2005-01")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(any("NEWER than superseded" in v for v in violations))

    def test_denominator_count_removed_mismatch_is_refused(self):
        register = {"objects": [_entry("bookB", "2012-01", "bookA", "2009-08")],
                    "denominator": {"count_removed": 99}}
        result = gate_mod.validate_register(register, self.finder)
        self.assertFalse(result["ok"])
        self.assertTrue(any("count_removed" in v for v in result["violations"]))


class MissingEvidenceRefusalTest(HermeticCorpusMixin, unittest.TestCase):
    """The defect this cycle fixed: an entry with no `source_file`/
    `source_line` on a side must be a HARD violation, never silently
    accepted via a fallback to the register's own cached `raw_lines` --
    that fallback was dead code in production (no shipped entry ever
    carried source_file/source_line) and let a wholly fabricated entry,
    or one with its raw_lines swapped for nonsense, pass clean."""

    def test_a_side_with_no_source_file_or_line_is_refused(self):
        entry = _entry("bookB", "2012-01", "bookA", "2009-08")
        del entry["surviving"]["source_file"]
        del entry["surviving"]["source_line"]
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(
            any("no source_file/source_line" in v for v in violations),
            f"a side with no re-derivable citation must be refused, got: {violations}",
        )

    def test_a_fabricated_entry_with_raw_lines_but_no_citation_is_refused(self):
        """The exact shape that passed before this fix: raw_lines present
        (even set to something bogus), source_file/source_line absent."""
        entry = _entry("bookB", "2012-01", "bookA", "2009-08")
        del entry["surviving"]["source_file"]
        del entry["surviving"]["source_line"]
        entry["raw_lines"] = {
            "bookB": "CLASS:Totally Fabricated Object\tHD:99\tTYPE:PC.Nonsense",
            "bookA": "CLASS:Totally Fabricated Object\tHD:99\tTYPE:PC.Nonsense",
        }
        register = {"objects": [entry], "denominator": {"count_removed": 1}}
        result = gate_mod.validate_register(register, self.finder)
        self.assertFalse(result["ok"], "a fabricated entry must never pass just because raw_lines is present")

    def test_a_citation_the_oracle_cannot_resolve_is_refused(self):
        entry = _entry("bookB", "2012-01", "bookA", "2009-08",
                        surviving_file="does_not_exist.lst")
        violations = gate_mod.validate_entry(entry, self.finder)
        self.assertTrue(
            any("could not be re-derived" in v for v in violations),
            f"an unresolvable citation must be refused, got: {violations}",
        )


class NoOracleStructuralOnlyTest(unittest.TestCase):
    """When no corpus root is available the gate still runs its structural
    checks (variant/core_essentials/date-order) rather than silently
    passing everything -- `finder=None` must not turn refusal 1 into a
    no-op that would let refusal-1-shaped bad data through unnoticed."""

    def test_variant_refusal_still_fires_with_no_finder(self):
        entry = _entry("pathfinder_unchained", "2015-04", "bookA", "2009-08")
        violations = gate_mod.validate_entry(entry, None)
        self.assertTrue(any("VARIANT" in v for v in violations))


if __name__ == "__main__":
    unittest.main()
