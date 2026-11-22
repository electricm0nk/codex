---
title: GE05-E2-F2 Execution Handoff — PF1 Human Fighter Level 1 Governed Fixture Instance
handoff_id: HANDOFF-CODEX-GE-05-E2-F2-CODING-2026-06-24
stc_id: STC-CODEX-GE-05
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: running-under-card-triggered-harness
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md
source_stc: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/README.md
readiness_closure: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md
selected_slice: GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance
run_in: Claude Code or equivalent frontier coding harness
code_authority: true
created_at: 2026-06-24
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104
  recommended_branch: ge05-e2-f2-governed-fixture-instance
  pr_target: develop
allowed_write_scope:
  - src/oracle_validation/golden_fixture.rs
  - tests/golden_case_fixture_schema.rs
  - tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - src/lib.rs
  - src/oracle_validation/mod.rs
  - src/oracle_validation/selected_parity_dimensions.rs
  - src/pcgen_import/**
  - src/rules_core/**
  - tests/ge06_selected_parity_dimensions.rs
  - tests/fixtures/rules_core/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE05-E2-F2 Execution Handoff — PF1 Human Fighter Level 1 Governed Fixture Instance

## Status
This is the live stage-specific code-authorizing brief for GE05-E2-F2.

It carries `code_authority: true` for GE05-E2-F2 only and is currently `running-under-card-triggered-harness` via `kanban://codex-phase-2/t_0cdc64d0`.

## Run in
Claude Code or an equivalent frontier coding harness.

Do not run this in Hermes as a documentary card. Hermes produced this handoff; the coding harness implements it.

## Core problem
GE05-E2-F1 created a truthful schema and seed fixture, but that fixture still encodes provisional input assumptions from the old GE05-E1-F2 oracle route rather than the now-accepted deterministic pilot contract. The repo therefore has a first comparison container, but not yet the first governed case instance.

The lesser approach would jump straight to comparator logic or pretend the GE-06 computed surfaces already equal oracle evidence. That would counterfeit closure. The next honest move is narrower: govern the first case instance, preserve the real legacy evidence unchanged, and keep every still-missing comparison field explicitly unresolved or blocked.

## Objective
Upgrade the existing repo-local PF1 Human Fighter level 1 golden fixture from provisional seed to governed first-case instance without inventing parity results.

The result must prove:

1. the fixture remains scoped to `pf1-crb-human-fighter-level1`
2. inherited character-input truth is grounded from the accepted deterministic GE-06 pilot contract rather than left in provisional assumptions
3. the existing GE05-E1-F2 legacy oracle route, raw-output reference, retention posture, reduced-facts reference, and SHA-256 remain preserved exactly
4. any final expected values or Codex output references are either linked to already-grounded repo truth or left explicitly unresolved/blocked with known-gap posture
5. the fixture still does not imply `OracleChecked`

## Exact deliverable
Produce the smallest bounded change that makes the current fixture file the governed first-case representation.

At minimum, the finished state should include all of the following:

- `character_input_ref` points at the accepted deterministic fixture lineage rather than the minimal placeholder input
- the previously provisional Human ability bonus / feat-slot / skill-allocation / equipment closure is no longer carried as if it were unresolved pilot truth
- the old-system oracle evidence fields from GE05-E1-F2 stay intact
- the fixture remains explicit about what is still unresolved, blocked, or known-gap on the Codex/parity side
- tests prove the governed fixture loads cleanly and still does not claim parity

## Branch / worktree policy
Do not continue from the current checked-out branch. Live repo verification found the checkout on `ge06-e3-f2-classifier-impl`, while the truthful base for this slice is current `origin/develop` at `7bc89e8c1edf8f1d1a6d490a0ad28ac72fc6f104`.

Start from fresh `origin/develop`:

```bash
git fetch origin --prune
git switch -C ge05-e2-f2-governed-fixture-instance origin/develop
```

Acceptable clean-worktree alternative:

```bash
git fetch origin --prune
git worktree add /home/ubuntu/workspace/worktrees/codex-ge05-e2-f2 -b ge05-e2-f2-governed-fixture-instance origin/develop
```

If branch/worktree setup would overwrite unrelated local work, stop and report instead of mixing scopes.

## Required reads before coding
Read these first, in order:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-readiness-closure-2026-06-24.md`
3. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/execution-handoff.md`
4. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f1-merge-receipt-2026-06-21.md`
5. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/golden-case-fixture-format.md`
6. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md`
7. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/epic-breakdown.md`
8. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
9. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md`
10. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
11. `/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs`
12. `/home/ubuntu/workspace/repos/codex/tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`
13. `/home/ubuntu/workspace/repos/codex/tests/ge06_selected_parity_dimensions.rs`

## Required RED -> GREEN execution pattern
TDD is mandatory here.

### RED first
Before changing production code, add or update tests in `tests/golden_case_fixture_schema.rs` so they fail for the intended governed-instance reason.

At minimum, the failing assertions must cover:

- the governed fixture now references `fixture:rules_core/pf1_human_fighter_level1_ge06_deterministic_input`
- legacy oracle evidence fields remain unchanged from the GE05-E1-F2 seed evidence
- the former provisional pilot-input closures are no longer treated as unresolved pilot truth
- unresolved or blocked Codex/parity posture remains explicit and non-passing
- `fixture.parity_claimed()` remains false and `current_claim_status` remains below `OracleChecked`

Capture the RED result in the final report.

### GREEN second
After the intended RED failure, make the smallest change necessary inside the allowed write scope to satisfy the governed-instance contract.

Prefer this order:

1. update the fixture file itself
2. make the minimum supporting `golden_fixture.rs` change only if the existing loader/tests cannot express the governed state honestly
3. keep scope as narrow as possible; do not widen into new modules or adjacent GE-06 surfaces

## Exact scope rules
You may modify only the three allowed paths listed in the frontmatter.

Interpret that narrowly:

- `golden_fixture.rs` is available only for minimum loader/model support needed by the governed instance
- `tests/golden_case_fixture_schema.rs` is the proof surface
- `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` is the single governed fixture artifact to evolve

Do not create a second competing PF1 Human Fighter golden fixture file.

Do not touch `src/lib.rs` just to repair the stale crate-level prose note. That cleanup remains real, but it is outside this slice because this handoff does not require `src/lib.rs`.

## Non-goals
Do not implement or modify:

- GE05-E3 output capture adapters
- GE05-E3 normalization rules
- GE05-E4 comparator, diff reporter, or parity report writer
- GE05-E5 known-gap ledger implementation
- GE06 selected-parity adapter logic
- deterministic input fixture content under `tests/fixtures/rules_core/**`
- any file under `/home/ubuntu/workspace/repos/pcgen`
- any documentary artifact under `programs/codex/**`
- release, packaging, CI, UI, or frontend work

## Output-truth rule
Use already-grounded evidence where it exists. Do not invent it where it does not.

That means:

- preserve the real GE05-E1-F2 legacy oracle evidence exactly
- if a stable already-grounded repo-local Codex output reference can be expressed honestly under the existing fixture shape, use it
- otherwise keep the Codex side explicitly unresolved/blocked and record the known-gap posture instead of fabricating a resolved output contract

No part of this slice may promote the case to `OracleChecked`.

## Verification commands
Run at minimum:

```bash
"$HOME/.cargo/bin/cargo" test --test golden_case_fixture_schema --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet
"$HOME/.cargo/bin/cargo" test --quiet
```

## Acceptance checklist
- [ ] RED observed first for governed-instance expectations
- [ ] the single PF1 Human Fighter fixture file is now the governed first-case representation
- [ ] inherited character inputs are grounded from the accepted deterministic contract
- [ ] legacy oracle route/hash/reduced-facts evidence remains intact
- [ ] unresolved or blocked Codex/parity posture remains explicit and non-passing
- [ ] no files outside the allowed write scope changed
- [ ] all verification commands pass

## Final report requirements
When you stop, report exactly:

- branch name
- actual base SHA used
- files changed
- RED test command and failure summary
- GREEN verification commands and results
- whether a stable Codex output ref was grounded or left unresolved, and why
- scope audit result
- PR-readiness posture

The governed Kanban CODE card is the launch trigger for Claude Code / the frontier coding harness. Todd retains merge authority. Stop at verified branch/PR-ready state.