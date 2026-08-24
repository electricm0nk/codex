"""
Tests for scripts/retro.py, the retrospective event log.

Doctrine (v0.6 alpha swarm, 2026-07-30):

  - Every hand-maintained artifact in this project drifted and then actively
    misled. This log's whole claim is that it does not, which rests on two
    properties these tests exist to hold: an event is written by the mechanism
    that observed it, and an event that would be useless is refused rather than
    stored. Both are asserted here, not assumed.
  - The log is sharded one file per actor because up to ~45 agents work
    concurrently in sibling worktrees. `test_concurrent_appends_to_one_shard`
    covers the case sharding does NOT remove -- one actor emitting from several
    processes at once -- because that is the failure mode (a half-written line)
    that would corrupt the log silently rather than loudly.
  - `test_derive_git_*` runs against a real temporary git repository with a
    real `git revert`, not a fixture of git output. The derivation exists to be
    trusted more than hand emission; a mocked test would not establish that.

Invoked via subprocess against the real CLI, matching the pattern already used
by scripts/tests/test_pcgen_normalize_output.py and
scripts/tranche/tests/test_validate_tranche_notes.py.

    python3 -m unittest discover -s scripts/tests -p 'test_retro.py'
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
RETRO = REPO_ROOT / "scripts" / "retro.py"
VERIFY = REPO_ROOT / "scripts" / "verify.sh"

# Mirrors retro.py's own pattern for a worktree name that identifies a checkout
# rather than a role. Duplicated rather than imported: retro.py is exercised as
# a CLI here, the same as every other python script test in this repo.
OPAQUE_WORKTREE_NAME = re.compile(r"^agent-[0-9a-f]{8,}$")


class RetroTestCase(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory(prefix="retro-test-")
        self.events_dir = Path(self._tmp.name) / "events"
        self.addCleanup(self._tmp.cleanup)

    def run_retro(self, *args: str, env_extra: dict | None = None, expect: int | None = 0):
        env = dict(os.environ)
        env["RETRO_EVENTS_DIR"] = str(self.events_dir)
        env.pop("RETRO_ACTOR", None)
        if env_extra:
            env.update(env_extra)
        proc = subprocess.run(
            [sys.executable, str(RETRO), *args],
            capture_output=True,
            text=True,
            env=env,
            timeout=120,
        )
        if expect is not None:
            self.assertEqual(
                proc.returncode,
                expect,
                f"args={args}\nstdout={proc.stdout}\nstderr={proc.stderr}",
            )
        return proc

    def shard(self, actor: str) -> Path:
        self.events_dir.mkdir(parents=True, exist_ok=True)
        return self.events_dir / f"{actor}.jsonl"

    def events(self) -> list[dict]:
        out: list[dict] = []
        if not self.events_dir.is_dir():
            return out
        for shard in sorted(self.events_dir.glob("*.jsonl")):
            for line in shard.read_text(encoding="utf-8").splitlines():
                if line.strip():
                    out.append(json.loads(line))
        return out


class TestEmission(RetroTestCase):
    def test_correction_writes_one_valid_event(self) -> None:
        self.run_retro(
            "correction",
            "--actor", "scout",
            "--subject", "lead",
            "--claimed", "186 feats",
            "--actual", "185 feats",
            "--verified-by", "count over the corpus",
        )
        events = self.events()
        self.assertEqual(len(events), 1)
        event = events[0]
        self.assertEqual(event["type"], "correction")
        self.assertEqual(event["actor"], "scout")
        self.assertEqual(event["subject"], "lead")
        self.assertEqual(event["origin"], "agent")
        self.assertIs(event["derived"], False)
        # The envelope is filled in by the tool, never typed by the caller:
        # that is the whole reason emission is cheap enough to actually happen.
        self.assertTrue(event["id"])
        self.assertTrue(event["ts"].endswith("Z"))
        self.assertEqual(event["source"], "cli")
        self.assertIn("branch", event["repo"])

    def test_summary_is_synthesised_when_not_given(self) -> None:
        self.run_retro(
            "correction", "--actor", "scout", "--subject", "lead",
            "--claimed", "twenty books", "--actual", "nineteen books",
            "--verified-by", "roster count",
        )
        self.assertEqual(
            self.events()[0]["summary"],
            "lead claimed twenty books; actual nineteen books",
        )

    def test_missing_required_field_is_refused_and_writes_nothing(self) -> None:
        # A correction without `verified_by` is one assertion competing with
        # another. Storing it would put an unchecked claim in the artifact that
        # exists to check claims.
        proc = self.run_retro(
            "correction", "--actor", "scout", "--subject", "lead",
            "--claimed", "x", "--actual", "y",
            expect=2,
        )
        self.assertIn("verified-by", proc.stderr)
        self.assertEqual(self.events(), [])

    def test_note_without_a_summary_is_refused(self) -> None:
        proc = self.run_retro("note", "--actor", "scout", expect=2)
        self.assertIn("summary", proc.stderr)
        self.assertEqual(self.events(), [])

    def test_unset_boolean_field_is_absent_not_false(self) -> None:
        # `"silent": false` would read as "someone checked and it was not
        # silent". Absence is the honest encoding of "nobody said".
        self.run_retro(
            "incident", "--actor", "scout",
            "--impact", "disk full", "--detected-by", "df",
        )
        self.assertNotIn("silent", self.events()[0])

    def test_boolean_field_is_a_flag(self) -> None:
        self.run_retro(
            "incident", "--actor", "scout",
            "--impact", "shared target dir", "--detected-by", "unexpected pass",
            "--silent",
        )
        self.assertIs(self.events()[0]["silent"], True)

    def test_dedupe_key_makes_emission_idempotent(self) -> None:
        for _ in range(3):
            self.run_retro(
                "rework", "--actor", "scout",
                "--what", "rebuilt the cache", "--cause", "stale artifacts",
                "--dedupe-key", "cache-rebuild-1",
            )
        self.assertEqual(len(self.events()), 1)

    def test_dry_run_writes_nothing(self) -> None:
        proc = self.run_retro(
            "deferral", "--actor", "scout",
            "--what", "Path B picker", "--reason", "not on the gate",
            "--dry-run",
        )
        self.assertEqual(json.loads(proc.stdout)["type"], "deferral")
        self.assertEqual(self.events(), [])

    def test_set_cannot_rewrite_provenance(self) -> None:
        # The log is append-only, so a line that lies about who observed
        # something can never be edited out. Write time is the only place the
        # check can live.
        for pair in ("actor=someone-else", "ts=2020-01-01T00:00:00Z", "derived=true"):
            proc = self.run_retro(
                "note", "--actor", "scout", "--summary", "provenance probe",
                "--set", pair,
                expect=2,
            )
            self.assertIn("cannot be overridden", proc.stderr)
        self.assertEqual(self.events(), [])

    def test_set_still_accepts_a_field_the_schema_does_not_name(self) -> None:
        self.run_retro(
            "note", "--actor", "scout", "--summary", "extension probe",
            "--set", "pcgen_parity_dimension=combat.base_attack_bonus",
        )
        self.assertEqual(
            self.events()[0]["pcgen_parity_dimension"], "combat.base_attack_bonus"
        )

    def test_actor_defaults_to_env(self) -> None:
        self.run_retro(
            "note", "--summary", "env actor",
            env_extra={"RETRO_ACTOR": "caster-level"},
        )
        event = self.events()[0]
        self.assertEqual(event["actor"], "caster-level")
        self.assertEqual(event["actor_source"], "env")

    def test_explicit_actor_records_how_it_was_resolved(self) -> None:
        self.run_retro("note", "--actor", "scout", "--summary", "flag actor")
        self.assertEqual(self.events()[0]["actor_source"], "flag")

    def test_worktree_fallback_is_recorded_and_warned_about(self) -> None:
        # `agent-<hex>` names a checkout, not a role. Emission still succeeds --
        # refusing would lose the event -- but a retrospective grouping by
        # actor must be able to tell this apart from a declared name, and the
        # warning arrives while the worktree still exists to be identified.
        proc = self.run_retro("note", "--summary", "worktree fallback")
        event = self.events()[0]
        self.assertEqual(event["actor_source"], "worktree")
        if OPAQUE_WORKTREE_NAME.match(event["actor"]):
            self.assertIn("RETRO_ACTOR", proc.stderr)

    def test_value_beginning_with_a_dash_is_accepted(self) -> None:
        # verify.sh passes `--mode=--only` because MODE_LABEL is literally
        # "--only". Passed as a separate argument it is read as a flag and the
        # whole emission fails with a usage error that verify.sh's `|| true`
        # swallows in silence. This is a regression test for that exact bug.
        self.run_retro(
            "verification", "--actor", "scout",
            "--mode=--only", "--result", "PASS", "--derived",
        )
        self.assertEqual(self.events()[0]["mode"], "--only")


class TestSharding(RetroTestCase):
    def test_each_actor_gets_its_own_shard(self) -> None:
        for actor in ("apg-acg-feats", "corpus-spells", "dashboard-truth"):
            self.run_retro("note", "--actor", actor, "--summary", f"{actor} was here")
        self.assertEqual(
            sorted(p.name for p in self.events_dir.glob("*.jsonl")),
            ["apg-acg-feats.jsonl", "corpus-spells.jsonl", "dashboard-truth.jsonl"],
        )

    def test_actor_name_is_slugified_into_a_safe_filename(self) -> None:
        self.run_retro("note", "--actor", "Todd Hintzmann/lead", "--summary", "slug")
        self.assertTrue(self.shard("todd-hintzmann-lead").exists())

    def test_concurrent_appends_to_one_shard_stay_whole_lines(self) -> None:
        env = dict(os.environ)
        env["RETRO_EVENTS_DIR"] = str(self.events_dir)
        procs = [
            subprocess.Popen(
                [
                    sys.executable, str(RETRO), "note",
                    "--actor", "swarm",
                    "--summary", f"concurrent emission number {n} " + "x" * 400,
                    "--quiet",
                ],
                env=env,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
                text=True,
            )
            for n in range(24)
        ]
        for proc in procs:
            _, stderr = proc.communicate(timeout=120)
            self.assertEqual(proc.returncode, 0, stderr)
        lines = [l for l in self.shard("swarm").read_text().splitlines() if l.strip()]
        self.assertEqual(len(lines), 24)
        for line in lines:
            json.loads(line)  # every line is whole; none was split by a peer


class TestValidate(RetroTestCase):
    def test_clean_log_validates(self) -> None:
        self.run_retro("note", "--actor", "scout", "--summary", "fine")
        proc = self.run_retro("validate")
        self.assertIn("all valid", proc.stdout)

    def test_unparseable_line_fails_validation(self) -> None:
        self.run_retro("note", "--actor", "scout", "--summary", "fine")
        with open(self.shard("scout"), "a", encoding="utf-8") as handle:
            handle.write("{not json\n")
        proc = self.run_retro("validate", expect=1)
        self.assertIn("unparseable", proc.stdout)

    def test_event_missing_a_required_field_fails_validation(self) -> None:
        with open(self.shard("scout"), "w", encoding="utf-8") as handle:
            handle.write(json.dumps({
                "id": "1", "ts": "2026-07-30T00:00:00Z", "type": "correction",
                "actor": "scout", "origin": "agent", "source": "cli",
                "derived": False, "summary": "no verification given",
                "subject": "lead", "claimed": "a", "actual": "b",
            }) + "\n")
        proc = self.run_retro("validate", expect=1)
        self.assertIn("verified_by", proc.stdout)


class TestSummary(RetroTestCase):
    def seed(self) -> None:
        # Four corrections of the same subject by four different actors: the
        # shape the log exists to make visible.
        for actor, claimed, actual in (
            ("apg-acg-feats", "186 feats", "185 feats"),
            ("corpus-spells", "spell totals compare like-for-like", "they do not"),
            ("dashboard-books", "twenty books remaining", "nineteen"),
            ("stale-claims", "clippy baseline 83", "66"),
        ):
            self.run_retro(
                "correction", "--actor", actor, "--subject", "lead",
                "--claimed", claimed, "--actual", actual,
                "--verified-by", "recount",
                "--blast-radius", "every brief written this session",
            )
        self.run_retro(
            "incident", "--actor", "ops", "--impact", "disk at 100%",
            "--detected-by", "df", "--recurrence-key", "disk-full",
        )
        self.run_retro(
            "incident", "--actor", "ops", "--impact", "disk at 100% again",
            "--detected-by", "livelocked resize", "--recurrence-key", "disk-full",
        )
        self.run_retro(
            "near-miss", "--actor", "verify-and-reach",
            "--would-have", "merge 34 failing tests",
            "--caught-by", "verify.sh:root-full",
        )
        self.run_retro(
            "verification", "--actor", "verify-and-reach", "--derived",
            "--mode", "full", "--result", "FAIL", "--stages-failed", "clippy,desktop",
        )
        self.run_retro(
            "verification", "--actor", "verify-and-reach", "--derived",
            "--mode", "full", "--result", "PASS",
        )

    def summary(self, *args: str) -> dict:
        proc = self.run_retro("summary", "--json", *args)
        return json.loads(proc.stdout)

    def test_repeat_subject_is_surfaced(self) -> None:
        self.seed()
        doc = self.summary()
        self.assertEqual(doc["corrections"]["total"], 4)
        self.assertEqual(doc["corrections"]["by_subject"]["lead"], 4)
        self.assertEqual(
            doc["corrections"]["repeat_subjects"],
            [{"subject": "lead", "count": 4}],
        )
        # Four different actors caught them -- the finding is the pattern, not
        # any one fix, and it is only a pattern because the correctors differ.
        self.assertEqual(len(doc["corrections"]["by_corrector"]), 4)
        self.assertEqual(doc["corrections"]["with_blast_radius"], 4)

    def test_recurring_incident_is_clustered(self) -> None:
        self.seed()
        doc = self.summary()
        self.assertEqual(
            doc["incidents"]["recurring"],
            [{"recurrence_key": "disk-full", "count": 2}],
        )

    def test_verification_rate_is_reported(self) -> None:
        self.seed()
        verification = self.summary()["verification"]
        self.assertEqual(verification["runs"], 2)
        self.assertEqual(verification["failed_runs"], 1)
        self.assertEqual(verification["fail_rate"], 0.5)
        self.assertEqual(verification["by_failing_stage"], {"clippy": 1, "desktop": 1})

    def test_origin_split_separates_derived_from_asserted(self) -> None:
        self.seed()
        origins = self.summary()["events"]["by_origin"]
        self.assertEqual(origins["derived"], 2)
        self.assertEqual(origins["agent"], 7)

    def test_window_excludes_older_events(self) -> None:
        self.seed()
        with open(self.shard("ancient"), "w", encoding="utf-8") as handle:
            handle.write(json.dumps({
                "id": "old", "ts": "2020-01-01T00:00:00Z", "type": "note",
                "actor": "ancient", "origin": "agent", "source": "cli",
                "derived": False, "summary": "long ago",
            }) + "\n")
        self.assertEqual(self.summary()["events"]["total"], 10)
        self.assertEqual(self.summary("--since", "1d")["events"]["total"], 9)

    def test_git_join_supplies_the_denominator(self) -> None:
        self.seed()
        joined = self.summary("--since", "1d")["git_join"]
        self.assertTrue(joined["available"])
        self.assertIsInstance(joined["commits"], int)

    def test_text_render_names_the_repeated_subject(self) -> None:
        self.seed()
        proc = self.run_retro("summary")
        self.assertIn("CORRECTIONS", proc.stdout)
        self.assertIn("repeated", proc.stdout)

    def test_query_filters_by_subject(self) -> None:
        self.seed()
        proc = self.run_retro("query", "--type", "correction", "--subject", "lead", "--json")
        self.assertEqual(len(json.loads(proc.stdout)), 4)


class TestDeferralResolution(RetroTestCase):
    """Regression coverage for the defect this fix closes: `deferrals.open`
    used to be `deferrals[-limit:]` -- the last N deferrals emitted, in
    emission order -- which meant it had never measured openness at all.
    `--limit 3` reported 3 'open' deferrals, `--limit 29` reported 29 of the
    same total, and a closure lane that read the default `--limit 10` as the
    bundle's whole deferral list left the other 19 of 29 real deferrals in
    the SD-32 window unchecked.
    """

    def resolve(self, actor: str, event_id: str, how: str) -> None:
        self.run_retro(
            "resolution", "--actor", actor, "--resolves", event_id, "--how", how,
        )

    def test_unresolved_deferral_counts_as_open(self) -> None:
        self.run_retro(
            "deferral", "--actor", "scout",
            "--what", "widen equipment coverage", "--reason", "not this cycle",
        )
        doc = json.loads(self.run_retro("summary", "--json").stdout)
        self.assertEqual(doc["deferrals"]["total"], 1)
        self.assertEqual(doc["deferrals"]["open"], 1)
        self.assertEqual(doc["deferrals"]["resolved"], 0)

    def test_resolved_deferral_is_not_open(self) -> None:
        proc = self.run_retro(
            "deferral", "--actor", "scout",
            "--what", "widen equipment coverage", "--reason", "not this cycle",
        )
        deferral_id = self.events()[0]["id"]
        self.resolve("closer", deferral_id, "cargo run gen_equipment; verified 0 gaps")
        doc = json.loads(self.run_retro("summary", "--json").stdout)
        self.assertEqual(doc["deferrals"]["total"], 1)
        self.assertEqual(doc["deferrals"]["open"], 0)
        self.assertEqual(doc["deferrals"]["resolved"], 1)

    def test_open_count_does_not_vary_with_limit(self) -> None:
        # 29 deferrals, none resolved -- the exact SD-32-window shape that
        # exposed the defect. `open` must read 29 regardless of --limit.
        for i in range(29):
            self.run_retro(
                "deferral", "--actor", "scout",
                "--what", f"item {i}", "--reason", "not this cycle",
            )
        for limit in (3, 10, 29):
            doc = json.loads(
                self.run_retro("summary", "--json", "--limit", str(limit)).stdout
            )
            self.assertEqual(
                doc["deferrals"]["open"], 29,
                f"open should be the true unresolved count (29) at --limit {limit}, "
                f"not a tail slice sized by --limit",
            )

    def test_open_items_list_is_not_capped_by_limit(self) -> None:
        for i in range(15):
            self.run_retro(
                "deferral", "--actor", "scout",
                "--what", f"item {i}", "--reason", "not this cycle",
            )
        doc = json.loads(self.run_retro("summary", "--json", "--limit", "3").stdout)
        self.assertEqual(len(doc["deferrals"]["open_items"]), 15)

    def test_resolution_requires_resolves_and_how(self) -> None:
        proc = self.run_retro("resolution", "--actor", "closer", expect=2)
        self.assertIn("resolves", proc.stderr)


class TestDeriveGit(RetroTestCase):
    def make_repo(self) -> Path:
        tmp = tempfile.TemporaryDirectory(prefix="retro-git-")
        self.addCleanup(tmp.cleanup)
        root = Path(tmp.name)

        def git(*args: str) -> None:
            subprocess.run(
                ["git", *args], cwd=str(root), check=True,
                capture_output=True, text=True,
            )

        git("init", "-q", "-b", "main")
        git("config", "user.email", "retro@test.invalid")
        git("config", "user.name", "Retro Test")
        (root / "a.txt").write_text("one\n")
        git("add", "a.txt")
        git("commit", "-qm", "feat: add a")
        (root / "a.txt").write_text("two\n")
        git("commit", "-qam", "feat: change a")
        git("revert", "--no-edit", "HEAD")
        return root

    def test_revert_commit_becomes_a_derived_rework_event(self) -> None:
        root = self.make_repo()
        self.run_retro("derive-git", env_extra={"RETRO_GIT_ROOT": str(root)})
        events = self.events()
        self.assertEqual(len(events), 1)
        event = events[0]
        self.assertEqual(event["type"], "rework")
        self.assertIs(event["derived"], True)
        self.assertEqual(event["origin"], "derived")
        self.assertEqual(event["source"], "git")
        self.assertTrue(event["summary"].startswith("Revert "))
        self.assertTrue(event["dedupe_key"].startswith("git-revert:"))

    def test_derivation_is_idempotent(self) -> None:
        root = self.make_repo()
        for _ in range(3):
            self.run_retro("derive-git", env_extra={"RETRO_GIT_ROOT": str(root)})
        self.assertEqual(len(self.events()), 1)

    def test_ordinary_commits_are_not_derived_into_events(self) -> None:
        # Anything short of a revert is a guess about intent, and a guessed
        # event is worth less than no event. Two ordinary commits exist in the
        # fixture repo and neither may produce a row.
        root = self.make_repo()
        commits = subprocess.run(
            ["git", "rev-list", "--count", "HEAD"], cwd=str(root),
            capture_output=True, text=True, check=True,
        ).stdout.strip()
        self.assertEqual(commits, "3")
        self.run_retro("derive-git", env_extra={"RETRO_GIT_ROOT": str(root)})
        self.assertEqual(len(self.events()), 1, "only the revert may produce an event")


class TestVerifyHook(unittest.TestCase):
    """verify.sh must emit an event and must not let emission change its result."""

    def test_verify_sh_emits_and_can_be_disabled(self) -> None:
        source = VERIFY.read_text(encoding="utf-8")
        self.assertIn("emit_retro_event", source)
        # The two properties that keep a logging failure from becoming a gate
        # failure. Point 3 of verify.sh's own header is that a false green cost
        # this repo real breakage; a false RED from a logger would be the same
        # class of mistake in the other direction.
        self.assertIn("RETRO_DISABLE", source)
        self.assertIn('python3 "${args[@]}" >/dev/null 2>&1 || true', source)
        # Emission happens after every stage has run, so it cannot preempt one.
        self.assertLess(source.index("for stage in"), source.index("emit_retro_event"))

    def test_disk_pressure_emitter_is_guarded_and_deduped(self) -> None:
        source = VERIFY.read_text(encoding="utf-8")
        self.assertIn("emit_disk_pressure_event", source)
        # Per day per filesystem: a hundred runs on a bad day are one event, so
        # the count reads as "days under pressure" and not as "times the gate
        # happened to be run".
        self.assertIn('--dedupe-key "disk-pressure:$(date -u +%Y-%m-%d):$mount"', source)
        # `df` output that cannot be parsed must produce no event rather than a
        # zero: an unparseable percentage compares as 0 in bash arithmetic and
        # would silently mean "never under pressure", which is the same shape
        # as the empty-clippy-count bug this script's own header documents.
        self.assertIn('[[ "$used_pct" =~ ^[0-9]+$ ]] || return 0', source)


class TestDiskPressureEvent(RetroTestCase):
    """The retro.py half of what verify.sh's disk check emits."""

    def test_percentage_is_stored_as_a_number(self) -> None:
        self.run_retro(
            "incident", "--actor", "verify", "--derived", "--source", "verify.sh",
            "--impact", "/ at 97% used after a verify run",
            "--detected-by", "df, at the end of scripts/verify.sh",
            "--recurrence-key", "disk-pressure",
            "--used-percent", "97",
        )
        event = self.events()[0]
        self.assertEqual(event["used_percent"], 97)
        self.assertNotIsInstance(event["used_percent"], str)
        self.assertEqual(event["origin"], "derived")


if __name__ == "__main__":
    unittest.main()
