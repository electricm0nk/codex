#!/usr/bin/env python3
"""Tests for `scripts/ingest_generic_kind.py` -- the generic, kind-
parameterized `no_record` transcriber for SD-32's small tails (`race`,
`monster`, `class`), mirroring `scripts/ingest_race_trait_generic.py`'s own
test coverage (SD-32 `decisions.md §17`/`§20`).

Covers: `row_tokens` (verbatim, identity-column-skipped tokenisation),
`declared_pi` (`NAMEISPI:`/`DESCISPI:` detection), `slugify` (collision-safe
slug assignment), and `load_units`/`load_no_record_ids` (the live-join
`no_record` filter, parameterized by `kind` -- never the possibly-stale
`status` field, per this bundle's own finding-5/6 lesson).
"""
import json
import os
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import ingest_generic_kind as gen  # noqa: E402


class RowTokensTests(unittest.TestCase):
    def test_skips_identity_column_and_splits_on_first_colon(self):
        line = "Some Name\tKEY:Foo ~ Bar\tCATEGORY:Special Ability\tBONUS:VAR|X|1+2"
        tokens = gen.row_tokens(line)
        self.assertEqual(
            tokens,
            [
                {"key": "KEY", "value": "Foo ~ Bar"},
                {"key": "CATEGORY", "value": "Special Ability"},
                {"key": "BONUS", "value": "VAR|X|1+2"},
            ],
        )

    def test_field_with_no_colon_gets_empty_value(self):
        line = "Some Name\tSTANDALONE\tDESC:hi"
        tokens = gen.row_tokens(line)
        self.assertEqual(tokens[0], {"key": "STANDALONE", "value": ""})


class DeclaredPiTests(unittest.TestCase):
    def test_nameispi_yes_is_detected(self):
        tokens = [{"key": "NAMEISPI", "value": "YES"}, {"key": "DESC", "value": "x"}]
        self.assertEqual(gen.declared_pi(tokens), (True, False))

    def test_descispi_yes_is_detected(self):
        tokens = [{"key": "DESCISPI", "value": "YES"}]
        self.assertEqual(gen.declared_pi(tokens), (False, True))

    def test_neither_declared_by_default(self):
        tokens = [{"key": "CATEGORY", "value": "Special Ability"}]
        self.assertEqual(gen.declared_pi(tokens), (False, False))


class SlugifyTests(unittest.TestCase):
    def test_basic_slug(self):
        used = set()
        self.assertEqual(gen.slugify("Dwarf ~ Stonecunning", used), "dwarf_stonecunning")

    def test_collision_gets_suffix(self):
        used = {"foo"}
        self.assertEqual(gen.slugify("Foo", used), "foo_2")


class ScrubNamePiTokensTests(unittest.TestCase):
    """`decisions.md §24b`-2: a token VALUE restating the record's own PI
    name/key must be scrubbed even when the token itself is not `NAME`."""

    def test_key_token_repeating_the_pi_name_is_redacted(self):
        tokens = [
            {"key": "KEY", "value": "Demon Lord (Pazuzu)"},
            {"key": "CATEGORY", "value": "Race"},
        ]
        scrubbed, any_redacted = gen.scrub_name_pi_tokens(tokens, "Pazuzu", "Demon Lord (Pazuzu)")
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], gen.REDACTED_PI_MARKER)
        self.assertEqual(scrubbed[1], {"key": "CATEGORY", "value": "Race"})

    def test_unrelated_token_is_never_mutated(self):
        tokens = [{"key": "CATEGORY", "value": "Race"}]
        scrubbed, any_redacted = gen.scrub_name_pi_tokens(tokens, "Pazuzu", "Demon Lord (Pazuzu)")
        self.assertFalse(any_redacted)
        self.assertEqual(scrubbed, tokens)
        self.assertIsNot(scrubbed, tokens)  # never mutates the input

    def test_pascalcase_compound_variable_identifier_embedding_the_name_is_redacted(self):
        """Real leak found live in this cycle's own dry-run output: PCGen
        `DEFINE`/`BONUS` tokens name their own class-scoped variables by
        concatenating the class name with no separator (`RedMantisAssassinLVL`,
        `WestcrownDevilLVL`) -- a space-preserving substring check on
        `"red mantis assassin" in value.lower()` never matches
        `"redmantisassassinlvl"` because the value has no spaces. This is
        exactly `§24b`-2's "appears nowhere that ships" and must be caught."""
        tokens = [
            {"key": "DEFINE", "value": "RedMantisAssassinLVL|0"},
            {"key": "BONUS", "value": "VAR|RedMantisAssassinLVL|CL"},
            {"key": "FACT", "value": "Abb|RMA"},
        ]
        scrubbed, any_redacted = gen.scrub_name_pi_tokens(
            tokens, "Red Mantis Assassin", "Red Mantis Assassin"
        )
        self.assertTrue(any_redacted)
        self.assertEqual(scrubbed[0]["value"], gen.REDACTED_PI_MARKER)
        self.assertEqual(scrubbed[1]["value"], gen.REDACTED_PI_MARKER)
        # A short, generic-looking abbreviation is left alone -- the
        # normalized-substring check is bounded to avoid over-redacting on
        # coincidental short matches.
        self.assertEqual(scrubbed[2]["value"], "Abb|RMA")


