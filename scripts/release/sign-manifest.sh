#!/usr/bin/env bash
# Sign update-manifest.json for the app's update check: writes <manifest>.sig, a 64-byte raw Ed25519
# signature over the file's exact bytes. The private key (PEM, PKCS8) comes only from the
# UPDATE_SIGNING_KEY environment variable and is never printed. Fails if the key is empty, or if its
# public half is not one of the keys compiled into the app (a release nobody can install is not
# published). Runbook: docs/release/update-signing-key-runbook.md.
#   usage: sign-manifest.sh [--trusted-keys <file>] <update-manifest.json>
#   --trusted-keys: file holding the base64 public keys as "<key>", lines (tests only);
#                   defaults to the app's apps/desktop/src-tauri/src/update/signature.rs.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TRUSTED="$ROOT/apps/desktop/src-tauri/src/update/signature.rs"
if [ "${1:-}" = "--trusted-keys" ]; then TRUSTED="$2"; shift 2; fi
MANIFEST="${1:?usage: sign-manifest.sh [--trusted-keys <file>] <update-manifest.json>}"
SIG="${MANIFEST}.sig"
fail() { echo "sign-manifest: $*" >&2; rm -f "$SIG"; exit 1; }

[ -f "$MANIFEST" ] || fail "no manifest at $MANIFEST"
[ -n "${UPDATE_SIGNING_KEY:-}" ] || fail "UPDATE_SIGNING_KEY is empty: set the repository secret (see the runbook)"

WORK="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/sign-manifest.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT
( umask 077; printf '%s\n' "$UPDATE_SIGNING_KEY" > "$WORK/k.pem" )

PUB="$(openssl pkey -in "$WORK/k.pem" -pubout -outform DER 2>/dev/null | tail -c 32 | base64)" \
  || fail "UPDATE_SIGNING_KEY is not a readable Ed25519 PEM private key"
[ -n "$PUB" ] || fail "UPDATE_SIGNING_KEY is not a readable Ed25519 PEM private key"
# Only the TRUSTED_PUBLIC_KEYS_B64 array counts; signature.rs also holds throwaway test-vector keys.
if ! awk -v k="\"$PUB\"" '/TRUSTED_PUBLIC_KEYS_B64/{on=1} on&&index($0,k){f=1} on&&/\];/{on=0} END{exit !f}' "$TRUSTED"; then
  fail "the signing key's public half ($PUB) is not one the app trusts ($TRUSTED)"
fi

openssl pkeyutl -sign -rawin -inkey "$WORK/k.pem" -in "$MANIFEST" -out "$SIG" || fail "openssl could not sign $MANIFEST"
[ "$(wc -c < "$SIG")" -eq 64 ] || fail "signature is not 64 bytes"
openssl pkey -in "$WORK/k.pem" -pubout -out "$WORK/pub.pem"
openssl pkeyutl -verify -pubin -inkey "$WORK/pub.pem" -rawin -in "$MANIFEST" -sigfile "$SIG" >/dev/null \
  || fail "the new signature does not verify"
echo "sign-manifest: signed $MANIFEST with trusted key $PUB -> $SIG"
