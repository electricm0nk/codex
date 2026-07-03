"""
Tests for the tranche release-notes mechanical validator.

Doctrine (programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening):

  - Each tranche has a release manifest that explicitly binds:
        tranche_id          (e.g. "tranche-2.5")
        release_notes_path  (repo-relative path to release-notes.md)
        channel             (alpha | beta | stable)
  - The release-notes.md at that path must contain every required section
    heading (H2 level, exact match, in canonical order), and no section may be
    empty.
  - The validator MUST refuse to invent, fall back to defaults, or coerce
    partial inputs into success.
  - The release lane MUST NOT infer tranche_id from PR titles, branch names,
    or file paths alone; the manifest is the only source of truth.
  - The validator MUST exit non-zero on any failure and print a final JSON
    report line on stdout.

These tests pin that contract.
"""

from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
VALIDATOR = REPO_ROOT / "scripts" / "tranche" / "validate-tranche-notes.py"


def _run_validator(*args: str, cwd: Path) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(VALIDATOR), *args],
        cwd=cwd,
        capture_output=True,
        text=True,
    )


REQUIRED_SECTIONS = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]


def _write_release_notes(path: Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body, encoding="utf-8")


def _valid_notes_body() -> str:
    sections = "\n\n".join(f"## {name}\n\n- _placeholder line._" for name in REQUIRED_SECTIONS)
    return sections + "\n"


def _valid_manifest_yaml(tranche_id: str = "2.5") -> str:
    return textwrap.dedent(
        f"""\
        schema_version: v1
        tranche_id: "{tranche_id}"
        channel: "alpha"
        promotion_branch: "develop"
        release_notes_path: "programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md"
        """
    )


class _Workspace:
    """Helper that builds a minimal repo-shaped fixture with a tranche manifest + notes."""

    def __init__(self, root: Path) -> None:
        self.root = root
        # Mirror the canonical validator location into the fixture.
        (root / "scripts" / "tranche").mkdir(parents=True, exist_ok=True)
        if VALIDATOR.exists():
            shutil.copy(VALIDATOR, root / "scripts" / "tranche" / "validate-tranche-notes.py")

    def write_manifest(
        self,
        *,
        tranche_id: str = "2.5",
        release_notes_path: str = "release-notes.md",
        channel: str = "alpha",
        promotion_branch: str = "develop",
        notes_subdir: str | None = None,
        spec_dir: str = "SD-16-feedback-loop-and-self-update-hardening",
    ) -> Path:
        # Spec-domain directory layout: programs/codex/requirements/<spec_dir>/tranche-<id>/manifest.yaml
        mdir = (
            self.root
            / "programs"
            / "codex"
            / "requirements"
            / spec_dir
            / f"tranche-{tranche_id}"
        )
        mdir.mkdir(parents=True, exist_ok=True)
        manifest = mdir / "manifest.yaml"
        manifest.write_text(
            textwrap.dedent(
                f"""\
                schema_version: v1
                tranche_id: "{tranche_id}"
                channel: "{channel}"
                promotion_branch: "{promotion_branch}"
                release_notes_path: "{release_notes_path}"
                """
            ),
            encoding="utf-8",
        )
        # Notes are written at <tranche-dir>/release-notes.md by default, or under
        # <tranche-dir>/<notes_subdir>/release-notes.md when notes_subdir is provided.
        if notes_subdir is None:
            notes_path = mdir / "release-notes.md"
        else:
            notes_path = mdir / notes_subdir / "release-notes.md"
        _write_release_notes(notes_path, _valid_notes_body())
        return manifest


class ValidatorImportSmokeTest(unittest.TestCase):
    def test_validator_script_exists(self) -> None:
        self.assertTrue(
            VALIDATOR.exists(),
            f"Validator script missing at {VALIDATOR}",
        )


