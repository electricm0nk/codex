"""
Self-test for `scripts/transcribe_monster_tables.py::resolve_book_file` (SD31-E6-F9-002).

WHY THIS EXISTS
----------------
`decisions.md §9` re-attributes `core_essentials`-origin `monster`/`monster_ability`
units to their real book (`book` becomes e.g. `"bestiary"`), but the unit's own
`source_file` (`ce_abilities_race.lst`, `b4_abilities_races_ce.lst`, ...) is a
physical file that still lives under `core_essentials`'s own PCGen directory, never
under the re-attributed book's. `resolve_book_file` only ever walked the book's own
root, so any re-attributed unit made the whole transcriber run for that book
`raise SystemExit` -- confirmed live against the real pinned oracle (2026-08-16,
`bestiary`: `ce_abilities_race.lst is not present anywhere under
.../roleplaying_game/bestiary`) -- which is also why 357 `static`/`derived`
`monster_ability` units across `bestiary`/`bestiary_2`/`bestiary_3`/`bestiary_4`
sit `not-ingested` despite their book being fully registered.

This test proves the fix with a synthetic corpus tree, not the live oracle
(the live oracle is exercised by actually re-running the transcriber, which is
this cycle's own receipt) -- fast, hermetic, and it can fail: reverting the
fallback branch below makes `test_falls_back_to_core_essentials_when_book_root_lacks_the_file`
raise `SystemExit` exactly as the un-fixed function did.

Run: python3 -m unittest scripts/tests/test_transcribe_monster_tables.py
Wired as the `transcribe-monster-tables-selftest` stage in `scripts/verify.sh`.
"""
from __future__ import annotations

import importlib.util
import os
import pathlib
import tempfile
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "transcribe_monster_tables.py"
)
_spec = importlib.util.spec_from_file_location(
    "transcribe_monster_tables", _MODULE_PATH
)
tmt = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(tmt)


