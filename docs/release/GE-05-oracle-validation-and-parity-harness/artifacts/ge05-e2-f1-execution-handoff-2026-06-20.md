---
title: GE05-E2-F1 Execution Handoff — Golden-Case Fixture Schema
stc_id: STC-CODEX-GE-05
artifact_type: execution-handoff
stc_kind: execution-handoff
template_version: 1
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: active
owner: Todd Hintzmann
scope: repo
code_authority: true
source_stc: ./README.md
source_readiness_closure: ./artifacts/ge05-e2-f1-execution-readiness-closure-2026-06-20.md
selected_slice: GE05-E2-F1 — Golden-case fixture schema
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  base_branch: origin/develop
  execution_branch: ge05-e2-f1-golden-fixture-schema
  write_scope:
    - src/lib.rs
    - src/oracle_validation/mod.rs
    - src/oracle_validation/golden_fixture.rs
    - tests/golden_case_fixture_schema.rs
    - tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
reviewed_at: 2026-06-20
---

# GE05-E2-F1 Execution Handoff — Golden-Case Fixture Schema

## Deliverable Type
`implementation-ready`

## Execution Readiness
`codex-ready`

## Harness route

This is a **code-authorizing execution handoff**.

Run it in:

```text
Claude Code / frontier coding harness
```

Do **not** execute this handoff as a Hermes research or collection run. Hermes has already completed the required readiness and oracle-output collection steps for this slice.

## Exact objective

Implement the smallest Rust GE-05 oracle-validation slice that defines a **golden-case fixture schema** capable of representing the PF1 Core Rulebook Human Fighter level 1 case without claiming parity.

This handoff is only for:

```text
GE05-E2-F1 — Golden-case fixture schema
```

The slice must establish a typed shape and minimal fixture loader that can represent:

- stable case identity
- fixture version
- source package / campaign identity
- character input reference or chosen-input summary
- legacy PCGen oracle route metadata
- old-system raw-output reference and retention mode
- old-system raw-output SHA-256
- reduced old-system runtime facts reference or compact reduced-facts structure
- unresolved / absent Codex new-system output
- comparison dimensions with non-passing states such as `candidate`, `blocked`, or `not_yet_grounded`
- normalization declarations as references/placeholders, not implemented normalization behavior
- known-gap references
- claim target and current claim status
- provisional assumption flags
- claim-blocking diagnostics for missing required fixture fields

## Target repo / workdir

```text
/home/ubuntu/workspace/repos/codex
```

Current grounded repo facts:

