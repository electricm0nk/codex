"""Unit tests for `scripts/denominator_gate.py` -- `AT-33-E1-004`.

Covers the pure-function line-level check (`find_violations`), path
expansion (`expand_paths`), and the CLI-level `run_check` mutation proof:
a deliberately-malformed synthetic receipt fails the check (RED), and the
corrected form passes (GREEN) -- executed directly against temp files, not
narrated, the same discipline `test_box_ledger.py`'s and
`test_probe_surface_census.py`'s mutation proofs use.
"""

import glob
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
        # AT-34-E1-006 widened DEFAULT_GLOBS to cover both SD-33's folder
        # (unchanged) and SD-34's own package -- every entry must start
        # with one of the two, not just the original.
        for pattern in dg.DEFAULT_GLOBS:
            self.assertTrue(
                pattern.startswith(dg.BUNDLE_DIR)
                or pattern.startswith(dg.SD34_BUNDLE_DIR),
                f"default glob escapes both known bundle dirs: {pattern}",
            )
        out = io.StringIO()
        status = dg.run_check([], out=out)
        self.assertEqual(status, 0, out.getvalue())


class TestDefaultGlobsCoverHeadlinePackageDocs(unittest.TestCase):
    """`AT-33-E1-004` scope-widening remediation (wave 3): attempt 3's
    final-acceptance scan found the bundle root was never scanned --
    `files_checked` stayed at the receipts+progress.md count because
    `DEFAULT_GLOBS` covered only those two paths. Pins the fix so a future
    edit cannot silently narrow the scope back to the pre-remediation set
    -- the exact failure mode `AT-33-E1-002`'s own gate exists to catch,
    applied here to this gate's own configuration."""

    EXPECTED_HEADLINE_DOCS = (
        "README.md",
        "decisions.md",
        "epic-breakdown.md",
        "release-notes.md",
        "scope-draft.md",
        "kanban.md",
        "THE-BOX.md",
    )

    def test_default_globs_include_every_headline_package_doc(self):
        expected = {
            os.path.join(dg.BUNDLE_DIR, name)
            for name in self.EXPECTED_HEADLINE_DOCS
        }
        missing = expected - set(dg.DEFAULT_GLOBS)
        self.assertEqual(
            missing, set(),
            f"headline package doc(s) dropped from DEFAULT_GLOBS: {missing}",
        )

    def test_headline_docs_are_real_files_the_gate_actually_reads(self):
        # Not just present in the pattern list -- actually resolved and
        # counted by expand_paths, the same function run_check uses.
        paths, missing = dg.expand_paths(list(dg.DEFAULT_GLOBS))
        self.assertEqual(missing, [])
        resolved = set(paths)
        for name in self.EXPECTED_HEADLINE_DOCS:
            full = os.path.join(dg.BUNDLE_DIR, name)
            self.assertIn(full, resolved, f"{name} not in the resolved file set")


class TestDefaultGlobsWidenedToSD34(unittest.TestCase):
    """`AT-34-E1-006`'s second obligation: widen `DEFAULT_GLOBS` from
    SD-33's folder alone to also cover this package, so a default
    (no-explicit-path) `--check` run examines every SD-34 `.md` --
    RED before this cycle (a default run saw zero SD-34 files), GREEN
    after (`decisions.md §3`)."""

    def test_sd34_bundle_dir_is_the_real_package_folder(self):
        self.assertTrue(os.path.isdir(dg.SD34_BUNDLE_DIR))
        self.assertTrue(dg.SD34_BUNDLE_DIR.endswith("SD-34-book-completion"))

    def test_default_run_includes_every_sd34_root_md_file(self):
        # The literal population named by AT-34-E1-006: every `.md` at
        # this package's root, resolved the same way `run_check` resolves
        # any other pattern.
        real_sd34_md = {
            p for p in glob.glob(os.path.join(dg.SD34_BUNDLE_DIR, "*.md"))
        }
        self.assertGreater(len(real_sd34_md), 0, "no SD-34 .md files found on disk")
        paths, missing = dg.expand_paths(list(dg.DEFAULT_GLOBS))
        self.assertEqual(missing, [])
        resolved = set(paths)
        not_covered = real_sd34_md - resolved
        self.assertEqual(
            not_covered, set(),
            f"SD-34 root .md file(s) not covered by the widened default: {not_covered}",
        )

    def test_default_run_files_checked_covers_sd34(self):
        out = io.StringIO()
        status = dg.run_check([], out=out)
        checked = int(
            [
                line for line in out.getvalue().splitlines()
                if line.startswith("files_checked=")
            ][0].split("=")[1]
        )
        real_sd34_md_count = len(
            glob.glob(os.path.join(dg.SD34_BUNDLE_DIR, "*.md"))
        )
        self.assertGreaterEqual(checked, real_sd34_md_count)
        self.assertEqual(status, 0, out.getvalue())


