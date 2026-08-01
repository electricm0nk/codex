---
title: GE06-E3-F1 Execution Handoff — Selected Parity-Dimension Adapter
handoff_id: HANDOFF-CODEX-GE-06-E3-F1-CODING-2026-06-22
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
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-readiness-closure-2026-06-22.md
merge_receipt: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md
selected_slice: GE06-E3-F1 — Selected parity-dimension adapter
run_in: Claude Code or equivalent frontier coding harness
code_authority: false
created_at: 2026-06-22
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  expected_base_sha_at_creation: 6977c86
  recommended_branch: ge06-e3-f1-selected-parity-dimensions
  pr_target: develop
allowed_write_scope:
  - src/oracle_validation/mod.rs
  - src/oracle_validation/selected_parity_dimensions.rs
  - tests/ge06_selected_parity_dimensions.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - src/rules_core/**
  - src/pcgen_import/**
  - src/oracle_validation/golden_fixture.rs
  - tests/ge06_pilot_headless_receipt.rs
  - tests/golden_case_fixture_schema.rs
  - programs/codex/**
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
---

# GE06-E3-F1 Execution Handoff — Selected Parity-Dimension Adapter

## Status
This is the preserved historical stage-specific code-authorizing brief that was consumed by the merged GE06-E3-F1 slice. It must not overwrite any prior GE-06 handoff artifact or be treated as a live handoff.

## Run in
Claude Code or an equivalent frontier coding harness.

At creation this handoff carried `code_authority: true` for GE06-E3-F1 only. After merge, authority has retired into `ge06-e3-f1-merge-receipt-2026-06-22.md`.

## Core problem
Codex now has one merged integrated headless receipt path for the accepted deterministic pilot, but GE-06 still lacks the next narrow carrier that projects that receipt into stable selected parity dimensions for later GE-05 comparison work.

The truthful next slice is not a comparator, not normalization, and not a parity report. It is one bounded adapter that emits the mandatory selected pilot dimensions with their current new-system values or references while keeping the claim tier at `Computed`.

## Objective
Create the smallest oracle-validation adapter that projects the merged GE06-E2-F3 receipt into a machine-checkable selected-dimension surface for the GE-06 pilot.

The adapter must preserve, at minimum:

1. stable dimension IDs for the mandatory pilot dimensions
2. pilot identity carried from the merged receipt
3. the currently grounded new-system values or references for those dimensions
4. explicit `Computed`-tier posture rather than `Oracle-checked`
5. a shape that later GE-05 comparison work can consume without having to reverse-engineer the receipt

This handoff does not authorize comparator logic, normalization, parity-report writing, PCGen execution, rules-core rewrites, UI work, or broad oracle-validation framework design.

## Required reads
Read these before editing code:

1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-f3-handoff-rack-2026-06-21.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-requirements.md`
7. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/technical-design.md`
8. `programs/codex/doctrine/quality-gate-policy.md`
9. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md`
10. `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md`
11. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
12. `/home/ubuntu/workspace/repos/codex/tests/ge06_pilot_headless_receipt.rs`
13. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/mod.rs`
14. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
15. `/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs`

## Branch setup
From `/home/ubuntu/workspace/repos/codex`:

```bash
git fetch origin --prune
git switch develop
git reset --hard origin/develop
git switch -c ge06-e3-f1-selected-parity-dimensions
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
src/oracle_validation/mod.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_selected_parity_dimensions.rs
```

If you need any other file, stop and report the blocker.

Do not modify `src/rules_core/**`, `golden_fixture.rs`, existing receipt/schema tests, dependency manifests, or repo-root instruction files.

## Required implementation behavior
Keep the implementation inside the oracle-validation lane.

Preferred outcome:

- add `src/oracle_validation/selected_parity_dimensions.rs`
- update `src/oracle_validation/mod.rs` only to expose the new module
- consume `build_pilot_headless_receipt` as read-only input
- emit one narrow selected-dimension carrier for the mandatory pilot dimensions only
- preserve a `Computed` claim-tier floor and refuse to imply `Oracle-checked`
- add no comparator, no normalization behavior, no report-writer, and no PCGen route

The adapter may choose its own exact type names, but the emitted surface must stay machine-checkable and reusable.

## Mandatory selected dimensions
The emitted carrier must represent exactly this mandatory pilot dimension set:

```text
character.identity
combat.baseline_melee_attack_bonus
defense.baseline_armor_class
defense.total_save.fortitude
defense.total_save.reflex
defense.total_save.will
skill.selected_modifier.climb
skill.selected_modifier.intimidate
skill.selected_modifier.swim
```

For the accepted deterministic fixture, the adapter must preserve the already-grounded new-system values or references behind those dimensions:

```text
character.identity -> case_id=pf1-crb-human-fighter-level1, source_package_id=pf1.core_rulebook
combat.baseline_melee_attack_bonus -> 5
defense.baseline_armor_class -> 17
defense.total_save.fortitude -> 4
defense.total_save.reflex -> 2
defense.total_save.will -> 1
skill.selected_modifier.climb -> 5
skill.selected_modifier.intimidate -> 3
skill.selected_modifier.swim -> 5
```

These are computed new-system facts only. They are not old-vs-new comparison verdicts.

## Strict TDD sequence
This handoff requires the following sequence.

### RED
1. Create `tests/ge06_selected_parity_dimensions.rs` first.
2. Load the accepted deterministic fixture and build the merged headless receipt through the existing rules-core path.
3. Assert the future adapter preserves:
   - the mandatory dimension IDs listed above
   - pilot identity for `character.identity`
   - the already-grounded current new-system values or references for the remaining dimensions
   - an explicit claim-tier floor no higher than `Computed`
4. Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet
```

5. Confirm it fails for the expected reason: the adapter surface does not exist yet.

If the test passes immediately, it is too weak or not testing new behavior.

### GREEN
Implement the smallest change needed to pass:

- add the new adapter module
- expose it from `src/oracle_validation/mod.rs`
- do not modify rules-core logic or the golden-fixture schema
- do not introduce comparator or report behavior

### VERIFY
Run:

```bash
"$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet
"$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet
"$HOME/.cargo/bin/cargo" test --test golden_case_fixture_schema --quiet
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

- [ ] A new selected-dimension proof test exists at `tests/ge06_selected_parity_dimensions.rs`.
- [ ] The new test fails before production code changes.
- [ ] `src/oracle_validation/selected_parity_dimensions.rs` is the only new production module.
- [ ] `src/oracle_validation/mod.rs` changes only to expose that module.
- [ ] The adapter consumes the merged receipt as read-only input.
- [ ] The emitted carrier contains exactly the mandatory selected pilot dimensions listed above.
- [ ] `character.identity` preserves pilot identity from the merged receipt.
- [ ] The remaining selected dimensions preserve the already-grounded current new-system values or references from the merged receipt.
- [ ] The carrier keeps its claim-tier floor at `Computed` and does not imply `Oracle-checked`.
- [ ] No comparator, normalization, report-writer, PCGen route, UI work, or rules-core rewrite is added.
- [ ] All verification commands pass.
- [ ] Scope audit shows only authorized files changed or newly created.

## Non-goals
Do not implement:

- edits to `src/rules_core/**`
- edits to `src/oracle_validation/golden_fixture.rs`
- parity comparator or pass/fail verdict logic
- normalization engine behavior
- parity report writer
- PCGen execution or exporter capture
- `Oracle-checked` or broader compatibility claims
- UI/view-model/export-sheet work
- Cargo dependency additions
- rewrites of existing receipt/schema tests
- broad Pathfinder support beyond the selected pilot dimensions

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
This handoff exists to expose one small selected-dimension carrier over the already-merged GE-06 headless receipt path. Preserve the selected pilot dimensions, preserve the `Computed` claim-tier floor, refuse counterfeit parity/report expansion, and stop there.
