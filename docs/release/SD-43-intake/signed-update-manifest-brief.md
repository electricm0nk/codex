# Execution brief: release channels — signed manifest, beta publishing, stable default

Drafted 2026-11-20. Status: READY (both public keys recorded in §15), waits for SD-42 to merge. Operator rulings 2026-11-20 are in §14.
Operator ruling 2026-11-20: this is its OWN SD (not SD-42, which is closing in Epic 5 on tranche-22;
not a card inside SD-43). It takes the next tranche cut after SD-42 merges. The SD number is the
operator's to assign. Three parts: A signing, B beta publishing, C default channel.

## 1. Objective

Part A (signing) is below. Parts B and C are in §11 and §12.

The app must refuse any update whose manifest does not carry a valid Ed25519 signature.
CI signs `update-manifest.json` at publish time. The app checks the signature before it trusts
any sha256 in the manifest. One signature covers AppImage, deb, and Windows.

## 2. Why

Today the manifest and the artifacts come from the same GitHub release. The sha256 check in
each installer only proves the file matches the manifest. It does not prove the manifest is ours.
Re-derive: `grep -n "sha256" apps/desktop/src-tauri/src/update/{appimage,deb,windows}_install.rs`
and `grep -n -i -E "signature|ed25519|minisign" apps/desktop/src-tauri/src/update/*.rs` (expect no verify code).

## 3. Target workdir

`/home/ubuntu/workspace/repos/codex`, in a NEW worktree (`git worktree add`) with its own
`CARGO_TARGET_DIR`. Do not use the shared checkout. Run `git status --porcelain` first.

## 4. Allowed write scope (file level)

- `apps/desktop/src-tauri/Cargo.toml` (add `ed25519-dalek`; no other dependency change)
- `apps/desktop/src-tauri/src/update/signature.rs` (NEW: verify function, embedded public keys)
- `apps/desktop/src-tauri/src/update/mod.rs` (declare module only)
- `apps/desktop/src-tauri/src/update/appimage_install.rs`, `deb_install.rs`, `windows_install.rs`
  (call verify BEFORE parsing hashes; the manifest parse in each file is where trust starts)
- The place that fetches the manifest bytes. Required read first: `apps/desktop/src/update/fetch.ts`
  and `apps/desktop/src/update/indexSource.ts`. If the fetch is TS-side, the brief owner must
  re-scope: the signature bytes must reach Rust unmodified. Stop and report if so.
- `.github/workflows/publish-tester-release.yml` (one signing step, after the unified manifest
  is built; near the step "Build unified update-manifest.json")
- `schemas/update/update-manifest.schema.json` only if `channel`, `version`, `published_at`
  are not already required fields (check first).
- `scripts/release/sign-manifest.sh` or `.py` (NEW) and its test.
- `docs/release/` one page: key storage and rotation runbook.

`FILES YOU OWN` closure check: the signed payload must reach every platform installer; the
`.sig` must be uploaded as a release asset; the channel-index must point at it. Verify each.

## 5. Required reads

