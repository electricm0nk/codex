---
title: GE07-E5 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E5 — Rules library and source-package pilot views
workflow_route: readiness-closure
readiness: blocked
handoff_created: false
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge07-e3-execution-readiness-closure-2026-06-22.md
  - ./ge07-e3-ui-truth-verification-receipt-2026-06-22.md
  - ./ge07-e2-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
---

# GE07-E5 Execution Readiness Closure

## Verdict
GE07-E5 is not yet grounded enough to mint a code-authorizing rules-library/source-package inspection handoff.

This pass did recover the real pilot-scoped rule identities and the current source-package lineage carriers already present in the live Codex tree. The decisive blockers remain architectural rather than descriptive:
1. `origin/develop` still contains no desktop scaffold under `apps/desktop/` or `src-tauri/`
2. the narrow pilot workspace/view-model bridge is still represented by the awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth
3. the repo still has only raw identity/lineage carriers, not a dedicated UI-consumer inspection projection for bounded rules browsing or source-package detail

Without those footholds, an E5 handoff would silently absorb scaffold creation, GE06-E4-F1 snapshot work, and a new browse/provenance adapter lane into one counterfeit slice.

## Core problem
GE07-E5 is supposed to let the pilot shell answer two product questions honestly:
- which bounded rules/source objects are relevant to the active pilot character path
- where that pilot case came from, including package/source lineage

The live repo can already preserve pieces of that truth in headless/core/oracle carriers, but it cannot yet expose them as one narrow UI-consumer inspection contract or render them inside a real shell subtree. Until those prerequisites exist, E5 cannot be isolated from upstream adapter work or absent-shell scaffolding.

## Selected bounded slice
```text
GE07-E5 — Rules library and source-package pilot views
```

Intended responsibility when it eventually becomes code-ready:
- present bounded pilot rules/source inspection surfaces
- preserve cross-links back into the active character path and current pilot workspace state
- expose source-package lineage from real upstream carriers rather than UI-authored summaries
- refuse frontend-owned rules semantics or provenance invention

What it must not become:
- the first desktop scaffold lane
- the pilot workspace/view-model lane already claimed by GE06-E4-F1
- a broad "build the UI" packet
- an ad hoc provenance adapter that invents source-package truth in the shell

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| Target repo/workdir exists | `/home/ubuntu/workspace/repos/codex` remains the future implementation surface for Codex. |
| Current base truth is grounded | `git rev-parse origin/develop` returned `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`. The checked-out branch remains `ge06-e3-f2-classifier-impl` at `cc45f2c84b0c6bd3b3a7886f9f3068ece8b58e48`, which is residue and not GE-07 execution authority. |
| No shell scaffold exists on `origin/develop` | `git ls-tree -r --name-only origin/develop` returned no `apps/desktop/` or `src-tauri/` entries. |
| No merged pilot workspace/view-model bridge exists on `origin/develop` | `git ls-tree -r --name-only origin/develop` still shows no `src/rules_core/pilot_view_model.rs` or `tests/ge06_pilot_view_model.rs`. |
| Real source-package lineage carriers do exist | `src/rules_core/character_input.rs` requires `source_package_id`; `src/rules_core/pilot_compute.rs` preserves that ID on `PilotHeadlessReceipt`; `src/oracle_validation/selected_parity_dimensions.rs` carries `source_package_id` through each selected dimension; `src/oracle_validation/golden_fixture.rs` defines structured `SourcePackage { system, package, campaign, game_mode }`. |
| Real pilot rule-facing identities do exist | `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` names the bounded pilot race, class level, feats, skills, equipment posture, and selected choices. `tests/ge06_pilot_headless_receipt.rs` proves the live receipt preserves stable explanation ids for surfaced values. |
| Cross-link target back into the active character path is already grounded | `artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md` and `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md` already define the minimum pilot workspace truth burden over real data. |
| No dedicated rules/source inspection projection exists yet | Content scans over `src/**/*.rs` and `tests/**/*.rs` found raw `source_package_id` carriers in `character_input.rs`, `pilot_compute.rs`, and `selected_parity_dimensions.rs`, but no `pilot_view_model` module and no bounded rules-library/source-package inspection contract. |
| Shell-side documentary duties already exist | `artifacts/ui-information-architecture-requirements.md`, `artifacts/component-surface-inventory.md`, and `artifacts/ui-command-boundary-requirements.md` already define the GE07-E5 surfaces and prohibitions. |
| The narrow workspace bridge is already claimed elsewhere | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and `ge06-e4-f1-execution-handoff-2026-06-22.md` already define the next honest UI-consumer snapshot lane, but that lane is still `awaiting-todd-launch`. |

