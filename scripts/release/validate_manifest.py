#!/usr/bin/env python3
"""Validate a Codex update-manifest.json against schemas/update/update-manifest.schema.json.

SD16-E4-F3b — owned by `t_b7833349`. The publish-tester-release.yml workflow
calls this script as a guard step; an invalid manifest fails closed (exit
non-zero) so the publish step never uploads a malformed artifact to the
GitHub Release.

Doctrine reminders (per F2 handoff §AV rows owned):
  - AV-PUB-6  manifest validates against schema. Validator runs the schema
              check AND the workflow guard step runs the same check.
  - AV-SCH-5  manifest-validator cross-check on the merged branch.
              Schema-side half is enforced here; the release-notes side
              lives in scripts/tranche/validate-tranche-notes.py (E1-F3).
  - AV-SCH-6  negative fixture: release_notes_path outside the
              programs/codex/requirements/<SD>/ path MUST fail validation.

Exit codes:
  0   manifest satisfies the schema
  1   usage / setup error
  2   manifest failed validation (publish must abort)
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

try:
    import jsonschema  # type: ignore
except ImportError as exc:  # pragma: no cover - jsonschema is installed in CI via actions/setup-python
    print(f"FATAL: python jsonschema module not importable: {exc}", file=sys.stderr)
    sys.exit(1)


def _load_json(path: Path) -> object:
    text = path.read_text(encoding="utf-8")
    try:
        return json.loads(text)
    except json.JSONDecodeError as exc:
        print(f"FAIL: {path}: not valid JSON ({exc})", file=sys.stderr)
        raise SystemExit(2)


def _check(manifest_path: Path, schema_path: Path) -> int:
    manifest: object = _load_json(manifest_path)
    schema: object = _load_json(schema_path)
    if not isinstance(schema, dict):
        print(f"FAIL: {schema_path} is not a JSON object", file=sys.stderr)
        return 2
    if not isinstance(manifest, dict):
        print(f"FAIL: {manifest_path} is not a JSON object", file=sys.stderr)
        return 2
    validator = jsonschema.Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(manifest), key=lambda e: list(e.absolute_path))
    if errors:
        print(f"FAIL: {manifest_path} does not satisfy {schema_path}", file=sys.stderr)
        for err in errors:
            # `absolute_path` is a deque of path parts; join with / for readability.
            where = "/".join(str(p) for p in err.absolute_path) or "<root>"
            print(f"  - {where}: {err.message}", file=sys.stderr)
        return 2
    print(f"PASS  {manifest_path} satisfies {schema_path}")
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Validate a Codex update-manifest.json against schemas/update/update-manifest.schema.json.")
    parser.add_argument(
        "--manifest",
        required=True,
        type=Path,
        help="Path to the update-manifest.json to validate.",
    )
    parser.add_argument(
        "--schema",
        required=True,
        type=Path,
        help="Path to schemas/update/update-manifest.schema.json.",
    )
    args = parser.parse_args(argv)

    if not args.manifest.is_file():
        print(f"FATAL: manifest not found: {args.manifest}", file=sys.stderr)
        return 1
    if not args.schema.is_file():
        print(f"FATAL: schema not found: {args.schema}", file=sys.stderr)
        return 1

    return _check(args.manifest, args.schema)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
