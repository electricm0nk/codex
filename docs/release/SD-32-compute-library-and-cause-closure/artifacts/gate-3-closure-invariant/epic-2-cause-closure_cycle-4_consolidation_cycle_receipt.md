# Cycle epic-2-cause-closure/4 — Gate 3 closure invariant / Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`, consolidation

- **Card ID:** `epic-2-cause-closure` (this cycle closes the row — all six dispatched lanes plus
  the sibling T8 write-scope lane have now landed on `origin/tranche/12`).
- **Commit SHA:** (filled in at push — see this file's own commit in `git log`)
- **Files touched:** this receipt (new); `kanban.md` (row 11 status); `progress.md` (consolidation
  cycle entry appended before `## Open blockers`). No production source changed by this cycle — it
  is a re-derivation and disposition pass over work six other lanes already landed.
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 (all ten shapes:
  T1/T2a/T2b/T3/T4/T5/T7/T8/T9/T12); `decisions.md §10` Definition of Done (every Epic 1-5 card
  `complete`); `decisions.md §11` condition 4 ("T8 closing removes the last non-`complete`
  condition on card 11").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (unchanged this cycle;
  `scripts/verify.sh --only preflight-oracle` PASS at this pin, re-confirmed).

## Per-shape disposition (re-derived from the repo, not from lane self-reports)

| Shape | Disposition | Evidence |
|---|---|---|
| **T1** | closed-this-run (cycle 1) | `185027717` — dispatch-gap ("Monk shape") closed corpus-wide across classes/races/monsters; new standing test `reach_gate::tests::dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`, present and passing in the live suite (confirmed below, `reach_gate` 31/31 via `scripts/verify.sh --only reach`). |
| **T2a** | closed-this-run, honest residual named | `985e24c1e` — `class_feature.rs` 5-tier `data.class` resolution chain fixed at cause; 4,936 records corrected corpus-wide (`data/corpus/**/class_feature/**/*.json`). Population re-derived (not the stale cited 8,243): pre-fix 5,678 → post-fix 4,284. ~2,775-record residual (unresolvable category labels — Domain Power, Wild Talent, etc.) named as a follow-on, not fabricated to zero. |
| **T2b** | closed-this-run as a legitimate measurement cycle (standing lesson 6); disposition ruled by `decisions.md §11` cond. 4 | `b440d1680` — named cause (`modelled_race_of_race_trait()` compound-key matcher) proven non-operative: zero overlap between the 2,472 residual units' provenance and any ingested `race_trait` record's `source.path`/`source.line`. Real cause identified (book-onboarding gap, 1,754 units; un-transcribed selector rows, 718 units) and named as a separate, out-of-lane content project — not a defect this card's criterion (matcher-cause closure) covers. New standing regression `race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`, present in the live suite. |
| **T3** | closed-earlier-and-cited | Card 1's own receipt, 7 of 29 generators fixed (`progress.md` line 583-584). |
| **T4** | closed-this-run for its L8 population, sibling L9 named disjoint | `4911a9b33` — re-derived population 6,975 (not the stale 2,763); root cause (`buildClassFeatureSurface` only enriches a row an `ExplanationDto` already created) fixed corpus-wide, data-driven off `heldClasses`, via `unmatchedClassFeatureDescriptions()` + `ClassFeatureDescriptionReferenceSection`. L9 (471 units, `class_feature_feat_bridge.rs`) confirmed structurally disjoint (synthetic pool-group `classSlug`, not a real class token) — correctly out of this shape's own scope, not a gap in this closure. |
| **T5** | closed-earlier-and-cited | Card 4's own receipt, all four books' `RuleSetId`s land, 422 units matching `epic-breakdown.md` (`progress.md` line 583). |
| **T7** | closed-this-run, corpus-wide | `caaef7762` — `resolvable_grants()` now refuses any bare-`PRECLASS:`-only pair lacking `mod_row_*` corroboration. Re-derived risk population: 1 pair (`gunslinger, Gunslinger ~ Gun Training`), not the 4 named by D12 — the other 3 already double-protected by the module's own pre-existing `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` (correction logged, `docs/retro/events/epic-2-t7-t8.jsonl`). |
| **T8** | closed-this-run (separate concurrent lane, landed on `origin/tranche/12` after the six-lane dispatch) | `e3f3559dd` (fix), `5f5d82813` (warm-cache-invalidation correction — schema bump 12→13). Population re-derived: exactly 12 `core_rulebook` `class_feature` units, `wiring_class=='display' and status=='grounded'`, matching D13 exactly. Closed by class (a predicate on kind/wiring_class/status/evidence, no literal ids) per `decisions.md §11` condition 1. All four moved dashboard figures re-derived against the real warm cache and matched exactly: `corpus_wide.display` 14285→14273, `corpus_wide.computed` 9464→9476, `doneness.done` 13458→13470, `doneness.held` 1230→1218. `decisions.md §11` condition 4 states in the committed ruling itself: "T8 closing removes the last non-`complete` condition on card 11." |
| **T9** | closed-this-run as a legitimate measurement cycle (standing lesson 6); disposition ruled by `decisions.md §11` cond. 4 | `212dc9f7c` — population re-derived 2,712 (not the filed 2,651; correction logged). Forensic pass on the `monster` family (28 units, all 6 residual books, byte-identical fresh-transcription cross-check) found no uniform onboarding gap: 21 PI-excluded (blocked on a separate, explicitly-DRAFT operator PI ruling — `docs/governance/ogl-pi-blacklist.md` — not this card's criterion to grant), 6 correctly-excluded `.MOD`/`.COPY` overlay rows (not creatures), 1 genuine unwired gap (`occult_adventures`, single instance — closing it alone would violate AT-32-E2-001's own "closes one instance and stops" anti-gaming rule). Spot-check on `companion`/`core_rulebook` found the same orphan-ability-row shape (not generalized further this cycle). |
| **T12** | closed-this-run (combined with T2a per card 11's own cycle-1 receipt) | Same commit as T2a (`985e24c1e`). |T12| re-derived at 2,453 (unchanged — `v06_work_inventory.rs` never reads `data.class`, so nothing about the T2a fix could move it). |T2a ∩ T12| 1,509 (S20 join method); |T2a ∪ T12| 5,228. |

**Ruling applied to T2b/T9's "ruling needed" flag:** both lane reports asked, verbatim, whether a
zero-units-banked measurement cycle with a proven, re-derivable, non-fabricated cause-disproof
counts as this shape's own closure, or needs a further dedicated multi-cycle ingestion effort.
`decisions.md §11` (committed, operator-pinned, dated the same day, written and landed *after*
those two lanes' reports) answers this directly in its own text: condition 4 states T8 — not
T2b, not T9 — is "the last non-`complete` condition on card 11." That sentence is only true if
T2a/T2b/T4/T9/T12/T7/T1/T3/T5 were already regarded as resolved at the moment Decision 11 was
written. This consolidation cycle applies that committed ruling rather than filing a fresh
`## Open blockers` request for the same question Decision 11 already answered — re-filing it would
repeat exactly the mistake `decisions.md §10` corrected (asking for a ruling by pausing the bundle
instead of reading the one already in force). No new number was invented to make T2b/T9 look
closed: both retain their real, re-derived, honestly-reported residual populations (2,472 and
2,712 respectively), named as future book-onboarding/content-ingestion scope in
`forward-scope-register.md` territory for a successor bundle, not silently absorbed into "done."

## Suite re-runs (this cycle, on `tranche/12` tip after all seven lanes)

```
$ cargo test --locked --lib
test result: ok. 2388 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out; finished in 12.92s

$ cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
test result: ok. 516 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 47.17s

$ scripts/verify.sh --only reach
PASS  reach  (31 passed)
RESULT: PASS

$ scripts/verify.sh --only shape-coverage-standing-gate
PASS  shape-coverage-standing-gate  (population=24914 unclassified=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
RESULT: PASS

$ scripts/verify.sh --only shape-coverage-standing-gate-selftest
PASS  shape-coverage-standing-gate-selftest  (9 cases passed)
RESULT: PASS

$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger-check.json
population (not-done units considered): 24914
unclassified: 0
```

No regressions from any of the seven lanes' combined landings. `rule_set_mapping_tests::uncompiled_books_stay_none`,
flagged as a pre-existing unrelated failure in the T2b lane report, is not failing in this cycle's
full-suite run (0 failed across 2388 + 516 tests) — resolved by a later concurrent lane, not by
this cycle.

## Four gates re-confirmed

- **Gate 0** (census): unaffected by this cycle; card 15 (a separate, still `in-progress` card
  outside this cycle's Epic 2 scope) is working the kind-unenumerable-object reconciliation
  concurrently — noted, not acted on here (out of this dispatch's granted scope).
- **Gate 1** (shape closure): `shape-coverage-standing-gate-selftest` PASS, 9 cases.
- **Gate 2** (engines): unaffected by this cycle (cards 6-8 already `complete`, no engine file
  touched by any of the seven card-11 lanes).
- **Gate 3** (closure invariant): `shape-coverage-standing-gate` PASS, population 24,914,
  `unclassified_count` 0, corpus SHA `7f818006e371188e5717fd18d74d18a420747fc6` — matches the
  gate's prior closed state exactly; no card-11 lane moved the not-done population (T8's move is
  a *classification* correction inside `wiring_class`/`doneness`, not a change to `unclassified_count`
  or the not-done total).

## Kanban row 11

Set to **`complete`**. Every one of the ten AT-32-E2-001 shapes now carries either a closed-this-run
disposition with a commit and re-derivable evidence, or a closed-earlier-and-cited disposition
pointing at the citing card's own receipt. No shape is silently dropped, and no residual population
was zeroed by assertion — T2a (~2,775), T2b (2,472), T9 (~2,684 non-`monster` units not yet
forensically checked) all retain their honestly-stated, re-derivable residuals, named as
successor-bundle scope rather than folded into "complete."

## Discoveries

None requiring a new card. Card 15 (`census-scope-closure`) remains `in-progress` under a separate,
concurrently-running dispatch — outside this cycle's card-11 scope, noted for the bundle's overall
Definition-of-Done status but not actioned here.

## Next-cycle plan

Card 11 needs no further work under AT-32-E2-001 as currently scoped. A successor bundle's own
scope (not this card's): the T2a ~2,775-record category-label residual; T2b's 2,472-unit
book-onboarding/transcription project; T9's PI-ruling-gated 21 units, single genuine
`occult_adventures` gap, and unforensicked `spell`/`feat`/`equipment`/remaining
`companion`/`monster_ability` residuals; T4's L9 (471 units, needs a feat-held reachability gate,
not a class-held one).

`df -h /`: (pasted below, run at end of cycle).
