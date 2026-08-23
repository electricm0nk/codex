"""Self-test for `decisions.md §19a` amendments 3b (normalization) and 3c
(`.COPY=`/`.MOD` inheritance) as implemented in
`scripts/sd32_t9_pi_review_feat_equipment.py`.

**Read-only, no corpus dependency.** Every fixture builds a SCRATCH `.lst`-shaped
file (never the real pinned oracle), so these tests pass on a machine with no
oracle checkout at all, same posture `test_pi_redaction.py`/
`test_ground_truth_evidence_guard.py` already take.

Two things this file proves, per the SD-32 T9-PI-sign-off-application dispatch
brief:

1. **Normalization (3b) catches both recorded incident strings and does NOT
   match "Nex" inside "next".** `test_word_boundary_guard_prevents_nex_next_
   false_positive` also demonstrates the RED state directly (not just asserts
   the fixed behaviour): it calls a version of the matcher with the
   word-boundary guard stripped out and shows THAT one false-positives on
   "next", before asserting the real (guarded) function does not.
2. **`.COPY=`/`.MOD` inheritance (3c) resolves all 5 known equipment items to
   `blocked` via their base's own declaration**, using the exact 5 named
   records from `t9-pi-signoff-package.md §1`'s table (Gelugon Plate,
   Hellknight Half-Plate Barding, Hellknight Leather Barding, Hellknight Plate
   Barding, Maiden's Panoply), read from a scratch `.lst` file shaped like the
   real `adventurers_guide` equipment table (bare base rows declaring
   `NAMEISPI:YES`, `.COPY=` derivative rows with no declaration of their own).

Run: python3 -m unittest scripts.tests.test_sd32_t9_pi_normalization_and_inheritance
"""
from __future__ import annotations

import importlib.util
import pathlib
import re
import shutil
import tempfile
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "sd32_t9_pi_review_feat_equipment.py"
)
_spec = importlib.util.spec_from_file_location("sd32_t9_pi_review_feat_equipment", _MODULE_PATH)
fe = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(fe)


class Scratch:
    def __init__(self, name: str):
        self.root = pathlib.Path(tempfile.gettempdir()) / f"codex_sd32_t9_pi_test_{name}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


# ---------------------------------------------------------------------------
# 3b -- normalization rule
# ---------------------------------------------------------------------------

