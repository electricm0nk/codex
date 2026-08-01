---
title: GE04-E1-F1 Execution Handoff — Character Input Record Shape
stc_id: STC-CODEX-GE-04
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
source_readiness_closure: ./artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md
selected_slice: GE04-E1-F1 — Character input record shape
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  base_branch: develop
  execution_branch: ge04-e1-f1-character-input-record-shape
  write_scope:
    - src/lib.rs
    - src/rules_core/mod.rs
    - src/rules_core/character_input.rs
    - tests/character_input_record.rs
    - tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt
    - README.md
reviewed_at: 2026-06-20
---

# GE04-E1-F1 Execution Handoff — Character Input Record Shape

## Deliverable Type
`implementation-ready`

## Execution Readiness
`codex-ready`

## Exact objective
Implement the smallest Rust rules-core slice that defines a bounded **character input record shape** for the PF1 pilot path while preserving a strict separation between:

- chosen input state
- imported/canonical content references
- validation/diagnostic output
- future derived runtime state that this slice must **not** compute yet

This handoff is only for:

```text
GE04-E1-F1 — Character input record shape
```

The slice must establish a typed shape that can represent at minimum:

- source package identity
- race selection
- class and level selection
- ability score inputs
- feat selections
- skill choices or allocations needed by the pilot
- equipment selections and equipped/active state when relevant to later pilot computation
- selected choices needed by later engine slices
- selection provenance sufficient for later explanation output
- structured diagnostics for invalid character input

## Target repo / workdir

```text
/home/ubuntu/workspace/repos/codex
```

Current grounded repo facts:

- `origin/develop` already contains the merged GE03-E1-F1 parser work.
- `origin/develop` tree is still narrow: `Cargo.toml`, `README.md`, `src/lib.rs`, `src/pcgen_import/*`, and `tests/pcc_entry_parse.rs` plus its fixture.
- The current local shell is still on `ge03-e1-f1-pcc-entry-parser`, but `origin/develop` is the correct integration base for this slice.
- Local untracked `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, and `target/` are **not** tracked on `origin/develop` and must not be modified by this slice.

`AGENTS.md` is the repo-root conduct surface. Follow it.

## Branch / worktree policy
Before implementation:

1. fetch `origin`
2. start from a clean, current `develop`
3. create or use:

```bash
git fetch origin --prune
git switch -c develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c ge04-e1-f1-character-input-record-shape
```

If `ge04-e1-f1-character-input-record-shape` already exists, use it only after confirming it still belongs exclusively to this slice.

Do **not** implement directly on `main`.

Do **not** branch from `ge03-e1-f1-pcc-entry-parser` now that GE03-E1-F1 is merged to `develop`.

If the current shell cannot establish a clean `develop` without mixing unrelated changes, create a clean worktree or stop and report.

## Exact allowed write scope
You may create or modify only these paths in `/home/ubuntu/workspace/repos/codex`:

```text
src/lib.rs
src/rules_core/mod.rs
src/rules_core/character_input.rs
tests/character_input_record.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt
README.md                         # only if needed to document test commands or crate purpose
```

Do not modify `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, or `target/`.

Do not write outside `/home/ubuntu/workspace/repos/codex`.

## Explicitly forbidden scope
Do not implement or modify:

```text
/home/ubuntu/workspace/repos/pcgen/**
programs/codex/**
src/pcgen_import/**
effect evaluation core
formula evaluator
prerequisite evaluator
choice availability engine
derived stat calculator
explanation graph builder
rules-core CLI runner beyond the smallest test surface
GE-05 oracle/parity behavior
GE-06 integrated vertical-slice behavior
UI work
```

This slice may read upstream requirement artifacts as evidence. It must not write them.

## Required reads before coding
Read these first:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-requirements.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/technical-design.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/diagnostic-schema.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md`

Use these as narrow evidence surfaces, not as permission to broaden scope.

## Upstream evidence this slice must preserve
### GE-04 requirement pressure
From `technical-requirements.md`:

- **TR-04-003** requires a minimum character input shape covering package identity, race, class/level, ability scores, feats, skills, equipment/active state, selected choices, and provenance for explanation.
- **TR-04-014** requires deterministic pilot fixture requirements without fabricating final computed values.
- **TR-04-012** requires structured diagnostics that can distinguish invalid character input from other failure classes.

### Diagnostic posture
From `artifacts/diagnostic-schema.md`:

- this slice must preserve `invalid_character_input` as a first-class diagnostic class
- diagnostics must be structured records, not only strings printed to stdout/stderr
- claim-blocking posture matters when input shape is invalid

### Fixture posture
From `artifacts/pilot-golden-computation-fixture-requirements.md`:

- the pilot target remains PF1 Core Rulebook Human Fighter level 1
- fixture inputs must cover package identity, race/class/level, ability scores, feats, skills, equipment state, selected choices, provenance/source-map expectations, and expected diagnostics or known gaps
- this slice must **not** fabricate final computed outputs or parity claims

## Required implementation shape
Create a new rules-core module under:

```text
src/rules_core/character_input.rs
```

Expose it from:

```text
src/rules_core/mod.rs
src/lib.rs
```

The public API may be named differently if the tests remain clear, but it must support these concepts:

```rust
CharacterInput / equivalent:
  source_package_id
  race_id
  class_levels
  ability_scores
  selected_feats
  skill_state_or_allocations
  equipment_selections
  selected_choices
  selection_provenance

