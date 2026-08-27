#!/usr/bin/env python3
"""Tests for `scripts/ingest_race_trait_generic.py` -- the generic,
book-agnostic `race_trait` `no_record` transcriber (SD-32 `decisions.md
§17`/`§20`, mirroring `scripts/ingest_ability.py`'s wave-1 precedent).

Covers: `row_tokens` (verbatim, identity-column-skipped tokenisation),
`declared_pi` (`NAMEISPI:`/`DESCISPI:` detection), `slugify` (collision-safe
slug assignment), and `load_units` (the live-join `no_record` filter --
never the possibly-stale `status` field, per this bundle's own finding-5/6
lesson from `epic-2-t2b-pure-ability-pointer-row-fix_cycle-1_cycle_receipt.md`).
"""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import ingest_race_trait_generic as gen  # noqa: E402


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


class LoadUnitsUsesLiveJoinNotStatusFieldTests(unittest.TestCase):
    """The regression this test guards: `docs/work-inventory.json`'s own
    `status` field has drifted from the live corpus join before (finding
    5/6, cited in the module doc comment) -- a unit already carrying a real
    corpus record but still stamped `status: engine-does-not-hold` in the inventory
    must NOT be re-transcribed, because `load_units` filters on the ledger's
    `join_status`, never on `status`."""

    def test_unit_excluded_when_absent_from_no_record_ids_even_if_status_stale(self):
        gen.INVENTORY_PATH = os.path.join(
            os.path.dirname(__file__), "fixtures", "ingest_race_trait_generic_inventory.json"
        )
        os.makedirs(os.path.dirname(gen.INVENTORY_PATH), exist_ok=True)
        import json

        with open(gen.INVENTORY_PATH, "w", encoding="utf-8") as fh:
            json.dump(
                {
                    "units": [
                        {
                            "id": "book:race_trait:stale-but-ingested",
                            "kind": "race_trait",
                            "book": "book",
                            "status": "engine-does-not-hold",
                        },
                        {
                            "id": "book:race_trait:genuinely-open",
                            "kind": "race_trait",
                            "book": "book",
                            "status": "engine-does-not-hold",
                        },
                        {
                            "id": "book:class_feature:wrong-kind",
                            "kind": "class_feature",
                            "book": "book",
                            "status": "engine-does-not-hold",
                        },
                    ]
                },
                fh,
            )
        try:
            no_record_ids = {"book:race_trait:genuinely-open"}
            units = gen.load_units(no_record_ids)
            self.assertEqual([u["id"] for u in units], ["book:race_trait:genuinely-open"])
        finally:
            os.remove(gen.INVENTORY_PATH)


if __name__ == "__main__":
    unittest.main()
