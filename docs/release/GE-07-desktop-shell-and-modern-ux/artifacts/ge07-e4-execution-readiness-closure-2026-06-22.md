---
title: GE07-E4 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E4 — Explanation and diagnostics surfaces
workflow_route: readiness-closure
readiness: blocked
handoff_created: false
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md
  - ./ge07-e3-execution-readiness-closure-2026-06-22.md
  - ./ge07-e3-ui-truth-verification-receipt-2026-06-22.md
  - ./ge07-e5-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
---

# GE07-E4 Execution Readiness Closure

## Verdict
GE07-E4 is not yet grounded enough to mint a code-authorizing explanation/diagnostics handoff.

This pass did recover live explanation payload detail and three concrete structured diagnostic families already present in the Codex tree:
1. rules-core blocked-route diagnostics from the GE-06 headless receipt lane
2. character-input validation diagnostics from the GE-04/GE-06 loader
3. raw PCC importer diagnostics from the GE-03 parser lane

The decisive blockers remain architectural and contractual rather than descriptive:
1. `origin/develop` still contains no desktop scaffold under `apps/desktop/` or `src-tauri/`
2. the narrow UI-consumer bridge is still represented by the awaiting-Todd-launch GE06-E4-F1 handoff rather than merged repo truth
3. the live repo still has no bounded view-model or transport layer that projects explanations, blocked-route diagnostics, validation problems, and importer warnings into one UI-consumer surface
4. the live rules-core still has no grounded invalid-choice/prerequisite-reason payload; only chosen selections are represented today

Without those footholds, an E4 handoff would silently absorb scaffold creation, GE06-E4-F1 bridge work, diagnostic aggregation, and a not-yet-grounded invalid-choice lane into one counterfeit slice.

## Core problem
GE07-E4 is supposed to let the shell answer four product questions honestly:
- why is this value what it is?
- why is this route blocked or warning-bearing?
- what validation problem exists before or during compute?
- what importer warning or unsupported semantic still matters?

The live repo can already prove pieces of that truth. It cannot yet isolate one narrow shell slice that consumes those truths without also inventing the absent consumer bridge and the still-unimplemented invalid-choice reason lane.

## Selected bounded slice
```text
GE07-E4 — Explanation and diagnostics surfaces
```

Intended responsibility when it eventually becomes code-ready:
- render derived-value explanations from upstream payloads/details
- preserve blocked-route diagnostics, validation problems, and importer warnings without softening them into product gloss
- expose the real primary failure owner and route posture when the pilot path is blocked
- keep explanation/diagnostic detail cross-linked back to the active value, route, or source line

What it must not become:
- the first desktop scaffold lane
- the GE06-E4-F1 view-model bridge lane already claimed elsewhere
- a new prerequisite/choice engine hidden inside a UI packet
- a broad "build the UI" packet

## Required source evidence recovered
| Gate | Evidence |
|---|---|
| Target repo/workdir exists | `/home/ubuntu/workspace/repos/codex` remains the future implementation surface for Codex. |
| Current base truth is grounded | `git rev-parse --short origin/develop` returned `7bc89e8`; the checked-out branch remains `ge06-e3-f2-classifier-impl` at `cc45f2c`; `git merge-base --is-ancestor HEAD origin/develop` succeeded. |
| No shell scaffold exists on `origin/develop` | `git ls-tree -r --name-only origin/develop | grep -E '^(apps/desktop/|src-tauri/)'` returned no matches. |
| No merged consumer bridge exists on `origin/develop` | `git ls-tree -r --name-only origin/develop | grep -E '^(src/rules_core/pilot_view_model.rs|tests/ge06_pilot_view_model.rs)'` returned no matches. |
| Live explanation payloads are grounded | `cargo test --test ge06_pilot_headless_receipt --quiet` and `cargo test --test ge06_pilot_combat_baseline --quiet` passed; the temporary probe at `/home/ubuntu/.hermes/kanban/boards/codex/workspaces/t_71b6d8d0/explanation_probe` printed the real explanation details for Strength modifier, baseline melee attack bonus, baseline armor class, total Fortitude save, and selected Climb modifier. |
| Live blocked-route diagnostics are grounded | The same probe emitted `Blocked` receipts with `primary_owner=EngineFlaw` plus real `class_chassis.unsupported`, `combat.baseline_unsupported`, `defense.total_save.unsupported`, and `skill.selected_modifier.unsupported` diagnostics from the merged GE-06 headless receipt lane. |
| Live validation-problem payloads are grounded | `cargo test --test ge06_pilot_input_contract --quiet` and `cargo test --test character_input_record --quiet` passed; the probe also emitted a real `CharacterInputDiagnostic` for an unsupported equipment active-state token with class `InvalidCharacterInput`, severity `Error`, and `subject_ref=equipment_selections`. |
| Live raw importer diagnostics are grounded | `cargo test --test pcc_entry_parse --quiet` passed; the probe emitted a real `PccDiagnostic` with kind `MalformedInclude`, line number, raw line, and message for a malformed `PCC:` directive. |
| No grounded invalid-choice reason payload exists yet | `search_files` over `src/rules_core/**/*.rs` for `unavailable choice|choice availability|prerequisite evaluation|prerequisite|invalid choice` returned only the comment in `pilot_compute.rs` that feat prerequisites are still out of scope; no implementation of invalid-choice reasoning surfaced. |
| No bounded explanation/diagnostic projection exists yet | `search_files` over `src/**/*.rs` for `pilot_view_model|view_model` returned no results. |
| Shell-side documentary duties already exist | `artifacts/ui-command-boundary-requirements.md`, `artifacts/ui-information-architecture-requirements.md`, `artifacts/component-surface-inventory.md`, and `artifacts/pilot-ux-flow-requirements.md` already define the GE07-E4 surfaces and prohibitions. |
| Narrow consumer bridge is already claimed elsewhere | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and `ge06-e4-f1-execution-handoff-2026-06-22.md` already define the next honest view-model lane, but that lane is still `awaiting-todd-launch`. |

