"""
Tests for scripts/reclaim.sh, the disk-reclamation companion to the
CARGO_TARGET_DIR-per-agent rule.

Doctrine (2026-08-01, following the SD-27 retrospective's disk-exhaustion
finding, `docs/retro/tranche-7-retrospective.md` §4.1):

  - "The rule shipped in the brief; the matching `rm -rf` did not." A cleanup
    script that is wrong is worse than no cleanup script, so these tests
    exist to hold the safety properties, not just the happy path: a
    CARGO_TARGET_DIR a live build is using must survive; a git worktree with
    uncommitted or unpushed work must survive; a dry run must delete nothing
    at all.
  - Every safety test here was written to fail first: the guard it covers was
    temporarily disabled, the test was confirmed to fail against the broken
    script, then the guard was restored and the test confirmed to pass. See
    the tranche/8 session report for the exact break/restore transcript;
    that discipline is not re-encoded here as a runtime check because it
    would just be testing that a comment exists.
  - `reclaim.sh` derives its own `REPO_ROOT` from `${BASH_SOURCE[0]}`'s
    location (`dirname .. /..`), the same convention `scripts/verify.sh`
    uses. To exercise the git-worktree/branch categories without touching
    this actual repo's checkout, tests copy the real script into a
    throwaway `<tmp>/repo/scripts/reclaim.sh` and invoke that copy, so
    `REPO_ROOT` resolves to the disposable git repo built for the test.
  - Invoked via subprocess against the real CLI, matching the pattern already
    used by `scripts/tests/test_retro.py` and
    `scripts/tests/test_pcgen_normalize_output.py`.

    python3 -m unittest discover -s scripts/tests -p 'test_reclaim.py'
"""

from __future__ import annotations

import os
import shutil
import stat
import subprocess
import tempfile
import time
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
RECLAIM = REPO_ROOT / "scripts" / "reclaim.sh"


def _sandboxed_env(extra: dict | None = None) -> dict:
    """Every invocation of reclaim.sh in this file must run with retro
    emission disabled or redirected into a throwaway directory. reclaim.sh
    execs `python3 $SCRIPT_DIR/retro.py incident ...` on its own account
    (not the copy under a temp git fixture -- see GitFixture below, which
    copies reclaim.sh but retro.py stays at the real repo path via
    $SCRIPT_DIR), and retro.py writes to the real
    docs/retro/events/<actor>.jsonl by default. An early version of this
    test file did not do this and polluted the real log with synthetic
    `disk-full` incidents from temp-directory test fixtures -- caught by
    `git status` showing docs/retro/events/codex.jsonl modified after a test
    run, not by any assertion. RETRO_DISABLE=1 is the belt; explicit tests
    below that need to see a real emission use RETRO_EVENTS_DIR (the
    braces are the suspenders).
    """
    env = dict(os.environ)
    env["RETRO_DISABLE"] = "1"
    env.pop("RETRO_EVENTS_DIR", None)
    if extra:
        env.update(extra)
    return env


def run(args, cwd=None, env=None, timeout=60):
    if env is None:
        env = _sandboxed_env()
    return subprocess.run(
        [str(RECLAIM), *args],
        cwd=cwd,
        env=env,
        capture_output=True,
        text=True,
        timeout=timeout,
    )


def make_cargo_target_dir(root: Path, name: str, mtime_hours_ago: float = 0) -> Path:
    """A directory shaped like a real cargo CARGO_TARGET_DIR: CACHEDIR.TAG at
    the top plus a `debug/` subdirectory, matching what `is_cargo_target_dir`
    in reclaim.sh actually checks for (not just the presence of
    CACHEDIR.TAG, which fontconfig/uv/man-db also write -- see the comment
    in reclaim.sh explaining that false positive)."""
    d = root / name
    (d / "debug").mkdir(parents=True)
    (d / "CACHEDIR.TAG").write_text(
        "Signature: 8a477f597d28d172789f06886806bc55\n"
        "# This file is a cache directory tag created by cargo.\n"
    )
    (d / "debug" / "somebinary").write_bytes(b"x" * 1024)
    if mtime_hours_ago:
        old = time.time() - mtime_hours_ago * 3600
        for p in [d, d / "debug", d / "CACHEDIR.TAG", d / "debug" / "somebinary"]:
            os.utime(p, (old, old))
    return d


