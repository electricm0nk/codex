---
title: GE06-E4-F1 Launch Posture Re-evaluation
artifact_type: route-posture-review
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E4-F1 — Pilot view-model contract from real outputs
workflow_route: planning
readiness: planning-ready
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge06-post-f3-handoff-rack-2026-06-21.md
  - ../../GE-07-desktop-shell-and-modern-ux/README.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md
  - ../../GE-07-desktop-shell-and-modern-ux/artifacts/pilot-ux-flow-requirements.md
  - ../../../research/codex-reference-architecture-2026-06-17.md
---

# GE06-E4-F1 Launch Posture Re-evaluation

## Verdict
The stale blocker is cleared.

GE06-E4-F1 is no longer blocked because "GE-07 is still only a spec domain." GE-07 now exists as a planning-ready source STC with explicit shell, component-surface, command-boundary, and UI-truth requirements.

GE06-E4-F1 is still not ready for a live code-authorizing handoff.

The truthful next posture is a bounded pre-viability spike, not a broad product-authorizing UI lane.

## Why the old blocker is stale
The old rack premise was that GE06-E4-F1 lacked an accepted UI authority surface. That is no longer true.

GE-07 now provides:
- an accepted source STC at `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md`
- a command-boundary contract at `artifacts/ui-command-boundary-requirements.md`
- a bounded surface inventory at `artifacts/component-surface-inventory.md`
- a pilot UX flow contract at `artifacts/pilot-ux-flow-requirements.md`
- an explicit rule that UI work remains non-authorizing until GE-06 viability is accepted or a bounded spike posture is authorized

That clears the planning-authority gap.

## Grounded evidence available now
| Evidence surface | What it proves now | What it still does not prove |
|---|---|---|
| `../../GE-07-desktop-shell-and-modern-ux/README.md` | GE-07 exists as a planning-ready source STC and explicitly forbids broad UI authority without a later grounded handoff. | A ready-to-run GE06-E4-F1 coding brief. |
| `../../GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md` | The first shell must consume structured character snapshot, explanation, validation/problem, import-diagnostic, and bounded rules payloads. | Final transport, final frontend stack, or final repo file layout. |
| `../../GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md` | The first UI slice must keep diagnostics visible and treat the pilot character workspace as the center of gravity. | That those surfaces already exist in the repo. |
| `../../GE-07-desktop-shell-and-modern-ux/artifacts/pilot-ux-flow-requirements.md` | The first UI truth claim must load the pilot from real domain outputs and reject hardcoded-number proof. | A concrete verification command set for the current repo. |
| `../../../research/codex-reference-architecture-2026-06-17.md` | The preferred repo shape for future desktop work is `apps/desktop/src-tauri/` plus `apps/desktop/ui/`. | That those paths already exist or are ready for immediate write authority. |
| Live repo observation on 2026-06-22 | `/home/ubuntu/workspace/repos/codex` currently contains Rust core/test surfaces only; `apps/desktop/`, `apps/desktop/src-tauri/`, and `apps/desktop/ui/` are absent. Local branch is `ge06-e2-f3-headless-receipt-path` at `0b84fdc70acd5a4fca8e9d91f8be8ca02bc18acf`; `origin/develop` is `6977c862d7e0f40e105b0360ac34f36e18dccd43`. | A grounded UI write scope or shell scaffold already present in the repo. |
| Live toolchain observation on 2026-06-22 | `node` and `npm` are present in this Hermes runtime. | Tauri/Rust scaffold readiness in this runtime; `cargo` and `rustc` were not available here when checked. |

## Decision table
| Question | Answer |
|---|---|
| Is GE06-E4-F1 still blocked because GE-07 was only a spec domain? | No. That blocker is obsolete. |
| Is GE06-E4-F1 ready for a live code-authorizing handoff now? | No. The route is not grounded tightly enough. |
| What is the truthful next posture? | A bounded pre-viability spike posture. |

## What can be derived now
What is derivable now is documentary and routing truth, not live coding authority.

That means:
1. the current rack can stop claiming GE-07 is only a spec domain
2. GE06-E4-F1 can be described as a bounded pre-viability spike candidate
3. the future live artifact family can be named without pretending it is launchable now:
   - `ge06-e4-f1-execution-readiness-closure-YYYY-MM-DD.md`
   - `ge06-e4-f1-execution-handoff-YYYY-MM-DD.md`
   - `ge06-e4-f1-merge-receipt-YYYY-MM-DD.md`

## Remaining gates before a live handoff exists
1. Decide the route truthfully.
   - Either wait for GE-06 viability review to bless a product-visible next step.
   - Or explicitly authorize a bounded non-production spike before that verdict.

2. Ground exact repo paths and write scope.
   - The reference architecture suggests `apps/desktop/src-tauri/` and `apps/desktop/ui/`.
   - The live repo does not yet contain those paths.
   - A later readiness closure must turn the suggested directory classes into exact writable files.

3. Verify the runtime/toolchain in the target execution surface.
   - Node/npm exist here.
   - Cargo/rustc were not available in this Hermes runtime when checked.
   - Therefore a Tauri-backed shell spike is not yet grounded as runnable in this exact environment.

4. Define spike-specific verification receipts.
   The first live E4-F1 slice should prove only a narrow truth set, such as:
   - the shell scaffold exists and boots or otherwise produces a bounded startup receipt
   - the shell can load real GE-06 headless receipt data or an explicit blocked/no-data state
   - the pilot character workspace consumes real data rather than hardcoded character truth
   - diagnostics visibility is not suppressed, even if deeper explanation and export surfaces remain deferred to E4-F2 / E4-F3

## Smallest honest first slice after gate clear
If Todd chooses to move before a full GE-06 viability verdict, the first honest E4 slice should be a non-production shell spike that does only this:

1. establish the smallest desktop-shell scaffold consistent with the GE-07 reference posture
2. mount one pilot-character workspace surface
3. feed that surface from real GE-06 headless receipt inputs or an explicit blocked-state carrier
4. preserve visible diagnostics/problem posture
5. stop short of explanation-surface depth, export breadth, or broad packaging claims

This keeps E4-F1 narrower than the whole GE-07 epic and prevents counterfeit "product-visible" claims from mock state.

## Explicit non-authorizations
This review does not authorize:
- a broad "build the UI" lane
- final Tauri/React stack decisions
- final command transport
- repo-wide restructuring beyond the later bounded write scope
- explanation-panel implementation (E4-F2)
- export-summary implementation (E4-F3)
- any claim that GE-06 viability is already proven

## Completion rule
This artifact is complete when it leaves no ambiguity about three facts:
1. the old GE-07 blocker is stale
2. the next truthful E4 posture is a bounded pre-viability spike
3. the remaining gates are repo/write-scope grounding, toolchain verification, and spike-specific receipts rather than missing planning authority
