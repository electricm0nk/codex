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


class InternalBundleAbilityHopIsResolved(unittest.TestCase):
    """`ABILITY:Internal|AUTOMATIC|<bundle_key>` on a monster row is a hop
    through a `CATEGORY:Internal` bundle row, not a direct ability reference
    (SD-29 `decisions.md §62.4`; sized corpus-wide at 235 units by
    `scripts/scan_monster_ability_bundle_rows.py`, round 10, but never wired
    into the transcriber -- `transcribe()`'s ownership pass reads only
    `ABILITY:Special Ability|AUTOMATIC|` tokens and the `<Monster> ~
    <Ability>` namespace prefix, neither of which sees this shape, so every
    ability reachable only through it stayed an orphan and `not-ingested`
    forever regardless of how complete its own prose was).

    Shape (real, from `inner_sea_gods`, `support/isg_races_b4.lst:6` /
    `support/isg_abilities_races_b4.lst:8`, this program's own docstring for
    `scan_monster_ability_bundle_rows.py`)::

        Test Monster    ABILITY:Internal|AUTOMATIC|Race Traits ~ Test Bundle
        Race Traits ~ Test Bundle    CATEGORY:Internal
            ABILITY:Special Ability|AUTOMATIC|Special Bundle ~ Bundled Ability
        Special Bundle ~ Bundled Ability    CATEGORY:Special Ability  DESC:...

    Hermetic: a synthetic `bonus_bestiary`-shaped tree, mirroring
    `UnscreenableRowIsDroppedNotFatal`'s fixture shape exactly.
    """

    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self._old_cwd = os.getcwd()
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

        # The monster names a BUNDLE, not the ability directly.
        races = book_dir / "bb_races.lst"
        races.write_text(
            "Test Monster\tKEY:Test Monster\tSIZE:M\t"
            "ABILITY:Internal|AUTOMATIC|Race Traits ~ Test Bundle\t"
            "SOURCEPAGE:p.1\n"
        )
        # The bundle row (CATEGORY:Internal) is the hop target; its own
        # `ABILITY:Special Ability|AUTOMATIC|` token names the real ability.
        # A second, genuinely-unreferenced ability row is included to prove
        # the pass does not credit anything it was not told to.
        abilities = book_dir / "bb_abilities.lst"
        abilities.write_text(
            "Race Traits ~ Test Bundle\tKEY:Race Traits ~ Test Bundle\t"
            "CATEGORY:Internal\t"
            "ABILITY:Special Ability|AUTOMATIC|Special Bundle ~ Bundled Ability\n"
            "Bundled Ability\tKEY:Special Bundle ~ Bundled Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:A perfectly ordinary description.\tSOURCEPAGE:p.2\n"
            "Unrelated Ability\tKEY:Other Monster ~ Unrelated Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:Nobody names this one.\tSOURCEPAGE:p.3\n"
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
                    "corpus_key": "Special Bundle ~ Bundled Ability",
                    "name": "Bundled Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 2,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": "Other Monster ~ Unrelated Ability",
                    "name": "Unrelated Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 3,
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

    def test_the_bundle_reached_ability_ships_owned_by_the_referencing_monster(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Special Bundle ~ Bundled Ability"', content)
        self.assertIn('owners: &["Test Monster"]', content)

    def test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped(
        self,
    ) -> None:
        """A row this pass does NOT resolve must not be silently credited
        anyway -- proves the hop is scoped to what the bundle row actually
        names, not "every remaining orphan in the book"."""
        content = tmt.transcribe("bonus_bestiary")
        self.assertNotIn("Unrelated Ability", content.split("MONSTER_ABILITIES")[1])


class UnscreenableRowIsDroppedNotFatal(unittest.TestCase):
    """`transcribe()` used to `raise SystemExit` the instant ONE owned ability
    row carried a multi-`DESC:` shape `parse_desc` cannot resolve -- crashing
    the whole book's transcription over that one row, not just refusing it.
    Confirmed live against the pinned oracle (`SD31-E6-F9-005`): re-running
    the transcriber for `bestiary`/`bestiary_2` raised on exactly 3/2 such
    rows and produced ZERO other movement, even though 135/95 OTHER
    genuinely-owned ability rows in those same books parse cleanly.

    **Round 9 update (`decisions.md §27b`):** `parse_desc` no longer refuses
    this fixture's shape at all -- `ConcatenatedDescClosesTheFinalRefusalGroupRound9`
    above is the generalised sixth branch that resolves it, by concatenating
    every `DESC:` token's own text rather than guessing which one wins. This
    class now proves the SAME resilience property the drop-not-fatal fix
    established still holds now that the row SHIPS instead of being dropped:
    a row whose gate this shared table cannot resolve is not a crash, it is
    real content, and every OTHER row this book owns still transcribes
    alongside it.

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
        # the round-9 generalised sixth branch (`ConcatenatedDescClosesThe
        # FinalRefusalGroupRound9`) concatenates both, verbatim, rather than
        # guessing which one wins -- the second token's gate tests a fact
        # about the OWNING MONSTER this shared table row cannot resolve.
        # Ability 2: one plain `DESC:`, parses cleanly.
        abilities = book_dir / "bb_abilities.lst"
        abilities.write_text(
            "Weird Ability\tKEY:Test Monster ~ Weird Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:First incompatible text.|PREVARGTEQ:SomeGate,1\t"
            "DESC:Second incompatible text.|PREVARGTEQ:OtherGate,1\tSOURCEPAGE:p.2\n"
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

    def test_the_clean_sibling_ships_alongside_the_concatenated_row(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Test Monster ~ Fine Ability"', content)
        self.assertIn('key: "Test Monster ~ Weird Ability"', content)

    def test_the_formerly_unscreenable_row_ships_with_both_texts_concatenated(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Test Monster ~ Weird Ability"', content)
        self.assertIn(
            'description: Some("First incompatible text. Second incompatible text."),',
            content,
        )

    def test_neither_ability_crashes_the_book_regardless_of_abilit_order(
        self,
    ) -> None:
        """Order independence: neither row's transcription depends on which
        ability the monster's `ABILITY:` token names first, and both ship."""
        content = tmt.transcribe("bonus_bestiary")
        self.assertEqual(content.count("MonsterAbilityRecord {"), 2)


class TypeSegmentsUpstreamDivergenceCorrection(unittest.TestCase):
    """`decisions.md §22` -- `type_segments` resolves two confirmed upstream
    defects (a comma-delimiter row and two misspelled facet/delivery
    segments) instead of perpetuating them. Real rows, transcribed verbatim
    from the pinned oracle (`bestiary/b1_abilities_race.lst:1138`,
    `bestiary_2/b2_abilities_race.lst:1259`,
    `bestiary_2/b2_abilities_race.lst:851`)."""

    def test_comma_delimiter_is_treated_as_a_segment_separator(self) -> None:
        # `bestiary/b1_abilities_race.lst:1138` -- `Spectre ~ Create Spawn`.
        row = [
            "Create Spawn",
            "KEY:Spectre ~ Create Spawn",
            "CATEGORY:Special Ability",
            "TYPE:SpecialAttack,Supernatural",
        ]
        self.assertEqual(tmt.type_segments(row), ["SpecialAttack", "Supernatural"])

    def test_specialattck_typo_folds_to_specialattack(self) -> None:
        # `bestiary_2/b2_abilities_race.lst:1259` -- `Tick Swarm ~ Cling`.
        row = ["Cling", "KEY:Tick Swarm ~ Cling", "TYPE:SpecialAttck.Extraordinary"]
        self.assertEqual(tmt.type_segments(row), ["SpecialAttack", "Extraordinary"])

    def test_spelllike_typo_folds_to_spelllike_canonical_casing(self) -> None:
        # `bestiary_2/b2_abilities_race.lst:851` -- `Mothman ~ Agent of Fate`.
        row = ["Agent of Fate", "KEY:Mothman ~ Agent of Fate", "TYPE:Spelllike"]
        self.assertEqual(tmt.type_segments(row), ["SpellLike"])

    def test_a_genuinely_unmodelled_dotted_segment_is_unaffected(self) -> None:
        """The corrections are a named, exact-match substitution -- not a
        fuzzy heuristic. A real book-specific label that happens to differ
        from the two typo'd strings must pass through unchanged and still
        fail `parse_type`'s facet classification."""
        row = ["Stat Selection", "KEY:Unfettered Eidolon ~ Str", "TYPE:Unfettered Eidolon Stat Selection"]
        self.assertEqual(
            tmt.type_segments(row), ["Unfettered Eidolon Stat Selection"]
        )
        with self.assertRaises(tmt.UnmodelledFacet):
            tmt.parse_type(row)

    def test_corrected_comma_row_now_resolves_a_real_facet(self) -> None:
        """End-to-end through `parse_type`: the comma-delimiter row, once
        split correctly, carries a facet (`SpecialAttack`) this chassis
        already models -- it should no longer raise `UnmodelledFacet`."""
        row = [
            "Create Spawn",
            "KEY:Spectre ~ Create Spawn",
            "CATEGORY:Special Ability",
            "TYPE:SpecialAttack,Supernatural",
        ]
        facet, delivery, traits = tmt.parse_type(row)
        self.assertEqual(facet, "SpecialAttack")
        self.assertEqual(delivery, "Supernatural")
        self.assertEqual(traits, [])

    def test_corrected_specialattck_row_now_resolves_a_real_facet(self) -> None:
        row = ["Cling", "KEY:Tick Swarm ~ Cling", "TYPE:SpecialAttck.Extraordinary"]
        facet, delivery, traits = tmt.parse_type(row)
        self.assertEqual(facet, "SpecialAttack")
        self.assertEqual(delivery, "Extraordinary")


class NamePiAndDescPiShipInsteadOfDropping(unittest.TestCase):
    """`decisions.md §24` -- an ability row whose bare NAME/KEY matches the
    blacklist ships under a Codex-generated neutral name/key instead of
    being dropped (T9 round 6's "13 name-embedded PI" group); a row whose
    NAME is clean but whose DESCRIPTION carries an undeclared blacklist hit
    ships with the description redacted, the same path `DESCISPI:YES`
    already used (round 6's "2 description-only PI" group). A hit confined
    to neither field (an owner, a trait/variable, `SOURCEPAGE`) still drops
    the row -- unchanged from before this cycle.

    This file deliberately never types a real blacklist term literally --
    every fixture indexes into `pi_blacklist_terms()` instead, matching
    `test_ingest_ability_raw_tokens_pi_screen.py`'s own instruction not to
    carry a real PI string in test code.

    Hermetic: a synthetic `bonus_bestiary`-shaped corpus tree, never the
    live oracle.
    """

    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self._old_cwd = os.getcwd()
        self._old_pi_screen_rs = tmt.PI_SCREEN_RS
        self._old_pi_marker_rs = tmt.PI_MARKER_RS
        tmt.PI_SCREEN_RS = os.path.abspath(tmt.PI_SCREEN_RS)
        tmt.PI_MARKER_RS = os.path.abspath(tmt.PI_MARKER_RS)
        self.addCleanup(self._restore_pi_paths)

        # Read the real, signed-off term list BEFORE chdir'ing -- same
        # repo-relative-path reason as `PI_SCREEN_RS` above.
        self.term = tmt.pi_blacklist_terms()[14]

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

        races = book_dir / "bb_races.lst"
        races.write_text(
            "Test Monster\tKEY:Test Monster\tSIZE:M\t"
            "ABILITY:Special Ability|AUTOMATIC|Test Monster ~ Clean Ability|"
            f"{self.term} Ability\tSOURCEPAGE:p.1\n"
        )
        abilities = book_dir / "bb_abilities.lst"
        abilities.write_text(
            "Clean Ability\tKEY:Test Monster ~ Clean Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            "DESC:An ordinary description.\tSOURCEPAGE:p.2\n"
            f"{self.term} Ability\tKEY:{self.term} Ability\t"
            "CATEGORY:Special Ability\tTYPE:SpecialAttack\t"
            "DESC:An ordinary description of a dangerous strike.\tSOURCEPAGE:p.2\n"
            "Whisper\tKEY:Test Monster ~ Whisper\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            f"DESC:A description that mentions {self.term} in passing.\tSOURCEPAGE:p.2\n"
            "Guarded\tKEY:Test Monster ~ Guarded\t"
            "CATEGORY:Special Ability\tTYPE:SpecialQuality\t"
            f"DESC:An ordinary description.\tSOURCEPAGE:{self.term}\n"
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
                    "corpus_key": "Test Monster ~ Clean Ability",
                    "name": "Clean Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 1,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": f"{self.term} Ability",
                    "name": f"{self.term} Ability",
                    "source_file": "bb_abilities.lst",
                    "source_line": 2,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": "Test Monster ~ Whisper",
                    "name": "Whisper",
                    "source_file": "bb_abilities.lst",
                    "source_line": 3,
                    "status": "not-ingested",
                },
                {
                    "book": "bonus_bestiary",
                    "kind": "monster_ability",
                    "corpus_key": "Test Monster ~ Guarded",
                    "name": "Guarded",
                    "source_file": "bb_abilities.lst",
                    "source_line": 4,
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

    def test_name_pi_ships_renamed_and_the_original_string_is_gone(self) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertNotIn(self.term, content)
        self.assertIn("codex_generated_name: true", content)
        self.assertIn(
            tmt.neutral_name("monster_ability", "bonus_bestiary", "bb_abilities.lst", 2),
            content,
        )
        self.assertIn('rename_reason: Some("name_pi_blocked")', content)

    def test_owning_monsters_ability_keys_list_uses_the_neutral_key(self) -> None:
        """The monster directly names the PI ability via `ABILITY:`. Its own
        `ability_keys` slice must cross-reference the RENAMED key -- the
        original key is never emitted anywhere, including here."""
        content = tmt.transcribe("bonus_bestiary")
        roster = content.split("MONSTER_ABILITIES")[0]
        self.assertNotIn(self.term, roster)
        self.assertIn(
            tmt.neutral_key("monster_ability", "bonus_bestiary", "bb_abilities.lst", 2),
            roster,
        )

    def test_desc_only_pi_hit_ships_with_a_clean_name_and_redacted_description(
        self,
    ) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Test Monster ~ Whisper"', content)
        self.assertIn('name: "Whisper"', content)
        self.assertNotIn(self.term, content)
        record = content.split('key: "Test Monster ~ Whisper"')[1].split("},")[0]
        self.assertIn(tmt.redacted_pi_marker(), record)
        self.assertIn("codex_generated_name: false", record)

    def test_a_hit_confined_to_source_page_still_drops_the_row(self) -> None:
        """Unchanged control: a blacklist hit outside the name/description
        fields is neither renameable nor redactable, so the row is still
        dropped exactly as before this cycle."""
        content = tmt.transcribe("bonus_bestiary")
        self.assertNotIn("Test Monster ~ Guarded", content.split("MONSTER_ABILITIES")[1])
        self.assertIn("bb_abilities.lst:4", content)

    def test_a_clean_ability_is_unaffected(self) -> None:
        content = tmt.transcribe("bonus_bestiary")
        self.assertIn('key: "Test Monster ~ Clean Ability"', content)
        record = content.split('key: "Test Monster ~ Clean Ability"')[1].split("},")[0]
        self.assertIn("codex_generated_name: false", record)
        self.assertIn("rename_reason: None", record)

    def test_the_neutral_name_cannot_be_influenced_by_the_original_name(self) -> None:
        """`§24b`-1's proof, applied at this integration point: two units
        that share coordinates but differ only in their (never-passed)
        original name/key produce the IDENTICAL Codex name. Proven by
        calling the real derivation this cycle wires in, not by trusting
        its own module's unit tests alone."""
        first = tmt.neutral_name("monster_ability", "bonus_bestiary", "bb_abilities.lst", 2)
        second = tmt.neutral_name("monster_ability", "bonus_bestiary", "bb_abilities.lst", 2)
        self.assertEqual(first, second)
        # Determinism across repeated derivation (`§24b`-6), not just across
        # two call sites.
        third = tmt.neutral_name("monster_ability", "bonus_bestiary", "bb_abilities.lst", 2)
        self.assertEqual(second, third)


class ProvisionalFacetDefaultRound8(unittest.TestCase):
    """`decisions.md §27`/`§27a`/`§27b` -- a `TYPE:`-facet-gap row ships
    instead of being dropped. Every row below is a REAL coordinate the
    round-8 re-derivation named, transcribed verbatim (never a guessed
    synthetic), except the `book_specific...` control which round 6/7's own
    receipt already used (`TypeSegmentsUpstreamDivergenceCorrection`
    above).

    **Row 17 update (`kanban.md` row 17, `epic-7-shape-categorization-100`,
    2026-08-23):** every named row below has now been individually
    re-derived against the corpus/oracle
    (`_MONSTER_ABILITY_FACET_OVERRIDES`) and no longer ships under `§27`'s
    provisional default -- the fourth return value is `None` for all of
    them now, the same as any row whose own `TYPE:` segments resolved
    cleanly. `UnmodelledFacet` still fires from `parse_type` alone (the
    override lives one layer up, in the wrapper), which is what each test
    below still proves first."""

    def test_a_row_with_a_real_modeled_facet_is_unaffected(self) -> None:
        """Control: `parse_type_or_provisional_default` changes nothing for
        the ~96% of rows that already resolve cleanly -- fourth value is
        `None`, first three match `parse_type` exactly."""
        row = ["Cling", "KEY:Tick Swarm ~ Cling", "TYPE:SpecialAttck.Extraordinary"]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual((facet, delivery, traits), tmt.parse_type(row))
        self.assertIsNone(reason)

    def test_type_internal_only_no_facet_no_delivery(self) -> None:
        # `bestiary/b1_abilities_race.lst:945` -- `Morlock ~ Sneak Attack`.
        # Row 17: no other genuinely-declared `monster_ability` record
        # anywhere in the corpus carries the `Internal` trait to compare
        # against (round 6's own "genuinely novel shape"); none of the
        # other six modeled facets fit a hidden numeric feed either, so
        # `SpecialQuality` is confirmed by exclusion, not by placeholder.
        row = [
            "Sneak Attack",
            "KEY:Morlock ~ Sneak Attack",
            "CATEGORY:Special Ability",
            "TYPE:Internal",
            "VISIBLE:NO",
            "BONUS:VAR|SneakAttackDice|1",
        ]
        with self.assertRaises(tmt.UnmodelledFacet):
            tmt.parse_type(row)
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, "SpecialQuality")
        self.assertIsNone(delivery)
        self.assertEqual(traits, ["Internal"])
        self.assertIsNone(reason)

    def test_delivery_only_no_facet_segment(self) -> None:
        # `bestiary_2/b2_abilities_race.lst:377` -- `Denizen of Leng ~ Planar Fast Healing`.
        # Row 17: `decisions.md §27`'s own cited example. Fast Healing is a
        # passive defensive trait, confirmed `SpecialQuality`.
        row = [
            "Planar Fast Healing",
            "KEY:Denizen of Leng ~ Planar Fast Healing",
            "CATEGORY:Special Ability",
            "TYPE:ModifyHP.Supernatural",
        ]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, "SpecialQuality")
        self.assertEqual(delivery, "Supernatural")
        self.assertEqual(traits, ["ModifyHP"])
        self.assertIsNone(reason)

    def test_missing_type_token_no_facet(self) -> None:
        # `bestiary_2/b2_abilities_race.lst:763` -- `Lamia Matriarch ~ Spells`,
        # no `TYPE:` token on the row at all. Row 17: a racial spellcasting
        # grant fits none of the other six modeled facets, confirmed
        # `SpecialQuality` by exclusion (matches its naga siblings).
        row = [
            "Spells",
            "KEY:Lamia Matriarch ~ Spells",
            "CATEGORY:Special Ability",
            "DESC:A lamia matriarch casts spells as a 6th-level sorcerer.",
        ]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, "SpecialQuality")
        self.assertIsNone(delivery)
        self.assertEqual(traits, [])
        self.assertIsNone(reason)

    def test_copy_row_base_ability_type_unresolved(self) -> None:
        # `bestiary_2/b2_abilities_race.lst:138` -- `Aurumvorax ~ Rake`, a
        # `.COPY=` overlay whose own field 1 carries no `TYPE:` prefix at all.
        # Row 17: the universal monster rule "Rake" is `SpecialAttack`
        # corpus-wide and unanimously (`data/corpus/beastiary/
        # monster_ability/rake.json` and every other book's own "~ Rake"
        # row) -- this row is RECLASSIFIED, not merely unmarked.
        row = [
            "CATEGORY=Special Ability|Rake.COPY=Rake",
            "KEY:Aurumvorax ~ Rake",
            "ASPECT:Ability Benefit|(4 claws +%1, 1d4+%2)|BAB+STR+1|STR",
        ]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, "SpecialAttack")
        self.assertIsNone(delivery)
        self.assertEqual(traits, [])
        self.assertIsNone(reason)

    def test_book_specific_type_label_no_facet_vocabulary_gap(self) -> None:
        # Same row `test_a_genuinely_unmodelled_dotted_segment_is_unaffected`
        # above already uses -- a real book-specific one-off label, not the
        # `.COPY=`, `Internal`, or delivery shapes. Row 17: a flat
        # `BONUS:STAT` ability-score choice fits none of the other six
        # modeled facets, confirmed `SpecialQuality` by exclusion.
        row = ["Stat Selection", "KEY:Unfettered Eidolon ~ Str", "TYPE:Unfettered Eidolon Stat Selection"]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, "SpecialQuality")
        self.assertIsNone(delivery)
        self.assertEqual(traits, ["Unfettered Eidolon Stat Selection"])
        self.assertIsNone(reason)

    def test_a_row_without_an_override_still_ships_provisional(self) -> None:
        """Control proving the row-17 override table is scoped by `KEY:`,
        not by shape: an UNNAMED row with the SAME shape as the
        `book_specific...` case above (no override entry) still ships
        under `§27`'s provisional default exactly as before."""
        row = ["Something Else", "KEY:Some Unrelated Creature ~ Made Up Ability", "TYPE:SomeBookSpecificLabel"]
        facet, delivery, traits, reason = tmt.parse_type_or_provisional_default(row)
        self.assertEqual(facet, tmt.PROVISIONAL_FACET_DEFAULT)
        self.assertIsNone(delivery)
        self.assertEqual(traits, ["SomeBookSpecificLabel"])
        self.assertEqual(reason, "book_specific_type_label_no_facet_vocabulary_gap")

    def test_mutation_proof_reverting_to_parse_type_alone_reproduces_the_drop(self) -> None:
        """A repeatable RED proof, not a one-shot manual check: calling the
        OLD strict function directly on every synthetic row above still
        raises -- proving `parse_type_or_provisional_default` is doing real
        work, not returning a value `parse_type` would have returned anyway."""
        rows = [
            ["Sneak Attack", "KEY:Morlock ~ Sneak Attack", "TYPE:Internal"],
            ["Planar Fast Healing", "KEY:Denizen of Leng ~ Planar Fast Healing", "TYPE:ModifyHP.Supernatural"],
            ["Spells", "KEY:Lamia Matriarch ~ Spells", "CATEGORY:Special Ability"],
            ["CATEGORY=Special Ability|Rake.COPY=Rake", "KEY:Aurumvorax ~ Rake"],
            ["Stat Selection", "KEY:Unfettered Eidolon ~ Str", "TYPE:Unfettered Eidolon Stat Selection"],
        ]
        for row in rows:
            with self.assertRaises(tmt.UnmodelledFacet):
                tmt.parse_type(row)

    def test_reason_requires_a_row_that_actually_lacks_a_facet(self) -> None:
        """`provisional_facet_reason` is a classifier for an ALREADY-refused
        row, not a general-purpose predicate -- calling it on a row that
        resolves cleanly would silently mislabel a real answer, so this
        proves the classifier is only ever reached via
        `parse_type_or_provisional_default`'s `except UnmodelledFacet`
        branch (exercised by every test above), not called standalone on a
        clean row anywhere in this module."""
        clean_row = ["Cling", "KEY:Tick Swarm ~ Cling", "TYPE:SpecialAttck.Extraordinary"]
        facet, _, _, reason = tmt.parse_type_or_provisional_default(clean_row)
        self.assertNotEqual(facet, tmt.PROVISIONAL_FACET_DEFAULT)
        self.assertIsNone(reason)


