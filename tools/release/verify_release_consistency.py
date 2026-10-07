#!/usr/bin/env python3
"""Fail the release when its artifacts disagree with its manifest.

Every check below is a defect that shipped in alpha v0.16.140: the binary reported 0.16.0 while the
manifest said 0.16.140 (the version stamp never reached the build), and the release-notes URL served
an HTML page that could never hash to `release_notes_hash`.

Checks (all failures are reported, not just the first):
  1. apps/desktop/package.json and src-tauri/tauri.conf.json carry the manifest's version, i.e. the
     stamp reached the tree that was built.
  2. linux_appimage.name contains that version.
  3. linux_deb (when present): the staged file matches the manifest's sha256 and size, and
     `dpkg-deb -f` reports Package=codex and Version=<manifest version>.
  4. release_notes_hash equals the sha256 of the notes file the manifest points at, and that file is
     not a closure placeholder.
  5. with --fetch-notes: the bytes served at release_notes_url hash to release_notes_hash, which is
     what the desktop app checks before showing notes.

Usage: verify_release_consistency.py --repo-root . --staging release-staging [--fetch-notes]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import urllib.request
from pathlib import Path

PLACEHOLDER_MARKERS = ("Populated at closure", "Pre-population placeholder")


def _sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _json_version(path: Path) -> str:
    return json.loads(path.read_text(encoding="utf-8")).get("version", "<absent>")


def _dpkg_field(deb: Path, field: str) -> str:
    out = subprocess.run(["dpkg-deb", "-f", str(deb), field], capture_output=True, text=True)
    if out.returncode != 0:
        raise RuntimeError(f"dpkg-deb -f {field} failed: {out.stderr.strip()}")
    return out.stdout.strip()


def check(repo_root: Path, staging: Path, fetch_notes: bool) -> list[str]:
    errors: list[str] = []
    manifest = json.loads((staging / "update-manifest.json").read_text(encoding="utf-8"))
    version = manifest["version"]

    for rel in ("apps/desktop/package.json", "apps/desktop/src-tauri/tauri.conf.json"):
        found = _json_version(repo_root / rel)
        if found != version:
            errors.append(f"{rel} says {found} but the manifest says {version}: the version stamp did not reach the build")

    appimage = manifest["linux_appimage"]["name"]
    if version not in appimage:
        errors.append(f"AppImage name {appimage} does not contain the manifest version {version}")

    deb_block = manifest.get("linux_deb")
    if deb_block is not None:
        deb = staging / deb_block["name"]
        if not deb.is_file():
            errors.append(f"linux_deb {deb_block['name']} is not staged")
        else:
            data = deb.read_bytes()
            if _sha(data) != deb_block["sha256"]:
                errors.append(f"linux_deb sha256 {_sha(data)} does not match the manifest {deb_block['sha256']}")
            if len(data) != deb_block["size_bytes"]:
                errors.append(f"linux_deb size {len(data)} does not match the manifest {deb_block['size_bytes']}")
            try:
                package, deb_version = _dpkg_field(deb, "Package"), _dpkg_field(deb, "Version")
            except RuntimeError as err:
                errors.append(str(err))
            else:
                if package != "codex":
                    errors.append(f"deb Package is {package!r}, expected 'codex'")
                if deb_version != version:
                    errors.append(f"deb Version is {deb_version} but the manifest version is {version}")

    notes_file = repo_root / manifest["release_notes_path"]
    if not notes_file.is_file():
        errors.append(f"release notes file {manifest['release_notes_path']} does not exist")
    else:
        notes = notes_file.read_bytes()
        if _sha(notes) != manifest["release_notes_hash"]:
            errors.append(
                f"release_notes_hash {manifest['release_notes_hash']} is not the sha256 of "
                f"{manifest['release_notes_path']} ({_sha(notes)})"
            )
        text = notes.decode("utf-8", errors="replace")
        for marker in PLACEHOLDER_MARKERS:
            if marker in text:
                errors.append(f"release notes are still a closure placeholder ({marker!r})")

    if fetch_notes:
        url = manifest["release_notes_url"]
        try:
            with urllib.request.urlopen(url, timeout=30) as response:  # noqa: S310 - URL comes from our own manifest
                served = response.read()
        except OSError as err:
            errors.append(f"release_notes_url {url} could not be fetched: {err}")
        else:
            if _sha(served) != manifest["release_notes_hash"]:
                errors.append(
                    f"bytes served at release_notes_url {url} ({_sha(served)}) does not hash to release_notes_hash "
                    f"{manifest['release_notes_hash']}; the app would show 'unavailable'"
                )
    return errors


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--staging", required=True, type=Path)
    parser.add_argument("--fetch-notes", action="store_true", help="also fetch release_notes_url and check its hash")
    args = parser.parse_args(argv)
    errors = check(args.repo_root, args.staging, args.fetch_notes)
    for error in errors:
        print(f"FAIL: {error}", file=sys.stderr)
    if errors:
        return 2
    print("OK: release artifacts, manifest and notes are consistent")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
