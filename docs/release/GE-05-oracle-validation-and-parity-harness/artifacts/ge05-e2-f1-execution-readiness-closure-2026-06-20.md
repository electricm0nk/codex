---
title: GE05-E2-F1 Execution-Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-05
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE05-E2-F1 — Golden-case fixture schema
workflow_route: readiness-closure
work_type: implementation-readiness
readiness: execution-ready
status: active
created_at: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
handoff_may_be_created: true
recommended_next_artifact: ../execution-handoff.md
run_in: Hermes
handoff_target_harness: Claude Code / frontier coding harness
---

# GE05-E2-F1 Execution-Readiness Closure

## Verdict

**GE05-E2-F1 is ready for a code-authorizing execution handoff.**

This closure does **not** itself authorize code changes. It authorizes deriving a narrow `../execution-handoff.md` for Claude Code or another frontier coding harness.

The implementation slice is ready because the missing oracle-evidence prerequisite from GE05-E1 has now been grounded by successful PCGen runtime output, and the Codex repo already has a small Rust crate, test substrate, TDD doctrine, and adjacent character-input model for a bounded fixture-schema slice.

## Route contract

| Field | Value |
|---|---|
| Current artifact | `artifacts/ge05-e2-f1-execution-readiness-closure-2026-06-20.md` |
| Current route | readiness closure |
| Run this artifact in Hermes | yes |
| Run this artifact in Claude Code | no |
| Create code from this artifact directly | no |
| May derive `execution-handoff.md` | yes |
| Future code-authorizing artifact | `../execution-handoff.md` |
| Future coding harness | Claude Code / frontier coding harness |

## Selected implementation slice

`GE05-E2-F1 — Golden-case fixture schema`

Implementation objective for the later handoff:

> Add the first Codex-side golden-case fixture schema surface that can represent the PF1 Core Rulebook Human Fighter level 1 case, link to old-system runtime evidence, preserve raw-output retention posture, expose reduced facts references, carry provisional-assumption metadata, and represent unresolved Codex/new-system output without pretending parity has passed.

## Evidence basis

### Source-STC requirements

`../technical-requirements.md` establishes the relevant requirements:

- TR-05-005 — golden-case fixture format
- TR-05-007 — PCGen output capture contract
- TR-05-008 — normalization boundary requirements
- TR-05-009 — comparison dimensions
- TR-05-010 — parity report schema
- TR-05-012 — known-gap and non-comparable output policy
- TR-05-013 — compatibility claim-tier promotion
- TR-05-015 — legal and fixture-retention constraints
- TR-05-018 — downstream routing rule

### Epic breakdown basis

`../epic-breakdown.md` defines GE05-E2-F1 acceptance:

- fixture schema includes case ID, source package, character inputs, old/new output references, compared dimensions, normalization declarations, diagnostics, known-gap links, and claim-tier target
- fixture schema can represent blocked or not-yet-grounded outputs without pretending they passed

### Existing documentary schema basis

`artifacts/golden-case-fixture-format.md` already defines the conceptual shape:

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

### Runtime oracle evidence basis

GE05-E1-F2 now has successful old-system runtime evidence:

- `artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`

Key facts from that receipt:

| Fact | Value |
|---|---|
| PCGen route | headless Gradle `run` batch export |
| campaign identity | `Core Rulebook` |
| runtime game mode | `Pathfinder_RPG` |
| export sheet | `code/testsuite/base-xml.ftl` |
| raw XML path | `/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml` |
| raw XML retention | local/generated only |
| raw XML SHA-256 | `3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1` |
| output status | produced successfully |

Reduced facts exist for race, class, level, scores, BAB, HP, AC, saves, attacks, feats, languages, equipment state, and skill points.

## Target runtime

| Field | Value |
|---|---|
| Target repo | `/home/ubuntu/workspace/repos/codex` |
| Language/toolchain | Rust, Cargo |
| Verification substrate | `cargo test` |
| Existing crate root | `src/lib.rs` |
| Existing adjacent model | `src/rules_core/character_input.rs` |
| Existing test style | integration tests under `tests/*.rs` with fixtures under `tests/fixtures/` |
| Repo conduct surface | `/home/ubuntu/workspace/repos/codex/AGENTS.md` |

