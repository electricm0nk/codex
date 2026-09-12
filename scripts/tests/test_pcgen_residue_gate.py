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
               '"%CHOICE" "%LIST" "TYPE=Combat" let c = &r.raw_bonus_chains;\n'
               'use codex::pcgen_import::race_trait_tokens;\n')
        res = prg.scan(self.root)
        self.assertEqual(res.live_files, 2)
        for name in prg.PATTERNS:
            self.assertGreaterEqual(res.hits_by_pattern[name], 1, name)
        # 12 token-syntax hits + raw_bonus_chains + pcgen_import (ruling B16).
        self.assertEqual(res.hits_by_root["apps/desktop"], 14)
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


class TestCfgTestRegionsAreNotLiveCode(_TreeCase):
    """Operator ruling B15 (2026-09-12): a `#[cfg(test)]` region is NOT live
    code. It is compiled out of the shipping binary, and the corpus token text
    inside it is exactly how the rewrite is proved to survive real
    PCGen-shaped input -- the same asset reason the converter and the oracle
    harness are KEPT (`decisions.md` §11). The skip is REGION-aware, not
    line-aware: everything from the `#[cfg(test)]` attribute to the end of the
    item it annotates.

    RED->GREEN, executed rather than narrated: a token in shipping code fails;
    the same token inside a `#[cfg(test)]` module passes; a token in shipping
    code in a file that ALSO carries a `#[cfg(test)]` module still fails.
    """

    def test_shipping_token_fails_same_token_in_cfg_test_passes_and_both_fails(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        planted = "src/rules_core/planted.rs"

        # 1. the token in SHIPPING code.
        _write(self.root, planted,
               'const Q: &[&str] = &["PREFEAT:1,Dodge"];\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=2 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )

        # 2. the same token, now a fixture inside a `#[cfg(test)]` module.
        _write(self.root, planted,
               'const Q: &[&str] = &["Dodge"];\n'
               '\n'
               '#[cfg(test)]\n'
               'mod tests {\n'
               '    #[test]\n'
               '    fn verbatim_corpus_row_still_parses() {\n'
               '        let row = "PREFEAT:1,Dodge";\n'
               '        assert!(!row.is_empty());\n'
               '    }\n'
               '}\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 0, out)
        self.assertEqual(
            _last_line(out),
            "live_files=1 live_hits=1 baseline_files=1 baseline_hits=1 verdict=PASS",
        )

        # 3. a real read in shipping code in the SAME file -- the cfg(test)
        #    module must not mask it.
        _write(self.root, planted,
               'const Q: &[&str] = &["PREFEAT:1,Dodge"];\n'
               '\n'
               '#[cfg(test)]\n'
               'mod tests {\n'
               '    #[test]\n'
               '    fn verbatim_corpus_row_still_parses() {\n'
               '        let row = "PREFEAT:1,Dodge";\n'
               '        assert!(!row.is_empty());\n'
               '    }\n'
               '}\n')
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=2 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )

    def test_the_region_ends_at_the_items_closing_brace(self):
        # Code AFTER the `#[cfg(test)]` module ships, and still counts.
        _write(self.root, "src/rules_core/after.rs",
               '#[cfg(test)]\n'
               'mod tests {\n'
               '    fn f() { let _ = "BONUS:STAT|STR|2"; }\n'
               '}\n'
               '\n'
               'pub fn ships() -> &\'static str { "BONUS:STAT|STR|2" }\n')
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_pattern["BONUS:"], 1)
        self.assertEqual(sorted(res.files),
                         ["src/rules_core/after.rs", "src/rules_core/reader.rs"])

    def test_a_braceless_cfg_test_item_does_not_swallow_the_rest_of_the_file(self):
        # `#[cfg(test)] use ...;` annotates a one-line item. A brace-matcher
        # that waits for a `{` would blank the whole file after it.
        _write(self.root, "src/rules_core/braceless.rs",
               '#[cfg(test)]\n'
               'use crate::fixtures::row; // DESC:Words\n'
               '\n'
               'pub fn ships() -> &\'static str { "DESC:Words" }\n')
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_pattern["DESC:"], 1)
        self.assertIn("src/rules_core/braceless.rs", res.files)

    def test_nested_braces_inside_the_region_do_not_end_it_early(self):
        _write(self.root, "src/rules_core/nested.rs",
               '#[cfg(test)]\n'
               'mod tests {\n'
               '    fn a() { if true { let _ = "TYPE=Combat"; } }\n'
               '    fn b() { let _ = "TYPE=Combat"; }\n'
               '}\n')
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_pattern["TYPE="], 0)
        self.assertNotIn("src/rules_core/nested.rs", res.files)


class TestRuntimeConverterImportsAreCounted(_TreeCase):
    """Operator ruling B16 (2026-09-12): the gate's blind spot IS the residue.

    Live code that calls `src/pcgen_import::` at run time reads the converter
    -- the renderer, `ingest_record::token_pairs`, `lst_parser::*`,
    `ir_converter::*` -- without ever spelling a literal PCGen token, so no
    token-syntax pattern fired and no identifier pattern matched
    (`\\brender_pcgen_desc\\b` never matched `render_pcgen_desc_with_values`).
    The gate was counting code that does not ship and missing code that does.
    Naming `pcgen_import` in shipping code under a live root is a hit.
    """

    def test_a_runtime_converter_import_is_a_hit(self):
        _run(["--rebaseline", "--root", self.root, "--baseline", self.baseline])
        _write(self.root, "src/rules_core/loader.rs",
               "use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;\n"
               "pub fn f(d: &str) { let _ = crate::pcgen_import::ingest_record::token_pairs(d); }\n")
        code, out = _run(["--check", "--root", self.root, "--baseline", self.baseline])
        self.assertEqual(code, 1, out)
        self.assertEqual(
            _last_line(out),
            "live_files=2 live_hits=3 baseline_files=1 baseline_hits=1 verdict=FAIL_INCREASED",
        )
        self.assertIn("pattern pcgen_import files=1 hits=2", out)

    def test_the_desktop_crate_stops_reading_zero(self):
        _write(self.root, "apps/desktop/src-tauri/src/reach_gate.rs",
               "use codex::pcgen_import::race_trait_tokens;\n")
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_root["apps/desktop"], 1)
        self.assertEqual(res.files_by_root["apps/desktop"], 1)

    def test_a_converter_import_inside_cfg_test_is_not_a_hit(self):
        # B15 and B16 compose: the ruling is about what SHIPS.
        _write(self.root, "src/rules_core/fixture_only.rs",
               "#[cfg(test)]\n"
               "mod tests {\n"
               "    use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;\n"
               "    fn f(_: &EquipmentRecord) {}\n"
               "}\n")
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_pattern["pcgen_import"], 0)
        self.assertNotIn("src/rules_core/fixture_only.rs", res.files)

    def test_the_converter_itself_is_never_scanned(self):
        # `src/pcgen_import/**` is tool side and KEPT (`decisions.md` §11).
        _write(self.root, "src/pcgen_import/ir_converter.rs",
               "pub use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;\n")
        res = prg.scan(self.root)
        self.assertEqual(res.hits_by_pattern["pcgen_import"], 0)

    def test_the_new_pattern_is_not_folded_into_the_identifier_subset(self):
        # `identifier_files=`/`identifier_hits=` is the authoring-time "78
        # files" population; widening it silently would change what every
        # earlier receipt's figure means.
        self.assertNotIn("pcgen_import", prg.IDENTIFIER_PATTERNS)
        self.assertIn("pcgen_import", prg.PATTERNS)


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