class ManifestValidationTest(unittest.TestCase):
    def test_valid_manifest_passes(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest()
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertEqual(
                proc.returncode,
                0,
                msg=f"expected success\nstdout:\n{proc.stdout}\nstderr:\n{proc.stderr}",
            )
            # Last non-empty line of stdout must be a JSON report.
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertEqual(report["status"], "ok")
            self.assertEqual(report["tranche_id"], "2.5")
            self.assertEqual(report["channel"], "alpha")
            self.assertEqual(report["required_sections_found"], REQUIRED_SECTIONS)

    def test_missing_required_section_fails(self) -> None:
        bad_body = _valid_notes_body().replace(
            "## Update Eligibility\n\n- _placeholder line._",
            "## Update Eligibility\n",
        )
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest()
            spec_dir = (
                Path(td)
                / "programs"
                / "codex"
                / "requirements"
                / "SD-16-feedback-loop-and-self-update-hardening"
                / "tranche-2.5"
            )
            _write_release_notes(spec_dir / "release-notes.md", bad_body)
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertEqual(report["status"], "error")
            self.assertTrue(
                any("empty" in e.lower() for e in report["errors"]),
                f"expected an 'empty section' error, got: {report['errors']}",
            )

    def test_missing_section_heading_fails(self) -> None:
        sections = REQUIRED_SECTIONS[:-1]
        bad_body = "\n\n".join(f"## {n}\n\n- _x_" for n in sections) + "\n"
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest()
            spec_dir = (
                Path(td)
                / "programs"
                / "codex"
                / "requirements"
                / "SD-16-feedback-loop-and-self-update-hardening"
                / "tranche-2.5"
            )
            _write_release_notes(spec_dir / "release-notes.md", bad_body)
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertEqual(report["status"], "error")
            self.assertTrue(
                any("Update Eligibility" in e for e in report["errors"]),
                f"expected missing-section error naming 'Update Eligibility', got: {report['errors']}",
            )

    def test_manifest_with_unknown_channel_fails(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest(channel="canary")
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertTrue(
                any("channel" in e for e in report["errors"]),
                f"expected channel error, got: {report['errors']}",
            )

    def test_manifest_tranche_id_must_match_directory(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            bad_dir = (
                Path(td)
                / "programs"
                / "codex"
                / "requirements"
                / "SD-16-feedback-loop-and-self-update-hardening"
                / "tranche-2.5"
            )
            bad_dir.mkdir(parents=True, exist_ok=True)
            bad_manifest = bad_dir / "manifest.yaml"
            bad_manifest.write_text(
                textwrap.dedent(
                    """\
                    schema_version: v1
                    tranche_id: "3.0"
                    channel: "alpha"
                    promotion_branch: "develop"
                    release_notes_path: "release-notes.md"
                    """
                ),
                encoding="utf-8",
            )
            _write_release_notes(bad_dir / "release-notes.md", _valid_notes_body())
            proc = _run_validator("--manifest", str(bad_manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertTrue(
                any("directory" in e.lower() or "tranche" in e.lower() for e in report["errors"]),
                f"expected directory-mismatch error, got: {report['errors']}",
            )

    def test_release_notes_path_must_resolve_under_manifest_dir(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest(release_notes_path="../../../etc/passwd")
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertTrue(
                any(
                    "release_notes_path" in e or "under" in e.lower() or "contain" in e.lower()
                    for e in report["errors"]
                ),
                f"expected release_notes_path containment error, got: {report['errors']}",
            )

    def test_release_notes_file_missing_fails(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest(release_notes_path="missing-release-notes.md")
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertTrue(
                any("not found" in e.lower() or "missing" in e.lower() for e in report["errors"]),
                f"expected missing-file error, got: {report['errors']}",
            )

    def test_section_order_must_match_schema(self) -> None:
        reordered = list(REQUIRED_SECTIONS)
        reordered[0], reordered[1] = reordered[1], reordered[0]
        bad_body = "\n\n".join(f"## {n}\n\n- _x_" for n in reordered) + "\n"
        with tempfile.TemporaryDirectory() as td:
            ws = _Workspace(Path(td))
            manifest = ws.write_manifest()
            spec_dir = (
                Path(td)
                / "programs"
                / "codex"
                / "requirements"
                / "SD-16-feedback-loop-and-self-update-hardening"
                / "tranche-2.5"
            )
            _write_release_notes(spec_dir / "release-notes.md", bad_body)
            proc = _run_validator("--manifest", str(manifest), cwd=Path(td))
            self.assertNotEqual(proc.returncode, 0)
            report_line = next(
                (ln for ln in reversed(proc.stdout.strip().splitlines()) if ln.strip()),
                "",
            )
            report = json.loads(report_line)
            self.assertTrue(
                any("order" in e.lower() for e in report["errors"]),
                f"expected order error, got: {report['errors']}",
            )


if __name__ == "__main__":
    unittest.main()