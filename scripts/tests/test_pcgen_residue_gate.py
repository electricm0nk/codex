"""Unit tests for `scripts/pcgen_residue_gate.py` -- `AT-35-E1-005`
(`docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md`), enforcing
`decisions.md` §11 (no PCGen on the live side; the count never rises; zero at
closure).

Every test builds its own synthetic tree under a temp dir and runs the gate's
real CLI entry point against it (`--root` / `--baseline`), so the RED->GREEN
evidence the criterion asks for -- plant one `raw_tokens` read under
`src/rules_core/`, the gate fails; remove it, the gate passes; `--closure`
fails while the baseline is above zero -- is executed, not narrated. No test
here reads the real repository: the `pcgen-residue-gate` stage in
`scripts/verify.sh` is what runs the gate on the live tree.
"""

import io
import os
import shutil
import sys
import tempfile
import unittest
from contextlib import redirect_stdout

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import pcgen_residue_gate as prg  # noqa: E402


def _write(root, rel, text):
    path = os.path.join(root, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)
    return path


def _run(argv):
    """Run the CLI, returning (exit_code, stdout_text)."""
    buf = io.StringIO()
    with redirect_stdout(buf):
        code = prg.main(argv)
    return code, buf.getvalue()


def _last_line(out):
    lines = [ln for ln in out.splitlines() if ln.strip()]
    return lines[-1] if lines else ""


class _TreeCase(unittest.TestCase):
    def setUp(self):
        self.root = tempfile.mkdtemp(prefix="pcgen-residue-")
        self.baseline = os.path.join(self.root, "scripts", "pcgen-residue-baseline.env")
        os.makedirs(os.path.dirname(self.baseline), exist_ok=True)
        # A clean live side with one real reader, plus tool-side and excluded
        # files that must NOT count.
        _write(self.root, "src/rules_core/reader.rs",
               "fn f(r: &Rec) { let t = &r.raw_tokens; let _ = t; }\n")
        _write(self.root, "src/pcgen_import/cache_gen/gen.rs",
               "// converter side, relocated by AT-35-E6-002: raw_tokens raw_tokens BONUS:\n")
        _write(self.root, "src/pcgen_import/parser.rs",
               "// tool side: raw_tokens PcgenFormulaEvaluator BONUS:STAT|STR|2\n")
        _write(self.root, "src/bin/gen_thing.rs", "// raw_tokens\n")
        _write(self.root, "tests/oracle.rs", "// raw_tokens\n")
        _write(self.root, "apps/desktop/node_modules/x/index.js", "raw_tokens BONUS:\n")
        _write(self.root, "apps/desktop/src/fixture.json", '{"raw_tokens": ["BONUS:STAT|STR|2"]}\n')

    def tearDown(self):
        shutil.rmtree(self.root, ignore_errors=True)


class TestScan(_TreeCase):
    def test_counts_only_the_live_side_source_files(self):
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 1)
        self.assertEqual(res.live_hits, 1)
        self.assertEqual(res.hits_by_pattern["raw_tokens"], 1)
        self.assertEqual(sorted(res.files), ["src/rules_core/reader.rs"])

    def test_every_design_pattern_is_counted(self):
        _write(self.root, "apps/desktop/src-tauri/src/catalog.rs",
               'let e = PcgenFormulaEvaluator::new(); render_pcgen_desc(x);\n'
               'use crate::bonus_stack_reader; use pre_tokens::parse;\n'
               '"BONUS:STAT|STR|2" "DEFINE:X|0" "PREFEAT:1,Dodge" "SAB:Text" "DESC:Words"\n'
               '"%CHOICE" "%LIST" "TYPE=Combat" let c = &r.raw_bonus_chains;\n')
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 2)
        for name in prg.PATTERNS:
            self.assertGreaterEqual(res.hits_by_pattern[name], 1, name)
        self.assertEqual(res.hits_by_root["apps/desktop"], 13)  # 12 tokens + raw_bonus_chains
        self.assertEqual(res.hits_by_root["src/rules_core"], 1)

    def test_identifier_subset_is_reported_separately(self):
        _write(self.root, "src/rules_core/tables.rs", '"PREFEAT:1,Dodge" "BONUS:STAT|STR|2"\n')
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 2)
        self.assertEqual(res.live_hits, 3)
        self.assertEqual(res.identifier_files, 1)
        self.assertEqual(res.identifier_hits, 1)


