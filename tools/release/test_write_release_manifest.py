#!/usr/bin/env python3
"""
Tests for the linux_deb support in scripts/release/write_release_manifest.py.

A package-installed (.deb) shell can only self-update when the published manifest carries the
.deb's identity, so the writer must emit `linux_deb` (and schema 1.2.0) whenever a .deb is staged,
and must keep emitting schema 1.1.0 manifests otherwise.

Run: python3 -m pytest tools/release/test_write_release_manifest.py -v
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import jsonschema

REPO_ROOT = Path(__file__).resolve().parents[2]
WRITER = REPO_ROOT / "scripts" / "release" / "write_release_manifest.py"
SCHEMA = json.loads((REPO_ROOT / "schemas" / "update" / "update-manifest.schema.json").read_text())

DEB_BYTES = b"deb-bytes-for-test"
BASE = "https://github.com/electricm0nk/codex/releases/download/alpha-v0.16.141-abcdef12"


def _stage() -> tuple[Path, Path]:
    root = Path(tempfile.mkdtemp(prefix="write-manifest-"))
    notes = root / "docs/release/SD-99-example/release-notes.md"
    notes.parent.mkdir(parents=True)
    notes.write_text("## Summary\n\nreal\n")
    staging = root / "release-staging"
    staging.mkdir()
    (staging / "Codex_0.16.141_amd64.AppImage").write_bytes(b"appimage-bytes")
    (staging / "Codex_0.16.141_amd64.deb").write_bytes(DEB_BYTES)
    return root, staging


def _run(root: Path, staging: Path, *extra: str, install_kind: str = "any") -> subprocess.CompletedProcess:
    out = staging / "update-manifest.json"
    return subprocess.run(
        [
            sys.executable, str(WRITER),
            "--repo-root", str(root),
            "--channel", "alpha",
            "--tag", "alpha/v0.16.141-abcdef12",
            "--version", "0.16.141",
            "--source-branch", "develop",
            "--source-commit", "a" * 40,
            "--release-notes-path", "docs/release/SD-99-example/release-notes.md",
            "--appimage-name", "Codex_0.16.141_amd64.AppImage",
            "--appimage-path", str(staging / "Codex_0.16.141_amd64.AppImage"),
            "--appimage-url", f"{BASE}/Codex_0.16.141_amd64.AppImage",
            "--workflow", ".github/workflows/publish-tester-release.yml",
            "--run-id", "1",
            "--run-attempt", "1",
            "--release-notes-url", "https://raw.githubusercontent.com/electricm0nk/codex/" + "a" * 40 + "/docs/release/SD-99-example/release-notes.md",
            "--min-supported-version", "0.0.0",
            "--appimage-install", "true",
            "--required-install-kind", install_kind,
            "--output", str(out),
            *extra,
        ],
        capture_output=True,
        text=True,
    )


def _deb_flags(staging: Path) -> list[str]:
    return [
        "--linux-deb-name", "Codex_0.16.141_amd64.deb",
        "--linux-deb-path", str(staging / "Codex_0.16.141_amd64.deb"),
        "--linux-deb-url", f"{BASE}/Codex_0.16.141_amd64.deb",
    ]


class LinuxDebManifestTests(unittest.TestCase):
    def test_deb_block_carries_the_real_hash_and_size_of_the_staged_file(self) -> None:
        root, staging = _stage()
        result = _run(root, staging, *_deb_flags(staging))
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = json.loads((staging / "update-manifest.json").read_text())
        self.assertEqual(
            manifest["linux_deb"],
            {
                "name": "Codex_0.16.141_amd64.deb",
                "url": f"{BASE}/Codex_0.16.141_amd64.deb",
                "sha256": hashlib.sha256(DEB_BYTES).hexdigest(),
                "size_bytes": len(DEB_BYTES),
            },
        )

    def test_deb_manifest_is_schema_1_2_0_with_deb_install_and_validates(self) -> None:
        root, staging = _stage()
        _run(root, staging, *_deb_flags(staging))
        manifest = json.loads((staging / "update-manifest.json").read_text())
        self.assertEqual(manifest["schema_version"], "1.2.0")
        self.assertIs(manifest["eligibility"]["deb_install"], True)
        self.assertEqual(manifest["eligibility"]["required_install_kind"], "any")
        jsonschema.validate(manifest, SCHEMA)

    def test_without_a_deb_the_manifest_stays_1_1_0_and_has_no_deb_fields(self) -> None:
        root, staging = _stage()
        result = _run(root, staging)
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = json.loads((staging / "update-manifest.json").read_text())
        self.assertEqual(manifest["schema_version"], "1.1.0")
        self.assertNotIn("linux_deb", manifest)
        self.assertNotIn("deb_install", manifest["eligibility"])
        jsonschema.validate(manifest, SCHEMA)

    def test_partial_deb_flags_are_rejected(self) -> None:
        root, staging = _stage()
        result = _run(root, staging, "--linux-deb-name", "Codex_0.16.141_amd64.deb")
        self.assertEqual(result.returncode, 2)
        self.assertIn("complete triple", result.stderr)

    def test_missing_deb_file_is_rejected(self) -> None:
        root, staging = _stage()
        (staging / "Codex_0.16.141_amd64.deb").unlink()
        result = _run(root, staging, *_deb_flags(staging))
        self.assertEqual(result.returncode, 2)
        self.assertIn("does not exist", result.stderr)


if __name__ == "__main__":
    unittest.main()
