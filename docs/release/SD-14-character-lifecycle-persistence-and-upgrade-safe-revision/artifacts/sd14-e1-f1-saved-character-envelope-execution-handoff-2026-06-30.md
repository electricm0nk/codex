---
title: SD14-E1-F1 Execution Handoff — Saved-character envelope and local-store boundary
handoff_id: HANDOFF-CODEX-SD-14-E1-F1-CODING-2026-06-30
stc_id: STC-CODEX-SD-14
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: autonomous-launch-authorized
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/sd14-e1-f1-saved-character-envelope-execution-handoff-2026-06-30.md
source_stc: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md
source_epic_breakdown: programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/epic-breakdown.md
selected_slice: SD14-E1-F1 — Saved-character envelope and local-store boundary
run_in: Claude Code only
code_authority: true
authority_dependencies:
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/technical-requirements.md
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/technical-design.md
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/persisted-character-state-contract.md
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/character-lifecycle-operations-contract.md
  - programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/upgrade-migration-and-compatibility-contract.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: c2cea5c6baeb3ca34077b85331214c4b42a4809c
  recommended_branch: feat/sd14-e1-f1-saved-character-envelope
  pr_target: develop
allowed_write_scope:
  - src/lib.rs
  - src/saved_character/**
  - tests/sd14_saved_character_envelope.rs
  - tests/fixtures/sd14/**
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen/**
  - programs/codex/**
  - apps/desktop/**
  - Cargo.toml
  - Cargo.lock
  - src/rules_core/**
  - src/homebrew_authoring/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - tests/character_input_record.rs
  - tests/ge08_package_file_lifecycle.rs
  - AGENTS.md
  - CLAUDE.md
reviewed_at: 2026-06-30
---

# SD14-E1-F1 Execution Handoff — Saved-character envelope and local-store boundary

## Status
This is the stage-specific code-authorizing brief for SD14-E1-F1.

It carries `code_authority: true` for the bounded saved-character envelope slice only. It does not claim branch, PR, merge, or Claude execution evidence itself. That truth belongs to the downstream governed CODE lane and its receipt.

## Run in
Claude Code only.

Do not execute this implementation primarily through Hermes file editing. Hermes authored this handoff. Claude Code implements it. If Claude Code is unavailable, block the lane instead of substituting another coding harness.

## Core problem
Codex already has two adjacent truths, but not the persistence seam this program actually needs:
- `src/rules_core/character_input.rs` proves a bounded authoritative chosen-input substrate.
- `src/homebrew_authoring/package_store.rs` proves deterministic local file persistence for GE-08 authored packages.

What does not exist yet is a saved-character boundary that combines those ideas without widening into lifecycle breadth, autosave, migration execution, tester UX, or Tauri/Desktop work. If the first SD-14 code lane improvises that seam, later lifecycle and compatibility work will inherit counterfeit authority.

The first honest move is therefore narrower than “build save/load.” Establish the saved-character envelope and a deterministic local-store boundary that can round-trip one bounded authoritative character artifact while keeping identity, revision, and provenance explicit.

## Objective
Implement the smallest truthful Rust-first persistence slice for SD14-E1-F1.

The result must prove all of the following:
1. a bounded saved-character envelope exists as a typed Rust surface under `src/saved_character/**`
2. the envelope carries explicit persisted identity/revision/provenance fields rather than burying them in ad hoc test data
3. the authoritative saved payload is the existing bounded `CharacterInput` substrate or a direct equivalent projection over it, not a frozen derived-stat snapshot
4. a deterministic local-store boundary can save and reload one bounded saved-character artifact from disk
5. the on-disk fixture and roundtrip tests keep authoritative state distinguishable from any absent or subordinate derived/cache material
6. failure to load an incomplete or malformed saved-character artifact stays explicit instead of masquerading as a valid reopen

This slice stops at the saved-character envelope and local-store boundary. It does not claim broad lifecycle completeness.

## Why this route is authorized now
The source STC and epic breakdown fix the first executable slice as SD14-E1-F1. The live repo also fixes the narrowest truthful implementation seam:
- `CharacterInput` already captures bounded authoritative chosen state without derived computation.
- GE-08 `PackageStore` already demonstrates deterministic directory-backed persistence with headless Rust tests.
- live verification on 2026-06-30 confirmed `cargo test --test character_input_record --test ge08_package_file_lifecycle` passes from `/home/ubuntu/workspace/repos/codex`
- live verification on 2026-06-30 confirmed `npm run tauri:check` passes from `/home/ubuntu/workspace/repos/codex/apps/desktop`
- `git fetch origin --prune` on 2026-06-30 showed `origin/develop` advanced to `c2cea5c6baeb3ca34077b85331214c4b42a4809c` while the current local checkout remains `sd11-f10-update-action-surface` at `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`; the current local branch is not the truthful base for this slice
- there is currently no `src/saved_character/**` surface at all, which is exactly why this first lane must create it explicitly instead of smuggling persistence semantics into unrelated modules

## Target repo / workdir
```text
repo:    /home/ubuntu/workspace/repos/codex
workdir: /home/ubuntu/workspace/repos/codex
```

Current grounded repo facts:
- current local branch during handoff authoring: `sd11-f10-update-action-surface`
- current local `HEAD` during handoff authoring: `a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293`
- current accepted remote base for this lane after live fetch: `origin/develop` at `c2cea5c6baeb3ca34077b85331214c4b42a4809c`
- `origin/sd11-f10-update-action-surface` was deleted during live fetch, which is another reason not to branch from the current local checkout
- `AGENTS.md` and `CLAUDE.md` are read-only conduct surfaces for this lane

## Branch policy
Launch from a fresh `origin/develop`-based branch, not from the current local checkout branch.

Use this exact setup:
```bash
git fetch origin --prune
git switch develop --track origin/develop 2>/dev/null || git switch develop
git pull --ff-only origin develop
git switch -c feat/sd14-e1-f1-saved-character-envelope
```

If `feat/sd14-e1-f1-saved-character-envelope` already exists, use it only after confirming it still belongs exclusively to this slice.

Do not continue implementation on `sd11-f10-update-action-surface`.

## Required reads before coding
Read these first, in order:
1. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
2. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/sd14-e1-f1-saved-character-envelope-execution-handoff-2026-06-30.md`
4. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md`
5. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/technical-requirements.md`
6. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/technical-design.md`
7. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/epic-breakdown.md`
8. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/persisted-character-state-contract.md`
9. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/character-lifecycle-operations-contract.md`
10. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/upgrade-migration-and-compatibility-contract.md`
11. `/home/ubuntu/workspace/repos/codex/src/lib.rs`
12. `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs`
13. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs`
14. `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/mod.rs`
15. `/home/ubuntu/workspace/repos/codex/tests/character_input_record.rs`
16. `/home/ubuntu/workspace/repos/codex/tests/ge08_package_file_lifecycle.rs`
17. `/home/ubuntu/workspace/repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_minimal_character_input.txt`

Use these as bounded authority surfaces, not as permission to widen scope.

## Conditional reads
Read these only if the corresponding condition actually occurs:
1. `/home/ubuntu/workspace/repos/codex/Cargo.toml`
   - only if you think a new dependency or crate-topology change is required
   - if this file would need to change, stop and block the lane instead of widening scope
2. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/revision-autosave-and-recovery-policy.md`
   - only if your implementation starts drifting toward autosave/recovery semantics
   - if that happens, stop; that is later SD14-E3 scope, not this lane
3. `/home/ubuntu/workspace/programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md`
   - only if you think you need full diagnostic taxonomy beyond one bounded malformed-artifact failure test
   - if broader diagnostic routing becomes necessary, stop; that is later SD14-E5 scope
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/**`
   - only for the final read-only `npm run tauri:check` verification context
   - do not treat that as permission to edit desktop/Tauri files

## Exact allowed write scope
You may create or modify only these paths under `/home/ubuntu/workspace/repos/codex`:
```text
src/lib.rs
src/saved_character/**
tests/sd14_saved_character_envelope.rs
tests/fixtures/sd14/**
```

Write-scope interpretation:
- `src/lib.rs` may change only to expose the new saved-character module
- `src/saved_character/**` is the only implementation surface for the new envelope/store boundary
- `tests/sd14_saved_character_envelope.rs` is the only new test surface for this slice
- `tests/fixtures/sd14/**` is the only new checked-in fixture surface for this slice

Do not modify any other repo file.

## Forbidden write scope and explicit non-goals
This handoff does not authorize:
- edits under `src/rules_core/**` beyond consuming the existing public `CharacterInput` surface
- edits under `src/homebrew_authoring/**` beyond reading it as a persistence pattern reference
- edits under `apps/desktop/**`
- edits under `programs/codex/**`
- edits to `Cargo.toml`, `Cargo.lock`, `AGENTS.md`, or `CLAUDE.md`
- edits to `tests/character_input_record.rs` or `tests/ge08_package_file_lifecycle.rs`
- saved-character catalog/index breadth
- duplicate/archive/delete flows
- autosave, backup retention, interrupted-write recovery, or dirty-state UX
- compatibility classification engine, migration execution, read-only reopen posture, or rollback-aware state handling
- SD-11 tester issue-flow coupling
- SD-12 update/rollback logic
- Tauri/Desktop/UI work
- cloud sync, accounts, or multi-character library management
- freezing derived combat/stat outputs as authoritative saved truth

If you need any of the above to make the slice pass, stop and block the lane rather than widening it.

## Contract to implement
Implement a new bounded `saved_character` module that introduces a deterministic saved-character envelope over the existing `CharacterInput` authoritative payload.

### Required module shape
The preferred minimal shape is:
```text
src/saved_character/mod.rs
src/saved_character/local_store.rs
```

A slightly different split inside `src/saved_character/**` is acceptable only if it stays narrowly about this envelope/store boundary and does not require dependency or write-scope expansion.

Expose the module from:
```text
src/lib.rs
```

### Required data model
The implementation must surface a typed saved-character envelope that makes these fields explicit, using these names or equally clear Rust equivalents:
- `character_id`
- `revision_id`
- `revision_kind`
- `saved_at`
- `schema_version`
- `app_or_runtime_version`
- `content_or_rules_provenance`
- `latest_authoritative_revision_ref`
- `display_label`
- authoritative bounded character payload based on `CharacterInput`

Required posture:
- the authoritative payload must preserve user-authored chosen state
- derived/computed state must be absent or explicitly subordinate in this slice
- the envelope must be able to round-trip through disk without losing identity/revision/provenance truth

### Exact local-store boundary for this slice
Do not invent a catalog, database, or multi-character library.

Use one bounded deterministic saved-character bundle rooted at:
```text
tests/fixtures/sd14/pf1_human_fighter_level1_saved_character/
```

The bundle must use exactly these two checked-in files:
```text
tests/fixtures/sd14/pf1_human_fighter_level1_saved_character/envelope.txt
tests/fixtures/sd14/pf1_human_fighter_level1_saved_character/authoritative_character_input.txt
```

Required boundary rules:
- `envelope.txt` holds the persisted identity/revision/provenance metadata in a deterministic text format
- `authoritative_character_input.txt` holds the authoritative character-input payload in a deterministic text format
- reusing the existing `CharacterInput` fixture grammar for `authoritative_character_input.txt` is preferred
- the first slice may keep the text format simple and hand-rolled; do not add serde or format/framework dependencies
- save/load API must operate on a single saved-character root path, not a global store index

### Minimum API expectations
The public API may differ slightly in naming, but it must support the equivalent of:
```rust
SavedCharacterEnvelope
SavedCharacterRevisionKind
SavedCharacterStoreError
SavedCharacterStore::save(...)
SavedCharacterStore::load(...)
```

`save()` and `load()` must be headless Rust calls usable directly from integration tests.

### Required failure posture
At minimum, one bounded failure test must prove that malformed or incomplete saved-character artifacts do not masquerade as valid reopen success.

Acceptable narrow failure examples:
- missing `envelope.txt`
- missing `authoritative_character_input.txt`
- missing required envelope field such as `character_id` or `revision_id`

This does not require the full later SD14-E4/SD14-E5 compatibility engine. It only requires an honest refusal to pretend malformed persistence is valid.

## TDD requirement
TDD is mandatory.

Execution order:
1. create `tests/sd14_saved_character_envelope.rs` before production code changes
2. add the checked-in fixture bundle under `tests/fixtures/sd14/pf1_human_fighter_level1_saved_character/`
3. run the new SD-14 test target and capture the real RED failure
4. implement the smallest code required inside `src/saved_character/**` and `src/lib.rs`
5. rerun the SD-14 test target to green
6. rerun the regression verification commands below

Important RED rule:
- `cargo test --test sd14_saved_character_envelope` does not count as meaningful RED until the test file exists and is failing for the intended behavioral reason
- a missing test target error is not sufficient

### Minimum RED assertions
The failing tests should prove at least:
1. a bounded authoritative envelope can be saved and reloaded without losing `character_id`, `revision_id`, `revision_kind`, schema/runtime/provenance fields, or the authoritative `CharacterInput` payload
2. the checked-in fixture bundle at `tests/fixtures/sd14/pf1_human_fighter_level1_saved_character/` loads as the same bounded envelope shape the save path writes
3. malformed or incomplete saved-character artifacts fail honestly instead of yielding counterfeit reopen success

## Verification commands
Run these at minimum.

From `/home/ubuntu/workspace/repos/codex`:
```bash
cargo test --test sd14_saved_character_envelope
cargo test --test character_input_record --test ge08_package_file_lifecycle --test sd14_saved_character_envelope
```

From `/home/ubuntu/workspace/repos/codex/apps/desktop`:
```bash
npm run tauri:check
```

Verification interpretation:
- the first command proves the new slice itself
- the second command proves the new slice does not break the adjacent rules-core or GE-08 persistence baselines it relies on
- `tauri:check` proves the crate remains acceptable to the existing desktop shell consumer without authorizing desktop changes

## Stop conditions
Stop and block the lane instead of widening it if any of these occur:
- truthful implementation requires edits outside `src/lib.rs`, `src/saved_character/**`, `tests/sd14_saved_character_envelope.rs`, or `tests/fixtures/sd14/**`
- truthful implementation requires dependency changes in `Cargo.toml` or `Cargo.lock`
- the slice appears to require catalog/index breadth, duplicate/archive/delete flows, autosave/recovery, migration execution, or UI/Tauri changes
- a valid solution would require modifying `CharacterInput` or GE-08 package-store code rather than consuming those surfaces as read-only upstream truth
- the repo cannot be refreshed to a clean `origin/develop`-based execution branch
- `cargo test --test character_input_record --test ge08_package_file_lifecycle --test sd14_saved_character_envelope` or `npm run tauri:check` fails after the bounded change

If any stop condition lands, do not improvise. Block the card with the exact broader surface now required.

## Expected completion class
This lane is complete only at `pr-created` truth:
- fresh branch launched from `origin/develop`
- bounded changes confined to the allowed write scope
- branch pushed to `origin`
- normal PR opened against `develop`
- durable Claude receipt attached to the governed CODE card

This handoff does not authorize merge to `develop` or `main`.

## Required Claude receipt
Before the downstream CODE card completes, add a durable `claude-execution-receipt` comment that records:
- exact handoff path
- invocation mode
- repo/workdir
- branch and base SHA at launch
- durable Claude session/process handle when available, or `unknown`
- model identity when available, or `unknown`
- files changed
- RED failure summary
- verification commands run and their real results
- resulting commit and PR handle
- final completion class (`pr-created` or truthful blocker)

Without that receipt, this lane must not be described as Claude-executed.

## Merge authority boundary
This handoff authorizes only the bounded implementation slice above.

It does not authorize:
- merging the branch or PR
- landing code onto `develop` or `main`
- broadening into later SD-14 lifecycle, recovery, migration, diagnostics, or UI lanes
- any write outside the exact allowed scope

Stop at verified `pr-created` state and return control to Todd through the governed review surface.
