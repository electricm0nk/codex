---
title: GE06-E3-F2 Execution Handoff — Failure Classifier and Owner Mapping
handoff_id: HANDOFF-CODEX-GE-06-E3-F2-CODING-2026-06-22
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: merged
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-readiness-closure-2026-06-22.md
merge_receipt: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md
selected_slice: GE06-E3-F2 — Failure classifier and owner mapping
run_in: Claude Code or equivalent frontier coding harness
code_authority: false
created_at: 2026-06-22
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 6977c86
  recommended_branch: ge06-e3-f2-failure-classifier
  pr_target: develop
allowed_write_scope:
  - src/rules_core/mod.rs
  - src/rules_core/pilot_failure.rs
  - tests/ge06_failure_classifier.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/pilot_compute.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/golden_case_fixture_schema.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E3-F2 Execution Handoff — Failure Classifier and Owner Mapping

## Status
This is the preserved historical stage-specific code-authorizing brief that was consumed by the merged GE06-E3-F2 slice. It must not overwrite any prior GE-06 handoff artifact or be treated as a live handoff.

## Run in
Claude Code or an equivalent frontier coding harness.

At creation this handoff carried `code_authority: true` for GE06-E3-F2 only. After merge, authority has retired into `ge06-e3-f2-merge-receipt-2026-06-22.md`.

## Core problem
Codex now has one merged integrated headless receipt path for the accepted deterministic pilot, but GE-06 still lacks the next narrow surface that maps those receipt facts into one primary failure owner.

The truthful next slice is not a generic incident system and not a broad architecture pass. It is one bounded classifier over the merged receipt surface that exposes the full GE-06 owner vocabulary while remaining honest about the limited evidence the current receipt actually carries.

## Objective
Create the smallest rules-core classifier that maps the merged GE06-E2-F3 receipt into one primary owner for the pilot: model flaw, importer flaw, engine flaw, oracle gap, or UI gap.

The classifier must preserve, at minimum:

1. one required primary owner
2. no terminal `IntegrationIssue` sink
3. optional contributing-owner context or reason text when useful
4. first-broken-contract reasoning grounded on actual observable receipt states
5. a shape that later GE-06 evidence routing can consume without pretending to be a program-wide incident framework

This handoff does not authorize a generic incident framework, comparator logic, parity-report writing, UI work, importer rewrites, or broad rules-core redesign.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-f3-handoff-rack-2026-06-21.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
7. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md`
8. `programs/codex/doctrine/quality-gate-policy.md`
9. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
10. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
11. `/home/ubuntu/workspace/repos/codex/src/rules_core/mod.rs`
12. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
13. `/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs`

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e3-f2-failure-classifier
```

Expected base at handoff creation:

```text
origin/develop = 6977c86
```

If `origin/develop` has advanced, use the current fetched `origin/develop` and record the actual base SHA in the final report.

## Baseline repo posture
Observed during documentary promotion:

```text
## ge06-e2-f3-headless-receipt-path...origin/ge06-e2-f3-headless-receipt-path [gone]
```

Rules:

- do not branch from the stale checked-out topic branch
- reset to current `origin/develop` first
- treat the current branch header as residue from the already-merged E2-F3 work, not as your implementation base

## Toolchain posture
Observed in the current Hermes shell:

```text
cargo is not on the default PATH
$HOME/.cargo/bin/cargo is present and works
```

Use the explicit cargo path in commands below unless your shell already resolves `cargo` and you record that fact in the final report.

## Merge authority boundary
This handoff does not authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the launch/review/merge decisions.

## Allowed write scope
You may write only:

```text
src/rules_core/mod.rs
src/rules_core/pilot_failure.rs
tests/ge06_failure_classifier.rs
```

If you need any other file, stop and report the blocker.

Do not modify `src/rules_core/pilot_compute.rs`, `src/oracle_validation/**`, existing receipt/schema tests, dependency manifests, or repo-root instruction files.

## Required implementation behavior
Keep the implementation inside the rules-core classifier lane.

Preferred outcome:

- add `src/rules_core/pilot_failure.rs`
- update `src/rules_core/mod.rs` only to expose the new module
- consume `PilotHeadlessReceipt` as read-only input
- expose one narrow classifier result with a required primary owner and optional contributing-owner or reason surface
- keep the full GE-06 owner vocabulary explicit
- refuse `IntegrationIssue` as a terminal bucket
- add no broad incident framework and no oracle-validation edits

The classifier may choose its own exact type names, but the result must stay machine-checkable and bounded.

## Required primary-owner vocabulary
The classifier must preserve this primary-owner vocabulary:

```text
ModelFlaw
ImporterFlaw
EngineFlaw
OracleGap
UiGap
```

The first implementation must remain honest about what the merged receipt can currently prove.

At minimum, tests must prove:

```text
supported deterministic receipt with computed outputs but no comparison evidence -> OracleGap
blocked deterministic receipt with claim-blocking rules diagnostics -> EngineFlaw
```

Do not invent importer/model/UI signals that the merged receipt does not yet carry.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_failure_classifier.rs` first.
2. Load the accepted deterministic fixture and build the merged headless receipt through the existing rules-core path.
3. Assert the future classifier preserves:
   - the full five-owner vocabulary
   - a required primary owner
   - `OracleGap` for the computed receipt path that still lacks comparison evidence
   - `EngineFlaw` for a blocked receipt with claim-blocking rules diagnostics
   - no `IntegrationIssue` sink or equivalent terminal vague bucket
4. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet
```

5. Confirm it fails for the expected reason: the classifier surface does not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- add the new classifier module
- expose it from `src/rules_core/mod.rs`
- do not modify `pilot_compute.rs`
- do not invent a broader incident framework
- do not add oracle-validation behavior

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet
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

- [ ] A new failure-classifier proof test exists at `tests/ge06_failure_classifier.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/rules_core/pilot_failure.rs` is the only new production module.
- [ ] `src/rules_core/mod.rs` changes only to expose that module.
- [ ] The classifier consumes the merged receipt as read-only input.
- [ ] The full five-owner vocabulary exists exactly as the GE-06 doctrine requires.
- [ ] A computed receipt with no comparison evidence classifies to `OracleGap`.
- [ ] A blocked receipt with claim-blocking rules diagnostics classifies to `EngineFlaw`.
- [ ] No terminal `IntegrationIssue` sink exists.
- [ ] No incident framework, oracle-validation edit, importer rewrite, or UI work is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- edits to `src/oracle_validation/**`
- edits to `src/rules_core/pilot_compute.rs`
- a generic cross-program incident framework
- parity comparator or report writer behavior
- UI implementation or GE-07 work
- importer or rules-engine rewrites disguised as classifier work
- Cargo dependency additions
- rewrites of existing receipt/schema tests
- broad Pathfinder support beyond the merged receipt states already observable here

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
This handoff exists to expose one small primary-owner classifier over the already-merged GE-06 headless receipt path. Preserve the full owner vocabulary, classify only what the current receipt can truthfully support, refuse counterfeit incident-framework expansion, and stop there.
