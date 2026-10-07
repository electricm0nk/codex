#!/usr/bin/env python3
"""
Tests for tools/release/verify_release_consistency.py.

Each case is a defect that shipped (or nearly shipped) in alpha v0.16.140: a binary that said
0.16.0 while the manifest said 0.16.140, and release notes whose URL could never match their hash.

Run: python3 -m pytest tools/release/test_verify_release_consistency.py -v
"""

from __future__ import annotations

import hashlib
import http.server
import json
import shutil
import subprocess
import sys
import tempfile
import threading
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
GATE = REPO_ROOT / "tools" / "release" / "verify_release_consistency.py"

VERSION = "0.16.141"
NOTES_REL = "docs/release/SD-99-example/release-notes.md"
NOTES = b"## Summary\n\nreal notes\n"
HAVE_DPKG = shutil.which("dpkg-deb") is not None


def _sha(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


class _Handler(http.server.BaseHTTPRequestHandler):
    body = NOTES

    def do_GET(self) -> None:  # noqa: N802
        self.send_response(200)
        self.end_headers()
        self.wfile.write(type(self).body)

    def log_message(self, *args) -> None:
        pass


def _serve(body: bytes):
    handler = type("H", (_Handler,), {"body": body})
    server = http.server.HTTPServer(("127.0.0.1", 0), handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server, f"http://127.0.0.1:{server.server_port}/notes.md"


def _build_deb(path: Path, version: str, package: str = "codex") -> None:
    pkg = path.with_suffix("")
    (pkg / "DEBIAN").mkdir(parents=True)
    (pkg / "DEBIAN/control").write_text(
        f"Package: {package}\nVersion: {version}\nArchitecture: all\nMaintainer: t\nDescription: t\n"
    )
    subprocess.run(["dpkg-deb", "--build", str(pkg), str(path)], check=True, capture_output=True)


def _make(*, app_version=VERSION, tauri_version=VERSION, deb_version=VERSION, notes_hash=None,
          appimage_name=None, deb_sha=None, appimage_sha=None, nsis=None) -> tuple[Path, Path]:
    # nsis: None (no Windows installer) or a dict overriding name/sha/size/staged for the windows_nsis block.
    root = Path(tempfile.mkdtemp(prefix="consistency-"))
    (root / "apps/desktop/src-tauri").mkdir(parents=True)
    (root / "apps/desktop/package.json").write_text(json.dumps({"version": app_version}))
    (root / "apps/desktop/src-tauri/tauri.conf.json").write_text(json.dumps({"version": tauri_version}))
    notes = root / NOTES_REL
    notes.parent.mkdir(parents=True)
    notes.write_bytes(NOTES)
    staging = root / "release-staging"
    staging.mkdir()
    deb = staging / f"Codex_{VERSION}_amd64.deb"
    _build_deb(deb, deb_version)
    appimage = staging / f"Codex_{VERSION}_amd64.AppImage"
    appimage.write_bytes(b"appimage")
    manifest = {
        "version": VERSION,
        "release_notes_path": NOTES_REL,
        "release_notes_hash": notes_hash or _sha(NOTES),
        "release_notes_url": "REPLACED",
        "linux_appimage": {"name": appimage_name or appimage.name, "sha256": appimage_sha or _sha(b"appimage"), "size_bytes": 8, "url": "u"},
        "linux_deb": {"name": deb.name, "sha256": deb_sha or _sha(deb.read_bytes()), "size_bytes": deb.stat().st_size, "url": "u"},
    }
    if nsis is not None:
        name = nsis.get("name", f"Codex_{VERSION}_x64-setup.exe")
        payload = b"nsis-installer"
        if nsis.get("staged", True):
            (staging / name).write_bytes(payload)
        manifest["windows_nsis"] = {
            "name": name,
            "sha256": nsis.get("sha256", _sha(payload)),
            "size_bytes": nsis.get("size_bytes", len(payload)),
            "url": "u",
        }
    (staging / "update-manifest.json").write_text(json.dumps(manifest))
    return root, staging


def _run(root: Path, staging: Path, *extra: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(GATE), "--repo-root", str(root), "--staging", str(staging), *extra],
        capture_output=True, text=True,
    )


@unittest.skipUnless(HAVE_DPKG, "dpkg-deb is required to build a real .deb fixture")
class ConsistencyGateTests(unittest.TestCase):
    def test_an_appimage_whose_bytes_do_not_match_the_manifest_fails(self) -> None:
        # AppImage self-update downloads the file the manifest describes and refuses a hash mismatch,
        # so a release whose manifest hash is wrong could never be installed.
        root, staging = _make(appimage_sha="0" * 64)
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("linux_appimage sha256", result.stderr)

    def test_a_consistent_windows_installer_passes(self) -> None:
        root, staging = _make(nsis={})
        result = _run(root, staging)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_a_windows_installer_whose_hash_differs_from_the_manifest_fails(self) -> None:
        root, staging = _make(nsis={"sha256": "1" * 64})
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("windows_nsis sha256", result.stderr)

    def test_a_windows_installer_whose_size_differs_from_the_manifest_fails(self) -> None:
        root, staging = _make(nsis={"size_bytes": 999})
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("windows_nsis size", result.stderr)

    def test_a_windows_installer_that_is_not_staged_fails(self) -> None:
        root, staging = _make(nsis={"staged": False})
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("windows_nsis", result.stderr)
        self.assertIn("not staged", result.stderr)

    def test_a_windows_installer_named_for_another_version_fails(self) -> None:
        root, staging = _make(nsis={"name": "Codex_0.16.0_x64-setup.exe"})
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("windows_nsis name", result.stderr)

    def test_a_consistent_release_passes(self) -> None:
        root, staging = _make()
        result = _run(root, staging)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_binary_version_that_never_took_the_stamp_fails(self) -> None:
        # v0.16.140: package.json / tauri.conf.json still said 0.16.0 while the manifest said 0.16.140.
        root, staging = _make(app_version="0.16.0", tauri_version="0.16.0")
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("apps/desktop/package.json", result.stderr)
        self.assertIn("0.16.0", result.stderr)

    def test_tauri_conf_version_mismatch_fails(self) -> None:
        root, staging = _make(tauri_version="0.16.0")
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tauri.conf.json", result.stderr)

    def test_deb_whose_package_version_differs_from_the_manifest_fails(self) -> None:
        root, staging = _make(deb_version="0.16.0")
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Version", result.stderr)

    def test_deb_hash_that_does_not_match_the_manifest_fails(self) -> None:
        root, staging = _make(deb_sha="0" * 64)
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sha256", result.stderr)

    def test_appimage_name_without_the_version_fails(self) -> None:
        root, staging = _make(appimage_name="Codex_0.16.0_amd64.AppImage")
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("AppImage", result.stderr)

    def test_notes_hash_that_does_not_match_the_notes_file_fails(self) -> None:
        root, staging = _make(notes_hash="1" * 64)
        result = _run(root, staging)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("release_notes_hash", result.stderr)

    def test_fetched_notes_url_bytes_must_hash_to_the_manifest_hash(self) -> None:
        root, staging = _make()
        server, url = _serve(NOTES)
        try:
            manifest = json.loads((staging / "update-manifest.json").read_text())
            manifest["release_notes_url"] = url
            (staging / "update-manifest.json").write_text(json.dumps(manifest))
            self.assertEqual(_run(root, staging, "--fetch-notes").returncode, 0)
        finally:
            server.shutdown()

    def test_a_notes_url_serving_an_html_page_fails(self) -> None:
        # v0.16.140: release_notes_url was the GitHub release HTML page, never the hashed markdown.
        root, staging = _make()
        server, url = _serve(b"<html>release page</html>")
        try:
            manifest = json.loads((staging / "update-manifest.json").read_text())
            manifest["release_notes_url"] = url
            (staging / "update-manifest.json").write_text(json.dumps(manifest))
            result = _run(root, staging, "--fetch-notes")
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("does not hash to release_notes_hash", result.stderr)
        finally:
            server.shutdown()

    def test_a_release_with_no_deb_is_checked_without_the_deb_rules(self) -> None:
        root, staging = _make()
        manifest = json.loads((staging / "update-manifest.json").read_text())
        del manifest["linux_deb"]
        (staging / "update-manifest.json").write_text(json.dumps(manifest))
        self.assertEqual(_run(root, staging).returncode, 0)


if __name__ == "__main__":
    unittest.main()
