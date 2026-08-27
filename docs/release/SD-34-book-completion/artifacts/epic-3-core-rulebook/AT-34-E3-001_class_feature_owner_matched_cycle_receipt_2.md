# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 2)

- **Commit SHA:** see below (this receipt's own commit).
- **Files touched:** this receipt only (`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_2.md`), plus the standing three shared files (`progress.md`, `kanban.md`). No production code changed.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (the epic-scoped diff against `origin/develop`
  carries pre-existing `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` **data-field
  values** inside `docs/work-inventory.json` from earlier, already-audited cycles — not a token
  this cycle introduced; `git diff --stat` for this cycle's own commit shows only the
  markdown files above).
- **Wired-integration audit result:** OK_NO_TOKENS (the epic-scoped diff's `placeholder` hits
  are all pre-existing, already-audited occurrences — the real PCGen "no selection" CHOOSE-menu
  placeholder-row content named by the `class_feature_option_pool` cycle's own fix, and this
  mechanism's own prior receipt's prose. Nothing introduced this cycle.)
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
→ **346** at this cycle's HEAD (`251ad7929a`), unchanged from the prior cycle's receipt. Also
re-verified via the prior cycle's own committed regression test:

```
cargo test --locked --lib rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly -- --nocapture
...
AT-34-E3-001 class_feature_owner_matched sub-cause: 1 | bare_percent_reference
AT-34-E3-001 class_feature_owner_matched sub-cause: 67 | catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion
AT-34-E3-001 class_feature_owner_matched sub-cause: 6 | class_specific_level_phrase
AT-34-E3-001 class_feature_owner_matched sub-cause: 143 | description_is_null_internal_bookkeeping
AT-34-E3-001 class_feature_owner_matched sub-cause: 5 | dropped_pcgen_args
AT-34-E3-001 class_feature_owner_matched sub-cause: 121 | engine_effect_token_present
AT-34-E3-001 class_feature_owner_matched sub-cause: 3 | multi_desc_segment_not_regenerated
test ... ok
```
143+121+67+6+5+3+1 = 346. Partition unchanged from the prior cycle.

## This cycle's own investigation — one narrow-fix hypothesis tested, ruled out

The prior cycle's receipt (`AT-34-E3-001_class_feature_owner_matched_cycle_receipt.md`) already
hand-walked the same seven render-and-refuse gates in `class_feature_pool_catalog.rs` against
the code at `v06_work_inventory.rs:10060-10129` and found every gate deliberate, correct, and
pre-existing Decision-7 architecture (`SD31-D7-PROSE-004`). Re-reading that code this cycle
(`src/bin/v06_work_inventory.rs:10060-10129`, `src/rules_core/class_feature_pool_catalog.rs`
lines 380-420, 550-610, 1356-1490) confirms the same seven gates, unchanged, in the same order,
with the same behavior.

This cycle tested one hypothesis the prior receipt did not: **is any of the 346 a genuine atlas
defect** — i.e. does a *different*, already-`DONE`/computed unit already cover the SAME feature
under a different corpus key, making this bucket-B entry a duplicate enumeration rather than a
real gap (`decisions.md §2`)? Sampled the `class_specific_level_phrase` sub-cause's cited example
(`Arcane Bond ~ Bonded Object`, "200 gp per wizard level"):

```bash
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
for u in d['units']:
    if u.get('book')=='core_rulebook' and 'Arcane Bond' in (u.get('corpus_key') or ''):
        print(u.get('corpus_key'), u.get('status'), u.get('evidence'))
"
```
Output:
```
Arcane Bloodline ~ Arcane Bond   engine-does-not-hold  no_explanation_id_and_no_diagnostic_names_this_feature
Arcane Bond ~ Bonded Object      engine-does-not-hold  class_feature_owner_matched_by_name_but_record_not_held_by_engine
Arcane Bond ~ Familiar           engine-does-not-hold  no_explanation_id_and_no_diagnostic_names_this_feature
Wizard ~ Arcane Bond             engine-does-not-hold  no_explanation_id_and_no_diagnostic_names_this_feature
```
No sibling record for this feature is `DONE` or otherwise computed — every related unit is
independently `engine-does-not-hold`. **Not an atlas defect; a real, unbuilt computation.** This
rules out one candidate narrow-fix path (finding a mis-partitioned duplicate) rather than
providing one; it does not move any of the 346.

## Why 0 of 346 can move this cycle

Every one of the seven sub-causes requires either (a) new engine wiring for a genuinely
mechanical or level-scaled record (spellcasting grants, domain spell-list grants, bonus-feat
grants, proficiency grants, per-character size-bonus effects — each independently scoped,
comparable to a single Epic 2 table build), or (b) new ingest work writing a description that
does not exist upstream for the corpus record, or (c) an operator-scoped ruling on whether a
zero-description internal-bookkeeping row can ever satisfy bucket B at all
(`description_is_null_internal_bookkeeping`, 143 units — the prior receipt's own open
next-cycle question). None of the three is a narrow, safely-scoped fix inside this mechanism's
own catalog-consulting lane; attempting one without the underlying engine capability would
either loosen a deliberate Decision-7 render-and-refuse gate (misreporting a still-uncomputed
mechanic as served) or hand-author corpus prose outside the guarded ingest path
(`workflow-instruction.md §6`'s "never hand-edit `data/corpus/**`" rule — these are corpus
*description* fields, the same guarded-generator-only class).

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 346 (command above, denominator: `core_rulebook`
  units with `status=='engine-does-not-hold'` and this evidence string).
- **Sub-cause partition:** 143+121+67+6+5+3+1 = 346 (command above).
- **Bucket B, `core_rulebook` (atlas-real partition):** unchanged at 750 of 6,701 —
  `python3 scripts/completion_atlas.py --by-book`.
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `violations=0`.

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
  → exit 0, run at this cycle's HEAD `251ad7929a`.
- `apps/desktop/src-tauri`: not touched this cycle (no production or test file changed); not
  re-run — no change in this crate's compile surface since the prior cycle's own green run.

## Sweep population

N/A — no corpus record added, regenerated, or otherwise touched this cycle.
`corpus_literal_sweep`'s examined population is unchanged from this bundle's baseline
(`48,699 of 51,473`).

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

- **Status:** partial. This cycle closes **0 of 346** units (bucket B, `core_rulebook`,
  unchanged at 750 of 6,701) and confirms, by independent re-derivation at this cycle's own
  HEAD plus one new falsified hypothesis (no duplicate/atlas-defect path exists), the prior
  cycle's exact 7-way, sum-exact 346-unit partition. AT-34-E3-001 as a whole does NOT close
  this cycle: the other eight mechanisms are owned by other cycles and are not this cycle's
  scope.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0 (no unit's evidence string changed).
- **Reachability:** 0 (no new engine table or fallback consulted this cycle).
- **Instrument-correction:** 0 (the population re-derived cleanly to the same 346; the
  duplicate-unit hypothesis tested this cycle was falsified, not confirmed, so no correction
  event is warranted — `AGENTS.md`'s `--verified-by` bar for a `correction` event requires an
  actual wrong claim, and none was found).

## Notes

- This cycle is a **measurement wave that banks zero units** (`decisions.md §12` L… / `workflow-
  instruction.md §12` row 6) — a legitimate outcome, not a stall, because it independently
  re-verifies the population and partition at a newer HEAD and tests (and rules out) a real
  alternative closure path rather than repeating the prior cycle's own claim unchecked.
- **Remainder, named by sub-cause, summing exactly to 346** (the deliverable this cycle
  reconfirms):

| Sub-cause | Units | What closes it |
|---|---:|---|
| `description_is_null_internal_bookkeeping` | 143 | Operator-scoped ruling: does a zero-description internal-bookkeeping row (`ADD:SPELLCASTER`/`SPELLKNOWN`/`SPELLLEVEL` rows with no `DESC:`) ever satisfy bucket B, or should these be reclassified as a non-player-facing kind-A shape? Not decidable by this cycle. |
| `engine_effect_token_present` | 121 | Real engine wiring, one mechanical shape at a time (spellcaster grants, domain spell-list grants, bonus-feat grants, proficiency grants — each an Epic-2-table-sized project). |
| `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` | 67 | Real per-character computation for the specific `wiring_class`/`universal_sheet_modifier` shape each record actually needs (verified genuine, not a gate bug, on sampled records both this cycle and the prior one). |
| `class_specific_level_phrase` | 6 | Real per-character, class-level-scaled computation (e.g. "200 gp per wizard level" formulas) — confirmed this cycle to have no sibling `DONE` unit anywhere in the corpus. |
| `dropped_pcgen_args` | 5 | Real per-character argument resolution this static catalog cannot perform. |
| `multi_desc_segment_not_regenerated` | 3 | New engine support for showing only the character's actual mutually-exclusive branch, not all branches at once. |
| `bare_percent_reference` | 1 | Same as `dropped_pcgen_args` — a real per-character reference this catalog cannot resolve. |
| **Total** | **346** | — |

## Next-cycle plan

Unchanged from the prior receipt's own next-cycle plan: the 143-unit bookkeeping question needs
an operator-scoped classification ruling (whether a zero-description row can ever be bucket-B
eligible); the remaining 203 units split into five real, separately-scoped engine-wiring
projects. A future cycle should pick exactly one shape (proficiency grants is the cleanest,
narrowest of the five) and build real wiring for it — this cycle's scope did not extend to
building new production engine capability, only to re-verifying the population and testing one
falsifiable narrow-fix hypothesis.
