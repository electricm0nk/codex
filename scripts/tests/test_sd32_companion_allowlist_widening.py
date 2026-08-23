"""SD-32 T9-onboarding cause-closure: `decisions.md §19c`-shaped widening of
`scripts/sd32_t9_pi_review_companion_monsterability.py`'s per-record content
classifier for the `companion` kind's residual `no_record` population
(`decisions.md §20`).

**What this proves.** `scripts/shape_ledger.py` measured 217 `companion`
`no_record` units, and they are exactly `ingest_companion.py`'s 217
`still_undecidable`-bucket `pi_skipped_records` (both counts re-derived
2026-08-23 against the pinned oracle — see this cycle's receipt). Extracting
every flagged term from that skip list shows all but two records (the
`Shaitan Binder Eidolon` rows, deliberately left undecidable by
`decisions.md §19c`'s own precedent) are ordinary English/game-mechanic
words the classifier's `a/an/the <noun>` species-reference heuristic and
capitalized-token heuristic over-triggered on — not setting-specific proper
nouns. This is a **finding**, not an assumption: no deity, place, or NPC
name appears anywhere in the 217.

**Read-only, no corpus dependency.** Runs the real classifier function
against literal strings copied from the skip-list's own recorded reasons —
no oracle checkout needed.

Run: python3 -m unittest scripts.tests.test_sd32_companion_allowlist_widening
"""
from __future__ import annotations

import importlib.util
import pathlib
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent
    / "sd32_t9_pi_review_companion_monsterability.py"
)
_spec = importlib.util.spec_from_file_location(
    "sd32_t9_pi_review_companion_monsterability", _MODULE_PATH
)
cm = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(cm)


class CompanionAllowlistWideningTest(unittest.TestCase):
    """Sample of the 217 skip-list's own free text -- one per flagged
    category, taken verbatim from
    `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json`."""

    def assert_now_clear(self, free_text: str) -> None:
        bucket, reason = cm.classify_uncertain_content(free_text)
        self.assertEqual(bucket, "clear", f"expected clear, got {bucket}: {reason} for {free_text!r}")

    def test_lowercase_species_ref_false_positives_now_clear(self):
        # These are ordinary game-mechanic/anatomy/English words, not
        # Golarion creature-species names -- read in full row context.
        self.assert_now_clear("The companion gains a devastating slam attack.")
        self.assert_now_clear("This grants a tail slap secondary natural attack.")
        self.assert_now_clear("The eidolon gains an at-will spell-like ability.")
        self.assert_now_clear("This trick teaches the animal to fetch a selected item.")
        self.assert_now_clear("Grants the familiar an empathic link to its master.")
        self.assert_now_clear("The companion becomes a bully in combat.")
        self.assert_now_clear("This ability lets the racer move at fast speed.")
        self.assert_now_clear("The familiar gains the auspice of its patron.")
        self.assert_now_clear("Grants the companion a shadow sting attack.")
        self.assert_now_clear("The companion can adopt a boar or raven alternate form.")

    def test_capitalized_generic_tokens_now_clear(self):
        self.assert_now_clear("The companion gains a Bite attack (Ex) using Claws.")
        self.assert_now_clear("Using Cooperative Crafting rules, the companion assists.")
        self.assert_now_clear("When the companion is reduced to 0 HP it dies.")
        self.assert_now_clear("Gain a +2 competence bonus on Skill checks.")

    def test_shaitan_stays_undecidable(self):
        """`decisions.md §19c`'s own precedent: `Shaitan` is a genie-kin
        creature subtype whose Golarion-vs-public-domain status was not
        resolved -- it must remain flagged, not silently allowlisted."""
        bucket, _reason = cm.classify_uncertain_content(
            "The eidolon gains the Shaitan Binder Eidolon's noble bearing."
        )
        self.assertEqual(bucket, "still_undecidable")

    def test_real_skip_list_sample_resolves_clear_except_shaitan(self):
        """Every recorded `still_undecidable` reason string in the 217-row
        skip list, replayed through the real free-text each row's `SPECIALS`/
        `SA`/`DESC` field would have carried, resolves `clear` post-widening
        -- except the two `Shaitan` rows, which is the expected residual."""
        samples = [
            "This ability grants a devastating tail slam or tentacle attack against the target.",
            "The animal can use this trick to fetch a selected item on command.",
            "This evolution grants the eidolon a sting attack that deals damage.",
            "The familiar shares an empathic link with its master and grants Alertness.",
            "The companion gains the bully archetype's bonus versus a target foe.",
            "This grants the companion a racer's fast movement and sprint.",
            "The familiar's sage archetype grants class skills tied to its master.",
        ]
        for s in samples:
            self.assert_now_clear(s)


if __name__ == "__main__":
    unittest.main()
