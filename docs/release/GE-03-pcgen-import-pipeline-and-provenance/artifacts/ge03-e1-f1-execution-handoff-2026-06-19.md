---
title: GE03-E1-F1 Execution Handoff — PCC Entry-File Parse Shape
stc_id: STC-CODEX-GE-03
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
source_readiness_closure: ./artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md
selected_slice: GE03-E1-F1 — PCC entry-file parse shape
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  base_branch: main
  execution_branch: ge03-e1-f1-pcc-entry-parser
  write_scope:
    - Cargo.toml
    - src/lib.rs
    - src/pcgen_import/mod.rs
    - src/pcgen_import/pcc.rs
    - tests/pcc_entry_parse.rs
    - tests/fixtures/pcc/core_rulebook_minimal.pcc
    - README.md
reviewed_at: 2026-06-19
---

# GE03-E1-F1 Execution Handoff — PCC Entry-File Parse Shape

## Deliverable Type
`implementation-ready`

## Execution Readiness
`codex-ready`

## Exact objective
Implement the smallest Rust parser slice that turns a PCC entry-file text input into a structured parse result preserving:

- source PCC file identity
- `PCC:` include directives as structural include edges
- one-based source line numbers for each include edge
- raw include directive evidence
- diagnosable malformed PCC include lines

This handoff is only for:

```text
GE03-E1-F1 — PCC entry-file parse shape
```

## Target repo / workdir

```text
/home/ubuntu/workspace/repos/codex
```

The repo is currently a near-empty Rust project surface with:

```text
README.md
LICENSE
AGENTS.md
CLAUDE.md
```

`AGENTS.md` is the repo-root conduct surface. Follow it.

## Branch / worktree policy

Before implementation, create or use:

```bash
git switch -c ge03-e1-f1-pcc-entry-parser
```

If the branch already exists, switch to it only after confirming it is appropriate for this slice.

Do **not** implement directly on `main`.

If the working tree contains unrelated changes beyond the existing untracked repo instruction surfaces (`AGENTS.md`, `CLAUDE.md`), stop and report instead of mixing scopes.

## Exact allowed write scope

You may create or modify only these paths in `/home/ubuntu/workspace/repos/codex`:

```text
Cargo.toml
src/lib.rs
src/pcgen_import/mod.rs
src/pcgen_import/pcc.rs
tests/pcc_entry_parse.rs
tests/fixtures/pcc/core_rulebook_minimal.pcc
README.md                         # only if needed to document crate/test commands or project purpose
```

Do not modify `AGENTS.md` or `CLAUDE.md`.

Do not write outside `/home/ubuntu/workspace/repos/codex`.

## Explicitly forbidden scope

Do not implement or modify:

```text
/home/ubuntu/workspace/repos/pcgen/**
programs/codex/**
token registry implementation
LST parser implementation
semantic conversion handlers
canonical object emission
source-map writer beyond minimal parse/source identity fields
conversion-report CLI
parity harness
UI work
GE-04 runtime behavior
GE-05 oracle/parity behavior
```

This slice may read upstream requirement artifacts and PCGen source files as evidence. It must not write them.

## Required reads before coding

Read these first:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/technical-design.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/references/ge03-importer-dependency-contract.md`

Use these as narrow evidence surfaces if needed, not as permission to broaden scope:

6. `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv`
7. `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv`
8. `/home/ubuntu/workspace/programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`
9. `/home/ubuntu/workspace/programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/provenance-source-map-specification.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md`

## Upstream evidence this slice must preserve

### GE-01 evidence

From `pilot-corpus-inventory.csv`:

- `core_rulebook.pcc` is the authoritative pilot campaign root.
- It is grounded at:

```text
/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc
```

- GE-01 records `core_rulebook.pcc` as `pilot-critical`.
- GE-01 records the include graph pressure:

```text
core_rulebook.pcc -> conversion_support.pcc
core_rulebook.pcc -> _core_essentials.pcc
core_rulebook.pcc -> cr_*.lst
core_rulebook.pcc -> race _race.pcc includes
```

From `pilot-token-taxonomy.csv`:

- token family: `PCC include directives`
- source kind: `PCC`
- pilot criticality: `critical`
- downstream owner: `GE-02 source package + GE-03 parser`
- notes: grounded in `core_rulebook.pcc` lines 42-43 and 90-96

From `conversion-matrix.csv`:

- legacy token family: `PCC include directives`
- target concept: `GE-02 source package manifest and include graph`
- support disposition: `deferred`
- lossiness class: `none-expected`
- provenance requirement: preserve source file path plus include edge and source line for each include
- validation requirement: future importer reconstructs the same package/include graph for the pilot corpus

### PCGen source evidence

The fixture for this slice should be a minimal PCC sample derived from these `core_rulebook.pcc` lines:

```text
42|PCC:@/homebrew/conversion_support/conversion_support.pcc
43|PCC:@/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc
90|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\dwarf\_race.pcc
91|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\elf\_race.pcc
92|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\gnome\_race.pcc
93|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\half_elf\_race.pcc
94|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\half_orc\_race.pcc
95|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\halfling\_race.pcc
96|PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\human\_race.pcc
```

You may create `tests/fixtures/pcc/core_rulebook_minimal.pcc` containing only the minimum comments/blanks/include lines required to prove parser behavior. Do not copy the whole legacy file unless there is a narrow test reason.

### GE-02 evidence

From `ge03-importer-dependency-contract.md`, the parser output must preserve enough source identity for GE-02 provenance fields:

- package / source identity when known
- include chain where known
- file path
- line/token span or explicit downgrade
- legacy construct

From `provenance-source-map-specification.md`:

- `pcc_path` and `include_chain` are required when available.
- `source_span` may be line, token span, or structured location.
- exact line span is preferred.
- file-only lineage is allowed only for package/include surfaces or explicit early-stage diagnostics.
- unknown lineage is a blocker for parity claims.

From `content-validation-and-diagnostics-specification.md`, diagnostics must be first-class records, not prose comments. Relevant diagnostic classes include:

- `unsupported_construct`
- `deferred_semantics`
- `unresolved_reference`
- `invalid_package_shape`
- `provenance_gap`

For this slice, use diagnostics only for PCC entry/include parse-shape problems. Do not build the broader diagnostics system.

## Required implementation shape

Create a small library crate if none exists.

Recommended package/crate name:

```toml
name = "codex"
edition = "2024"
```

Implement a PCC parser module under:

```text
src/pcgen_import/pcc.rs
```

Expose it from:

```text
src/lib.rs
src/pcgen_import/mod.rs
```

The public API may be named differently if the tests remain clear, but it must support these concepts:

```rust
parse_pcc_entry(source_path, input_text) -> structured result

