# SD13-E6-R1 Readiness Closure — Unsupported-depth diagnostics and tester-visible reporting

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `ready-for-stage-specific-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not create repo code, a PR, or a merge surface
- next board move if accepted: mint the same-domain successor `SD13-E6-R2 FLOW: Unsupported-depth diagnostics and tester-visible reporting handoff artifact`

## Executive route verdict
The first downstream SD13-E6 move should not remain stuck at documentary/control-plane analysis. It can advance into a stage-specific execution handoff artifact now.

It is not yet honest to jump directly from this closure into an unconstrained CODE lane, because the exact desktop/runtime bridge shape and the proof contract still need to be frozen in one explicit handoff. But the route is concrete enough to author that handoff now.

## Live repo truth grounded on 2026-06-30
- `git -C /home/ubuntu/workspace/repos/codex rev-parse --abbrev-ref HEAD` returned `feat/sd13-e1-f1-rules-core-support-state-matrix`.
- `git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD` returned `3827378a5bfe6dda22ad18695140d7f4fa723a5f`.
- `git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop` returned `c2cea5c6baeb3ca34077b85331214c4b42a4809c`.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` now exists as the first machine-usable SD-13 truth carrier, and `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` proves the seeded current-truth row set.
- `cargo test --test sd13_support_state_matrix` passed on the live repo during this card with 18 tests green.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` already renders an SD-11 tester workbench frame, feedback composers, evidence capture, and update/status surfaces. The tester workbench is no longer hypothetical surface doctrine only.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` already maps a real bounded GE-08 snapshot or explicit pilot fallback into the tester-facing SD-11 workbench model, including diagnostics, blocked claims, explanation refs, provenance refs, and support-tier/channel truth.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts` already auto-captures the current workbench diagnostics, blocked claims, explanation refs, and provenance refs into structured feedback payload state. That means SD13-E6 does not need to invent an issue-evidence channel from nothing; it needs to extend an existing one honestly.
- `npm run typecheck`, `npm run build`, and `npm run tauri:check` all passed during this card from `/home/ubuntu/workspace/repos/codex/apps/desktop`.
- Existing TypeScript behavioral proof files already exist under `apps/desktop/src/sd11/**/*.test.ts`, but the repo does not currently expose a truthful script/runner for them in `package.json`. Direct `node` execution of representative tests failed with `ERR_MODULE_NOT_FOUND` on extensionless TypeScript imports. The later handoff must treat those files as proof surfaces and runner-gap evidence, not pretend they are currently wired as executable verification commands.

## Why the route is now concrete enough for a handoff artifact
1. SD-13 already owns the required truth atoms in concrete form.
   - `artifacts/core-roster-and-support-state-matrix.md` defines row state, evidence tier, grounding, and next uplift.
   - `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` defines visible debt, who must see it, and why it is not supported.
   - `artifacts/tester-facing-support-language-contract.md` defines the only approved tester wording.

2. The repo already contains the exact downstream SD-11 consumption seam.
   - `loadSd11TesterWorkbenchSurface.ts` is the current aggregation point for bounded workflow truth.
   - `App.tsx` is the current tester-visible rendering surface.
   - `captureFeedbackEvidence.ts` is the current issue/report packaging seam.

3. The repo already contains an SD-13 machine-usable source rather than only markdown.
   - `support_state_matrix.rs` plus `sd13_support_state_matrix.rs` make the roster/debt surface consumable without forcing SD-11 to reinterpret markdown prose.

4. The runnable verification floor is real today.
   - the SD-13 carrier test passes
   - the desktop TypeScript build/typecheck passes
   - the Tauri bridge compiles against the live root crate

## Why this card should not jump straight into CODE
- The later implementation must freeze whether SD-13 data reaches the tester workbench through a new read-only Tauri command, through an expanded existing desktop boundary contract, or through another explicitly bounded adapter. That choice is architectural enough to deserve an explicit handoff artifact before code starts.
- The existing SD-11 TypeScript behavioral tests are real proof surfaces, but not yet repo-runnable through a declared command. A code-authorizing lane must name the exact runner truth instead of implying one exists.
- The authority split between SD-13 and SD-11 is sharp enough that the first code lane must be constrained in writing: SD-11 may render and package SD-13 truth, but must not become a second support-taxonomy owner.

## Exact required reads for the next handoff artifact
The stage-specific handoff produced by `SD13-E6-R2` should require reading exactly these surfaces:
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/diagnostics/buildSd11WorkbenchEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

## First honest downstream slice the handoff should authorize
The first repo-facing SD13-E6 implementation slice should be a read-only support/debt reporting bridge over the existing SD-11 tester workbench.

That means:
- consume SD-13 truth from the existing matrix carrier
- expose the selected row/debt/evidence/uplift data to the SD-11 workbench
- render the data with SD-13-approved wording only
- preserve the same truth in structured feedback evidence capture
- avoid all claim promotion, breadth recomputation, release inference, or persistence inference

This is narrower and more truthful than a broad “reporting feature” lane.

## Exact candidate repo/doc surfaces for the later handoff
### Candidate primary write surfaces
1. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - render the tester-visible support/debt surface and keep unsupported-depth truth visible
   - may add a bounded panel, card group, or evidence block, but must not invent local support labels

2. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - extend the SD-11 workbench model so SD-13 row/debt/evidence data becomes part of the bounded tester surface
   - preserve the fallback-versus-real-data distinction

3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/diagnostics/buildSd11WorkbenchEvidence.ts`
   - map any new SD-13 explanation/evidence/debt references into the existing tester-visible evidence model
   - keep triage-grade detail rather than flattening it into generic warnings

