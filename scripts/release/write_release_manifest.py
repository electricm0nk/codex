#!/usr/bin/env python3
"""Emit a Codex update-manifest.json that satisfies schemas/update/update-manifest.schema.json.

SD16-E4-F3b — owned by `t_b7833349`. The publish-tester-release.yml workflow
calls this script from the `Generate update manifest` step; the next step
calls `validate_manifest.py` against the same file. Any failure in either
script fails the workflow closed.

The writer is intentionally explicit about every field the schema requires;
no field is hardcoded silently. Every argument the writer takes is a
deliberate shape decision the F3b slice owns. Future contract bumps must
extend both this script and the schema together.

Doctrine reminders (per F2 handoff §AV rows owned):
  - AV-PUB-2  release tag is channel-scoped. Writer enforces
              `${channel}/${version}-${shortsha}` so the manifest's `tag`
              field matches the schema's `^(alpha|beta|stable)/.+$`
              pattern. The GitHub release tag (used by gh release create
              and the channel index) is the schema-tag with the `/`
              replaced by `-`; the two coexist for backward compatibility
              with deployed tooling until a follow-up epic reconciles
              them.
  - AV-PUB-5  provenance identifies repo/workflow/run/source commit.
              `workflow_provenance.workflow` carries the workflow file
              path; `run_id` and `run_attempt` come from the live run.
  - AV-PUB-6  manifest validates against schema. The next step enforces
              this via validate_manifest.py; the writer does not
              self-validate (single responsibility, schema guard owns
              the check).
  - AV-SCH-5  manifest-validator cross-check. The schema enforces
              `tranche_id == 'STC-CODEX-SD-16'`; this writer emits
              exactly that value.
  - AV-SCH-6  release_notes_path is locked to
              `programs/codex/requirements/<SD>/release-notes.md`.
              Writer refuses to emit a manifest whose notes path is
              outside that prefix.

Exit codes:
  0   manifest written and JSON-serialised
  1   usage / setup error
  2   a required input was missing or a hard invariant was violated
"""

from __future__ import annotations

import argparse
import datetime as _dt
import hashlib
import json
import os
import re
import sys
from pathlib import Path
from typing import Any, NoReturn

SCHEMA_VERSION = "1.0.0"
TRANCHE_ID = "STC-CODEX-SD-16"
ALLOWED_CHANNELS = ("alpha", "beta", "stable")
RELEASE_NOTES_PATH_PATTERN = re.compile(r"^programs/codex/requirements/[^/]+/release-notes\.md$")
SOURCE_COMMIT_PATTERN = re.compile(r"^[0-9a-f]{40}$")
TAG_PATTERN = re.compile(r"^(alpha|beta|stable)/.+$")


def _fail(msg: str) -> NoReturn:
    print(f"FAIL: {msg}", file=sys.stderr)
    raise SystemExit(2)


def _git_release_notes_hash(repo_root: Path, notes_rel_path: str) -> str:
    """SHA-256 the release-notes file from the current working tree.

    The writer runs inside the publish workflow after `actions/checkout@v4`,
    so the file is present on disk. We hash the bytes literally; the
    manifest's `release_notes_hash` is a content fingerprint, not a
    Git object hash, so the workflow's checkout-of-the-publishing-branch
    is the authoritative source.
    """
    abs_path = repo_root / notes_rel_path
    if not abs_path.is_file():
        _fail(f"release-notes file does not exist on disk: {abs_path}")
    h = hashlib.sha256()
    h.update(abs_path.read_bytes())
    return h.hexdigest()


def _appimage_identity(name: str, path: Path) -> tuple[str, int]:
    if not name:
        _fail("AppImage name is empty")
    if not path.is_file():
        _fail(f"AppImage artifact does not exist on disk: {path}")
    size = path.stat().st_size
    if size < 1:
        _fail(f"AppImage artifact is empty: {path}")
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return (h.hexdigest(), size)


