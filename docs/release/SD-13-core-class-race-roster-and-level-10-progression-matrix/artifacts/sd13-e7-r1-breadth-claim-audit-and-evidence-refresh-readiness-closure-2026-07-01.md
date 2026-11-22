# SD13-E7-R1 Readiness Closure — Breadth-claim audit and evidence-refresh posture

## Card outcome
- evidence_class: `documentary-artifact`
- readiness_verdict: `ready-for-stage-specific-handoff-authoring`
- route truth: this card closes as a documentary readiness artifact only; it does not create repo code, a PR, or a merge surface
- next board move if accepted: mint the same-domain successor `SD13-E7-R2 FLOW: Breadth-claim audit and evidence-refresh handoff artifact`

## Executive route verdict
The first honest SD13-E7 move can advance into a stage-specific handoff artifact now.

It is not yet honest to jump directly from this closure into an unconstrained CODE lane, because the first SD13-E7 slice still needs one explicit handoff to freeze three things in writing:
- the exact audit subject surface (`SD13-E7-F13`, not a broad E7 omnibus)
- the exact freshness posture (what counts as refreshed evidence versus merely displayed evidence)
- the exact repo write scope required to surface stale-claim findings without counterfeiting broader support or cross-lane propagation closure

## Live repo truth grounded on 2026-07-01
- `git -C /home/ubuntu/workspace/repos/codex branch --show-current` returned `feat/sd13-e6-f11-support-state-debt-presentation`.
- `git -C /home/ubuntu/workspace/repos/codex rev-parse HEAD` returned `122de6a60609d9452de53c6d3ad406aeb81c2a82`.
- `git -C /home/ubuntu/workspace/repos/codex rev-parse origin/develop` returned `25765e8c2cb4ed50bd936183b24a2f2189977bc0`.
- `git -C /home/ubuntu/workspace/repos/codex merge-base --is-ancestor e9de753762c3aef568e19adcdda22ce0cd2b39f0 HEAD` exited `1`, while the same check against `origin/develop` exited `0`. That means the live local worktree is still on the pre-merge feature branch even though the accepted SD13-E6-F11 merge commit is present on `origin/develop`.
- `git -C /home/ubuntu/workspace/repos/codex status --short --branch` showed `## feat/sd13-e6-f11-support-state-debt-presentation...origin/feat/sd13-e6-f11-support-state-debt-presentation` plus untracked `apps/desktop/src-tauri/gen/`. A later repo-facing lane must therefore refresh from `origin/develop` instead of treating the current local branch as the authoritative launch substrate.
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs` exists as the live typed SD-13 carrier. It preserves `support_state`, `evidence_tier`, `grounding_ref`, `blocker_or_lossiness_note`, and `next_required_uplift`, but it does not yet carry explicit evidence-refresh timestamp metadata.
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs` exists and proves the seeded 21-row matrix shape and anchor invariants.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` already projects the typed SD-13 carrier into a read-only desktop snapshot with canonical tester-facing wording.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts` already loads that snapshot through the existing Tauri command instead of reinterpreting markdown in the UI.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` already maps the SD-13 snapshot into a bounded `supportDebt` presentation for the SD-11 tester workbench.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` already wires `loadSd13SupportStateMatrix` into the live runtime path. No new runtime dependency-threading seam needs to be invented for the first E7 slice.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` already renders the support/debt section, including raw state token, evidence tier, canonical wording, blocker/lossiness note, grounding reference, next uplift, and stable row id. The live tester workbench is already consuming SD-13 truth visibly.
- `cargo test --test sd13_support_state_matrix` passed with 18 tests green.
- `cargo test --manifest-path /home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/Cargo.toml sd13_support_state_matrix` passed with 8 targeted bridge/unit tests green.
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck`, `npm run build`, and `npm run tauri:check` all passed on the live repo.
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` still exposes `typecheck`, `build`, and `tauri:check`, but no governed test runner for the existing TypeScript `*.test.ts` files. The first E7 handoff must not counterfeit executable TypeScript behavioral proof that the repo does not currently wire.

## Why the route is now concrete enough for a handoff artifact
1. The SD-13 truth carrier is no longer hypothetical.
   - `src/rules_core/support_state_matrix.rs` and `tests/sd13_support_state_matrix.rs` provide a real typed matrix surface.
   - the desktop Tauri bridge and TypeScript boundary already consume that carrier instead of improvising local labels.

2. The first live breadth-consumer surface already exists.
   - `loadSd11TesterWorkbenchSurface.ts` is now the real aggregation point for tester-visible SD-13 support/debt truth.
   - `App.tsx` already exposes the support/debt surface where future stale-claim findings can be surfaced without inventing a new product entry point.

3. The next honest E7 slice is narrower than “cross-lane propagation.”
   - `epic-breakdown.md` splits `SD13-E7` into `SD13-F13 — Evidence-refresh and stale-claim audit surface` and `SD13-F14 — Cross-lane propagation rules`.
   - the current repo grounding is strong enough to author a handoff for `F13` now, while leaving `F14` explicitly deferred.

4. The remaining ambiguity is handoff-shape ambiguity, not missing truth.
   - the repo already tells us where claims are rendered
   - the repo already tells us where matrix truth originates
   - the only unresolved question is where freshness metadata should live for the first audit slice: inside the typed SD-13 carrier and its projection, or only as looser downstream heuristics

## Why this card should not jump straight into CODE
- `SD13-E7` is the first lane that must distinguish “visible current-state truth” from “fresh evidence strong enough to support a claim today.” The repo does not yet encode explicit evidence-refresh timestamps in the typed carrier, so a code lane needs one written handoff to freeze how freshness is represented before code starts.
- `SD13-E7-F14` cross-lane propagation remains a broader systems concern than the first honest slice. The first code lane should not quietly expand from workbench claim audit into SD-12 or SD-14 synchronization logic.
- The current local checkout is not the authoritative accepted substrate; `origin/develop` is. A code-authorizing lane must encode fresh-branch launch instructions explicitly instead of assuming the current feature branch is the right base.

## Exact required reads for the next handoff artifact
The stage-specific handoff produced by `SD13-E7-R2` should require reading exactly these surfaces:
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/README.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/acceptance-and-verification.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r2-unsupported-depth-diagnostics-and-tester-visible-reporting-execution-handoff-2026-06-30.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e7-r1-breadth-claim-audit-and-evidence-refresh-readiness-closure-2026-07-01.md`
- `/home/ubuntu/workspace/repos/codex/README.md`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`

## First honest downstream slice the handoff should authorize
The first repo-facing SD13-E7 implementation slice should be `SD13-E7-F13 — Evidence-refresh and stale-claim audit surface`, not a broad `SD13-E7` omnibus and not `SD13-E7-F14` cross-lane propagation.

That means:
- consume the existing SD-13 matrix carrier and existing SD-11 support/debt workbench surface
- add one bounded freshness/audit posture that can say when visible claim language outruns current matrix truth or rests on stale/missing refresh metadata
- keep audit findings separate from support-state ownership itself
- preserve `SD13-E7-F14` as explicitly deferred rather than smuggling propagation automation into the first slice

## Exact candidate repo/doc surfaces for the later handoff
### Candidate primary write surfaces for the first honest `SD13-E7-F13` code lane
1. `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
   - candidate carrier change only if the handoff decides freshness metadata must live in typed SD-13 truth rather than in downstream UI heuristics
   - any change here must remain documentary/control-plane only: no support-state promotion logic, no row recomputation, no propagation engine