class LoadUnitsIsKindScopedAndUsesLiveJoinTests(unittest.TestCase):
    """The regression this test guards: (1) the generic ingester must be
    scoped to the `--kind` it was invoked for -- a `race`-kind unit must
    never leak into a `monster`-kind run and vice versa; (2) `docs/work-
    inventory.json`'s own `status` field has drifted from the live corpus
    join before -- a unit already carrying a real corpus record but still
    stamped `status: not-ingested` in the inventory must NOT be
    re-transcribed, because `load_units` filters on the ledger's
    `join_status`, never on `status`."""

    def test_units_filtered_by_kind_and_live_join_not_status_field(self):
        gen.INVENTORY_PATH = os.path.join(
            os.path.dirname(__file__), "fixtures", "ingest_generic_kind_inventory.json"
        )
        os.makedirs(os.path.dirname(gen.INVENTORY_PATH), exist_ok=True)
        with open(gen.INVENTORY_PATH, "w", encoding="utf-8") as fh:
            json.dump(
                {
                    "units": [
                        {
                            "id": "book:race:stale-but-ingested",
                            "kind": "race",
                            "book": "book",
                            "status": "not-ingested",
                        },
                        {
                            "id": "book:race:genuinely-open",
                            "kind": "race",
                            "book": "book",
                            "status": "not-ingested",
                        },
                        {
                            "id": "book:monster:wrong-kind",
                            "kind": "monster",
                            "book": "book",
                            "status": "not-ingested",
                        },
                    ]
                },
                fh,
            )
        try:
            no_record_ids = {"book:race:genuinely-open"}
            units = gen.load_units(no_record_ids, "race")
            self.assertEqual([u["id"] for u in units], ["book:race:genuinely-open"])
        finally:
            os.remove(gen.INVENTORY_PATH)

    def test_load_no_record_ids_is_kind_scoped(self):
        ledger_path = os.path.join(
            os.path.dirname(__file__), "fixtures", "ingest_generic_kind_ledger.json"
        )
        os.makedirs(os.path.dirname(ledger_path), exist_ok=True)
        with open(ledger_path, "w", encoding="utf-8") as fh:
            json.dump(
                {
                    "rows": [
                        {"id": "a", "kind": "race", "join_status": "no_record"},
                        {"id": "b", "kind": "monster", "join_status": "no_record"},
                        {"id": "c", "kind": "race", "join_status": "matched"},
                    ]
                },
                fh,
            )
        try:
            self.assertEqual(gen.load_no_record_ids(ledger_path, "race"), {"a"})
            self.assertEqual(gen.load_no_record_ids(ledger_path, "monster"), {"b"})
        finally:
            os.remove(ledger_path)


class ExistingOnDiskSlugIsNeverOverwrittenTests(unittest.TestCase):
    """Regression test for a real collision found live this cycle: a
    pre-existing `<kind>_generic/<slug>.json` file for one unit was
    silently overwritten by a DIFFERENT unit that happened to slugify to
    the identical string in a later, separate script invocation (two
    distinct source lines sharing the exact same `corpus_key` text). The
    fix seeds each book's `used` slug set from the filesystem, not only
    from slugs assigned earlier in the same run."""

    def test_a_second_run_writing_a_colliding_slug_gets_a_suffix_not_an_overwrite(self):
        tmp = tempfile.mkdtemp()
        try:
            pcgen_root = os.path.join(tmp, "pcgen_data")
            book_dir = os.path.join(pcgen_root, "book")
            os.makedirs(book_dir)
            with open(os.path.join(book_dir, "file.lst"), "w", encoding="utf-8") as fh:
                fh.write("Some Name\tKEY:Same Key\tDESC:hi\n")

            repo_root = os.path.join(tmp, "repo")
            os.makedirs(repo_root)
            out_dir = os.path.join(repo_root, "data/corpus/book/kind_generic")
            os.makedirs(out_dir)
            preexisting = {"data": {"name": "PRE-EXISTING, DO NOT TOUCH"}}
            with open(os.path.join(out_dir, "same_key.json"), "w", encoding="utf-8") as fh:
                json.dump(preexisting, fh)

            inventory_path = os.path.join(tmp, "inventory.json")
            with open(inventory_path, "w", encoding="utf-8") as fh:
                json.dump(
                    {
                        "units": [
                            {
                                "id": "book:kind:new-unit",
                                "kind": "kind",
                                "book": "book",
                                "source_file": "file.lst",
                                "source_line": 1,
                                "name": "Same Key",
                                "corpus_key": "Same Key",
                            }
                        ]
                    },
                    fh,
                )
            ledger_path = os.path.join(tmp, "ledger.json")
            with open(ledger_path, "w", encoding="utf-8") as fh:
                json.dump({"rows": [{"id": "book:kind:new-unit", "kind": "kind", "join_status": "no_record"}]}, fh)

            old_repo_root = gen.REPO_ROOT
            old_inventory = gen.INVENTORY_PATH
            old_argv = sys.argv
            old_environ = os.environ.get("PCGEN_CORPUS_ROOT")
            gen.REPO_ROOT = repo_root
            gen.INVENTORY_PATH = inventory_path
            os.environ["PCGEN_CORPUS_ROOT"] = pcgen_root
            sys.argv = ["ingest_generic_kind.py", "--kind", "kind", "--ledger", ledger_path]
            try:
                rc = gen.main()
                self.assertEqual(rc, 0)
                # The pre-existing file must be untouched...
                with open(os.path.join(out_dir, "same_key.json"), encoding="utf-8") as fh:
                    self.assertEqual(json.load(fh), preexisting)
                # ...and the new unit lands under a suffixed slug instead.
                self.assertTrue(os.path.exists(os.path.join(out_dir, "same_key_2.json")))
            finally:
                gen.REPO_ROOT = old_repo_root
                gen.INVENTORY_PATH = old_inventory
                sys.argv = old_argv
                if old_environ is None:
                    os.environ.pop("PCGEN_CORPUS_ROOT", None)
                else:
                    os.environ["PCGEN_CORPUS_ROOT"] = old_environ
        finally:
            shutil.rmtree(tmp, ignore_errors=True)


if __name__ == "__main__":
    unittest.main()
