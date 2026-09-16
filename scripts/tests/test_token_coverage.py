"""Unit tests for `scripts/token_coverage.py` -- SD-35 `AT-35-E2-004`
(`docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md`), enforcing
`decisions.md` §8 (every figure states its denominator) and `§9` L9 (sum the
piles, always): the remainder is named by token type, and the token sums are
checked.

Every test builds its own synthetic package (`_tokens.json`, `_refused.json`,
`_report.json`), inventory and mapping table under a temp dir and runs the
ledger's real CLI entry point against them, so the RED->GREEN evidence the
criterion asks for -- plant a double-count, the check fails; remove it, the
check passes -- is executed, not narrated. No test here reads the real
repository: the `token-coverage` stage in `scripts/verify.sh` is what runs
the ledger on the live tree.
"""

import copy
import io
import json
import os
import shutil
import sys
import tempfile
import unittest
from contextlib import redirect_stdout

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import token_coverage as tc  # noqa: E402


def _unit(id_, kind, status, evidence="x"):
    book = id_.split(":", 1)[0]
    return {"id": id_, "book": book, "kind": kind, "status": status, "evidence": evidence}


def _census(id_, kind, tokens, refusals=None):
    book = id_.split(":", 1)[0]
    return {"id": id_, "book": book, "kind": kind, "tokens": list(tokens), "refusals": refusals or {}}


class _Fixture(unittest.TestCase):
    """Six units: two DONE, four non-DONE; three refused (one of them DONE);
    one token-less; one carrying a head the table has no row for."""

    def setUp(self):
        self.root = tempfile.mkdtemp(prefix="token-coverage-")
        self.package = os.path.join(self.root, "sheet_rules")
        os.makedirs(self.package)
        self.inventory_path = os.path.join(self.root, "work-inventory.json")
        self.table_path = os.path.join(self.root, "mapping-table.v1.json")
        self.out_path = os.path.join(self.root, "token-coverage.json")

        self.units = [
            _unit("b1:feat:u1", "feat", "engine-does-not-hold"),          # non-DONE, converted, A+B
            _unit("b1:feat:u2", "feat", "grounded"),                      # DONE, converted, A
            _unit("b1:feat:u3", "feat", "ingested-magnitude"),            # non-DONE, refused under B
            _unit("b1:spell:u4", "spell", "engine-does-not-hold"),        # non-DONE, token-less, refused
            _unit("b2:feat:u5", "feat", "text-complete"),                 # DONE, refused under A
            _unit("b2:trait:u6", "trait", "engine-does-not-hold"),        # non-DONE, converted, unmapped head
        ]
        self.census = [
            _census("b1:feat:u1", "feat", ["A", "B"]),
            _census("b1:feat:u2", "feat", ["A"]),
            _census("b1:feat:u3", "feat", ["B"], {"B (shape)": ["B"]}),
            _census("b1:spell:u4", "spell", [], {"no_corpus_record": ["token-less"]}),
            _census("b2:feat:u5", "feat", ["A"], {"A:[redacted PI]": ["A"]}),
            _census("b2:trait:u6", "trait", ["unmapped:C"]),
        ]
        self.refused = {
            "records": 6, "converted": 3, "refused": 3,
            "by_token_type": {"B (shape)": 1, "no_corpus_record": 1, "A:[redacted PI]": 1},
            "entries": [
                {"id": "b1:feat:u3", "book": "b1", "kind": "feat", "token_types": ["B (shape)"]},
                {"id": "b1:spell:u4", "book": "b1", "kind": "spell", "token_types": ["no_corpus_record"]},
                {"id": "b2:feat:u5", "book": "b2", "kind": "feat", "token_types": ["A:[redacted PI]"]},
            ],
        }
        self.report = {"records": 6, "converted": 3, "refused": 3, "oracle_pin": "deadbeef",
                       "converter_version": "sheet_rule_convert/test"}
        self.table = {"schema": "mapping-table.v1", "rows": [
            {"token_type": "A", "family": "bonus", "maps_to": "Number(Expr)"},
            {"token_type": "A:[redacted PI]", "family": "bonus", "maps_to": "REFUSE"},
            {"token_type": "B", "family": "prose", "maps_to": "Text"},
            {"token_type": "B (shape)", "family": "prose", "maps_to": "REFUSE"},
        ]}
        self._write_all()

    def tearDown(self):
        shutil.rmtree(self.root, ignore_errors=True)

    def _write_all(self):
        with open(self.inventory_path, "w", encoding="utf-8") as fh:
            json.dump({"generated_at": "2026-09-08T00:00:00Z", "units": self.units}, fh)
        with open(os.path.join(self.package, "_tokens.json"), "w", encoding="utf-8") as fh:
            json.dump({"schema": 1, "entries": self.census}, fh)
        with open(os.path.join(self.package, "_refused.json"), "w", encoding="utf-8") as fh:
            json.dump(self.refused, fh)
        with open(os.path.join(self.package, "_report.json"), "w", encoding="utf-8") as fh:
            json.dump(self.report, fh)
        with open(self.table_path, "w", encoding="utf-8") as fh:
            json.dump(self.table, fh)

    def _run(self, *extra):
        argv = ["--inventory", self.inventory_path, "--package", self.package,
                "--table", self.table_path, "--out", self.out_path, *extra]
        buf = io.StringIO()
        with redirect_stdout(buf):
            code = tc.main(argv)
        out = buf.getvalue()
        lines = [ln for ln in out.splitlines() if ln.strip()]
        return code, out, (lines[-1] if lines else "")

    def _ledger(self):
        with open(self.out_path, encoding="utf-8") as fh:
            return json.load(fh)


