#!/usr/bin/env python3
"""Tests for `scripts/cycle_scope_gate.py` (SD-35 Epic 1, AT-35-E1-001).

Proves the load-bearing claim the gate exists to make (`decisions.md §2`):
a cycle scoped under the 500-unit floor **exits non-zero**, unless the scope
is the whole remaining non-DONE population. The three cases the criterion's
evidence sentence names are executed, not narrated:

  - a 12-unit scope exits 1                     (`FAIL_UNDER_FLOOR`)
  - a 500-unit scope exits 0                    (`PASS`)
  - a 12-unit scope that is the entire remainder exits 0
                                                (`PASS_WHOLE_REMAINDER`)

Uses small synthetic inventory fixtures, not the live 49,438-unit corpus,
so these tests stay fast and are not subject to corpus drift across cycles
(`test_completion_atlas.py` sets the precedent). The `--receipt` math
(id-set moved into DONE, bucket-to-bucket relabels, Rust lines from
`git diff --numstat`, compile sessions from cargo's fingerprint timestamps,
the live-side PCGen file count from `pcgen_residue_gate.py`) is proven on
fixtures and on a throwaway git repository, never on the shared checkout.
"""

import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import time
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import cycle_scope_gate as CSG  # noqa: E402

SCRIPT = os.path.join(REPO_ROOT, "scripts", "cycle_scope_gate.py")


def _unit(id_, status, evidence=None, kind="class_feature", book="core_rulebook", signals=None, tokens=None):
    u = {
        "id": id_,
        "book": book,
        "kind": kind,
        "status": status,
        "evidence": evidence,
        "wiring_class_signals": signals if signals is not None else [],
    }
    if tokens is not None:
        u["tokens"] = tokens
    return u


def _bucket_a(n, prefix="a", kind="companion", book="bestiary"):
    return [
        _unit(f"{book}:{kind}:{prefix}{i}", "engine-does-not-hold", "companion_content_has_no_engine_table", kind=kind, book=book)
        for i in range(n)
    ]


def _bucket_b(n, prefix="b", kind="class_feature", book="core_rulebook"):
    return [
        _unit(
            f"{book}:{kind}:{prefix}{i}", "engine-does-not-hold",
            "class_feature_owner_matched_by_name_but_record_not_held_by_engine",
            kind=kind, book=book, signals=["derived:bonus"],
        )
        for i in range(n)
    ]


def _done(n, prefix="d", kind="feat", book="core_rulebook"):
    return [_unit(f"{book}:{kind}:{prefix}{i}", "grounded", "feat_probe", kind=kind, book=book) for i in range(n)]


def _inventory(units):
    return {"schema_version": 1, "units": units}


def _write_inventory(dirpath, name, units):
    path = os.path.join(dirpath, name)
    with open(path, "w", encoding="utf-8") as fh:
        json.dump(_inventory(units), fh)
    return path


def _run_main(argv):
    out = io.StringIO()
    err = io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        code = CSG.main(argv)
    return code, out.getvalue(), err.getvalue()


def _last_line(text):
    lines = [ln for ln in text.splitlines() if ln.strip()]
    return lines[-1] if lines else ""


class TestBucketing(unittest.TestCase):
    def test_done_statuses_are_done(self):
        # `sheet-complete`: SD-35 AT-35-E2-003, the sheet rule's terminal status.
        for status in ("grounded", "text-complete", "oracle-agree", "oracle-unverifiable", "sheet-complete"):
            self.assertTrue(CSG.is_done(_unit("x", status)), status)

    def test_non_done_statuses_are_not_done(self):
        for status in ("engine-does-not-hold", "ingested-magnitude", "literal-verified", "unmeasurable",
                       "deferred-with-reason", "not-started"):
            self.assertFalse(CSG.is_done(_unit("x", status)), status)

    def test_unknown_status_is_unclassified_and_not_done(self):
        u = _unit("x", "some-status-nobody-declared")
        self.assertEqual(CSG.bucket_of(u), CSG.UNCLASSIFIED)
        self.assertFalse(CSG.is_done(u))

    def test_bucket_delegates_to_the_atlas_partition(self):
        self.assertEqual(CSG.bucket_of(_bucket_a(1)[0]), "A")
        self.assertEqual(CSG.bucket_of(_bucket_b(1)[0]), "B")
        self.assertEqual(CSG.bucket_of(_unit("x", "ingested-magnitude")), "M")
        self.assertEqual(CSG.bucket_of(_unit("x", "not-started")), "Z")


