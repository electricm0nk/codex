---
title: SD15-E7 Execution Handoff — Released Linux alpha tester-defect repair bundle
handoff_id: HANDOFF-CODEX-SD15-E7-CODING-2026-07-02
stc_id: STC-CODEX-SD-15
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md
source_stc: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md
source_epic_breakdown: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/epic-breakdown.md
selected_slice: SD15-E7 — Released Linux alpha tester-defect repair bundle
run_in: Codex CLI or Claude Code; must follow repo AGENTS.md and TDD
code_authority: true
created_at: 2026-07-02
authority_dependencies:
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-lnx-a-testing-instructional-brief-2026-07-02.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-clean-machine-validation-report.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/intake-to-triage-mapping.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md
  - /home/ubuntu/workspace/repos/codex/AGENTS.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: 4162b673c4c5a9aea597296e4f94c050139e87a6
  recommended_branch: sd15-e7-linux-alpha-test-defect-repairs
  pr_target: develop
allowed_write_scope:
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
  - apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts
  - apps/desktop/src-tauri/src/ge08_workbench.rs
  - apps/desktop/src-tauri/src/ge08_workbench_tests.rs
  - apps/desktop/src-tauri/tauri.conf.json
  - apps/desktop/src-tauri/Cargo.toml
  - apps/desktop/src-tauri/resources/**
  - .github/workflows/publish-tester-release.yml
  - apps/desktop/src/App.tsx
  - apps/desktop/src/sd11/feedback/bug/submitBugReport.ts
  - apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts
  - apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.ts
  - apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**/.env*
  - any file that embeds GitHub tokens, personal access tokens, or long-lived credentials
  - broad product UX rewrites outside the named SD-11/SD-15 feedback and workbench paths
  - status-surface closure claims or tranche-closure verdict files
---

# SD15-E7 Execution Handoff — Released Linux alpha tester-defect repair bundle

## Status
This is the stage-specific code-authorizing brief for repairing defects discovered during the 2026-07-02 LNX-A governed alpha tester run.

It carries `code_authority: true` only for the bounded repo paths listed above. It does **not** authorize edits to the SD-15 documentary control plane; Hermes authored this handoff and has already updated the source artifact. The coding worker must change only the implementation repo and must open a PR back to `develop`.

## Core problem
The released Linux alpha artifact is installable and launchable, but external tester execution exposed three bounded defects or posture gaps:

1. **Runtime content/data packaging failure:** the GE08 authoring workbench falls back because the packaged app resolves the proof package root to a repo-local CI/source path that does not exist on tester machines.
2. **Publication truth mismatch:** the actual GitHub release asset is dot-separated (`Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb`), while `checksums.sha256` records the older spaced filename (`./Codex Desktop Shell Scaffold_0.0.0_amd64.deb`). The hash matches, but filename truth is inconsistent.
3. **Manual issue-draft transport posture must remain honest:** this build deliberately has no GitHub issue submission transport and no auth. The UI and submit modules must preserve drafts and never claim submission, while making clear that manual filing is the intended current path. This slice must not add hardcoded credentials or implement a secret-bearing GitHub poster.

The decisive action is to repair what the release can truthfully support: bundle/load the GE08 proof package from packaged resources or another governed shipped path; make release asset naming and checksum naming agree; and harden the no-transport issue-draft posture without adding unsafe auth.

## Evidence basis from tester run

### Environment and package evidence
Tester evidence from Todd on `terminus`:

```text
Package: codex-desktop-shell-scaffold
Status: install ok installed
Architecture: amd64
Version: 0.0.0
Depends: libwebkit2gtk-4.1-0, libgtk-3-0
Binary: /usr/bin/codex_desktop_shell_scaffold
OS: Ubuntu 25.10 (Questing Quokka)
Kernel: Linux terminus 6.17.0-35-generic x86_64
```

### Integrity evidence
The downloaded `.deb` matched the published SHA-256 value:

```text
local artifact: Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb
local hash:     c8f2f1b48a5f7fcdc2bc7e5db6ce0e2e1568d8e23a62c2d8b97ce8438b2e4031
published line: c8f2f1b48a5f7fcdc2bc7e5db6ce0e2e1568d8e23a62c2d8b97ce8438b2e4031  ./Codex Desktop Shell Scaffold_0.0.0_amd64.deb
```

