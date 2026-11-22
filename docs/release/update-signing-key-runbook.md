# Update signing key: custody and rotation runbook

The desktop app installs an update only if that release's `update-manifest.json` carries a valid
Ed25519 signature from one of two public keys compiled into the app. This page is for the operator.
The executor never sees a private key.

## What signs, what verifies

- **Signing (CI).** `.github/workflows/publish-tester-release.yml`, job `finalize`, step
  "Sign update-manifest.json", runs `scripts/release/sign-manifest.sh` on the unified manifest. The
  script reads the private key only from the secret `UPDATE_SIGNING_KEY` (PEM, PKCS8 Ed25519) and runs
  `openssl pkeyutl -sign -rawin -inkey <key.pem> -in update-manifest.json -out update-manifest.json.sig`
  (64 raw bytes). It fails the publish if the secret is empty, is not an Ed25519 key, or its public half
  is not one of the app's two keys. It never prints the key.
- **Where the signature goes.** The release asset `update-manifest.json.sig`, and the `update-index`
  mirror `manifests/<channel>/v<version>-<sha>/update-manifest.json.sig` beside the mirrored manifest.
  All three channels (alpha, beta, stable) are signed; there is no unsigned channel.
- **Verifying (app).** `apps/desktop/src-tauri/src/update/signature.rs`. On Install, the app fetches the
  mirrored manifest bytes and the `.sig`, and verifies the signature over those exact bytes against
  `TRUSTED_PUBLIC_KEYS_B64` (active key first, spare second; either is accepted). Missing, short or bad
  signature = refused with the reason shown; no setting or environment variable skips it.

## Keys (public halves, raw 32 bytes, base64)

| Role | Public key | Private half |
|---|---|---|
| Active | `VCG7pyELsjtD58KkWmmxmaKseuw290Fe2Ley6UDT+uY=` | GitHub secret `UPDATE_SIGNING_KEY` + operator's Bitwarden |
| Spare | `X5vaJgOFUXoYvfTo60JoQfEhKkfrBxaDSQKou2FXCM4=` | operator's Bitwarden (separate note) only; NOT a GitHub secret |

Re-derive the public half of a PEM you hold: `openssl pkey -in key.pem -pubout -outform DER | tail -c 32 | base64`.

## Transition: the first verifying build

Installs already in the field have no verify code, so they install the first verifying build like any
other update (the one unsigned hop the operator ruled on 2026-11-20). From that build on, every update
is verified with no skip.

- **Last build without verify:** the last `0.22.<run>` published before this branch merges to `develop`
  (at the time of writing, alpha `0.22.158`; re-derive with
  `gh release list -R electricm0nk/codex -L 5`).
- **First verifying build:** the next `0.22.<run>` the publish workflow produces after the merge. Its run
  number is not known until it publishes. Record it here and in that release's notes when it exists:
  `First verifying build: 0.22.<run> (<release tag>)`.
- Before that merge, the secret `UPDATE_SIGNING_KEY` must be set, or the first publish fails at
  "Sign update-manifest.json" (by design: no unsigned release ships).

## Routine checks

- Verify a published release by hand:
  ```
  gh release download <release-tag> -R electricm0nk/codex -p 'update-manifest.json*'
  printf '%s' 'VCG7pyELsjtD58KkWmmxmaKseuw290Fe2Ley6UDT+uY=' | base64 -d > raw.pub
  ( printf '302a300506032b6570032100' | xxd -r -p; cat raw.pub ) > pub.der
  openssl pkeyutl -verify -pubin -keyform DER -inkey pub.der -rawin -in update-manifest.json -sigfile update-manifest.json.sig
  ```
  Expect `Signature Verified Successfully`.

## Rotation (active key retired or exposed)

The spare exists so a rotation never strands installed copies: every shipped app already trusts it.

1. Take the spare private key from Bitwarden. Set it as the GitHub secret `UPDATE_SIGNING_KEY`
   (repository Settings, Secrets and variables, Actions). Nothing else changes for CI.
2. Publish. Every installed copy accepts the spare-signed manifest.
3. Make a new spare: `openssl genpkey -algorithm ed25519 -out new-spare.pem`. Store it in Bitwarden.
   Derive its public half (command above).
4. In `signature.rs`, set `TRUSTED_PUBLIC_KEYS_B64` to `[<old spare, now active>, <new spare>]` and update
   the test `the_embedded_keys_are_the_two_published_public_keys` and the table above. Ship it as an
   ordinary build. Delete `new-spare.pem` from disk once it is in Bitwarden.
5. If the old active key was exposed: until the build from step 4 has reached users, installed copies
   still trust it. Step 4 is urgent in that case.

If both private keys are lost, no installed copy can be updated by the app; users must reinstall from
a release download. Keep both Bitwarden notes backed up.