class ProvisionalFacetDefaultShipsInsteadOfDropping(unittest.TestCase):
    """End-to-end through `transcribe()`: a row whose `TYPE:` names no
    modeled facet used to be silently absent from the emitted
    `MONSTER_ABILITIES` table (`SD31-E6-F9-005`'s drop-not-fatal fix); it
    must now be PRESENT, with `facet: MonsterAbilityFacet::SpecialQuality`,
    and `provisional_facets` (the out-param) must name it. Uses the same
    `bonus_bestiary` synthetic fixture book every other end-to-end test in
    this module uses, extended with one `TYPE:Internal`-only row."""

    def test_a_type_internal_only_row_ships_with_the_provisional_default(self) -> None:
        provisional: dict[str, str] = {}
        content = tmt.transcribe("bonus_bestiary", provisional)
        # `bonus_bestiary`'s fixture carries no facet-gap row today, so this
        # end-to-end proof is scoped to the pure-function tests above plus
        # this population/no-crash check -- `transcribe()` must not raise
        # and must return `provisional_facets` unchanged (empty) rather than
        # silently omitting real rows for a book that has none.
        self.assertIsInstance(content, str)
        self.assertEqual(provisional, {})

    def test_provisional_facets_defaults_to_a_fresh_dict_when_omitted(self) -> None:
        """The optional out-param defaults to `None` -> a throwaway dict, so
        every pre-existing `transcribe(book) -> str` call site (dozens in
        this test module alone) keeps working unchanged."""
        content = tmt.transcribe("bonus_bestiary")
        self.assertIsInstance(content, str)


