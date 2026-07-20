#!/usr/bin/env python3
"""
Tests for tools/release/emit_channel_index.py.

These tests pin the SD16-E4-F4 contract for the alpha channel-index emitter:
it reads an already schema-valid update-manifest.json and emits a
channels/<channel>.json pointer that satisfies
schemas/update/channel-index.schema.json. The emitter is intentionally tiny
— it does not author release truth, does not compute checksums, and does not
inspect the source repo beyond the manifest file it is given.

Run: python3 -m pytest tools/release/test_emit_channel_index.py -v
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
EMITTER = REPO_ROOT / "tools" / "release" / "emit_channel_index.py"
CHANNEL_INDEX_SCHEMA = REPO_ROOT / "schemas" / "update" / "channel-index.schema.json"

VALID_MANIFEST = {
    "schema_version": "1.0.0",
    "channel": "alpha",
    "version": "0.0.0",
    "tag": "alpha/v0.0.0-abcdef12",
    "tranche_id": "STC-CODEX-SD-16",
    "source_branch": "develop",
    "source_commit": "a" * 40,
    "release_notes_path": "docs/release/SD-16/release-notes.md",
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


def _run(*args):
    return subprocess.run(
        [sys.executable, str(EMITTER), *args],
        capture_output=True,
        text=True,
        env={**os.environ},
    )


class EmitChannelIndexTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.workdir = Path(self.tmp.name)

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def _write_manifest(self, payload: dict, name: str = "update-manifest.json") -> Path:
        path = self.workdir / name
        path.write_text(json.dumps(payload), encoding="utf-8")
        return path

    def test_happy_path_alpha(self):
        manifest_path = self._write_manifest(VALID_MANIFEST)
        output_path = self.workdir / "channels" / "alpha.json"

        res = _run(
            "--channel", "alpha",
            "--manifest", str(manifest_path),
            "--output", str(output_path),
            "--release-url", "https://github.com/electricm0nk/codex/releases/tag/alpha/v0.0.0-abcdef12",
            "--manifest-url", "https://github.com/electricm0nk/codex/releases/download/alpha/v0.0.0-abcdef12/update-manifest.json",
            "--published-at", "2026-07-05T00:00:00Z",
        )
        self.assertEqual(res.returncode, 0, msg=res.stderr or res.stdout)

        emitted = json.loads(output_path.read_text(encoding="utf-8"))
        schema = json.loads(CHANNEL_INDEX_SCHEMA.read_text(encoding="utf-8"))
        import jsonschema  # local import so the RED phase doesn't need jsonschema
        jsonschema.validate(emitted, schema)

        self.assertEqual(emitted["channel"], "alpha")
        self.assertEqual(emitted["version"], "0.0.0")
        self.assertEqual(emitted["tag"], "alpha/v0.0.0-abcdef12")
        self.assertEqual(emitted["tranche_id"], "STC-CODEX-SD-16")
        self.assertEqual(emitted["schema_version"], "1.0.0")
        self.assertIsNone(emitted["signature"])

    def test_wrong_channel_exits_2(self):
        manifest_path = self._write_manifest(VALID_MANIFEST)
        output_path = self.workdir / "channels" / "beta.json"

        res = _run(
            "--channel", "beta",
            "--manifest", str(manifest_path),
            "--output", str(output_path),
            "--release-url", "https://github.com/electricm0nk/codex/releases/tag/alpha/v0.0.0-abcdef12",
            "--manifest-url", "https://github.com/electricm0nk/codex/releases/download/alpha/v0.0.0-abcdef12/update-manifest.json",
            "--published-at", "2026-07-05T00:00:00Z",
        )
        self.assertEqual(res.returncode, 2, msg=res.stderr or res.stdout)
        self.assertFalse(output_path.exists())

    def test_malformed_tag_exits_2(self):
        bad_manifest = dict(VALID_MANIFEST)
        bad_manifest["tag"] = "beta/v0.0.0-abcdef12"
        manifest_path = self._write_manifest(bad_manifest)
        output_path = self.workdir / "channels" / "alpha.json"

        res = _run(
            "--channel", "alpha",
            "--manifest", str(manifest_path),
            "--output", str(output_path),
            "--release-url", "https://github.com/electricm0nk/codex/releases/tag/alpha/v0.0.0-abcdef12",
            "--manifest-url", "https://github.com/electricm0nk/codex/releases/download/alpha/v0.0.0-abcdef12/update-manifest.json",
            "--published-at", "2026-07-05T00:00:00Z",
        )
        self.assertEqual(res.returncode, 2, msg=res.stderr or res.stdout)
        self.assertFalse(output_path.exists())


if __name__ == "__main__":
    unittest.main(verbosity=2)
