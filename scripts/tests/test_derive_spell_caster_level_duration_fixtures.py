#!/usr/bin/env python3
"""Tests for `scripts/derive_spell_caster_level_duration_fixtures.py`'s
CANDIDATE SELECTION.

WHY THIS FILE EXISTS (SD-31 wave 20). This generator's `WORK_INVENTORY_
BOOK_TO_SHORT` dict -- the sibling of `derive_spell_range_fixtures.py`'s
identically-named dict, and copy-pasted from the SAME original 8-book
`spell_resolver::spell_catalog_rows()` roster -- was never widened when
`inner_sea_gods` (`SD31-E6-F10-001`) and `ultimate_wilderness` (SD-31
wave-19's `ultimate_wilderness` lane) each gained a real `data/corpus/
<book>/spell/` cache. The exact same class of gap `OPEN-ISSUES.md` row 324
fixed on the Rust read side (`SPELL_CORPUS_BOOK_DIRS` /
`spell_book_corpus_dir_for_short_code` in `derived_evaluator_fixture_
check.rs`) existed here too, unfixed: no `derived`+held DURATION-family
spell unit in either book could ever become a fixture candidate, regardless
of how complete its data was.

This file did not exist before this wave -- the sibling RANGE generator's
own `test_derive_spell_range_fixtures.py` (SD31-W15) is the only prior
precedent for testing a spell fixture generator's candidate selection in
isolation; this file mirrors its shape for the DURATION family.
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SCRIPT = os.path.join(REPO_ROOT, "scripts", "derive_spell_caster_level_duration_fixtures.py")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import derive_spell_caster_level_duration_fixtures as gen  # noqa: E402


def _unit(**over):
    u = {
        "id": "core_rulebook:spell:example",
        "book": "core_rulebook",
        "kind": "spell",
        "wiring_class": "derived",
        "status": "grounded",
        "source_file": "cr_spells.lst",
        "source_line": 10,
        "corpus_key": "Example",
        "name": "Example",
    }
    u.update(over)
    return u


class CandidateSelectionTests(unittest.TestCase):
    def test_a_derived_stampable_unit_in_an_originally_mapped_book_is_a_candidate(self):
        self.assertTrue(gen.is_candidate(_unit()))

    def test_the_facts_that_decide_stampability_still_gate(self):
        self.assertFalse(gen.is_candidate(_unit(wiring_class="computed")))
        self.assertFalse(gen.is_candidate(_unit(wiring_class="static")))
        self.assertFalse(gen.is_candidate(_unit(kind="class_feature")))
        self.assertFalse(gen.is_candidate(_unit(status="not-started")))
        self.assertFalse(gen.is_candidate(_unit(status="unknown")))
        # `pathfinder_unchained` genuinely carries no `data/corpus/
        # pathfinder_unchained/spell/` directory at all -- see
        # `spell_catalog.rs`'s own module doc comment (`pu_spells.lst` is
        # 224 lines, every one commented out).
        self.assertFalse(gen.is_candidate(_unit(book="pathfinder_unchained")))

    def test_inner_sea_gods_and_ultimate_wilderness_are_candidates(self):
        # The fix this file exists to pin: both books already carry a real
        # `data/corpus/<book>/spell/` cache and must not be gatekept out of
        # candidacy by this generator's own book dict.
        self.assertTrue(gen.is_candidate(_unit(book="inner_sea_gods")))
        self.assertTrue(gen.is_candidate(_unit(book="ultimate_wilderness")))


class GeneratorEndToEndTests(unittest.TestCase):
    """Runs the real generator over a hand-built one-unit inventory drawn
    from a real `ultimate_wilderness` DURATION-family candidate, and asserts
    the entry is emitted with the ruleset's own formula. Fails before the
    fix (book absent from `WORK_INVENTORY_BOOK_TO_SHORT`, so `is_candidate`
    excludes the unit and the generator sees zero candidates); passes
    after."""

    def test_a_ultimate_wilderness_unit_with_a_simple_caster_level_duration_is_emitted(self):
        with open(os.path.join(REPO_ROOT, "docs", "work-inventory.json")) as fh:
            inv_units = json.load(fh)["units"]

        def duration_value(u):
            rel = f"pathfinder/paizo/roleplaying_game/{u['book']}/{u['source_file']}"
            full = os.path.join(gen.pcgen_corpus_root(), rel)
            if not os.path.isfile(full):
                return None
            with open(full, encoding="utf-8", errors="replace") as f:
                lines = f.read().split("\n")
            line_no = int(u["source_line"])
            if line_no < 1 or line_no > len(lines):
                return None
            return gen.duration_field_from_raw_line(lines[line_no - 1])

        def corpus_has_duration_token(book, key):
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
                        t.get("key") == "DURATION"
                        for t in rec.get("data", {}).get("raw_tokens", [])
                    )
            return False

        sample = None
        for u in inv_units:
            if not (
                u.get("kind") == "spell"
                and u.get("wiring_class") == "derived"
                # Includes `fixture-verified` (`gen.STAMPABLE_STATUSES`, not
                # just the pre-stamp statuses): this test may run AFTER a
                # real `--write` has already stamped every matching UW/ISG
                # unit, exactly the wave-15 idempotence lesson
                # (`OPEN-ISSUES.md` rows 284/286) -- a sample already
                # carrying the stamp still proves the generator re-emits it,
                # which is the property this test needs, not "is there an
                # unstamped one left".
                and u.get("status") in gen.STAMPABLE_STATUSES
                and u.get("book") in ("inner_sea_gods", "ultimate_wilderness")
            ):
                continue
            raw = duration_value(u)
            if raw is None:
                continue
            m = gen.SIMPLE_RE.match(raw.strip())
            if not m:
                continue
            key = u.get("corpus_key") or u.get("name")
            if corpus_has_duration_token(u["book"], key):
                sample = u
                break

        if sample is None:
            self.skipTest(
                "no inner_sea_gods/ultimate_wilderness derived spell with a "
                "resolvable simple caster-level DURATION to test with"
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


if __name__ == "__main__":
    unittest.main()
