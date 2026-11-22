---
title: GE06-E4-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E4-F1 — Pilot view-model contract from real outputs
workflow_route: readiness-closure
readiness: codex-ready
handoff_created: true
created_handoff:
  - ./ge06-e4-f1-execution-handoff-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E4-F1 Execution Readiness Closure

## Verdict
GE-06 is now grounded sufficiently to mint the next narrow code-producing handoff for the pilot view-model contract from real outputs, and that paired artifact now exists.

The active E4-F1 code-authorizing artifact created from this readiness closure is:

```text
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
```

This readiness closure is not code authority. It records why the separate E4-F1 handoff now truthfully carries `code_authority: true` while the root `execution-handoff.md` remains a route surface.

## Core problem
GE-06 now has a merged headless receipt path, a selected parity-dimension carrier, and a primary failure-owner classifier, but it still lacks the next narrow bridge that turns those real domain outputs into a UI-consumable contract without fabricating shell behavior or hardcoded character data.

The smallest honest next move is not Tauri scaffolding, not a TypeScript shell, and not a product-visible screen. It is one bounded rules-core view-model contract that projects the deterministic pilot's real computed snapshot or explicit blocked posture into a machine-checkable read-model lane for later GE06-E4-F2 and GE06-E4-F3 work.

## Selected bounded slice

```text
GE06-E4-F1 — Pilot view-model contract from real outputs
```

This slice should do only four things:

1. consume the merged GE-06 headless receipt and failure-classifier surfaces as read-only input
2. emit one bounded pilot character view-model / snapshot contract for the accepted deterministic pilot when the receipt is `Computed`
3. emit explicit blocked posture plus real diagnostics when the receipt is `Blocked`, rather than zero-filled faux success data
4. preserve real explanation payloads or stable explanation references so later inspection work can stay downstream of real domain outputs

This slice does not authorize desktop shell scaffolding, Tauri setup, TypeScript/React work, JSON/serde plumbing, parity logic, export logic, importer changes, or rules-engine rewrites.

## Required source evidence recovered

