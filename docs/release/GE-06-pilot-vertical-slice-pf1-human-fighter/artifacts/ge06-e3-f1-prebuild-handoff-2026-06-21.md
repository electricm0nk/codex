---
title: GE06-E3-F1 Prebuild Handoff Draft — Selected Parity-Dimension Adapter
handoff_id: HANDOFF-CODEX-GE-06-E3-F1-PREBUILD-2026-06-21
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff-draft
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: prebuilt-draft
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md
selected_slice: GE06-E3-F1 — Selected parity-dimension adapter
run_in: Claude Code or equivalent frontier coding harness, but only after post-E2-F3 promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  recommended_branch: ge06-e3-f1-selected-parity-dimensions
future_live_artifacts:
  - artifacts/ge06-e3-f1-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge06-e3-f1-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge06-e3-f1-merge-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - src/oracle_validation/mod.rs
  - src/oracle_validation/selected_parity_dimensions.rs
  - tests/ge06_selected_parity_dimensions.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/**
  - src/pcgen_import/**
  - tests/ge06_pilot_headless_receipt.rs
  - tests/golden_case_fixture_schema.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E3-F1 Prebuild Handoff Draft — Selected Parity-Dimension Adapter

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after E2-F3 merge evidence exists.

## Objective
Once E2-F3 is merged, create the smallest new-system adapter that projects the merged headless GE-06 receipt into a selected parity-dimension surface for later GE-05 comparison work.

The adapter should preserve:
- stable dimension IDs
- new-system value/reference surfaces for the selected pilot dimensions
- claim-tier honesty (`Computed`, not `Oracle-checked`)
- known-gap / blocked posture when required comparison inputs do not yet exist

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-merge-receipt-YYYY-MM-DD.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md`
6. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
8. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
9. `/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs`
10. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md`
11. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md`
12. `programs/codex/doctrine/quality-gate-policy.md`

## Post-merge promotion gate
A future documentary run may mint the live E3-F1 execution handoff only if all are true:

1. E2-F3 is merged and receipted.
2. The merged repo still exposes a stable headless receipt surface with the fields E3-F1 expects.
3. The candidate write scope below remains the smallest truthful implementation.
4. No new upstream doctrine changed the selected-dimension list or claim-tier boundary.

If any item fails, throw this draft away and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
src/oracle_validation/mod.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_selected_parity_dimensions.rs
```

The goal of that future code slice should be to keep E3-F1 entirely inside the oracle-validation lane and off the E2-F3 rules-core surface.

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. write a failing focused test first for the selected-dimension projection
2. consume the merged E2-F3 receipt shape as read-only input
3. emit one machine-checkable selected-dimension carrier for the mandatory pilot dimensions only
4. preserve `Computed` claim posture and explicit known-gap / blocked states
5. avoid comparator, normalization, report-writer, or PCGen-runner work

## Candidate selected dimensions
The future live handoff should start from this dimension list unless the post-merge audit proves it too broad or too narrow:
- `character.identity`
- `combat.baseline_melee_attack_bonus`
- `defense.baseline_armor_class`
- `defense.total_save.fortitude`
- `defense.total_save.reflex`
- `defense.total_save.will`
- `skill.selected_modifier.climb`
- `skill.selected_modifier.intimidate`
- `skill.selected_modifier.swim`

## Non-goals
The future live handoff must not authorize:
- old-system PCGen execution
- parity comparator or pass/fail verdict logic
- normalization rule implementation
- report writer implementation
- `Oracle-checked` claims
- UI or GE-07 work
- edits to `src/rules_core/**`
- changes to `tests/ge06_pilot_headless_receipt.rs` or `tests/golden_case_fixture_schema.rs`

## Why this draft exists
Todd asked for the E3 packet family to be prebuilt before E2-F3 truth lands so the next launch gates are explicit.

This draft satisfies that request without counterfeit activation: it captures the bounded write scope, required reads, and selected-dimension target while leaving the real code-authorizing moment in the future, where it belongs.
