# SD13-E2-F3 Halfling Bounded Race-Semantics Slice — Merge Receipt

**Slice card:** `t_1731714c` (SD-13 CODE: Race/Halfling — bounded race semantics classification slice)
**Branch:** `feat/sd13-race-halfling-bounded-semantics`
**PR:** https://github.com/electricm0nk/codex/pull/100
**Merge commit (squash on origin/develop):** `4150c92f1e0795722cd54ea08400c20f9c52eb50`
**Merge base:** `tranche/3` (squash-merged into develop as Tranche 2.6 closeout)
**Slice tip (pre-squash):** `b3d71bab80d04659836b87dc253834139a4c4367`
**Date:** 2026-07-06 (slice); reconciled 2026-07-07 by PR #109 (`ad9be2a`)

---

## Matrix row state change (merge-receipt field)

| row_id | subject_id | support_state | evidence_tier | evidence_freshness | grounding_ref |
|---|---|---|---|---|---|
| `race.halfling.bounded_semantics` | `race:halfling` | `Unverified` (UNCHANGED) | `Observed` (UNCHANGED) | `AwaitingInitialEvidence` (UNCHANGED) | `tests/sd13_race_halfling_bounded_semantics.rs` (NEW) |

**Honest classification:** `Unverified` with explicit Halfling-named blocker note and Halfling-specific dimension / next-uplift. Row did NOT move to `Supported`, `Partial`, or `Lossy` — no live Halfling computed proof surface exists on the deterministic seam (`pilot_compute.rs` grounds race semantics only for `race:human` and emits a non-claim-blocking `race.semantics.unverified` diagnostic for every other race identity, including Halfling).

### Blocker note (verbatim from `src/rules_core/support_state_matrix.rs:511-535` at origin/develop HEAD = 62f3b64)

> no direct runtime evidence for any of the seven required Halfling race-semantic families at the live evidence floor (2026-07-06): identity/provenance is observed-only via the SD-13 packet roster and the typed matrix row carrier, but ability-score modifiers (PF1 Core +2 Dex / -2 Str or any alternative), size/speed/movement baseline (Small size, 20-ft base speed), senses (no Halfling darkvision; only the human-sense baseline), racial bonus feats and skill modifiers (+1 thrown attack roll with thrown weapons and slings, +2 Appraise, +2 Climb), prerequisite/feat/class-feature interactions (favored class bonus, Halfling racial traits interacting with class features), and Halfling racial trait dimension / Halfling-specific row justifications stay unproven at the compute seam.

### Next required uplift (verbatim)

> classify the seven required Halfling race-semantic families honestly against any grounded Halfling pilot seam, mirroring the Half-Elf recognition slice precedent.

### Reconciliation note (2026-07-07)

A brief `Partial` / `Computed` uplift was applied on `origin/tranche/3` in commit `4cfe284` (Halfling recognition slice, after #100 had already landed) which recognized ability modifiers, size, speed, and senses as direct runtime evidence on `explain_halfling_race_seam`. PR #109 (`ad9be2a`) reconciled that uplift back to `Unverified / Observed / AwaitingInitialEvidence` on `origin/develop` once the half-elf precedent made the Half-Elf recognition slice the authoritative non-Human race-seam pattern. This receipt captures the post-reconcile authoritative state.

---

## Verifications (verbatim command outputs against origin/develop HEAD 62f3b64)

```
$ cargo test --test sd13_race_halfling_bounded_semantics
running 12 tests
test halfling_row_blocker_note_carries_honest_unverified_reason ... ok
test halfling_row_coexists_with_other_unverified_race_rows ... ok
test halfling_row_does_not_promote_above_unverified ... ok
test halfling_row_dimension_unchanged_for_this_slice ... ok
test halfling_row_evidence_freshness_is_awaiting_initial_evidence ... ok
test halfling_row_grounding_ref_is_present ... ok
test halfling_row_evidence_tier_is_observed_only ... ok
test halfling_row_is_present_in_seeded_matrix ... ok
test halfling_row_state_is_unverified_at_evidence_floor ... ok
test halfling_row_is_not_part_of_seven_by_eleven_combination_claim ... ok
test halfling_row_subject_id_unchanged ... ok
test halfling_row_next_uplift_points_at_classification_artifact ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo test --test sd13_support_state_matrix
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ gh pr view 100 --repo electricm0nk/codex --json state,mergedAt,mergeCommit,baseRefName
{"baseRefName":"tranche/3","headRefName":"feat/sd13-race-halfling-bounded-semantics","mergeCommit":{"oid":"4150c92f1e0795722cd54ea08400c20f9c52eb50"},"mergedAt":"2026-07-07T03:05:06Z","number":100,"state":"MERGED","title":"feat(sd13-e2-f3): honest Halfling race-semantics classification stays Unverified"}
```

---

## Precedent honored

- `t_3cf90c2c` (Dwarf row slice, commit `db105f8`, branch `feat/sd13-race-dwarf-bounded-semantics`) — same template, same unverified-with-explicit-blocker pattern.
- `t_7f355f9c` (Half-Orc merge-receipt format) — the `sd13-e2-half-orc-merge-receipt-2026-07-06.md` artifact structure is the direct template.

## Non-goals observed

- Did not promote `race.halfling.bounded_semantics` to `Supported`, `Partial`, or `Lossy`.
- Did not invent Halfling trait code, Halfling ability modifier math, Halfling class-feature interaction, or Halfling darkvision.
- Did not hand-edit the matrix markdown file `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` (forbidden write scope).
- Did not collapse into a non-Halfling race slice.
- Did not assert a seven-by-eleven combination claim (no breadth fabrication).

## Author

Todd Hintzmann <todd@hintzmann.net>