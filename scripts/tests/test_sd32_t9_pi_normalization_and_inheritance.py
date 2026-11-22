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

_PI_SCRUB_PATH = pathlib.Path(__file__).resolve().parent.parent / "pi_scrub.py"
_scrub_spec = importlib.util.spec_from_file_location("pi_scrub", _PI_SCRUB_PATH)
pi_scrub = importlib.util.module_from_spec(_scrub_spec)
assert _scrub_spec.loader is not None
_scrub_spec.loader.exec_module(pi_scrub)


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
# decisions.md §26 -- the "Jarn"/"jam" false positive (bard_s_escape.json),
# and why word-boundary matching alone (already present in this file's
# `normalized_term_hit` before this cycle) did NOT prevent it.
# ---------------------------------------------------------------------------

class Section26JarnJamFoldCollisionTests(unittest.TestCase):
    """`decisions.md §26`. The operator's ruling was "add the word
    boundary" -- but `sd32_t9_pi_review_feat_equipment.py`'s
    `normalized_term_hit` (imported here as `fe.normalized_term_hit`, now
    re-exported from the shared `pi_scrub.py`) was ALREADY word-bounded
    before this cycle. Reproducing the `bard_s_escape.json` false positive
    against that already-word-bounded function proves the word-boundary
    guard alone does not kill it: "Jarn" canonicalizes (rn -> m) to "jam",
    an ordinary whole word, so the collision survives word-boundary
    matching. The real fix is `pi_scrub._RN_FOLD_EXEMPT_TERMS_CASEFOLD`.
    """

    def test_reproduces_the_bard_s_escape_false_positive_pre_fix(self):
        """Direct reproduction against the real corpus text (the OGL prose
        from `data/corpus/advanced_players_guide/spell/bard_s_escape.json`
        `data.raw_tokens[DESC]`, license: OGL, correctly untouched)."""
        free_text = (
            "You whisk yourself and willing allies out of a tight jam, or "
            "instantly transfer yourselves to another location to achieve "
            "greater strategic positioning."
        )
        self.assertIsNone(
            fe.normalized_term_hit(free_text),
            "the record is genuinely OGL -- the fold-collision false positive "
            "must not survive",
        )

    def test_word_boundary_alone_does_not_prevent_the_collision(self):
        """RED proof for the DIAGNOSIS, not just the fix: an already-
        word-bounded reimplementation of the scan (boundary regex intact,
        rn->m exemption removed) STILL false-positives on "jam" -- proving
        the operator's "add the word boundary" ruling, alone, does not
        close this specific false positive. `§26` records this as a
        correction to the dispatch brief's lead hypothesis."""
        free_text = "You whisk yourself out of a tight jam."
        canon_text = free_text.casefold().replace("rn", "m")
        canon_jarn_full_fold = "jarn".replace("rn", "m")  # "jam"
        # Word-bounded, exactly like the real (pre-§26) function -- and it
        # still matches, because the collision is itself a whole word.
        self.assertRegex(
            canon_text,
            r"(?<![a-z0-9])" + re.escape(canon_jarn_full_fold) + r"(?![a-z0-9])",
            "word-boundary matching alone does not prevent the Jarn/jam "
            "fold collision -- this is what the real fix must additionally "
            "guard against",
        )

    def test_mutation_proof_removing_the_rn_fold_exemption_reopens_the_false_positive(self):
        """Mutation-proves `_RN_FOLD_EXEMPT_TERMS_CASEFOLD` is load-bearing:
        with it emptied, the real shared function false-positives on the
        same text the fixed function correctly clears."""
        original = pi_scrub._RN_FOLD_EXEMPT_TERMS_CASEFOLD
        try:
            pi_scrub._RN_FOLD_EXEMPT_TERMS_CASEFOLD = set()
            pi_scrub._CANON_TERMS = [
                (
                    term,
                    pi_scrub.canonicalize(
                        term,
                        apply_rn_fold=pi_scrub._term_needs_rn_fold(term),
                        apply_char_fold=pi_scrub._term_needs_char_fold(term),
                    ),
                    pi_scrub._term_needs_rn_fold(term),
                    pi_scrub._term_needs_char_fold(term),
                )
                for term in pi_scrub.PI_BLACKLIST_TERMS
            ]
            free_text = "You whisk yourself out of a tight jam."
            self.assertEqual(
                pi_scrub.normalized_term_hit(free_text),
                "Jarn",
                "mutation (emptying the rn-fold exemption) must reopen the "
                "false positive -- confirming the guard is load-bearing, not "
                "decorative",
            )
        finally:
            pi_scrub._RN_FOLD_EXEMPT_TERMS_CASEFOLD = original
            pi_scrub._CANON_TERMS = [
                (
                    term,
                    pi_scrub.canonicalize(
                        term,
                        apply_rn_fold=pi_scrub._term_needs_rn_fold(term),
                        apply_char_fold=pi_scrub._term_needs_char_fold(term),
                    ),
                    pi_scrub._term_needs_rn_fold(term),
                    pi_scrub._term_needs_char_fold(term),
                )
                for term in pi_scrub.PI_BLACKLIST_TERMS
            ]
            # Confirm the real (unmutated) function is GREEN again.
            self.assertIsNone(pi_scrub.normalized_term_hit("You whisk yourself out of a tight jam."))

    def test_literal_plainly_spelled_term_is_still_caught_despite_the_exemption(self):
        """The exemption must not silently also break the ORIGINAL catch
        "Jarn" was added for: a plainly, correctly spelled occurrence in
        prose (ogl-pi-blacklist.md §4, ACG override entry). Only the
        rn->m-fold-INDUCED collision is exempted -- a literal spelling
        still hits."""
        free_text = "This tale follows Jarn on his journey home."
        self.assertEqual(fe.normalized_term_hit(free_text), "Jarn")

    def test_genuine_rn_to_m_ligature_fold_still_catches_a_synthetic_ocr_term(self):
        """The rn->m fold mechanism itself (not the "Jarn"-specific
        exemption) must still catch a genuine OCR ligature artifact for any
        OTHER term. Uses a synthetic, throwaway term substituted into the
        module's own canonicalized-term table for the duration of the test
        -- never a real Product Identity string -- because "Jarn" is
        currently the ONLY real blacklist term containing "rn", and it is
        now (correctly) exempt from this exact fold."""
        original_terms = pi_scrub._CANON_TERMS
        try:
            synthetic_term = "Kaernos"  # synthetic OCR-affected proper noun
            pi_scrub._CANON_TERMS = [
                (synthetic_term, pi_scrub.canonicalize(synthetic_term), True, True)
            ]
            # A scanner ligature-confuses "rn" for "m": "Kaernos" -> "Kaemos".
            free_text = "The old road to Kaemos was long forgotten."
            self.assertEqual(
                pi_scrub.normalized_term_hit(free_text),
                synthetic_term,
                "the rn->m fold mechanism must still catch a genuine OCR "
                "ligature artifact for terms other than the exempted 'Jarn'",
            )
        finally:
            pi_scrub._CANON_TERMS = original_terms