class TestScopeParsing(unittest.TestCase):
    def test_no_filters_is_one_empty_clause(self):
        clauses = CSG.parse_clauses([])
        self.assertEqual(len(clauses), 1)
        self.assertTrue(CSG.clause_is_empty(clauses[0]))

    def test_same_flag_repeats_accumulate(self):
        clauses = CSG.parse_clauses(["--bucket", "A", "--bucket", "X", "--kind", "companion"])
        self.assertEqual(len(clauses), 1)
        self.assertEqual(clauses[0]["bucket"], {"A", "X"})
        self.assertEqual(clauses[0]["kind"], {"companion"})

    def test_or_splits_clauses(self):
        clauses = CSG.parse_clauses(["--bucket", "A", "--kind", "companion", "--or", "--bucket", "X"])
        self.assertEqual(len(clauses), 2)
        self.assertEqual(clauses[0]["kind"], {"companion"})
        self.assertEqual(clauses[1]["bucket"], {"X"})
        self.assertEqual(clauses[1]["kind"], set())

    def test_unknown_flag_is_rejected(self):
        with self.assertRaises(CSG.ScopeError):
            CSG.parse_clauses(["--colour", "red"])

    def test_flag_without_value_is_rejected(self):
        with self.assertRaises(CSG.ScopeError):
            CSG.parse_clauses(["--bucket"])


class TestScopeMatching(unittest.TestCase):
    def test_within_a_clause_flags_and_together(self):
        units = _bucket_a(3, kind="companion") + _bucket_a(2, prefix="p", kind="power")
        scoped = CSG.scoped_units(units, CSG.parse_clauses(["--bucket", "A", "--kind", "companion"]))
        self.assertEqual(len(scoped), 3)

    def test_clauses_union(self):
        units = _bucket_a(3) + _bucket_b(4) + [_unit("z", "not-started", kind="spell")]
        scoped = CSG.scoped_units(
            units, CSG.parse_clauses(["--bucket", "A", "--kind", "companion", "--or", "--bucket", "Z"])
        )
        self.assertEqual(len(scoped), 4)

    def test_done_units_never_count_as_scoped(self):
        units = _done(50, kind="companion", book="bestiary") + _bucket_a(3)
        scoped = CSG.scoped_units(units, CSG.parse_clauses(["--kind", "companion"]))
        self.assertEqual(len(scoped), 3)

    def test_evidence_prefix(self):
        units = _bucket_a(2) + _bucket_b(2)
        scoped = CSG.scoped_units(units, CSG.parse_clauses(["--evidence-prefix", "class_feature_owner"]))
        self.assertEqual({u["id"] for u in scoped}, {u["id"] for u in _bucket_b(2)})

    def test_book(self):
        units = _bucket_a(2, book="bestiary") + _bucket_a(5, prefix="q", book="bestiary_2")
        scoped = CSG.scoped_units(units, CSG.parse_clauses(["--book", "bestiary_2"]))
        self.assertEqual(len(scoped), 5)

    def test_token_matches_tokens_field_when_present(self):
        units = [
            _unit("t1", "engine-does-not-hold", "x", tokens=["BONUS:"]),
            _unit("t2", "engine-does-not-hold", "x", tokens=["DEFINE:"]),
        ]
        scoped = CSG.scoped_units(units, CSG.parse_clauses(["--token", "BONUS:"]))
        self.assertEqual([u["id"] for u in scoped], ["t1"])

    def test_token_matches_wiring_class_signal_family(self):
        units = _bucket_b(3) + _bucket_a(2)  # B carries derived:bonus, A carries no signal
        self.assertEqual(len(CSG.scoped_units(units, CSG.parse_clauses(["--token", "bonus"]))), 3)
        self.assertEqual(len(CSG.scoped_units(units, CSG.parse_clauses(["--token", "derived:bonus"]))), 3)
        self.assertEqual(len(CSG.scoped_units(units, CSG.parse_clauses(["--token", "spells"]))), 0)

    def test_unclassified_bucket_is_selectable_and_counts_as_remaining(self):
        units = [_unit("u", "never-heard-of-it")] + _done(1)
        self.assertEqual(len(CSG.scoped_units(units, CSG.parse_clauses(["--bucket", CSG.UNCLASSIFIED]))), 1)
        self.assertEqual(len(CSG.remaining_non_done(units)), 1)


