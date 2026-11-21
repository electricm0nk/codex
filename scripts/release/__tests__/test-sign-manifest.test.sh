#!/usr/bin/env bash
# sign-manifest.sh signs the exact manifest bytes with UPDATE_SIGNING_KEY, refuses an empty key or a
# key whose public half the app does not trust, and never prints the key. Throwaway keys only.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SCRIPT="$ROOT/scripts/release/sign-manifest.sh"
WF="$ROOT/.github/workflows/publish-tester-release.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }
D="$(mktemp -d)"; trap 'rm -rf "$D"' EXIT

openssl genpkey -algorithm ed25519 -out "$D/key.pem" 2>/dev/null
openssl genpkey -algorithm ed25519 -out "$D/other.pem" 2>/dev/null
pub_b64() { openssl pkey -in "$1" -pubout -outform DER | tail -c 32 | base64; }
trusted_list() { echo 'pub const TRUSTED_PUBLIC_KEYS_B64: [&str; 2] = ['; printf '    "%s",\n' "$@"; echo '];'; }
trusted_list "$(pub_b64 "$D/other.pem")" "$(pub_b64 "$D/key.pem")" > "$D/trusted.rs"
printf '{\n  "channel": "beta",\n  "version": "0.22.1"\n}\n' > "$D/update-manifest.json"
cp "$D/update-manifest.json" "$D/original.json"

# 1. A trusted key signs; the signature is 64 raw bytes over the unchanged file; the key never prints.
UPDATE_SIGNING_KEY="$(cat "$D/key.pem")" bash "$SCRIPT" --trusted-keys "$D/trusted.rs" "$D/update-manifest.json" > "$D/out.txt" 2>&1 \
  || fail "signing with a trusted key failed: $(cat "$D/out.txt")"
[ -f "$D/update-manifest.json.sig" ] || fail "no .sig written"
[ "$(wc -c < "$D/update-manifest.json.sig")" -eq 64 ] || fail "signature is not 64 bytes"
cmp -s "$D/update-manifest.json" "$D/original.json" || fail "signing changed the manifest bytes"
openssl pkey -in "$D/key.pem" -pubout -out "$D/pub.pem"
openssl pkeyutl -verify -pubin -inkey "$D/pub.pem" -rawin -in "$D/update-manifest.json" -sigfile "$D/update-manifest.json.sig" >/dev/null \
  || fail "the signature does not verify"
if awk '/PRIVATE KEY/{f=1} END{exit !f}' "$D/out.txt"; then fail "the key was printed"; fi
KEYLINE="$(sed -n 2p "$D/key.pem")"
if awk -v k="$KEYLINE" 'index($0,k){f=1} END{exit !f}' "$D/out.txt"; then fail "key material was printed"; fi
[ -z "$(find "$D" -name '*.key.*' -o -name 'signing-key*')" ] || fail "a key file was left behind"

# 2. One flipped byte no longer verifies (the check above can fail).
printf '{\n  "channel": "stable",\n  "version": "0.22.1"\n}\n' > "$D/tampered.json"
if openssl pkeyutl -verify -pubin -inkey "$D/pub.pem" -rawin -in "$D/tampered.json" -sigfile "$D/update-manifest.json.sig" >/dev/null 2>&1; then
  fail "a tampered manifest verified"
fi

# 3. Empty or unset key: fail, write nothing.
rm -f "$D/update-manifest.json.sig"
for k in "" ; do
  if UPDATE_SIGNING_KEY="$k" bash "$SCRIPT" --trusted-keys "$D/trusted.rs" "$D/update-manifest.json" > "$D/out.txt" 2>&1; then fail "an empty key was accepted"; fi
  grep -q "UPDATE_SIGNING_KEY is empty" "$D/out.txt" || fail "empty-key error must name the secret: $(cat "$D/out.txt")"
done
if env -u UPDATE_SIGNING_KEY bash "$SCRIPT" --trusted-keys "$D/trusted.rs" "$D/update-manifest.json" >/dev/null 2>&1; then fail "an unset key was accepted"; fi
[ ! -f "$D/update-manifest.json.sig" ] || fail "a .sig was written without a key"

# 4. A key the app does not trust: fail before publishing an unverifiable release.
{ trusted_list "$(pub_b64 "$D/other.pem")"; printf '// a test vector outside the array: "%s"\n' "$(pub_b64 "$D/key.pem")"; } > "$D/only-other.rs"
if UPDATE_SIGNING_KEY="$(cat "$D/key.pem")" bash "$SCRIPT" --trusted-keys "$D/only-other.rs" "$D/update-manifest.json" > "$D/out.txt" 2>&1; then
  fail "an untrusted key was accepted"
fi
grep -q "not one the app trusts" "$D/out.txt" || fail "untrusted-key error must say why: $(cat "$D/out.txt")"
[ ! -f "$D/update-manifest.json.sig" ] || fail "a .sig was left for an untrusted key"

# 5. By default the trusted list is the one compiled into the app; it holds exactly two keys.
N="$(awk '/TRUSTED_PUBLIC_KEYS_B64/{on=1} on && /"[A-Za-z0-9+\/]+=",?/{n++} on && /\];/{exit} END{print n+0}' "$ROOT/apps/desktop/src-tauri/src/update/signature.rs")"
[ "$N" -eq 2 ] || fail "signature.rs embeds $N keys, expected 2"

# 6. The workflow signs after the unified manifest is built, publishes the .sig on the release and
#    the update-index mirror, and passes the secret only through the environment.
awk '/name: Build unified update-manifest.json/{b=NR} /sign-manifest\.sh/{s=NR} /name: Create GitHub release/{c=NR} END{exit !(b && s>b && c>s)}' "$WF" \
  || fail "signing must run after the unified manifest is built and before the release is created"
awk '/GLOB=\(/{on=1} on&&/update-manifest\.json\.sig/{f=1} on&&/\)$/{on=0} END{exit !f}' "$WF" || fail "the .sig is not a release asset"
awk '/MIRROR_FILE}\.sig|MIRROR_FILE\}\.sig/{f=1} END{exit !f}' "$WF" || fail "the .sig is not pushed to the update-index mirror"
awk '/UPDATE_SIGNING_KEY: \$\{\{ secrets\.UPDATE_SIGNING_KEY \}\}/{f=1} END{exit !f}' "$WF" || fail "the secret must reach the step as env"
if awk '/echo.*UPDATE_SIGNING_KEY|set -x/{f=1} END{exit !f}' "$WF"; then fail "the workflow may print the key"; fi
echo "PASS"
