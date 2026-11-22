---
title: GE06-E3-F2 Prebuild Handoff Draft — Failure Classifier and Owner Mapping
handoff_id: HANDOFF-CODEX-GE-06-E3-F2-PREBUILD-2026-06-21
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
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md
selected_slice: GE06-E3-F2 — Failure classifier and owner mapping
run_in: Claude Code or equivalent frontier coding harness, but only after post-E2-F3 promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  recommended_branch: ge06-e3-f2-failure-classifier
future_live_artifacts:
  - artifacts/ge06-e3-f2-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge06-e3-f2-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge06-e3-f2-merge-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - src/rules_core/mod.rs
  - src/rules_core/pilot_failure.rs
  - tests/ge06_failure_classifier.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/ge06_pilot_headless_receipt.rs
  - tests/golden_case_fixture_schema.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E3-F2 Prebuild Handoff Draft — Failure Classifier and Owner Mapping

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after E2-F3 merge evidence exists.

## Objective
Once E2-F3 is merged, create the smallest classifier surface that maps merged integrated GE-06 receipt facts into one primary failure owner for the pilot: model flaw, importer flaw, engine flaw, oracle gap, or UI gap.

The classifier should preserve:
- one required primary owner
- optional contributing owners when useful
- a stable reason/explanation for why that owner was chosen
- explicit blocked posture rather than vague “integration issue” language

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-merge-receipt-YYYY-MM-DD.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md`
7. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
8. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
9. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
10. `programs/codex/doctrine/quality-gate-policy.md`

## Post-merge promotion gate
A future documentary run may mint the live E3-F2 execution handoff only if all are true:

1. E2-F3 is merged and receipted.
2. The merged repo still exposes structured receipt diagnostics the classifier can evaluate.
3. The candidate write scope below remains the smallest truthful implementation.
4. No upstream doctrine has introduced a different required owner vocabulary.

If any item fails, discard this draft and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
src/rules_core/mod.rs
src/rules_core/pilot_failure.rs
tests/ge06_failure_classifier.rs
```

The goal of that future code slice should be to keep E3-F2 inside the rules-core integration lane and off the E3-F1 oracle-validation adapter surface.

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. write a failing focused classifier test first
2. consume the merged E2-F3 receipt and diagnostics as read-only input
3. emit one narrow classification result with a required primary owner and optional contributing owners
4. map blocked/computed receipt scenarios through the first-broken-contract rule from GE-06 technical design
5. refuse to create a terminal `IntegrationIssue` bucket

## Draft owner vocabulary
The future live handoff should preserve exactly this primary-owner enum unless post-merge truth disproves it:
- `ModelFlaw`
- `ImporterFlaw`
- `EngineFlaw`
- `OracleGap`
- `UiGap`

## Non-goals
The future live handoff must not authorize:
- generic cross-program incident taxonomy work
- parity comparator or report writer behavior
- UI implementation
- importer or rules-engine rewrites disguised as classifier work
- edits to `src/oracle_validation/**`
- changes to `tests/ge06_pilot_headless_receipt.rs`

## Why this draft exists
Todd asked for the E3 packet family to be prebuilt before E2-F3 truth lands so the next launch gates are explicit.

This draft satisfies that request without counterfeit activation: it captures the bounded write scope, required reads, and owner-mapping target while leaving the real code-authorizing moment in the future, where it belongs.