## Live repo findings

Observed on 2026-06-20:

```text
current branch: ge04-e1-f1-character-input-record-shape
origin/develop: 2f32636e82c176a207f4117880585f9f2b0e56aa
HEAD: 24a80c6 feat: add rules-core character input record shape
```

`HEAD` is contained in `origin/develop`, but the local checkout remains on the old GE-04 feature branch.

Untracked local files exist:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
?? target/
```

`AGENTS.md` and `CLAUDE.md` are repo-root conduct/instruction surfaces for the coding harness. `target/` is build output. `Cargo.lock` is pre-existing untracked cargo output. These are not blockers to deriving an execution handoff, but the handoff must include branch hygiene and forbidden-write rules so Claude Code does not mistake this dirty checkout for the intended base state.

## Verification command grounded

The current test suite is runnable:

```bash
cargo test
```

Observed result:

```text
running 3 tests in tests/character_input_record.rs ... ok
running 2 tests in tests/pcc_entry_parse.rs ... ok
Doc-tests codex ... ok
```

Total observed integration tests:

```text
5 passed; 0 failed
```

## Execution-readiness gates

| Gate | Status | Evidence / ruling |
|---|---|---|
| Bounded implementation slice selected | pass | GE05-E2-F1 only |
| Target repo exists | pass | `/home/ubuntu/workspace/repos/codex` |
| Runtime instruction surface exists | pass | `AGENTS.md` present in current checkout; future handoff must require reading it |
| Toolchain/verification command grounded | pass | `cargo test` succeeds |
| Old-system oracle evidence exists | pass | GE05-E1-F2 attempt 3 receipt |
| Retention posture known | pass | raw XML local/generated only; receipt + reduced facts + SHA committed |
| Branch/worktree policy explicit | pass with mandatory condition | future handoff must branch from `origin/develop`, not continue implicitly from stale branch state |
| Allowed write scope can be named | pass | see below |
| TDD target can be named | pass | integration test for golden fixture schema first |
| Unresolved upstream pilot choices isolated | pass | provisional `.pcg` assumptions remain non-canonical and out of scope |
| Comparator/report scope excluded | pass | GE05-E3/GE05-E4 deferred |

## Required branch policy for derived handoff

The derived execution handoff must require one of these before code work begins:

### Preferred

Create or reset a feature branch from `origin/develop` in the existing checkout:

```bash
git fetch origin
git switch -C ge05-e2-f1-golden-fixture-schema origin/develop
```

The worker must preserve and read the local conduct files if present:

- `AGENTS.md`
- `CLAUDE.md`

### Acceptable alternative

Create a fresh worktree from `origin/develop`:

```bash
git fetch origin
git worktree add /home/ubuntu/workspace/worktrees/codex-ge05-e2-f1 -b ge05-e2-f1-golden-fixture-schema origin/develop
```

If using a fresh worktree, the execution handoff must explicitly provide or copy the repo conduct surface before launch, because the current `AGENTS.md` / `CLAUDE.md` files are untracked in the existing checkout.

## Allowed write scope for future execution handoff

The first GE-05 code handoff should allow only these repo paths:

```text
/home/ubuntu/workspace/repos/codex/src/lib.rs
/home/ubuntu/workspace/repos/codex/src/oracle_validation/mod.rs
/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs
/home/ubuntu/workspace/repos/codex/tests/golden_case_fixture_schema.rs
/home/ubuntu/workspace/repos/codex/tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
```

Optional only if the coding harness proves it is unavoidable:

```text
/home/ubuntu/workspace/repos/codex/Cargo.toml
/home/ubuntu/workspace/repos/codex/Cargo.lock
```

Default stance: **do not add dependencies for GE05-E2-F1.** Use native Rust structs and a small text-fixture parser consistent with the GE04 character-input slice unless the handoff explicitly authorizes dependency changes.

## Required reads for future execution handoff

The derived `execution-handoff.md` should require the coding harness to read exactly:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `../artifacts/golden-case-fixture-format.md`
3. `../artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md`
4. `../technical-requirements.md` sections TR-05-005, TR-05-007, TR-05-012, TR-05-013, TR-05-015
5. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
6. `/home/ubuntu/workspace/repos/codex/tests/character_input_record.rs`

Conditional reads:

- Read `../artifacts/known-gap-policy.md` if modeling known-gap references beyond opaque IDs.
- Read `../artifacts/parity-report-format.md` only if the implementation starts drifting into report shape; otherwise stop because GE05-E4 is out of scope.

## Expected implementation boundary

The later handoff should create a minimal Codex-side schema surface that can represent:

- stable case identity
- fixture version
- source package / campaign identity
- character input reference or embedded chosen-input metadata
- legacy oracle route metadata
- old-system raw-output reference and retention mode
- old-system raw-output hash
- reduced old-system facts reference or reduced facts structure
- Codex/new-system output reference as unresolved / absent
- comparison dimensions with `blocked`, `candidate`, or `not_yet_grounded` status
- normalization declarations as references/placeholders, not rules engine behavior
- known-gap references
- claim target and current claim status
- provisional assumption flags
- claim-blocking diagnostics for missing required fixture fields

## TDD obligations for future execution handoff

The coding harness must write the failing test first.

Recommended first test file:

```text
tests/golden_case_fixture_schema.rs
```

Recommended first failing behavior:

```text
loads_pf1_human_fighter_fixture_with_oracle_hash_and_provisional_assumptions
```

The test should assert that a fixture representing `pf1-crb-human-fighter-level1` can load and exposes at minimum:

- `case_id == "pf1-crb-human-fighter-level1"`
- `source_package.campaign == "Core Rulebook"`
- `source_package.game_mode == "Pathfinder_RPG"`
- legacy raw-output retention mode is `local_generated_only`
- legacy raw-output SHA-256 is `3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1`
- old-system evidence state is captured as runtime evidence, not static source truth
- Codex/new-system output state is unresolved or absent
- provisional assumptions include Human `+2 Strength`, no final equipment loadout, no skill allocation, and no additional feat-slot closure beyond `Power Attack`
- current claim status is not `OracleChecked`

Recommended second behavior:

```text
missing_legacy_oracle_hash_returns_claim_blocking_diagnostic
```

Recommended third behavior:

```text
fixture_can_represent_blocked_or_unresolved_codex_output_without_passing_parity
```

## Verification commands for future execution handoff

Specific RED/GREEN target:

```bash
cargo test --test golden_case_fixture_schema
```

Full regression command:

```bash
cargo test
```

The future handoff must require the worker to report:

- first failing test output
- final passing specific-test output
- final passing full-suite output
- `git status --short`
- file-granular scope audit

## Explicit non-goals

The future execution handoff must not authorize:

- comparator implementation
- normalization engine implementation
- parity report writer implementation
- PCGen command runner implementation
- committing raw PCGen XML
- rewriting GE-06 pilot truth
- treating provisional `.pcg` choices as canonical
- adding broad YAML/JSON framework dependencies without explicit justification
- modifying the PCGen repository
- modifying governance docs outside this GE-05 package and the Codex repo write scope

## Remaining risks

| Risk | Handling |
|---|---|
| Current Codex checkout is on stale GE-04 branch | future handoff must branch/reset from `origin/develop` |
| Repo conduct files are untracked | future handoff must preserve/read them or carry their instructions explicitly into a fresh worktree |
| Existing `Cargo.lock` is untracked | future handoff should avoid dependency changes unless unavoidable |
| Fixture text format could become throwaway | acceptable for GE05-E2-F1; schema concepts matter more than final serialization choice |
| GE-06 canonical pilot choices remain unresolved | fixture must label provisional assumptions and must not close GE-06 debt |

## Decision

Create a GE-05 `execution-handoff.md` next.

That handoff should be a **single-slice Claude Code brief** for GE05-E2-F1 only.

It should not attempt GE05-E2-F2, GE05-E3, GE05-E4, or any comparator/reporting implementation.

## Completion statement

GE05-E2-F1 passes execution-readiness gates **conditional on the derived execution handoff enforcing branch hygiene, TDD, the narrow write scope, and provisional-assumption boundaries**.

This is the correct harness switch point:

```text
Hermes readiness closure -> GE-05 execution-handoff.md -> Claude Code / frontier coding harness
```
