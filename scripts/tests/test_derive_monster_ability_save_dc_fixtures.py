#!/usr/bin/env python3
"""Tests for `scripts/derive_monster_ability_save_dc_fixtures.py`'s
`find_owner_row()` -- the owner-monster-row resolver shared by BOTH the flat
(`monster_ability_entries`) and full-formula (`monster_ability_formula_entries`)
save-DC sub-seams.

WHY THIS FILE EXISTS (SD31 wave 17, `monster_ability` lane). `OPEN-ISSUES.md`
row 295 (wave 16) named a real, pre-existing gap: `find_owner_row()` matches
an owner monster row ONLY via an explicit `KEY:<name>` token. Three
formula-shape candidates' owner rows carry no `KEY:` token at all -- PCGen
lets a monster row's bare leading field stand in for its name when no `KEY:`
is stated (`inner_sea_bestiary/isb_races.lst`'s `Fungus Queen` row,
`inner_sea_world_guide/iswg_races.lst`'s `Aluum` and `Spine Dragon` rows all
begin `<Name>\tSTARTFEATS:...` with no `KEY:` field anywhere on the line) --
so those three formula-shape units fell into
`formula_orphan_no_owner_monster_row_in_this_book` even though a real,
unambiguous owner row is right there in the same book directory.

The bar these tests hold the resolver to: it may fall back to a bare
leading-field match ONLY when (a) no `KEY:`-token row matches the name in
that book directory at all, (b) the fallback row carries no `KEY:` token of
its OWN (so it can never shadow or out-rank a real `KEY:` match for a
DIFFERENT owner), and (c) the fallback row still carries a readable
`MONSTERCLASS:` token, the same requirement the `KEY:`-matched path already
enforces. It must NOT change resolution for any owner that already resolves
via `KEY:` -- the already-gated 92 flat-shape + 23 disagreement + 6
formula-shape units all depend on the current `KEY:`-match behavior and must
see byte-identical results.
"""

import os
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import derive_monster_ability_save_dc_fixtures as gen  # noqa: E402


def _write_lst(path, lines):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")