The checksum value matches. The filename does not.

### Runtime fallback evidence
Visible tester screenshot text included:

```text
GE08 authoring workbench unavailable: Failed to load GE08 authoring workbench: package root does not exist: tests/fixtures/ge08/guard-stance-package (resolved to /home/runner/work/codex/codex/tests/fixtures/ge08/guard-stance-package). This fallback exists because the real bounded snapshot could not load and the UI must not counterfeit product truth.
```

The UI also showed:

```text
build: codex-desktop-shell-scaffold@0.0.0
channel: alpha
platform: Linux
workflow: GE07 pilot snapshot seam / Unknown/Unavailable
diagnostics: 2 diagnostic(s) · 0 blocked claim(s)
```

### Submission posture evidence
The app reported:

```text
Submission status: draft-preserved
No GitHub submission transport is configured in this build. The complete structured draft is preserved for manual filing or copy — no submission was performed and none is claimed.
No issue handle was returned, so no successful submission is claimed.
```

This is expected in principle. Todd explicitly stated no GitHub credentials were supplied and no token must be hardcoded. Treat this as a posture to preserve and clarify, not as permission to add credential-bearing transport.

## Exact target repo and branch policy

Target repo:

```text
/home/ubuntu/workspace/repos/codex
```

Grounded base at handoff creation:

```text
branch: develop
HEAD:   4162b673c4c5a9aea597296e4f94c050139e87a6
remote: git@github.com:electricm0nk/codex.git
```

Launch from fresh `origin/develop`:

```bash
git fetch origin --prune
git switch -c sd15-e7-linux-alpha-test-defect-repairs origin/develop
```

Do not work directly on `develop`. Do not merge directly. Push a feature branch and open a PR targeting `develop`.

## Required reads before coding
Read these first, in order:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/README.md`
3. This handoff: `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-lnx-a-testing-instructional-brief-2026-07-02.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-install-and-use-matrix.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/ge08_workbench.rs`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
11. `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
12. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
13. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.ts`
14. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.ts`

Conditional reads:
- `apps/desktop/src-tauri/src/main.rs` if resource resolution requires a Tauri app handle or command wiring change.
- `tests/fixtures/ge08/guard-stance-package/**` as read-only source material for the packaged resource copy.
- Tauri v2 resource documentation if exact `bundle.resources` syntax or runtime resource lookup API is uncertain.

## Required implementation outcomes

### Outcome 1 — GE08 proof package is available in released desktop artifact
The released desktop app must no longer rely on `/home/runner/work/codex/codex/tests/fixtures/ge08/guard-stance-package` or any source-checkout-only path for the default tester workbench.

Acceptable implementation shapes:
- bundle the guarded GE08 proof package under `apps/desktop/src-tauri/resources/**` and configure Tauri to include it, then resolve it through a packaged-resource path at runtime; or
- implement an equally governed packaged-data path that works in both dev and installed builds.

Required behavior:
- dev/test runs still load the GE08 proof package.
- installed/released builds have a shipped proof package path available without `CODEX_REPO_ROOT`.
- fallback remains available only for genuine load failures, not because the default path is source-only.
- no UI may pretend broader product readiness than GE08 proves.

Recommended failing test posture:
- add or update Rust tests around `resolve_package_path` / `build_ge08_workbench_snapshot` so a packaged-resource-style default can load without `CODEX_REPO_ROOT`.
- add or update TypeScript tests so `loadSd11TesterWorkbenchSurface` no longer hardcodes a source-test fixture path as the production default without a packaged-build alternate.

### Outcome 2 — Release checksum manifest filename agrees with uploaded asset names
The publication workflow must produce a `checksums.sha256` whose filenames match the names uploaded to GitHub release assets.

Observed defect:
- uploaded asset: `Codex.Desktop.Shell.Scaffold_0.0.0_amd64.deb`
- checksum entry: `./Codex Desktop Shell Scaffold_0.0.0_amd64.deb`

Required behavior:
- the checksum line for the `.deb` must use the exact final uploaded asset filename.
- the checksum line for the `.AppImage` must likewise use the exact final uploaded asset filename.
- release notes and staged asset names must not drift from checksum names.
- preserve the existing hash/integrity logic; fix naming truth, not cryptographic behavior.