class TestVerdict(unittest.TestCase):
    def test_under_floor_and_not_remainder_fails(self):
        self.assertEqual(CSG.verdict(12, 612, 500), ("FAIL_UNDER_FLOOR", 1))

    def test_at_floor_passes(self):
        self.assertEqual(CSG.verdict(500, 612, 500), ("PASS", 0))

    def test_above_floor_passes(self):
        self.assertEqual(CSG.verdict(7866, 23315, 500), ("PASS", 0))

    def test_whole_remainder_passes_under_floor(self):
        self.assertEqual(CSG.verdict(12, 12, 500), ("PASS_WHOLE_REMAINDER", 0))

    def test_nothing_left_is_whole_remainder(self):
        self.assertEqual(CSG.verdict(0, 0, 500), ("PASS_WHOLE_REMAINDER", 0))

    def test_zero_scoped_with_work_left_fails(self):
        self.assertEqual(CSG.verdict(0, 40, 500), ("FAIL_UNDER_FLOOR", 1))


class TestFloorCli(unittest.TestCase):
    """The three cases `epic-breakdown.md` AT-35-E1-001 names, through `main()`."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)

    def test_twelve_unit_scope_exits_one(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(12) + _bucket_b(600) + _done(100))
        code, out, _ = _run_main(["--min", "500", "--inventory", inv, "--bucket", "A", "--kind", "companion"])
        self.assertEqual(code, 1)
        self.assertEqual(_last_line(out), "scoped=12 remaining_non_done=612 floor=500 verdict=FAIL_UNDER_FLOOR")

    def test_five_hundred_unit_scope_exits_zero(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(500) + _bucket_b(600) + _done(100))
        code, out, _ = _run_main(["--min", "500", "--inventory", inv, "--bucket", "A", "--kind", "companion"])
        self.assertEqual(code, 0)
        self.assertEqual(_last_line(out), "scoped=500 remaining_non_done=1100 floor=500 verdict=PASS")

    def test_twelve_unit_scope_that_is_the_entire_remainder_exits_zero(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(12) + _done(100))
        code, out, _ = _run_main(["--min", "500", "--inventory", inv, "--bucket", "A", "--kind", "companion"])
        self.assertEqual(code, 0)
        self.assertEqual(_last_line(out), "scoped=12 remaining_non_done=12 floor=500 verdict=PASS_WHOLE_REMAINDER")

    def test_no_filters_means_the_whole_remainder(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(12) + _bucket_b(30) + _done(100))
        code, out, _ = _run_main(["--min", "500", "--inventory", inv])
        self.assertEqual(code, 0)
        self.assertEqual(_last_line(out), "scoped=42 remaining_non_done=42 floor=500 verdict=PASS_WHOLE_REMAINDER")

    def test_bundling_mechanisms_with_or_reaches_the_floor(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(300) + _bucket_b(250) + _bucket_b(900, prefix="other", kind="feat"))
        under, out, _ = _run_main(["--min", "500", "--inventory", inv, "--bucket", "A"])
        self.assertEqual(under, 1)
        self.assertEqual(_last_line(out), "scoped=300 remaining_non_done=1450 floor=500 verdict=FAIL_UNDER_FLOOR")
        bundled, out, _ = _run_main(
            ["--min", "500", "--inventory", inv, "--bucket", "A", "--or", "--bucket", "B", "--kind", "class_feature"]
        )
        self.assertEqual(bundled, 0)
        self.assertEqual(_last_line(out), "scoped=550 remaining_non_done=1450 floor=500 verdict=PASS")

    def test_breakdown_lines_precede_the_verdict(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(2) + _bucket_b(3))
        _, out, _ = _run_main(["--min", "500", "--inventory", inv])
        self.assertIn("scoped_by_bucket=A:2 B:3", out)
        self.assertIn("scoped_by_kind=class_feature:3 companion:2", out)

    def test_missing_inventory_exits_two(self):
        code, _, err = _run_main(["--min", "500", "--inventory", os.path.join(self.tmp.name, "nope.json")])
        self.assertEqual(code, 2)
        self.assertIn("nope.json", err)

    def test_bad_scope_flag_exits_two(self):
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(2))
        code, _, err = _run_main(["--min", "500", "--inventory", inv, "--colour", "red"])
        self.assertEqual(code, 2)
        self.assertIn("--colour", err)

    def test_process_exit_code_is_nonzero_under_floor(self):
        """The criterion's title: a script with a nonzero exit -- proven at
        the process boundary, not only through `main()`."""
        inv = _write_inventory(self.tmp.name, "inv.json", _bucket_a(12) + _bucket_b(600))
        proc = subprocess.run(
            [sys.executable, SCRIPT, "--min", "500", "--inventory", inv, "--bucket", "A"],
            capture_output=True, text=True, cwd=REPO_ROOT,
        )
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertTrue(proc.stdout.rstrip().endswith("verdict=FAIL_UNDER_FLOOR"), proc.stdout)


class TestMovement(unittest.TestCase):
    def test_closed_is_the_id_set_moved_into_done(self):
        before = _bucket_a(3) + _bucket_b(2)
        after = [dict(u, status="text-complete", evidence="sheet_rule_rendered:words") for u in _bucket_a(3)] + _bucket_b(2)
        mv = CSG.movement(before, after)
        self.assertEqual(mv["closed"], 3)
        self.assertEqual(sorted(mv["closed_ids"]), sorted(u["id"] for u in _bucket_a(3)))
        self.assertEqual(mv["relabeled"], 0)

    def test_bucket_to_bucket_is_a_relabel_not_a_closure(self):
        before = _bucket_a(2)
        after = [dict(u, status="ingested-magnitude") for u in _bucket_a(2)]
        mv = CSG.movement(before, after)
        self.assertEqual(mv["closed"], 0)
        self.assertEqual(mv["relabeled"], 2)

    def test_already_done_is_not_closed_again(self):
        before = _done(4)
        after = [dict(u, status="text-complete") for u in _done(4)]
        mv = CSG.movement(before, after)
        self.assertEqual(mv["closed"], 0)
        self.assertEqual(mv["relabeled"], 0)

    def test_done_to_not_done_is_a_regression(self):
        before = _done(2)
        after = [dict(u, status="not-started") for u in _done(2)]
        mv = CSG.movement(before, after)
        self.assertEqual(mv["regressed"], 2)
        self.assertEqual(mv["closed"], 0)

    def test_added_and_dropped_ids_are_counted_not_closed(self):
        before = _bucket_a(2)
        after = _done(5, prefix="new")
        mv = CSG.movement(before, after)
        self.assertEqual(mv["closed"], 0)
        self.assertEqual(mv["added"], 5)
        self.assertEqual(mv["dropped"], 2)


class TestRustLines(unittest.TestCase):
    def test_numstat_sums_added_and_deleted(self):
        text = "12\t3\tsrc/rules_core/sheet_rule.rs\n0\t7\tsrc/bin/x.rs\n-\t-\tdata/blob.bin\n"
        self.assertEqual(CSG.rust_lines_from_numstat(text), 22)

    def test_empty_numstat_is_zero(self):
        self.assertEqual(CSG.rust_lines_from_numstat(""), 0)

    def test_counts_only_rs_files_in_a_real_repo(self):
        with tempfile.TemporaryDirectory() as d:
            env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@x", GIT_COMMITTER_NAME="t",
                       GIT_COMMITTER_EMAIL="t@x", HOME=d)

            def git(*args):
                return subprocess.run(["git", *args], cwd=d, env=env, capture_output=True, text=True, check=True).stdout

            git("init", "-q")
            with open(os.path.join(d, "a.rs"), "w") as fh:
                fh.write("fn a() {}\n")
            with open(os.path.join(d, "n.md"), "w") as fh:
                fh.write("prose\n")
            git("add", "a.rs", "n.md")
            git("commit", "-q", "-m", "base")
            base = git("rev-parse", "HEAD").strip()
            with open(os.path.join(d, "a.rs"), "a") as fh:
                fh.write("fn b() {}\nfn c() {}\n")
            with open(os.path.join(d, "n.md"), "a") as fh:
                fh.write("more prose\nand more\nand more\n")
            git("commit", "-q", "-am", "change")
            with open(os.path.join(d, "a.rs"), "a") as fh:
                fh.write("fn d() {}\n")  # uncommitted -- still this cycle's work
            self.assertEqual(CSG.rust_lines_changed(base, repo=d), 3)
            self.assertEqual(CSG.rust_lines_changed("HEAD", repo=d), 1)


class TestBuildSessions(unittest.TestCase):
    def _stamp(self, root, crate, when):
        p = os.path.join(root, "debug", ".fingerprint", crate)
        os.makedirs(p, exist_ok=True)
        f = os.path.join(p, "invoked.timestamp")
        with open(f, "w") as fh:
            fh.write("")
        os.utime(f, (when, when))

    def test_clusters_fingerprint_timestamps_into_sessions(self):
        now = time.time()
        with tempfile.TemporaryDirectory() as d:
            for i in range(20):
                self._stamp(d, f"crate{i}-aaaa", now - 7200 + i * 3)      # one 1-minute session two hours ago
            for i in range(5):
                self._stamp(d, f"crate{i}-bbbb", now - 600 + i * 2)       # a second session ten minutes ago
            self.assertEqual(CSG.build_sessions(d), 2)

    def test_since_excludes_older_sessions(self):
        now = time.time()
        with tempfile.TemporaryDirectory() as d:
            self._stamp(d, "old-aaaa", now - 7200)
            self._stamp(d, "new-bbbb", now - 60)
            self.assertEqual(CSG.build_sessions(d, since_epoch=now - 3600), 1)

    def test_empty_or_missing_target_dir_is_zero(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(CSG.build_sessions(d), 0)
            self.assertEqual(CSG.build_sessions(os.path.join(d, "absent")), 0)


class TestResidueGateProbe(unittest.TestCase):
    def test_reads_live_files_from_the_gate_output(self):
        with tempfile.TemporaryDirectory() as d:
            gate = os.path.join(d, "gate.py")
            with open(gate, "w") as fh:
                fh.write("print('live_files=78 live_hits=412 baseline_files=78 baseline_hits=412 verdict=PASS')\n")
            self.assertEqual(CSG.pcgen_live_files(gate), "78")

    def test_reads_the_count_even_when_the_gate_fails(self):
        with tempfile.TemporaryDirectory() as d:
            gate = os.path.join(d, "gate.py")
            with open(gate, "w") as fh:
                fh.write("import sys\nprint('live_files=79 live_hits=413 baseline_files=78 baseline_hits=412 verdict=FAIL_INCREASED')\nsys.exit(1)\n")
            self.assertEqual(CSG.pcgen_live_files(gate), "79")

    def test_missing_gate_is_unavailable_never_zero(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(CSG.pcgen_live_files(os.path.join(d, "absent.py")), "unavailable")


class TestReceiptCli(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.before = _write_inventory(self.tmp.name, "before.json", _bucket_a(3) + _bucket_b(4) + _done(2))
        after_units = (
            [dict(u, status="text-complete", evidence="sheet_rule_rendered:words") for u in _bucket_a(3)]
            + [dict(_bucket_b(4)[0], status="ingested-magnitude")]
            + _bucket_b(4)[1:]
            + _done(2)
        )
        self.after = _write_inventory(self.tmp.name, "after.json", after_units)
        self.gate = os.path.join(self.tmp.name, "gate.py")
        with open(self.gate, "w") as fh:
            fh.write("print('live_files=78 live_hits=412 baseline_files=78 baseline_hits=412 verdict=PASS')\n")
        self.target = os.path.join(self.tmp.name, "target")
        os.makedirs(os.path.join(self.target, "debug", ".fingerprint", "codex-abcd"))
        with open(os.path.join(self.target, "debug", ".fingerprint", "codex-abcd", "invoked.timestamp"), "w") as fh:
            fh.write("")

    def _receipt(self, extra):
        return _run_main(
            ["--receipt", "--before", self.before, "--after", self.after, "--residue-gate", self.gate,
             "--target-dir", self.target, "--since", "HEAD", "--repo", REPO_ROOT] + extra
        )

    def test_receipt_line_is_the_schema_verbatim(self):
        code, out, _ = self._receipt([])
        self.assertEqual(code, 0)
        line = _last_line(out)
        self.assertRegex(
            line,
            r"^closed=3 relabeled=1 rust_lines_changed=\d+ ratio=\d+\.\d\d builds_recorded=1 pcgen_live_files=78$",
        )

    def test_ratio_is_not_a_number_when_nothing_closed(self):
        same = _write_inventory(self.tmp.name, "same.json", _bucket_a(3))
        code, out, _ = _run_main(
            ["--receipt", "--before", same, "--after", same, "--residue-gate", self.gate,
             "--target-dir", self.target, "--since", "HEAD", "--repo", REPO_ROOT]
        )
        self.assertEqual(code, 0)
        self.assertRegex(_last_line(out), r"^closed=0 relabeled=0 rust_lines_changed=\d+ ratio=n/a builds_recorded=1 pcgen_live_files=78$")

    def test_receipt_names_the_movement_detail_above_the_line(self):
        _, out, _ = self._receipt([])
        self.assertIn("closed_by_kind=companion:3", out)
        self.assertIn("relabeled_moves=B->M:1", out)
        self.assertIn("regressed=0", out)

    def test_receipt_requires_before_and_after(self):
        code, _, err = _run_main(["--receipt", "--before", self.before])
        self.assertEqual(code, 2)
        self.assertIn("--after", err)


class TestModeSelection(unittest.TestCase):
    def test_no_mode_is_a_usage_error(self):
        code, _, err = _run_main([])
        self.assertEqual(code, 2)
        self.assertIn("--min", err)
        self.assertIn("--receipt", err)


if __name__ == "__main__":
    unittest.main()
