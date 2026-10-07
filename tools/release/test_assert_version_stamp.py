#!/usr/bin/env python3
"""
Tests for tools/release/assert_version_stamp.py.

The stamp job's artifact held apps/desktop/package.json and apps/desktop/src-tauri/tauri.conf.json;
`upload-artifact` strips the common parent, so the platform jobs unpacked them at the repo root and
built the unstamped files. This gate runs right before `tauri build` and fails if the tree about to
be built does not carry the stamped version.

Run: python3 -m pytest tools/release/test_assert_version_stamp.py -v
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

GATE = Path(__file__).resolve().parent / "assert_version_stamp.py"


def _tree(package: str, tauri: str, *, stray_root_files: bool = False) -> Path:
    root = Path(tempfile.mkdtemp(prefix="stamp-"))
    (root / "apps/desktop/src-tauri").mkdir(parents=True)
    (root / "apps/desktop/package.json").write_text(json.dumps({"version": package}))
    (root / "apps/desktop/src-tauri/tauri.conf.json").write_text(json.dumps({"version": tauri}))
    if stray_root_files:
        # What the broken download left behind: stamped copies in the wrong place.
        (root / "package.json").write_text(json.dumps({"version": "0.16.141"}))
        (root / "src-tauri").mkdir()
        (root / "src-tauri/tauri.conf.json").write_text(json.dumps({"version": "0.16.141"}))
    return root


def _run(root: Path, version: str = "0.16.141") -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(GATE), "--repo-root", str(root), "--version", version],
        capture_output=True, text=True,
    )


class AssertVersionStampTests(unittest.TestCase):
    def test_passes_when_both_files_carry_the_stamp(self) -> None:
        self.assertEqual(_run(_tree("0.16.141", "0.16.141")).returncode, 0)

    def test_fails_when_package_json_was_not_stamped(self) -> None:
        result = _run(_tree("0.16.0", "0.16.141"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("apps/desktop/package.json", result.stderr)

    def test_fails_when_tauri_conf_was_not_stamped(self) -> None:
        result = _run(_tree("0.16.141", "0.16.0"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tauri.conf.json", result.stderr)

    def test_stamped_files_unpacked_at_the_repo_root_are_called_out(self) -> None:
        result = _run(_tree("0.16.0", "0.16.0", stray_root_files=True))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("repo root", result.stderr)


if __name__ == "__main__":
    unittest.main()
