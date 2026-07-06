---
artifact_id: SD13-E4-F8-MERGE-RECEIPT
artifact_type: merge-receipt
epic: SD-13
slice: SD13-E4-F8
slice_title: SD-13 Sorcerer level-1 bloodline and spontaneous spell-slot slice
generated_by: tech-priest (Magos Ferrix-9)
generated_on: 2026-07-06
kanban_card_id: t_69465222
kanban_card_title: "SD-13 CODE: class:sorcerer -> BLOCKED/COMPUTED -> class.sorcerer.progression_and_spell_burden"
branch_base: origin/develop @ c78287cce76d3cce10fe814806558976fcfd70543
feature_branch: feat/sd13-class-sorcerer-bloodline-spontaneous-slice
branch_target: develop
canonical: true
---

# SD13-E4-F8 Merge Receipt — Sorcerer level-1 bloodline and spontaneous spell-slot slice

## TL;DR

The SD13-E4-F8 follow-up slice to SD13-E4-F7 has been authored on
`feat/sd13-class-sorcerer-bloodline-spontaneous-slice` (based at
`origin/develop @ c78287c`) and lifts the deterministic Human Sorcerer
level-1 matrix row from `Blocked` / `Computed` to `Partial` / `Computed`
by adding direct computed evidence for the level-1 bloodline burden (bloodline
selection + Arcane Bond level-1 power) and the spontaneous known-spell / slot
posture burden (spells known, slots per day, save DC for a 1st-level spell at
CHA 17). The integrated posture remains Blocked on the bounded remaining gap
(Sorcerer level-2+ progression, level-3+ bloodline arcana, school choice,
sorcerer metamagic, sorcery points, multiclass spell progression), which is
named explicitly as a single distinct diagnostic rather than the pre-F8
two-burden pair.

## Source-of-truth evidence surface

| Surface | Path | Bytes | Lines |
| --- | --- | --- | --- |
| F8 slice proof (new) | `tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs` | ~21k | ~470 |
| F7 baseline proof (post-F8 invariants) | `tests/sd13_sorcerer_level1_spell_baseline.rs` | ~13k | ~270 |
| Compute seam | `src/rules_core/pilot_compute.rs` | refactor in slice | refactor in slice |
| Matrix seed | `src/rules_core/support_state_matrix.rs` | row reclassified | row reclassified |
| Bounded fixture (unchanged) | `tests/fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt` | 1494 | 27 |

