# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 3)

- **Commit SHA:** `0398240942c6a2d7432c2e876a88638c8a9474f4`
- **Files touched:** this receipt only (`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_3.md`), plus the standing three shared files (`progress.md`, `kanban.md`). No production code shipped changed (one temporary `eprintln!` debug line was added to `src/rules_core/class_feature_pool_catalog.rs` for this cycle's own investigation and reverted before commit — `git diff --stat src/rules_core/class_feature_pool_catalog.rs` is empty at commit time).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (the epic-scoped diff against `origin/develop` for this cycle's own committed files carries no new bundle-tag identifiers; the large `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` hits in the full epic-scoped diff are pre-existing `docs/work-inventory.json` data-field values from earlier, already-audited cycles, unchanged by this cycle).
- **Wired-integration audit result:** OK_NO_TOKENS (no new `STUB`/`MOCK`/`placeholder`/etc. token introduced by this cycle's own commit; pre-existing `placeholder` hits are from earlier cycles' already-audited work, e.g. the `class_feature_option_pool` mechanism's `VACUOUS_PLACEHOLDER_CLASS_FEATURES` naming).
- **Acceptance criterion:** AT-34-E3-001 — bucket B closes: records reach their tables — this
  cycle owns exactly mechanism 4 of 9,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`.

## Re-derived population (do not quote the prior receipt's number without checking)

```bash
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
```
→ **346** at this cycle's HEAD (`c3202a90ce`), unchanged from both prior cycles' receipts.

## This cycle's own investigation — a real, deeper sub-partition of the largest open sub-cause

The two prior cycles left `engine_effect_token_present` (121 units) as an unbroken pile: "real
engine wiring, one mechanical shape at a time." This cycle broke it open by name, temporarily
instrumenting the existing committed regression test
(`class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`) with a
debug `eprintln!` of each unit's `corpus_key` at the `engine_effect_token_present` branch,
running it, then reverting the instrumentation (confirmed clean via `git diff --stat` before
commit — no production change ships this cycle).

```bash
cargo test --locked --lib rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly -- --nocapture 2>&1 | grep DEBUGKEY | wc -l
```
→ 121 (matches the sub-cause count exactly). Grouped by corpus-key prefix:

| Shape | Units |
|---|---:|
| `Sorcerer Bloodline Feat ~ *` | 87 |
| `Ranger Combat Style Feat ~ *` | 16 |
| `Rogue Talent ~ *` | 3 |
| `Wizard ~ *` (Opposition-School bookkeeping) | 3 |
| `Core Domain ~ *` | 2 |
| `Monk ~ *` | 2 |
| `Shadowdancer ~ *` | 2 |
| `Assassin ~ *` | 1 |
| `Cleric ~ *` | 1 |
| `Duelist ~ *` | 1 |
| `Nobility Domain ~ *` | 1 |
| `Sorcerer Bonus Spell L3 ~ *` | 1 |
| `Sorcerer ~ *` | 1 |
| **Total** | **121** |

**The two largest shapes (103 of 121) are the same architectural pattern already ratified
elsewhere in this engine, not a new one.** `Sorcerer Bloodline Feat ~ <feat name>` (87 records)
and `Ranger Combat Style Feat ~ <feat name>` (16 records) are each PCGen's per-option
enumeration of an automatic-bonus-feat slot: `ABILITY:FEAT|AUTOMATIC|<name>` gated by
`PREVARGTEQ:<Class>_BloodlineFeat_<Name>,1` / an equivalent Combat-Style variable, each with a
real `DESC`/`BENEFIT` naming what the specific feat option does. Sampled records confirmed this
shape directly (`Sorcerer Bloodline Feat ~ Empower Spell`, `~ Skill Focus (Knowledge [Planes])`,
`~ Improved Overrun`, `~ Agile Maneuvers`).

`src/rules_core/pilot_compute/mod.rs:1837-1844` already documents the SAME shape for the Arcane
bloodline's own bonus-feat slots (`ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS`), and states the
ratified treatment verbatim: *"Only the COUNT of slots is grounded as a magnitude; which feat
fills a slot is a player choice this seam deliberately does not model, the ratified treatment
already used for Fighter's, Cavalier's, and Brawler's own bonus feats."* Consistent with that:
`Fighter ~ Bonus Feats` is a single aggregate `core_rulebook` unit in the inventory (evidence
`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`) — Fighter's corpus never
ingests one unit per eligible feat option the way Sorcerer's and Ranger's corpora do.

**Confirmed this cycle, not merely inferred:** `ENGINE_EFFECT_TOKEN_KEYS` (line 228-229,
`class_feature_pool_catalog.rs`) includes `ABILITY`, and `has_no_engine_effect_token` gates the
SAME real serving path used by `load_pool_catalog` (line 573), not only this test's classifier
(line 1427). So even widening `REGISTERED_POOL_GROUPS` to include `"Sorcerer Bloodline Feat"` /
`"Ranger Combat Style Feat"` would not text-complete these 103 records today — the catalog would
still correctly refuse them as carrying a real mechanic, exactly as Decision 7 requires.

**Why 0 of 121 (and thus 0 of 346) can move this cycle from this finding.** The finding narrows
the *kind* of work needed for the 103-unit majority of this sub-cause, but does not supply a
safe fix inside this mechanism's own catalog-consulting lane:
- Building the SAME "count-only, choice-not-modelled" ratified pattern for Sorcerer's and
  Ranger's per-feat sub-choice records is real new engine work (a slot-count magnitude for each
  class), and even once built it would not close these 103 units under bucket B — it would
  instead argue they should never have been scored as individually-placeable bucket-B records at
  all, the same shape the prior cycle already surfaced for the 143-unit
  `description_is_null_internal_bookkeeping` sub-cause. This is an operator-scoped
  classification question, not a narrow fix: *should a PCGen per-option enumeration record for a
  slot whose ratified treatment is "count only, not the choice" ever be a bucket-B target, or
  should it be reclassified alongside Fighter's non-ingested equivalent?* Deciding that
  unilaterally here would either misreport 103 records as closed without building the real
  count-grounding wiring, or silently narrow the mechanism's own population — both prohibited
  (`acceptance-and-verification.md §5`).
- The remaining 18 of 121 (`Rogue Talent`/`Wizard`/`Core Domain`/`Monk`/`Shadowdancer`/
  `Assassin`/`Cleric`/`Duelist`/`Nobility Domain`/`Sorcerer Bonus Spell L3`/`Sorcerer`) are a
  long tail, each its own genuine mechanical shape (domain spell-list grants, opposition-school
  bookkeeping, bonus-spell-slot grants, etc.) — real engine wiring, one at a time, matching the
  prior cycle's own characterization.

The other six sub-causes (`description_is_null_internal_bookkeeping` 143,
`catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` 67,
`class_specific_level_phrase` 6, `dropped_pcgen_args` 5, `multi_desc_segment_not_regenerated` 3,
`bare_percent_reference` 1) are unchanged from the prior cycle's own investigation; this cycle
did not re-open them beyond re-confirming the total.

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 346 (command above, denominator: `core_rulebook`
  units with `status=='engine-does-not-hold'` and this evidence string).
- **`engine_effect_token_present` sub-cause, re-partitioned by corpus-key prefix:** 87+16+3+3+2+2+2+1+1+1+1+1+1 = 121 (command above).
- **Bucket B, `core_rulebook` (atlas-real partition):** unchanged at 694 of 6,701 —
  `python3 scripts/completion_atlas.py --by-book` (denominator: all `core_rulebook` content
  units the atlas classifies).
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=1` (one pre-existing
  violation at `progress.md:134`, from an earlier cycle's own entry, not introduced or touched
  this cycle — left as-is, out of this cycle's file-touch scope).

## Row-count command output

```
$ cargo test --locked --lib rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly -- --nocapture
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2890 filtered out; finished in 56.56s
```
This cycle's own artifact is this receipt; it makes no new claim of closed units, so the
row-count that governs `status` is the mechanism's population count above: 346 remaining, 0
closed.

## Build scope verified

- `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`)
  → exit 0 (`Finished `test` profile [unoptimized + debuginfo] target(s) in 2m 32s`, no
  `error[` lines), run at this cycle's starting HEAD `c3202a90ce` (no commit in this cycle moves
  any figure this build depends on — docs-only change).
