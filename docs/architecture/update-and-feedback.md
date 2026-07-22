# Update & Feedback

> Scope: The desktop app's self-update chain and its feedback/defect-report submission chain, including exactly what is real vs. stubbed today.
> Last verified: 2026-07-22 against tranche/5-3 (SD-25 closure)
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

Both subsystems share one ethos, stated verbatim in multiple places in the source: **never claim more than is proven.** A failed or missing piece degrades honestly to `'unknown'` / a named reason string, never to a fabricated success. This document traces both chains through the real files and calls out, precisely, where that posture currently means "not wired yet."

## Self-update chain

All paths below are under `apps/desktop/src/sd16/update/` unless noted.

**`updateModel.ts`** is the shared contract the UI renders against: `UPDATE_CHANNEL_OPTIONS = ['alpha', 'beta', 'stable']` (this exact order — the release-promotion order, not a stability order); `InstalledState`, `LastCheckState`, and `PendingRollbackState` are the three state shapes threaded through `UpdateControllerDeps`. The `UpdateController` interface (`runCheck`/`computeEligibility`/`disabledReason`/`releaseNotes`) is the dependency seam: the UI (`Ui.tsx`) never calls a fetcher or `invoke()` itself, only this interface. Until a real controller is supplied, `buildUnwiredUpdateDeps()` provides a deterministic "not wired" controller so the UI never fabricates an eligibility verdict before a real one exists.

**Discovery fetch** (`fetch.ts`) fetches two documents over plain `fetch` (indirected through an injectable `fetchImpl` so tests never touch the network): the channel index at `https://raw.githubusercontent.com/electricm0nk/codex/update-index/channels/<channel>.json` (`channelIndexUrl`), and the update manifest at the URL the index names. Both are validated against canonical JSON Schemas — `schemas/update/channel-index.schema.json` and `schemas/update/update-manifest.schema.json` at the repo root — via the ajv-based parsers in `parseChannelIndex.ts` / `parseUpdateManifest.ts` (`loadSchemas.ts` imports the schema files as typed JSON modules, enabled by `tsconfig.json`'s additive `include` entries). Every failure mode — HTTP error, invalid JSON, schema violation, unsupported channel — returns a discriminated `FetchResult<T>` (`{ ok: false, failure: {...} }`); nothing throws on the happy-vs-sad-path boundary, so callers can render a deterministic reason instead of catching exceptions. `fetch.ts` also fetches and SHA-256-verifies the release-notes body named by the manifest (`fetchReleaseNotesBody`) — a hash mismatch fails exactly like an HTTP error; the UI never renders unverified release-notes prose.

**`eligibility.ts`** is a pure decision function, `decideEligibility(input: EligibilityInput): EligibilityDecision`, evaluated as a fixed, first-match-wins row order over installed-state, manifest identity, and fetch outcomes — so `install_disabled_reason` is deterministic. `unknown` outranks `ineligible`, which outranks `eligible`; the UI (`InstallControl.tsx`) enables the Install button only when the result is exactly `'eligible'`. `compareVersions` is a minimal dotted-segment semver-like comparator (numeric-aware, missing trailing segments treated as `0`).

**`controllerAdapter.ts`**'s `createUpdateControllerDeps()` builds the one real controller in the app, bridging the genuinely-real pieces: `fetch.ts`'s fetch/validate, `eligibility.ts`'s pure table, and the three Tauri commands with real bodies (`verify_relaunch_artifact`, `perform_restore_previous`, and `is_install_eligible`). Its `runCheck(channel)` fetches the index and manifest, then calls `is_install_eligible` and feeds the result into `decideEligibility`; every step that can fail degrades to `'unknown'` with a named reason rather than guessing (e.g. `NO_LOCAL_RECORD_REASON = 'no local installed-state record yet — is_install_eligible has nothing to compare the fetched manifest against'`). It calls `invoke()` directly rather than through a `boundary/*.ts` wrapper (see [desktop-app.md](./desktop-app.md)'s boundary-rule exception note), but still guards `hasTauriRuntime()` from `boundary/runtime.ts` and accepts an injectable `invokeImpl` for tests.

### The four native transaction commands

All four are defined in `apps/desktop/src-tauri/src/update/transaction.rs` and registered in `main.rs`'s `generate_handler![...]`:

