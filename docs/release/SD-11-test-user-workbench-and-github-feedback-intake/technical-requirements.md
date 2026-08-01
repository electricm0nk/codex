# Technical Requirements

## Problem Statement
Codex has a real but bounded desktop workbench under `repos/codex/apps/desktop/`, yet it is still a developer proof surface rather than a tester-ready product loop. SD-11 must define the first tester-facing workbench that exposes real bounded domain truth, preserves diagnostics and explanation context, captures actionable feedback into GitHub issues, and presents update posture honestly without turning git-branch mechanics into end-user UX.

## Current-State Facts
- the live desktop app in `repos/codex/apps/desktop/src/App.tsx` currently renders a GE-08 Guard Stance authoring workbench over a real Tauri command boundary
- the current authoring workbench proves that the desktop shell can surface real headless data, provenance, diagnostics, and preview state without fabricating success
- `repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts` and `src-tauri/src/main.rs` still preserve a placeholder GE-07 pilot-character seam rather than a completed bounded character-building workbench
- the repo root README truthfully states that Codex is a developer proof harness plus a buildable desktop workbench surface, not a finished end-user product
- the tranche doctrine already fixes Linux first-class, macOS second-class, and Windows third-class support
- the live operator promotion model is branch-first: `develop -> main`; `beta` remains reserved until a governed candidate promotion surface exists

## Desired Behavior
- define the first tester workbench as a bounded desktop surface over real Codex data rather than mock/demo-only state
- keep explanation, diagnostics, unsupported-scope messaging, and build/channel/support metadata visible enough that testers can report actionable defects
- define bug-report and enhancement-request submission as distinct GitHub issue flows with structured evidence requirements
- define what evidence is auto-captured, what must be typed by the tester, and how attachments/redaction should work
- define an honest update/status surface that maps operator branch truth to tester-facing channel/support semantics without exposing raw branch names as the primary UX
- keep the workbench boundary local-first, desktop-first, and governed by current upstream proof surfaces rather than broad product fantasy

## Architecture Constraints
- the workbench is a consumer of headless/domain truth and must not become rules authority
- the SD-11 packet is planning-only and grants no implementation code authority
- the tester workbench must remain bounded; broad roster coverage belongs to SD-13 and lifecycle/persistence belongs to SD-14
- GitHub is the feedback system of record for this tranche; do not invent a parallel feedback backend here
- platform support promises must remain asymmetric: Linux first-class, macOS second-class, Windows third-class
- update UX must distinguish operator branch truth from tester-facing channel language

## Interfaces / Contracts / Schemas
- **Tester workbench read model** — the future workbench must consume a real bounded snapshot through the desktop/Tauri boundary; placeholder shells are allowed only when visibly labeled as such and never as counterfeit product truth
- **Bug report contract** — the workbench must be able to create a GitHub issue payload containing build/channel/support metadata, workflow path, observed/expected behavior, diagnostic/explanation context, and attachment/redaction rules
- **Enhancement request contract** — the workbench must be able to create a GitHub issue payload containing tester goal, current friction, requested capability, affected surface, and supporting evidence/examples
- **Evidence capture matrix** — the workbench must declare which fields are auto-captured, user-supplied, optional, redacted, or refused
- **Update/channel contract** — the workbench must present current version/build/channel/support state and map it honestly to the live operator promotion flow `develop -> main`, while treating any future `beta` label as unavailable until it gains a governed backing surface

## Required Reads Carried into This Document
- `README.md` — authority, readiness, and packet scope
- `../../plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` — strategic SD-11 boundary
- `../GE-07-desktop-shell-and-modern-ux/README.md` — shell and UI-boundary truth
- `../GE-08-homebrew-authoring-and-rules-studio/README.md` — current real workbench proof surface
- `../GE-10-demo-proof-and-onboarding/README.md` — current-state/onboarding truth
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` — current visible desktop surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts` — current pilot-seam placeholder rule
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` — current real workbench boundary
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` — current runtime seam

## Subsystem Notes
### Tester workbench shell
- responsibilities:
  - expose one bounded tester-facing workbench over real Codex data
  - display current build/channel/support metadata and bounded-scope messaging
  - anchor downstream feedback and update actions in the visible surface
- relevant files:
  - `artifacts/tester-workbench-surface-specification.md`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
  - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- known risks:
  - pressure to treat the current GE-08 workbench as “close enough” and skip the explicit tester boundary definition
  - pressure to hide unsupported scope instead of surfacing it visibly

### Diagnostics and explanation visibility
- responsibilities:
  - keep derived-value explanations, invalid-choice reasons, diagnostics, and blocked-claim context visible enough for tester comprehension and issue filing
  - preserve the anti-counterfeit-success posture already present in GE-08
- relevant files:
  - `artifacts/tester-workbench-surface-specification.md`
  - `artifacts/tester-feedback-evidence-capture-matrix.md`
  - upstream GE-07 / GE-08 / GE-04 requirement surfaces
- known risks:
  - losing explanation context in favor of cleaner UI copy
  - collapsing diagnostics into generic error prose that is useless for triage

### GitHub bug-report intake
- responsibilities:
  - define the bug issue schema, required labels/metadata, error handling, and fallback behavior when submission fails
  - preserve a contract strong enough for later execution handoffs to implement without inventing fields
- relevant files:
  - `artifacts/github-bug-report-intake-contract.md`
  - `artifacts/tester-feedback-evidence-capture-matrix.md`
- known risks:
  - assuming auth or token storage behavior not yet grounded
  - allowing free-form complaints with no structured evidence burden

### GitHub enhancement-request intake
- responsibilities:
  - define the enhancement issue schema, goal/friction/requested-capability fields, and evidence expectations
  - distinguish request-for-capability from defect reporting without losing comparability in triage
- relevant files:
  - `artifacts/github-enhancement-request-intake-contract.md`
  - `artifacts/tester-feedback-evidence-capture-matrix.md`
- known risks:
  - letting enhancement requests become vague product brainstorming detached from actual blocked workflows

### Update/channel and support posture
- responsibilities:
  - define the truthful relation between operator branches, tester-facing channels, build labels, and support tiers
  - state what the updater surface must reveal and what it must never pretend
- relevant files:
  - `artifacts/update-channel-and-promotion-mapping.md`
- known risks:
  - leaking raw git vocabulary into tester UX
  - pretending Windows support is equivalent to Linux support
  - treating “pull from GitHub and do the needful” as a product contract instead of a dev convenience

## Non-Goals
- implementing the workbench UI or GitHub transport in repo code during this planning pass
- broad roster or level-10 class/race execution work that belongs to SD-13
- persistence, save/load, migration, or revision implementation work that belongs to SD-14
- public release engineering, installer design, or release-authority workflow
- cloud services, accounts, mobile UX, or non-GitHub feedback systems

## Decision Boundaries
- Decisions already made:
  - Linux is first-class, macOS is second-class, Windows is third-class for this tranche
  - GitHub is the destination for bug and enhancement intake
  - the live operator promotion path is `develop -> main`, and any future `beta` label must stay reserved until a governed candidate promotion surface exists
  - the tester workbench must not expose raw branch names as primary UX
- Decisions still open:
  - the exact bounded character-building flow that anchors the first tester workbench
  - the exact GitHub auth/storage/posture for issue submission
  - failure behavior when the app cannot submit or cannot update
  - the final tester-facing label set for channel/build/support wording
- Decisions forbidden at this stage:
  - granting code authority from this packet alone
  - claiming general character-builder readiness
  - flattening platform support into fake symmetry
