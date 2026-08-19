#!/usr/bin/env python3
"""Tests for `scripts/derive_spell_range_fixtures.py`'s CANDIDATE SELECTION.

WHY THIS FILE EXISTS (SD31-W15, spell/class_feature seam lane). The generator
picked its candidates with `u["wiring_class_reason"] == "range_keyword"`. That
field is not a statement about the record's `RANGE:` token at all -- it is
`wiring_class::classify()`'s tie-break, which returns
`sigs.iter().filter(|s| s.starts_with("derived:")).min()`, i.e. the
LEXICOGRAPHICALLY SMALLEST `derived:` signal. `"derived:prose_expr"` sorts
before `"derived:range_keyword"` (`p` < `r`), so a unit that carries BOTH
signals can never report `range_keyword`, no matter how plainly its own
upstream row says `RANGE:Close`.

The consequence is measurable, not theoretical: 151 `derived`+held `spell`
units in the eight ingested books carry a verbatim `RANGE:Close`/`Medium`/
`Long` token upstream and were excluded from the fixture purely by that
alphabetical accident. The sibling generator for the DURATION family
(`derive_spell_caster_level_duration_fixtures.py`) never filtered on
`wiring_class_reason`, which is why the two families' coverage diverged.

The bar these tests hold the generator to: candidate selection may depend on
kind / wiring_class / status / book -- the facts that decide whether a
`fixture-verified` stamp is even applicable (`v06_work_inventory::
apply_done_rung_stamps`) -- and on the record's OWN `RANGE:` token, which the
generator reads from the pinned upstream bytes. It may NOT depend on which
`derived:` signal happened to sort first.
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SCRIPT = os.path.join(REPO_ROOT, "scripts", "derive_spell_range_fixtures.py")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import derive_spell_range_fixtures as gen  # noqa: E402


def _unit(**over):
    u = {
        "id": "core_rulebook:spell:example",
        "book": "core_rulebook",
        "kind": "spell",
        "wiring_class": "derived",
        "wiring_class_reason": "range_keyword",
        "status": "grounded",
        "source_file": "cr_spells.lst",
        "source_line": 10,
        "corpus_key": "Example",
        "name": "Example",
    }
    u.update(over)
    return u


class CandidateSelectionTests(unittest.TestCase):
    def test_a_range_keyword_reason_unit_is_a_candidate(self):
        self.assertTrue(gen.is_candidate(_unit()))

    def test_the_alphabetical_tie_break_does_not_decide_candidacy(self):
        """`prose_expr` is what `classify()` returns for ANY unit that also
        carries a prose scalar -- it says nothing about the `RANGE:` token,
        which this generator re-reads from the upstream bytes itself."""
        self.assertTrue(gen.is_candidate(_unit(wiring_class_reason="prose_expr")))
        self.assertTrue(
            gen.is_candidate(_unit(wiring_class_reason="prose_formula_segment"))
        )

    def test_the_facts_that_decide_stampability_still_gate(self):
        # `apply_done_rung_stamps` only ever stamps kind-agnostic
        # wiring_class=Derived rows, so a non-derived or wrong-kind row could
        # never be moved by this fixture and must not be entered into it.
        self.assertFalse(gen.is_candidate(_unit(wiring_class="computed")))
        self.assertFalse(gen.is_candidate(_unit(wiring_class="static")))
        self.assertFalse(gen.is_candidate(_unit(kind="class_feature")))
        # `not-started` / `unknown` carry no ingested magnitude to verify.
        self.assertFalse(gen.is_candidate(_unit(status="not-started")))
        self.assertFalse(gen.is_candidate(_unit(status="unknown")))
        # A book with no `data/corpus/<book>/spell/` ingest has nothing for
        # `run_spell_range_bar_check` to evaluate against.
        self.assertFalse(gen.is_candidate(_unit(book="inner_sea_gods")))

    def test_the_candidate_predicate_reads_no_signal_ordering_field(self):
        """A unit record with NO `wiring_class_reason` key at all is still a
        candidate -- the strongest form of "candidacy does not consult it"."""
        u = _unit()
        del u["wiring_class_reason"]
        self.assertTrue(gen.is_candidate(u))


class GeneratorEndToEndTests(unittest.TestCase):
    """Runs the real generator over a hand-built one-unit inventory whose
    `wiring_class_reason` is `prose_expr`, and asserts the entry is emitted
    with the ruleset's own formula. Fails before the fix; passes after."""

    def test_a_prose_expr_reason_unit_with_a_range_keyword_is_emitted(self):
        with open(os.path.join(REPO_ROOT, "docs", "work-inventory.json")) as fh:
            inv_units = json.load(fh)["units"]
        sample = None
        for u in inv_units:
            if (
                u.get("kind") == "spell"
                and u.get("wiring_class") == "derived"
                and u.get("wiring_class_reason") == "prose_expr"
                and u.get("status") in ("ingested-magnitude", "grounded")
                and u.get("book") == "core_rulebook"
            ):
                raw = gen.upstream_range_value(gen.pcgen_corpus_root(), u)
                if raw in gen.KNOWN_KEYWORDS:
                    sample = u
                    break
        self.assertIsNotNone(
            sample, "no core_rulebook prose_expr spell with a RANGE keyword to test with"
        )

        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as fh:
            json.dump({"units": [sample]}, fh)
            path = fh.name
        try:
            out = subprocess.run(
                [sys.executable, SCRIPT, "--work-inventory", path],
                capture_output=True,
                text=True,
                cwd=REPO_ROOT,
            )
            self.assertEqual(out.returncode, 0, out.stderr)
            entries = json.loads(out.stdout)
        finally:
            os.unlink(path)

        self.assertEqual(len(entries), 1, f"stderr: {out.stderr}")
        entry = entries[0]
        self.assertEqual(entry["unit_id"], sample["id"])
        formulas = gen.load_spellrange_formulas()
        keyword = entry["corpus_field"][len("RANGE:"):]
        base_ft, rate_ft, per_levels = formulas[keyword]
        self.assertEqual(entry["expected"]["base_ft"], base_ft)
        self.assertEqual(entry["expected"]["rate_ft"], rate_ft)
        self.assertEqual(entry["expected"]["per_levels"], per_levels)


if __name__ == "__main__":
    unittest.main()