- GE-03 code is merged to `develop`.
- GE-04 code is merged to `origin/develop` via merge commit `2f32636`.
- The current local checkout may still be on stale branch `ge04-e1-f1-character-input-record-shape`.
- `origin/develop` is the correct integration base for this slice.
- Local untracked `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, and `target/` may exist in the checkout.
- `AGENTS.md` is the repo-root conduct surface. Follow it.

## Branch / worktree policy

Before implementation, start from `origin/develop`.

Preferred existing-checkout path:

```bash
git fetch origin --prune
git switch -C ge05-e2-f1-golden-fixture-schema origin/develop
```

Acceptable clean-worktree alternative:

```bash
git fetch origin --prune
git worktree add /home/ubuntu/workspace/worktrees/codex-ge05-e2-f1 -b ge05-e2-f1-golden-fixture-schema origin/develop
```

If using a fresh worktree, preserve the conduct instructions from the main checkout or otherwise ensure the coding harness has read-equivalent access to:

```text
/home/ubuntu/workspace/repos/codex/AGENTS.md
/home/ubuntu/workspace/repos/codex/CLAUDE.md
```

Do **not** implement directly on `main`.

Do **not** continue implicitly from stale local branch `ge04-e1-f1-character-input-record-shape`.

If branch/worktree setup would overwrite unrelated local work, stop and report instead of mixing scopes.

## Exact allowed write scope

You may create or modify only these paths in `/home/ubuntu/workspace/repos/codex`:

```text
src/lib.rs
src/oracle_validation/mod.rs
src/oracle_validation/golden_fixture.rs
tests/golden_case_fixture_schema.rs
tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
```

Optional only if strictly necessary and justified in the final report:

```text
Cargo.toml
Cargo.lock
```

Default stance: **do not add dependencies for GE05-E2-F1.** Use native Rust structs and a small text-fixture parser consistent with the GE04 `rules_core::character_input` slice unless there is a narrow, unavoidable reason.

Do not modify `AGENTS.md`, `CLAUDE.md`, or `target/`.

Do not write outside `/home/ubuntu/workspace/repos/codex`.

## Explicitly forbidden scope

Do not implement or modify:

```text
/home/ubuntu/workspace/repos/pcgen/**
programs/codex/**
src/pcgen_import/**
src/rules_core/character_input.rs
GE05-E2-F2 fixture instance finalization
GE05-E3 output capture adapter
GE05-E3 normalization rule engine
GE05-E4 comparator
GE05-E4 diff reporter
GE05-E4 parity report writer
PCGen command runner
PCGen XML parser beyond storing references/reduced facts needed by this schema
raw PCGen XML fixture commits
GE-06 pilot truth closure
UI work
release/packaging work
```

This slice may read upstream requirement artifacts and runtime receipts as evidence. It must not write them.

## Required reads before coding

Read these first:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f1-execution-readiness-closure-2026-06-20.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/golden-case-fixture-format.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/technical-requirements.md` sections TR-05-005, TR-05-007, TR-05-012, TR-05-013, TR-05-015
6. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
7. `/home/ubuntu/workspace/repos/codex/tests/character_input_record.rs`

Conditional reads:

- Read `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md` only if modeling known-gap references beyond opaque IDs.
- Read `/home/ubuntu/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/parity-report-format.md` only if implementation starts drifting into report shape. If that drift occurs, stop: GE05-E4 is out of scope.

## Upstream evidence this slice must preserve

### GE-05 fixture-format evidence

From `artifacts/golden-case-fixture-format.md`, the fixture schema must be able to represent:

- `case_id`
- `case_version`
- `scope`
- `source_package`
- `character_input`
- `legacy_oracle`
- `codex_output`
- `dimensions`
- `normalization`
- `known_gaps`
- `claim_target`

The fixture must be narrow enough for the PF1 Human Fighter pilot and broad enough to represent unresolved oracle/new-system outputs without falsifying readiness.

### GE05-E1-F2 runtime evidence

From `artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`:

- PCGen headless batch exporter works.
- The working provisional `.pcg` identity is:

```text
CAMPAIGN:Core Rulebook
GAMEMODE:Pathfinder_RPG
```

- Raw XML was produced at:

```text
/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml
```

- Raw XML retention posture is:

```text
local-generated-only
```

- Raw XML SHA-256 is:

```text
3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1
```

- Reduced runtime facts are old-system runtime evidence for the provisional input, not canonical GE-06 truth.

### Provisional assumption boundary

The GE05-E1-F2 output reflects non-canonical assumptions:

- Human ability bonus set to `+2 Strength`
- no final equipment loadout
- no skill-rank allocation
- no additional feat-slot closure beyond `Power Attack`

The fixture schema must preserve these as provisional assumptions or claim blockers. It must not promote them into final GE-06 pilot truth.

## Required implementation shape

Create a new module:

```text
src/oracle_validation/golden_fixture.rs
```

Expose it from:

```text
src/oracle_validation/mod.rs
src/lib.rs
```

The public API may use different exact names if the tests remain clear, but it must support these concepts:

```rust
GoldenCaseFixture / equivalent:
  case_id
  case_version
  scope
  source_package
  character_input_ref_or_summary
  legacy_oracle
  codex_output
  dimensions
  normalization_refs
  known_gap_refs
  claim_target
  current_claim_status
  provisional_assumptions

SourcePackage / equivalent:
  system
  package
  campaign
  game_mode

LegacyOracleEvidence / equivalent:
  route
  trust_tier_or_evidence_kind
  raw_output_ref
  raw_output_retention
  raw_output_sha256
  reduced_facts_ref_or_summary

CodexOutputEvidence / equivalent:
  state unresolved_or_absent
  output_ref optional
  diagnostics optional

ComparisonDimension / equivalent:
  id
  status candidate_or_blocked_or_not_yet_grounded

FixtureDiagnostic / equivalent:
  class
  severity
  subject_ref
  message
  claim_blocking

FixtureLoadResult / equivalent:
  fixture_or_none
  diagnostics
```

### Required behavior

Minimum behavior for this slice:

1. Load a narrow text fixture for `pf1-crb-human-fighter-level1`.
2. Preserve `Core Rulebook` / `Pathfinder_RPG` source identity.
3. Preserve raw-output retention mode as `local_generated_only` or equivalent.
4. Preserve the raw-output SHA-256 exactly.
5. Represent old-system evidence as runtime evidence, not static source truth.
6. Represent Codex/new-system output as unresolved or absent without treating parity as passed.
7. Represent provisional assumptions explicitly.
8. Represent current claim status as **not** `OracleChecked`.
9. Return structured, claim-blocking diagnostics when required fixture fields are missing.
10. Avoid any comparator, normalization engine, report writer, or PCGen runner behavior.

## TDD requirement

TDD is mandatory.

Execution order:

1. Create `tests/golden_case_fixture_schema.rs` with a failing test before implementing production code.
2. Create the minimal fixture text file under `tests/fixtures/oracle_validation/` if needed by the failing test.
3. Run the specific test and capture the real failure output.
4. Implement the smallest production code needed to pass.
5. Add only the next failing test, then repeat.
6. Run full verification.

Recommended first test:

```rust
loads_pf1_human_fighter_fixture_with_oracle_hash_and_provisional_assumptions
```

It should assert at minimum:

- `case_id == "pf1-crb-human-fighter-level1"`
- source campaign is `Core Rulebook`
- source game mode is `Pathfinder_RPG`
- raw-output retention mode is `local_generated_only` or equivalent
- raw-output SHA-256 is `3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1`
- old-system evidence state is runtime evidence, not static source truth
- Codex/new-system output state is unresolved or absent
- provisional assumptions include Human `+2 Strength`, no final equipment loadout, no skill allocation, and no additional feat-slot closure beyond `Power Attack`
- current claim status is not `OracleChecked`

Recommended second test:

```rust
missing_legacy_oracle_hash_returns_claim_blocking_diagnostic
```

Recommended third test:

```rust
fixture_can_represent_blocked_or_unresolved_codex_output_without_passing_parity
```

## Required fixture file

Create a minimal text fixture only if needed by the tests:

```text
tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
```

The text format may be deliberately simple and pilot-local. Prefer a key/value format consistent with the existing GE04 fixture-loader style. Do not introduce a broad serialization framework unless strictly necessary.

The fixture should encode the old-system evidence from the GE05-E1-F2 successful receipt and mark the following as provisional/non-canonical:

```text
human_ability_bonus=+2 Strength
final_equipment_loadout=unresolved
skill_allocation=unresolved
additional_feat_slot_closure=unresolved
```

## Verification commands

Specific RED/GREEN target:

```bash
cargo test --test golden_case_fixture_schema
```

Full regression command:

```bash
cargo test
```

The final report must include:

- first failing test output
- final passing specific-test output
- final passing full-suite output
- `git status --short`
- file-granular scope audit
- whether the branch is ready for Todd to open or merge the PR

## File-granular scope audit

Before reporting completion, run:

```bash
git diff --name-only
git ls-files --others --exclude-standard
```

Confirm that changed/untracked implementation files are limited to the allowed write scope, excluding pre-existing untracked conduct/build artifacts that were not modified by this run.

If `Cargo.toml` or `Cargo.lock` changes, explain why a dependency change was unavoidable. The expected implementation should not require dependency changes.

## Acceptance criteria

This handoff is complete when:

- a GE-05 `oracle_validation::golden_fixture` surface exists
- the schema can load or construct the PF1 Human Fighter fixture case
- the fixture preserves old-system runtime evidence metadata and SHA-256
- the fixture records provisional assumptions without promoting them to canonical truth
- the fixture represents unresolved Codex/new-system output without passing parity
- missing required legacy-oracle evidence produces structured claim-blocking diagnostics
- no comparator, normalization engine, report writer, PCGen runner, or GE-06 truth closure is implemented
- `cargo test --test golden_case_fixture_schema` passes
- `cargo test` passes
- file-granular scope audit passes

## Explicit non-goals

Do not implement:

- GE05-E2-F2 final fixture instance closure
- GE05-E3 output capture adapter
- GE05-E3 normalization rules
- GE05-E4 comparator
- GE05-E4 diff reporter
- GE05-E4 parity report writer
- PCGen command runner
- raw XML parsing pipeline
- broad fixture serialization framework
- GE-06 canonical pilot choices
- UI behavior
- release/packaging behavior

## Stop conditions

Stop and report if:

- branch/worktree setup cannot safely start from `origin/develop`
- `AGENTS.md` or equivalent conduct instructions are unavailable to the coding harness
- the implementation requires modifying files outside the allowed write scope
- the implementation requires new dependencies and the reason is not narrowly justified
- tests cannot be made to fail first for the intended reason
- comparator/report/normalization behavior appears necessary to make this slice pass
- GE-06 canonical pilot choices would need to be decided to complete this slice

## Merge authority boundary

This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the merge decision and merge action.

## Delivery report format

When complete, report:

1. branch used
2. files changed
3. tests added
4. first failing test output
5. final specific-test output
6. final full-suite output
7. scope audit output
8. whether the branch is ready for Todd to open or merge the PR
9. unresolved risks or follow-on handoff candidates

## Next-stage boundary

If this handoff succeeds, the next GE-05 slice should be selected separately. Likely candidates are:

- `GE05-E2-F2 — PF1 Human Fighter level 1 fixture instance`
- `GE05-E3-F2 — PCGen output capture adapter`
- `GE05-E3-F1 — New-system output contract adapter`

Do not implement those in this handoff.
