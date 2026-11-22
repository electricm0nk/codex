---
title: GE04-E1-F1 Execution Readiness Closure
artifact_type: execution-readiness-closure
stc_id: STC-CODEX-GE-04
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE04-E1-F1 — Character input record shape
workflow_route: coding
readiness: codex-ready
handoff_created: true
created_handoff: ./ge04-e1-f1-execution-handoff-2026-06-20.md
review_date: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE04-E1-F1 Execution Readiness Closure

## Verdict
GE04-E1-F1 is the correct first bounded implementation slice for GE-04, and it is now **codex-ready**.

The code-authorizing artifact is:

```text
./ge04-e1-f1-execution-handoff-2026-06-20.md
```

This closure records why that handoff is now justified. The closure itself remains non-code-authorizing; code authority lives only in the derived execution handoff.

## Selected bounded slice

```text
GE04-E1-F1 — Character input record shape
```

Source in `epic-breakdown.md`:

- outcome: a bounded record shape for package identity, race/class/level, ability scores, feats, skills, equipment, and selected choices
- acceptance signals:
  - chosen state is separated from derived state
  - fixture inputs can be loaded headlessly
  - invalid character input produces diagnostics

## Why this is the first slice
The GE-04 epic breakdown orders GE04-E1 before effect evaluation, formula/prerequisite evaluation, choice availability, diagnostics, explanation graph construction, and CLI/test entry points.

GE04-E1-F1 is narrower than the whole rules engine and can be implemented without selecting the final expression evaluator, implementing effect activation, computing derived values, or claiming PCGen parity.

It must not include:

- effect evaluation core
- formula or prerequisite evaluator implementation
- choice availability engine
- explanation graph builder
- rules-core CLI runner beyond the smallest test surface
- GE-05 oracle comparison
- GE-06 full integrated pilot vertical slice
- UI work

## Grounded facts recovered by tools

| Fact | Result |
|---|---|
| Workspace date from runtime | `2026-06-20` |
| Target repo | `/home/ubuntu/workspace/repos/codex` |
| Correct integration base | `origin/develop` |
| `origin/develop` head | `611decb Merge pull request #1 from electricm0nk/ge03-e1-f1-pcc-entry-parser` |
| `origin/main` head | `47a5b41 Merge pull request #2 from electricm0nk/copilot/main-only-accepts-merge-from-develop` |
| GE03 parser branch contained by develop | yes |
| Tree diff from `ge03-e1-f1-pcc-entry-parser` to `origin/develop` | empty |
| Repo instruction surface | `/home/ubuntu/workspace/repos/codex/AGENTS.md` exists and requires a bounded execution handoff before implementation |
| `origin/develop` tracked tree | `Cargo.toml`, `README.md`, `src/lib.rs`, `src/pcgen_import/*`, `tests/pcc_entry_parse.rs`, fixture |
| Local untracked residue | `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, `target/` |
| Untracked residue tracked on `origin/develop` | no |
| Rust toolchain | `cargo 1.96.0`, `rustc 1.96.0`, `rustfmt 1.9.0-stable`, `clippy 0.1.96` |
| Runtime verification | `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` all passed in the target runtime |

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Source STC exists | pass | GE-04 source-STC bundle exists under `programs/codex/requirements/GE-04-rules-engine-and-explainability-core/`. |
| Code-producing GE-04 intent | pass | GE-04 eventually requires rules-core implementation evidence: headless character input, computation, diagnostics, and explanations. |
| Downstream slice selected | pass | GE04-E1-F1 is selected as the first candidate implementation slice. |
| Slice narrower than spec domain | pass | Character input record shape excludes effect evaluation, formula/prerequisite evaluation, choice availability, explanation graph, CLI runner, oracle comparison, and UI work. |
| Target repo/workdir grounded | pass | `/home/ubuntu/workspace/repos/codex`. |
| Branch/worktree policy explicit | pass | Feature branches target `develop`; this slice branches from clean current `develop` into `ge04-e1-f1-character-input-record-shape`. |
| Allowed write scope explicit | pass | Exact repo paths are declared in `./ge04-e1-f1-execution-handoff-2026-06-20.md`. |
| Runtime instruction surface grounded | pass | Repo `AGENTS.md` exists and mandates explicit handoff fields and TDD. |
| Execution substrate grounded | pass | Rust/Cargo/rustfmt/clippy are available in the target runtime. |
| Verification commands runnable | pass | `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` were executed successfully in the target runtime. |
| Unresolved questions quarantined | pass | Final evaluator choice, effect evaluation, prerequisite logic, choice availability, explanation graph, parity, and UI work remain explicit non-goals. |
| Non-goals explicit | pass | The derived handoff forbids GE-04 broad-engine work and any GE-05/GE-06/UI expansion. |

## Launch cautions
These are not blockers to deriving the handoff, but they are mandatory discipline for the coding run:

- the current local shell is still on `ge03-e1-f1-pcc-entry-parser`
- local untracked `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, and `target/` must not be modified or mixed into the GE-04 slice
- the coding run must establish a clean `develop` checkout or a clean worktree before implementation begins

## Granted execution surface
The derived handoff grants only this repo surface:

```text
src/lib.rs
src/rules_core/mod.rs
src/rules_core/character_input.rs
tests/character_input_record.rs
tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt
README.md                         # only if needed to document test commands or crate purpose
```

Everything else remains out of scope.

## Verification evidence from this pass
The target runtime successfully ran:

```bash
. "$HOME/.cargo/env"
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
git status --short
```

Observed status output:

```text
?? AGENTS.md
?? CLAUDE.md
?? Cargo.lock
?? target/
```

These residue paths remain outside the granted write surface.

## Result
A code-authorizing handoff now exists at:

```text
./ge04-e1-f1-execution-handoff-2026-06-20.md
```

Future coding runs for GE04-E1-F1 must use that handoff, follow repo `AGENTS.md`, and enforce TDD.

## Stop condition for the coding run
Stop the implementation run if any of the following become true:

1. a clean `develop` checkout cannot be established,
2. unrelated working-tree changes would be mixed into the slice,
3. the implementation needs to write outside the granted scope,
4. the implementation requires effect evaluation, formulas, prerequisites, choice availability, explanation graphs, parity behavior, or UI work.

## Closure status
GE04-E1-F1 is selected, grounded, and **codex-ready**. The derived `./ge04-e1-f1-execution-handoff-2026-06-20.md` artifact is now the correct execution surface.
