---
title: GE03-E1-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-03
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE03-E1-F1 — PCC entry-file parse shape
workflow_route: coding
readiness: codex-ready
handoff_created: true
review_date: 2026-06-19
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE03-E1-F1 Execution Readiness Closure

## Verdict
GE03-E1-F1 is the correct first bounded implementation slice for GE-03 and now has a derived **`ge03-e1-f1-execution-handoff-2026-06-19.md`** artifact.

This closure pass resolves the policy and environment gates needed to create the handoff. It still does **not** authorize code by itself. Code authority now lives in the derived `./ge03-e1-f1-execution-handoff-2026-06-19.md`.

## Selected bounded slice

```text
GE03-E1-F1 — PCC entry-file parse shape
```

Source in `epic-breakdown.md`:

- outcome: a bounded definition of how pilot PCC entry files become structured parse records
- acceptance signals:
  - entry-file identity is preserved
  - include relationships are represented
  - parse failures are diagnosable rather than silent

## Why this is the first slice

The GE-03 epic breakdown orders GE03-E1 before LST parsing, registry, handlers, provenance writer, report CLI, and fixture/parity work. GE03-E1-F1 is narrower than the whole PCC parser boundary and can be handed to a coding harness without forcing it to invent the broader importer.

It must not include:

- LST parser implementation
- token registry implementation
- semantic conversion handlers
- canonical object emission
- source-map writer beyond minimal parse/source identity fields
- conversion-report CLI
- parity harness work
- PCGen repository writes

## Grounded facts recovered by tools

| Fact | Result |
|---|---|
| Workspace date from runtime | `2026-06-19` |
| Target repo | `/home/ubuntu/workspace/repos/codex` |
| Git branch at audit time | `main` |
| Git remote | `https://github.com/electricm0nk/codex.git` |
| Recent commit | `ce0bbf8 Initial commit` |
| Repo status at audit time | `AGENTS.md` and `CLAUDE.md` are untracked in the implementation repo |
| Repo content | initial README/LICENSE plus repo instruction files; no Rust scaffold discovered |
| Repo instruction surface | `AGENTS.md` exists and requires an explicit execution handoff before implementation |
| Codex CLI | `/home/ubuntu/.hermes/node/bin/codex`, version `codex-cli 0.135.0` |
| Codex doctor | `17 ok · 1 idle · 1 notes · 0 warn · 0 fail`; auth configured; websocket connected |
| Rust toolchain | installed and verified in this runtime |
| Cargo | `/home/ubuntu/.cargo/bin/cargo`, `cargo 1.96.0 (30a34c682 2026-05-25)` |
| Rustc | `/home/ubuntu/.cargo/bin/rustc`, `rustc 1.96.0 (ac68faa20 2026-05-25)` |
| Rustfmt | `rustfmt 1.9.0-stable (ac68faa20c 2026-05-25)` |
| Clippy | `clippy 0.1.96 (ac68faa20c 2026-05-25)` |
| Pilot PCC root | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` |
| PCC root evidence | GE-01 inventory records it as pilot-critical; `core_rulebook.pcc` lines 42-96 enumerate include and object references |

## GE-01 evidence for the first slice

From `pilot-corpus-inventory.csv`:

- `core_rulebook.pcc` is the authoritative pilot campaign root.
- It includes `conversion_support.pcc` and `_core_essentials.pcc`.
- It references pilot-critical and adjacent LST files that later GE03 slices must parse or defer.

From `pilot-token-taxonomy.csv`:

- `PCC include directives` are critical.
- Their downstream owner is `GE-02 source package + GE-03 parser`.

From `conversion-matrix.csv`:

- PCC include directives map to `GE-02 source package manifest and include graph`.
- Provenance must preserve source file path plus include edge and source line for each include.
- Future validation must reconstruct the same package/include graph for the pilot corpus.

These facts were sufficient to derive a bounded execution handoff for GE03-E1-F1.

## Resolved gates

| Gate | Status | Resolution |
|---|---|---|
| Code-producing GE-03 intent | pass | Roadmap exit gate requires pilot source files to convert into canonical objects with diagnostics. |
| Source STC exists | pass | GE-03 source-STC bundle exists under `programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance/`. |
| Downstream slice chosen | pass | GE03-E1-F1 is selected as the first bounded slice. |
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex`. |
| Runtime instruction surface grounded | pass | Repo `AGENTS.md` exists and requires explicit handoff fields. |
| Branch/worktree policy | pass | Future handoff must create or use branch `ge03-e1-f1-pcc-entry-parser` from `main`; do not implement directly on `main`. |
| Allowed write scope | pass | Exact allowed paths are defined below. |
| Execution substrate | pass | Rust toolchain, rustfmt, clippy, Codex CLI, and Codex auth are verified in this runtime. |
| Verification commands | pass | Runnable commands are defined below; they depend on the handoff-created Rust scaffold. |
| Execution handoff created | pass | `./ge03-e1-f1-execution-handoff-2026-06-19.md` now exists and preserves the resolved gates from this closure. |
| First-slice provenance threshold | pass | Minimum threshold is source PCC path plus line number for include edges; token/span precision may be deferred if explicitly diagnosed. |
| Final schema/runtime decisions | excluded from slice | GE03-E1-F1 parses PCC entry/include structure only and must not settle GE-02 final schema or GE-04 runtime behavior. |

