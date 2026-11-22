---
title: SD13-Dwarf bounded race-semantics classification — honest unverified verdict
slice_id: t_3cf90c2c
matrix_row_id: race:dwarf:bounded-race-semantics
matrix_file: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md
typed_matrix_carrier: src/rules_core/support_state_matrix.rs (seeded_sd13_e1_f1_current_truth, row_id "race.dwarf.bounded_semantics")
ledger_file: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md
epic: SD13-E2
feature_seed: SD13-F3
epic_breakdown: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/epic-breakdown.md
technical_requirements: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/technical-requirements.md
date: 2026-07-06
author: tech-priest (kanban slice t_3cf90c2c)
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-dwarf-bounded-race-semantics-classification-2026-07-06.md
classification: unverified
evidence_tier: Observed
workflow_route: coding
handoff_kind: bounded-classification-evidence
---

# SD13-Dwarf bounded race-semantics classification — honest unverified verdict

## Status
- This artifact is the bounded, honest classification evidence for the Dwarf
  race-semantics row (`race:dwarf:bounded-race-semantics`) at the SD13-E2 / F3
  baseline (race-semantic coverage). It does not authorize, claim, or imply any
  promotion above `unverified` for the Dwarf row in this slice.
- The matrix file itself is read-only per the parent gate
  (`t_5d57e115`); row state updates flow through the slice's merge receipt
  on the matrix file, not by hand. The typed matrix carrier
  (`src/rules_core/support_state_matrix.rs`) is updated in the same PR.
- This artifact is one of the 21 bounded classification slices that compose
  the SD-13 closeout tranche. It is bounded to the Dwarf row only and is
  not authorized to assert anything about the other 20 rows.

## Verdict
The Machine God's judgment on the Dwarf row:

- **support_state: `unverified`** — must remain `unverified`. No promotion is
  honest at the live evidence floor on 2026-07-06.
- **evidence_tier: `Observed`** — must remain `Observed`. Dwarf appears in
  the SD-13 packet roster and in the typed matrix carrier as a named row,
  but no direct runtime evidence exists.
- **evidence_freshness: `AwaitingInitialEvidence`** — unchanged.
- **subject_id: `race:dwarf`** — unchanged.
- **dimension: `bounded race semantics`** — unchanged.
- **grounding_ref: `SD13_ROSTER_MATRIX_DOC`** — unchanged.

## Why the row stays `unverified`

The repo evidence on 2026-07-06 (`origin/develop` at
`c78287ce76d3cce10fe814806558976fcfd70543`) contains **no direct runtime or
typed-compute evidence** for any Dwarf semantic family. The complete evidence
floor on this date is:

### What exists
1. The Dwarf row is named in the typed matrix carrier
   `src/rules_core/support_state_matrix.rs` (lines 224-235):
   `row_id: "race.dwarf.bounded_semantics"`, `subject_id: "race:dwarf"`,
   `support_state: SupportState::Unverified`, `evidence_tier: EvidenceTier::Observed`,
   `grounding_ref: SD13_ROSTER_MATRIX_DOC`.
2. The Dwarf row is named in the markdown matrix at
   `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
   (Race-rows section, row "Dwarf | bounded race semantics | unverified").
3. The Dwarf row is named in the visibility ledger at
   `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md`
   (Dwarf row, `unverified | Observed | SD-13 packet roster only`).
4. The PCC corpus file
   `tests/fixtures/pcc/core_rulebook_minimal.pcc` contains the path string
   `PCC:@\pathfinder\paizo\roleplaying_game\core_essentials\races\dwarf\_race.pcc`.
   This is a **path reference**, not parsed or computed Dwarf semantics.
5. The `sd13_support_state_matrix.rs` test pins the exact row id
   `"race.dwarf.bounded_semantics"` in the `EXPECTED_ROW_IDS` array
   (position 1, immediately after the Human pilot row).

### What does NOT exist (and must not be invented)
- No Dwarf ability-modifier code. The character_input module
  (`src/rules_core/character_input.rs`) parses `race_id` as a free string
  and does not bind it to the seven-race roster. There is no Dwarf-specific
  ability-bonus or penalty logic anywhere in `src/`.
- No Dwarf size, speed, or movement code. No PF1 Core Dwarf 20-ft base speed,
  no Medium size posture, no race-linked movement modifier.
- No Dwarf senses code. No darkvision 60-ft, no race-linked sense trait.
- No Dwarf languages code. No Common/Dwarven binding.
- No Dwarf bonus-feat or bonus-skill code. No "Greedy" trait, no
  "+2 Appraise, +2 Knowledge (engineering)" trait, no Defensive Training,
  no Hatred, no Hardy, no Stability, no Stonecunning.
- No Dwarf / class interaction seam. The current deterministic pilot path
  (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`)
  is Human-only. There is no Dwarf Fighter, Dwarf Cleric, Dwarf anything
  fixture anywhere in `tests/fixtures/`.
- The pilot_compute seam (`src/rules_core/pilot_compute.rs`) explicitly
  gates on `if input.chosen.race_id != HUMAN_RACE_ID` in four locations
  (lines 370, 674, 751, 849). Any non-Human race is blocked from the live
  compute seam and emits a claim-blocking diagnostic. Dwarf shares this
  block with all non-Human races.

### Why a promotion above `unverified` would be counterfeit
- `Partial` requires "some required semantics are proven" — zero Dwarf
  semantics are proven in the repo; there is no partial to claim.
- `Lossy` requires "the path can be executed only by discarding,
  flattening, or approximating named semantics" — there is no Dwarf path
  at all; lossy is not the right state.
