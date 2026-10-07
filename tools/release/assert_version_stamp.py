#!/usr/bin/env python3
"""Fail unless the tree about to be built carries the stamped version.

Run right before `tauri build` in every platform job. Checks apps/desktop/package.json and
apps/desktop/src-tauri/tauri.conf.json; if they are unstamped while stamped copies sit at the repo
root, says so, because that is the signature of a stamped-sources artifact unpacked in the wrong
directory.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

FILES = ("apps/desktop/package.json", "apps/desktop/src-tauri/tauri.conf.json")
STRAY = ("package.json", "src-tauri/tauri.conf.json")


def _version(path: Path) -> str | None:
    if not path.is_file():
        return None
    return json.loads(path.read_text(encoding="utf-8")).get("version")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", required=True)
    args = parser.parse_args(argv)

    errors = []
    for rel in FILES:
        found = _version(args.repo_root / rel)
        if found != args.version:
            errors.append(f"{rel} says {found} but the stamped version is {args.version}")
    if errors and any(_version(args.repo_root / rel) == args.version for rel in STRAY):
        errors.append(
            "stamped copies exist at the repo root: the stamped-sources artifact was unpacked "
            "into the wrong directory (download it with `path: apps/desktop`)"
        )
    for error in errors:
        print(f"FAIL: {error}", file=sys.stderr)
    if errors:
        return 2
    print(f"OK: build tree carries version {args.version}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
