#!/usr/bin/env python3
"""
Tests for tools/release/check_release_manifest.py.

These tests pin the SD-16 contract for the release manifest gate at PR time.

Each test invokes the validator as a subprocess (NOT by importing it) so the
exact on-disk artefact is exercised. A bona-fide `jsonschema` install is
required only for the negative-schema test; the others exercise only the
path-resolution and coercion logic that the validator itself owns.

Doctrine reminders: the validator must FAIL CLOSED. Under no circumstance
should it coerce, default, or invent a missing field into a pass.

Run: python3 tools/release/test_check_release_manifest.py
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
VALIDATOR = REPO_ROOT / "tools" / "release" / "check_release_manifest.py"

REQUIRED_SECTIONS = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]


def _shell(*args, cwd=None, env=None, validator=None):
    return subprocess.run(
        [sys.executable, str(validator or VALIDATOR), *args],
        cwd=str(cwd or REPO_ROOT),
        capture_output=True,
        text=True,
        env={**os.environ, **(env or {})},
    )


def _write(path: Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body, encoding="utf-8")


def _valid_release_notes() -> str:
    blocks = []
    for name in REQUIRED_SECTIONS:
        blocks.append(f"## {name}\n\n- _placeholder line._")
    return "\n\n".join(blocks) + "\n"


VALID_MANIFEST = {
    "schema_version": "v1",
    "tranche_id": "tranche-2.5",
    "release_notes_path": (
        "programs/codex/requirements/"
        "sd-16-feedback-loop-and-self-update-hardening/release-notes.md"
    ),
    "channel": "alpha",
    "spec_domain_id": "SD-16",
    "platform_artifacts": [
        {
            "platform": "linux",
            "tier": "first-class",
            "update_eligibility": "manual-only",
            "artifacts": [
                {"name": "codex_0.0.0_amd64.deb", "format": "deb"},
                {"name": "codex_0.0.0_amd64.AppImage", "format": "AppImage"},
            ],
        }
    ],
}


class TmpRepo(unittest.TestCase):
    """Runs each test against a sandbox copy of the repo so we don't pollute HEAD."""

    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.workdir = Path(self.tmp.name)
        # Copy the validator + schema only — that's all the validator needs.
        target = self.workdir / "tools" / "release"
        target.mkdir(parents=True, exist_ok=True)
        (target / "check_release_manifest.py").write_bytes(
            VALIDATOR.read_bytes()
        )
        (target / "release-manifest.schema.json").write_bytes(
            (VALIDATOR.parent / "release-manifest.schema.json").read_bytes()
        )
        # Init a minimal git repo so `_git_text` can resolve the merge-base ref.
        subprocess.check_output(["git", "-C", str(self.workdir), "init", "-q"])
        subprocess.check_output(
            [
                "git",
                "-C",
                str(self.workdir),
                "config",
                "user.email",
                "ci@example.invalid",
            ]
        )
        subprocess.check_output(
            [
                "git",
                "-C",
                str(self.workdir),
                "config",
                "user.name",
                "Codex CI",
            ]
        )
        subprocess.check_output(["git", "-C", str(self.workdir), "checkout", "-q", "-b", "main"])
        # Initial empty commit so refs exist.
        subprocess.check_output(["git", "-C", str(self.workdir), "commit", "--allow-empty", "-m", "init", "-q"])

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def _commit(self, *files: Path, message: str = "fixture") -> None:
        subprocess.check_output(
            ["git", "-C", str(self.workdir), "add", *map(str, files)]
        )
        subprocess.check_output(
            [
                "git",
                "-C",
                str(self.workdir),
                "commit",
                "-q",
                "-m",
                message,
                "--author=Codex CI <ci@example.invalid>",
            ]
        )

    def test_valid_manifest_passes(self):
        notes = self.workdir / VALID_MANIFEST["release_notes_path"]
        _write(notes, _valid_release_notes())
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(VALID_MANIFEST))
        self._commit(notes, manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 0, msg=res.stderr or res.stdout)

    def test_missing_tranche_id_fails(self):
        manifest_payload = dict(VALID_MANIFEST)
        manifest_payload.pop("tranche_id")
        notes = self.workdir / VALID_MANIFEST["release_notes_path"]
        _write(notes, _valid_release_notes())
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(manifest_payload))
        self._commit(notes, manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2, msg=res.stderr or res.stdout)
        self.assertRegex(
            res.stderr,
            r"(schema validation failed: .*tranche_id|missing required field 'tranche_id')",
            msg=res.stderr,
        )

    def test_missing_release_notes_path_fails(self):
        manifest_payload = dict(VALID_MANIFEST)
        manifest_payload.pop("release_notes_path")
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(manifest_payload))
        self._commit(manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        self.assertIn("release_notes_path", res.stderr)

    def test_missing_release_notes_file_fails(self):
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(VALID_MANIFEST))
        self._commit(manifest)
        # release-notes.md intentionally NOT staged.

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        self.assertIn("does not exist", res.stderr)

    def test_empty_release_notes_fails(self):
        notes = self.workdir / VALID_MANIFEST["release_notes_path"]
        _write(notes, "")
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(VALID_MANIFEST))
        self._commit(notes, manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        self.assertIn("release-notes file is empty", res.stderr)

    def test_notes_with_missing_section_fails(self):
        notes = self.workdir / VALID_MANIFEST["release_notes_path"]
        body = _valid_release_notes().replace("## Update Eligibility\n\n- _placeholder line._\n", "")
        _write(notes, body)
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(VALID_MANIFEST))
        self._commit(notes, manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        self.assertIn("Update Eligibility", res.stderr)

    def test_wrong_layout_fails(self):
        manifest_payload = dict(VALID_MANIFEST)
        manifest_payload["release_notes_path"] = "docs/release-notes.md"
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(manifest_payload))
        self._commit(manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        # Either failure mode is acceptable — the schema regex on
        # release_notes_path OR the explicit layout check on the validator side.
        # The gate held either way; the validator surfaces a failure with rc=2.
        self.assertEqual(res.returncode, 2)
        self.assertTrue(
            "release-notes" in res.stderr
            or "release_notes_path" in res.stderr
            or "schema validation failed" in res.stderr,
            msg=res.stderr,
        )
    

    def test_tranche_id_must_start_with_tranche(self):
        manifest_payload = dict(VALID_MANIFEST)
        manifest_payload["tranche_id"] = "2.5"
        manifest = self.workdir / "release-manifest.json"
        _write(manifest, json.dumps(manifest_payload))
        self._commit(manifest)

        res = _shell("release-manifest.json", cwd=self.workdir, env={"GITHUB_BASE_REF": "main"}, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertEqual(res.returncode, 2)
        # schema rejects via regex (pattern) — that's the canonical path
        self.assertTrue(
            "tranche-" in res.stderr or "schema validation failed" in res.stderr,
            msg=res.stderr,
        )

    def test_no_manifests_is_usage_error(self):
        # Calling with zero paths means a usage error (rc=1).
        res = _shell(cwd=self.workdir, validator=self.workdir / "tools" / "release" / "check_release_manifest.py")
        self.assertNotEqual(res.returncode, 0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