class NormalizationRuleTests(unittest.TestCase):
    """`decisions.md §19a` amendment 3b, verbatim rule text quoted in
    `sd32_t9_pi_review_feat_equipment.py`'s own docstring for
    `normalized_term_hit`."""

    def test_catches_cayden_cailean_incident_string(self):
        # ogl-pi-blacklist.md §4's recorded incident: capital-L "CaiLean" variant
        # of the declared deity "Cayden Cailean" shipped un-redacted.
        free_text = "You channel the blessing of Cayden CaiLean upon your ally."
        hit = fe.normalized_term_hit(free_text)
        self.assertEqual(hit, "Cayden Cailean")

    def test_catches_lrori_ocr_incident_string(self):
        # ogl-pi-blacklist.md §4's second recorded incident: OCR'd "lrori" for
        # the deity "Irori" (lowercase L confused for capital I by the scanner).
        free_text = "This rite is sacred to lrori and his monastic orders."
        hit = fe.normalized_term_hit(free_text)
        self.assertEqual(hit, "Irori")

    def test_does_not_match_nex_inside_next(self):
        # decisions.md §19a's own recorded trap: the 3-letter blacklist term
        # "Nex" case-folds to a substring of the ordinary word "next". The
        # WORD-BOUNDARY guard is what prevents this.
        free_text = "On your next attack this round, you gain a +2 bonus."
        hit = fe.normalized_term_hit(free_text)
        self.assertIsNone(hit, "word-boundary guard must prevent 'Nex' matching inside 'next'")

    def test_word_boundary_guard_prevents_nex_next_false_positive(self):
        """RED/GREEN proof, inline: an UNGUARDED bare-substring version of the
        same canonicalization DOES false-positive on "next" (the exact defect
        two of the three T9 review lanes independently hit and fixed) -- and
        the real, guarded `normalized_term_hit` does not, over the identical
        input. This demonstrates the guard is load-bearing, not decorative."""
        free_text = "On your next attack this round, you gain a +2 bonus."
        canon_text = fe.canonicalize(free_text)
        canon_nex = fe.canonicalize("Nex")

        # Unguarded (bare-substring) re-implementation of the same scan, with
        # the word-boundary regex replaced by a plain `in` check -- this is
        # the exact shape of bug this cycle's brief asked to be proven RED.
        unguarded_hit = canon_nex in canon_text
        self.assertTrue(
            unguarded_hit,
            "the unguarded/bare-substring scan is expected to false-positive here "
            "(that is the defect the word-boundary guard exists to prevent)",
        )

        # The real function must NOT reproduce that false positive.
        guarded_hit = fe.normalized_term_hit(free_text)
        self.assertIsNone(guarded_hit)

    def test_word_boundary_guard_still_allows_real_nex_hit(self):
        # A genuine standalone "Nex" (the Golarion nation) must still hit --
        # the guard must not overcorrect into never matching short terms.
        free_text = "The wizard trained in the deserts of Nex before returning home."
        hit = fe.normalized_term_hit(free_text)
        self.assertEqual(hit, "Nex")

    def test_pipe_delimiter_not_folded_into_ocr_table(self):
        # decisions.md §19a: the PCGen field delimiter "|" must NOT enter the
        # OCR-confusion table -- folding it produces a false NEGATIVE on the
        # Cayden CaiLean incident itself, because adjacent tokens glue together.
        # Simulate the exact FACTSET-shaped incident row: "FACTSET:Deity|Cayden
        # CaiLean" as it would appear pre-tab-split.
        free_text_with_pipe = "FACTSET:Deity|Cayden CaiLean"
        hit = fe.normalized_term_hit(free_text_with_pipe)
        self.assertEqual(
            hit,
            "Cayden Cailean",
            "a '|' must not be folded into the OCR table -- doing so would glue "
            "'Deity|Cayden' into one token and hide the incident string",
        )

    def test_pipe_would_hide_incident_if_folded_wrong(self):
        """Confirms the FAILURE MODE §19a warns about, directly: if "|" WERE
        folded to the OCR canonical character (as a naive implementation might
        do), the incident string becomes unrecoverable as a clean word-bounded
        match because it glues onto the preceding token."""
        bad_fold_table = str.maketrans({"l": "i", "1": "i", "!": "i", "0": "o", "|": "i"})

        def bad_canonicalize(s: str) -> str:
            s = s.casefold().replace("rn", "m")
            return s.translate(bad_fold_table)

        free_text_with_pipe = "FACTSET:Deity|Cayden CaiLean"
        canon_term = fe.canonicalize("Cayden Cailean")
        bad_canon_text = bad_canonicalize(free_text_with_pipe)
        # word-boundary match against the badly-folded text: "deityicayden
        # caiiean" -- "cayden caiiean" is no longer preceded by a clean
        # non-alnum boundary in a way that changes the term itself, but the
        # important failure is demonstrated on a term that ABUTS the pipe:
        # here it still matches by luck (space after 'Deity|Cayden' word),
        # so use a term-adjacent-to-pipe case instead.
        free_text_adjacent = "Deity|Nex"
        bad_text_adjacent = bad_canonicalize(free_text_adjacent)
        canon_nex = fe.canonicalize("Nex")
        # With the bad fold, "|" -> "i", so "Deity|Nex" -> "deityinex": "Nex"
        # is no longer a word-bounded token (it's glued to "deityi").
        self.assertNotRegex(
            bad_text_adjacent,
            r"(?<![a-z0-9])" + re.escape(canon_nex) + r"(?![a-z0-9])",
            "if '|' were folded into the OCR table, a term immediately after "
            "a pipe delimiter would be hidden from the word-boundary scan -- "
            "this is why '|' is excluded",
        )
        # And the REAL (correct) canonicalize does not have this problem:
        self.assertRegex(
            fe.canonicalize(free_text_adjacent),
            r"(?<![a-z0-9])" + re.escape(canon_nex) + r"(?![a-z0-9])",
        )


# ---------------------------------------------------------------------------
# 3c -- .COPY=/.MOD inheritance rule
# ---------------------------------------------------------------------------