class FindOwnerRowKeyMatchUnaffectedTests(unittest.TestCase):
    """The pre-existing `KEY:`-token match path must be untouched."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.book_dir = os.path.join(self.tmp.name, "some_book")

    def tearDown(self):
        self.tmp.cleanup()

    def test_a_key_token_row_still_resolves_exactly_as_before(self):
        _write_lst(
            os.path.join(self.book_dir, "monsters.lst"),
            ["Boilborn\tKEY:Boilborn\tMONSTERCLASS:Ooze:2\tOUTPUTNAME:Boilborn"],
        )
        cache = {}
        hits = gen.find_owner_row(self.book_dir, "Boilborn", cache)
        self.assertEqual(len(hits), 1)
        path, line_no, fields = hits[0]
        self.assertEqual(line_no, 1)
        self.assertEqual(gen.token(fields, "KEY"), "Boilborn")

    def test_a_bare_name_row_does_not_shadow_a_real_key_match(self):
        """When BOTH a `KEY:`-tagged row and an unrelated bare-name row exist,
        the `KEY:` row wins -- the fallback never even runs."""
        _write_lst(
            os.path.join(self.book_dir, "monsters.lst"),
            [
                "Boilborn\tKEY:Boilborn\tMONSTERCLASS:Ooze:2",
                "Boilborn Decoy\tSTARTFEATS:1\tMONSTERCLASS:Ooze:9",
            ],
        )
        cache = {}
        hits = gen.find_owner_row(self.book_dir, "Boilborn", cache)
        self.assertEqual(len(hits), 1)
        self.assertEqual(gen.token(hits[0][2], "KEY"), "Boilborn")


class FindOwnerRowBareNameFallbackTests(unittest.TestCase):
    """The NEW behavior this cycle adds."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.book_dir = os.path.join(self.tmp.name, "inner_sea_bestiary")

    def tearDown(self):
        self.tmp.cleanup()

    def test_a_bare_leading_field_resolves_when_no_key_token_row_exists(self):
        """FAILS before the fix: `find_owner_row` returns no hits at all for
        `Fungus Queen` today because no row states `KEY:Fungus Queen`."""
        _write_lst(
            os.path.join(self.book_dir, "isb_races.lst"),
            ["Fungus Queen\tSTARTFEATS:2\tMONSTERCLASS:Plant:9\tOUTPUTNAME:Fungus Queen"],
        )
        cache = {}
        hits = gen.find_owner_row(self.book_dir, "Fungus Queen", cache)
        self.assertEqual(
            len(hits), 1, "a bare leading-field name should resolve as a fallback owner"
        )
        path, line_no, fields = hits[0]
        self.assertEqual(fields[0], "Fungus Queen")
        self.assertIsNone(gen.token(fields, "KEY"))
        self.assertEqual(gen.token(fields, "MONSTERCLASS"), "Plant:9")

    def test_the_fallback_still_requires_a_readable_monsterclass_token(self):
        _write_lst(
            os.path.join(self.book_dir, "isb_races.lst"),
            ["Fungus Queen\tSTARTFEATS:2\tOUTPUTNAME:Fungus Queen"],
        )
        cache = {}
        hits = gen.find_owner_row(self.book_dir, "Fungus Queen", cache)
        self.assertEqual(hits, [])

    def test_the_fallback_never_matches_a_differently_named_row(self):
        _write_lst(
            os.path.join(self.book_dir, "isb_races.lst"),
            ["Someone Else\tSTARTFEATS:1\tMONSTERCLASS:Fey:4"],
        )
        cache = {}
        hits = gen.find_owner_row(self.book_dir, "Fungus Queen", cache)
        self.assertEqual(hits, [])

    def test_two_of_the_three_named_wave16_orphans_resolve_against_the_pinned_oracle(self):
        """End-to-end against the real pinned corpus, not a synthetic tree.

        Skips (does not fail) when the pinned oracle checkout is unavailable.

        Only 2 of row 295's 3 named units are actually the bare-leading-field
        shape -- re-derived this cycle, correcting row 295's own text.
        `Fungus Queen` and `Aluum` both name a race row whose leading field is
        LITERALLY their own name. `Spine Dragon` does not: `iswg_races.lst`'s
        only matching race row is named `Dragon (Spine)` (word order
        reversed, `OUTPUTNAME:[NAME] Dragon` is what prints "Spine Dragon" to
        a player, but the row's own leading field/KEY never states that
        string) -- a genuine name-identity mismatch, not a missing-`KEY:`
        gap, and NOT safe to bridge with a reordering heuristic (that would
        be exactly the kind of guess this seam's doctrine forbids). `Spine
        Dragon` stays correctly unresolved.
        """
        # `SD31-W17-INTEGRATE-001` (adversarial review, wave 17): this test
        # used to require `PCGEN_CORPUS_ROOT` set explicitly and silently
        # SKIPPED otherwise, while the generator it exercises resolves the
        # oracle via `pcgen_data_root()`'s own `$HOME`-relative fallback --
        # so a default run never actually proved this test's claim. Fall
        # back the same way the generator does, and only skip when NEITHER
        # resolves to a real directory.
        root = os.environ.get("PCGEN_CORPUS_ROOT") or gen.pcgen_data_root()
        if not root or not os.path.isdir(root):
            self.skipTest("neither PCGEN_CORPUS_ROOT nor the $HOME-relative pcgen checkout is available in this environment")
        index = gen.index_lst(root)
        isb_dir = os.path.dirname(index["isb_races.lst"][0])
        iswg_dir = os.path.dirname(index["iswg_races.lst"][0])
        cache = {}
        resolves = [
            (isb_dir, "Fungus Queen"),
            (iswg_dir, "Aluum"),
        ]
        for book_dir, owner_key in resolves:
            with self.subTest(owner_key=owner_key):
                hits = gen.find_owner_row(book_dir, owner_key, cache)
                self.assertEqual(
                    len(hits), 1, f"{owner_key} should resolve to exactly one owner row"
                )
                self.assertIsNone(gen.token(hits[0][2], "KEY"))

        still_orphan_hits = gen.find_owner_row(iswg_dir, "Spine Dragon", cache)
        self.assertEqual(
            still_orphan_hits,
            [],
            "Spine Dragon is a name-identity mismatch (row named 'Dragon "
            "(Spine)'), not a missing-KEY: gap -- must stay unresolved",
        )


if __name__ == "__main__":
    unittest.main()
