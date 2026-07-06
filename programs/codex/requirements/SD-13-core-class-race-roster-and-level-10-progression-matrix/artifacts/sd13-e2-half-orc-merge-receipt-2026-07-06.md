# SD13-E2 Half-Orc Bounded Race-Semantics Slice — Merge Receipt

**Slice card:** `t_7f355f9c` (SD-13 CODE: Race/Half-Orc — bounded race semantics classification slice)
**Branch:** `feat/sd13-race-half-orc-bounded-semantics`
**PR:** https://github.com/electricm0nk/codex/pull/90
**Merge commit (target):** develop @ c78287c (slice base, unchanged by this PR)
**Slice commit:** `32de867b1ea71d74bde536d253a057f6f6289831`
**Date:** 2026-07-06

---

## Matrix row state change (merge-receipt field)

| row_id | subject_id | support_state | evidence_tier | evidence_freshness | grounding_ref |
|---|---|---|---|---|---|
| `race.half_orc.bounded_semantics` | `race:half-orc` | `Unverified` (UNCHANGED) | `Observed` (UNCHANGED) | `AwaitingInitialEvidence` (UNCHANGED) | `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` (UNCHANGED) |

**Honest classification:** `Unverified` with explicit Half-Orc-named blocker note and Half-Orc-specific dimension / next-uplift. Row did NOT move to `Supported`, `Partial`, or `Lossy` — no live Half-Orc proof surface exists on the deterministic seam (`pilot_compute.rs` grounds race semantics only for `race:human` and emits a non-claim-blocking `race.semantics.unverified` diagnostic for every other race identity, including Half-Orc).

---

## Verifications (verbatim command outputs)

```
$ cargo test --test sd13_half_orc_bounded_race_semantics
running 9 tests
test half_orc_fighter_pilot_does_not_emit_human_race_explanations ... ok
test half_orc_fighter_pilot_emits_race_semantics_unverified_diagnostic ... ok
test half_orc_fighter_pilot_receipt_remains_computed_with_only_race_gap ... ok
test half_orc_row_blocker_note_is_non_empty_and_names_missing_burden ... ok
test half_orc_row_dimension_names_half_orc_specific_semantics ... ok
test half_orc_row_grounding_ref_still_points_to_roster_only ... ok
test half_orc_row_next_uplift_is_half_orc_specific ... ok
test half_orc_row_remains_a_race_row_not_a_class_or_interaction_row ... ok
test half_orc_row_stays_unverified_and_observed_awaiting_initial_evidence ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo test --test sd13_support_state_matrix
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo test  (full repo floor)
TOTAL: passed=128 failed=0
```

---

## Landed files (this PR)

| Path | Status | Description |
|---|---|---|
| `src/rules_core/support_state_matrix.rs` | modified | Half-Orc row enriched with Half-Orc-specific `dimension`, non-empty `blocker_or_lossiness_note` enumerating the named missing Half-Orc trait burden, and Half-Orc-specific `next_required_uplift`. Row state, evidence tier, evidence freshness, and grounding_ref are unchanged. |
| `tests/sd13_half_orc_bounded_race_semantics.rs` | added (9 tests) | Pins row shape (Unverified/Observed/AwaitingInitialEvidence), Half-Orc-specific dimension/non-empty blocker note/Half-Orc-specific uplift, roster-only grounding-ref, and the live compute path's behavior on a deterministic Half-Orc Fighter level-1 fixture (the fixture emits `race.semantics.unverified` exactly once, remains non-claim-blocking, and the bounded non-race seam still produces a `Computed` pilot receipt). |
| `tests/fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt` | added | Deterministic Half-Orc Fighter level-1 chosen-input fixture driving the new tests. |

Total: 3 files, 380 insertions, 3 deletions.

---

## Remaining boundary / what this slice deliberately does NOT do

- Half-Orc row stays `Unverified` (not `Partial`, not `Supported`, not `Lossy`). No Half-Orc race semantics are added to `pilot_compute.rs`.
- The SD-13 roster matrix markdown file (`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`) is NOT modified — that file is the documentary roster authority and the task body forbids hand-edits. Row state updates flow through the seeded Rust carrier instead.
- The 6 sibling non-Human race rows (Dwarf, Elf, Gnome, Half-Elf, Halfling) are NOT touched — they are separate CODE cards on this tranche.

---

## Next truthful move

A future slice that introduces:
1. a deterministic Half-Orc pilot fixture carrying Half-Orc-specific selections
   (e.g. a Half-Orc ability-bonus, weapon-familiarity markers, ferocity posture),
2. a Half-Orc-specific race-semantics diagnostic in `pilot_compute.rs` that names
   the grounded subset (e.g. ability modifier, darkvision) versus the still-unverified
   subset (size, ferocity, weapon familiarity, skill bonus, favored-class, racial archetype),

…would let the row move from `Unverified` to `Partial` with a bounded partial
classification. The `next_required_uplift` field on the row now names this
explicit path.

Until such a slice exists, the honest answer is "Unverified with explicit named
blocker note" — exactly what this slice proves.

---

## CI status (PR self-review)

- GitHub Actions check `copilot-pull-request-reviewer` is the only configured
  required check on `develop`-targeted PRs. It was in_progress at slice close;
  silent while green. Self-review per `tech-priest-ci-watchdog` step 1
  completed clean: scope audit (Half-Orc row only), test presence (9 new +
  26 existing matrix tests still green), secrets scan (none), diff-vs-brief
  drift (none — slice delivers exactly the "leave unverified with explicit
  blocker" branch of the card body).

## Operator handoff

Todd retains approval/merge authority on this PR per the project's branch
governance. Once #90 is merged to develop, the slice is complete and the
matrix row's enriched dimension/blocker-note/uplift is the new merge-truth
on `origin/develop`. The card transitions to `done` at that moment, not
before — the merge claim must come from Todd, not from this lane.