def make_noncargo_cachedir(root: Path, name: str) -> Path:
    """A directory that carries CACHEDIR.TAG but is NOT cargo-shaped -- the
    fontconfig/uv/man-db false-positive case."""
    d = root / name
    d.mkdir(parents=True)
    (d / "CACHEDIR.TAG").write_text(
        "Signature: 8a477f597d28d172789f06886806bc55\n"
        "# This file is a cache directory tag created by some-other-tool.\n"
    )
    (d / "payload").write_bytes(b"y" * 2048)
    return d


class CargoTargetTests(unittest.TestCase):
    """Category: cargo-target. Exercised against real (throwaway)
    directories under a temp scratchpad/cache root -- reclaim.sh's own
    SCRATCHPAD_ROOT/CACHE_ROOT are overridable via --scratchpad-root /
    --cache-root exactly so tests never need to touch the real
    /tmp/claude-1000 or ~/.cache."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(prefix="reclaim-test-")
        self.addCleanup(self._tmp.cleanup)
        self.scratch = Path(self._tmp.name) / "scratch"
        self.cache = Path(self._tmp.name) / "cache"
        self.scratch.mkdir()
        self.cache.mkdir()

    def base_args(self, *extra):
        return [
            "--only", "cargo-target",
            "--scratchpad-root", str(self.scratch),
            "--cache-root", str(self.cache),
            *extra,
        ]

    def test_dry_run_deletes_nothing(self):
        d = make_cargo_target_dir(self.scratch, "codex-target-abandoned", mtime_hours_ago=48)
        proc = run(self.base_args("--older-than", "6"))
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("WOULD REMOVE", proc.stdout)
        self.assertTrue(d.exists(), "dry run must never delete anything")

    def test_apply_removes_old_abandoned_target_dir(self):
        d = make_cargo_target_dir(self.scratch, "codex-target-abandoned", mtime_hours_ago=48)
        proc = run(self.base_args("--older-than", "6", "--apply"))
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("REMOVED", proc.stdout)
        self.assertFalse(d.exists(), "an old, unused target dir should be removed under --apply")

    def test_apply_skips_dir_younger_than_threshold(self):
        d = make_cargo_target_dir(self.scratch, "codex-target-fresh", mtime_hours_ago=0)
        proc = run(self.base_args("--older-than", "6", "--apply"))
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(d.exists(), "a freshly-touched target dir must survive the age floor")
        self.assertIn("too young", proc.stdout)

    def test_non_cargo_cachedir_tag_is_never_touched(self):
        """The real false positive this script hit on its first dry run
        against this repo's own environment: fontconfig and uv both write a
        CACHEDIR.TAG into their cache roots. A candidate lacking cargo's
        actual shape (debug/release or .rustc_info.json) must never be
        reported or removed."""
        d = make_noncargo_cachedir(self.cache, "fontconfig")
        proc = run(self.base_args("--older-than", "0", "--apply"))
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(d.exists(), "a non-cargo CACHEDIR.TAG directory must never be removed")
        self.assertNotIn(str(d), proc.stdout)

    def test_refuses_to_remove_target_dir_a_live_process_is_using(self):
        """The safety property that matters most: a build actually in
        progress must never lose its target dir out from under it. A real
        process (not a string match) is launched with CARGO_TARGET_DIR
        pointed at the candidate dir, its executable renamed to `cargo` (via
        a symlink to `sleep`) so it is indistinguishable from a real cargo
        invocation at the `ps -o comm=` level reclaim.sh actually checks."""
        d = make_cargo_target_dir(self.scratch, "codex-target-inuse", mtime_hours_ago=48)

        fake_bin_dir = Path(self._tmp.name) / "bin"
        fake_bin_dir.mkdir()
        fake_cargo = fake_bin_dir / "cargo"
        sleep_bin = shutil.which("sleep")
        self.assertIsNotNone(sleep_bin, "test requires /bin/sleep")
        os.symlink(sleep_bin, fake_cargo)

        env = dict(os.environ)
        env["CARGO_TARGET_DIR"] = str(d)
        proc_handle = subprocess.Popen([str(fake_cargo), "20"], env=env)
        try:
            # Give the kernel a moment to register argv[0]/comm before we scan.
            time.sleep(0.3)
            proc = run(self.base_args("--older-than", "6", "--apply"))
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.assertTrue(d.exists(), "a target dir a live 'cargo' process is using must survive --apply")
            self.assertIn("in use", proc.stdout)
        finally:
            proc_handle.terminate()
            proc_handle.wait(timeout=5)

        # Once the process is gone, the same dir is fair game (proves the
        # check is a real liveness check, not a permanent skip).
        proc2 = run(self.base_args("--older-than", "6", "--apply"))
        self.assertEqual(proc2.returncode, 0, proc2.stderr)
        self.assertFalse(d.exists(), "once the process exits, the dir should be reclaimable")

    def test_forbidden_path_never_touched_even_if_scanned(self):
        """Defence in depth: even if --scratchpad-root were pointed at this
        repo's own checkout by mistake, nothing inside it is ever removed."""
        d = make_cargo_target_dir(REPO_ROOT, "reclaim-test-forbidden-sentinel", mtime_hours_ago=48)
        try:
            proc = run([
                "--only", "cargo-target",
                "--scratchpad-root", str(REPO_ROOT),
                "--cache-root", str(self.cache),
                "--older-than", "6",
                "--apply",
            ])
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.assertTrue(d.exists(), "a path inside the repo checkout must never be removed")
            self.assertIn("forbidden", proc.stdout.lower())
        finally:
            shutil.rmtree(d, ignore_errors=True)


class VerifyLogsTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(prefix="reclaim-test-")
        self.addCleanup(self._tmp.cleanup)
        self.verify_tmp = Path(self._tmp.name) / "vtmp"
        self.verify_tmp.mkdir()

    def test_old_verify_log_dir_removed_under_apply(self):
        d = self.verify_tmp / "codex-verify-XYZ123"
        d.mkdir()
        (d / "root-full.log").write_text("stale log\n")
        old = time.time() - 48 * 3600
        os.utime(d / "root-full.log", (old, old))
        os.utime(d, (old, old))

        proc = run([
            "--only", "verify-logs",
            "--verify-tmp-root", str(self.verify_tmp),
            "--older-than", "6",
            "--apply",
        ])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertFalse(d.exists())

    def test_fresh_verify_log_dir_survives(self):
        d = self.verify_tmp / "codex-verify-FRESH99"
        d.mkdir()
        (d / "root-full.log").write_text("live log\n")

        proc = run([
            "--only", "verify-logs",
            "--verify-tmp-root", str(self.verify_tmp),
            "--older-than", "6",
            "--apply",
        ])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(d.exists())


class GitFixture:
    """Builds a throwaway bare 'origin' + a working checkout with a copy of
    reclaim.sh under <checkout>/scripts/, so REPO_ROOT resolves to the
    disposable checkout rather than this actual repo."""

    def __init__(self, tmp: Path):
        self.tmp = tmp
        self.bare = tmp / "origin.git"
        self.work = tmp / "work"
        self.scripts_copy = self.work / "scripts" / "reclaim.sh"

    def _git(self, *args, cwd=None, check=True):
        cwd = cwd or self.work
        proc = subprocess.run(
            ["git", *args], cwd=cwd, capture_output=True, text=True,
            env={**os.environ, "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t.example",
                 "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t.example"},
        )
        if check and proc.returncode != 0:
            raise AssertionError(f"git {args} failed: {proc.stdout}\n{proc.stderr}")
        return proc

    def build(self):
        self.bare.mkdir(parents=True)
        subprocess.run(["git", "init", "--bare", "-q", str(self.bare)], check=True)

        self.work.mkdir(parents=True)
        self._git("init", "-q", "-b", "develop")
        self._git("remote", "add", "origin", str(self.bare))
        (self.work / "README.md").write_text("root\n")
        self._git("add", "README.md")
        self._git("commit", "-q", "-m", "root commit")
        self._git("push", "-q", "-u", "origin", "develop")

        self.scripts_copy.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(RECLAIM, self.scripts_copy)
        self.scripts_copy.chmod(self.scripts_copy.stat().st_mode | stat.S_IEXEC)

    def new_merged_branch(self, name: str) -> Path:
        """A branch with one commit, merged into develop and pushed with an
        upstream, then checked out into its own worktree. Returns the
        worktree path."""
        self._git("branch", name, "develop")
        wt = self.tmp / f"wt-{name}"
        self._git("worktree", "add", "-q", str(wt), name)
        (wt / f"{name}.txt").write_text("feature work\n")
        self._git("add", f"{name}.txt", cwd=wt)
        self._git("commit", "-q", "-m", f"{name} commit", cwd=wt)
        self._git("push", "-q", "-u", "origin", name, cwd=wt)
        self._git("checkout", "-q", "develop")
        self._git("merge", "-q", "--no-edit", name)
        self._git("push", "-q", "origin", "develop")
        return wt

    def run_reclaim(self, *args, env=None):
        return subprocess.run(
            [str(self.scripts_copy), *args],
            cwd=self.work, capture_output=True, text=True, timeout=60,
            env=env if env is not None else _sandboxed_env(),
        )


class WorktreeTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(prefix="reclaim-wt-test-")
        self.addCleanup(self._tmp.cleanup)
        self.fixture = GitFixture(Path(self._tmp.name))
        self.fixture.build()

    def test_dry_run_reports_merged_worktree_but_deletes_nothing(self):
        wt = self.fixture.new_merged_branch("feature-a")
        proc = self.fixture.run_reclaim("--only", "worktrees", "--develop-ref", "develop")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("WOULD REMOVE", proc.stdout)
        self.assertTrue(wt.exists(), "dry run must not remove a worktree")

    def test_refuses_worktree_with_uncommitted_changes(self):
        wt = self.fixture.new_merged_branch("feature-b")
        (wt / "uncommitted.txt").write_text("oops, forgot to commit\n")

        proc = self.fixture.run_reclaim("--only", "worktrees", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(wt.exists(), "a worktree with uncommitted changes must survive --apply")
        self.assertIn("uncommitted", proc.stdout.lower())

        # Clean it up and confirm the SAME worktree is now eligible -- proves
        # the refusal is a real state check, not a permanent block.
        (wt / "uncommitted.txt").unlink()
        proc2 = self.fixture.run_reclaim("--only", "worktrees", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc2.returncode, 0, proc2.stderr)
        self.assertFalse(wt.exists(), "once clean, a merged worktree should be removable")

    def test_refuses_worktree_with_unpushed_commits(self):
        wt = self.fixture.new_merged_branch("feature-c")
        (wt / "extra.txt").write_text("more work\n")
        self.fixture._git("add", "extra.txt", cwd=wt)
        self.fixture._git("commit", "-q", "-m", "unpushed commit", cwd=wt)

        proc = self.fixture.run_reclaim("--only", "worktrees", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(wt.exists(), "a worktree with an unpushed commit must survive --apply")
        self.assertIn("unpushed", proc.stdout.lower())

    def test_unmerged_branch_worktree_survives(self):
        # A branch that diverges from develop and is never merged back.
        self.fixture._git("branch", "feature-unmerged", "develop")
        wt = self.fixture.tmp / "wt-feature-unmerged"
        self.fixture._git("worktree", "add", "-q", str(wt), "feature-unmerged")
        (wt / "feature-unmerged.txt").write_text("still in progress\n")
        self.fixture._git("add", "feature-unmerged.txt", cwd=wt)
        self.fixture._git("commit", "-q", "-m", "wip", cwd=wt)
        self.fixture._git("push", "-q", "-u", "origin", "feature-unmerged", cwd=wt)

        proc = self.fixture.run_reclaim("--only", "worktrees", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertTrue(wt.exists(), "an unmerged branch's worktree must never be removed")


class BranchTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(prefix="reclaim-branch-test-")
        self.addCleanup(self._tmp.cleanup)
        self.fixture = GitFixture(Path(self._tmp.name))
        self.fixture.build()

    def test_merged_branch_deleted_under_apply(self):
        self.fixture._git("branch", "old-merged", "develop")
        proc = self.fixture.run_reclaim("--only", "branches", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("DELETED", proc.stdout)
        branches = self.fixture._git("branch", "--list", "old-merged").stdout
        self.assertNotIn("old-merged", branches)

    def test_unmerged_branch_survives(self):
        self.fixture._git("checkout", "-q", "-b", "unmerged-work", "develop")
        (self.fixture.work / "wip.txt").write_text("in progress\n")
        self.fixture._git("add", "wip.txt")
        self.fixture._git("commit", "-q", "-m", "wip commit")
        self.fixture._git("checkout", "-q", "develop")

        proc = self.fixture.run_reclaim("--only", "branches", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        branches = self.fixture._git("branch", "--list", "unmerged-work").stdout
        self.assertIn("unmerged-work", branches, "an unmerged branch must never be deleted")

    def test_checked_out_branch_never_deleted_even_if_merged(self):
        # The branch backing a live worktree must never be deleted, even
        # when trivially merged (it IS develop's own history).
        wt = self.fixture.tmp / "wt-checked-out"
        self.fixture._git("worktree", "add", "-q", "-b", "checked-out-branch", str(wt), "develop")

        proc = self.fixture.run_reclaim("--only", "branches", "--develop-ref", "develop", "--apply")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        branches = self.fixture._git("branch", "--list", "checked-out-branch").stdout
        self.assertIn("checked-out-branch", branches)


class RetroEventTests(unittest.TestCase):
    """reclaim.sh must emit a retro.py `incident` event when --apply actually
    reclaims something -- and, just as important for every OTHER test in
    this file, must never do so against the real docs/retro/events/ log
    during a test run. Both properties are asserted here, into a redirected
    RETRO_EVENTS_DIR rather than RETRO_DISABLE, specifically so this test
    can look at what got written."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(prefix="reclaim-retro-test-")
        self.addCleanup(self._tmp.cleanup)
        self.scratch = Path(self._tmp.name) / "scratch"
        self.cache = Path(self._tmp.name) / "cache"
        self.events_dir = Path(self._tmp.name) / "events"
        self.scratch.mkdir()
        self.cache.mkdir()

    def test_apply_emits_incident_event_into_the_redirected_log_only(self):
        make_cargo_target_dir(self.scratch, "codex-target-abandoned", mtime_hours_ago=48)
        env = _sandboxed_env({"RETRO_EVENTS_DIR": str(self.events_dir), "RETRO_DISABLE": ""})
        proc = run(
            ["--only", "cargo-target", "--scratchpad-root", str(self.scratch),
             "--cache-root", str(self.cache), "--older-than", "6", "--apply"],
            env=env,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        shards = list(self.events_dir.glob("*.jsonl")) if self.events_dir.exists() else []
        self.assertTrue(shards, "an --apply run that reclaimed something must emit a retro event")
        content = shards[0].read_text()
        self.assertIn('"type": "incident"', content)
        self.assertIn("disk-full", content)

    def test_dry_run_never_emits_an_event(self):
        make_cargo_target_dir(self.scratch, "codex-target-abandoned", mtime_hours_ago=48)
        env = _sandboxed_env({"RETRO_EVENTS_DIR": str(self.events_dir), "RETRO_DISABLE": ""})
        proc = run(
            ["--only", "cargo-target", "--scratchpad-root", str(self.scratch),
             "--cache-root", str(self.cache), "--older-than", "6"],
            env=env,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        shards = list(self.events_dir.glob("*.jsonl")) if self.events_dir.exists() else []
        self.assertFalse(shards, "a dry run must never emit a retro event, nothing was reclaimed")


class GeneralTests(unittest.TestCase):
    def test_help_lists_every_category(self):
        proc = run(["--help"])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        for cat in ("cargo-target", "verify-logs", "worktrees", "branches"):
            self.assertIn(cat, proc.stdout)

    def test_unknown_category_is_a_usage_error(self):
        proc = run(["--only", "not-a-real-category"])
        self.assertNotEqual(proc.returncode, 0)

    def test_non_numeric_older_than_is_a_usage_error(self):
        proc = run(["--older-than", "banana"])
        self.assertNotEqual(proc.returncode, 0)

    def test_default_mode_is_dry_run(self):
        proc = run(["--only", "cargo-target", "--scratchpad-root", "/nonexistent-xyz"])
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("DRY RUN", proc.stdout)


if __name__ == "__main__":
    unittest.main()
