# GE-07 Acceptance and Verification

## Acceptance posture
This document defines what must be true for the GE-07 source STC to count as a truthful planning-ready artifact, and what later evidence must exist before any UI slice can claim product truth.

## Planning acceptance criteria

### A1. Source-STC control bundle exists and is internally linked
**Given** the GE-07 requirement directory
**When** the bundle is inspected
**Then** `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md` all exist and describe the same shell/UX boundary.

Evidence:
- file existence under `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/`
- cross-links and consistent route posture across the documents

### A2. Same-epic documentary outputs exist
**Given** the GE-07 spec domain required concrete outputs beyond the control bundle
**When** the artifact set is inspected
**Then** shell architecture requirements, UI information architecture, pilot UX flow, component inventory, command boundary requirements, and cross-platform build questions all exist as explicit documents under `artifacts/`.

Evidence:
- file existence under `artifacts/`
- each artifact has a bounded purpose and does not drift into ungoverned implementation prose

### A3. The UI is explicitly a consumer of domain truth
**Given** the GE-07 bundle
**When** the shell boundary is reviewed
**Then** the documents state plainly that the UI consumes GE-03/GE-04/GE-06 outputs and is forbidden from owning rules semantics.

Evidence:
- `README.md`
- `technical-requirements.md`
- `artifacts/ui-command-boundary-requirements.md`

### A4. Route authority remains non-coding
**Given** this source STC
**When** the route posture is inspected
**Then** the active route is planning/Hermes, `execution-handoff.md` does not exist, and no part of the bundle pretends to grant code authority now.

Evidence:
- `README.md` readiness/next-stage rule
- directory listing showing no GE-07 `execution-handoff.md`
- execution-status-ledger entry updated to reflect source-STC existence without code authority

### A5. UI-truth requirements are explicit
**Given** the quality-gate policy
**When** GE-07 requirements are compared against it
**Then** the bundle requires real domain data, explanation visibility, and diagnostics visibility, and rejects mock-state proof.

Evidence:
- `technical-requirements.md`
- `artifacts/pilot-ux-flow-requirements.md`
- `artifacts/component-surface-inventory.md`

## Future implementation gate criteria
A future GE-07 execution handoff is not allowed until the following are grounded:
1. GE-06 viability posture is explicitly accepted for the intended slice, or a bounded pre-viability spike is explicitly authorized.
2. exact repo paths / write scope are named.
3. branch/worktree policy is explicit.
4. runtime toolchain and shell scaffold prerequisites are verified in the target repo.
5. verification commands or receipt obligations are concrete.
6. the chosen slice is narrower than the whole epic.

## Verification steps performed for this documentary pass
1. verified the GE-07 spec domain exists and defines the missing-STC problem.
2. verified the Codex requirements index and execution-status ledger still treated GE-07 as spec-domain-only before this pass.
3. verified the live Codex repo exists at `/home/ubuntu/workspace/repos/codex`.
4. verified the live repo observation on 2026-06-21: local branch `ge06-e2-f3-headless-receipt-path`, `origin/develop` commit `2deb11b3e8b8c03c6c009a3c6bade18d7f6e6177`.
5. created the GE-07 source-STC bundle and required same-epic documentary outputs.
6. updated route/index surfaces so GE-07 is now visible as a source STC with no code authority.

## Documentary verification checklist
- [x] `README.md` contains valid STC metadata.
- [x] route posture is planning-only, not coding.
- [x] the shell/non-semantic boundary is explicit.
- [x] same-epic documentary outputs are enumerated and created.
- [x] required reads and conditional reads are explicit.
- [x] blockers and forbidden assumptions are explicit.
- [x] expected future code gate conditions are explicit.

## Failure conditions
This source STC fails if any later reader can honestly conclude:
- GE-07 authorizes UI coding right now
- mock UI state counts as product proof
- the UI is allowed to compute rules truth locally
- diagnostics or explanation visibility are optional niceties rather than product obligations
