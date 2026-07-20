---
title: SD-18 — Technical Requirements
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Technical Requirements

The bundle's hard preconditions. Each must hold before the bundle's first action lands.

## Preconditions for bundle startup

1. **Tranche-2-7 fully landed.** SD-17's slices A through E all merged to `tranche/2-7`, with the closure posture described in `programs/codex/requirements/tranche-2-7-pcgen-corpus-ingestion/README.md` §Acceptance. The `origin/tranche/2-7` branch must contain the canonical source-IR types (SD17-E) at `src/rules_core/source_content.rs`.

2. **`src/rules_core/source_content.rs` exists and exports** `SourcePackageContent`, `SourceContentRecord`, `SourceContentKind`, `SourceContentPayload`, `SourceRef`, `SourceContentDiagnostic`, and `SourceContentLoadResult`. These are SD17-E's deliverables. SD-18 cannot start §1.1 without them.

3. **`src/pcgen_import/` parsers and resolver exist and pass.** SD17-A (include-graph resolution), SD17-B-1 through SD17-B-6 (LST object-kind parsers), SD17-C (LST-to-canonical converter), SD17-D (record-aggregate relocation). All merged. `cargo test --locked` green for the SD-17 test files.

4. **`origin/tranche/3` exists.** The bundle's loop branches off `tranche/3`. The branch may contain prior tranche work or be empty; either is acceptable.

5. **`codex-tranche-3` board exists.** The kanban surface where loop-completed cards land. The board initializes at `~/.hermes/kanban/boards/codex-tranche-3/`.

6. **`/home/ubuntu/workspace/` contains the canonical scope doc** at `SD-18-core-rules-breadth-scope-draft.md`. The loop reads this file on every iteration. The path must remain stable across the bundle's lifetime.

7. **The progress doc co-exists with the scope doc.** Defaults to `SD-18-core-rules-breadth-progress.md` at the same path, created by the loop on first run.

## Preconditions for §1.1 (pre-loop gate)

1. All bundle-startup preconditions (above) hold.
2. `src/pilot_compute.rs` has at least one seam function pattern that a composer can target. Verified concrete entry points:
   - `src/rules_core/pilot_compute.rs:2168` — `pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt`
   - `src/rules_core/pilot_compute.rs:2186` — `pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation`
3. At least one race seam function is present in `src/rules_core/pilot_compute.rs`. Verified concrete entry points (7 race seams already implemented):
   - `explain_human_pilot_race_seam` at line 2403
   - `explain_dwarf_race_seam` at line 2529
   - `explain_elf_race_seam` at line 2654
   - `explain_gnome_race_seam` at line 2771
   - `explain_half_elf_race_seam` at line 2884
   - `explain_half_orc_race_seam` at line 3005
   - `explain_halfling_race_seam` at line 3126
4. `sd13-class-uplift-loop-prompt.md` is available as a reference (operator's binding for the matured SD-13 model); see `references/sd13-loop-model-excerpt.md` for the bundled excerpt.

The 7 race seams are proof the rules engine *can* consume race choice and produce derivations — exactly what §1.1 must extend to corpus-side records.

## Preconditions for §3.* (loop-routed coverage)

1. §1.1 has shipped (the only binary gate).
2. The progress doc's state shows `§1.1 done` and `§3.* open`.
3. The loop's per-iteration shell commands (cargo, git, gh) are all available on PATH.
4. Classic-token PAT (`~/.config/gh/.claude_gh_token`) is present and not expired; the loop uses it for any optional `gh` operations.

## What is NOT a precondition

- A full test suite passing — only the SD-17 test files and at least one `ge06_*` test must pass at bundle start. The loop's own tests accrete with each cycle.
- The operator's UI surfaces — the operator builds UI directly (per the 2026-07-10 assumption); SD-18 does not require UI to exist for closure.
- Convergence to a single PR — each loop cycle is its own branch and its own merge; the loop accumulates many small merges onto `tranche/3`, not one large one.

## Bundle startup checklist (operator)

When the operator initiates the bundle:

1. Verify all bundle-startup preconditions hold.
2. Mint the §1.1 card on `codex-tranche-3`. Assignee: tech-priest.
3. Once §1.1 ships, the loop is launchable.
4. Operator authors the loop instruction document at `/home/ubuntu/workspace/SD-18-core-rules-breadth-loop-instruction.md` (referenced from `artifacts/`).
5. Operator launches the loop with `/loop 60m /batch /goal <loop-instruction-doc>` per the matured SD-13 invocation pattern.
6. Operator monitors `## Open blockers` and the `codex-tranche-3` board for closure signs.

The bundle startup checklist is the operator's first read of the bundle after this STC is signed off.

## Cross-reference

- `README.md` (bundle overview)
- `acceptance-and-verification.md` (closure gates)
- `epic-breakdown.md` (per-criterion lane mapping)
- `decisions.md` §11 (§1.2 stays open)