## Grounded pilot inspection truth recovered this pass
The live repo already preserves enough evidence to define what GE07-E5 must eventually expose.

### 1. Source-package lineage carriers
The current pilot lineage is not imaginary; it is already carried in multiple bounded surfaces:
- deterministic character input fixture: `source_package_id = pf1.core_rulebook`
- headless receipt: preserves `source_package_id` unchanged on `PilotHeadlessReceipt`
- selected parity dimensions: each emitted dimension preserves the same `source_package_id`
- golden oracle fixture: expands lineage into `source_system = pathfinder-1e`, `source_package = core_rulebook`, `source_campaign = Core Rulebook`, `source_game_mode = Pathfinder_RPG`

This means a future source-package view must be a consumer of structured upstream lineage, not a UI-authored prose summary.

### 2. Pilot-scoped rule identities already present
The bounded pilot identities already recoverable from the deterministic fixture are:
- race: `race:human`
- class level: `class:fighter:1`
- feats: `feat:power_attack`, `feat:dodge`, `feat:weapon_focus`
- skills: `skill:climb:1`, `skill:intimidate:1`, `skill:swim:1`
- equipment posture: `item:chain_shirt`, `item:longsword`, `item:shield:absent`, `power_attack:selected_inactive`
- selected choices: level-1 feat, human bonus feat, fighter bonus feat with longsword specialization, and human Strength bonus

The receipt tests also prove stable explanation ids already exist for surfaced values, including `ability_modifier.strength`, `class_chassis.base_attack_bonus`, `combat.baseline_melee_attack_bonus`, `defense.baseline_armor_class`, `defense.total_save.fortitude`, and `skill.selected_modifier.climb`.

This is enough to ground the inspection burden. It is not yet a UI-consumer browse model.

### 3. Cross-link obligation back into the active character path
GE07-E5 is not allowed to become a detached content browser. The active character path is already grounded by GE07-E3 as:
- case identity `pf1-crb-human-fighter-level1`
- source package `pf1.core_rulebook`
- current selections, grouped values, explanation hooks, and computed-vs-blocked route framing

Any future GE07-E5 slice must link inspected rule/package items back to that live pilot context instead of presenting a free-floating library explorer.

## Exact write-scope posture
### What can be named honestly now
The only implementation-relevant path classes that can be named without invention are:

```text
apps/desktop/package.json
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/src/main.rs
src/rules_core/mod.rs
src/rules_core/pilot_view_model.rs
tests/ge06_pilot_view_model.rs
src/rules_core/character_input.rs
src/rules_core/pilot_compute.rs
src/oracle_validation/selected_parity_dimensions.rs
src/oracle_validation/golden_fixture.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt
tests/ge06_pilot_headless_receipt.rs
tests/ge06_selected_parity_dimensions.rs
```

Interpretation:
- the first six paths are the already-grounded GE07-E1 scaffold candidates
- the next three paths are the already-claimed GE06-E4-F1 pilot workspace bridge candidate
- the remaining files are read-only evidence carriers for current pilot rule identity and source-package lineage

### What cannot be named honestly yet
An exact GE07-E5-only writable file list cannot yet be named truthfully.