## Grounded explanation and diagnostics truth recovered this pass
The live repo already preserves enough evidence to define what GE07-E4 must eventually expose.

### 1. Derived-value explanation payloads already exist
The computed deterministic pilot receipt currently preserves machine-checkable explanation ids plus human-auditable detail strings, including:
- `ability_modifier.strength` — `strength ability modifier from chosen score 16: floor(16 / 2) - 5 = 3`
- `combat.baseline_melee_attack_bonus` — cites Fighter BAB, Strength, Weapon Focus (Longsword), and the inactive Power Attack posture
- `defense.baseline_armor_class` — cites base 10, Chain Shirt, Dexterity within `MAXDEX:4`, Dodge, and the absent shield posture
- `defense.total_save.fortitude` — cites Fighter base Fortitude save plus Constitution modifier
- `skill.selected_modifier.climb` — cites rank, Strength modifier, class-skill bonus, and Chain Shirt armor-check penalty

The computed receipt probe observed `status=Computed`, `primary_owner=OracleGap`, `explanation_count=18`, and `claim_blocking_diagnostics=0`.

### 2. Blocked-route diagnostics remain visible and selectively withhold false explanations
The live blocked-path probes prove the shell must not convert failure into a clean-looking surface.

Observed facts:
- equipping the shield mutated the route to `status=Blocked` with `primary_owner=EngineFlaw`
- the blocked shield posture preserved a real `combat.baseline_unsupported` claim-blocking diagnostic
- the blocked shield posture withheld `combat.baseline_melee_attack_bonus` and `defense.baseline_armor_class` explanations while still retaining unrelated upstream explanation truth such as `ability_modifier.strength`
- mutating the chassis from Fighter to Rogue preserved multiple real claim-blocking diagnostics instead of a faux success snapshot

That means visibility is not only about showing warnings. It is also about refusing to show explanations for values the engine no longer truthfully computed.

### 3. Validation problems already have a structured payload shape
Before compute, the character-input loader already surfaces structured claim-blocking validation diagnostics.

Observed shape from the live probe:
- class: `InvalidCharacterInput`
- severity: `Error`
- claim_blocking: `true`
- subject reference: `equipment_selections`
- message: `invalid character input equipment selection 'item:longsword:equiped_primary_active' has an unsupported state`

That is a real UI-consumer obligation: the shell must not flatten these into generic "something went wrong" prose.

### 4. Raw importer diagnostics already have a structured payload shape
The GE-03 PCC parser already emits structured diagnostics rather than dropping malformed input silently.

Observed shape from the live probe:
- diagnostic kind: `MalformedInclude`
- source line: `3`
- raw line: `PCC:`
- message: `PCC include directive has no target`

This proves importer warnings can be structured. It does not yet prove that the pilot shell can consume them through one merged projection.

### 5. Invalid-choice reason payloads are still not grounded
The live source tree currently represents chosen selections and deterministic computed values, but not a distinct invalid-choice/prerequisite-reason payload.

