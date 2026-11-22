---
title: GE06-E4-F1 Execution Handoff — Pilot View-Model Contract From Real Outputs
handoff_id: HANDOFF-CODEX-GE-06-E4-F1-CODING-2026-06-22
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: awaiting-todd-launch
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
selected_slice: GE06-E4-F1 — Pilot view-model contract from real outputs
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-22
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 7bc89e8
  recommended_branch: ge06-e4-f1-pilot-view-model-contract
  pr_target: develop
allowed_write_scope:
  - src/rules_core/mod.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_view_model.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src-tauri/**
  - apps/**
  - frontend/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/pilot_compute.rs
  - src/rules_core/pilot_failure.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/ge06_failure_classifier.rs
  - tests/ge06_selected_parity_dimensions.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E4-F1 Execution Handoff — Pilot View-Model Contract From Real Outputs

## Status
This is the live stage-specific code-authorizing brief for GE06-E4-F1.

It carries `code_authority: true` for GE06-E4-F1 only and is currently `awaiting-todd-launch`.

## Run in
Claude Code or an equivalent frontier coding harness.

## Core problem
Codex now has a real deterministic headless receipt path, a selected parity-dimension carrier, and a primary failure-owner classifier, but it still lacks the next narrow bridge that lets a future UI consume real pilot state or explicit blocker truth without hardcoded character data.

The truthful next slice is not Tauri scaffolding and not a product-visible shell. It is one bounded rules-core view-model contract that projects the merged receipt into a UI-consumable snapshot when computed, and into an explicit blocked posture with real diagnostics when blocked.

## Objective
Create the smallest rules-core view-model contract that projects the real GE-06 pilot receipt into a machine-checkable UI-consumer boundary.

The contract must preserve, at minimum:

1. pilot identity from the real receipt
2. real computed snapshot values when the receipt is `Computed`
3. explicit blocked posture plus real diagnostics when the receipt is `Blocked`
4. the primary failure owner from the existing classifier lane
5. real explanation payloads or stable explanation references for surfaced values

This handoff does not authorize any desktop shell, frontend, or export implementation.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f1-decision-rack-2026-06-22.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-launch-posture-2026-06-22.md`
7. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
8. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md`
9. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-requirements.md`
10. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-design.md`
11. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md`
12. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md`
13. `programs/codex/doctrine/quality-gate-policy.md`
14. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
15. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_failure.rs`
16. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/selected_parity_dimensions.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
18. `/home/ubuntu/workspace/repos/codex/tests/ge06_failure_classifier.rs`
19. `/home/ubuntu/workspace/repos/codex/tests/ge06_selected_parity_dimensions.rs`

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e4-f1-pilot-view-model-contract
```

Expected base at handoff creation:

```text
origin/develop = 7bc89e8
```

If `origin/develop` has advanced, use the current fetched `origin/develop` and record the actual base SHA in the final report.

## Baseline repo posture
Observed during documentary promotion:

```text
HEAD -> ge06-e3-f2-classifier-impl
origin/develop -> 7bc89e8
```

Rules:

- do not branch from the stale checked-out topic branch
- reset to current `origin/develop` first
- treat the current branch header as residue from the already-merged E3 work, not as your implementation base

## Toolchain posture
Observed in the current Hermes shell:

```text
node v22.22.3
npm 10.9.8
cargo 1.96.0
rustc 1.96.0
```

Even though Node/npm are available, E4-F1 must stay in the Rust rules-core lane. Do not widen into frontend or Tauri work.

## Merge authority boundary
This handoff does not authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the launch/review/merge decisions.

## Allowed write scope
You may write only:

```text
src/rules_core/mod.rs
src/rules_core/pilot_view_model.rs
tests/ge06_pilot_view_model.rs
```

If you need any other file, stop and report the blocker.

Do not modify `pilot_compute.rs`, `pilot_failure.rs`, any oracle-validation module, any frontend/Tauri path, dependency manifests, or repo-root instruction files.

## Required implementation behavior
Keep the implementation inside the rules-core read-model lane.

Preferred outcome:

- add `src/rules_core/pilot_view_model.rs`
- update `src/rules_core/mod.rs` only to expose the new module
- consume `PilotHeadlessReceipt` and the existing failure-classifier lane as read-only input
- emit one bounded pilot snapshot when the receipt is `Computed`
- emit explicit blocked posture plus real diagnostics when the receipt is `Blocked`
- preserve real explanation payloads or stable explanation references for the surfaced values
- add no Tauri shell, no TypeScript UI, no JSON serialization surface, and no export logic

The adapter may choose its own exact type names, but the emitted surface must stay machine-checkable and reusable.

## Mandatory contract content
The emitted contract must preserve these fields or exact equivalents:

```text
case_id
source_package_id
status
primary_owner
snapshot (optional/present only when computed)
explanations or stable explanation references
diagnostics
```

For the accepted deterministic fixture, the computed snapshot must preserve these real values from the merged receipt:

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

The contract must preserve `case_id = pf1-crb-human-fighter-level1` and `source_package_id = pf1.core_rulebook`.

For a blocked fixture, the contract must:

- keep `status = Blocked`
- preserve the real primary owner from the classifier lane
- preserve the real diagnostics from the receipt
- refuse to surface a faux success snapshot with zero-filled values

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_pilot_view_model.rs` first.
2. Load the accepted deterministic fixture and build the merged headless receipt through the existing rules-core path.
3. Assert the future view-model contract preserves:
   - real pilot identity
   - the deterministic computed snapshot values listed above
   - the required primary owner vocabulary through the classifier lane
   - real explanation payloads or stable explanation references
4. Mutate the fixture to the existing blocked-path variant and assert the future contract preserves blocked posture and real diagnostics without a faux success snapshot.
5. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_view_model --quiet
```

6. Confirm it fails for the expected reason: the view-model surface does not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- add the new adapter module
- expose it from `src/rules_core/mod.rs`
- do not modify receipt-generation or failure-classifier logic
- do not introduce shell/frontend/export behavior

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_view_model --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet
"$HOME/.cargo/bin/cargo" test --quiet
```

All must pass.

### SCOPE AUDIT
Run:

```bash
git diff --name-only
git ls-files --others --exclude-standard
```

Confirm every new or modified file is inside the allowed write scope.

## Acceptance criteria
The handoff is complete only if all are true:

- [ ] A new view-model proof test exists at `tests/ge06_pilot_view_model.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_view_model.rs` is the only new production module.
- [ ] `src/rules_core/mod.rs` changes only to expose that module.
- [ ] The adapter consumes the merged receipt / failure-classifier lane as read-only input.
- [ ] The computed path preserves the deterministic snapshot values listed above.
- [ ] `case_id` and `source_package_id` are preserved from the real receipt.
- [ ] The blocked path preserves explicit blocked posture and real diagnostics.
- [ ] The blocked path does not emit a faux success snapshot.
- [ ] The contract preserves explanation payloads or stable explanation references for surfaced values.
- [ ] No shell/frontend/export/parity behavior is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- any Tauri, React, TypeScript, or desktop-shell scaffolding
- edits to `src/oracle_validation/**`
- edits to `src/pcgen_import/**`
- edits to `src/rules_core/pilot_compute.rs`
- edits to `src/rules_core/pilot_failure.rs`
- parity comparator or pass/fail verdict logic
- exportable summary or sheet generation
- JSON/serde transport or API-server work
- product-visible UI claims
- Cargo dependency additions
- rewrites of existing receipt/classifier/parity tests

## Final report required from coding harness
Return a concise factual report with:

- branch name
- base SHA actually used
- files changed
- tests added or updated
- RED command and failure summary
- GREEN command and pass summary
- full verification command output summary
- scope audit result
- whether the branch is ready for Todd to open the PR
- any blockers or deviations

## Final rule
This handoff exists to expose one bounded pilot view-model contract over the already-merged GE-06 receipt and failure-classifier lanes. Preserve real outputs, preserve blocked-posture honesty, refuse counterfeit shell expansion, and stop there.