- `Blocked` requires "known missing semantics, explicit claim-blocking
  diagnostics, or known contradictory behavior prevent the claim" — the
  claim is not "blocked" by a known contradiction; the claim simply has
  no evidence yet. Promoting Dwarf to `blocked` would falsely assert that
  the repo has actively determined the impossibility of Dwarf support.
  The honest state is "no evidence yet" = `unverified`.
- `Supported` is forbidden by all of the above.

## Named semantic-family coverage under SD13-F3

The race-semantic requirements (`technical-requirements.md` §6) require a
race support claim to classify at least these families when they affect the
bounded character-builder surface. The honest Dwarf classification per
family:

| Family | Verdict | Evidence |
|---|---|---|
| Identity and ruleset provenance | observed-only | SD-13 packet names Dwarf as a PF1 Core Rulebook core race; no parsed/converted/computed provenance in `src/`. |
| Ability-score modifiers or bonuses | unproven | no `+2 Constitution, -2 Charisma` (or any alternative Dwarf modifier) code path in `src/rules_core/character_input.rs` or `pilot_compute.rs`. |
| Size, speed, and movement-relevant baseline posture | unproven | no race-linked speed modifier; pilot_compute gates all non-Human races. |
| Senses or visibility-affecting traits | unproven | no darkvision code; no race-linked sense trait. |
| Racial bonus feats, skill modifiers, or derived-stat modifiers | unproven | no Dwarf bonus-feat, no Dwarf skill modifier, no Dwarf derived-stat modifier in any source. |
| Prerequisite, feat, or class-feature interactions triggered by the race | unproven | no Dwarf interaction seam; the only interaction seam exercised is Human bonus-feat / ability-bonus with Fighter level 1. |
| Other core racial traits that materially affect bounded level-10 support | unproven | no Defensive Training, Hardy, Stability, Hatred, Stonecunning, Greedy, or weapon familiarity code anywhere. |

Every required family is `unproven`. The Dwarf row therefore must stay
`unverified` / `Observed` until a later slice lands grounded evidence for
at least one of these families and an explicit row-state upgrade.

## Permitted movement on the typed matrix carrier
Per the parent gate (`t_5d57e115`): "matrix file is read-only for hand-edits;
row state updates flow through the same PR that lands the slice." This slice
moves the typed matrix carrier as follows:

- `blocker_or_lossiness_note`: replaces the empty string with the truthful
  honest-unverified note describing the seven required race-semantic families
  and their unproven status (per this artifact). This is the same kind of
  note the existing Human pilot row carries for its own blocker fields, so
  the typed matrix stays symmetric.
- `next_required_uplift`: replaces the generic "SD13-E2 race-semantic slice"
  text with an explicit pointer to this artifact plus the seven named
  family families that need grounded evidence before the row can move.
- `support_state`: `Unverified` (unchanged).
- `evidence_tier`: `Observed` (unchanged).
- `evidence_freshness`: `AwaitingInitialEvidence` (unchanged).
- `subject_id`: `race:dwarf` (unchanged).
- `dimension`: `bounded race semantics` (unchanged).
- `row_id`: `race.dwarf.bounded_semantics` (unchanged).
- `grounding_ref`: `SD13_ROSTER_MATRIX_DOC` (unchanged).

The matrix markdown file at
`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
is NOT edited by this slice. Its Dwarf row stays exactly as the parent gate
seeded it (`unverified | Observed | named by SD-13 scope only | create
race-semantic execution slice and classify the row honestly`). If the operator
decides to widen the markdown to mirror this artifact's blocker note later,
that is a separate gate-side decision, not this slice's authority.

## Required tests added by this slice
- `tests/sd13_race_dwarf_bounded_semantics.rs` — a focused race-row truth
  test that pins the seven unproven-family verdicts above, asserts the
  matrix row stays `unverified` / `Observed`, and rejects any promotion
  above `unverified` without an explicit slice decision. RED-then-GREEN
  discipline is enforced.

## Verification commands run on 2026-07-06
The focused RED-then-GREEN cycle for the new test file, plus the
existing matrix carrier test, were run inside the slice worktree at
`/home/ubuntu/.hermes/profiles/tech-priest/home/.hermes/worktrees/codex-tranche-2-6/t_3cf90c2c`.

## Non-goals observed
- Did not promote the Dwarf row above `unverified`.
- Did not introduce any Dwarf trait code, Dwarf ability modifier, Dwarf
  speed, Dwarf senses, Dwarf bonus-feat, or Dwarf interaction seam.
- Did not modify the matrix markdown file directly.
- Did not collapse this slice into a non-Dwarf race slice.
- Did not invent a "Dwarf is core supported" or "Dwarf is partial"
  breadth claim.
- Did not promote the existing typed matrix test
  (`tests/sd13_support_state_matrix.rs`) row count above 21.
- Did not bypass the matrix update path.

## Next required uplift (named, not invented)
For the Dwarf row to honestly move out of `unverified`, a later bounded
slice MUST ground at least one of the seven named race-semantic families
listed in the table above with:

1. A new accepted fixture family or pilot path that exercises the family.
2. A new typed module (or expansion of `support_state_matrix.rs` /
   `pilot_compute.rs`) that emits the computed evidence, explanation, or
   claim-blocking diagnostic for that family.
3. A new focused test that pins the family evidence at the
   `Computed` / `Oracle-checked` evidence tier.
4. An updated row state in the typed matrix carrier with a non-empty
   `blocker_or_lossiness_note` describing the remaining gap.

Until that later slice lands, the Dwarf row stays `unverified` /
`Observed`. The Machine God records this verdict.

Let it be recorded.