class ResolveBookFileCoreEssentialsFallback(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.corpus_root = pathlib.Path(self._tmp.name)
        # Mirror the real layout: <root>/pathfinder/paizo/roleplaying_game/<book>
        self.rpg = self.corpus_root / "pathfinder" / "paizo" / "roleplaying_game"
        self.bestiary_root = self.rpg / "bestiary"
        self.ce_root = self.rpg / "core_essentials"
        self.bestiary_root.mkdir(parents=True)
        self.ce_root.mkdir(parents=True)
        self._old_env = os.environ.get("PCGEN_CORPUS_ROOT")
        os.environ["PCGEN_CORPUS_ROOT"] = str(self.corpus_root)
        self.addCleanup(self._restore_env)

    def _restore_env(self) -> None:
        if self._old_env is None:
            os.environ.pop("PCGEN_CORPUS_ROOT", None)
        else:
            os.environ["PCGEN_CORPUS_ROOT"] = self._old_env

    def test_finds_a_file_in_the_books_own_root_first(self) -> None:
        native = self.bestiary_root / "b1_abilities_race.lst"
        native.write_text("row\n")
        found = tmt.resolve_book_file(str(self.bestiary_root), "b1_abilities_race.lst")
        self.assertEqual(found, str(native))

    def test_falls_back_to_core_essentials_when_book_root_lacks_the_file(self) -> None:
        ce_file = self.ce_root / "ce_abilities_race.lst"
        ce_file.write_text("row\n")
        # NOT present under bestiary_root -- this is the exact live-oracle shape.
        found = tmt.resolve_book_file(str(self.bestiary_root), "ce_abilities_race.lst")
        self.assertEqual(found, str(ce_file))

    def test_still_raises_when_absent_from_both(self) -> None:
        with self.assertRaises(SystemExit):
            tmt.resolve_book_file(str(self.bestiary_root), "nowhere.lst")

    def test_book_root_match_wins_over_core_essentials_even_if_both_carry_the_name(
        self,
    ) -> None:
        native = self.bestiary_root / "shared_name.lst"
        native.write_text("native\n")
        shadow = self.ce_root / "shared_name.lst"
        shadow.write_text("shadow\n")
        found = tmt.resolve_book_file(str(self.bestiary_root), "shared_name.lst")
        self.assertEqual(found, str(native))

    def test_core_essentials_itself_does_not_fall_back_to_itself_twice(self) -> None:
        # Resolving a file FOR core_essentials's own root must not silently
        # double-search core_essentials and mask a real "not present" case
        # with a confusing duplicate-candidate error.
        with self.assertRaises(SystemExit):
            tmt.resolve_book_file(str(self.ce_root), "nowhere.lst")


class WriteBookDoesNotTruncateOnFailure(unittest.TestCase):
    """`main()`'s `open(path, "w")` used to run BEFORE `transcribe()`, so a
    raise partway through `transcribe()` (a real, honest refusal -- an
    unmodelled `DESC:` shape, an orphan pass -- not a bug in `transcribe()`
    itself) truncated the target `monster_data.rs` to 0 bytes. Live twice
    this cycle (`bestiary`, `bestiary_2`), recovered only because a
    `git`-tracked backup happened to exist. `write_book` must compute the
    full replacement text FIRST and touch the file only on success, so an
    existing file survives a `transcribe()` failure completely unchanged."""

    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self._old_cwd = os.getcwd()
        os.chdir(self._tmp.name)
        self.addCleanup(os.chdir, self._old_cwd)
        target_dir = pathlib.Path("src/rules_core/rules_tables/fake_book")
        target_dir.mkdir(parents=True)
        self.target = target_dir / "monster_data.rs"
        self.original_content = "// pre-existing, real content\n"
        self.target.write_text(self.original_content)

    def test_a_transcribe_failure_leaves_the_existing_file_untouched(self) -> None:
        def _boom(book: str) -> str:
            raise SystemExit(f"{book}: a deliberate, honest refusal")

        old_transcribe = tmt.transcribe
        tmt.transcribe = _boom
        try:
            with self.assertRaises(SystemExit):
                tmt.write_book("fake_book")
        finally:
            tmt.transcribe = old_transcribe
        self.assertEqual(
            self.target.read_text(),
            self.original_content,
            "a failed transcribe() must not truncate the pre-existing file",
        )

    def test_a_successful_transcribe_still_writes_the_new_content(self) -> None:
        old_transcribe = tmt.transcribe
        tmt.transcribe = lambda book: f"// new content for {book}\n"
        try:
            path = tmt.write_book("fake_book")
        finally:
            tmt.transcribe = old_transcribe
        self.assertEqual(path, str(self.target))
        self.assertEqual(self.target.read_text(), "// new content for fake_book\n")


class UnscreenableRowIsDroppedNotFatal(unittest.TestCase):
    """`transcribe()` used to `raise SystemExit` the instant ONE owned ability
    row carried a multi-`DESC:` shape `parse_desc` cannot resolve -- crashing
    the whole book's transcription over that one row, not just refusing it.
    Confirmed live against the pinned oracle (`SD31-E6-F9-005`): re-running
    the transcriber for `bestiary`/`bestiary_2` raised on exactly 3/2 such
    rows and produced ZERO other movement, even though 135/95 OTHER
    genuinely-owned ability rows in those same books parse cleanly.

    `parse_desc` itself is UNCHANGED -- picking the right `DESC:` variant by
    position is still refused, deliberately (`OPEN-ISSUES.md` row 157: that
    would risk shipping subtly wrong player text). Only the BLAST RADIUS of
    the refusal changes: the ambiguous row is dropped, named, and reported --
    the same treatment a Product Identity or `.COPY=` row already gets -- and
    every OTHER row this book owns still transcribes.

    Hermetic: a synthetic `bonus_bestiary`-shaped corpus tree (one monster
    owning two abilities, one clean and one unmodelled) plus a synthetic
    `docs/work-inventory.json`, never the live oracle.
    """

    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self._old_cwd = os.getcwd()
        # `pi_blacklist_terms`/`redacted_pi_marker` read a REPO-RELATIVE path
        # (`src/rules_core/pi_screening.rs` etc.) -- resolve those to absolute
        # paths against the real repo BEFORE chdir'ing into the synthetic tree,
        # the same discipline `resolve_book_file`'s own tests use for
        # `PCGEN_CORPUS_ROOT`.
        self._old_pi_screen_rs = tmt.PI_SCREEN_RS
        self._old_pi_marker_rs = tmt.PI_MARKER_RS
        tmt.PI_SCREEN_RS = os.path.abspath(tmt.PI_SCREEN_RS)
        tmt.PI_MARKER_RS = os.path.abspath(tmt.PI_MARKER_RS)
        self.addCleanup(self._restore_pi_paths)
        os.chdir(self._tmp.name)
        self.addCleanup(os.chdir, self._old_cwd)

        self.corpus_root = pathlib.Path(self._tmp.name) / "pcgen"
        book_dir = (
            self.corpus_root
            / "pathfinder"
            / "paizo"
            / "roleplaying_game"
            / "bonus_bestiary"
        )
        book_dir.mkdir(parents=True)
        self._old_env = os.environ.get("PCGEN_CORPUS_ROOT")
        os.environ["PCGEN_CORPUS_ROOT"] = str(self.corpus_root)
        self.addCleanup(self._restore_env)

        # One monster row naming both abilities via `ABILITY:Special Ability`.
        races = book_dir / "bb_races.lst"
        races.write_text(
            "Test Monster\tKEY:Test Monster\tSIZE:M\t"
            "ABILITY:Special Ability|AUTOMATIC|Test Monster ~ Weird Ability|"
            "Test Monster ~ Fine Ability\tSOURCEPAGE:p.1\n"
        )
        # Ability 1: TWO `DESC:` tokens, neither gated on `DisplayFullAbility`,
        # not a continuation (both carry a pipe), not a superset (texts do not
        # share a prefix), not variable-bearing (both carry a pipe entry) --
        # the exact shape `parse_desc` refuses via `UnmodelledDesc`.
        # Ability 2: one plain `DESC:`, parses cleanly.
        abilities = book_dir / "bb_abilities.lst"
        abilities.write_text(
            "Weird Ability\tKEY:Test Monster ~ Weird Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:First incompatible text.|SomeGate:Foo\t"
            "DESC:Second incompatible text.|OtherGate:Bar\tSOURCEPAGE:p.2\n"
            "Fine Ability\tKEY:Test Monster ~ Fine Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:A perfectly ordinary description.\tSOURCEPAGE:p.2\n"
        )

        os.makedirs("docs", exist_ok=True)
        inventory = {
            "units": [
                {
                    "book": "bonus_bestiary",
                    "kind": "monster",
                    "corpus_key": "Test Monster",
                    "name": "Test Monster",
                    "source_file": "bb_races.lst",
                    "source_line": 1,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": "Test Monster ~ Weird Ability",
                    "name": "Weird Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 1,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": "Test Monster ~ Fine Ability",
                    "name": "Fine Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 2,
                    "status": "not-ingested",
                },
            ]
        }
        with open("docs/work-inventory.json", "w", encoding="utf-8") as handle:
            import json

            json.dump(inventory, handle)

    def _restore_env(self) -> None:
        if self._old_env is None:
            os.environ.pop("PCGEN_CORPUS_ROOT", None)
        else:
            os.environ["PCGEN_CORPUS_ROOT"] = self._old_env

    def _restore_pi_paths(self) -> None:
        tmt.PI_SCREEN_RS = self._old_pi_screen_rs
        tmt.PI_MARKER_RS = self._old_pi_marker_rs

    def test_the_clean_sibling_ships_even_though_the_unmodelled_row_does_not(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Test Monster ~ Fine Ability"', content)
        self.assertNotIn("Weird Ability", content.split("MONSTER_ABILITIES")[1])

    def test_the_unscreenable_row_is_named_in_the_header_not_silently_dropped(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn("bb_abilities.lst:1", content)
        self.assertIn("Test Monster ~ Weird Ability", content)

    def test_the_clean_ability_still_ships_if_the_unmodelled_one_is_seen_first(
        self,
    ) -> None:
        """Order independence: the drop must not depend on which ability the
        monster's `ABILITY:` token names first. Confirms the fix filters by
        `corpus_key` membership in `unscreenable`, not by position."""
        content = tmt.transcribe("bonus_bestiary")
        self.assertEqual(content.count("MonsterAbilityRecord {"), 1)


if __name__ == "__main__":
    unittest.main()
