#!/usr/bin/env python3
"""Regression test for `scripts/extract_multiclass_census_panel.py`.

SD-36 Epic F, batch F0, step F0d (`docs/release/SD-36-consolidation/
epic-f-class-completion.md` §2/§5). Covers the extractor's pure parsing
helpers against synthetic Rust-shaped text (so a future edit to the real
test suites cannot silently make this regression test vacuous), plus an
end-to-end run against the real repo that cross-checks the extracted panel
against the two independently measured `cargo test -- --list` counts and
the explicit hand-written-file enumeration -- the same three-source
denominator `docs/release/SD-36-consolidation/epic-f-class-completion.md`
§5's review finding 8 calls for.
"""

import os
import subprocess
import sys
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import extract_multiclass_census_panel as EX  # noqa: E402


class ExtractPairsTests(unittest.TestCase):
    def test_dedupes_the_echoed_pair_in_order(self):
        # A `.replace(from, to)` call's `to` argument legitimately repeats
        # the `from` pair verbatim before adding the new one -- this must
        # collapse to the two DISTINCT pairs, in first-seen order, not three.
        text = "class_level=class:barbarian:12\nclass_level=class:fighter:1"
        self.assertEqual(
            EX.extract_pairs(text),
            [("barbarian", 12), ("fighter", 1)],
        )

    def test_no_match_yields_empty_list(self):
        self.assertEqual(EX.extract_pairs("no class levels here"), [])

    def test_three_distinct_pairs_all_kept(self):
        text = (
            "class_level=class:wizard:5\n"
            "class_level=class:cleric:3\n"
            "class_level=class:fighter:1"
        )
        self.assertEqual(
            EX.extract_pairs(text),
            [("wizard", 5), ("cleric", 3), ("fighter", 1)],
        )


class FunctionBodyTests(unittest.TestCase):
    def test_stops_at_the_top_level_closing_brace_not_a_nested_one(self):
        # The real defect this regression guards: a naive brace-counter
        # mis-closes on a format string's literal `{:?}`, which is not a
        # code brace. `function_body` must use the same rustfmt
        # column-0-closing-brace convention every fn in this repo's test
        # suites follows instead.
        text = (
            "fn multiclass_x_is_not_promoted_by_this_slice() {\n"
            "    let a = 1;\n"
            '    assert!(true, "{:?}", a);\n'
            "}\n"
            "\n"
            "fn unrelated_next_fn() {\n"
            "    let b = 2;\n"
            "}\n"
        )
        start = text.index("{") + 1
        body = EX.function_body(text, start)
        self.assertIn('assert!(true, "{:?}", a);', body)
        self.assertNotIn("unrelated_next_fn", body)
        self.assertTrue(body.rstrip().endswith("}"))

    def test_raises_when_no_closing_brace_exists(self):
        with self.assertRaises(EX.ExtractionError):
            EX.function_body("fn x() {\n    let a = 1;\n", 8)


class ReplaceCallShapeTests(unittest.TestCase):
    def test_matches_the_real_multiline_replace_shape(self):
        body = (
            "{\n"
            "    let multiclass = FIXTURE.replace(\n"
            '        "class_level=class:monk:1",\n'
            '        "class_level=class:monk:1\\nclass_level=class:fighter:1",\n'
            "    );\n"
            "}\n"
        )
        matches = EX.REPLACE_CALL_RE.findall(body)
        self.assertEqual(len(matches), 1)
        to_arg = matches[0][1]
        self.assertEqual(
            EX.extract_pairs(to_arg),
            [("monk", 1), ("fighter", 1)],
        )


class FormatFallbackShapeTests(unittest.TestCase):
    def test_matches_the_ranger_style_append(self):
        body = (
            "{\n"
            "    let multiclass_fixture = format!(\"{RANGER_FIXTURE}\\n"
            'class_level=class:fighter:1\\n");\n'
            "}\n"
        )
        m = EX.FORMAT_FALLBACK_RE.search(body)
        self.assertIsNotNone(m)
        self.assertEqual(m.group(1), "RANGER_FIXTURE")
        self.assertEqual(EX.extract_pairs(m.group(2)), [("fighter", 1)])


def _cargo_list_multiclass_count(test_binary: str) -> int:
    result = subprocess.run(
        ["cargo", "test", "--locked", "-j", "2", "--test", test_binary, "--", "--list"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        timeout=600,
    )
    if result.returncode != 0:
        raise RuntimeError(f"cargo test --test {test_binary} -- --list failed:\n{result.stderr}")
    return sum(1 for line in result.stdout.splitlines() if "multiclass" in line and ": test" in line)


@unittest.skipUnless(
    os.environ.get("RUN_CARGO_INTEGRATION_TESTS") == "1",
    "runs `cargo test -- --list` against the real workspace (slow); "
    "set RUN_CARGO_INTEGRATION_TESTS=1 to include it",
)
class LiveDenominatorCrossCheckTests(unittest.TestCase):
    """The expensive end-to-end check: build the real panel from the real
    repo and diff its row count against the two independently-measured
    `cargo test -- --list` counts plus the explicit hand-written-file
    enumeration -- the exact three-source sum this batch's
    BASELINE_CENSUS_MIX_COMPUTED provenance states. Gated behind an env
    var (not `--check`'s default path) because it shells out to `cargo`
    and can take real wall time; `scripts/verify.sh`'s `class-census`
    stage runs the fast `--check` path instead and trusts this test to
    have been run at least once when the panel was authored/last changed."""

    def test_panel_row_count_matches_the_measured_denominator(self):
        sd18_count = _cargo_list_multiclass_count("sd18_widening")
        sd13_count = _cargo_list_multiclass_count("sd13_progression")
        top_level_files = EX.measured_hand_written_top_level_files()
        for path in top_level_files:
            n = len(EX.FN_DEF_RE.findall(path.read_text()))
            self.assertEqual(n, 1, f"{path}: expected exactly 1 multiclass negative-control fn, found {n}")

        expected = sd18_count + sd13_count + len(top_level_files)
        rows = EX.build_panel()
        self.assertEqual(
            len(rows),
            expected,
            f"panel has {len(rows)} rows; measured denominator is "
            f"{sd18_count} (sd18_widening) + {sd13_count} (sd13_progression) + "
            f"{len(top_level_files)} (hand-written top-level files) = {expected}",
        )
        self.assertEqual(len(rows), len({r['key'] for r in rows}), "panel keys must be unique")


if __name__ == "__main__":
    unittest.main()
