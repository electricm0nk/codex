---
title: GE07-E1 Runtime Boundary ADR Input
artifact_type: adr-input
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E1 — Shell scaffold and runtime boundary spike
workflow_route: planning
readiness: planning-ready
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge07-e1-shell-scaffold-receipt-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ../../../research/codex-reference-architecture-2026-06-17.md
---

# GE07-E1 Runtime Boundary ADR Input

## Decision prompt
What is the smallest truthful runtime boundary for the first GE-07 shell spike?

## Recommended answer
Start with a read-only desktop shell over the already-grounded GE-06 headless receipt path.

Do not start with editable character state, rules browsing breadth, or a generalized RPC fabric.

The first boundary should prove only that a desktop shell can boot, request real headless pilot data, and surface computed-versus-blocked state plus diagnostics without inventing semantic truth locally.

## Boundary doctrine
### 1. Keep the core sovereign
The existing Rust crate at repo root remains the domain authority.

Current bounded upstream surfaces already present in the repo:
- `src/rules_core/character_input.rs` — chosen-input fixture loader and validation diagnostics
- `src/rules_core/pilot_compute.rs` — GE-06 pilot headless receipt builder and explanation/diagnostic carriers
- `src/rules_core/pilot_failure.rs` — primary failure-owner classifier over the headless receipt

GE07-E1 should consume those surfaces through a desktop adapter. It should not replace them.

### 2. Keep the shell additive
The desktop shell should live under a future `apps/desktop/` subtree so the repo continues to communicate one clear truth:
- root crate = headless core
- desktop app = presentation/runtime shell

### 3. Keep the first transport boring
The first honest transport choice is direct Tauri command invocation over library calls.

Why:
- it proves the shell/runtime seam without inventing a network/service architecture
- it keeps the non-production spike narrow
- it preserves the ability to swap to a richer internal boundary later if the shell survives the spike

This is a provisional runtime answer, not a final platform constitution.

## Minimum command/read-model contract
The first shell spike needs one narrow read model, not a large API catalog.

### Proposed read model
```text
PilotShellSnapshot
  case_id
  source_package_id
  receipt_status              # Computed | Blocked
  primary_failure_owner?      # present only when derivable honestly
  summary_values              # only already-grounded GE-06 outputs
  diagnostics[]
  explanation_refs[]          # identifiers/details the UI can reveal, not UI-owned math
```

### Proposed first commands
1. `load_pilot_shell_snapshot()`
   - returns one bounded read model derived from the real GE-06 headless receipt path
2. `get_value_explanation(explanation_id)`
   - optional for E1, but acceptable if it simply forwards already-grounded explanation records

Everything else should wait.

Not yet allowed in E1:
- mutable character editing
- broad rules-library browsing
- packaging/update flows
- UI-local recomputation of values, prerequisites, or explanation logic
- fake sample data that stands in for GE-06 truth

## State ownership split
### UI-owned state
- route/view selection
- active panel/drawer state
- selected explanation focus
- loading/error presentation state

### Core-owned state
- authoritative character values
- blocked/computed status
- diagnostics
- explanations
- primary failure-owner classification
- provenance/source lineage when surfaced

This split matters because GE-07 is allowed to orchestrate visibility, not invent domain answers.

## First-slice acceptance shape
The first runtime-boundary spike is successful when it can prove all of the following without exaggeration:
1. a desktop shell scaffold exists and can start or otherwise produce a bounded startup receipt
2. the shell can request a real GE-06-derived snapshot through an explicit boundary
3. the shell can show either `Computed` or `Blocked` honestly
4. diagnostics remain visible when present
5. no claim is made that the shell is already a product-grade character builder

## Open ADR questions preserved for later
These remain intentionally unresolved after E1:
- Is React final, or merely the current preferred binding?
- Does the long-term boundary stay on direct Tauri commands, or graduate to a richer internal RPC/service layer?
- What exact explanation presentation pattern survives into E4: drawer, split pane, tab, or something else?
- When do rules-library and source-package inspection surfaces join the first shell?
- What packaging/signing decisions deserve their own ADR instead of being smuggled into shell work?

## Decisive recommendation
If Todd authorizes a later GE07-E1 execution lane, the resulting readiness closure should target exactly one proof burden:

Build the smallest additive Tauri desktop scaffold under `apps/desktop/` and wire one read-only pilot snapshot boundary over the real GE-06 headless receipt path.

Everything broader is noise at this stage.