## Branch / worktree policy

The derived `ge03-e1-f1-execution-handoff-2026-06-19.md` artifact must use this policy:

```text
Create or use branch: ge03-e1-f1-pcc-entry-parser
Base: main
Implementation must not occur directly on main.
```

If the branch already exists when the handoff is executed, the coding harness must verify it is appropriate for GE03-E1-F1 before continuing. If the working tree contains unrelated changes, the harness must stop and report instead of overwriting or mixing scope.

A separate worktree is allowed only if the handoff executor chooses that as the safest way to isolate work; the branch name remains the same.

## Allowed write scope for the future handoff

The derived `./ge03-e1-f1-execution-handoff-2026-06-19.md` artifact may grant write authority only to:

```text
Cargo.toml
src/lib.rs
src/pcgen_import/mod.rs
src/pcgen_import/pcc.rs
tests/pcc_entry_parse.rs
tests/fixtures/pcc/core_rulebook_minimal.pcc
README.md                         # only if needed to document test commands or crate purpose
```

Forbidden writes:

```text
/home/ubuntu/workspace/repos/pcgen/**
programs/codex/**                 # unless the handoff explicitly says to write an upstream delta/no-change receipt after implementation
token registry implementation
LST parser implementation
semantic conversion handlers
canonical object emission
source-map writer beyond minimal parse/source identity fields
conversion-report CLI
parity harness
UI work
```

The repo-root `AGENTS.md` and `CLAUDE.md` are instruction surfaces. The coding handoff may read them, but must not modify them unless a separate governance decision grants that write.

## Verification commands for the future handoff

The coding harness must source the Rust environment before running Cargo commands:

```bash
. "$HOME/.cargo/env"
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Because the repo currently has no Rust scaffold, the first failing test must be created before implementation. The harness must report the actual failing-test output before making production code pass.

## First-slice provenance threshold

For GE03-E1-F1, the minimum acceptable provenance is:

- source PCC path / source identity for the parsed entry file
- line number for each parsed include edge
- include edge represented structurally as source PCC -> included target
- raw include directive or equivalent raw evidence preserved for review
- malformed or unsupported PCC lines surfaced as diagnostics with source path and line number

Token/span precision beyond line number is deferred for this slice, but the parser must not pretend stronger precision exists.

## Required handoff posture

The derived `./ge03-e1-f1-execution-handoff-2026-06-19.md` is an `implementation-ready` / `codex-ready` brief for only:

```text
GE03-E1-F1 — PCC entry-file parse shape
```

It should require TDD and implement the smallest parser slice that can:

- read a PCC entry file fixture or source sample
- preserve source file identity
- identify include directives relevant to the pilot root
- represent include edges structurally
- emit diagnosable parse errors rather than silently ignoring malformed lines
- avoid any LST parsing, token registry, semantic handler, canonical conversion, report CLI, UI, or parity claim

## Stop conditions for the future handoff

The coding harness must stop and report if:

- it cannot create or use branch `ge03-e1-f1-pcc-entry-parser`
- the working tree contains unrelated changes that would be mixed into the slice
- `cargo`, `rustc`, `rustfmt`, or `cargo clippy` are unavailable after sourcing `$HOME/.cargo/env`
- it cannot create the first failing test before implementation
- it needs to write outside the allowed write scope
- it needs final GE-02 schema, GE-04 runtime behavior, LST parsing, token registry, semantic conversion, report CLI, or parity logic to complete the slice

## Closure status

GE03-E1-F1 is selected, policy-grounded, environment-grounded, and has a derived `./ge03-e1-f1-execution-handoff-2026-06-19.md`. This artifact does not itself authorize code; the derived handoff is now the code-authorizing brief.
