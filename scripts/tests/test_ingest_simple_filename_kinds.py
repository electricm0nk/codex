"""SD-32 `decisions.md §20` -- unit tests for `scripts/ingest_simple_filename_kinds.py`.

Covers the three load-bearing behaviours: raw-token parsing, the group-header
citation match (`<group> ~ <leaf>`), and PI redaction (both PCGen's own
declared `NAMEISPI:`/`DESCISPI:` tokens and the shared term-list scan).

Run: python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds
"""
import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

import ingest_simple_filename_kinds as isfk  # noqa: E402
from shape_ledger import BOOK_CORPUS_DIR_ALIASES  # noqa: E402


class ParseRowTests(unittest.TestCase):
    def test_parse_row_splits_tab_delimited_key_value_fields(self):
        row = "Acrobatics\tKEYSTAT:Dex\tTYPE:Physical\tBONUS:SKILL|Acrobatics|3|TYPE=ClassSkill"
        tokens = isfk.parse_row(row)
        self.assertIn({"key": "KEYSTAT", "value": "Dex"}, tokens)
        self.assertIn({"key": "TYPE", "value": "Physical"}, tokens)
        self.assertIn({"key": "BONUS", "value": "SKILL|Acrobatics|3|TYPE=ClassSkill"}, tokens)

    def test_parse_row_ignores_fields_with_no_colon(self):
        row = "Acrobatics\tstray_no_colon_field\tKEYSTAT:Dex"
        tokens = isfk.parse_row(row)
        self.assertEqual(tokens, [{"key": "KEYSTAT", "value": "Dex"}])


class DeclaredPiTests(unittest.TestCase):
    def test_declared_pi_reads_nameispi_and_descispi_tokens(self):
        tokens = isfk.parse_row("X\tNAMEISPI:YES\tDESCISPI:YES")
        self.assertEqual(isfk.declared_pi(tokens), (True, True))

    def test_declared_pi_false_when_absent(self):
        tokens = isfk.parse_row("X\tTYPE:Foo")
        self.assertEqual(isfk.declared_pi(tokens), (False, False))


class CitationMatchTests(unittest.TestCase):
    @staticmethod
    def matches(identity: str, corpus_key: str) -> bool:
        return identity == corpus_key or corpus_key.endswith(" ~ " + identity)

    def test_citation_match_accepts_group_header_prefixed_corpus_key(self):
        """`v06_work_inventory.rs` composes some `corpus_key`s as
        `<group header> ~ <row identity>` while the LST row's own leading
        field is the bare leaf -- the exact shape
        `data/corpus/core_rulebook/class_feature/air_domain/lightning_arc.json`
        already ships (`record_key: "Air Domain ~ Lightning Arc"`)."""
        self.assertTrue(self.matches("Lightning Arc", "Air Domain ~ Lightning Arc"))

    def test_citation_match_rejects_unrelated_identity(self):
        self.assertFalse(self.matches("Something Else", "Air Domain ~ Lightning Arc"))


class FreeTextAndPiScanTests(unittest.TestCase):
    def test_free_text_of_extracts_desc_field_only_up_to_pipe(self):
        tokens = isfk.parse_row("X\tDESC:Some prose about Nex.|Var1\tTYPE:Foo")
        self.assertEqual(isfk.free_text_of(tokens), "Some prose about Nex.")

    def test_term_list_hit_on_core_deity_name_in_free_text(self):
        # t9-onboarding-pi-final-leaks-and-generators cycle: the module now
        # scans via `blacklist_term_hit_including_concatenated` (strong scan),
        # not the weaker `normalized_term_hit` -- see that import's own
        # module-level comment for why (adjectival/demonym forms with a
        # suffix concatenated onto the root, no word boundary after it).
        self.assertEqual(
            isfk.blacklist_term_hit_including_concatenated("A shrine sacred to Abadar stands here."), "Abadar"
        )

    def test_term_list_no_hit_on_ordinary_text(self):
        self.assertIsNone(isfk.blacklist_term_hit_including_concatenated("A shrine sacred to nobody stands here."))

    def test_term_list_hit_on_a_demonym_form_with_no_word_boundary_after_the_root(self):
        # The exact live shape this cycle's fix closes: `declared_pi_shipping_
        # audit`'s CHECK C found 3 `template` records shipped by this script
        # under a demonym name -- a real blacklisted place-name root
        # (indices 23 and 33) immediately followed by a suffix with no
        # separator, so the OLD word-bounded-only `normalized_term_hit`
        # never caught it (no boundary exists after the root). Built from
        # the indexed term, never typed literally, per `decisions.md §24b`-2.
        demonym_23 = isfk.PI_BLACKLIST_TERMS[23] + "n"
        demonym_33 = isfk.PI_BLACKLIST_TERMS[33] + "i"
        self.assertIsNotNone(isfk.blacklist_term_hit_including_concatenated(demonym_23))
        self.assertIsNotNone(isfk.blacklist_term_hit_including_concatenated(demonym_33))


