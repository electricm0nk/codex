---
title: GE07-E3 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E3 — Pilot character workspace shell
workflow_route: readiness-closure
readiness: blocked
handoff_created: false
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge07-e3-ui-truth-verification-receipt-2026-06-22.md
  - ./ge07-e2-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
---

# GE07-E3 Execution Readiness Closure

## Verdict
GE07-E3 is not yet grounded enough to mint a code-authorizing pilot-workspace-shell handoff.

This pass did recover the real minimum workspace state and one UI-truth verification receipt over live pilot data. The decisive blockers remain architectural rather than descriptive:
1. `origin/develop` still contains no desktop scaffold under `apps/desktop/` or `src-tauri/`
2. the narrow UI-consumer view-model bridge is still represented by the awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth

Without those footholds, an E3 handoff would silently absorb scaffold creation, rules-core adapter work, and workspace presentation into one counterfeit slice.

## Core problem
The pilot workspace shell is supposed to present the real character path through value groups, current selections, and route framing over real domain outputs. That means the shell needs two things before its own coding lane can be honest:

1. an actual shell subtree to render inside
2. a merged, UI-consumable snapshot contract derived from the real GE-06 receipt path

This pass proves what the shell must show. It does not prove that the shell lane can be isolated yet.

## Selected bounded slice
```text
GE07-E3 — Pilot character workspace shell
```

Intended responsibility when it eventually becomes code-ready:
- present the active pilot case with real identity, current selections, grouped values, and route framing
- preserve computed-versus-blocked honesty over the real GE-06 receipt lane
- surface diagnostics and explanation hooks without turning the workspace into a second rules engine

What it must not become:
- the first desktop scaffold lane
- the rules-core view-model invention lane already claimed by GE06-E4-F1
- a broad "build the UI" handoff

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| Target repo/workdir exists | `/home/ubuntu/workspace/repos/codex` remains the future implementation surface for Codex. |
| Current base truth is grounded | `git rev-parse origin/develop` remains `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`. The checked-out branch remains `ge06-e3-f2-classifier-impl` at `cc45f2c84b0c6bd3b3a7886f9f3068ece8b58e48`, and `git merge-base --is-ancestor HEAD origin/develop` succeeded, proving the live probe ran against code already contained by the current base lineage. |
| No shell scaffold exists on `origin/develop` | `git ls-tree -r --name-only origin/develop | grep -E '^(apps/desktop/|src-tauri/)'` returned no matches. |
| No merged pilot workspace bridge exists on `origin/develop` | `git ls-tree -r --name-only origin/develop | grep -E '^(src/rules_core/pilot_view_model.rs|tests/ge06_pilot_view_model.rs)'` returned no matches. |
| Real pilot receipt path is live | `"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet` passed in the live repo. |
| Real pilot workspace evidence was extracted | The temporary workspace probe at `/home/ubuntu/.hermes/kanban/boards/codex/workspaces/t_41c6b298/pilot_probe` loaded the deterministic pilot fixture, built the real GE-06 headless receipt, and emitted both computed and blocked route examples. The results are captured in `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md`. |
| Upstream snapshot bridge is already claimed elsewhere | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and `ge06-e4-f1-execution-handoff-2026-06-22.md` already define the next honest rules-core view-model lane, but that lane is still `awaiting-todd-launch`. |
| Shell-side documentary boundary exists | `artifacts/ui-information-architecture-requirements.md`, `artifacts/component-surface-inventory.md`, `artifacts/pilot-ux-flow-requirements.md`, and `artifacts/ui-command-boundary-requirements.md` already define the shell's duties and prohibitions. |

## Minimum pilot workspace state recovered this pass
The minimum honest pilot workspace is now explicit. It must preserve all of the following over real domain outputs:

### 1. Route frame and status truth
- `case_id = pf1-crb-human-fighter-level1`
- `source_package_id = pf1.core_rulebook`
- route posture from the real receipt: `Computed` or `Blocked`
- primary owner from the existing classifier lane: `OracleGap` on the supported computed path, `EngineFlaw` on the blocked test path
- diagnostics visibility that does not disappear when the path is blocked

### 2. Current selections summary
- race: `race:human`
- class level: `class:fighter:1`
- ability choices: Strength 16, Dexterity 14, Constitution 14, Intelligence 10, Wisdom 12, Charisma 8
- feats: `feat:power_attack`, `feat:dodge`, `feat:weapon_focus`
- skill ranks: `skill:climb:1`, `skill:intimidate:1`, `skill:swim:1`
- equipment posture: Chain Shirt active/worn, Longsword active/primary, Shield absent, Power Attack selected but inactive for baseline outputs
- explicit slot choices including the human ability bonus to Strength and Weapon Focus (Longsword)