The matrix file
`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
is **not modified** by this slice; per the epic-breakdown handoff boundary,
row state updates flow through merge receipts only.

## Computed magnitudes proven on the deterministic seam

These are bounded PF1 Core Rulebook magnitudes sourced from
`cr_abilities_class.lst` rows (Sorcerer Level 1, Arcane bloodline level-1 power,
High Ability Scores sidebar) — **not oracle-checked parity**.

| Magnitude | Value | Derivation |
| --- | --- | --- |
| Spells known (Sorcerer 1, CHA 17) | **6** | 4 (Sorcerer table) + 2 (CHA 17 bonus bracket) |
| Spells per day, 1st-level (Sorcerer 1, CHA 17) | **5** | 3 (Sorcerer table) + 2 (CHA 17 bonus bracket) |
| Spell save DC, 1st-level (CHA 17) | **14** | 10 + 1 spell level + 3 CHA modifier |
| Arcane bloodline level-1 power | **Arcane Bond** | choice:sorcerer_bloodline -> bloodline:arcane (recognition, +0) |
| Bounded remaining gap | level-2+ progression + broader spell-support | single distinct diagnostic |

## Matrix row reclassification (in merge receipt body, NOT in matrix file)

Row `class.sorcerer.progression_and_spell_burden`:

- **Pre-F8**: `Blocked` / `Computed` / `RefreshableFromLiveProof`
  - grounding_ref: `tests/sd13_sorcerer_level1_spell_baseline.rs`
  - blocker note: bloodline burden + spontaneous burden both not implemented
- **Post-F8**: `Partial` / `Computed` / `RefreshableFromLiveProof`
  - grounding_ref: `tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs`
  - blocker note: only the level-2+ progression + broader spell-support gap remains

Row `class.paladin.hybrid_chassis_and_spell_burden`: unchanged (Blocked / Computed).
Row `class.ranger.hybrid_chassis_and_spell_burden`: unchanged (Blocked / Computed).
Row `class.bard.progression_and_spell_burden`: unchanged (Unverified / Observed).
Row `class.wizard.progression_and_spell_burden`: unchanged (Unverified / Observed).
All other 16 matrix rows: unchanged.

No row is promoted to `Supported` or `Lossy` by this slice.

## Verification

- `cargo test --lib --tests`: **128 passed; 0 failed; 0 ignored**.
- `cargo test --test sd13_sorcerer_bloodline_and_spontaneous_slice`: **12 / 12 passed** (direct slice proof).
- `cargo test --test sd13_sorcerer_level1_spell_baseline`: **10 / 10 passed** (post-F8 F7 invariants).
- `cargo test --test sd13_support_state_matrix`: **26 / 26 passed** (control plane).
- `cargo test --test sd13_hybrid_level1_chassis_baseline`: passed (no regression on Paladin / Ranger).
- `cargo clippy --tests`: clean (no new warnings).
- `cargo check`: clean.

## TDD discipline observed

1. Authored the failing slice test (`tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs`)
   **before** touching production code. Confirmed 6 of 12 tests failed for the
   intended reason (missing explanations, missing matrix reclassification).
2. Implemented the smallest change in `src/rules_core/pilot_compute.rs`
   (refactored `explain_sorcerer_level1_spell_baseline` to surface bloodline
   + spontaneous math as direct evidence with bounded CRB-cited magnitudes)
   and `src/rules_core/support_state_matrix.rs` (reclassified the Sorcerer
   row + refreshed blocker note + updated module-level doc comment).
3. Updated the pre-existing F7 baseline test
   (`tests/sd13_sorcerer_level1_spell_baseline.rs`) to pin only the F7
   invariants that SURVIVE the F8 follow-up (recognition, level-2 negative
   control, race seam, non-Sorcerer matrix preservation, Sorcerer row
   reclassified to Partial) — its pre-F8 two-burden diagnostic assertions and
   "no fabricated spell math" assertions are now obsolete because the F8
   slice computes the bounded level-1 bloodline + spontaneous math directly.
4. Confirmed GREEN: 128 / 128 tests pass with no regressions across all
   test binaries.

## Non-goals observed (per card body)

- No release publish from this card. The slice lands on `develop` via PR;
  the publish-lane invocation is gated by the parent GATE
  (t_5d57e115) which is itself waiting for all 21 SD-13 child slices to
  merge.
- No scope expansion beyond the bounded Human Sorcerer level-1 bloodline +
  spontaneous math. No level 2+ Sorcerer progression, no school choice,
  no sorcerer metamagic, no sorcery points, no multiclass spell progression.
- No matrix file edit. The matrix file is read-only for hand-edits; this
  slice updates the matrix seed carrier in
  `src/rules_core/support_state_matrix.rs`, which is the single source of
  truth for matrix truth and which the matrix file mirrors at release time
  via the merge receipt.

## Decisions captured

- **Decision 1**: The SD13-E4-F7 baseline test file
  (`tests/sd13_sorcerer_level1_spell_baseline.rs`) is preserved and updated
  rather than deleted. Its role shifts from "first truthful slice proof" to
  "F7 invariants that survive the F8 follow-up." The post-F8-specific
  assertions (bloodline + spontaneous explanations, new remaining-gap
  diagnostic, the reclassified matrix row state) live in the new
  `tests/sd13_sorcerer_bloodline_and_spontaneous_slice.rs` file.
- **Decision 2**: The slice recognizes only the Arcane bloodline's
  level-1 power (Arcane Bond) as bounded direct evidence in this fixture.
  A non-Arcane bloodline selection is honored with a generic recognition
  record (level-1 power name explicitly out of scope), preserving the
  bounded fixture's deterministic surface while staying honest about
  what this slice computes. No cross-bloodline-power math is fabricated.
- **Decision 3**: The two pre-F8 claim-blocking diagnostics
  (`class_feature.sorcerer.bloodline.unsupported` and
  `class_spell.sorcerer.spontaneous.unsupported`) are replaced by a single
  `class_sorcerer.sorcerer.level_2_plus_progression_unsupported` diagnostic
  that names only the remaining gap. The pre-F8 bloodline diagnostic is
  retained as a claim-blocker ONLY in the no-bloodline-selected branch,
  so the slice still refuses a Sorcerer with no bloodline choice.
- **Decision 4**: Spell save DC is computed for a 1st-level spell only
  (the highest level a Sorcerer level 1 can cast). The slice does not
  project save DCs for level 2+, since level 2+ is the bounded remaining
  gap and is explicitly out of scope for this slice.

## PR

- Branch: `feat/sd13-class-sorcerer-bloodline-spontaneous-slice`
- Base: `origin/develop @ c78287cce76d3cce10fe814806558976fcfd70543`
- Target: `develop`
- PR URL: https://github.com/electricm0nk/codex/pull/96
- Head SHA: `33f8b052d4367b801a1f1d56de1ccd10367d3af1`

Let it be recorded.