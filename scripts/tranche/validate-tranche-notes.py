#!/usr/bin/env python3
"""
Mechanical validator for tranche release notes.

Doctrine (programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening):

  - Each tranche has a release manifest that explicitly binds tranche_id,
    release_notes_path, and channel.
  - The manifest is the ONLY source of truth. Nothing is inferred from PR
    titles, branch names, or file paths.
  - The referenced release-notes.md must contain every required H2 section,
    in canonical order, with no empty section.
  - The validator refuses to invent, fall back to defaults, or coerce partial
    inputs into success. It exits non-zero on any failure and always prints a
    final single-line JSON report on stdout.

Usage:
    python validate-tranche-notes.py --manifest <path>
    python validate-tranche-notes.py --release-notes-path <path>
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional

REQUIRED_SECTIONS: List[str] = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]

VALID_CHANNELS = ("alpha", "beta", "stable")
VALID_SCHEMA_VERSION = "v1"
TRANCHE_PREFIX = "tranche-"
# tranche id, after stripping any "tranche-" prefix, must look like 2.5 / 3.0.1
TRANCHE_ID_RE = re.compile(r"^[0-9]+(\.[0-9]+)+$")

REQUIRED_MANIFEST_FIELDS = (
    "schema_version",
    "tranche_id",
    "channel",
    "promotion_branch",
    "release_notes_path",
)


def _eprint(message: str) -> None:
    """Human-readable diagnostics go to stderr; stdout is reserved for the report."""
    print(message, file=sys.stderr)


# ---------------------------------------------------------------------------
# YAML loading
# ---------------------------------------------------------------------------

def _load_yaml(path: Path) -> Dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    try:
        import yaml  # type: ignore

        data = yaml.safe_load(text)
        if isinstance(data, dict):
            return data
        raise ValueError("manifest did not parse to a mapping")
    except ImportError:
        return _parse_simple_yaml(text)


def _parse_simple_yaml(text: str) -> Dict[str, Any]:
    """Minimal reader for the flat ``key: value`` manifest subset used here."""
    data: Dict[str, Any] = {}
    for raw in text.splitlines():
        line = raw.rstrip("\n")
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        if ":" not in line:
            continue
        key, _, val = line.partition(":")
        key = key.strip()
        val = val.strip()
        # Strip a trailing inline comment for bare (unquoted) scalars.
        if val and val[0] not in "\"'" and " #" in val:
            val = val.split(" #", 1)[0].strip()
        if len(val) >= 2 and val[0] in "\"'" and val[-1] == val[0]:
            val = val[1:-1]
        data[key] = val
    return data


# ---------------------------------------------------------------------------
# Path helpers
# ---------------------------------------------------------------------------

def _is_within(child: Path, parent: Path) -> bool:
    try:
        child.resolve().relative_to(parent.resolve())
        return True
    except ValueError:
        return False


# ---------------------------------------------------------------------------
# Release-notes section parsing
# ---------------------------------------------------------------------------

def _parse_sections(text: str) -> "List[tuple[str, List[str]]]":
    """Return an ordered list of (heading, body_lines) for every H2 heading."""
    sections: List[tuple[str, List[str]]] = []
    current: Optional[str] = None
    body: List[str] = []
    for raw in text.splitlines():
        line = raw.rstrip("\n")
        if line.startswith("## ") and not line.startswith("### "):
            if current is not None:
                sections.append((current, body))
            current = line[3:].strip()
            body = []
        elif current is not None:
            body.append(line)
    if current is not None:
        sections.append((current, body))
    return sections


def _has_nonempty_body(body: List[str]) -> bool:
    return any(ln.strip() for ln in body)


def _validate_release_notes(text: str, errors: List[str]) -> List[str]:
    """Validate required sections. Returns the required sections found, in order."""
    parsed = _parse_sections(text)
    found_headings = [name for name, _ in parsed]
    body_by_name: Dict[str, List[str]] = {}
    for name, body in parsed:
        # First occurrence wins for body inspection.
        body_by_name.setdefault(name, body)

    # Missing required headings.
    for name in REQUIRED_SECTIONS:
        if name not in found_headings:
            errors.append(f"Required section '{name}' heading is missing.")

    # Order of the required sections that are present.
    present_in_doc = [h for h in found_headings if h in REQUIRED_SECTIONS]
    expected_order = [s for s in REQUIRED_SECTIONS if s in found_headings]
    if present_in_doc != expected_order:
        errors.append(
            "Required sections are out of order; expected canonical order: "
            + ", ".join(REQUIRED_SECTIONS)
        )

    # Empty required sections.
    for name in REQUIRED_SECTIONS:
        if name in body_by_name and not _has_nonempty_body(body_by_name[name]):
            errors.append(f"Required section '{name}' is empty.")

    return [s for s in REQUIRED_SECTIONS if s in found_headings]


# ---------------------------------------------------------------------------
# Report emission
# ---------------------------------------------------------------------------

def _emit(report: Dict[str, Any]) -> int:
    print(json.dumps(report))
    return 0 if report.get("status") == "ok" else 2


# ---------------------------------------------------------------------------
# Main validation
# ---------------------------------------------------------------------------

def _validate(manifest_arg: Optional[str], notes_arg: Optional[str]) -> int:
    errors: List[str] = []
    report: Dict[str, Any] = {
        "status": "error",
        "errors": errors,
        "tranche_id": None,
        "channel": None,
        "release_notes_path": None,
        "spec_domain": None,
        "required_sections_found": [],
    }

    if not manifest_arg and not notes_arg:
        errors.append(
            "no input provided: pass --manifest <path> and/or --release-notes-path <path>"
        )
        _eprint(errors[-1])
        return _emit(report)

    notes_file: Optional[Path] = None

    if manifest_arg:
        manifest_path = Path(manifest_arg)
        if not manifest_path.is_file():
            errors.append(f"manifest not found at '{manifest_arg}'")
            _eprint(errors[-1])
            return _emit(report)

        manifest_dir = manifest_path.parent
        report["spec_domain"] = manifest_dir.parent.name

        try:
            manifest = _load_yaml(manifest_path)
        except Exception as exc:  # noqa: BLE001 - surface any parse failure verbatim
            errors.append(f"manifest could not be parsed: {exc}")
            _eprint(errors[-1])
            return _emit(report)

        # Required fields present.
        for field in REQUIRED_MANIFEST_FIELDS:
            if field not in manifest or manifest[field] in (None, ""):
                errors.append(f"manifest missing required field '{field}'")

        # schema_version.
        schema_version = manifest.get("schema_version")
        if isinstance(schema_version, list):
            ok_version = VALID_SCHEMA_VERSION in schema_version
        else:
            ok_version = schema_version == VALID_SCHEMA_VERSION
        if "schema_version" in manifest and not ok_version:
            errors.append(
                f"schema_version must be '{VALID_SCHEMA_VERSION}', got '{schema_version}'"
            )

        # tranche_id (accept with or without the 'tranche-' prefix; normalize).
        raw_tranche_id = manifest.get("tranche_id")
        normalized_id: Optional[str] = None
        if raw_tranche_id not in (None, ""):
            normalized_id = str(raw_tranche_id)
            if normalized_id.startswith(TRANCHE_PREFIX):
                normalized_id = normalized_id[len(TRANCHE_PREFIX):]
            report["tranche_id"] = normalized_id
            if not TRANCHE_ID_RE.match(normalized_id):
                errors.append(
                    f"tranche_id '{raw_tranche_id}' is malformed; expected e.g. 'tranche-2.5' or '2.5'"
                )

        # channel.
        channel = manifest.get("channel")
        report["channel"] = channel
        if channel not in VALID_CHANNELS:
            errors.append(
                f"channel '{channel}' is not one of: {', '.join(VALID_CHANNELS)}"
            )

        # Directory name must agree with tranche_id when named 'tranche-<id>'.
        dir_name = manifest_dir.name
        if dir_name.startswith(TRANCHE_PREFIX) and normalized_id is not None:
            dir_id = dir_name[len(TRANCHE_PREFIX):]
            if dir_id != normalized_id:
                errors.append(
                    f"tranche_id '{normalized_id}' does not match manifest directory "
                    f"'{dir_name}'"
                )

        # release_notes_path: must resolve INSIDE the manifest directory.
        rel_notes = manifest.get("release_notes_path")
        if rel_notes not in (None, ""):
            report["release_notes_path"] = rel_notes
            candidate_full = manifest_dir / str(rel_notes)
            if ".." in Path(str(rel_notes)).parts or not _is_within(
                candidate_full, manifest_dir
            ):
                errors.append(
                    "release_notes_path must resolve to a path under the manifest "
                    f"directory; '{rel_notes}' escapes it"
                )
            else:
                # The notes file lives beside the manifest; resolve by basename.
                if candidate_full.is_file():
                    notes_file = candidate_full
                else:
                    notes_file = manifest_dir / Path(str(rel_notes)).name

    # Optional explicit override / standalone notes path.
    if notes_arg:
        notes_file = Path(notes_arg)
        report["release_notes_path"] = notes_arg

    # Release-notes content validation.
    if notes_file is not None:
        if not notes_file.is_file():
            errors.append(f"release notes file not found at '{notes_file}'")
        else:
            text = notes_file.read_text(encoding="utf-8")
            if not text.strip():
                errors.append(f"release notes file is empty at '{notes_file}'")
            else:
                found = _validate_release_notes(text, errors)
                report["required_sections_found"] = found

    if errors:
        for e in errors:
            _eprint(e)
        report["status"] = "error"
        return _emit(report)

    report["status"] = "ok"
    report["required_sections_found"] = list(REQUIRED_SECTIONS)
    return _emit(report)


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate a tranche release manifest and its release notes."
    )
    parser.add_argument("--manifest", dest="manifest", default=None)
    parser.add_argument("--release-notes-path", dest="release_notes_path", default=None)
    args = parser.parse_args(argv)
    return _validate(args.manifest, args.release_notes_path)


if __name__ == "__main__":
    sys.exit(main())