class LedgerRows(_Fixture):
    def test_derive_writes_the_ledger_and_prints_the_sums(self):
        code, out, last = self._run()
        self.assertEqual(code, 0, out)
        self.assertIn("non_done=4", last)
        self.assertIn("tokened=3", last)
        self.assertIn("token_less=1", last)
        self.assertIn("refused=3", last)
        self.assertIn("refused_non_done=2", last)
        self.assertTrue(last.endswith("verdict=PASS"), last)
        ledger = self._ledger()
        self.assertEqual(ledger["denominators"]["units"], 6)
        self.assertEqual(ledger["denominators"]["non_done"], 4)
        self.assertEqual(ledger["denominators"]["refused_records"], 3)
        self.assertEqual(ledger["verdict"], "PASS")

    def test_per_token_rows_count_carrying_converted_and_refused_because_of_this_token(self):
        self._run()
        rows = self._ledger()["token_types"]
        a = rows["A"]
        self.assertEqual(a["carrying"], 3)                        # u1 u2 u5
        self.assertEqual(a["carrying_non_done"], 1)               # u1
        self.assertEqual(a["converted_non_done"], 1)
        self.assertEqual(a["refused_non_done"], 0)
        self.assertEqual(a["refused_because_of_this_token"], 1)   # u5 (DONE)
        self.assertEqual(a["refused_because_of_this_token_non_done"], 0)
        self.assertEqual(a["mapping_row"], {"family": "bonus", "maps_to": "Number(Expr)"})
        self.assertEqual(a["refusal_shapes"], {"A:[redacted PI]": 1})
        b = rows["B"]
        self.assertEqual(b["carrying_non_done"], 2)               # u1 u3
        self.assertEqual(b["converted_non_done"], 1)              # u1
        self.assertEqual(b["refused_non_done"], 1)                # u3
        self.assertEqual(b["refused_because_of_this_token_non_done"], 1)
        self.assertEqual(b["refusal_shapes"], {"B (shape)": 1})

    def test_token_less_is_its_own_row_and_an_unmapped_head_has_no_mapping_row(self):
        self._run()
        ledger = self._ledger()
        rows = ledger["token_types"]
        tl = rows["token-less"]
        self.assertEqual(tl["carrying_non_done"], 1)              # u4
        self.assertEqual(tl["refused_because_of_this_token"], 1)
        self.assertEqual(tl["refusal_shapes"], {"no_corpus_record": 1})
        self.assertIsNone(tl["mapping_row"])
        c = rows["unmapped:C"]
        self.assertIsNone(c["mapping_row"])
        self.assertEqual(c["carrying_non_done"], 1)
        self.assertIn("unmapped:C", ledger["unmapped_token_types"])
        self.assertNotIn("A", ledger["unmapped_token_types"])

    def test_refusal_shapes_section_names_the_token_each_shape_arose_under(self):
        self._run()
        shapes = self._ledger()["refusal_shapes"]
        self.assertEqual(shapes["B (shape)"], {"records": 1, "non_done": 1, "under": {"B": 1}})
        self.assertEqual(shapes["A:[redacted PI]"], {"records": 1, "non_done": 0, "under": {"A": 1}})
        self.assertEqual(shapes["no_corpus_record"]["under"], {"token-less": 1})

    def test_rederiving_the_same_inputs_is_byte_identical(self):
        self._run()
        with open(self.out_path, "rb") as fh:
            first = fh.read()
        self._run()
        with open(self.out_path, "rb") as fh:
            second = fh.read()
        self.assertEqual(first, second)