2. `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
   - companion proof surface if the typed carrier gains explicit refresh metadata or new invariants
   - must continue proving row-count, state/evidence separation, and anti-counterfeit claims

3. `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
   - projection surface if the typed carrier snapshot gains freshness or audit-relevant metadata
   - this remains a read-only bridge, not an audit engine

4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
   - TypeScript boundary surface if the Tauri snapshot shape changes
   - no markdown parsing, no local state promotion, no side effects

5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - primary SD-11 aggregation seam for deriving bounded stale-claim / evidence-refresh posture from the returned SD-13 snapshot
   - the audit summary must remain subordinate to the row truth it reads

6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
   - primary tester-visible audit presentation surface
   - may add a bounded claim-audit or freshness section, warning card, or summary block, but must not recast `partial`, `lossy`, `blocked`, or `unverified` as generic success language

### Conditional surfaces that should stay read-only unless the handoff proves they are required
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts`
  - already wires the SD-13 loader today; do not touch unless the dependency signature itself changes
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs`
  - the Tauri command is already registered today; do not touch unless the handoff proves a second command is necessary, which should be treated as a widening risk
- `/home/ubuntu/workspace/repos/codex/README.md`
  - read-only grounding surface for product-claim posture; not part of the first write scope unless a later lane is explicitly chartered to refresh non-workbench product wording

### Read-only grounding surfaces for this lane
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/coverage-evidence-and-fixture-plan.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/tester-facing-support-language-contract.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md`
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`