- **`is_install_eligible`** — real. Reads `installed-state.json` from the resolved config root (`$CODEX_CONFIG_DIR` or `$HOME/.config`) and reports install-kind/version/hash/managed-path-writability facts. It deliberately renders no eligible/ineligible verdict itself — `decideEligibility` (TS) is the single source of that decision, so the fact-probe and the decision table cannot drift into two independent copies.
- **`perform_install`** — **stub, verified precisely.** Its Rust body is:
  ```rust
  pub fn perform_install(_args: PerformInstallArgs) -> Result<RelaunchPrompt, String> {
      Err(
          "perform_install is registered but not wired: downloading the AppImage artifact \
           requires an HTTP client this crate does not carry as a dependency yet; \
           execute_transaction itself is real and tested, but no install is performed here \
           until a download step lands"
              .to_string(),
      )
  }
  ```
  The staged-transaction body it would drive (`execute_transaction`, same file) is real and fully unit-tested against a fixture download closure — the only missing piece is a real HTTP client dependency, which `apps/desktop/src-tauri/Cargo.toml` does not carry (confirmed: no `reqwest`/`ureq`/etc. in the dependency list). On the TypeScript side, `installAction.ts`'s `performInstall()` calls `invoke("perform_install", ...)` directly and is fully implemented (including mounting an `#install-relaunch-prompt` DOM hook on success) — but it is **never called from the live UI**: `Ui.tsx`'s `handleInstall` callback is itself a documented no-op (*"The actual install transaction lives in the update backend (Tauri); F3c only owns the gate... F3c never invokes a Tauri command directly"*), confirmed by grep — `performInstall(` has no call site outside its own definition and its test file. So the stub is doubly inert: the Rust command always errors, and nothing in the shipped UI even calls it yet.
- **`perform_restore_previous`** — real. Implements the full AV-RB rollback decision tree (`perform_restore_previous_impl`): no pending update → `NoPending`; the 3-consecutive-mismatch auto-restore fast path (`AUTO_RESTORE_THRESHOLD = 3`, tracked in a `rollback-state.json` sidecar) → `AutoRestored`; no backup at the canonical slot → `NoBackup`; backup unreadable or atomic-replace fails → `RollbackFailed` (sidecar records `rollback_state: "rollback-failed"` with the exact reason, kept until explicit operator clear); otherwise → `Promoted`, which copies the most-recent backup (`Codex.previous.AppImage`) over the managed path, rewrites `installed-state.json`, deletes `pending-update.json`, and resets the sidecar.
- **`verify_relaunch_artifact`** — real. Hashes the running binary (`std::env::current_exe()` streamed through SHA-256) and compares it to `pending-update.json`'s expected hash. Match → promotes: writes a fresh `installed-state.json` and deletes the pending marker (`ReloadVerifyOutcome::Promoted`). Mismatch → flags the pending record `pending_update_state: Mismatch` but leaves `installed-state.json` untouched (`VerificationFailed`) so `restoreOffer.tsx` can offer a rollback. No pending file at all → `NoPendingUpdate`, a clean no-op. A corrupt/unreadable pending file is also treated as `NoPendingUpdate` rather than wedging the shell.

**A fifth command, `perform_retention_sweep`, exists with a real, tested body** (`perform_retention_sweep_impl`, enforcing the two-slot backup cap, post-success staging cleanup, and the never-auto-delete-pending-while-unresolved rule) but is not imported or registered in `main.rs`'s `generate_handler![...]` — it is unreachable from the frontend via `invoke()` today.

**Where the update data comes from at runtime**: the channel index and the manifest it points at both live on the `update-index` branch of the `codex` GitHub repo, published by the release lane — see [release-pipeline.md](./release-pipeline.md) for how that branch gets written.

### The UI panels

`Ui.tsx`'s `UpdateUi` composes: `ChannelSelector` (the pinned three-option select), `CheckPanel` (drives `controller.runCheck`), `InstallControl` (the eligibility badge + Install button, disabled unless `eligibility === 'eligible'`, with a deterministic `#install-disabled-reason` DOM hook), `InstalledPanel` (renders `deps.installed` verbatim), `LastCheckPanel`, `PendingRollbackPanel`, and — only when `App.tsx`'s `UpdateSection` supplies one after `verify_relaunch_artifact` reports a mismatch — `RestoreOffer` plus a live "Restore previous version" button wired to `restorePreviousVersion()`. `App.tsx`'s `UpdateSection` component is the mount point: it calls `loadMountTimeState()` (which runs `verify_relaunch_artifact`) once per mount, builds `UpdateControllerDeps` via `createUpdateControllerDeps`, and re-runs both after a restore completes.

