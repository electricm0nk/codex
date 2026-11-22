#!/usr/bin/env python3
"""Emit a Codex channels/<channel>.json from a schema-valid update-manifest.json.

SD16-E4-F4. Reads an update-manifest.json that already satisfies
schemas/update/update-manifest.schema.json, computes the channel-index shape
per schemas/update/channel-index.schema.json, writes channels/<channel>.json,
and validates the emitted file against that schema.

The emitter is intentionally tiny — it does not author release truth, does
not compute checksums, and does not inspect the source repo. It only reads a
manifest and emits a channel-index pointer.

Exit codes:
  0   channel index written and validated
  1   usage / setup error
  2   the manifest failed schema validation, or manifest.channel does not
      match --channel
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import NoReturn

try:
    import jsonschema  # type: ignore
except ImportError as exc:  # pragma: no cover - installed in CI via actions/setup-python
    print(f"FATAL: python jsonschema module not importable: {exc}", file=sys.stderr)
    sys.exit(1)

REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_SCHEMA_PATH = REPO_ROOT / "schemas" / "update" / "update-manifest.schema.json"
CHANNEL_INDEX_SCHEMA_PATH = REPO_ROOT / "schemas" / "update" / "channel-index.schema.json"

SCHEMA_VERSION = "1.0.0"
ALLOWED_CHANNELS = ("alpha", "beta", "stable")


def _fail(msg: str) -> NoReturn:
    print(f"FAIL: {msg}", file=sys.stderr)
    raise SystemExit(2)


def _load_json(path: Path) -> object:
    try:
        text = path.read_text(encoding="utf-8")
    except FileNotFoundError:
        _fail(f"{path}: not found")
    try:
        return json.loads(text)
    except json.JSONDecodeError as exc:
        _fail(f"{path}: not valid JSON ({exc})")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Emit a Codex channels/<channel>.json from a schema-valid update-manifest.json."
    )
    parser.add_argument("--channel", required=True, choices=list(ALLOWED_CHANNELS))
    parser.add_argument("--manifest", required=True, type=Path, help="Path to a schema-valid update-manifest.json.")
    parser.add_argument("--output", required=True, type=Path, help="Destination path for channels/<channel>.json.")
    parser.add_argument("--release-url", required=True, dest="release_url")
    parser.add_argument("--manifest-url", required=True, dest="manifest_url")
    parser.add_argument("--published-at", required=True, dest="published_at")
    args = parser.parse_args(argv)

    if not args.manifest.is_file():
        print(f"FATAL: manifest not found: {args.manifest}", file=sys.stderr)
        return 1

    manifest = _load_json(args.manifest)
    manifest_schema = _load_json(MANIFEST_SCHEMA_PATH)
    index_schema = _load_json(CHANNEL_INDEX_SCHEMA_PATH)

    if not isinstance(manifest, dict):
        _fail(f"{args.manifest} is not a JSON object")

    try:
        jsonschema.validate(manifest, manifest_schema)
    except jsonschema.ValidationError as exc:
        _fail(f"{args.manifest}: does not satisfy update-manifest schema: {exc.message}")

    if manifest.get("channel") != args.channel:
        _fail(
            f"manifest channel {manifest.get('channel')!r} does not match --channel {args.channel!r}"
        )

    index = {
        "schema_version": SCHEMA_VERSION,
        "channel": args.channel,
        "version": manifest["version"],
        "tag": manifest["tag"],
        "release_url": args.release_url,
        "manifest_url": args.manifest_url,
        "publication_timestamp": args.published_at,
        "tranche_id": manifest["tranche_id"],
        "signature": None,
    }

    try:
        jsonschema.validate(index, index_schema)
    except jsonschema.ValidationError as exc:
        _fail(f"emitted channel index does not satisfy channel-index schema: {exc.message}")

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(index, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"OK: wrote {args.output} (channel={args.channel} tag={manifest['tag']})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
