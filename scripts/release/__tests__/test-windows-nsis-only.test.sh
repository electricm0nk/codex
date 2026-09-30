#!/usr/bin/env bash
# The Windows lane builds NSIS only. WiX 3 `light.exe` fails on the
# ~107k-file resource bundle (run 36595679219), so the MSI is not built
# and the stage step must not require one.
set -euo pipefail
WF="$(cd "$(dirname "$0")/../../.." && pwd)/.github/workflows/publish-tester-release.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }

grep -qE 'tauri build --target x86_64-pc-windows-msvc --bundles nsis --ci' "$WF" \
  || fail "windows build must use --bundles nsis only"
grep -qE -- '--bundles msi' "$WF" && fail "windows build must not request msi"
grep -q 'ERROR: No .msi artifact found' "$WF" && fail "stage step must not require an .msi"
echo "PASS"