Why not:
- the desktop subtree still does not exist on `origin/develop`
- the workspace/view-model bridge E5 should cross-link through is still represented by the unlaunched GE06-E4-F1 handoff rather than merged repo truth
- the live tree still lacks any dedicated bounded rules-library/source-package inspection projection
- inventing concrete E5-only writable files now would smuggle scaffold work, GE06-E4-F1 work, or a new raw-carrier-to-view-model adapter into one false packet

## Gate table
| Gate | Status | Resolution |
|---|---|---|
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` is explicit. |
| Shell documentary duties grounded | pass | GE-07 artifact set already defines IA, command-boundary, and component-surface obligations. |
| Pilot rule identities grounded | pass | deterministic fixture names the exact pilot-scoped race, class, feat, skill, equipment, and choice IDs. |
| Source-package lineage carriers grounded | pass | live rules-core/oracle surfaces preserve package identity and expanded golden-fixture lineage fields. |
| Active-character cross-link target grounded | pass | GE07-E3 already defines the current pilot workspace truth burden over real data. |
| Executed shell scaffold exists on repo base | fail | `origin/develop` still has no `apps/desktop/` or `src-tauri/` entries. |
| Merged pilot workspace/view-model bridge exists on repo base | fail | `pilot_view_model.rs` and `tests/ge06_pilot_view_model.rs` are absent on `origin/develop`; the bridge is still an awaiting-Todd-launch handoff. |
| Dedicated UI-consumer rules/source inspection projection exists | fail | the live tree preserves raw carriers only; there is no bounded browse/provenance inspection contract yet. |
| Exact GE07-E5-only writable file list is grounded | fail | any list would currently absorb scaffold work, GE06-E4-F1 work, or speculative projection design. |
| Runnable RED/GREEN command set for a truthful E5 coding lane exists | fail | until scaffold, workspace bridge, and inspection projection truths exist, there is no honest E5-specific verification contract to authorize. |
| Code-authorizing handoff justified | fail | prerequisites missing; `handoff_created: false`. |

## Branch and dependency posture
If GE-07 later resumes toward execution, the base-reset rule remains:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
```

But no GE07-E5 branch should be created yet, because the rules/source inspection lane still depends on three truths not present on that base:
- a real desktop scaffold subtree
- a merged pilot workspace/view-model bridge that E5 can cross-link through honestly
- a decision about whether the inspection projection is a narrow extension of that bridge or a separate bounded read-model lane over the existing rules/provenance carriers

## Shortest honest next move
1. Todd launches GE06-E4-F1 or otherwise lands an equivalent merged pilot workspace/view-model bridge on `origin/develop`.
2. If the shell subtree is still absent after that, derive a separate GE07-E1 execution-readiness closure and bounded handoff for scaffold creation only.
3. Once those two footholds are real, decide whether GE07-E5 first needs a narrow rules/provenance projection lane before shell presentation, or whether the required contract is now explicit enough to keep E5 bounded inside the desktop layer.
4. Only then rerun GE07-E5 readiness against the live tree and mint a stage-specific handoff limited to pilot inspection surfaces with exact writable files and verification commands.

## Explicit non-goals for this pass
This closure does not authorize:
- `apps/desktop/**` creation by implication
- Tauri, React, TypeScript, or package-manager implementation work
- a duplicate GE06-E4-F1 workspace/view-model lane
- UI-owned rules, provenance, or explanation logic
- a broad "build the UI" packet
- product-visible breadth claims beyond the bounded pilot rules/source inspection burden

## Completion rule
This readiness closure is complete because it does all of the honest work GE07-E5 can support today:
- grounds the live pilot rule identities and source-package lineage carriers already present in the repo
- ties those inspection duties back into the already-grounded active character path from GE07-E3
- records why no exact E5-only write scope or verification contract exists yet
- refuses to mint counterfeit code authority while scaffold truth, the GE06-E4-F1 bridge, and any dedicated inspection projection remain unresolved