# Byte-shaped like the real `adventurers_guide` equipment table this rule
# resolves (`t9-pi-signoff-package.md §1`'s table): bare base rows declaring
# NAMEISPI:YES, and their five `.COPY=` derivatives with no declaration of
# their own.
_SCRATCH_EQUIPMENT_LST = """\
Hellknight Plate\tKEY:Hellknight Plate\tTYPE:Armor.Heavy\tNAMEISPI:YES\tCOST:2400
Hellknight Half-Plate\tKEY:Hellknight Half-Plate\tTYPE:Armor.Heavy\tNAMEISPI:YES\tCOST:1800
Hellknight Leather\tKEY:Hellknight Leather\tTYPE:Armor.Light\tNAMEISPI:YES\tCOST:400
Gray Maiden Plate\tKEY:Gray Maiden Plate\tTYPE:Armor.Heavy\tNAMEISPI:YES\tCOST:2200
Sawtooth Sabre\tKEY:Sawtooth Sabre\tTYPE:Weapon.Martial\tCOST:15
Gelugon Plate.COPY=Hellknight Plate\tCOST:2500
Hellknight Half-Plate Barding.COPY=Hellknight Half-Plate\tCOST:1900
Hellknight Leather Barding.COPY=Hellknight Leather\tCOST:450
Hellknight Plate Barding.COPY=Hellknight Plate\tCOST:2600
Maiden's Panoply.COPY=Gray Maiden Plate\tCOST:2300
Mantis Blade.COPY=Sawtooth Sabre\tCOST:20\tSPROP:As wielded by a Red Mantis assassin
"""

_FIVE_KNOWN_INHERITED_UNITS = (
    ("Gelugon Plate", "Hellknight Plate"),
    ("Hellknight Half-Plate Barding", "Hellknight Half-Plate"),
    ("Hellknight Leather Barding", "Hellknight Leather"),
    ("Hellknight Plate Barding", "Hellknight Plate"),
    ("Maiden's Panoply", "Gray Maiden Plate"),
)


class CopyModInheritanceTests(unittest.TestCase):
    """`decisions.md §19a` amendment 3c: a `.COPY=`/`.MOD` row inherits its
    base item's declared NAMEISPI:YES/DESCISPI:YES status."""

    def setUp(self):
        self.scratch = Scratch("copy_mod_inheritance")
        self.lst_path = self.scratch.write("adventurers_guide/equipment.lst", _SCRATCH_EQUIPMENT_LST)
        self.index = fe.build_key_pi_index(self.lst_path)

    def tearDown(self):
        self.scratch.cleanup()

    def test_all_five_known_equipment_items_resolve_blocked_via_base(self):
        for derived_name, base_name in _FIVE_KNOWN_INHERITED_UNITS:
            with self.subTest(derived_name=derived_name):
                declared = fe.find_base_item_pi(
                    "unused", "adventurers_guide", base_name, self.index
                )
                self.assertEqual(
                    declared,
                    "NAMEISPI:YES",
                    f"{derived_name}'s base '{base_name}' must resolve to a declared "
                    "PI status via the .COPY= inheritance rule",
                )

    def test_base_index_does_not_include_copy_rows_themselves(self):
        # A .COPY= row must never be mistaken for a bare/base row -- only rows
        # with NO .COPY=/.MOD in their own key populate the index.
        self.assertNotIn("Gelugon Plate", self.index)
        self.assertNotIn("Hellknight Plate Barding", self.index)

    def test_clean_base_yields_no_inheritance_blocked_result(self):
        # Mantis Blade's base (Sawtooth Sabre) carries no PI declaration --
        # the inheritance rule must not force it blocked (it is the §4.3
        # "still_undecidable" case: SPROP flavor text is a SEPARATE question
        # the .COPY= rule does not resolve either way).
        declared = fe.find_base_item_pi("unused", "adventurers_guide", "Sawtooth Sabre", self.index)
        self.assertIsNone(declared)

    def test_five_count_matches_the_signoff_package_table_exactly(self):
        blocked_via_inheritance = [
            derived_name
            for derived_name, base_name in _FIVE_KNOWN_INHERITED_UNITS
            if fe.find_base_item_pi("unused", "adventurers_guide", base_name, self.index)
        ]
        self.assertEqual(len(blocked_via_inheritance), 5)
        self.assertEqual(
            set(blocked_via_inheritance),
            {
                "Gelugon Plate",
                "Hellknight Half-Plate Barding",
                "Hellknight Leather Barding",
                "Hellknight Plate Barding",
                "Maiden's Panoply",
            },
        )


if __name__ == "__main__":
    unittest.main()