| Gate | Evidence |
|---|---|
| Upstream merge truth | `git rev-parse --short origin/develop` now returns `7bc89e8`; `git log --oneline -5 origin/develop` shows `7bc89e8 Merge pull request #16 from electricm0nk/ge06-e3-f2-classifier-impl` and `b2f2154 Merge pull request #15 from electricm0nk/ge06-e3-f1-selected-parity-dimensions`. |
| Baseline proof commands pass | `"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet`, `"$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet`, `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet`, and full `"$HOME/.cargo/bin/cargo" test --quiet` all pass in `/home/ubuntu/workspace/repos/codex`. |
| Headless receipt surface exists | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs` exposes `PilotHeadlessReceipt`, `HeadlessReceiptStatus`, and `build_pilot_headless_receipt`. |
| Failure-owner surface exists | `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_failure.rs` exposes `FailureClassifier` and the required five-owner vocabulary including `OracleGap` and `UiGap`. |
| Selected-dimension precedent exists | `/home/ubuntu/workspace/repos/codex/src/oracle_validation/selected_parity_dimensions.rs` already proves the repo can add a bounded projection layer over real pilot receipt outputs without widening into parity verdict logic. |
| Real explanation and diagnostic payloads already exist | `PilotBaseChassisComputation` carries `explanations` and `diagnostics`, with stable ids such as `ability_modifier.strength`, `class_chassis.base_attack_bonus`, `combat.baseline_melee_attack_bonus`, `defense.baseline_armor_class`, `defense.total_save.fortitude`, and `skill.selected_modifier.climb`. |
| GE-07 UI consumer boundary exists | `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-requirements.md`, `technical-design.md`, `artifacts/ui-command-boundary-requirements.md`, and `artifacts/component-surface-inventory.md` require character snapshot/state, explanation visibility, and validation/problem payloads while forbidding frontend-owned rules truth. |
| No current view-model / snapshot module exists | file searches under `/home/ubuntu/workspace/repos/codex/src` for `*view*` and `*snapshot*` return zero results, so E4-F1 is still a real missing bridge. |
| Toolchain truth is grounded | `node --version` -> `v22.22.3`, `npm --version` -> `10.9.8`, `"$HOME/.cargo/bin/cargo" --version` -> `cargo 1.96.0`, and `"$HOME/.cargo/bin/rustc" --version` -> `rustc 1.96.0`; however E4-F1 does not require Node/npm because it must stay in the rules-core lane. |

## Grounded implementation posture
Because the repo now has:

- one merged headless receipt surface
- one merged failure-classifier surface
- one bounded selected-dimension adapter precedent
- stable explanation and diagnostic payloads already emitted from rules-core
- and no existing view-model / snapshot bridge

...the smallest truthful implementation is:

1. add one new `pilot_view_model` module under `src/rules_core/`
2. update `src/rules_core/mod.rs` only to expose that module
3. add one focused test proving the deterministic fixture yields a real snapshot and a mutated blocked fixture yields explicit blocked posture plus diagnostics

Anything broader would be counterfeit UI expansion.

## Expected view-model boundary
The derived handoff should require one bounded contract that preserves, at minimum:

```text
case_id
source_package_id
status
primary_owner
snapshot (present only when the receipt is Computed)
real explanation payloads or stable explanation references
real diagnostics
```

For the accepted deterministic fixture, the snapshot portion must preserve these currently grounded new-system values from the merged receipt:

```text
ability_modifiers.strength = 3
ability_modifiers.dexterity = 2
ability_modifiers.constitution = 2
ability_modifiers.intelligence = 0
ability_modifiers.wisdom = 1
ability_modifiers.charisma = -1
base_attack_bonus = 1
base_saves.fortitude = 2
base_saves.reflex = 0
base_saves.will = 0
combat.baseline_melee_attack_bonus = 5
defense.baseline_armor_class = 17
defense.total_save.fortitude = 4
defense.total_save.reflex = 2
defense.total_save.will = 1
skill.selected_modifier.climb = 5
skill.selected_modifier.intimidate = 3
skill.selected_modifier.swim = 5
```

`case_id` must preserve `pf1-crb-human-fighter-level1`, and `source_package_id` must preserve `pf1.core_rulebook`.

For blocked receipts, the contract must preserve explicit `Blocked` posture, the real primary owner, and real diagnostics. It must not silently convert blocked zero-value placeholders into a success snapshot.

Equivalent type names are acceptable if the emitted surface remains:

- machine-checkable in tests
- derived only from the real headless receipt / classifier lane
- explicit about blocked vs computed posture
- reusable by later GE06-E4-F2 and GE06-E4-F3 work
- non-authoritative about oracle parity or product-visible UI truth

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Prior merged footholds exist | pass | E2-F3, E3-F1, and E3-F2 are verified on `origin/develop`, now at `7bc89e8`. |
| Bounded implementation slice selected | pass | E4-F1 is limited to one rules-core view-model / snapshot contract over the real receipt and failure-owner surfaces. |
| Target repo/workdir exists | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch policy explicit | pass | Reset to current `origin/develop`, then branch `ge06-e4-f1-pilot-view-model-contract`. |
| Allowed write scope explicit | pass | `src/rules_core/mod.rs`, `src/rules_core/pilot_view_model.rs`, and `tests/ge06_pilot_view_model.rs` only. |
| Runtime instruction surface exists | pass | Repo `AGENTS.md` exists and requires strict TDD plus bounded scope. |
| Toolchain grounded | pass | Explicit cargo path works and baseline tests pass on the current repo state. |
| Verification commands known | pass | Exact RED/GREEN/VERIFY commands are named below. |
| Blocked-posture rule explicit | pass | E4-F1 must surface blocked state and diagnostics without inventing a faux success snapshot. |
| Desktop-shell expansion forbidden | pass | Tauri, TypeScript, package manifests, and UI directories remain outside scope. |
| Harness route explicit | pass | E4-F1 now has its own stage-specific execution handoff; the root route surface remains non-authorizing. |

## Authorized write scope for the derived handoff
The derived handoff may authorize writes only to:

```text
src/rules_core/mod.rs
src/rules_core/pilot_view_model.rs
tests/ge06_pilot_view_model.rs
```

It may read but must not modify these grounded surfaces:

```text
src/rules_core/pilot_compute.rs
src/rules_core/pilot_failure.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_pilot_headless_receipt.rs
tests/ge06_failure_classifier.rs
tests/ge06_selected_parity_dimensions.rs
Cargo.toml
Cargo.lock
AGENTS.md
CLAUDE.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-requirements.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-design.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md
programs/codex/doctrine/quality-gate-policy.md
```

If a compile break proves another file is required, stop and report the blocker rather than widening silently.

## Required TDD posture
The coding harness must:

1. create the failing `tests/ge06_pilot_view_model.rs` test first
2. run the specific test and capture RED
3. implement the smallest view-model adapter inside `src/rules_core/pilot_view_model.rs`
4. update `src/rules_core/mod.rs` only to expose the new module
5. run the specific test and capture GREEN
6. re-run the receipt-path proof, failure-classifier proof, and selected-parity proof
7. run full `"$HOME/.cargo/bin/cargo" test --quiet`
8. run a file-granular scope audit

## Explicit non-goals
The derived handoff must not authorize:

- `src-tauri/**`, `apps/**`, `frontend/**`, or any TypeScript / React / Tauri scaffolding
- edits to `src/oracle_validation/**`
- edits to `src/pcgen_import/**`
- edits to `src/rules_core/pilot_compute.rs` or `src/rules_core/pilot_failure.rs`
- parity comparator or pass/fail verdict logic
- export-sheet or summary-generation work
- importer, provenance, or rules-engine rewrites
- `Oracle-checked` or broader compatibility claims
- product-visible UI or shell-polish claims
- Cargo dependency changes or JSON/serde dependency additions
- rewrites of existing receipt/classifier/parity tests

## Claim tier after this slice
If the later E4-F1 handoff succeeds, GE-06 may claim:

```text
headless deterministic pilot route: computed
failure-owner classifier: computed
bounded pilot view-model contract over real receipt outputs: computed
product-visible UI truth / shell implementation / export surface / oracle parity: not yet
```

## Completion rule
This readiness closure is complete when the package truthfully records all of the following:

- E3 upstream evidence remains merged on the current `origin/develop` head
- E4-F1 is now grounded enough for a code-authorizing handoff
- the paired `ge06-e4-f1-execution-handoff-2026-06-22.md` artifact exists and is awaiting Todd launch
- the root `execution-handoff.md` points at the live E4-F1 pair without becoming code authority itself
- any later E4-F1 implementation run must stay inside the rules-core view-model lane, preserve blocked-posture honesty, and refuse desktop-shell expansion