Recommended implementation shape:
- normalize artifact filenames in `release-staging` before computing checksums and before upload, or compute checksums over the exact uploaded filenames after the final rename.
- ensure release notes reference the same normalized names.

### Outcome 3 — Manual issue-draft behavior is explicit and safe
The current no-transport posture is acceptable and must remain safe.

Required behavior:
- no hardcoded GitHub token, PAT, OAuth token, or secret-bearing transport is introduced.
- `submitBugReport({ transport: null })` and `submitEnhancementRequest({ transport: null })` continue to produce `draft-preserved`, `claimedSubmitted: false`, and `resultHandle: null` for complete drafts.
- UI copy makes clear the current released build preserves a manual draft because no GitHub submission transport is configured.
- if button labels currently imply a live GitHub submission when transport is statically null, adjust copy toward “Preserve draft for manual filing” / “Prepare issue draft” or equivalent without inventing transport.
- regression tests must preserve that no submission is claimed without a real issue URL returned by an injected transport.

Out of scope for this slice:
- implementing GitHub OAuth, device flow, PAT input, server-side broker, GitHub App, or any credentialed issue poster.
- changing the policy that transport must be injected and must return a real issue handle before submission is claimed.

## Exact allowed write scope
You may modify only these repo paths if needed:

```text
apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts
apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts
apps/desktop/src-tauri/src/ge08_workbench.rs
apps/desktop/src-tauri/src/ge08_workbench_tests.rs
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/resources/**
.github/workflows/publish-tester-release.yml
apps/desktop/src/App.tsx
apps/desktop/src/sd11/feedback/bug/submitBugReport.ts
apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts
apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.ts
apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts
```

If a truthful fix requires a path outside this list, stop and report the narrower missing authority. Do not widen silently.

## Forbidden scope
Do not:
- edit `programs/codex/**` from the coding run
- hardcode GitHub credentials or tokens anywhere
- create a GitHub submission transport in this slice
- claim an issue was filed without a real issue handle
- turn GE08 fallback suppression into fake success
- change support tiers, channel semantics, or tranche-closure status
- broaden into SD-13 rules breadth or SD-14 persistence work
- publish a release from the feature branch

## Verification commands
Run the smallest relevant tests first, then the full desktop floor.

Required targeted commands from repo root or stated workdir:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm ci
npm run typecheck
npm test
npm run build
npm run tauri:check
```

Required Rust checks from repo root:

```bash
cd /home/ubuntu/workspace/repos/codex
cargo test --locked ge08
cargo test --locked
```

If package-resource behavior can be tested by a debug bundle within runtime limits, additionally run:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npx tauri build --debug --bundles deb --ci
```

Then inspect the generated `.deb`/staging behavior or resource contents enough to prove the GE08 proof package is included or otherwise available through the packaged path.

If the debug bundle is too slow or blocked by environment dependencies, state that explicitly and leave the targeted resource-resolution tests as the verified proof floor.

## Acceptance criteria
- [ ] A source-checkout-free GE08 package path exists for released desktop builds.
- [ ] The default SD-11 tester workbench path no longer causes the released Linux alpha to fall back because `tests/fixtures/ge08/guard-stance-package` is missing under `/home/runner/work/...`.
- [ ] Existing explicit fallback behavior still appears for genuine missing/corrupt package loads.
- [ ] `checksums.sha256` generation names the same `.deb` and `.AppImage` files that the release upload exposes.
- [ ] No credential, token, or GitHub issue-poster transport is added.
- [ ] Complete no-transport bug/enhancement drafts remain `draft-preserved`, never `submitted`.
- [ ] UI copy no longer implies live GitHub submission is configured when the build passes `transport: null`.
- [ ] Required tests and build checks pass or blockers are reported with exact command output.
- [ ] A PR targeting `develop` is opened with this handoff path cited in the PR body.

## Final delivery requirements
The coding worker must report:
- branch name
- base SHA
- files changed
- tests added/updated
- commands run and results
- generated package/resource proof if available
- PR URL targeting `develop`
- any remaining defect that should become a separate handoff

The problem is not that the alpha failed to launch. It launched and told the truth. Now make the truth less disappointing.