## Feedback / defect-report

### Evidence capture

`apps/desktop/src/testerWorkbench/feedback/evidence/` (renamed from `sd11/feedback/evidence/` by SD-25 criterion 1.1's identifier cleanup) is the shared substrate both the bug-report and enhancement-request flows depend on, so their schemas and redaction rules cannot drift apart.

- **`captureFeedbackEvidence.ts`**'s `captureAutoEvidence(surface)` pulls a fixed backbone of fields from the live `TesterWorkbenchSurface` — build label, channel/support label, platform, current workflow, data-source identity, and (when available) release-truth fields like `releaseUnitId`/`sourceRevision`/`updateEligibilityState`/`trustGateStatus` — every string passed through `sanitizeReportableOutput`. `assembleFeedbackEvidence(input)` merges that backbone with tester-entered narrative fields (`observedBehavior`/`expectedBehavior`/`reproductionSteps` for bugs; `testerGoal`/`currentFriction`/`requestedCapability`/`affectedSurface` for enhancements) into one `FeedbackEvidencePayload`, categorizing every applicable field as auto-captured / tester-entered / redacted / optional and collecting `problems: string[]` for anything `required` but missing.
- **`redaction.ts`** enforces that nothing is captured silently: `evaluateAttachment()` returns `'requires-confirmation'` for any attachment that may contain sensitive data and lacks explicit `testerConfirmedInclude`, or is simply unconfirmed at all; `validateRedaction()` additionally requires a non-empty redaction-declaration statement whenever any attachment is present. The `REDACTION_POLICY_NOTICE` constant (surfaced verbatim in `App.tsx`'s `FeedbackEvidencePanel`) states this policy in user-facing language.

### Bug + enhancement composers

`testerWorkbench/feedback/bug/composeBugReport.ts` and `testerWorkbench/feedback/enhancement/composeEnhancementRequest.ts` are structurally identical and deliberately non-interchangeable: each throws if handed a payload whose `flow` doesn't match its own kind, so a bug composer can never silently produce an enhancement draft or vice versa. Each renders a `GithubBugIssueDraft` / equivalent enhancement draft as distinct markdown sections (bug: Summary / Current build-channel-platform-workflow / Observed behavior / Expected behavior / Reproduction steps / Diagnostics / Attachments; enhancement mirrors this with its own four narrative fields) plus a derived label set (`bug`/`enhancement` base label, `channel:*`, `platform:*`, `surface:*`). `submittable` is true only when `payload.complete && title.length > 0`.

`submitBugReport.ts` / `submitEnhancementRequest.ts` accept an **injected transport** (`transport?: BugReportTransport | null`) and never fabricate a filed issue:
- `!composed.submittable` → `status: 'blocked-incomplete'`, draft preserved.
- `submittable` but `transport` is `null` (today's default — no transport is wired into the shell) → `status: 'draft-preserved'`, message states plainly *"No GitHub submission transport is configured in this build."*
- `transport` throws, or returns `ok: false`, or returns no valid `issueUrl` → `status: 'draft-preserved'` again, never `'submitted'`.
- Only a transport call that returns `ok: true` **and** a URL that parses as `http(s)` → `status: 'submitted'`, `claimedSubmitted: true`, `resultHandle: { issueUrl, issueNumber }`.

Every outcome carries `copyablePayload` (the full rendered markdown) so a tester's evidence is never lost regardless of transport outcome — `App.tsx`'s composers always render a "Copy governed draft" button next to the outcome.

### Browser handoff

`App.tsx`'s composers route every `submittable` draft through the governed browser-handoff path instead of the (transport-less) `submitBugReport`/`submitEnhancementRequest` — only non-submittable drafts still call those, purely to get the honest `blocked-incomplete` preservation outcome.

**`sd16/feedback/browserHandoff.ts`**'s `runBrowserHandoff(draft)` drives a pure reducer (`submissionState.ts`'s `reduceSubmissionState`) through a fixed event sequence:

```
OPEN → URL_BUILT → BROWSER_OPENED(url)   only after the real OS open succeeded
                 → BROWSER_FAILED(reason) on every other outcome
```

The reducer's states are `idle` / `opening` / `awaiting-issue-url` / `confirmed` / `failed` — there is deliberately **no `submitted` state**. `confirmed` is reachable only via a `BROWSER_OPENED` event carrying a non-empty URL; even a `BROWSER_OPENED` with an empty URL routes to `failed(reason: 'empty-url')` rather than `confirmed`. `canClaimSubmitted(state)` — true only for `confirmed` with a non-empty URL — is the single source of truth `submissionUiState.ts`'s `deriveSubmissionUiState` re-derives from, tested exhaustively to match.

If `hasTauriRuntime()` is false, `runBrowserHandoff` immediately dispatches `BROWSER_FAILED` with reason *"desktop runtime unavailable..."* and returns — no `invoke()` call is attempted. Otherwise it calls `invokeImpl('handoff_defect_report_to_browser', { req: { owner, repo, title, body, labels } })` (owner/repo pinned to `GITHUB_ISSUE_OWNER = 'electricm0nk'` / `GITHUB_ISSUE_REPO = 'codex'`).

**Rust side** (`apps/desktop/src-tauri/src/browser_handoff.rs`, renamed from `sd16_browser_handoff.rs` by SD-24 criterion 1.1): `handoff_defect_report_to_browser` builds a prefilled GitHub "new issue" URL (`build_github_issue_url`, hand-rolled percent-encoding — no `percent-encoding` crate dependency added), shape-validates owner/repo/title/body/label lengths, **re-validates the built URL** as defense-in-depth (`validate_github_issues_url` — must be `https://github.com/<owner>/<repo>/issues/new`, ≤ `MAX_URL_LENGTH = 8192` bytes), then hands it to `tauri-plugin-opener`'s real OS browser open. `opened: true` is returned in `IssueUrlResponse` **only after** the OS-level open call itself reports success; a failed open returns `IssueUrlError::BrowserOpenFailed { reason, url }`, carrying the already-validated URL back so the shell can offer a manual link instead of discarding the prepared handoff.

`App.tsx`'s `BrowserHandoffResultPanel` renders the honest framing directly in the UI copy: on `confirmed`, *"A prefilled GitHub issue form was opened in your browser. Review it and press 'Create' there to file the issue — the shell only confirms the form was opened; it does not claim the issue was submitted."* On failure, the composed draft (and, when available, a manual link to the validated URL) stays visible so nothing is lost.

## The honest-degradation ethos, concretely

Every degradation point traced above names its own reason rather than defaulting silently:

- `decideEligibility` returns `'unknown'` (never a guessed `'eligible'`/`'ineligible'`) whenever a fetch step or the local install probe hasn't completed, is missing, or has failed — each branch carries its own distinct reason string.
- `is_install_eligible`'s `Ok(InstallEligibilityProbe { installed: None, .. })` is documented as "honest 'nothing to report' result, not an error" for a fresh install with no prior `installed-state.json`.
- `perform_install` refuses to fabricate an install; its error message names the exact missing dependency class (an HTTP client), not a vague failure.
- `verify_relaunch_artifact` treats a hash-compute failure as `VerificationFailed` (prompting restore) rather than silently promoting.
- `runBrowserHandoff` never reaches `confirmed` without a real, non-empty, OS-confirmed URL; a corrupt or empty result always routes to `failed`.
- `submitBugReport`/`submitEnhancementRequest` never report `'submitted'` without a transport-confirmed issue handle, and always preserve the full draft as copyable text so a degraded path never destroys tester evidence.

## See also

- [desktop-app.md](./desktop-app.md) — the full Tauri command inventory (including these commands in context), the boundary-rule exception these modules represent, and the frontend directory map.
- [release-pipeline.md](./release-pipeline.md) — how the channel index / update manifest this chain fetches are published to the `update-index` branch.
- [testing.md](./testing.md) — how `fetch.ts`/`parseChannelIndex.ts`/`parseUpdateManifest.ts`/the transaction module's injected-closure seams are exercised without real network or filesystem access.
- [status.md](./status.md) — current capability/stub status across the whole repo.