class UnitInScopeTests(unittest.TestCase):
    """`--book` scoped-remediation filter (t9-onboarding-pi-final-leaks-and-
    generators cycle): the Python-side sibling of `gen_cache_class_feature.
    rs`'s own `--coordinates <file>` mode -- re-running a full `--kind` pass
    touches every unit of that kind corpus-wide (2,248 for `template`
    alone), an unacceptable blast radius for closing a handful of newly-
    confirmed leaks after a scan-strength fix."""

    def test_wrong_kind_is_out_of_scope_regardless_of_book(self):
        self.assertFalse(isfk.unit_in_scope("power", {"template"}, "any_book", None))

    def test_right_kind_with_no_book_filter_is_in_scope(self):
        self.assertTrue(isfk.unit_in_scope("template", {"template"}, "any_book", None))

    def test_right_kind_but_book_not_in_the_filter_set_is_out_of_scope(self):
        self.assertFalse(isfk.unit_in_scope("template", {"template"}, "other_book", {"inner_sea_world_guide"}))

    def test_right_kind_and_book_in_the_filter_set_is_in_scope(self):
        self.assertTrue(isfk.unit_in_scope("template", {"template"}, "inner_sea_world_guide", {"inner_sea_world_guide"}))

    def test_empty_book_filter_set_excludes_everything(self):
        # Sanity: `books=None` (unset) must behave differently from
        # `books=set()` (explicitly empty) -- the former means "no
        # restriction", the latter means "nothing matches".
        self.assertFalse(isfk.unit_in_scope("template", {"template"}, "inner_sea_world_guide", set()))


class ComposeSourcePathTests(unittest.TestCase):
    """Mechanical control for the SD-32 `decisions.md §20` defect: 3,124
    records shipped with `source.path` missing its leading `pathfinder/`
    segment because the caller joined `pcgen_root` with `"pathfinder"`
    before taking the relpath, double-stripping the segment
    `corpus_literal_sweep`'s `book_dir_of` shape check requires. This must
    fail red if that mistake is ever reintroduced."""

    def test_compose_source_path_keeps_leading_system_segment(self):
        root = "/fake/pcgen/data"
        file_path = "/fake/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst"
        self.assertEqual(
            isfk.compose_source_path(file_path, root),
            "pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst",
        )

    def test_compose_source_path_rejects_a_pcgen_root_pre_joined_with_pathfinder(self):
        """Reproduces the actual bug: passing `os.path.join(pcgen_root,
        "pathfinder")` as the relpath base (the code shape this cycle
        removed) strips the leading system segment and must be refused."""
        root = "/fake/pcgen/data"
        buggy_root = os.path.join(root, "pathfinder")
        file_path = "/fake/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst"
        with self.assertRaises(ValueError):
            isfk.compose_source_path(file_path, buggy_root)

    def test_compose_source_path_accepts_the_dreamscarred_press_shape(self):
        root = "/fake/pcgen/data"
        file_path = "/fake/pcgen/data/pathfinder/dreamscarred_press/dsp_book/dsp_x.lst"
        self.assertEqual(
            isfk.compose_source_path(file_path, root),
            "pathfinder/dreamscarred_press/dsp_book/dsp_x.lst",
        )


class RowIdentityTests(unittest.TestCase):
    """A `KEY:` token, when present, overrides the leading display-name
    column as the row's real identifying string -- the convention
    `ingest_races.rs`/`ingest_race_traits.rs`/
    `derive_monster_ability_save_dc_fixtures.py` already honour."""

    def test_key_token_overrides_leading_column_when_present(self):
        row = "Has Swim Speed\tKEY:Swimming Master ~ Has Swim\tVISIBLE:NO"
        self.assertEqual(isfk.row_identity(row), "Swimming Master ~ Has Swim")

    def test_falls_back_to_leading_column_when_no_key_token(self):
        row = "Acrobatics\tKEYSTAT:Dex\tTYPE:Physical"
        self.assertEqual(isfk.row_identity(row), "Acrobatics")

    def test_key_token_beats_a_similarly_named_keystat_token(self):
        # KEYSTAT: must never be mistaken for KEY: (startswith("KEY:") guards this).
        row = "Fallback Name\tKEYSTAT:Dex"
        self.assertEqual(isfk.row_identity(row), "Fallback Name")


class OutputDirAliasTests(unittest.TestCase):
    """`shape_ledger.py`'s reader joins a unit's `book` against the aliased
    corpus directory (e.g. `bestiary` -> `data/corpus/beastiary/`). The
    writer's `out_dir` computation must apply the identical alias, or every
    record it writes under the unaliased directory is invisible to the join
    forever (decisions.md §20 footgun 1, re-confirmed: 1,051 bestiary
    template/language units)."""

    def test_out_dir_book_segment_matches_shape_ledger_alias_for_bestiary(self):
        self.assertEqual(BOOK_CORPUS_DIR_ALIASES.get("bestiary", "bestiary"), "beastiary")
        self.assertEqual(
            isfk.resolve_out_dir("data/corpus", "bestiary", "template"),
            os.path.join("data/corpus", "beastiary", "template"),
        )

    def test_out_dir_book_segment_is_unchanged_for_a_non_aliased_book(self):
        self.assertEqual(
            isfk.resolve_out_dir("data/corpus", "core_rulebook", "template"),
            os.path.join("data/corpus", "core_rulebook", "template"),
        )


class SlugifyTests(unittest.TestCase):
    def test_slugify_collapses_non_alnum_and_lowercases(self):
        self.assertEqual(
            isfk.slugify("Race Builder Subtype ~ Charau-ka"),
            "race_builder_subtype_charau_ka",
        )


if __name__ == "__main__":
    unittest.main()