- `apps/desktop/src-tauri`: not touched this cycle (no production or test file changed — the
  debug `eprintln!` was reverted before commit); not re-run, no change in this crate's compile
  surface since the last cycle's own green run.

## Sweep population

N/A — no corpus record added, regenerated, or otherwise touched this cycle.
`corpus_literal_sweep`'s examined population is unchanged from this bundle's baseline
(`48,699 of 51,473`).

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

- **Status:** partial. This cycle closes **0 of 346** units (bucket B, `core_rulebook`,
  unchanged at 694 of 6,701) and produces a real, deeper 13-shape sub-partition of the largest
  open sub-cause (`engine_effect_token_present`, 121), pinpointing that 103 of those 121 are the
  SAME already-ratified "count grounds, the choice is not modelled" pattern used for
  Fighter/Cavalier/Brawler/Arcane-bloodline bonus feats elsewhere in this engine — not a new
  unclassified shape. AT-34-E3-001 as a whole does NOT close this cycle: the other eight
  mechanisms are owned by other cycles and are not this cycle's scope.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0 (no unit's evidence string changed this cycle; the finding above is a
  candidate reclassification for a FUTURE cycle or operator ruling, not one applied here).
- **Reachability:** 0 (no new engine table or fallback consulted this cycle).
- **Instrument-correction:** 0 (the population re-derived cleanly to the same 346; no wrong prior
  claim was found — the prior cycles' 121-unit pile was correct, just unbroken).

## Notes

- This cycle is a **measurement wave that banks zero units** (`decisions.md §12` L6 /
  `workflow-instruction.md §12` row 6) — legitimate because it adds real, verifiable structure
  (a 13-shape, sum-exact partition of the previously-flat 121, tied to an existing, cited,
  ratified architectural precedent) rather than repeating a prior cycle's claim unchecked.
- The temporary debug instrumentation used to derive the per-key breakdown was reverted before
  this cycle's commit; `git diff --stat src/rules_core/class_feature_pool_catalog.rs` against
  this cycle's own HEAD is empty.
- **Remainder, named by sub-cause, summing exactly to 346** (the deliverable this cycle
  refines):

| Sub-cause | Units | What closes it |
|---|---:|---|
| `description_is_null_internal_bookkeeping` | 143 | Operator-scoped ruling (unchanged from prior cycle): does a zero-description internal-bookkeeping row ever satisfy bucket B? |
| `engine_effect_token_present` — of which: | 121 | — |
| &nbsp;&nbsp;`Sorcerer Bloodline Feat ~ *` | 87 | Operator-scoped ruling (NEW, this cycle): should a per-option enumeration record for a "count grounds, choice not modelled" slot (the SAME ratified pattern already used for Fighter/Cavalier/Brawler/Arcane-bloodline bonus feats) be scored in bucket B at all, or reclassified alongside Fighter's non-ingested equivalent — plus, separately, real slot-count-magnitude engine wiring for the Sorcerer bloodline-feat count itself. |
| &nbsp;&nbsp;`Ranger Combat Style Feat ~ *` | 16 | Same ruling as above, Ranger's combat-style-feat slots. |
| &nbsp;&nbsp;Long tail (`Rogue Talent`, `Wizard`, `Core Domain`, `Monk`, `Shadowdancer`, `Assassin`, `Cleric`, `Duelist`, `Nobility Domain`, `Sorcerer Bonus Spell L3`, `Sorcerer`) | 18 | Real engine wiring, one mechanical shape at a time (domain spell-list grants, opposition-school bookkeeping, bonus-spell-slot grants, ...) — each independently scoped. |
| `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` | 67 | Real per-character computation for the specific `wiring_class`/`universal_sheet_modifier` shape each record actually needs (verified genuine, not a gate bug, across two prior cycles' sampling). |
| `class_specific_level_phrase` | 6 | Real per-character, class-level-scaled computation — confirmed to have no sibling `DONE` unit anywhere in the corpus. |
| `dropped_pcgen_args` | 5 | Real per-character argument resolution this static catalog cannot perform. |
| `multi_desc_segment_not_regenerated` | 3 | New engine support for showing only the character's actual mutually-exclusive branch. |
| `bare_percent_reference` | 1 | Same as `dropped_pcgen_args` — a real per-character reference this catalog cannot resolve. |
| **Total** | **346** | — |

## Next-cycle plan

Two live paths, either can be picked up independently:
1. **Operator-scoped ruling** on whether a "count grounds, choice not modelled" per-option
   enumeration record (103 of 346, `Sorcerer Bloodline Feat`/`Ranger Combat Style Feat`) is a
   valid bucket-B target at all — parallel to the still-open 143-unit
   `description_is_null_internal_bookkeeping` question from the prior cycle. Both questions
   together cover 246 of 346 (71%) of this mechanism's remaining population.
2. **Real engine wiring**, one shape at a time, for any of: the 67-unit
   `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` sub-cause (each an
   Epic-2-table-sized project per prior cycle's sampling), the 18-unit long tail inside
   `engine_effect_token_present` (smallest, most tractable individually), or the 15-unit
   remainder of `class_specific_level_phrase`/`dropped_pcgen_args`/
   `multi_desc_segment_not_regenerated`/`bare_percent_reference`.