PccEntryFile / equivalent:
  source_path
  includes
  diagnostics

PccIncludeEdge / equivalent:
  source_path
  line_number        # one-based line number in the parsed input
  raw_directive      # full raw PCC line or equivalent raw evidence
  target             # include target after PCC:

PccDiagnostic / equivalent:
  source_path
  line_number
  raw_line
  kind
  message
```

Minimum parsing behavior:

1. Ignore blank lines.
2. Ignore comment lines beginning with `#` after leading whitespace.
3. Recognize include directives beginning with `PCC:` after leading whitespace.
4. Preserve the include target exactly enough to distinguish both slash styles:
   - `@/homebrew/conversion_support/conversion_support.pcc`
   - `@\pathfinder\paizo\roleplaying_game\core_essentials\races\human\_race.pcc`
5. Preserve raw directive text.
6. Preserve one-based line number.
7. For malformed include directives such as `PCC:` with no target, emit a diagnostic instead of silently dropping it.
8. Do not parse or semantically interpret LST declarations such as `CLASS:cr_classes.lst`, `RACE:cr_races.lst`, or `SKILL:cr_skills.lst` in this slice.

## TDD requirement

TDD is mandatory.

Execution order:

1. Create the minimal Cargo crate scaffold only as needed for tests to run.
2. Create `tests/fixtures/pcc/core_rulebook_minimal.pcc`.
3. Create `tests/pcc_entry_parse.rs` with failing tests before implementing parser behavior.
4. Run the relevant failing test and capture the actual failing output.
5. Implement the smallest parser code needed to pass.
6. Run the full verification commands.

The first failing test should prove at least:

- include edge count from the fixture
- line number preservation
- raw directive preservation
- forward-slash include target preservation
- backslash include target preservation
- malformed `PCC:` line produces a diagnostic
- comments and blanks are ignored

## Acceptance criteria

The slice is complete when all of these are true:

1. `cargo test` passes.
2. `cargo fmt --check` passes.
3. `cargo clippy --all-targets -- -D warnings` passes.
4. The parser returns a structured result for a PCC entry file.
5. Include edges preserve source path, one-based line number, raw directive text, and target text.
6. Diagnostics are structured records, not only strings printed to stdout/stderr.
7. Malformed include directives produce diagnostics rather than disappearing.
8. LST parsing, token registry, semantic conversion, source-map writer, conversion report CLI, and parity claims remain unimplemented.
9. No files outside the allowed write scope are modified.
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

- branch `ge03-e1-f1-pcc-entry-parser` cannot be created or used
- unrelated working-tree changes would be mixed into this slice
- `cargo`, `rustc`, `rustfmt`, or `cargo clippy` are unavailable after sourcing `$HOME/.cargo/env`
- the first failing test cannot be created before implementation
- implementation requires writing outside the allowed scope
- implementation requires GE-02 final production schema syntax
- implementation requires formula/prerequisite evaluator decisions
- implementation requires GE-04 runtime engine behavior
- implementation requires LST parser, token registry, semantic conversion, conversion report, UI, or parity harness work

## Required final report from coding harness

When done, report:

- branch used
- files changed
- tests added
- first failing test command and actual failure summary
- final verification commands and actual results
- whether the branch is ready for Todd to open or merge the PR
- whether any upstream STC or GE-02/GE-03 artifact needs a delta/no-change review
- blockers or unresolved questions

## Merge authority boundary

This handoff does **not** authorize the coding harness to merge anything.

- Do not merge the branch or PR.
- Do not land code directly onto `develop` or `main`.
- Stop at a verified branch state and hand control back to Todd for the merge decision and merge action.

## Launch prompt for Codex CLI

Run from `/home/ubuntu/workspace/repos/codex` after verifying the handoff exists:

```bash
/home/ubuntu/.hermes/node/bin/codex exec 'Read /home/ubuntu/workspace/programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/execution-handoff.md and execute exactly that bounded task. Follow repo AGENTS.md. Use TDD: create the failing test first, report the failing output, implement the smallest Rust parser slice, then run cargo test, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and git status --short. Do not write outside the handoff allowed scope. Do not merge the branch or PR; stop at verified branch state and report back for Todd to handle merge.'
```

Use PTY mode if launched through Hermes tooling.