CharacterClassLevel / equivalent:
  class_id
  level

EquipmentSelection / equivalent:
  item_id
  equipped_or_active

CharacterInputDiagnostic / equivalent:
  class
  severity
  message
  subject_ref
  claim_blocking

CharacterInputLoadResult / equivalent:
  character_input_or_none
  diagnostics
```

### Required behavior
Minimum behavior for this slice:

1. Chosen state must be represented separately from future derived state.
2. No derived combat/math outputs may be computed in this slice.
3. The record shape must support the pilot-required input classes from TR-04-003.
4. Invalid input shape must surface as structured diagnostics, not panics or silent coercion.
5. The slice must be usable headlessly from Rust tests.
6. A minimal fixture input must be loadable headlessly either:
   - from `tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt`, or
   - from an equally narrow text fixture approach that stays inside the allowed write scope and does not require broad serialization/framework work.
7. If a fixture loader is introduced, keep it pilot-local and minimal. Do **not** introduce a general rules import, general config system, or broad serialization framework unless strictly necessary.

### Minimum invalid-input diagnostics
At minimum, tests must prove structured diagnostics for one or more shape failures such as:

- missing source package identity
- missing race selection
- empty class/level selection
- missing required ability score input

The exact shape-failure set may be slightly different if the tests remain narrow and the diagnostics are explicit.

## TDD requirement
TDD is mandatory.

Execution order:

1. create `tests/character_input_record.rs` with failing tests before implementing production code
2. if needed, create the minimal fixture text file under `tests/fixtures/rules_core/`
3. run the relevant failing test and capture the real failure output
4. implement the smallest rules-core code needed to pass
5. run the full verification commands

The first failing tests should prove at least:

- chosen state is represented separately from derived state
- the record shape covers package/race/class-level/ability-score input at minimum
- invalid input produces structured diagnostics
- the slice is usable headlessly from tests
- if a fixture file is used, the fixture loads without UI

## Acceptance criteria
The slice is complete when all of these are true:

1. `cargo test` passes.
2. `cargo fmt --check` passes.
3. `cargo clippy --all-targets -- -D warnings` passes.
4. `src/rules_core/character_input.rs` defines a bounded character-input record shape.
5. chosen input state is separated from future derived runtime state.
6. structured invalid-input diagnostics exist.
7. the tests prove headless use of the slice.
8. no effects, formulas, prerequisites, choices, explanations, derived-value computation, oracle comparison, or UI behavior are implemented.
9. no files outside the allowed write scope are modified.
10. `/home/ubuntu/workspace/repos/pcgen` is not modified.

## Verification commands
Before running Cargo commands:

```bash
. "$HOME/.cargo/env"
```

Then run:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Also report:

```bash
git status --short
```

## Stop conditions
Stop and report without implementing if any of these occur:

- a clean `develop` checkout cannot be established
- unrelated working-tree changes would be mixed into this slice
- `cargo`, `rustc`, `rustfmt`, or `cargo clippy` are unavailable after sourcing `$HOME/.cargo/env`
- the first failing test cannot be created before production code
- implementation requires writing outside the allowed scope
- implementation requires modifying `src/pcgen_import/**`
- implementation requires effect evaluation, formula/prerequisite evaluation, choice availability, explanation graph, derived-value computation, CLI surface expansion, UI work, or GE-05/GE-06 behavior
- implementation requires general serialization/import infrastructure instead of a narrow pilot-local record-shape proof

## Required final report from coding harness
When done, report:

- branch used
- files changed
- tests added
- first failing test command and actual failure summary
- final verification commands and actual results
- whether the branch is ready for Todd to open or merge the PR
- whether any upstream STC artifact needs a delta/no-change review
- blockers or unresolved questions

## Merge authority boundary
This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the merge decision and merge action.

## Launch prompt for Codex CLI
Run from `/home/ubuntu/workspace/repos/codex` after verifying the handoff exists:

```bash
/home/ubuntu/.hermes/node/bin/codex exec 'Read /home/ubuntu/workspace/programs/codex/requirements/GE-04-rules-engine-and-explainability-core/execution-handoff.md and execute exactly that bounded task. Follow repo AGENTS.md. Use TDD: create the failing test first, report the failing output, implement the smallest Rust character-input record slice, then run cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and git status --short. Do not write outside the handoff allowed scope. Do not merge the branch or PR; stop at verified branch state and report back for Todd to handle merge.'
```

Use PTY mode if launched through Hermes tooling.
