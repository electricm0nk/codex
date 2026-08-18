#!/usr/bin/env python3
"""Stamp a content hash onto every site stylesheet reference.

WHY THIS EXISTS
---------------
`site/styles.css` has a stable filename, so a browser that has visited the
site before will happily keep serving its cached copy after a deploy. On
2026-08-18 that produced a live page whose HTML was current but whose CSS was
not: the status table rendered as run-together text ("AdeptOriginNot started"),
the search box and filter chips fell back to native browser controls, and every
map hotspot lost `position: absolute` and piled up under the artwork. Nothing
was wrong with the committed files -- the browser simply never fetched the new
stylesheet.

Appending `?v=<hash-of-the-file>` makes the URL change whenever the bytes
change, so a stale cache entry can never be reused across a real edit, while an
unchanged stylesheet still caches normally.

Usage:
    python3 scripts/site/stamp_asset_versions.py            # rewrite in place
    python3 scripts/site/stamp_asset_versions.py --check    # verify only
"""
from __future__ import annotations

import hashlib
import pathlib
import re
import sys

REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
SITE = REPO_ROOT / "site"
ASSETS = ("styles.css",)
HASH_LEN = 10

# Matches href="/styles.css" with or without an existing ?v= stamp, so
# re-running is idempotent rather than appending a second query string.
def _pattern(asset: str) -> re.Pattern:
    return re.compile(r'(href="/' + re.escape(asset) + r')(\?v=[0-9a-f]+)?(")')


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()[:HASH_LEN]


def main(argv: list[str]) -> int:
    check_only = "--check" in argv[1:]

    if not SITE.is_dir():
        print(f"FAIL: no site/ directory at {SITE}", file=sys.stderr)
        return 1

    stamps = {}
    for asset in ASSETS:
        path = SITE / asset
        if not path.is_file():
            print(f"FAIL: expected asset missing: site/{asset}", file=sys.stderr)
            return 1
        stamps[asset] = digest(path)

    pages = sorted(SITE.glob("*.html"))
    if not pages:
        print("FAIL: no HTML pages found under site/", file=sys.stderr)
        return 1

    stale: list[str] = []
    changed: list[str] = []
    referenced = 0

    for page in pages:
        original = page.read_text(encoding="utf-8")
        updated = original
        for asset, stamp in stamps.items():
            pattern = _pattern(asset)
            hits = len(pattern.findall(updated))
            referenced += hits
            updated = pattern.sub(r"\g<1>?v=" + stamp + r"\g<3>", updated)
        if updated != original:
            rel = page.relative_to(REPO_ROOT)
            (stale if check_only else changed).append(str(rel))
            if not check_only:
                page.write_text(updated, encoding="utf-8")

    if referenced == 0:
        # A silent zero here would let the gate "pass" on a site that no longer
        # links the stylesheet the way this script understands -- exactly the
        # blind spot that let the stale CSS ship. Refuse instead.
        print(
            "FAIL: no stylesheet references matched. Either site/*.html stopped "
            'linking `href="/styles.css"`, or this script\'s pattern has drifted. '
            "Both are reasons to stop.",
            file=sys.stderr,
        )
        return 1

    if check_only:
        if stale:
            print("FAIL: stale cache-busting stamps in:", file=sys.stderr)
            for name in stale:
                print(f"  {name}", file=sys.stderr)
            print(
                "\nRun: python3 scripts/site/stamp_asset_versions.py",
                file=sys.stderr,
            )
            return 1
        print(f"OK: {referenced} stylesheet reference(s) carry a current stamp "
              f"({', '.join(f'{a}={h}' for a, h in stamps.items())})")
        return 0

    if changed:
        print("stamped:")
        for name in changed:
            print(f"  {name}")
    print(f"OK: {referenced} stylesheet reference(s) stamped "
          f"({', '.join(f'{a}={h}' for a, h in stamps.items())})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
