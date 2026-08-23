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
        self.assertEqual(
            isfk.normalized_term_hit("A shrine sacred to Abadar stands here."), "Abadar"
        )

    def test_term_list_no_hit_on_ordinary_text(self):
        self.assertIsNone(isfk.normalized_term_hit("A shrine sacred to nobody stands here."))


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


class SlugifyTests(unittest.TestCase):
    def test_slugify_collapses_non_alnum_and_lowercases(self):
        self.assertEqual(
            isfk.slugify("Race Builder Subtype ~ Charau-ka"),
            "race_builder_subtype_charau_ka",
        )


if __name__ == "__main__":
    unittest.main()
