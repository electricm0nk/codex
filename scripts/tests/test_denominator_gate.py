"""Unit tests for `scripts/denominator_gate.py` -- `AT-33-E1-004`.

Covers the pure-function line-level check (`find_violations`), path
expansion (`expand_paths`), and the CLI-level `run_check` mutation proof:
a deliberately-malformed synthetic receipt fails the check (RED), and the
corrected form passes (GREEN) -- executed directly against temp files, not
narrated, the same discipline `test_box_ledger.py`'s and
`test_probe_surface_census.py`'s mutation proofs use.
"""

import io
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import denominator_gate as dg  # noqa: E402


class TestFindViolations(unittest.TestCase):
    def test_bare_percentage_flagged(self):
        text = "Recognition rate: 97.9%.\n"
        violations = dg.find_violations(text, source="x.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["line"], 1)
        self.assertEqual(violations[0]["source"], "x.md")

    def test_percentage_with_of_denominator_passes(self):
        text = "97.9% recognised is true of the **4,798 units it ran**.\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_percentage_with_out_of_denominator_passes(self):
        text = "41 out of 11,652 units were reached at this stage (41%).\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_percentage_with_fraction_denominator_passes(self):
        text = "Coverage 8/19 kinds carry a probe (42%).\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_percentage_with_literal_denominator_word_passes(self):
        text = "Recognition rate: 97.9% (denominator: 4798).\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_no_percentage_no_violation(self):
        text = "Population is 49,438 units, partitioned into 9 groups.\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_multiple_lines_only_bad_one_flagged(self):
        text = (
            "line one: 97.9% of the 4,798 units it ran\n"
            "line two: 41% coverage\n"
            "line three: no percentage here at all\n"
        )
        violations = dg.find_violations(text, source="y.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["line"], 2)

    def test_multiple_violations_same_file(self):
        text = "first: 97.9%\nsecond: 41%\nthird: 8/19 (42%)\n"
        violations = dg.find_violations(text, source="z.md")
        self.assertEqual([v["line"] for v in violations], [1, 2])

    def test_bare_percentage_inside_fenced_code_block_is_not_flagged(self):
        text = (
            "Prose before, no percentage.\n"
            "```\n"
            "$ some command\n"
            "reports 97.9% recognised with no denominator here\n"
            "```\n"
            "Prose after, also no percentage.\n"
        )
        self.assertEqual(dg.find_violations(text), [])

    def test_bare_percentage_after_a_closed_fence_is_still_flagged(self):
        text = (
            "```\n"
            "97.9% inside the fence, skipped\n"
            "```\n"
            "97.9% outside the fence, must be flagged\n"
        )
        violations = dg.find_violations(text, source="w.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["line"], 4)

    def test_fence_marker_itself_never_flagged_even_with_percent_in_info_string(self):
        text = "```text 97.9%\ncontent\n```\n"
        self.assertEqual(dg.find_violations(text), [])

    def test_false_100_percent_idiom_space_form_not_flagged(self):
        text = (
            "are not yet examined and are **not** folded into a false 100%: "
            "5,478 (`equipment` remainder + `spell`) carry a real magnitude probe\n"
        )
        self.assertEqual(dg.find_violations(text), [])

    def test_false_100_percent_idiom_hyphen_form_not_flagged(self):
        text = (
            "marking this row `complete` on 11 of 1,741 would be the "
            "false-100% shape `decisions.md §2` and `AGENTS.md` rule 2 exist to prevent.\n"
        )
        self.assertEqual(dg.find_violations(text), [])

    def test_idiom_does_not_shadow_a_real_percentage_on_the_same_line(self):
        # The idiom must only exempt its own "100%" token -- a genuine,
        # separate percentage claim on the same line, with no denominator
        # of its own, is still a violation.
        text = "not a false 100% claim, but a real 63% figure with no denominator here\n"
        violations = dg.find_violations(text, source="idiom-shadow.md")
        self.assertEqual(len(violations), 1)
        self.assertIn("63%", violations[0]["text"])

    def test_idiom_with_its_own_denominator_still_passes(self):
        text = "## Not folded into a false 100% (of 6,589): the real 777 unexamined\n"
        self.assertEqual(dg.find_violations(text), [])


class TestExpandPaths(unittest.TestCase):
    def test_literal_existing_file(self):
        with tempfile.NamedTemporaryFile(suffix=".md", delete=False) as f:
            f.write(b"content\n")
            path = f.name
        try:
            paths, missing = dg.expand_paths([path])
            self.assertEqual(paths, [path])
            self.assertEqual(missing, [])
        finally:
            os.unlink(path)

    def test_literal_missing_file_is_hard_error(self):
        paths, missing = dg.expand_paths(["/nonexistent/path/denominator-gate-test.md"])
        self.assertEqual(paths, [])
        self.assertEqual(missing, ["/nonexistent/path/denominator-gate-test.md"])

    def test_glob_matching_nothing_is_not_an_error(self):
        with tempfile.TemporaryDirectory() as d:
            paths, missing = dg.expand_paths([os.path.join(d, "*.md")])
            self.assertEqual(paths, [])
            self.assertEqual(missing, [])

    def test_glob_matches_deduplicated_and_sorted(self):
        with tempfile.TemporaryDirectory() as d:
            a = os.path.join(d, "a.md")
            b = os.path.join(d, "b.md")
            for p in (a, b):
                with open(p, "w", encoding="utf-8") as f:
                    f.write("x\n")
            paths, missing = dg.expand_paths([os.path.join(d, "*.md"), a])
            self.assertEqual(paths, sorted([a, b]))
            self.assertEqual(missing, [])


class TestRunCheckMutationProof(unittest.TestCase):
    """The evidence obligation itself: a deliberately-malformed receipt
    fails (RED); the corrected form passes (GREEN). Run directly against a
    temp file standing in for `scripts/verify.sh --only denominator-gate`'s
    default target -- the same live invocation the stage makes, just
    pointed at a synthetic file instead of the real committed ones."""

    def setUp(self):
        fd, self.path = tempfile.mkstemp(suffix="_cycle_receipt.md")
        os.close(fd)

    def tearDown(self):
        os.unlink(self.path)

    def _write(self, text):
        with open(self.path, "w", encoding="utf-8") as f:
            f.write(text)

    def test_malformed_receipt_fails_red(self):
        self._write(
            "# Cycle FAKE — mutation-proof fixture\n\n"
            "- **Figures:** Gate 2's corpus-wide engine run reports **97.9% recognised**.\n"
        )
        out = io.StringIO()
        status = dg.run_check([self.path], out=out)
        self.assertEqual(status, 1, out.getvalue())
        self.assertIn("VIOLATION", out.getvalue())
        self.assertIn("violations=1", out.getvalue())

    def test_corrected_receipt_passes_green(self):
        self._write(
            "# Cycle FAKE — mutation-proof fixture\n\n"
            "- **Figures:** Gate 2's corpus-wide engine run reports **97.9% recognised**, "
            "true of the **4,798 units it ran** -- **41% of the 11,652** that exist.\n"
        )
        out = io.StringIO()
        status = dg.run_check([self.path], out=out)
        self.assertEqual(status, 0, out.getvalue())
        self.assertNotIn("VIOLATION", out.getvalue())
        self.assertIn("violations=0", out.getvalue())

    def test_missing_explicit_path_exits_2(self):
        out = io.StringIO()
        status = dg.run_check(["/nonexistent/denominator-gate-missing.md"], out=out)
        self.assertEqual(status, 2, out.getvalue())
        self.assertIn("MISSING_PATH", out.getvalue())

    def test_empty_glob_match_exits_1_with_no_files_matched(self):
        with tempfile.TemporaryDirectory() as d:
            out = io.StringIO()
            status = dg.run_check([os.path.join(d, "*.md")], out=out)
            self.assertEqual(status, 1, out.getvalue())
            self.assertIn("NO_FILES_MATCHED", out.getvalue())


class TestDefaultScopeIsCleanOnRealBundle(unittest.TestCase):
    """Acceptance case, run against the real committed files: this
    bundle's own cycle receipts and `progress.md` must currently pass
    clean (`decisions.md` §2 is a build obligation this cycle satisfies,
    not just a checker that exists)."""

    def test_default_globs_currently_clean(self):
        for pattern in dg.DEFAULT_GLOBS:
            self.assertTrue(
                pattern.startswith(dg.BUNDLE_DIR),
                f"default glob escapes the bundle dir: {pattern}",
            )
        out = io.StringIO()
        status = dg.run_check([], out=out)
        self.assertEqual(status, 0, out.getvalue())


if __name__ == "__main__":
    unittest.main()
