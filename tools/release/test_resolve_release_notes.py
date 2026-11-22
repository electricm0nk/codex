#!/usr/bin/env python3
"""
Tests for tools/release/resolve_release_notes.py.

The publish workflow used to hardcode `docs/release/v0.7/release-notes.md`, so every release
shipped a stale tranche's notes. The resolver replaces that literal with a committed pointer
(`docs/release/current-release.json`) and refuses a pointer that disagrees with the app version
or names placeholder notes.

Run: python3 -m pytest tools/release/test_resolve_release_notes.py -v
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
RESOLVER = REPO_ROOT / "tools" / "release" / "resolve_release_notes.py"

GOOD_NOTES = "# SD-99 Release Notes\n\n## Summary\n\nReal content.\n"
NOTES_PATH = "docs/release/SD-99-example/release-notes.md"


def _make_repo(
    *,
    version: str = "0.16.0",
    pointer: dict | None = None,
    notes: str | None = GOOD_NOTES,
    notes_path: str = NOTES_PATH,
) -> Path:
    root = Path(tempfile.mkdtemp(prefix="resolve-notes-"))
    (root / "apps/desktop").mkdir(parents=True)
    (root / "apps/desktop/package.json").write_text(json.dumps({"version": version}))
    (root / "docs/release").mkdir(parents=True)
    if pointer is None:
        pointer = {"tranche": 16, "notes_path": notes_path}
    (root / "docs/release/current-release.json").write_text(json.dumps(pointer))
    if notes is not None:
        target = root / notes_path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(notes)
    return root


def _run(root: Path) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(RESOLVER), "--repo-root", str(root)],
        capture_output=True,
        text=True,
    )


class ResolveReleaseNotesTests(unittest.TestCase):
    def test_prints_the_pointed_path_when_everything_agrees(self) -> None:
        result = _run(_make_repo())
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), NOTES_PATH)

    def test_stamped_build_number_does_not_matter_only_the_tranche_does(self) -> None:
        result = _run(_make_repo(version="0.16.141"))
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_tranche_mismatch_with_app_version_fails_naming_both(self) -> None:
        result = _run(_make_repo(version="0.17.0"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tranche 16", result.stderr)
        self.assertIn("0.17.0", result.stderr)

    def test_missing_pointer_fails(self) -> None:
        root = _make_repo()
        (root / "docs/release/current-release.json").unlink()
        result = _run(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("current-release.json", result.stderr)

    def test_missing_notes_file_fails(self) -> None:
        result = _run(_make_repo(notes=None))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not exist", result.stderr)

    def test_placeholder_notes_are_refused(self) -> None:
        for marker in ("Populated at closure", "Pre-population placeholder"):
            with self.subTest(marker=marker):
                result = _run(_make_repo(notes=f"## Summary\n\n{marker}\n"))
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(marker, result.stderr)

    def test_empty_notes_are_refused(self) -> None:
        result = _run(_make_repo(notes="   \n"))
        self.assertNotEqual(result.returncode, 0)

    def test_path_outside_the_release_layout_is_refused(self) -> None:
        result = _run(_make_repo(notes_path="docs/elsewhere/notes.md"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("docs/release/<spec-dir>/release-notes.md", result.stderr)

    def test_stdout_carries_only_the_path_so_workflows_can_capture_it(self) -> None:
        result = _run(_make_repo())
        self.assertEqual(len(result.stdout.strip().splitlines()), 1)


if __name__ == "__main__":
    unittest.main()
