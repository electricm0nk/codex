# SD13-E2-F3a Half-Elf Bounded Race-Semantics Artifact

**Slice:** SD-13-E2-F3a
**Subject:** `race:half-elf`
**Matrix row:** `race.half_elf.bounded_semantics`
**Author:** SD13-E2-F3a slice (tech-priest lane, kanban `t_356173db`)
**Branch:** `feat/sd13-race-half-elf-bounded-semantics`
**Base SHA:** `c78287c` (develop HEAD at slice launch)

---

## Honest classifier verdict

| Axis | Value | Why |
|------|-------|-----|
| `support_state` | `Unverified` | This slice grounds **no computed Half-Elf mechanic**; promoted state would falsely assert bounded mechanical truth |
| `evidence_tier` | `Observed` | Recognition-only; no live compute seam beyond the chosen-input shape |
| `evidence_freshness` | `AwaitingInitialEvidence` | Per the matrix freshness invariant (`freshness_tracks_live_proof_grounding_not_optimism`), refreshability tracks the evidence tier, not the grounding ref: this Observed row has no runtime Half-Elf evidence to refresh — the re-runnable proof surface pins the *absence* of computed evidence, and the minted fixture is recognition-only |
| `grounding_ref` | `tests/sd13_race_half_elf_bounded_semantics.rs` | The bounded proof surface that pins the recognition-only behavior and proves the absence of any Half-Elf `ComputationExplanation` |

The row **stays `Unverified`**. This is an honest classifier outcome, not a non-event: the slice promoted Half-Elf from "named in roster, no code-level footprint at all" to "named in roster, code seam recognizes it, bounded diagnostic names every Half-Elf PF1 trait — but no computed mechanic is asserted." A reader who treats Half-Elf as `Supported` or `Partial` at this slice is asserting truth the seed does not carry.

## Bounded scope statement (what this slice proves)

1. **Recognition on the chosen-input seam.** A `CharacterInput` whose `chosen.race_id` is `"race:half-elf"` parses without `InvalidCharacterInput` diagnostics and reaches the compute path. The deterministic pilot seam no longer leaves Half-Elf as a phantom or string-literal-free token.
2. **Explicit bounded diagnostic on the receipt.** The receipt carries exactly one new diagnostic with id `race.half_elf.bounded_semantics` and `claim_blocking = false`. Its message names every PF1 Core Rulebook Half-Elf racial trait and explicitly states that no computed mechanic is emitted for any of them.
3. **No Half-Elf `ComputationExplanation` is emitted.** There is **no** `race.half_elf.ability_bonus_target` analog (the minted Half-Elf deterministic input fixture is recognition-only; a computed-mechanic fixture is required before one can be honestly grounded) and no `race.half_elf.bonus_feat_grant` analog (Half-Elf gets no named bonus feat at level 1 in PF1).
4. **The Human race seam is byte-stable.** This slice refactors `explain_human_race_seam` into a named-race dispatcher without altering any `race.human.*` id, the `race.semantics.unverified` catch-all for non-Human / non-Half-Elf races, or the existing Human-deterministic-pilot test surface.

## Bounded scope statement (what this slice does NOT prove)

This slice **deliberately does not assert** any of the following. Each is named in the diagnostic message and in the matrix row's blocker note so downstream readers cannot infer them by omission:

- **Half-Elf ability-bonus to one chosen ability.** The minted Half-Elf deterministic input fixture is recognition-only (no computed-mechanic fixture exists); no `choice:half_elf_ability_bonus` choice set is defined; no per-ability-modifier emission is wired up.
- **Sleep immunity.** No spell-effect surface exists in the bounded pilot seam; sleep immunity cannot be honored or claim-blocked at this slice.
- **Elven blood / low-light vision.** No vision / senses interaction surface is wired up.
- **+2 racial bonus on Listen, Spot, Search skill focus.** No skill-circumstance-bonus engine exists; skill-rank emission only covers chosen-input ranks plus the deterministic +3 class-skill bonus.
- **Favored class flexibility.** No multiclass detection engine; the deterministic pilot is single-class level 1.
- **Multiclass adaptability (Human free feat slot analog).** No Half-Elf bonus-feat choice set is defined; no Human-bonus-feat analog exists for Half-Elf.

If any reader treats this slice as asserting truth for any of the above, they are inventing breadth not present in the seed.

## Files touched by this slice

| File | Change |
|------|--------|
| `src/rules_core/pilot_compute.rs` | Add `HALF_ELF_RACE_ID` constant; refactor `explain_human_race_seam` into a named-race dispatcher; add `explain_half_elf_bounded_race_seam` emitting the single bounded diagnostic |
| `src/rules_core/support_state_matrix.rs` | Add `SD13_HALF_ELF_RACE_TEST` constant; update the `race.half_elf.bounded_semantics` row in `seeded_sd13_e1_f1_current_truth()` with `blocker_or_lossiness_note` and a precise `next_required_uplift` |
| `tests/fixtures/rules_core/pf1_half_elf_fighter_level1_sd13_deterministic_input.txt` | New Half-Elf deterministic input fixture (recognition-only) |
| `tests/sd13_race_half_elf_bounded_semantics.rs` | New bounded proof surface (includes the Half-Elf-row classifier pin) |
| `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/race-bounded-semantics/half-elf-bounded-semantics.md` | This artifact |

The matrix markdown file
`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`
is **forbidden for hand-edit** at this slice; the row state change flows through the PR's merge-receipt only.

## Next required uplift

A later SD13-E2 race-semantic Half-Elf computed-mechanic slice must:

1. Upgrade the existing recognition-only Half-Elf deterministic input fixture into a computed-mechanic grounding fixture (parallel to `pf1_human_fighter_level1_sd13_deterministic_input.txt`).
2. Define a `choice:half_elf_ability_bonus` choice set and ground an explicit Half-Elf ability-bonus choice-target `ComputationExplanation` parallel to `race.human.ability_bonus_target`.
3. Ground at least one bounded non-ability Half-Elf mechanic (lowest-cost available: immunity to sleep, since it only requires a defensive-effect surface; low-light vision requires a senses interaction engine that is currently absent).
4. Promote the matrix row to `Partial` / `Computed` only after the bounded proof surface can sustain that promotion honestly.

Until that slice lands, the Half-Elf row stays `Unverified` / `Observed` with the blocker note and next-uplift language above.