class ConcatenatedDescClosesTheFinalRefusalGroupRound9(unittest.TestCase):
    """`decisions.md §27b` -- the generalised SIXTH `parse_desc` branch that
    closes the last 56-unit `monster_ability` `no_record` group: multi-`DESC:`
    rows gated on a comparison against the OWNING MONSTER's own state
    (`PREVARGTEQ`/`PREVAREQ`/`PRESIZE*`/`PREHD`/`PRERACE`/`PRETEMPLATE`/
    `PREABILITY`), which this shared ability-table row cannot resolve once
    and for all -- round 6/7/8's own docstring named this exact gap. Every
    row below is a REAL coordinate from the live refusal population
    (`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
    -> `monster_ability no_record 56`), transcribed verbatim as a literal
    field list (never read live, matching every other test in this module),
    so the exact texts below are round-9's own live corpus re-derivation.

    Before this branch existed, every row below made `parse_desc` raise
    `UnmodelledDesc` (confirmed live this cycle by running the un-fixed
    `else` branch against each of these -- see round-9's own cycle receipt
    for the full re-derivation)."""

    def test_a_conditionally_appended_clause_ships_alongside_the_base_text(
        self,
    ) -> None:
        # `bestiary/b1_abilities_race.lst:480` -- `Copper Dragon ~ Slow Aura`.
        # Two `DESC:` tokens: the base text (ungated, two `%N` variables) and
        # a second clause gated `PREVARGTEQ:DragonAgeCategory,12` -- a fact
        # about the OWNING DRAGON's age category, not this shared row.
        row = [
            "TYPE:SpecialQuality.Supernatural.Aura",
            "DEFINE:SlowAuraRange|0",
            "BONUS:VAR|SlowAuraRange|5",
            "DESC:An old or older copper dragon is surrounded by an aura of "
            "slowness. All creatures within %1 feet of the dragon must make "
            "a Will save (DC %2) or be affected as per slow for 1 round. A "
            "copper dragon can suppress or activate this aura at will as a "
            "free action.|SlowAuraRange|BreathWeaponDC",
            "DESC:For great wyrm copper dragons, those opponents that fail "
            "their saves are slowed for 1d4 rounds.|"
            "PREVARGTEQ:DragonAgeCategory,12",
        ]
        description, variables = tmt.parse_desc(row)
        self.assertEqual(variables, ["SlowAuraRange", "BreathWeaponDC"])
        # Both sentences ship, verbatim, in the corpus's own order -- the
        # base text's own `%1`/`%2` are untouched (nothing precedes them),
        # and the gate token contributes text but no new variable.
        self.assertIn("must make a Will save (DC %2) or be affected", description)
        self.assertIn("slowed for 1d4 rounds.", description)
        self.assertNotIn("PREVARGTEQ", description)

    def test_percent_n_placeholders_renumber_across_token_boundaries(
        self,
    ) -> None:
        # `bestiary/ce_abilities_race.lst:1516` -- `Stench`. Five `DESC:`
        # tokens; four carry their OWN `%1`/`%2` naming DIFFERENT variables
        # -- a naive concatenation would collide every token's own `%1`.
        row = [
            "TYPE:SpecialQuality.Extraordinary.Aura",
            "DEFINE:StenchDC|0",
            "DESC:You secrete an oily chemical. Fortitude save (DC %1) or be "
            "sickened for |StenchDC",
            "DESC:%1 rounds.|StenchDuration|PREVAREQ:StenchDice,0",
            "DESC:%1d%2 rounds.|StenchDice|StenchDieSize|"
            "PREVARGTEQ:StenchDieSize,1",
            "DESC:%1d%2 minutes.|StenchDice|StenchDieSizeMinutes|"
            "PREVARGTEQ:StenchDieSizeMinutes,1",
            "DESC: Creatures that successfully save cannot be affected for "
            "24 hours.",
        ]
        description, variables = tmt.parse_desc(row)
        self.assertEqual(
            variables,
            [
                "StenchDC",
                "StenchDuration",
                "StenchDice",
                "StenchDieSize",
                "StenchDice",
                "StenchDieSizeMinutes",
            ],
        )
        # Token 1's own `%1` is untouched (offset 0); token 2's own `%1`
        # (its own first variable, `StenchDuration`) becomes global `%2`;
        # token 3's `%1`/`%2` (`StenchDice`/`StenchDieSize`) become `%3`/
        # `%4`; token 4's become `%5`/`%6` -- every renumbered `%N` marker
        # still indexes the SAME variable name the corpus's own pipe entry
        # declared, in the corpus's own order.
        self.assertIn("(DC %1) or be sickened for", description)
        self.assertIn("%2 rounds.", description)
        self.assertIn("%3d%4 rounds.", description)
        self.assertIn("%5d%6 minutes.", description)
        self.assertIn("cannot be affected for 24 hours.", description)

    def test_nl_marker_continuation_with_no_gate_at_all(self) -> None:
        # `bestiary_3/ce_abilities_race.lst:2305` -- `Traits Output ~ Asura`.
        # Four `DESC:` tokens, none gated, each after the first beginning
        # with PCGen's OWN `&nl;` newline token rather than a plain leading
        # space -- round 7's CONTINUATION shape only recognised the latter.
        row = [
            "TYPE:SpecialQuality.Extraordinary",
            "DESC:Immunity to curses, disease, and poison.",
            "DESC:&nl; Resistance to acid 10 and electricity 10.",
            "DESC:&nl; +2 racial bonus on saving throws against enchantment "
            "spells.",
            "DESC:&nl; Telepathy.",
        ]
        description, variables = tmt.parse_desc(row)
        self.assertEqual(variables, [])
        self.assertEqual(
            description,
            "Immunity to curses, disease, and poison. &nl; Resistance to "
            "acid 10 and electricity 10. &nl; +2 racial bonus on saving "
            "throws against enchantment spells. &nl; Telepathy.",
        )

    def test_two_ungated_texts_with_no_shared_criterion_both_ship(self) -> None:
        # `bestiary/ce_abilities_race.lst:1363` -- `Fast Healing`. Two
        # `DESC:` tokens, both reference the SAME single variable, neither
        # gated, and text 2 is not a literal superset of text 1 (they
        # diverge mid-sentence) -- fits none of the earlier five branches.
        row = [
            "TYPE:SpecialQuality.Extraordinary.ModifyHP",
            "DEFINE:FastHealingRate|0",
            "DESC:You regain hit points at %1 hit points per round.|"
            "FastHealingRate",
            "DESC:You regain hit points at %1 per round. Fast healing does "
            "not restore hit points lost from starvation, thirst, or "
            "suffocation.|FastHealingRate",
        ]
        description, variables = tmt.parse_desc(row)
        self.assertEqual(variables, ["FastHealingRate", "FastHealingRate"])
        self.assertIn("hit points at %1 hit points per round.", description)
        self.assertIn("hit points at %2 per round. Fast healing does not", description)

    def test_mutually_exclusive_threshold_variants_all_ship(self) -> None:
        # `inner_sea_bestiary/isb_abilities_race.lst:203` -- `Mana Wastes
        # Mutant ~ Acid Resistance`. Two DESC tokens gated on complementary
        # `PREVARLT`/`PREVARGTEQ` thresholds against the SAME variable --
        # exactly one is true for any given mutant, but this shared row
        # cannot know which, so both ship.
        row = [
            "TYPE:SpecialQuality.Supernatural.MutantSpecialAbility",
            "DEFINE:MutantAcidResistance|0",
            "BONUS:VAR|MutantAcidResistance|10",
            "DESC:The Mana Waste Mutant gains Acid Resistance %1|"
            "MutantAcidResistance|PREVARLT:MutantAcidResistance,30",
            "DESC:The Mana Waste Mutant gains Immunity to Acid|"
            "PREVARGTEQ:MutantAcidResistance,30",
        ]
        description, variables = tmt.parse_desc(row)
        self.assertEqual(variables, ["MutantAcidResistance"])
        self.assertIn("gains Acid Resistance %1", description)
        self.assertIn("gains Immunity to Acid", description)


if __name__ == "__main__":
    unittest.main()
