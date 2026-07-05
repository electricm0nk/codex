#!/usr/bin/env python3
"""
Validate an update-manifest.json against schemas/update/update-manifest.schema.json
(the dev/schema-conformant shape produced by scripts/release/write_release_manifest.py),
then re-run check_release_manifest.py's `_coherence_check` (tranche_id /
release-notes binding) against the same manifest.

This shim exists because tools/release/check_release_manifest.py validates
against the LEGACY tools/release/release-manifest.schema.json shape, not the
dev schemas/update/update-manifest.schema.json shape. `_coherence_check`
itself is schema-agnostic (it only inspects `tranche_id` and
`release_notes_path`), so it is reused as-is against the dev-schema manifest.

Exit codes:
  0   manifest validates against the dev schema AND _coherence_check passes
  1   usage / setup error
  2   manifest failed schema validation OR _coherence_check reported errors
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

try:
    import jsonschema  # type: ignore
except ImportError as exc:  # pragma: no cover - installed in CI via actions/setup-python
    print(f"FATAL: python jsonschema module not importable: {exc}", file=sys.stderr)
    sys.exit(1)

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.release.check_release_manifest import _coherence_check  # noqa: E402

SCHEMA_PATH = REPO_ROOT / "schemas" / "update" / "update-manifest.schema.json"


def _load_json(path: Path) -> object:
    text = path.read_text(encoding="utf-8")
    try:
        return json.loads(text)
    except json.JSONDecodeError as exc:
        print(f"FAIL: {path}: not valid JSON ({exc})", file=sys.stderr)
        raise SystemExit(2)


def _check(manifest_path: Path) -> int:
    manifest = _load_json(manifest_path)
    schema = _load_json(SCHEMA_PATH)

    if not isinstance(manifest, dict):
        print(f"FAIL: {manifest_path} is not a JSON object", file=sys.stderr)
        return 2

    validator = jsonschema.Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(manifest), key=lambda e: list(e.absolute_path))
    if errors:
        print(f"FAIL: {manifest_path} does not satisfy {SCHEMA_PATH}", file=sys.stderr)
        for err in errors:
            where = "/".join(str(p) for p in err.absolute_path) or "<root>"
            print(f"  - {where}: {err.message}", file=sys.stderr)
        return 2

    coherence_errors = _coherence_check(REPO_ROOT, manifest, manifest_path)
    if coherence_errors:
        for err in coherence_errors:
            print(f"FAIL  {err}", file=sys.stderr)
        return 2

    print(f"PASS  {manifest_path}")
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Validate an update-manifest.json against the dev schema and _coherence_check."
    )
    parser.add_argument("manifest", type=Path, help="Path to update-manifest.json.")
    args = parser.parse_args(argv)

    if not args.manifest.is_file():
        print(f"FATAL: manifest not found: {args.manifest}", file=sys.stderr)
        return 1

    return _check(args.manifest)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
