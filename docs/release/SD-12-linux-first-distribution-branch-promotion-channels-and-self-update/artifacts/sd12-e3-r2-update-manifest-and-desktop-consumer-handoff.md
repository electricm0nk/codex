# SD12-E3-R2 Update Manifest and Desktop Consumer Handoff

## Card outcome
- evidence_class: `documentary-handoff-artifact`
- route truth: this FLOW card is documentary only
- expected delivery evidence for this card: this markdown artifact exists at the path named below
- not expected from this card: no PR, no branch-ready claim, no merge evidence, and no executable implementation change

## Source basis
This handoff is bounded by these upstream evidence artifacts and parent-task handoffs:

1. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r1-execution-readiness-closure-2026-06-29.md`
   - closes the prior lane as documentary readiness only
   - records that the repo had no accepted manifest producer and no dedicated desktop updater boundary at the time of that closure
   - lists the exact future reads, candidate write surfaces, non-goals, and verification commands

2. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-b1-manifest-producer-and-updater-boundary-truth.md`
   - repairs the boundary truth by making the lane explicitly documentary until real producer/consumer seams exist
   - states live manifest producer truth as `none yet`
   - states live desktop consumer truth as `none yet`
   - classifies current desktop update truth as hard-coded SD-11 `not-yet-supported` status plus unrelated GE-08 / pilot-shell Tauri seams

3. `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-b1-promotion-truth-repair-closure-2026-06-29.md`
   - repairs SD-12/SD-11 promotion truth away from stale three-stage `uat` doctrine
   - states live operator promotion truth as `develop -> main`
   - states tester-channel truth as `alpha` backed by `develop`, `stable` backed by `main`, and `beta` reserved/unavailable until a governed candidate promotion surface exists
   - preserves old repo UI/status code three-stage assumptions only as read-only comparison evidence, not publication-topology authority

4. Parent grounding from `t_132d117a`
   - applicable repo: `/home/ubuntu/workspace/repos/codex`
   - repo-root instruction surface: `/home/ubuntu/workspace/repos/codex/AGENTS.md`
   - desktop verification floor exists under `/home/ubuntu/workspace/repos/codex/apps/desktop`
   - `npm run tauri:check` is mandatory for desktop/Tauri/package-affecting implementation handoffs; a docs-only handoff still needs explicit acceptance evidence

5. Parent extraction from `t_7cfe16d2`
   - current manifest producer truth: `none yet; documentary-only repair path`
   - producer classification: `not workflow-backed, not Tauri-command-backed, not custom-fetch-backed today`
   - desktop consumer truth: `none yet; current real status is SD-11 hard-coded not-yet-supported over unrelated GE-08/pilot-shell seams`

## Exact target repo and workdir for any downstream execution handoff
- target repo: `/home/ubuntu/workspace/repos/codex`
- repo root as grounded by `git rev-parse --show-toplevel`: `/home/ubuntu/workspace/repos/codex`
- desktop workdir for desktop verification: `/home/ubuntu/workspace/repos/codex/apps/desktop`
- governing repo instruction file: `/home/ubuntu/workspace/repos/codex/AGENTS.md`

This documentary artifact lives outside the implementation repo at:

`/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r2-update-manifest-and-desktop-consumer-handoff.md`

## Exact allowed write scope for this FLOW card
This FLOW card may write only this artifact:

- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r2-update-manifest-and-desktop-consumer-handoff.md`

It must not write implementation code, workflows, package files, tests, release surfaces, or branch-policy surfaces.

## Grounded manifest producer truth
There is no accepted live manifest producer to hand off as executable repo truth yet.

Grounded producer classification:
- producer type: `none accepted yet`
- producer path: `none accepted yet`
- not workflow-backed today as accepted repo truth
- not Tauri-command-backed today as accepted repo truth
- not custom-fetch-backed today as accepted repo truth

The only candidate future manifest producer path named by the repair artifacts is:

- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`

That path is a candidate publication-plus-manifest-emission surface, not proof of a current official producer. A local dirty or untracked worktree file at that path is not by itself accepted topology, release policy, branch semantics, or publication authority. A future implementation handoff must read the accepted file state directly and prove that it owns manifest output explicitly before treating it as the producer.

## Grounded desktop consumer truth
There is no accepted dedicated desktop manifest/update consumer boundary yet.

