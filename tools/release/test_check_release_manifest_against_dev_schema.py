#!/usr/bin/env python3
"""
Tests for tools/release/check_release_manifest_against_dev_schema.py.

This shim validates a dev-schema (F3b-shape) update-manifest.json against
schemas/update/update-manifest.schema.json, then re-uses
check_release_manifest.py's `_coherence_check` (tranche_id / release-notes
binding) against the same manifest. It exists because
`_coherence_check`'s `tranche_id.startswith("tranche-")` rule does not
apply to the dev schema, which locks `tranche_id` to the const
"STC-CODEX-SD-16".

Run: python3 -m pytest tools/release/test_check_release_manifest_against_dev_schema.py -v
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SHIM = REPO_ROOT / "tools" / "release" / "check_release_manifest_against_dev_schema.py"
COHERENCE_CHECK_MODULE = REPO_ROOT / "tools" / "release" / "check_release_manifest.py"
MANIFEST_SCHEMA = REPO_ROOT / "schemas" / "update" / "update-manifest.schema.json"

REQUIRED_NOTES_SECTIONS = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]


def _valid_release_notes() -> str:
    blocks = [f"## {name}\n\n- _placeholder line._" for name in REQUIRED_NOTES_SECTIONS]
    return "\n\n".join(blocks) + "\n"


def _dev_manifest(release_notes_path: str) -> dict:
    return {
        "schema_version": "1.0.0",
        "channel": "alpha",
        "version": "0.0.0",
        "tag": "alpha/v0.0.0-abcdef12",
        "tranche_id": "STC-CODEX-SD-16",
        "source_branch": "develop",
        "source_commit": "a" * 40,
        "release_notes_path": release_notes_path,
        "release_notes_url": "https://github.com/electricm0nk/codex/releases/tag/alpha/v0.0.0-abcdef12",
        "release_notes_hash": "b" * 64,
        "linux_appimage": {
            "name": "codex_0.0.0_amd64.AppImage",
            "url": (
                "https://github.com/electricm0nk/codex/releases/download/"
                "alpha/v0.0.0-abcdef12/codex_0.0.0_amd64.AppImage"
            ),
            "sha256": "c" * 64,
            "size_bytes": 1024,
        },
        "workflow_provenance": {
            "workflow": ".github/workflows/publish-tester-release.yml",
            "run_id": 123456,
            "run_attempt": 1,
        },
        "eligibility": {
            "min_supported_version": "0.0.0",
            "appimage_install": True,
            "required_install_kind": "appimage",
        },
        "promotion_lineage": {
            "source_branch": "develop",
            "source_commit": "a" * 40,
            "promoted_at": "2026-07-05T00:00:00Z",
        },
        "signature": None,
    }


class TmpRepo(unittest.TestCase):
    """Runs each test against a sandbox copy of the repo so we don't pollute HEAD."""

    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.workdir = Path(self.tmp.name)

        tools_target = self.workdir / "tools" / "release"
        tools_target.mkdir(parents=True, exist_ok=True)
        (tools_target / "check_release_manifest_against_dev_schema.py").write_bytes(
            SHIM.read_bytes()
        )
        (tools_target / "check_release_manifest.py").write_bytes(
            COHERENCE_CHECK_MODULE.read_bytes()
        )

        schema_target = self.workdir / "schemas" / "update"
        schema_target.mkdir(parents=True, exist_ok=True)
        (schema_target / "update-manifest.schema.json").write_bytes(
            MANIFEST_SCHEMA.read_bytes()
        )

        subprocess.check_output(["git", "-C", str(self.workdir), "init", "-q"])
        subprocess.check_output(
            ["git", "-C", str(self.workdir), "config", "user.email", "ci@example.invalid"]
        )
        subprocess.check_output(
            ["git", "-C", str(self.workdir), "config", "user.name", "Codex CI"]
        )
        subprocess.check_output(["git", "-C", str(self.workdir), "checkout", "-q", "-b", "main"])
        subprocess.check_output(
            ["git", "-C", str(self.workdir), "commit", "--allow-empty", "-m", "init", "-q"]
        )

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def _write(self, path: Path, body: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(body, encoding="utf-8")

    def _commit(self, *files: Path, message: str = "fixture") -> None:
        subprocess.check_output(["git", "-C", str(self.workdir), "add", *map(str, files)])
        subprocess.check_output(
            [
                "git", "-C", str(self.workdir), "commit", "-q", "-m", message,
                "--author=Codex CI <ci@example.invalid>",
            ]
        )

    def _run_shim(self, manifest_path: Path):
        return subprocess.run(
            [
                sys.executable,
                str(self.workdir / "tools" / "release" / "check_release_manifest_against_dev_schema.py"),
                str(manifest_path),
            ],
            cwd=str(self.workdir),
            capture_output=True,
            text=True,
            env={**os.environ, "GITHUB_BASE_REF": "main"},
        )


class CheckReleaseManifestAgainstDevSchemaTests(TmpRepo):
    def test_dev_schema_manifest_passes(self):
        notes_rel = "docs/release/sd-16-fixture/release-notes.md"
        notes_path = self.workdir / notes_rel
        self._write(notes_path, _valid_release_notes())

        manifest_payload = _dev_manifest(notes_rel)
        manifest_path = self.workdir / "update-manifest.json"
        self._write(manifest_path, json.dumps(manifest_payload))

        self._commit(notes_path, manifest_path)

        res = self._run_shim(manifest_path)
        self.assertEqual(res.returncode, 0, msg=res.stderr or res.stdout)
        self.assertIn("PASS", res.stdout)

    def test_missing_release_notes_file_fails(self):
        notes_rel = "docs/release/sd-16-fixture/release-notes.md"
        # Spec-domain directory exists on disk, but the release-notes file
        # itself is never written or committed.
        (self.workdir / "docs" / "release" / "sd-16-fixture").mkdir(
            parents=True, exist_ok=True
        )

        manifest_payload = _dev_manifest(notes_rel)
        manifest_path = self.workdir / "update-manifest.json"
        self._write(manifest_path, json.dumps(manifest_payload))
        self._commit(manifest_path)

        res = self._run_shim(manifest_path)
        self.assertEqual(res.returncode, 2, msg=res.stderr or res.stdout)
        self.assertIn("does not exist", res.stderr)


if __name__ == "__main__":
    unittest.main(verbosity=2)