1. `AGENTS.md` and `docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit".
2. `apps/desktop/src-tauri/src/update/transaction.rs`, `appimage_install.rs`.
3. `.github/workflows/publish-tester-release.yml` lines near "Build unified update-manifest.json".
4. `schemas/update/update-manifest.schema.json`.
5. `apps/desktop/src/update/fetch.ts`.

## 6. Requirements

- Algorithm: Ed25519, detached signature over the exact manifest bytes as published.
- Signed content must include `channel`, `version`, and a publish timestamp, so an old alpha
  cannot replay as a new stable. Verify they are in the manifest; add if missing.
- Private key: GitHub Actions secret only. Never in the repo, logs, or artifacts.
- Public keys: compiled into the app as a LIST of two (active + spare). Verify accepts either.
  This makes rotation possible without stranding old installs.
- Verify runs BEFORE any hash or URL from the manifest is used. Failure = refuse, with a clear
  error and a logged reason. No "warn and continue" path. No env flag that skips it in release builds.
- Missing `.sig` = refuse (not "unsigned is fine").
- TDD. Failing tests first: valid sig passes; flipped manifest byte fails; wrong key fails;
  missing sig fails; second key passes. Mutation-check each test can fail.

## 7. Open items the operator must decide before dispatch

1. **Transition.** Installs already in the field have no verify code. Do they get ONE final
   unsigned hop to a verifying build? (Recommended: yes, one hop, then enforce.)
2. **Key custody.** Who holds the offline backup of the private key? (Operator.)
3. ~~SD placement~~ Ruled 2026-11-20: own SD, after SD-42 merges.

## 8. Non-goals

- Windows Authenticode / macOS notarization (OS code signing).
- Moving to the Tauri updater plugin.
- Beta channel publishing, default-channel change, version-format change (separate briefs).
- Any change to rollback or install-transaction logic beyond the verify call.

## 9. Verification (acceptance evidence)

- `cd apps/desktop/src-tauri && cargo test update::` green; paste the `test result:` lines and
  attribute each to its `Running` line.
- Full `scripts/verify.sh` at the widest scope (root workspace AND `apps/desktop/src-tauri`).
- CI dry run on a branch: signing step produces `update-manifest.json.sig`; a script check
  verifies it with the public key. Show the command and output.
- Negative proof: tamper one manifest byte, show the app-side verify rejects it.
- The four-check audit from the no-stub doctrine, output in the cycle receipt.
- State what the proof does NOT cover: macOS (no install path seen), and any real-device update.

## 10. Known unchecked at drafting

- Whether the manifest bytes are fetched in TS or Rust (see §4).
- Whether a macOS update path exists. `ls apps/desktop/src-tauri/src/update/` showed none.

## 11. Part B: publish beta from `test`

Today `publish-tester-release.yml` triggers on `develop` and `main` only, and maps every non-main
ref to `alpha` (`grep -n -E "branches:|develop|main|channel=" .github/workflows/publish-tester-release.yml`).
Header line 6 says beta is "reserved".

- Add `test` to the push trigger. Channel `beta`, `prerelease=true`, tag `beta-v<ver>-<sha>`.
  Resolve channel from the ref with an explicit three-way case; an unknown ref must fail, not default to alpha.
- Version: beta and stable must be monotonic per channel and must not collide with alpha tags.
  Decide the stamp (see open item 4). Check `GITHUB_RUN_NUMBER` still gives unique tags across three branches.
- Channel index and manifest: the beta index must be written the same way alpha and stable are
  (`channels/` in tree on `main`; the writer and `apps/desktop/src/update/indexSource.ts` agree on it).
  Re-read both before editing.
- The beta manifest is signed by Part A. No unsigned channel.
- `promotion-gates.yml` resolves only `alpha/` refs today (`grep -n 'alpha' .github/workflows/promotion-gates.yml`).
  Check it against a beta release; extend only if a gate reads the wrong channel.
- Allowed write scope adds: `.github/workflows/publish-tester-release.yml` (trigger + channel step),
  `.github/workflows/promotion-gates.yml` (only if needed), tests under `scripts/release/__tests__`.
- Verification: push to a throwaway `test`-shaped ref is NOT possible (`test` is a protected,
  self-healing gate; never delete or force it). Prove with a workflow dry-run of the channel step
  on all three ref names, and state plainly that a real `test` publish is unproven until the first
  `develop -> test` merge.

## 12. Part C: default channel = stable

New installs default to `stable`. Alpha and beta are opt-in.

- Code: the default is spelled `'alpha'` in `apps/desktop/src/update/updateModel.ts` (lines near 208 and 223),
  `controllerAdapter.ts` (near 214), `diagnostics.ts` (near 65 and 79), `Ui.tsx` (near 82).
  Re-derive: `grep -rn "'alpha'" apps/desktop/src --include=*.ts --include=*.tsx | grep -v -E "test|__fixtures__"`.
  Some hits are type unions or the controlled-defect feedback payload, not defaults. Change only defaults.
- Existing installs that already stored `alpha` keep it. Do not migrate a stored choice.
- Switching DOWN (alpha to stable) must not downgrade. The user waits for the next stable build.
  Check what `eligibility.ts` does today; add a test that pins the rule.
- README line 80 says "Today only `alpha`" works. Update it to the real state after Part B (docs write scope).
- Tests: a fresh profile reads `stable`; a stored `alpha` stays `alpha`; down-switch does not offer a lower version.
- Risk to state: `stable` has few or no builds until `main` is published. A fresh install with no stable
  release must show a clear "no stable release yet" message, not an error. Check `gh release list -R electricm0nk/codex | grep -c '^stable'`.

## 13. Added open items

4. **Version stamp for beta/stable.** Keep `0.<tranche>.<run_number>` for all channels (simple), or
   add a pre-release suffix (`-alpha.N`, `-beta.N`) for correct semver ordering. Recommended: keep the
   current form now; revisit at the first major version.
5. **First stable publish.** Part C is hollow until `main` has a release. Operator gates `develop -> test -> main`.

## 14. Operator rulings (2026-11-20)

1. **Transition: yes.** Installs in the field get exactly ONE final unsigned hop to the first
   verifying build. After that build, verify is enforced with no skip. The brief owner must make the
   hop explicit: which build is the last unsigned one, and how it is recorded (receipt + release notes).
2. **Key custody: operator.** The private key backup lives in the operator's Bitwarden.
   The executor never sees the private key. The operator creates the key pair and sets the
   GitHub Actions secret. The executor writes the runbook and receives only the PUBLIC key.
3. **Version format: unchanged** (`0.<tranche>.<run>` on all channels). Open item 4 is closed.
   Watch item: the operator may call a coming release major version 1. The stamp step and
   `buildVersionTriple` tests must keep the major digit in ONE place so that change is one edit.
   Moving to 1.x does not reset the tranche digit rule; re-read `publish-tester-release.yml`
   "tranche-base" comments before that change. Not part of this SD.

## 15. Key material (public only)

- GitHub Actions secret name: `UPDATE_SIGNING_KEY` (PEM, PKCS8 Ed25519). Operator sets it; the executor never reads it.
- CI signs with: `openssl pkeyutl -sign -rawin -inkey <key.pem> -in update-manifest.json -out update-manifest.json.sig` (64-byte raw signature; OpenSSL 3.x).
- **Active public key** (raw 32 bytes, base64, 2026-11-20): `VCG7pyELsjtD58KkWmmxmaKseuw290Fe2Ley6UDT+uY=`
- **Spare public key** (raw 32 bytes, base64, 2026-11-20): `X5vaJgOFUXoYvfTo60JoQfEhKkfrBxaDSQKou2FXCM4=` — private half in a second Bitwarden note; NOT a GitHub secret. The app accepts either key.
- The app embeds the raw 32 bytes. Test with a fixed vector: sign a known file with a throwaway key, check `ed25519-dalek` accepts it.
