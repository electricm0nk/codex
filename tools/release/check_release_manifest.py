#!/usr/bin/env python3
"""
Validate a release manifest against tools/release/release-manifest.schema.json
and cross-check tranche_id / release_notes_path coherence on the working tree.

Doctrine (SD-16 / Tranche 2.x):
  - Publication fails closed if tranche release notes are missing, empty,
    structurally incomplete, or manifest-disconnected.
  - No placeholder/emergency-synthesized notes and no alpha exception.

Exit codes:
  0   all probed manifests satisfied the gate
  1   usage / setup error
  2   one or more manifests failed validation (PR must not merge / publish must abort)
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path

try:
    import jsonschema  # type: ignore
except ImportError:  # pragma: no cover - installed in CI via actions/setup-python
    jsonschema = None

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = Path(__file__).resolve().parent / "release-manifest.schema.json"

REQUIRED_NOTES_SECTIONS = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]


def _load_schema() -> dict:
    with SCHEMA_PATH.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def _parse_manifest(path: Path) -> dict:
    text = path.read_text(encoding="utf-8").lstrip("﻿")
    text = re.sub(r"(?m)^\s*//.*$", "", text)
    return json.loads(text)


def _git_text(repo_root: Path, rel_path: str) -> str | None:
    """Return file text from the merge-base version (or HEAD~1 if unavailable).

    Returning None means the file does NOT exist on the branch under test, which
    is itself a failure for release_notes_path bindings.

    Resolution order (each falls through on missing-or-unfetched ref):
      1. origin/<GITHUB_BASE_REF>   — typical PR base in GitHub Actions
      2. <GITHUB_BASE_REF>          — sandbox / local-only branch
      3. HEAD~1                     — single-commit local fallback
      4. HEAD                       — last resort, treats current tip as base
    """
    base_ref = os.environ.get("GITHUB_BASE_REF")
    candidates = []
    if base_ref:
        candidates.append(f"origin/{base_ref}")
        candidates.append(base_ref)
    else:
        candidates.append("HEAD~1")
        candidates.append("HEAD")
    for ref in candidates:
        try:
            out = subprocess.check_output(
                ["git", "-C", str(repo_root), "show", f"{ref}:{rel_path}"],
                stderr=subprocess.STDOUT,
            )
            return out.decode("utf-8")
        except subprocess.CalledProcessError:
            continue
    return None


def _check_notes(notes_text: str, manifest_path: Path) -> list[str]:
    errors: list[str] = []
    if not notes_text or not notes_text.strip():
        errors.append(f"{manifest_path}: release-notes file is empty")
        return errors
    missing = [s for s in REQUIRED_NOTES_SECTIONS if s not in notes_text]
    if missing:
        errors.append(
            f"{manifest_path}: release-notes missing required sections: {', '.join(missing)}"
        )
    return errors


def _coherence_check(repo_root: Path, manifest: dict, manifest_path: Path) -> list[str]:
    errors: list[str] = []
    tranche_id = manifest.get("tranche_id", "")
    notes_rel = manifest.get("release_notes_path", "")
    spec_domain = manifest.get("spec_domain_id")

    m = re.match(
        r"^programs/codex/requirements/(?P<specdir>[^/]+)/release-notes\.md$",
        notes_rel,
    )
    if not m:
        errors.append(
            f"{manifest_path}: release_notes_path does not match required layout "
            "(expected programs/codex/requirements/<spec-dir>/release-notes.md)"
        )
        return errors
    spec_dir = m.group("specdir")

    if spec_domain:
        expected_prefix = spec_domain.lower()
        if not spec_dir.lower().startswith(expected_prefix.lower()):
            errors.append(
                f"{manifest_path}: spec_domain_id={spec_domain} does not match "
                f"notes directory '{spec_dir}'"
            )

    sd_dir_abs = repo_root / "programs" / "codex" / "requirements" / spec_dir
    if not sd_dir_abs.is_dir():
        errors.append(
            f"{manifest_path}: spec domain directory does not exist on branch: {sd_dir_abs}"
        )

    notes_text = _git_text(repo_root, notes_rel)
    if notes_text is None:
        errors.append(
            f"{manifest_path}: release_notes_path points to {notes_rel} but that file "
            "does not exist on the merge-base / HEAD ref"
        )
    else:
        errors.extend(_check_notes(notes_text, manifest_path))

    if not tranche_id.startswith("tranche-"):
        errors.append(f"{manifest_path}: tranche_id must start with 'tranche-'")

    return errors


def validate_manifests(paths: list[Path]) -> int:
    schema = _load_schema()
    rc = 0
    if not paths:
        print("No release manifests supplied to validate.")
        return 1
    for mp in paths:
        try:
            rel = mp if not mp.is_absolute() else mp.resolve().relative_to(REPO_ROOT)
        except ValueError:
            rel = mp
        print("-- validating %s --" % rel)
        try:
            manifest = _parse_manifest(mp)
        except json.JSONDecodeError as exc:
            print(f"FAIL  {rel}: not valid JSON ({exc})", file=sys.stderr)
            rc = 2
            continue
        if jsonschema is not None:
            try:
                jsonschema.validate(manifest, schema)
            except jsonschema.ValidationError as exc:
                print(f"FAIL  {rel}: schema validation failed: {exc.message}", file=sys.stderr)
                rc = 2
                continue
        else:
            for required in (
                "schema_version",
                "tranche_id",
                "release_notes_path",
                "channel",
                "platform_artifacts",
            ):
                if required not in manifest:
                    print(
                        f"FAIL  {rel}: missing required field '{required}'",
                        file=sys.stderr,
                    )
                    rc = 2
                    continue
        coherence_errors = _coherence_check(REPO_ROOT, manifest, mp)
        if coherence_errors:
            for err in coherence_errors:
                print("FAIL  %s: %s" % (rel, err), file=sys.stderr)
            rc = 2
        else:
            print("PASS  %s" % rel)
    return rc


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Validate Codex release manifest(s).")
    parser.add_argument(
        "manifests",
        nargs="+",
        type=Path,
        help="Path(s) to release-manifest.json file(s) within the repo.",
    )
    args = parser.parse_args(argv)
    return validate_manifests(args.manifests)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
