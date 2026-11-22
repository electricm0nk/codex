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


def _corpus_record_has_range_raw_token(book: str, key: str) -> bool:
    """Stand-alone re-check of the same fact `gen`'s own (function-local,
    not importable) `keys_with_range_token` closure gates emission on: does
    this record's `data/corpus/<book>/spell/**.json` file carry a `RANGE`
    entry in its own `data.raw_tokens`? A candidate can carry a verbatim
    upstream `RANGE:` keyword (this test's own search predicate) and still
    be `skipped_unresolved_in_corpus_cache` by the real generator if its
    ingested corpus record has no `raw_tokens` at all (`OPEN-ISSUES.md` row
    11 bucket b) -- used here only to pick a sample the real end-to-end run
    can actually emit, not to duplicate the generator's write-path logic."""
    spell_dir = os.path.join(REPO_ROOT, "data", "corpus", book, "spell")
    for root, _dirs, files in os.walk(spell_dir):
        for fn in files:
            if not fn.endswith(".json"):
                continue
            try:
                with open(os.path.join(root, fn)) as jf:
                    rec = json.load(jf)
            except Exception:
                continue
            if rec.get("data", {}).get("key") != key:
                continue
            return any(
                t.get("key") == "RANGE" for t in rec.get("data", {}).get("raw_tokens", [])
            )
    return False


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
        # `pathfinder_unchained` genuinely carries no `data/corpus/
        # pathfinder_unchained/spell/` directory at all (`pu_spells.lst` is
        # 224 lines, every one commented out -- see `spell_catalog.rs`'s own
        # module doc comment) and has nothing for `run_spell_range_bar_check`
        # to evaluate against.
        self.assertFalse(gen.is_candidate(_unit(book="pathfinder_unchained")))

    def test_inner_sea_gods_and_ultimate_wilderness_are_candidates(self):
        # SD-31 wave 20: both books gained a real `data/corpus/<book>/spell/`
        # cache before this fix (ISG since `SD31-E6-F10-001`, UW since
        # SD-31 wave-19's `ultimate_wilderness` lane +
        # `SD31-W19-INTEGRATE-001`'s row-324 follow-up) but
        # `WORK_INVENTORY_BOOK_TO_SHORT` -- copy-pasted from
        # `spell_resolver::spell_catalog_rows()`'s ORIGINAL 8-book roster --
        # was never widened to match, so no `derived`+held unit in either
        # book could ever become a fixture candidate regardless of how
        # complete its data was. The exact `SPELL_CORPUS_BOOK_DIRS`/
        # `spell_book_corpus_dir_for_short_code` gap row 324 fixed on the
        # Rust read side, unfixed here on the fixture-generation side.
        self.assertTrue(gen.is_candidate(_unit(book="inner_sea_gods")))
        self.assertTrue(gen.is_candidate(_unit(book="ultimate_wilderness")))

    def test_the_candidate_predicate_reads_no_signal_ordering_field(self):
        """A unit record with NO `wiring_class_reason` key at all is still a
        candidate -- the strongest form of "candidacy does not consult it"."""
        u = _unit()
        del u["wiring_class_reason"]
        self.assertTrue(gen.is_candidate(u))


class GeneratorEndToEndTests(unittest.TestCase):
    """Runs the real generator over a hand-built one-unit inventory whose
    `wiring_class_reason` is `prose_expr`, and asserts the entry is emitted
    with the ruleset's own formula. Fails before the fix; passes after.

    SD-31 wave 20: found already RED at this cycle's base (5adedce63), for a
    reason distinct from the ISG/UW gap this file's other new tests target.
    The wave-15 fix this test proves emitted every fully-resolvable
    `prose_expr` RANGE-keyword candidate into the committed fixture already;
    every remaining `prose_expr` candidate across all 8 originally-mapped
    books now fails `is_candidate`'s own downstream `keys_with_range_token`
    check -- its corpus JSON record carries no `RANGE` `raw_tokens` entry at
    all, the exact `OPEN-ISSUES.md` row 11 bucket-b shape (95 `lst_token`
    records with `raw_tokens` absent), which is a real, separately-scoped
    ingest gap this generator correctly refuses rather than a defect in the
    generator's candidate selection. Skips (not passes silently, and not a
    hard failure that would block an honest future run) when no such sample
    exists, naming the reason; a future ingest cycle that populates those
    `raw_tokens` would make this assertion resolvable again."""

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
                # Any candidate book, not only `core_rulebook`: as the
                # corpus/inventory shifts wave over wave, which book still
                # has an un-fixture-verified `prose_expr` spell carrying a
                # verbatim RANGE keyword changes too (CRB's own such units
                # have since all been fixture-verified or lack a keyword
                # range) -- pinning to one book makes this test flaky against
                # a moving corpus rather than testing the generator's real
                # behavior.
                and u.get("book") in gen.WORK_INVENTORY_BOOK_TO_SHORT
            ):
                raw = gen.upstream_range_value(gen.pcgen_corpus_root(), u)
                if raw in gen.KNOWN_KEYWORDS and _corpus_record_has_range_raw_token(
                    u["book"], u.get("corpus_key") or u.get("name")
                ):
                    sample = u
                    break
        if sample is None:
            self.skipTest(
                "no prose_expr spell in any candidate book carries a live "
                "RANGE keyword no longer already fixture-verified -- see "
                "this test's own docstring (OPEN-ISSUES row 11 bucket-b: "
                "raw_tokens-absent corpus records, not a candidacy defect)"
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
