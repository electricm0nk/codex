#!/usr/bin/env python3
"""Resolve the release notes the publish workflow ships.

Prints the repo-relative path of the current tranche's release notes, taken from the committed
pointer `docs/release/current-release.json`:

    {"tranche": 16, "notes_path": "docs/release/SD-36-consolidation/release-notes.md"}

Fails (exit 2, reason on stderr) when the pointer is missing, names a file that is absent, empty
or still a closure placeholder, sits outside `docs/release/<spec-dir>/release-notes.md`, or
disagrees with the tranche of `apps/desktop/package.json` (`0.<tranche>.<build>`). Only the path
goes to stdout so the workflow can capture it.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import NoReturn

POINTER = "docs/release/current-release.json"
NOTES_PATH_PATTERN = re.compile(r"^docs/release/[^/]+/release-notes\.md$")
PLACEHOLDER_MARKERS = ("Populated at closure", "Pre-population placeholder")


def _fail(message: str) -> NoReturn:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(2)


def resolve(repo_root: Path) -> str:
    pointer_path = repo_root / POINTER
    if not pointer_path.is_file():
        _fail(f"{POINTER} is missing; it names the current tranche's release notes")
    try:
        pointer = json.loads(pointer_path.read_text(encoding="utf-8"))
        tranche = pointer["tranche"]
        notes_path = pointer["notes_path"]
    except (ValueError, KeyError) as err:
        _fail(f"{POINTER} must be {{\"tranche\": <int>, \"notes_path\": <path>}}: {err!r}")

    version = json.loads((repo_root / "apps/desktop/package.json").read_text(encoding="utf-8"))["version"]
    parts = version.split(".")
    if len(parts) < 2 or not parts[1].isdigit() or int(parts[1]) != tranche:
        _fail(
            f"{POINTER} is for tranche {tranche} but apps/desktop/package.json is {version}; "
            "bump both together at closure"
        )

    if not NOTES_PATH_PATTERN.match(notes_path):
        _fail(f"notes_path {notes_path!r} must be docs/release/<spec-dir>/release-notes.md")
    notes_file = repo_root / notes_path
    if not notes_file.is_file():
        _fail(f"release notes file does not exist: {notes_path}")
    body = notes_file.read_text(encoding="utf-8")
    if not body.strip():
        _fail(f"release notes are empty: {notes_path}")
    for marker in PLACEHOLDER_MARKERS:
        if marker in body:
            _fail(f"release notes are still a closure placeholder ({marker!r}): {notes_path}")
    return notes_path


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--repo-root", required=True, type=Path)
    args = parser.parse_args(argv)
    print(resolve(args.repo_root))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