class TestFigureProvenanceGate(unittest.TestCase):
    """`AT-34-E1-006`'s first obligation: a figure with no re-derive
    command reachable from it fails; the sourced form, and a form whose
    command names a real script, both pass. Scoped to a receipt's
    "Figures + their re-derive commands" section -- see the module-level
    comment above `FIGURES_SECTION_START_RE` for why the rest of a
    receipt (its Acceptance-criterion quote, Notes, Next-cycle plan) is
    out of this check's scope."""

    FIGURES_HEADER = "- **Figures + their re-derive commands:**\n"
    NEXT_FIELD = "- **Row-count command output:**\n  (n/a for this fixture)\n"

    def test_unsourced_figure_is_a_violation_red(self):
        text = (
            self.FIGURES_HEADER
            + "  - The corpus holds **49,438** units across 37 books.\n"
            + self.NEXT_FIELD
        )
        violations = dg.find_provenance_violations(text, source="fixture.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["reason"], "unsourced")
        self.assertEqual(violations[0]["line"], 2)

    def test_sourced_figure_passes_green(self):
        text = (
            self.FIGURES_HEADER
            + "  - The corpus holds **49,438** units -- "
              "`python3 scripts/completion_atlas.py --check`\n"
            + self.NEXT_FIELD
        )
        self.assertEqual(dg.find_provenance_violations(text, source="fixture.md"), [])

    def test_wrong_command_figure_is_a_violation(self):
        # A command naming a script that does not exist in this tree --
        # it cannot possibly have produced the figure it sits beside.
        text = (
            self.FIGURES_HEADER
            + "  - The corpus holds **49,438** units -- "
              "`python3 scripts/does_not_exist_anywhere.py --check`\n"
            + self.NEXT_FIELD
        )
        violations = dg.find_provenance_violations(text, source="fixture.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["reason"], "unresolvable")
        self.assertEqual(violations[0]["bad_path"], "scripts/does_not_exist_anywhere.py")

    def test_command_naming_a_real_script_passes(self):
        text = (
            self.FIGURES_HEADER
            + "  - The corpus holds **49,438** units -- "
              "`python3 scripts/denominator_gate.py --check`\n"
            + self.NEXT_FIELD
        )
        self.assertEqual(dg.find_provenance_violations(text, source="fixture.md"), [])

    def test_percentage_figure_also_covered(self):
        text = (
            self.FIGURES_HEADER
            + "  - Recognition rate: 97.9%.\n"
            + self.NEXT_FIELD
        )
        violations = dg.find_provenance_violations(text, source="fixture.md")
        self.assertEqual(len(violations), 1)
        self.assertEqual(violations[0]["reason"], "unsourced")

    def test_figures_outside_the_figures_section_are_out_of_scope(self):
        # A receipt's Notes/Acceptance-criterion prose referencing an
        # already-sourced figure in passing is not re-flagged -- only the
        # dedicated Figures section is in scope (see class docstring).
        text = (
            "- **Acceptance criterion:** \"...covers **49,438** units...\"\n"
            + self.FIGURES_HEADER
            + "  - `population=8463` -- `python3 scripts/missing_engine_tables.py --check`\n"
            + "- **Notes:**\n"
            + "  - As stated above, **49,438** units were examined.\n"
        )
        self.assertEqual(dg.find_provenance_violations(text, source="fixture.md"), [])

    def test_file_with_no_figures_section_produces_no_violations(self):
        text = "Just some prose mentioning **49,438** units, no receipt structure.\n"
        self.assertEqual(dg.find_provenance_violations(text, source="fixture.md"), [])

    def test_run_provenance_check_cli_mutation_proof(self):
        fd, path = tempfile.mkstemp(suffix="_cycle_receipt.md")
        os.close(fd)
        try:
            with open(path, "w", encoding="utf-8") as f:
                f.write(
                    self.FIGURES_HEADER
                    + "  - The corpus holds **49,438** units, no command here.\n"
                    + self.NEXT_FIELD
                )
            out = io.StringIO()
            status = dg.run_provenance_check([path], out=out)
            self.assertEqual(status, 1, out.getvalue())
            self.assertIn("VIOLATION", out.getvalue())
            self.assertIn("figures_examined=1", out.getvalue())

            with open(path, "w", encoding="utf-8") as f:
                f.write(
                    self.FIGURES_HEADER
                    + "  - The corpus holds **49,438** units -- "
                      "`python3 scripts/completion_atlas.py --check`\n"
                    + self.NEXT_FIELD
                )
            out = io.StringIO()
            status = dg.run_provenance_check([path], out=out)
            self.assertEqual(status, 0, out.getvalue())
            self.assertNotIn("VIOLATION", out.getvalue())
            self.assertIn("figures_examined=1", out.getvalue())
            self.assertIn("violations=0", out.getvalue())
        finally:
            os.unlink(path)

    def test_missing_explicit_path_exits_2(self):
        out = io.StringIO()
        status = dg.run_provenance_check(
            ["/nonexistent/figure-provenance-missing.md"], out=out
        )
        self.assertEqual(status, 2, out.getvalue())
        self.assertIn("MISSING_PATH", out.getvalue())


class TestFigureProvenanceDefaultScope(unittest.TestCase):
    """The real, committed SD-34 package (receipts + root docs) passes
    the provenance gate clean today -- the mechanism is proven wired, not
    just unit-tested in isolation. Deliberately excludes SD-33's folder
    (`PROVENANCE_DEFAULT_GLOBS` docstring): this bundle may not write
    there, so the gate this cycle owns cannot default to a scope only a
    different, forbidden cycle could ever turn green."""

    def test_provenance_default_globs_are_sd34_only(self):
        for pattern in dg.PROVENANCE_DEFAULT_GLOBS:
            self.assertTrue(pattern.startswith(dg.SD34_BUNDLE_DIR))

    def test_provenance_default_run_is_clean(self):
        out = io.StringIO()
        status = dg.run_provenance_check([], out=out)
        self.assertEqual(status, 0, out.getvalue())
        self.assertIn("figures_examined=", out.getvalue())
        checked_line = [
            line for line in out.getvalue().splitlines()
            if line.startswith("figures_examined=")
        ][0]
        examined = int(checked_line.split("=")[1])
        self.assertGreater(examined, 0, "vacuous pass -- zero figures examined")


if __name__ == "__main__":
    unittest.main()