Current grounded consumer truth:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` hard-codes the current update state as `not-yet-supported`
- current tester-facing label remains under SD-11 authority, including the existing wording `Update checks not yet wired in this slice`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` is a GE-08 authoring boundary wrapper, not an update boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts` is a pilot-shell snapshot boundary wrapper, not an update boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` is only a candidate future registration surface if a dedicated read-only manifest/update command is accepted later

A future desktop consumer handoff must add or name a dedicated manifest/update boundary. It must not smuggle updater semantics into GE-08 or pilot-shell commands.

## Exact required reads for a future implementation handoff
A downstream implementation handoff must require these reads before any write:

### SD-12 and SD-11 documentary authority
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/self-update-transport-and-manifest-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/provenance-integrity-and-update-eligibility.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-b1-promotion-truth-repair-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r1-execution-readiness-closure-2026-06-29.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-b1-manifest-producer-and-updater-boundary-truth.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`

### Repo instruction and live repo surfaces
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml`
- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml` if it exists in the accepted implementation worktree
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

## Exact allowed write scope for a future implementation handoff
If a future handoff is authorized after the producer and consumer boundaries are accepted, the first honest write scope should stay inside these paths only:

- `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/**` for a new dedicated manifest/update boundary only
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

Read-only unless a later handoff explicitly proves need:

- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json`

If implementation cannot stay inside these paths, stop and route a new readiness/truth repair. Do not widen scope by implication.

## Exact non-goals
A future implementation handoff must state these non-goals plainly:

- no silent invention of a publication workflow, `uat` branch, or tester release topology that does not exist in accepted live repo truth
- no treating a detailed manifest schema as proof that a live producer or consumer seam exists
- no repurposing of `loadGe08AuthoringWorkbench.ts` or `loadPilotShellSnapshot.ts` as pseudo-updater surfaces
- no treating hard-coded SD-11 status copy as a dedicated manifest/update consumer
- no activation of `bundle.active`, packaging, or updater claims merely because manifest fields are documented
- no hidden publication topology, updater mechanics, release channels, branch semantics, rollback policy, withdrawal policy, or provenance policy
- no raw `develop`, `uat`, or `main` branch names as the primary tester-facing update UX
- no widening into rollback, downgrade, withdrawal, provenance-policy implementation, GitHub feedback intake, attachment transport, or unrelated SD-11 workbench flows
- no PR, merge, branch-ready, or executable implementation evidence from this documentary FLOW card

## SD-11 wording and operator provenance constraints
SD-11 remains authoritative for tester-facing workbench wording.

A future update-status implementation must preserve this hierarchy:
- primary tester-facing language: channel and support-state wording
- secondary operator/provenance detail: live branch lineage currently grounded as `develop -> main`

Any old SD-11 or repo UI/status references to `develop -> uat -> main` are read-only stale comparison evidence unless a later bounded lane creates and verifies a governed candidate promotion surface. They are not publication topology, branch authority, or tester-facing update language for this handoff.

Do not make raw branch provenance the main user-facing update message. Branch provenance may support operator/debug context, but it must not outrank channel availability, supported platform posture, manual-only status, blocked/withdrawn status, or tester support language.

## Exact verification commands for a future implementation handoff
A downstream implementation handoff must include verification commands or explicit acceptance evidence. For desktop/Tauri/package-affecting work, the verification floor is:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Branch and workflow truth checks:

```bash
git -C /home/ubuntu/workspace/repos/codex branch --list
git -C /home/ubuntu/workspace/repos/codex branch -r
git -C /home/ubuntu/workspace/repos/codex status --short
```

Repo-root/Rust checks from the parent grounding, when the implementation touches repo-root Rust or release-truth surfaces:

```bash
cd /home/ubuntu/workspace/repos/codex
. "$HOME/.cargo/env"
cargo test
cargo test ge06_
cargo test ge08_
```

This documentary card does not require running the desktop build floor because it writes only this markdown handoff artifact. Its acceptance evidence is the presence and content of this file.

## Acceptance evidence for this FLOW card
This card is complete when:
- this file exists at `/home/ubuntu/workspace/programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e3-r2-update-manifest-and-desktop-consumer-handoff.md`
- the file names the exact target repo/workdir
- the file names the exact allowed write scope for this documentary card
- the file names the exact required reads for downstream implementation
- the file names the exact future allowed write scope and non-goals
- the file plainly states that there is no accepted live manifest producer or dedicated desktop consumer boundary yet
- the file lists verification commands without claiming that this documentary FLOW card produced PR, branch-ready, merge, or executable implementation evidence
