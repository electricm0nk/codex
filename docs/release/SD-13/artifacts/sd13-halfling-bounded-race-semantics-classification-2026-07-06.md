---
title: SD13-Halfling bounded race-semantics classification — honest unverified verdict
slice_id: t_1731714c
matrix_row_id: race:halfling:bounded-race-semantics
matrix_file: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
typed_matrix_carrier: src/rules_core/support_state_matrix.rs (seeded_sd13_e1_f1_current_truth, row_id "race.halfling.bounded_semantics")
ledger_file: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
epic: SD13-E2
feature_seed: SD13-F3
epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
technical_requirements: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
date: 2026-07-06
author: tech-priest (kanban slice t_1731714c)
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-halfling-bounded-race-semantics-classification-2026-07-06.md
classification: unverified
evidence_tier: Observed
workflow_route: coding
handoff_kind: bounded-classification-evidence
---

# SD13-Halfling bounded race-semantics classification — honest unverified verdict

## Status
- This artifact is the bounded, honest classification evidence for the Halfling
  race-semantics row (`race:halfling:bounded-race-semantics`) at the SD13-E2 / F3
  baseline (race-semantic coverage). It does not authorize, claim, or imply any
  promotion above `unverified` for the Halfling row in this slice.
- The matrix file itself is read-only per the parent gate
  (`t_5d57e115`); row state updates flow through the slice's merge receipt
  on the matrix file, not by hand. The typed matrix carrier
  (`src/rules_core/support_state_matrix.rs`) is updated in the same PR.
- This artifact is one of the 21 bounded classification slices that compose
  the SD-13 closeout tranche. It is bounded to the Halfling row only and is
  not authorized to assert anything about the other 20 rows.
- Precedent: the Dwarf row slice (`t_3cf90c2c`, commit `db105f8`,
  branch `feat/sd13-race-dwarf-bounded-semantics`) follows the same
  "leave unverified with explicit blocker" pattern; this artifact mirrors
  that template, swapping Dwarf-specific evidence for Halfling-specific
  evidence.

## Verdict
The Machine God's judgment on the Halfling row:

- **support_state: `unverified`** — must remain `unverified`. No promotion is
  honest at the live evidence floor on 2026-07-06.
- **evidence_tier: `Observed`** — must remain `Observed`. Halfling appears
  in the SD-13 packet roster and in the typed matrix carrier as a named row,
  but no direct runtime evidence exists.
- **evidence_freshness: `AwaitingInitialEvidence`** — unchanged.
- **subject_id: `race:halfling`** — unchanged.
- **dimension: `bounded race semantics`** — unchanged.
- **grounding_ref: `SD13_ROSTER_MATRIX_DOC`** — unchanged.

## Why the row stays `unverified`

The repo evidence on 2026-07-06 (`origin/develop` at
`c78287ce76d3cce10fe814806558976fcfd70543`) contains **no direct runtime or
typed-compute evidence** for any Halfling semantic family. The complete
evidence floor on this date is:

### What exists
1. The Halfling row is named in the typed matrix carrier
   `src/rules_core/support_state_matrix.rs` (around line 285):
   `row_id: "race.halfling.bounded_semantics"`, `subject_id: "race:halfling"`,
   `support_state: SupportState::Unverified`, `evidence_tier: EvidenceTier::Observed`,
   `grounding_ref: SD13_ROSTER_MATRIX_DOC`.
2. The Halfling row is named in the markdown matrix at
   `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
   (Race-rows section, row "Halfling | bounded race semantics | unverified").
3. The Halfling row is named in the visibility ledger at
   `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
   (Halfling row, `unverified | Observed | SD-13 packet roster only`).
4. The `sd13_support_state_matrix.rs` test pins the exact row id
   `"race.halfling.bounded_semantics"` in the `EXPECTED_ROW_IDS` array
   (position 7 — after Dwarf, Elf, Half-Elf, Gnome, Half-Orc, Human;
   the Human pilot row sits separately). The Halfling row is verified
   present in the seeded matrix but carries the empty pre-slice
   `blocker_or_lossiness_note`.