class TestCheck(_TreeCase):
    def test_check_without_a_baseline_fails_closed(self):
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 2)
        self.assertIn("verdict=FAIL_NO_BASELINE", _last_line(out))

    def test_rebaseline_records_the_first_count_then_check_passes(self):
        code, out = _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertIn("rebaseline=RECORDED", _last_line(out))
        self.assertTrue(os.path.isfile(self.baseline))
        base = prg.read_baseline(self.baseline)
        self.assertEqual((base["files"], base["hits"]), (1, 1))

        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertEqual(
            _last_line(out),
            "live_files=1 live_hits=1 baseline_files=1 baseline_hits=1 verdict=PASS",
        )

    def test_planted_raw_tokens_read_fails_and_removing_it_passes(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        planted = _write(self.root, "src/rules_core/planted.rs",
                         "fn g(r: &Rec) -> usize { r.raw_tokens.len() }\n")
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=2 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )
        os.remove(planted)
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertIn("verdict=PASS", _last_line(out))

    def test_more_hits_in_the_same_file_count_also_fails(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        _write(self.root, "src/rules_core/reader.rs",
               "fn f(r: &Rec) { let t = &r.raw_tokens; let u = &r.raw_tokens; }\n")
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertIn("live_files=1 live_hits=2", _last_line(out))
        self.assertIn("verdict=FAIL_INCREASED", _last_line(out))

    def test_check_prints_the_per_pattern_and_per_root_rows(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        _, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertIn("pattern raw_tokens files=1 hits=1", out)
        self.assertIn("pattern PcgenFormulaEvaluator files=0 hits=0", out)
        self.assertIn("root src/rules_core files=1 hits=1", out)
        self.assertIn("root apps/desktop files=0 hits=0", out)
        self.assertIn("identifier_files=1 identifier_hits=1", out)


class TestClosure(_TreeCase):
    def test_closure_fails_while_anything_is_above_zero(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        code, out = _run(["--check", "--closure", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(_last_line(out), "live_files=1 live_hits=1 verdict=FAIL")

    def test_closure_passes_at_zero_even_with_a_nonzero_baseline(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        os.remove(os.path.join(self.root, "src/rules_core/reader.rs"))
        code, out = _run(["--check", "--closure", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertEqual(_last_line(out), "live_files=0 live_hits=0 verdict=PASS")


class TestRebaseline(_TreeCase):
    def test_rebaseline_is_refused_when_the_count_did_not_go_down(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        before = open(self.baseline, encoding="utf-8").read()
        code, out = _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertIn("rebaseline=REFUSED", _last_line(out))
        self.assertIn("verdict=FAIL_NOT_REDUCED", _last_line(out))
        self.assertEqual(open(self.baseline, encoding="utf-8").read(), before)

    def test_rebaseline_is_refused_when_the_count_went_up(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        _write(self.root, "src/rules_core/planted.rs", "r.raw_tokens\n")
        code, out = _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertIn("rebaseline=REFUSED", _last_line(out))
        self.assertEqual(prg.read_baseline(self.baseline)["files"], 1)

    def test_rebaseline_is_allowed_after_a_reduction(self):
        _write(self.root, "src/rules_core/second.rs", "r.raw_tokens\n")
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(prg.read_baseline(self.baseline)["files"], 2)
        os.remove(os.path.join(self.root, "src/rules_core/second.rs"))
        code, out = _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertIn("rebaseline=RECORDED", _last_line(out))
        base = prg.read_baseline(self.baseline)
        self.assertEqual((base["files"], base["hits"]), (1, 1))


class TestCommentAwareness(_TreeCase):
    """Operator ruling B14 (2026-09-11): a doc comment that quotes an
    ingest-format token is PROVENANCE, not a read. The gate counts hits in
    executable code; it does not count hits whose line is a comment. A comment
    must never mask a real read on another line of the same file.

    RED->GREEN, executed rather than narrated: plant a token in a CODE line --
    the gate fails; move that same token into a comment -- the gate passes;
    plant a second token in code beside that comment -- the gate fails again.
    """

    def test_code_line_fails_comment_line_passes_and_code_beside_a_comment_fails_again(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        planted = "src/rules_core/planted.rs"

        # 1. the token in CODE -- a real live-side read.
        _write(self.root, planted,
               'const Q: &[&str] = &["PREFEAT:1,Dodge"];\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=2 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )

        # 2. the same token, now provenance prose beside clean code.
        _write(self.root, planted,
               '//! Prereq came from `PREFEAT:1,Dodge` in the ingest format.\n'
               'const Q: &[&str] = &["Dodge"];\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertEqual(
            _last_line(out),
            "live_files=1 live_hits=1 baseline_files=1 baseline_hits=1 verdict=PASS",
        )

        # 3. a real read on another line of the same file -- the comment must
        #    not mask it.
        _write(self.root, planted,
               '//! Prereq came from `PREFEAT:1,Dodge` in the ingest format.\n'
               'const Q: &[&str] = &["PREFEAT:1,Dodge"];\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=2 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )

    def test_every_rust_comment_marker_is_provenance(self):
        _write(self.root, "src/rules_core/provenance.rs",
               "//! module doc: BONUS:STAT|STR|2\n"
               "/// item doc: DEFINE:X|0\n"
               "    // indented line comment: PREFEAT:1,Dodge\n"
               "// plain: raw_tokens render_pcgen_desc %CHOICE %LIST TYPE=Combat DESC:Words\n")
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 1)  # only setUp's reader.rs
        self.assertEqual(res.live_hits, 1)
        self.assertEqual(res.identifier_files, 1)
        self.assertEqual(res.identifier_hits, 1)

    def test_a_trailing_comment_after_code_still_counts_the_code(self):
        # The distinction is per LINE: a line whose left-stripped form starts
        # with `//` is prose. Anything else is code, trailing comment or not --
        # counting a whole line as prose because it ends in one would let a
        # real read hide behind `// ...`.
        _write(self.root, "src/rules_core/mixed.rs",
               'let t = &r.raw_tokens; // BONUS:STAT|STR|2 is where this came from\n')
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 2)
        self.assertEqual(res.hits_by_pattern["raw_tokens"], 2)
        self.assertEqual(res.hits_by_pattern["BONUS:"], 1)


class TestLiveRootsAreTheDesignBoundary(unittest.TestCase):
    """`technical-design.md §0`'s path table, pinned so a quiet widening of
    the allow-list (`acceptance-and-verification.md §3a`) fails here."""

    def test_live_roots(self):
        self.assertEqual(
            list(prg.LIVE_ROOTS),
            ["src/rules_core", "src/saved_character", "src/campaign",
             "src/homebrew_authoring", "apps/desktop"],
        )

    def test_no_live_path_is_carved_out(self):
        # AT-35-E6-002 moved `src/rules_core/cache_gen/` to the tool side, which
        # emptied the one carve-out this gate ever had. Pinned empty so a live
        # path cannot be quietly re-exempted.
        self.assertEqual(list(prg.EXCLUDED_PREFIXES), [])


if __name__ == "__main__":
    unittest.main()
