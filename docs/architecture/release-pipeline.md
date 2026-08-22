# Release pipeline

> Scope: how a commit on `develop` or `main` becomes a tagged, schema-validated tester release, and how branches get promoted between channels.
> Last verified: 2026-08-11 against tranche/9 (SD-29 Epic 9, Build Version Numbering — the §Version stamp section was re-derived in full; the rest of this document was last verified 2026-07-22 against tranche/5-3 at SD-25 closure and its line numbers may have drifted). **Path correction
> 2026-08-22** (SD-32 closure epilogue): the `check-release-manifest.yml` `paths:` filter note
> corrected — `sd16`/`sd17` no longer resolve either (see the item's own updated text below); no
> other content in this doc re-verified.
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Overview

Two independent systems cooperate here:

1. **Publish**: `.github/workflows/publish-tester-release.yml` turns a push to `develop` or `main` into a multi-platform GitHub Release with a schema-validated update manifest, and (conditionally) advances the `update-index` branch's channel pointer.
2. **Promotion**: a chain of branch-source guards and an evidence gate control which branch may open a PR into which downstream branch (`develop` → `test` → `main`), independent of the publish workflow.

These systems share doctrine constants (tranche id, release-notes path, required sections) but are enforced by separate code paths that must be kept in sync by hand — see [The pinned-SD-16 quirk](#the-pinned-sd-16-quirk) below.

## The publish pipeline (`publish-tester-release.yml`)

Trigger: `push` to `develop` or `main` (`.github/workflows/publish-tester-release.yml:8-12`). `develop` publishes to the `alpha` channel (prerelease); `main` publishes to `stable`; `beta` is reserved — no workflow trigger publishes it today (line 6).

Job graph:

```
stamp ──┬──┬─→ publish-tester-release (linux)   ──┐
test  ──┘  ├─→ publish-tester-release-macos      ─┼─→ finalize
           └─→ publish-tester-release-windows    ─┘
(finalize's needs: also lists stamp and test directly, in addition to the three publish jobs)
```

`stamp` and `test` declare no `needs:` of their own — they run in parallel
(`test` does its own fresh `actions/checkout`; it does not consume the
stamped sources). Every downstream job fans in from both.

- **`stamp`** (`publish-tester-release.yml:18-122`): checks out, derives `VERSION="0.9.${GITHUB_RUN_NUMBER}"` (line 97, re-derived 2026-08-11), and rewrites `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` in place. The stamped files are uploaded as the `stamped-sources` artifact so every downstream job (including the three platform builds) reads the exact same version — this replaced an earlier design where each platform job re-derived its own version and could disagree (see the `SD16-F-WINDOWS` comments at lines 22-26, 41-57).
- **`test`** (lines 98-144): `cargo test --locked` at repo root, `cargo test --locked` in `apps/desktop/src-tauri`, `npm run typecheck` and `npm test` in `apps/desktop`. All three platform-publish jobs `needs: [test, stamp]`, so a red test run blocks every artifact build.
- **`publish-tester-release`** (linux, lines 146-387): downloads the stamped sources, runs `npx tauri build --bundles deb,appimage --ci`, stages the `.deb`/`.AppImage` into `release-staging/`, writes a `provenance.json` receipt, generates and validates `update-manifest.json`, computes checksums, and uploads everything as the `platform-linux` artifact. It does **not** call `gh release create` itself.
- **`publish-tester-release-macos`** (lines 396-463) and **`publish-tester-release-windows`** (lines 473-570) mirror this on `macos-latest` / `windows-latest`, building `.app`/`.dmg` and `.msi`/`.exe` respectively, uploading `platform-macos` / `platform-windows` artifacts. Neither is code-signed yet (macOS DMG ships unsigned; Windows testers click through SmartScreen — see comments at lines 389-395, 506-508).
- **`finalize`** (lines 586-946) is the single writer of the GitHub release, the unified `update-manifest.json`, and the `update-index` branch push (comment block at lines 580-585). It downloads whichever platform artifacts succeeded (`continue-on-error: true` per download, lines 606-620 — a missing platform does not fail the run), rebuilds a unified manifest with whichever optional platform blocks are present, validates it twice, creates the GitHub release with `gh release create`, then emits and pushes the channel index.

### Version stamp

`VERSION="0.9.${GITHUB_RUN_NUMBER}"` (`publish-tester-release.yml:97`) is the sole place the build number is minted; every other consumer reads `needs.stamp.outputs.version`. The three files that must carry a matching `<major>.<tranche-base>.<build>` triple are `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, and `apps/desktop/src-tauri/Cargo.toml` — as of this verification all three are committed at `0.9.0` (confirmed via `node -p "require('./apps/desktop/package.json').version"`, `node -p "require('./apps/desktop/src-tauri/tauri.conf.json').version"`, and `grep -n '^version' apps/desktop/src-tauri/Cargo.toml`; `apps/desktop/src-tauri/Cargo.lock`'s `codex-desktop` entry carries the same triple and must be moved with them). The workflow's in-line comment describing "the repo keeps `0.<tranche>.0` as the committed placeholder" is accurate in shape: the three files are kept at the tranche's `.0` placeholder and the stamp step overwrites `package.json` / `tauri.conf.json` at publish time with the real build number (it does not touch `Cargo.toml`).

Versioning semantics (`docs/release/SD-22/decisions.md:52`, `docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §14`, and `apps/desktop/src/release/buildVersionTriple.test.ts`):
- **major**: stays `0` until the first publish to `main`.
- **tranche-base** (the `9` in `0.9.x`): bumped only when a new `tranche/N` branch is cut for the next bundle — explicitly *not* at a bundle's own closure while still on the same tranche branch. The test's anchor comment records that an SD-22 Epic 7 cycle bumped this to `0.6` in error and it was reverted (also called out in the workflow comment above the stamp step). Advances to date: `0.5` (tranche/5) → `0.7` (tranche/7, SD-27) → `0.8` (tranche/8, SD-28) → `0.9` (tranche/9, SD-29 Epic 9, 2026-08-11).
- **build**: the monotonic `GITHUB_RUN_NUMBER`.

The build label surfaced in the desktop UI is `Codex <version>` — `formatWorkbenchBuildLabel` in `apps/desktop/src/testerWorkbench/status/createWorkbenchStatus.ts:72-73` (`BUILD_PREFIX = 'Codex'` at line 61; both the function and the file were renamed from `formatSd11WorkbenchBuildLabel` / `sd11/status/createSd11WorkbenchStatus.ts` by SD-25 criterion 1.1).

Guard tests that keep the three files and the fixtures honest:
- `apps/desktop/src/release/buildVersionTriple.test.ts` (the fuller original, formerly `sd21/`; renamed to `release/` by SD-29's function-based-naming sweep) — asserts `package.json`, `tauri.conf.json`, and `Cargo.toml` versions are identical and match `^\d+\.\d+\.\d+$`, that the triple starts with `0.9.` on tranche/9, **and** that the workflow's stamp reuses the repo files' own `major.tranche` while taking its build position from `GITHUB_RUN_NUMBER` (a relationship check, not two independent literals) with no hardcoded triple anywhere in the stamp.
- `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts` — an SD-28-era partial duplicate of the above (the file-agreement + tranche-anchor half only; no workflow-stamp checks). Both anchors are moved together at each tranche cut. Deduping the pair is deferred to SD-29 Epic 10 (`docs/retro/events/sd29-e9-version.jsonl`).
- `apps/desktop/src/releaseChecks/buildLabelFixtureFreshness.test.ts` — asserts three named fixture files (`apps/desktop/src/testerWorkbench/loadTesterWorkbenchSurface.test.ts`, `apps/desktop/src/testerWorkbench/status/createWorkbenchStatus.test.ts`, `apps/desktop/src/testSupport/makeSurface.ts`) carry the *current* `Codex <version>-test` label literal and not the specific prior-bump literal, `Codex 0.8.0-test` as of the tranche/9 cut. **This is the sweep the version bump is easy to miss:** the freshness test names only three files, but **seven** `src/**` files carry `<version>-test` literals (12 occurrences) and all seven move with the triple — `git grep -l '0\.9\.0-test' -- apps/desktop/src` (7 files; occurrence count cross-checked with `awk '{n+=gsub(/0\.9\.0-test/,"")} END{print n}'` → 12, since `grep -o` is not trustworthy here per `AGENTS.md` §Concurrency and Measurement).

### Manifest generation + dual validation

`write_release_manifest.py` (`scripts/release/write_release_manifest.py`) builds `update-manifest.json` against `schemas/update/update-manifest.schema.json`. It hard-codes `TRANCHE_ID = "STC-CODEX-SD-16"` (line 58) and `SCHEMA_VERSION = "1.1.0"` (line 57), computes the AppImage's sha256/size from the file on disk (`_appimage_identity`, lines 89-101), and — since `SD16-F-WINDOWS` — accepts complete-triple-or-nothing `--windows-msi-*` / `--macos-dmg-*` flag sets (`_optional_platform_block`, lines 146-179) so a partial platform block can never be emitted.

Each publish job's manifest is checked twice, by two different scripts:
1. `scripts/release/validate_manifest.py --manifest update-manifest.json --schema schemas/update/update-manifest.schema.json` — pure `jsonschema.Draft202012Validator` check against the wire schema (`publish-tester-release.yml:326-334`).
2. `tools/release/check_release_manifest_against_dev_schema.py update-manifest.json` — re-validates against the same schema, then re-runs `tools/release/check_release_manifest.py`'s `_coherence_check` (tranche_id / release_notes_path binding) against the manifest (`publish-tester-release.yml:336-341`; the script itself explains why it exists at `tools/release/check_release_manifest_against_dev_schema.py:8-12` — `tools/release/check_release_manifest.py` normally validates the *legacy* `tools/release/release-manifest.schema.json` shape, not the dev `schemas/update/` shape).

The `finalize` job repeats both validations against the unified manifest (`publish-tester-release.yml:747-754`).

### Tag forms

Two tag strings coexist by design (comment at `publish-tester-release.yml:266-273`):

| Form | Shape | Used by |
|---|---|---|
| `MANIFEST_TAG` | `${channel}/v${VERSION}-${SHORT_SHA}` (e.g. `alpha/v0.5.96-a1b2c3d4`) | Satisfies `schemas/update/update-manifest.schema.json`'s `tag` pattern `^(alpha\|beta\|stable)/.+$`; stored as the manifest's `tag` field and as the mirror path under `manifests/<MANIFEST_TAG>/` on `update-index`. |
| `RELEASE_TAG` | `${channel}-v${VERSION}-${SHORT_SHA}` (e.g. `alpha-v0.5.96-a1b2c3d4`) | The actual `gh release create` tag and the URL path segment (GitHub tags cannot contain `/` without becoming a nested ref, so the slash is replaced with a hyphen for the release tag specifically). |

Both are computed twice, identically: in the linux publish job (lines 283-284) and in `finalize`'s `resolve` step (lines 636-637) — `finalize`'s computation is the one the actual `gh release create` uses.

### Channel-index emit + push

After `gh release create` succeeds, `finalize` prepares and pushes a channel-index pointer to the **protected `update-index` branch**:

- `tools/release/emit_channel_index.py` reads a schema-valid `update-manifest.json`, validates it, cross-checks `manifest.channel == args.channel`, and emits `channels/<channel>.json` validated against `schemas/update/channel-index.schema.json` (lines 91-107).
- The `Update channel index on update-index branch` step (`publish-tester-release.yml:904-945`) does a hard reset + fetch/checkout (or orphan-create if the branch doesn't exist yet), writes the channel-index JSON and a full manifest mirror under `manifests/<MANIFEST_TAG>/update-manifest.json`, commits as `github-actions[bot]`, and pushes `HEAD:update-index`.

### The fail-loud gate

Both the emit and push steps are gated by `hashFiles('docs/release/SD-16/tranche-*/manifest.yaml') != ''` (lines 793, 816, 905). If no file matches that glob, a dedicated step fails the whole job loudly instead of letting the channel-index steps silently no-op:

```yaml
- name: Assert channel-index gate preconditions
  if: success() && github.event_name == 'push' && (github.ref_name == 'develop' || github.ref_name == 'main') && hashFiles('docs/release/SD-16/tranche-*/manifest.yaml') == ''
  run: |
    echo "::error::channel-index: no tranche manifest matched docs/release/SD-16/tranche-*/manifest.yaml — the channel-index steps would be silently skipped and the update-index branch would never move. Failing loudly instead."
    exit 1
```
(`publish-tester-release.yml:792-796`)

## Branch promotion chain

```
feature/*  →  develop (alpha)  →  test (beta)  →  main (stable)
```

Enforcement is layered:

1. **GitHub-side branch protection** (not visible in-repo except as the documented intent in `.github/branch-protection-rulesets/`) blocks direct pushes to the protected branches.
2. **`allow-only-*` workflows** are the second enforcement layer, run on `pull_request_target`:
   - `.github/workflows/allow-only-develop-into-test.yml` — PRs into `test` must come from this repo's `develop` branch (not a same-named fork branch). Also restores `develop` from `test` if `develop` is ever deleted (lines 47-88).
   - `.github/workflows/allow-only-test-into-main.yml` — PRs into `main` must come from `test`; symmetric `restore-test-branch` job.
   - Both delegate the actual check to `bash tools/ci/branch-promotion-guard.sh` via `EXPECTED_SOURCE`/`SOURCE_BRANCH`/`HEAD_REPO`/`BASE_REPO` env vars (`allow-only-develop-into-test.yml:39-45`).
3. **`tools/ci/branch-promotion-guard.sh`** defines `verify_promotion_source()` (lines 29-47): rejects when `head_repo != base_repo` (forks) or `source_branch != expected`. It is sourceable (for unit tests) or directly runnable (as the Action step body). Unit tests live at `tests/sd16-e5-f1/test_branch_promotion_guard.sh`; the guard script's own header states: "Both the GitHub Actions workflows and the unit tests MUST exercise the same `verify_promotion_source` function. Drift between this file and the workflows fails the test suite." (`branch-promotion-guard.sh:17-19`).
4. **`.github/workflows/promotion-gates.yml`** — runs on `pull_request_target` into `test` or `main`, and is the evidence-rich self-blocking gate:
   - Determines the lane (`test` → `beta`, `main` → `stable`, lines 52-70).
   - Runs `python3 scripts/release/check_promotion_evidence.py --self-test` first (lines 72-79) — the CI job fails immediately if the checker's own built-in test suite is red, before it ever evaluates a real PR.
   - Fetches the PR body via REST (line 81-98, because `pull_request_target`'s event payload can truncate long bodies), resolves the most recent alpha/beta release via REST, and (stable lane only) downloads `provenance.json` from the most recent beta release's assets.
   - Runs `python3 scripts/release/check_promotion_evidence.py --lane <beta|stable> ...` (lines 158-214) and captures `gate_report.txt`.
   - Posts/updates a single PR comment (marker `<!-- promotion-gate-evidence -->`, lines 241-281) and sets a commit status with context `sd16-e5-f3a/promotion-gate` (lines 283-303) — this is what branch protection is expected to require.
   - Fails the job (`exit 1`) when the gate is blocked (lines 305-310).
5. **`.github/workflows/check-release-manifest.yml`** — a PR-time gate, scoped by `paths:` filters (`tools/release/**`, `apps/desktop/src/sd11/update/**`, `apps/desktop/src/sd15/**`, `apps/desktop/src/sd16/**`, `apps/desktop/src/sd17/**`, `release-manifest.json`, `docs/release/**/release-notes.md`, and the two workflow files themselves — lines 24-32). If a `release-manifest.json`-shaped file changed, it runs `tools/release/check_release_manifest.py` against every changed manifest and posts a failure-summary comment on failure (lines 94-116). **Latent gap, now wider:** SD-25 criterion 1.1 renamed the `sd11/` frontend directory to `testerWorkbench/` and `sd15/` to `operatorTriage/`, and a later sweep (`06d926e90`, 2026-08-10) moved the old sd16/feedback and sd16/update subdirectories up one level to `apps/desktop/src/feedback/` and `apps/desktop/src/update/`; the sd17 directory was never real in this checkout (`find apps/desktop/src -maxdepth 1 -iname 'sd17*'` finds nothing). None of the four globs (`sd11/update/**`, `sd15/**`, `sd16/**`, `sd17/**`) match a real path today — this workflow's entire `paths:` filter has gone stale. `check-release-manifest.yml`'s YAML itself is untouched by this correction (out of `docs/architecture/`'s own scope); this note only corrects the doc's prior claim that `sd16`/`sd17` "still resolve".
6. **`.github/workflows/tranche-3-ci.yml`** — **tranche/3-specific**, not a generic template. Guards that slice PRs target `tranche/3` (never `develop`, per the header comment's "devops/tranche-branch-governance refusal", lines 3-19), runs the same test+typecheck+test lane as the publish workflow's `test` job on every push/PR to `tranche/3` (lines 54-99), and validates any touched `docs/release/**/manifest.yaml` or `release-notes.md` via `check_release_manifest.py` (lines 101-151). `.github/branch-protection-rulesets/README.md`'s "To add a new tranche" procedure and the `_note` fields inside `tranche-2-7.json`'s status-check rules (lines 51, 56 — "The .github/workflows/tranche-2-7-ci.yml file (parallel to tranche-3-ci.yml) has not been authored yet") both establish that later tranches are expected to get their own parallel `<tranche>-ci.yml`; none has been authored for `tranche/5` either.

### `scripts/release/promote-alpha-to-beta.sh` / `promote-beta-to-stable.sh`

Local, human-run helpers (not invoked by any workflow) that evaluate the same doctrine gates as `check_promotion_evidence.py` but against real `gh` calls, and print (or write to `--body-out`) a ready-to-paste PR body carrying the `tranche_id:` / `release_notes_path:` / evidence keys the CI gate expects. Both source `scripts/release/_lib-gates.sh` for shared helpers (`validate_release_notes`, `known_issues_has_marker`, `release_url_for_tag`, `is_valid_evidence`, `emit_pr_body`). Neither script ever calls `gh pr create` — confirmed by `scripts/release/test-promotion-gates.test.sh`'s final assertion (`test-promotion-gates.test.sh:337-341`), which greps a full log of every `gh` invocation across the suite for the literal `pr create` and fails if found.

## The release-notes CI contract

`release_notes_path` is regex-locked in two independent schemas, which must be kept in agreement by hand:

- `schemas/update/update-manifest.schema.json:69` — `"pattern": "^docs/release/[^/]+/release-notes\\.md$"`
- `tools/release/release-manifest.schema.json:23` — `"pattern": "^docs/release/.+/release-notes\\.md$"` (looser: allows nested subdirectories under `docs/release/`, where the update-manifest schema requires exactly one path segment).

The seven required release-notes section headers are asserted in `tools/release/check_release_manifest.py`'s `REQUIRED_NOTES_SECTIONS` (lines 35-43):

```python
REQUIRED_NOTES_SECTIONS = [
    "Summary",
    "User-Visible Changes",
    "Defects Fixed",
    "Operational Notes",
    "Verification Evidence",
    "Known Issues",
    "Update Eligibility",
]
```

The same seven headers (as literal `## `-prefixed strings, and order-checked) are independently re-declared in `scripts/release/check_promotion_evidence.py:82-90` (`REQUIRED_NOTE_SECTIONS`) and `scripts/release/_lib-gates.sh:15-23` (`REQUIRED_NOTE_SECTIONS` bash array) — three separate lists that must stay in sync by convention, not by shared import (the Python promotion-gate checker and the schema-side checker are deliberately different modules; see `check_promotion_evidence.py:1-56`'s "stdlib only" note).

## The pinned-SD-16 quirk

Several pipeline surfaces are still pinned to frozen SD-16-era identifiers even though newer bundles (SD-17 through SD-22, tranche/5) exist. This is the manifest contract's frozen identity — intentional, not an oversight to "fix":

1. **`docs/release/SD-16/release-notes.md` hardcoded as the publish workflow's notes source.** `publish-tester-release.yml` reads/writes this exact path at multiple steps: the `Validate tranche release notes` step (line 213), the manifest-generation `--release-notes-path` argument (line 298), the `Stage tranche release notes` copy (line 362), the `finalize` job's manifest rebuild (lines 719, 721, 734), and the release-notes fallback when creating the GitHub release (line 775). Every tester release published today ships the SD-16 release-notes file regardless of which SD's code actually changed.
2. **`tranche_id` is a JSON Schema `const` locked to `"STC-CODEX-SD-16"`.** `schemas/update/update-manifest.schema.json:53` (`"const": "STC-CODEX-SD-16"`, with an explicit doc-comment "Locked... for the duration of this contract") and `schemas/update/channel-index.schema.json:60` both enforce this; `scripts/release/write_release_manifest.py:58` emits exactly that constant (`TRANCHE_ID = "STC-CODEX-SD-16"`).
3. **`codex-tranche-2-5` is a separate pinned constant inside the promotion-gate surface** (distinct from the manifest's `STC-CODEX-SD-16`): `scripts/release/check_promotion_evidence.py:78` (`TRANCHE_ID = "codex-tranche-2-5"`) and `scripts/release/_lib-gates.sh:11` (`TRANCHE_ID="codex-tranche-2-5"`) both gate the promotion-evidence PR-body and manifest checks against this literal string, independent of the update-manifest schema's `STC-CODEX-SD-16` pin.

These three pins are consistent with each other only in the sense that they all currently point at old identifiers; they are not the *same* identifier, and nothing in the codebase currently derives one from another. A future contract bump that changes any of the three needs to touch every file listed above plus its corresponding test fixtures (`scripts/release/test-promotion-gates.test.sh`, `scripts/release/__tests__/fixtures/`, `scripts/release/check_promotion_evidence.py`'s embedded `_t_*` self-tests).

## Scripts and tools inventory

| File | What it does | Invoked by |
|---|---|---|
| `scripts/release/_lib-gates.sh` | Shared bash helpers (`validate_release_notes`, `known_issues_has_marker`, `release_url_for_tag`, `is_valid_evidence`, `emit_pr_body`, `report_gate_outcome`, `deliver_body`) for the two promote-*.sh scripts. | Sourced by `promote-alpha-to-beta.sh`, `promote-beta-to-stable.sh`. |
| `scripts/release/check_promotion_evidence.py` | CI-side evidence validator for the beta/stable promotion gates; emits JSON report + `GATE=ready\|blocked`. Has a built-in `--self-test` harness. | `.github/workflows/promotion-gates.yml`. |
| `scripts/release/promote-alpha-to-beta.sh` | Local helper: evaluates the 5 alpha→beta gates against real `gh` state and prints/writes the AV-BR-6 PR body. Never calls `gh pr create`. | Run manually by an operator. |
| `scripts/release/promote-beta-to-stable.sh` | Local helper: evaluates the 6 beta→stable gates (including provenance.json download) and prints/writes the PR body. | Run manually by an operator. |
| `scripts/release/validate_manifest.py` | Validates an `update-manifest.json` against `schemas/update/update-manifest.schema.json` via `jsonschema`. | `publish-tester-release.yml` (linux publish job and `finalize`). |
| `scripts/release/write_release_manifest.py` | Builds and writes a schema-conformant `update-manifest.json`, computing AppImage/MSI/DMG sha256+size from disk. | `publish-tester-release.yml` (linux publish job and `finalize`). |
| `scripts/release/test-promotion-gates.test.sh` | Bash self-test for `promote-alpha-to-beta.sh` / `promote-beta-to-stable.sh` against a stubbed `gh`. | Run manually (`bash scripts/release/test-promotion-gates.test.sh`); not wired into any workflow found in `.github/workflows/`. |
| `scripts/release/__tests__/test-write-release-manifest.test.sh` | Bash self-test for `write_release_manifest.py` / `validate_manifest.py` round-trip, including a malformed-sha256 negative case. | Run manually (`bash scripts/release/__tests__/test-write-release-manifest.test.sh`). |
| `scripts/tranche/validate-tranche-notes.py` | Validates a tranche manifest YAML + its bound release-notes.md (required sections, order, non-empty). | `publish-tester-release.yml`'s `Validate tranche release notes` step (linux publish job, line 213). |
| `scripts/tranche/tests/test_validate_tranche_notes.py` | `unittest`-based test suite for `validate-tranche-notes.py` (9 cases as run). | Run manually (`python3 scripts/tranche/tests/test_validate_tranche_notes.py`). |
| `tools/ci/branch-promotion-guard.sh` | Defines `verify_promotion_source()`; sourceable for tests or directly runnable as the Action step body. | `allow-only-develop-into-test.yml`, `allow-only-test-into-main.yml`; unit-tested by `tests/sd16-e5-f1/test_branch_promotion_guard.sh`. |
| `tools/release/check_release_manifest.py` | Validates release-manifest.json files against the legacy `tools/release/release-manifest.schema.json` shape plus tranche_id/release_notes_path coherence against the working tree. | `check-release-manifest.yml`, `tranche-3-ci.yml`. |
| `tools/release/check_release_manifest_against_dev_schema.py` | Validates a manifest against the dev `schemas/update/update-manifest.schema.json` shape, then re-runs `check_release_manifest.py`'s `_coherence_check`. | `publish-tester-release.yml` (both the linux job's "Validate release manifest (gate)" step and `finalize`). |
| `tools/release/emit_channel_index.py` | Emits and validates a `channels/<channel>.json` pointer from a schema-valid manifest. | `publish-tester-release.yml`'s `finalize` job (both the always-run "Emit alpha channel index" step and the gated "Prepare channel index payload" step). |
| `tools/release/release-manifest.schema.json` | The legacy release-manifest schema (`schema_version: "v1"`, `platform_artifacts` array, linux-only). | Consumed by `check_release_manifest.py`. |
| `tools/release/test_check_release_manifest.py` | `unittest` suite for `check_release_manifest.py` (9 cases as run, uses a `TmpRepo` fixture class). | Run manually (`python3 tools/release/test_check_release_manifest.py`). |
| `tools/release/test_check_release_manifest_against_dev_schema.py` | `unittest` suite for the dev-schema shim (2 cases as run). | Run manually. |
| `tools/release/test_emit_channel_index.py` | `unittest` suite for `emit_channel_index.py` (3 cases as run, including a malformed-tag and wrong-channel negative case). | Run manually. |

All Python validators that call `jsonschema.validate`/`Draft202012Validator` need the `jsonschema` pip package (pinned to `4.21.1` everywhere it is installed in CI, e.g. `publish-tester-release.yml:324`, `check-release-manifest.yml:82`). It is already importable in this workspace (`python3 -c "import jsonschema"` exits 0).

## Workflow trigger and permissions summary

| Workflow | Trigger | Top-level `permissions:` | Concurrency group |
|---|---|---|---|
| `publish-tester-release.yml` | `push` to `develop`, `main` | `contents: write` (line 15) | none declared |
| `promotion-gates.yml` | `pull_request_target` → `test`, `main` | `{}` (job grants its own: `contents: read`, `pull-requests: write`, `issues: write`, `statuses: write`) | none declared |
| `allow-only-develop-into-test.yml` | `pull_request_target` → `test`, `delete` | `{}` (the `restore-develop-branch` job grants `contents: write`) | none declared |
| `allow-only-test-into-main.yml` | `pull_request_target` → `main`, `delete` | `{}` (the `restore-test-branch` job grants `contents: write`) | none declared |
| `check-release-manifest.yml` | `pull_request` → `develop`, `test`, `main` (path-filtered) | `contents: read`, `pull-requests: read` | none declared |
| `tranche-3-ci.yml` | `pull_request` → `tranche/3`, `push` → `tranche/3` | `contents: read`, `pull-requests: read` | `tranche-3-${{ github.ref }}`, `cancel-in-progress: true` (lines 32-34) |

`publish-tester-release.yml` is the only workflow here with no `concurrency:` block — two pushes to `develop` in quick succession can run two full `finalize` jobs concurrently, each pushing to the shared `update-index` branch (mitigated only by each push being a fast-forward-or-fail `git push origin HEAD:update-index`, not by the workflow itself serializing runs). `tranche-3-ci.yml` is the only workflow with an explicit `concurrency:` group, and it cancels in-progress runs on the same ref.

## Related docs

- [testing.md](./testing.md) — the full verification command set, including the standalone scripts referenced in the inventory table above.
- [overview.md](./overview.md) — system-level architecture context.
- [conventions.md](./conventions.md) — repo-wide coding and doc conventions.
- [status.md](./status.md) — current SD/tranche state.