def _build_manifest(args: argparse.Namespace) -> dict[str, Any]:
    # Channel + tag cross-field rule.
    if args.channel not in ALLOWED_CHANNELS:
        _fail(f"channel must be one of {ALLOWED_CHANNELS}, got {args.channel!r}")
    if not TAG_PATTERN.match(args.tag):
        _fail(f"tag {args.tag!r} does not match pattern ^(alpha|beta|stable)/.+$")
    if not args.tag.startswith(f"{args.channel}/"):
        _fail(f"tag prefix {args.tag.split('/', 1)[0]!r} does not equal channel {args.channel!r}")

    # release_notes_path pattern (AV-SCH-6).
    if not RELEASE_NOTES_PATH_PATTERN.match(args.release_notes_path):
        _fail(
            f"release_notes_path {args.release_notes_path!r} does not match "
            "^programs/codex/requirements/<SD>/release-notes.md$"
        )

    # source_commit pattern (40 lowercase hex).
    if not SOURCE_COMMIT_PATTERN.match(args.source_commit):
        _fail(f"source_commit must be 40 lowercase hex chars, got {args.source_commit!r}")

    # workflow path pattern.
    if not re.match(r"^\.github/workflows/[A-Za-z0-9._-]+\.ya?ml$", args.workflow):
        _fail(f"workflow {args.workflow!r} is not a .github/workflows/*.yml path")

    # AppImage identity.
    appimage_sha256, appimage_size = _appimage_identity(args.appimage_name, args.appimage_path)

    # Release-notes hash.
    notes_hash = _git_release_notes_hash(args.repo_root, args.release_notes_path)

    # promotion_lineage.promoted_at: now (UTC, RFC 3339).
    promoted_at = _dt.datetime.now(tz=_dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    manifest: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION,
        "channel": args.channel,
        "version": args.version,
        "tag": args.tag,
        "tranche_id": TRANCHE_ID,
        "source_branch": args.source_branch,
        "source_commit": args.source_commit,
        "release_notes_path": args.release_notes_path,
        "release_notes_url": args.release_notes_url,
        "release_notes_hash": notes_hash,
        "linux_appimage": {
            "name": args.appimage_name,
            "url": args.appimage_url,
            "sha256": appimage_sha256,
            "size_bytes": appimage_size,
        },
        "workflow_provenance": {
            "workflow": args.workflow,
            "run_id": args.run_id,
            "run_attempt": args.run_attempt,
        },
        "eligibility": {
            "min_supported_version": args.min_supported_version,
            "appimage_install": args.appimage_install.lower() == "true",
            "required_install_kind": args.required_install_kind,
        },
        "promotion_lineage": {
            "source_branch": args.source_branch,
            "source_commit": args.source_commit,
            "promoted_at": promoted_at,
        },
        "signature": None,
    }
    return manifest


def _emit(manifest: dict[str, Any], output: Path) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Emit a Codex update-manifest.json that satisfies schemas/update/update-manifest.schema.json."
    )
    parser.add_argument("--repo-root", required=True, type=Path, help="Repo root (where the working tree is checked out).")
    parser.add_argument("--repo", default="electricm0nk/codex", help="GitHub <owner>/<repo> for URL composition.")
    parser.add_argument("--channel", required=True, choices=list(ALLOWED_CHANNELS))
    parser.add_argument("--tag", required=True, help="Schema-conformant `<channel>/<version>-<shortsha>` tag.")
    parser.add_argument("--version", required=True, help="Release version string (e.g. 0.0.0).")
    parser.add_argument("--source-branch", required=True)
    parser.add_argument("--source-commit", required=True, help="Full 40-char SHA-1 of the source commit.")
    parser.add_argument("--release-notes-path", required=True)
    parser.add_argument("--appimage-name", required=True)
    parser.add_argument("--appimage-path", required=True, type=Path, help="Path to the staged AppImage artifact on disk.")
    parser.add_argument("--appimage-url", required=True, help="Canonical URL where the AppImage can be fetched.")
    parser.add_argument("--workflow", required=True, help="Path of the workflow file (e.g. .github/workflows/publish-tester-release.yml).")
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--run-attempt", required=True, type=int)
    parser.add_argument("--release-notes-url", required=True, dest="release_notes_url",
                        help="Canonical URL where the release-notes can be fetched. The workflow computes this from the GitHub release tag (NOT the schema-tag) so the URL stays loadable; the manifest's `tag` field is the schema-conformant `${channel}/${version}-${shortsha}` form.")
    parser.add_argument("--min-supported-version", required=True, dest="min_supported_version")
    parser.add_argument("--appimage-install", required=True, choices=["true", "false"])
    parser.add_argument("--required-install-kind", required=True, dest="required_install_kind", choices=["appimage", "dev", "any"])
    parser.add_argument("--output", required=True, type=Path, help="Destination path for update-manifest.json.")
    args = parser.parse_args(argv)

    if os.environ.get("GITHUB_BASE_REF") and not args.source_commit:
        _fail("source-commit is required when running under GitHub Actions")

    manifest = _build_manifest(args)
    _emit(manifest, args.output)
    print(f"OK: wrote {args.output} (channel={args.channel} tag={args.tag} version={args.version})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