class RedGreen(_Fixture):
    """The planted double-count the criterion names, and the other sums."""

    def test_a_planted_duplicate_census_entry_fails_the_check(self):
        self.census.append(copy.deepcopy(self.census[0]))   # u1 counted twice under A and B
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_DOUBLE_COUNT"), last)
        # Remove the plant: green again.
        self.census.pop()
        self._write_all()
        self._run()   # derive the committed ledger first
        code, out, last = self._run("--check")
        self.assertEqual(code, 0, out)
        self.assertTrue(last.endswith("verdict=PASS"), last)

    def test_a_planted_duplicate_token_on_one_record_fails_the_check(self):
        self.census[0]["tokens"] = ["A", "A", "B"]
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_DOUBLE_COUNT"), last)

    def test_a_non_done_unit_under_no_token_and_not_token_less_fails_coverage(self):
        self.census = [c for c in self.census if c["id"] != "b2:trait:u6"]
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_COVERAGE"), last)
        self.assertIn("b2:trait:u6", out)

    def test_a_refused_id_the_census_does_not_refuse_fails_the_refused_set_check(self):
        self.refused["entries"].append({"id": "b1:feat:u1", "book": "b1", "kind": "feat", "token_types": ["B (shape)"]})
        self.refused["by_token_type"]["B (shape)"] = 2
        self.refused["refused"] = 4
        self.refused["converted"] = 2
        self.report["refused"] = 4
        self.report["converted"] = 2
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_REFUSED_SET"), last)

    def test_a_shape_total_that_does_not_sum_fails_the_check(self):
        self.refused["by_token_type"]["B (shape)"] = 2
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_SHAPE_TOTALS"), last)

    def test_a_census_that_does_not_cover_the_population_fails(self):
        self.report["records"] = 7
        self._write_all()
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_POPULATION"), last)

    def test_check_fails_on_a_stale_committed_ledger_and_rewrites_it(self):
        self._run()
        ledger = self._ledger()
        ledger["token_types"]["A"]["carrying_non_done"] = 99
        with open(self.out_path, "w", encoding="utf-8") as fh:
            json.dump(ledger, fh)
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_STALE_ARTIFACT"), last)
        self.assertEqual(self._ledger()["token_types"]["A"]["carrying_non_done"], 1, "rewritten fresh")
        code, out, last = self._run("--check")
        self.assertEqual(code, 0, out)

    def test_check_fails_when_no_ledger_is_committed_yet(self):
        code, out, last = self._run("--check")
        self.assertEqual(code, 1, out)
        self.assertTrue(last.endswith("verdict=FAIL_STALE_ARTIFACT"), last)
        self.assertTrue(os.path.exists(self.out_path), "the fresh ledger is written so the fix is one commit")

    def test_a_missing_census_is_an_input_error_not_a_pass(self):
        os.remove(os.path.join(self.package, "_tokens.json"))
        code, out, last = self._run("--check")
        self.assertEqual(code, 2, out)


if __name__ == "__main__":
    unittest.main()