## Exact allowed write scope the later handoff should freeze for the first code lane
Unless the handoff finds a harder blocker than the current repo evidence shows, the first `SD13-E7-F13` handoff should freeze write authority to exactly these repo paths:
- `/home/ubuntu/workspace/repos/codex/src/rules_core/support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/tests/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/sd13_support_state_matrix.rs`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`

Write-scope interpretation:
- the `rules_core` carrier and its test are in scope only to encode or prove bounded freshness metadata required for the first audit surface
- the Tauri and TypeScript boundary files are in scope only to project existing or newly added read-only metadata
- `loadSd11TesterWorkbenchSurface.ts` is the only authorized aggregation seam for turning snapshot truth into a bounded claim-audit posture
- `App.tsx` is the only authorized tester-visible presentation seam for the first E7 slice
- `loadSd11TesterWorkbenchSurfaceRuntime.ts` and `main.rs` are not in the default first-slice write scope because the loader and command already exist

## Exact authority split the later handoff must preserve
### SD-13 remains the owner of
- support-state taxonomy
- evidence tier meaning
- grounding references
- next required uplift semantics
- any newly introduced refresh/freshness metadata
- approved tester-facing support wording
- what counts as stale, missing, or insufficient evidence for a breadth claim

### SD-11 remains the owner of
- tester-workbench structure and layout
- where an audit warning or freshness section appears in the workbench
- bounded presentation behavior and framing
- how the workbench separates claim-audit findings from other diagnostics

### The first E7 slice may consume from SD-13
- `row_id`
- `subject_id`
- `dimension`
- `support_state`
- `evidence_tier`
- `grounding_ref`
- `blocker_or_lossiness_note`
- `next_required_uplift`
- any explicitly added freshness metadata the handoff authorizes
- approved wording from `tester-facing-support-language-contract.md`

### The first E7 slice must not infer or upgrade
- UI presence into support
- row visibility into freshness
- build success into breadth completion
- Tauri compile success into refreshed evidence
- shipped/tester-build availability into external-cycle closure
- workbench wording improvements into cross-lane propagation completion
- SD-11-local labels into a replacement for SD-13 state words

## Exact non-goals for the later handoff
The next handoff must state these non-goals plainly:
- no new roster semantics, class progression, spellcasting burden, or support-state promotion work
- no edits to adjacent SD-12 distribution/update behavior or SD-14 persistence/lifecycle behavior
- no `SD13-E7-F14` propagation engine, propagation automation, or downstream lane rewiring in the first slice
- no GitHub issue-capture redesign, no feedback-evidence schema widening, and no `SD13-E6-F12` coupling work
- no multiclassing, non-core, archetype, prestige-class, or broader Pathfinder expansion
- no new command surface in `main.rs` unless the handoff proves the existing `load_sd13_support_state_matrix` route cannot carry the bounded audit metadata honestly
- no claim that a row is fresh merely because it still renders in the workbench
- no claim that a row is externally validated, parity-checked, or tester-confirmed unless the carrier and evidence surface actually record that fact
- no edits under `/home/ubuntu/workspace/programs/codex/**` during the code lane
- no edits to unrelated desktop feedback, update, packaging, or release surfaces outside the exact write scope above

## Exact verification commands and evidence surfaces for the later handoff
### Commands that are runnable and green today
Run these from the named directories:

```bash
cd /home/ubuntu/workspace/repos/codex && . "$HOME/.cargo/env" && cargo test --test sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex/apps/desktop && . "$HOME/.cargo/env" && cargo test --manifest-path src-tauri/Cargo.toml sd13_support_state_matrix
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Interpretation:
- the first command proves the typed `rules_core` carrier still preserves the seeded row truth and anti-counterfeit invariants
- the second command proves the Tauri bridge still projects the SD-13 snapshot and canonical wording truthfully
- `npm run typecheck` proves the bounded TypeScript/React surface remains coherent
- `npm run build` proves the current desktop bundle still compiles
- `npm run tauri:check` proves the desktop crate still checks against the live root crate after any bounded snapshot changes

### Evidence surfaces that exist but are not truthfully runnable today
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`

Current truth:
- the repo contains TypeScript proof files in this area, but `apps/desktop/package.json` still does not declare a governed test runner for them
- therefore the first E7 handoff must not name TypeScript behavioral test execution as a required verification command unless it first authorizes and grounds a real runner surface

## Readiness verdict
This lane is ready for stage-specific handoff authoring now.

Why it is ready:
- the support/debt bridge that E7 depends on is merged on `origin/develop` and reflected in real repo files
- the live workbench already renders the exact row/evidence/debt atoms that the first audit slice must inspect
- the repo already exposes a typed SD-13 carrier, a desktop snapshot, and a tester-visible presentation seam
- the runnable verification floor is real and green today
- the remaining uncertainty is narrow enough to freeze in one explicit handoff artifact rather than block the route

Why the next move is still a FLOW handoff artifact rather than CODE immediately:
- the first E7 slice must decide how freshness metadata is represented before code starts
- the first E7 slice must explicitly defer `SD13-E7-F14` instead of leaving propagation scope implicit
- the later code lane must launch from fresh `origin/develop`, not from the current pre-merge local feature branch

## Successor truth
The earned successor is:
- `SD13-E7-R2 FLOW: Breadth-claim audit and evidence-refresh handoff artifact`

That successor should author the bounded code-authorizing brief for `SD13-E7-F13` only. `SD13-E7-F14` should remain explicitly deferred unless the later handoff proves a smaller truthful split is impossible.