That means GE07-E4 cannot honestly claim full acceptance yet. A future E4 handoff must either:
- consume a new grounded upstream invalid-choice reason lane, or
- explicitly restate the slice so it excludes that duty instead of silently dropping it

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
src/rules_core/pilot_failure.rs
src/pcgen_import/pcc.rs
tests/ge06_pilot_headless_receipt.rs
tests/ge06_failure_classifier.rs
tests/ge06_pilot_combat_baseline.rs
tests/ge06_pilot_input_contract.rs
tests/character_input_record.rs
tests/pcc_entry_parse.rs
```

Interpretation:
- the first six paths are the already-grounded GE07-E1 scaffold candidates
- the next three paths are the already-claimed GE06-E4-F1 consumer-bridge candidates
- the remaining files are read-only evidence carriers for current explanation and diagnostics truth

### What cannot be named honestly yet
An exact GE07-E4-only writable file list cannot yet be named truthfully.

Why not:
- the desktop subtree still does not exist on `origin/develop`
- the consumer bridge E4 should consume is still represented by the unlaunched GE06-E4-F1 handoff rather than merged repo truth
- the repo has no bounded explanation/diagnostic projection contract yet
- invalid-choice reason payloads are not implemented in the live rules-core
- inventing concrete E4-only writable files now would smuggle scaffold work, GE06-E4-F1 work, an aggregation/projection lane, or a new prerequisite/choice lane into one false packet

## Gate table
| Gate | Status | Resolution |
|---|---|---|
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex` is explicit. |
| Shell documentary duties grounded | pass | GE-07 artifact set already defines the explanation/diagnostics surfaces and prohibitions. |
| Derived-value explanation payloads grounded | pass | live tests plus the probe recover real explanation ids and detail strings from the deterministic receipt path. |
| Blocked-route diagnostics grounded | pass | live tests plus the probe recover real blocked-route diagnostics and prove explanation withholding on unsupported values. |
| Validation-problem payloads grounded | pass | live tests plus the probe recover structured `InvalidCharacterInput` diagnostics. |
| Raw importer diagnostics grounded | pass | live tests plus the probe recover structured `PccDiagnostic` records for malformed include lines. |
| Executed shell scaffold exists on repo base | fail | `origin/develop` still has no `apps/desktop/` or `src-tauri/` entries. |
| Merged consumer bridge exists on repo base | fail | `pilot_view_model.rs` and `tests/ge06_pilot_view_model.rs` are absent on `origin/develop`; the bridge is still an awaiting-Todd-launch handoff. |
| Bounded explanation/diagnostic projection exists | fail | no `view_model` or equivalent projection surfaced in the live tree. |
| Invalid-choice/prerequisite reason payload exists | fail | the live rules-core search returned no such implementation. |
| Exact GE07-E4-only writable file list is grounded | fail | any list would currently absorb scaffold work, GE06-E4-F1 work, or speculative payload-aggregation design. |
| Runnable RED/GREEN command set for a truthful E4 coding lane exists | fail | until scaffold, consumer bridge, projection contract, and invalid-choice posture are real, there is no honest E4-specific verification contract to authorize. |
| Code-authorizing handoff justified | fail | prerequisites missing; `handoff_created: false`. |

## Branch and dependency posture
If GE-07 later resumes toward execution, the base-reset rule remains:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
```

But no GE07-E4 branch should be created yet, because the explanation/diagnostics lane still depends on four truths not present on that base:
- a real desktop scaffold subtree
- a merged consumer bridge that the shell can read without inventing local semantics
- a bounded projection for explanation + diagnostics payload families
- a decision about whether invalid-choice reasons are already grounded upstream or require a new dedicated rules-core slice

## Shortest honest next move
1. Todd launches GE06-E4-F1 or otherwise lands an equivalent merged pilot consumer bridge on `origin/develop`.
2. If the shell subtree is still absent after that, derive a separate GE07-E1 execution-readiness closure and bounded handoff for scaffold creation only.
3. Decide whether invalid-choice/prerequisite reasons require a new upstream GE-04 rules-core slice or are intentionally deferred from the first E4 coding lane; do not leave this implicit.
4. Once those truths are real, define whether GE07-E4's first coding lane is only a shell-side projection/rendering slice or whether an intermediate adapter/projection lane must exist first.
5. Only then rerun GE07-E4 readiness against the live tree and mint a stage-specific handoff limited to explanation/diagnostics surfaces with exact writable files and verification commands.

## Explicit non-goals for this pass
This closure does not authorize:
- `apps/desktop/**` creation by implication
- Tauri, React, TypeScript, or package-manager implementation work
- a duplicate GE06-E4-F1 consumer bridge lane
- a new prerequisite/choice engine hidden inside shell work
- UI-owned explanation math or warning reinterpretation
- a broad "build the UI" packet
- product-visible truth or parity claims beyond the already-grounded headless evidence

## Completion rule
This readiness closure is complete because it does all of the honest work GE07-E4 can support today:
- grounds the live explanation payload details already present in the deterministic receipt lane
- grounds three structured diagnostic families already present in the live tree
- records the current absence of any merged consumer bridge, bounded projection contract, or invalid-choice reason payload
- refuses to mint counterfeit code authority while scaffold truth, GE06-E4-F1 bridge truth, and the missing invalid-choice lane remain unresolved
