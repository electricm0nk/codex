# SD13-E2 Elf Bounded Race-Semantics Slice — Classification Artifact

**Slice card:** `t_37dbab62` (SD-13 CODE: Race/Elf — bounded race semantics classification slice)
**Branch:** `feat/sd13-race-elf-bounded-semantics`
**Slice base:** `c78287c` (develop tip, unchanged by this slice)
**Slice commit:** (see `git log -1 feat/sd13-race-elf-bounded-semantics` after push)
**Date:** 2026-07-06

---

## Matrix row state change (merge-receipt field)

| row_id | subject_id | support_state | evidence_tier | evidence_freshness | grounding_ref |
|---|---|---|---|---|---|
| `race.elf.bounded_semantics` | `race:elf` | `Unverified` (UNCHANGED) | `Observed` (UNCHANGED) | `AwaitingInitialEvidence` (UNCHANGED) | `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md` (UNCHANGED) |

**Honest classification:** `Unverified` with explicit Elf-named blocker note enumerating the seven required race-semantic families and an Elf-specific `next_required_uplift` pointing at this artifact. Row did NOT move to `Supported`, `Partial`, or `Lossy` — no live Elf proof surface exists on the deterministic seam (`pilot_compute.rs` grounds race semantics only for `race:human` and emits a non-claim-blocking `race.semantics.unverified` diagnostic for every other race identity, including Elf).

---

## Required race-semantic families (named debt, not yet grounded)

The Elf bounded row's `blocker_or_lossiness_note` now names these seven required race-semantic families, each still unproven on the live compute seam at the 2026-07-06 evidence floor:

1. **identity / provenance** — observed-only via the SD-13 packet roster and the typed matrix row carrier; not grounded as a runtime seam.
2. **ability-score modifiers** — PF1 Core +2 Dex / -2 Con, and the alternate +2 Int variant; ungrounded.
3. **size / speed / movement baseline** — Medium, 30 ft base; ungrounded.
4. **senses** — low-light vision; ungrounded.
5. **immunities** — sleep immunity; ungrounded.
6. **weapon familiarity** — longbow / rapier / longsword / shortbow / shortsword; ungrounded.
7. **other core racial traits** — bonus languages, keen senses, elven magic / weapon-training variants; ungrounded.

`pilot_compute.rs` explicitly gates every non-Human race out of the compute path via `if input.chosen.race_id != HUMAN_RACE_ID`. No Elf fixture exists under `tests/fixtures/`. Promotion above `Unverified` is counterfeit breadth until a later bounded slice lands grounded evidence for at least one of these families and updates the row state in the typed carrier.

---

## Verifications (verbatim command outputs)

```
$ cargo test --test sd13_elf_bounded_race_semantics
running 10 tests
test elf_row_carries_non_empty_blocker_note_naming_required_families ... ok
test elf_row_dimension_stays_stable_as_bounded_race_semantics ... ok
test elf_row_does_not_collude_with_human_race_seam ... ok
test elf_row_grounding_stays_on_the_roster_authority_only ... ok
test elf_row_is_not_silently_promoted_to_supported_partial_lossy_or_blocked ... ok
test elf_row_is_present_in_seeded_matrix ... ok
test elf_row_next_uplift_points_at_this_artifact ... ok
test elf_row_state_is_unverified_at_evidence_floor ... ok
test elf_row_subject_type_and_id_remain_intact ... ok
test slice_does_not_change_any_non_elf_row ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo test --test sd13_support_state_matrix --test ge06_pilot_base_computation --test ge06_failure_classifier
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo test --tests  (full repo floor)
... all test files: 0 failed
```

---

## Landed files (this slice)

| Path | Status | Description |
|---|---|---|
| `src/rules_core/support_state_matrix.rs` | modified | Elf row enriched with non-empty `blocker_or_lossiness_note` enumerating the seven required race-semantic families, and an Elf-specific `next_required_uplift` pointing at this artifact. Row state, evidence tier, evidence freshness, subject_type, subject_id, row_id, dimension, and grounding_ref are unchanged. |
| `tests/sd13_elf_bounded_race_semantics.rs` | added (10 tests) | Pins row shape (Unverified/Observed/AwaitingInitialEvidence), non-empty blocker note naming the seven families, Elf-specific uplift pointing at this artifact, roster-only grounding-ref, no-promotion invariant, no-Human-collusion invariant, and the slice_does_not_change_any_non_elf_row invariant. |
| `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-elf-bounded-race-semantics-classification-2026-07-06.md` | added (this file) | Travel-with-the-slice artifact naming the seven required race-semantic families, the verbatim verification outputs, and the explicit next-uplift acceptance criteria. |

Total: 3 files modified/added; ~280 insertions, 6 deletions in the carrier.

---

## Remaining boundary / what this slice deliberately does NOT do

- Elf row stays `Unverified` (not `Partial`, not `Supported`, not `Lossy`). No Elf race semantics are added to `pilot_compute.rs`.
- The SD-13 roster matrix markdown file (`programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/core-roster-and-support-state-matrix.md`) is NOT modified — that file is the documentary roster authority and the parent gate forbids hand-edits. Row state updates flow through the seeded Rust carrier instead.
- The 6 sibling non-Human race rows (Dwarf, Gnome, Half-Elf, Halfling, Half-Orc) are NOT touched — they are separate CODE cards on this tranche, each with its own slice branch and merge receipt.
- The Human race row (which sits at `Partial / Computed` with the `race:human:pilot_semantics` upstream pressure) is NOT touched.

---

## Next truthful move

A future slice that introduces:

1. a deterministic Elf pilot fixture carrying Elf-specific selections
   (e.g. an Elf ability-bonus, low-light-vision marker, sleep-immunity posture,
   weapon-familiarity markers),
2. an Elf-specific race-semantics diagnostic in `pilot_compute.rs` that names
   the grounded subset (e.g. ability modifier, low-light vision) versus the
   still-unverified subset (sleep immunity, weapon familiarity, bonus
   languages, keen senses, elven magic / weapon-training variants),

…would let the row move from `Unverified` to `Partial` with a bounded partial
classification. The `next_required_uplift` field on the row now names this
explicit path. Until such a slice exists, the honest answer is "Unverified
with explicit named blocker note" — exactly what this slice proves.

---

## CI / branch doctrine note

The slice branch `feat/sd13-race-elf-bounded-semantics` was authored under the
pre-2026-07-06 doctrine that required a PR to `develop`. Per the operator
doctrine change committed in `ab06d3e` ("ops(tranche-3): drop slice-base PR
guard — tranche shapes don't use PRs"), Tranche 3 slices push directly to
`tranche/3`. This slice therefore travels as a direct-push slice and is
ready for the operator's tranche/3 aggregation PR. The slice's base
(`c78287c`) is unchanged by the slice itself; no merge into `develop` is
required at slice close.

---

## Operator handoff

The slice is feature-complete on the `feat/sd13-race-elf-bounded-semantics`
branch with all 10 RED tests converted to GREEN, the E2-R1 regression floor
green (26/26), and the full `cargo test --tests` floor green. Todd retains
authority on whether to aggregate this slice branch into `tranche/3` via the
tranche-aggregation PR or to wait for additional sibling race slices to
land first.

Let it be recorded.