4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
   - auto-capture the SD-13 state/debt/evidence context into issue payload assembly when available
   - preserve the distinction between tester-entered narrative and system-captured roster truth

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
   - candidate new TypeScript boundary file for a dedicated SD-13 read-only desktop loader if the handoff chooses a separate command instead of widening the GE-08 snapshot contract

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
   - candidate new Tauri-side read-only adapter over `rules_core::support_state_matrix` if the handoff chooses a dedicated command
   - this must stay carrier/adapter only; no rules computation, no row promotion, no persistence, no issue transport

7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
   - command registration and bridge entrypoint for any new read-only SD-13 desktop command

8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`
   - focused proof surfaces for the bounded TypeScript behavior
   - the later handoff must name exactly how these will be executed truthfully if it puts them in write scope

### Read-only grounding surfaces for this lane
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-workbench-surface-specification.md`

## Exact authority split the later handoff must preserve
### SD-13 remains the owner of
- support-state taxonomy
- row truth and row identity
- debt reason / blocker-or-lossiness reason
- evidence tier and grounding reference
- next required uplift / owning future slice
- breadth-claim composition rules
- approved tester-facing support wording contract

### SD-11 remains the owner of
- tester-workbench structure and layout
- panel ordering, frame, and bounded workflow presentation
- issue-flow affordances and evidence packaging mechanics
- fallback handling, update/status framing, and workbench behavior
- how captured evidence is assembled into bug/enhancement payloads

### Later tester-visible/reporting surfaces may consume from SD-13
- `row_id`
- `subject_id`
- `dimension`
- `support_state`
- `evidence_tier`
- `grounding_ref`
- `blocker_or_lossiness_note`
- `next_required_uplift`
- approved wording derived from `tester-facing-support-language-contract.md`

### Later tester-visible/reporting surfaces must not infer or upgrade
- UI presence into support
- dropdown/list presence into support
- `partial`, `lossy`, `blocked`, or `unverified` into “ready”, “works”, or parity language
- distribution/update success into roster support truth
- persistence/save success into roster support truth
- generic bug language that suppresses whether the roster path is blocked, lossy, or merely unverified
- local SD-11 wording that outranks SD-13-approved state language

## Exact non-goals for the later handoff
The next handoff must state these non-goals plainly:
- no edits to `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` or `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` unless a later same-domain SD-13 lane explicitly widens scope
- no support-state promotion, demotion, or recomputation inside SD-11 or the desktop layer
- no new roster semantics, class progression, spellcasting burden, or breadth implementation work
- no multiclassing, non-core, archetype, prestige-class, or other tranche widening
- no SD-12 distribution/update-channel work beyond preserving existing status truth in the frame
- no SD-14 persistence/lifecycle work
- no GitHub auth/storage/transport redesign; issue transport remains whatever SD-11 already authorizes
- no claim-composition engine that lets the desktop infer broader support than the matrix/ledger state proves
- no hiding of blocked/partial/lossy/unverified debt behind generic UX polish
- no new serializer, persistence format, or broad export/report pipeline merely to move matrix rows into the tester workbench

## Exact verification commands and evidence surfaces for the later handoff
### Commands that are runnable and green today
Run these from the named directories:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Interpretation:
- the first command proves the SD-13 carrier rows and invariants still hold
- `npm run typecheck` proves the bounded TypeScript desktop surface still typechecks
- `npm run build` proves the Vite/React desktop bundle still compiles
- `npm run tauri:check` proves the desktop Tauri bridge still compiles against the live root crate and any read-only SD-13 adapter it consumes

### Evidence surfaces that exist but are not yet truthfully wired to a runner
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`

Current truth:
- the files exist and encode bounded behavior expectations
- direct `node` execution currently fails on module resolution for extensionless TypeScript imports
- therefore the next handoff must either name a real runner/setup step for these tests or keep them read-only and rely on the runnable command set above

## What the later tester-visible/reporting surfaces may say, and what they may not say
### May say
- the named roster path is `partial`, `lossy`, `blocked`, or `unverified`
- the blocking reason or debt reason recorded by SD-13
- the evidence reference or explanation/provenance pointer that grounds the visible state
- the next uplift owner/slice when that helps issue routing
- the approved tester-facing wording from the SD-13 language contract

### Must not say
- “supported” when any relevant row is not `supported`
- “ready” or “works” without state linkage
- “parity” without the required higher-order evidence
- “bug” when the real condition is documented unsupported breadth or explicit blocked semantics
- “fixed by shipping/build/update/save success” when the row itself remains partial, blocked, lossy, or unverified

## Readiness verdict
This lane is ready for stage-specific handoff authoring now.

Why it is ready:
- the authoritative SD-13 truth atoms are now available both in docs and in the live repo carrier
- the SD-11 tester workbench and evidence-packaging surfaces already exist and are concrete
- the live verification floor is strong enough to anchor a bounded handoff without guessing
- the authority split is explicit enough to write a precise non-goal set
- the only remaining ambiguity is handoff-shape ambiguity, not missing program truth

Why the next move is still a FLOW handoff artifact rather than CODE immediately:
- the exact write scope, command contract, and test-runner truth must be frozen once before a Claude-only CODE card is minted
- the handoff must encode the runner-gap truth for existing TypeScript proof files instead of letting a coding lane discover it ad hoc
- the handoff must preserve the doctrine that SD-11 renders and packages SD-13 truth but does not become its new owner

## Successor truth
The earned successor is:
- `SD13-E6-R2 FLOW: Unsupported-depth diagnostics and tester-visible reporting handoff artifact`

That successor should author the bounded code-authorizing brief for the first SD13-E6 implementation lane. The later CODE lane should be Claude-governed and must require a durable `claude-execution-receipt` before review/closeout.