### 3. Value groups for the computed pilot path
- ability modifiers: STR 3, DEX 2, CON 2, INT 0, WIS 1, CHA -1
- base chassis: BAB 1; base saves Fort 2 / Ref 0 / Will 0
- combat: baseline melee attack bonus 5
- defense: baseline armor class 17; total saves Fort 4 / Ref 2 / Will 1
- selected skills: Climb 5 / Intimidate 3 / Swim 5

### 4. Explanation/detail hooks
The computed path already exposes stable explanation ids that the workspace may reveal without inventing local semantics, including:
- `ability_modifier.strength`
- `class_chassis.base_attack_bonus`
- `combat.baseline_melee_attack_bonus`
- `defense.baseline_armor_class`
- `defense.total_save.fortitude`
- `skill.selected_modifier.climb`

### 5. Blocked-path truth rule
The blocked route example proves the workspace must not zero-fill its way into faux success. When the path is blocked, the workspace must preserve:
- `status = Blocked`
- `primary_owner = EngineFlaw`
- the fact that downstream chassis/combat/defense/selected-skill groups collapse to placeholder zero values rather than a successful computed claim
- the real claim-blocking diagnostics
- absence of a success snapshot claim

## Exact write-scope posture
### What can be named honestly now
The pilot workspace already has a truthful minimum state and a truthful proof receipt, but the only implementation-relevant path classes that can be named at all right now are the GE07-E1 scaffold candidates plus the live rules-core receipt surfaces:

```text
apps/desktop/package.json
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/tauri.conf.json
apps/desktop/src-tauri/src/main.rs
src/rules_core/pilot_compute.rs
src/rules_core/pilot_failure.rs
```

### What cannot be named honestly yet
An exact GE07-E3-only writable file list cannot yet be named truthfully.

Why not:
- the desktop subtree still does not exist on `origin/develop`
- the view-model bridge the workspace should consume is still represented by the unlaunched GE06-E4-F1 handoff rather than merged repo truth
- inventing specific shell files beyond the absent scaffold would either smuggle E1 scaffold work into E3 or speculate beyond the still-unimplemented read-model lane

## Gate table
| Gate | Status | Resolution |
|---|---|---|
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` is explicit. |
| Shell documentary duties grounded | pass | GE-07 artifact set already defines IA, route, and command-boundary obligations. |
| Real pilot data available | pass | live receipt test passes and the workspace probe produced computed and blocked route examples from the deterministic fixture. |
| Minimum pilot workspace state defined | pass | this closure and the paired verification receipt now name the exact selections, grouped values, explanation hooks, and blocked-posture rule. |
| UI-truth receipt exists | pass | `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md` now records the live proof burden. |
| Executed shell scaffold exists on repo base | fail | `origin/develop` still has no `apps/desktop/` or `src-tauri/` entries. |
| Merged view-model bridge exists on repo base | fail | `pilot_view_model.rs` and `tests/ge06_pilot_view_model.rs` are absent on `origin/develop`; the bridge is still an awaiting-Todd-launch handoff. |
| Exact GE07-E3-only writable file list is grounded | fail | any list would currently absorb scaffold work or speculate ahead of E4-F1. |
| Runnable RED/GREEN command set for a truthful E3 coding lane exists | fail | until the scaffold and snapshot bridge are real, there is no honest workspace-shell-specific verification contract to authorize. |
| Code-authorizing handoff justified | fail | prerequisites missing; `handoff_created: false`. |

## Branch and dependency posture
If GE-07 later resumes toward execution, the base-reset rule remains:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
```

But no GE07-E3 branch should be created yet, because the workspace shell still depends on two truths not present on that base:
- a real desktop scaffold subtree
- a merged rules-core view-model bridge that can be consumed without inventing UI semantics locally

## Shortest honest next move
1. Todd launches GE06-E4-F1 or otherwise lands an equivalent merged pilot view-model bridge on `origin/develop`.
2. If the shell subtree is still absent after that, derive a separate GE07-E1 execution-readiness closure and bounded handoff for scaffold creation only.
3. Once both footholds are real, rerun GE07-E3 readiness against the live tree.
4. Only then mint a stage-specific GE07-E3 handoff that is limited to the workspace shell and verified against the real pilot snapshot contract rather than documentary aspiration.

## Explicit non-goals for this pass
This closure does not authorize:
- `apps/desktop/**` creation by implication
- Tauri, React, TypeScript, or package-manager implementation work
- a duplicate rules-core snapshot/view-model lane alongside GE06-E4-F1
- UI-owned rules, explanation, or diagnostics logic
- a broad "build the UI" packet
- product-visible parity or launch claims

## Completion rule
This readiness closure is complete because it does all of the honest work GE07-E3 can support today:
- recovers the real pilot-workspace minimum state over real domain outputs
- produces a paired UI-truth verification receipt using live pilot data
- records why no exact E3-only write scope exists yet
- refuses to mint counterfeit code authority while the scaffold and upstream GE06-E4-F1 bridge remain unresolved
