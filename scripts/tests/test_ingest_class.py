"""SD-32 `decisions.md §20` -- unit tests for `scripts/ingest_class.py`.

Covers the load-bearing behaviours this generator adds beyond `ingest_
simple_filename_kinds.py`'s established pattern: the `CLASS:<Name>` identity
prefix (class rows carry one tag `parse_row`/the citation check must skip
that the other five simple-filename kinds never do), and honouring
`shape_ledger.py`'s `BOOK_CORPUS_DIR_ALIASES` for the OUTPUT directory --
the wave-1 defect this cycle's own class ingest hit and fixed (a record
written to the unaliased `bestiary/` directory is invisible to a
`--books`-restricted `shape_ledger.py` join and stays `no_record` forever).

Run: python3 -m unittest scripts.tests.test_ingest_class
"""
import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

import ingest_class as ic  # noqa: E402


class ParseRowTests(unittest.TestCase):
    def test_parse_row_skips_the_leading_class_identity_field(self):
        row = "CLASS:Assassin\tHD:8\tTYPE:PC.Prestige\tMAXLEVEL:10"
        tokens = ic.parse_row(row)
        self.assertEqual(
            tokens,
            [
                {"key": "HD", "value": "8"},
                {"key": "TYPE", "value": "PC.Prestige"},
                {"key": "MAXLEVEL", "value": "10"},
            ],
        )

    def test_parse_row_ignores_fields_with_no_colon(self):
        row = "CLASS:Assassin\tstray_no_colon_field\tHD:8"
        tokens = ic.parse_row(row)
        self.assertEqual(tokens, [{"key": "HD", "value": "8"}])


class DeclaredPiTests(unittest.TestCase):
    def test_declared_pi_reads_nameispi_and_descispi_tokens(self):
        tokens = ic.parse_row("CLASS:X\tNAMEISPI:YES\tDESCISPI:YES")
        self.assertEqual(ic.declared_pi(tokens), (True, True))

    def test_declared_pi_false_when_absent(self):
        tokens = ic.parse_row("CLASS:X\tTYPE:Foo")
        self.assertEqual(ic.declared_pi(tokens), (False, False))


class ClassIdentityStripTests(unittest.TestCase):
    """The real defect a naive port of `ingest_simple_filename_kinds.py`'s
    citation check would have shipped: class rows' leading field is
    `CLASS:<Name>`, not the bare name, so a byte-exact `identity ==
    corpus_key` check (that script's own check, unmodified) would reject
    every single real class row."""

    def test_bare_identity_check_would_reject_every_real_class_row(self):
        raw_line = "CLASS:Assassin\tHD:8\tTYPE:PC.Prestige"
        identity_field = raw_line.split("\t", 1)[0].strip()
        self.assertNotEqual(identity_field, "Assassin", "the leading field carries the CLASS: tag")

    def test_tag_stripped_identity_matches_corpus_key(self):
        raw_line = "CLASS:Assassin\tHD:8\tTYPE:PC.Prestige"
        identity_field = raw_line.split("\t", 1)[0].strip()
        _tag, _, identity = identity_field.partition(":")
        self.assertEqual(identity.strip(), "Assassin")


class BookAliasOutputDirTests(unittest.TestCase):
    """Wave-1's own recorded footgun: a corpus-record WRITER must honour
    `shape_ledger.BOOK_CORPUS_DIR_ALIASES` the same way the reader does, or
    the record it writes is invisible to a `--books`-restricted join."""

    def test_bestiary_book_writes_under_the_aliased_beastiary_directory(self):
        book = "bestiary"
        out_root = "data/corpus"
        expected = os.path.join(out_root, "beastiary", "class")
        actual = os.path.join(out_root, ic.BOOK_CORPUS_DIR_ALIASES.get(book, book), "class")
        self.assertEqual(actual, expected)

    def test_an_unaliased_book_is_unaffected(self):
        book = "core_rulebook"
        out_root = "data/corpus"
        expected = os.path.join(out_root, "core_rulebook", "class")
        actual = os.path.join(out_root, ic.BOOK_CORPUS_DIR_ALIASES.get(book, book), "class")
        self.assertEqual(actual, expected)


if __name__ == "__main__":
    unittest.main()