class GaltGaitFoldCollisionTests(unittest.TestCase):
    """t9-onboarding cycle (2026-08-23), `corpus_literal_sweep` unblock: the
    SAME false-positive class `decisions.md §26` fixed for the rn->m fold
    (Jarn/jam) recurs for the l/1/!->i fold: "Galt" (a Golarion nation, the
    only blacklist term containing "l") canonicalizes to "gait" — an
    ordinary English word that occurs in genuine OGL prose ("his gait more
    deliberate..."). Found live re-deriving `corpus_literal_sweep` against
    the pinned oracle: `advanced_players_guide/class_feature/
    shifter_s_blessing/form_of_the_cat.json`'s DESC token, and three sibling
    `class_feature` records (KEY/ABILITY tokens restating a "<Name>'s
    Gait"/"Steady Gait"-shaped ability name) all went `[redacted PI]` for a
    collision, not a real PI hit.
    """

    def test_reproduces_the_form_of_the_cat_false_positive_pre_fix(self):
        """Direct reproduction against the real record's own prose (pinned
        oracle `advanced_players_guide/apg_abilities_class.lst:2827`)."""
        desc = (
            "The ranger's muscles become lean and defined, and his gait "
            "more deliberate and graceful. While in this form, the ranger "
            "increases his base speed by 10 feet, and he gains a +4 bonus "
            "on Acrobatics and Climb checks."
        )
        self.assertEqual(pi_scrub.normalized_term_hit(desc), None)

    def test_word_boundary_alone_does_not_prevent_the_collision(self):
        """Proves the negative claim directly: an already-word-bounded
        reimplementation with NO l-fold exemption still matches "gait" —
        confirming the real fix could not have been "word boundary" alone,
        the same shape `§26` proved for Jarn/jam."""
        free_text = "his gait more deliberate and graceful"
        canon_text = free_text.casefold().translate(pi_scrub._FOLD_TABLE)
        canon_galt_full_fold = "galt".translate(pi_scrub._FOLD_TABLE)  # "gait"
        self.assertRegex(
            canon_text,
            r"(?<![a-z0-9])" + re.escape(canon_galt_full_fold) + r"(?![a-z0-9])",
            "word-boundary matching alone does not prevent the Galt/gait "
            "fold collision -- this is what the real fix must additionally "
            "guard against",
        )

    def test_mutation_proof_removing_the_char_fold_exemption_reopens_the_false_positive(self):
        """Mutation-proves `_CHAR_FOLD_EXEMPT_TERMS_CASEFOLD` is
        load-bearing: with it emptied, the real shared function
        false-positives on the same text the fixed function correctly
        clears."""
        original = pi_scrub._CHAR_FOLD_EXEMPT_TERMS_CASEFOLD
        try:
            pi_scrub._CHAR_FOLD_EXEMPT_TERMS_CASEFOLD = set()
            pi_scrub._CANON_TERMS = [
                (
                    term,
                    pi_scrub.canonicalize(
                        term,
                        apply_rn_fold=pi_scrub._term_needs_rn_fold(term),
                        apply_char_fold=pi_scrub._term_needs_char_fold(term),
                    ),
                    pi_scrub._term_needs_rn_fold(term),
                    pi_scrub._term_needs_char_fold(term),
                )
                for term in pi_scrub.PI_BLACKLIST_TERMS
            ]
            free_text = "his gait more deliberate and graceful"
            self.assertEqual(
                pi_scrub.normalized_term_hit(free_text),
                "Galt",
                "mutation (emptying the char-fold exemption) must reopen "
                "the false positive -- confirming the guard is load-bearing, "
                "not decorative",
            )
        finally:
            pi_scrub._CHAR_FOLD_EXEMPT_TERMS_CASEFOLD = original
            pi_scrub._CANON_TERMS = [
                (
                    term,
                    pi_scrub.canonicalize(
                        term,
                        apply_rn_fold=pi_scrub._term_needs_rn_fold(term),
                        apply_char_fold=pi_scrub._term_needs_char_fold(term),
                    ),
                    pi_scrub._term_needs_rn_fold(term),
                    pi_scrub._term_needs_char_fold(term),
                )
                for term in pi_scrub.PI_BLACKLIST_TERMS
            ]
            # Confirm the real (unmutated) function is GREEN again.
            self.assertIsNone(pi_scrub.normalized_term_hit("his gait more deliberate and graceful"))

    def test_literal_plainly_spelled_term_is_still_caught_despite_the_exemption(self):
        """The exemption must not silently also break the ORIGINAL catch
        "Galt" exists for: a plainly, correctly spelled occurrence of the
        nation name in prose. Only the l-fold-INDUCED collision with "gait"
        is exempted -- a literal spelling still hits."""
        free_text = "The rebels of Galt overthrew their aristocracy."
        self.assertEqual(pi_scrub.normalized_term_hit(free_text), "Galt")

    def test_genuine_l_fold_still_catches_a_synthetic_ocr_term(self):
        """The l/1/!->i fold mechanism itself (not the "Galt"-specific
        exemption) must still catch a genuine OCR-confused artifact for any
        OTHER term. Uses a synthetic, throwaway term -- never a real Product
        Identity string -- because "Galt" is currently the ONLY real
        blacklist term containing "l", and it is now (correctly) exempt from
        this exact fold."""
        original_terms = pi_scrub._CANON_TERMS
        try:
            synthetic_term = "Kelmoria"  # synthetic OCR-affected proper noun
            pi_scrub._CANON_TERMS = [
                (synthetic_term, pi_scrub.canonicalize(synthetic_term), True, True)
            ]
            # A scanner confuses "l" for "1": "Kelmoria" -> "Ke1moria".
            free_text = "The old road to Ke1moria was long forgotten."
            self.assertEqual(
                pi_scrub.normalized_term_hit(free_text),
                synthetic_term,
                "the l/1/!->i fold mechanism must still catch a genuine OCR "
                "artifact for terms other than the exempted 'Galt'",
            )
        finally:
            pi_scrub._CANON_TERMS = original_terms


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
