#!/usr/bin/env python3
"""SD-32 T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21) --
regression test for the fourth "one path screens, another doesn't" defect
in this bundle.

`regen_row17_pi_over_redaction.py`'s own module docstring records a
near-miss: its FIRST draft re-derived a record's `data.raw_tokens` through
`scrub_name_pi_tokens` alone, omitting the declared-PI DESC-blanking and
blacklist-scan steps `ingest_generic_kind.py::remediate` performs first --
so a record whose `DESC` prose does not happen to literally contain its own
PI name/key (an ordinary narrative sentence, not a restatement of the key)
would ship its FULL, un-redacted description text in `data.raw_tokens` even
though `DESCISPI:YES` declares it PI. That cycle caught it (via
`declared_pi_shipping_audit`), reverted, and rewrote its own script to
mirror the canonical `remediate()` pipeline -- but named the SAME gap as
still present, unfixed, in its two sibling regen drivers:
`regen_all_renamed_pi_scrub.py` and `regen_generic_kind_pi_scrub.py`. Both
are confirmed here to have carried the identical gap, and both are now
fixed by importing and calling the ONE canonical pipeline function
(`regen_row17_pi_over_redaction.redact_tokens`) instead of re-implementing
a second and third divergent copy.

This test module proves three things:

1. `redact_tokens` itself blanks a declared-PI `DESC` whose prose does NOT
   happen to match the identity/blacklist needles -- the exact shape the
   old two-step (`row_tokens` + `scrub_name_pi_tokens` alone) pipeline
   missed. Mutation-proved: reproducing the OLD two-step shape directly
   confirms it really does leak (RED for the old shape), and the new
   pipeline does not (GREEN).
2. Both sibling regen scripts import `redact_tokens` from
   `regen_row17_pi_over_redaction` -- the SAME function object, not a
   byte-identical copy that can drift again -- so a future regression back
   to a local duplicate is caught at import time, not by re-auditing output.
"""
from __future__ import annotations

import os
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import pi_scrub as PS  # noqa: E402
from ingest_ability import row_tokens  # noqa: E402
import regen_row17_pi_over_redaction as R17  # noqa: E402
import regen_all_renamed_pi_scrub as RALL  # noqa: E402
import regen_generic_kind_pi_scrub as RGEN  # noqa: E402

# A synthetic PCGen-shaped raw line: DESCISPI:YES, but the DESC prose is an
# ORDINARY narrative sentence that does not restate the record's own
# name/key and contains no blacklisted term -- the exact shape the old
# two-step pipeline let through un-redacted.
_RAW_LINE = (
    "Some Feature\tKEY:Concept ~ Some Feature\tCATEGORY:Special Ability\t"
    "TYPE:SpecialQuality\tDESCISPI:YES\t"
    "DESC:This ability lets the bearer channel raw force through their weapon, "
    "striking with unusual precision whenever the moment calls for it.\t"
    "BONUS:VAR|Foo|1"
)
_ORIG_NAME = "Some Feature"
_ORIG_KEY = "Concept ~ Some Feature"


class RedactTokensBlanksNonMatchingDescTest(unittest.TestCase):
    def test_declared_pi_desc_with_no_needle_match_is_blanked(self):
        tokens, stored_description, _extra = R17.redact_tokens(_RAW_LINE, _ORIG_NAME, _ORIG_KEY)
        self.assertEqual(stored_description, PS.REDACTED_PI_MARKER)
        desc_tokens = [t for t in tokens if t["key"] == "DESC"]
        self.assertTrue(desc_tokens, "expected a DESC token to survive in raw_tokens")
        for t in desc_tokens:
            self.assertEqual(
                t["value"],
                PS.REDACTED_PI_MARKER,
                "a DESCISPI:YES token whose prose does not match any identity/"
                "blacklist needle must still be blanked -- this is exactly the "
                "gap the old two-step pipeline missed",
            )
        # The mechanical BONUS survives untouched -- this fix must never
        # over-redact real formula content while closing the DESC gap.
        bonus_tokens = [t for t in tokens if t["key"] == "BONUS"]
        self.assertEqual(bonus_tokens[0]["value"], "VAR|Foo|1")

    def test_mutation_proof_the_old_two_step_pipeline_really_did_leak(self):
        """RED for the OLD shape: `row_tokens` + `scrub_name_pi_tokens`
        alone (no DESC pre-blanking step) leaves the declared-PI DESC
        prose fully un-redacted in raw_tokens -- proving `redact_tokens`'s
        extra pipeline steps are load-bearing, not decorative."""
        old_shape_tokens = row_tokens(_RAW_LINE)
        scrubbed, _any_redacted = PS.scrub_name_pi_tokens(old_shape_tokens, _ORIG_NAME, _ORIG_KEY)
        desc_tokens = [t for t in scrubbed if t["key"] == "DESC"]
        self.assertTrue(desc_tokens)
        self.assertNotEqual(
            desc_tokens[0]["value"],
            PS.REDACTED_PI_MARKER,
            "this reproduction is supposed to demonstrate the OLD leak -- if "
            "this assertion fails, scrub_name_pi_tokens alone has started "
            "blanking DESC and this test needs updating, not deleting",
        )
        self.assertIn("channel raw force", desc_tokens[0]["value"])


class SiblingRegenScriptsImportTheSharedPipelineTest(unittest.TestCase):
    """Import-identity check, not output re-derivation: both sibling regen
    drivers must call the SAME `redact_tokens` function object
    `regen_row17_pi_over_redaction.py` defines and mutation-proved, never a
    re-implemented copy that can silently drift again (`decisions.md §17`'s
    duplication-drift lesson)."""

    def test_regen_all_renamed_pi_scrub_uses_the_shared_redact_tokens(self):
        self.assertIs(RALL.redact_tokens, R17.redact_tokens)

    def test_regen_generic_kind_pi_scrub_uses_the_shared_redact_tokens(self):
        self.assertIs(RGEN.redact_tokens, R17.redact_tokens)


if __name__ == "__main__":
    unittest.main()