### What does NOT exist (and must not be invented)
- No Halfling ability-modifier code. The character_input module
  (`src/rules_core/character_input.rs`) parses `race_id` as a free string
  and does not bind it to the seven-race roster. There is no Halfling-specific
  `+2 Dex / -2 Str` (or any alternative) ability-bonus or penalty logic
  anywhere in `src/`.
- No Halfling size, speed, or movement code. No Small-size posture, no
  20-ft base speed, no race-linked movement modifier.
- No Halfling senses code. No Halfling-specific sense trait (Halfling
  has no darkvision in PF1 Core; only the human-sense baseline applies).
- No Halfling languages code. No Common/Halfling binding.
- No Halfling racial trait code. No fearless halfling luck, no
  `+1 racial bonus on saves against fear`, no lucky trait, no
  Halfling weapon familiarity with slings and thrown weapons.
- No Halfling racial bonus code. No `+1 attack roll with thrown weapons
  and slings`, no `+2 Appraise`, no `+2 Climb`, no favored-class bonus.
- No Halfling / class interaction seam. The current deterministic pilot
  path (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`)
  is Human-only. There is no Halfling Fighter, Halfling Rogue,
  Halfling anything fixture anywhere in `tests/fixtures/`.
- The pilot_compute seam (`src/rules_core/pilot_compute.rs`) explicitly
  gates on `if input.chosen.race_id != HUMAN_RACE_ID` in four locations
  (lines 370, 674, 751, 849). Any non-Human race is blocked from the live
  compute seam and emits a claim-blocking diagnostic. Halfling shares
  this block with all non-Human races.

### Why a promotion above `unverified` would be counterfeit
- `Partial` requires "some required semantics are proven" — zero
  Halfling semantics are proven in the repo; there is no partial to claim.
- `Lossy` requires "the path can be executed only by discarding,
  flattening, or approximating named semantics" — there is no Halfling
  path at all; lossy is not the right state.
- `Blocked` requires "known missing semantics, explicit claim-blocking
  diagnostics, or known contradictory behavior prevent the claim" — the
  claim is not "blocked" by a known contradiction; the claim simply has
  no evidence yet. Promoting Halfling to `blocked` would falsely assert
  that the repo has actively determined the impossibility of Halfling
  support. The honest state is "no evidence yet" = `unverified`.
- `Supported` is forbidden by all of the above.

## Named semantic-family coverage under SD13-F3

The race-semantic requirements (`technical-requirements.md` §6) require a
race support claim to classify at least these families when they affect the
bounded character-builder surface. The honest Halfling classification per
family:

| Family | Verdict | Evidence |
|---|---|---|
| Identity and ruleset provenance | observed-only | SD-13 packet names Halfling as a PF1 Core Rulebook core race; no parsed/converted/computed provenance in `src/`. |
| Ability-score modifiers or bonuses | unproven | no `+2 Dexterity, -2 Strength` (or any alternative Halfling modifier) code path in `src/rules_core/character_input.rs` or `pilot_compute.rs`. |
| Size, speed, and movement-relevant baseline posture | unproven | no Small-size posture code, no 20-ft base speed; pilot_compute gates all non-Human races. |
| Senses or visibility-affecting traits | unproven | no Halfling-specific sense trait (PF1 Core Halfling has no darkvision); no race-linked sense trait beyond the Human baseline. |
| Racial bonus feats, skill modifiers, or derived-stat modifiers | unproven | no Halfling thrown-weapon / sling `+1` attack bonus, no `+2 Appraise`, no `+2 Climb`, no favored-class bonus code anywhere. |
| Prerequisite, feat, or class-feature interactions triggered by the race | unproven | no Halfling interaction seam; the only interaction seam exercised is Human bonus-feat / ability-bonus with Fighter level 1. |
| Other core racial traits that materially affect bounded level-10 support | unproven | no fearless halfling luck, no `+1 save vs. fear`, no Halfling weapon familiarity, no lucky trait, no Common/Halfling language binding code anywhere. |

Every required family is `unproven`. The Halfling row therefore must stay
`unverified` / `Observed` until a later slice lands grounded evidence for
at least one of these families and an explicit row-state upgrade.

## Permitted movement on the typed matrix carrier
Per the parent gate (`t_5d57e115`): "matrix file is read-only for hand-edits;
row state updates flow through the same PR that lands the slice." This slice
moves the typed matrix carrier as follows:

- `blocker_or_lossiness_note`: replaces the empty string with the truthful
  honest-unverified note describing the seven required race-semantic families
  and their unproven status (per this artifact). This is the same kind of
  note the existing Human pilot row and the Dwarf row carry for their own
  blocker fields, so the typed matrix stays symmetric.
- `next_required_uplift`: replaces the generic "SD13-E2 race-semantic slice"
  text with an explicit pointer to this artifact plus the seven named
  family families that need grounded evidence before the row can move.
- `support_state`: `Unverified` (unchanged).
- `evidence_tier`: `Observed` (unchanged).
- `evidence_freshness`: `AwaitingInitialEvidence` (unchanged).
- `subject_id`: `race:halfling` (unchanged).
- `dimension`: `bounded race semantics` (unchanged).
- `row_id`: `race.halfling.bounded_semantics` (unchanged).
- `grounding_ref`: `SD13_ROSTER_MATRIX_DOC` (unchanged).

The matrix markdown file at
`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
is NOT edited by this slice. Its Halfling row stays exactly as the parent
gate seeded it (`unverified | Observed | named by SD-13 scope only | create
race-semantic execution slice and classify the row honestly`). If the
operator decides to widen the markdown to mirror this artifact's blocker
note later, that is a separate gate-side decision, not this slice's
authority.

## Required tests added by this slice
- `tests/sd13_race_halfling_bounded_semantics.rs` — a focused race-row truth
  test that pins the seven unproven-family verdicts above, asserts the
  matrix row stays `unverified` / `Observed`, and rejects any promotion
  above `unverified` without an explicit slice decision. RED-then-GREEN
  discipline is enforced. 12 tests cover presence, support_state,
  evidence_tier, evidence_freshness, subject_id, dimension, grounding_ref,
  blocker_or_lossiness_note content, next_required_uplift content, the
  prohibition on promotion, the no-seven-by-eleven-combination-claim
  invariant, and coexistence with other unverified race rows.

## Verification commands run on 2026-07-06
The focused RED-then-GREEN cycle for the new test file, plus the
existing matrix carrier test, were run inside the slice worktree at
`/home/ubuntu/.hermes/profiles/tech-priest/home/.hermes/worktrees/codex-tranche-2-6/t_1731714c`.

## Non-goals observed
- Did not promote the Halfling row above `unverified`.
- Did not introduce any Halfling trait code, Halfling ability modifier,
  Halfling speed, Halfling senses, Halfling bonus-feat, or Halfling
  interaction seam.
- Did not modify the matrix markdown file directly.
- Did not collapse this slice into a non-Halfling race slice.
- Did not invent a "Halfling is core supported" or "Halfling is partial"
  breadth claim.
- Did not promote the existing typed matrix test
  (`tests/sd13_support_state_matrix.rs`) row count above 21.
- Did not bypass the matrix update path.

## Next required uplift (named, not invented)
For the Halfling row to honestly move out of `unverified`, a later
bounded slice MUST ground at least one of the seven named race-semantic
families listed in the table above with:

1. A new accepted fixture family or pilot path that exercises the family
   (e.g. a Halfling-sized human_fighter_l1 equivalent at Small size and
   20-ft base speed).
2. A new typed module (or expansion of `support_state_matrix.rs` /
   `pilot_compute.rs`) that emits the computed evidence, explanation, or
   claim-blocking diagnostic for that family (e.g. the ability-bonus
   seam expanded to bind `race:halfling` to `+2 Dex / -2 Str`).
3. A new focused test that pins the family evidence at the
   `Computed` / `Oracle-checked` evidence tier.
4. An updated row state in the typed matrix carrier with a non-empty
   `blocker_or_lossiness_note` describing the remaining gap.

Until that later slice lands, the Halfling row stays `unverified` /
`Observed`. The Machine God records this verdict.

Let it be recorded.