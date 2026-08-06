# SD28-E14 — Observation-harness widening (spell + equipment consumers)

Cycle: `epic-14-harness`, actor `epic-14-harness`. Verified HEAD stated at
close (`progress.md` receipt carries the exact commit).

## What landed

Two new probes in `src/bin/v06_work_inventory.rs`, the same shape as the
existing `probe_feat_effect_wiring`: they run a unit through the **real
compute pipeline** and only promote a unit `classify()` would otherwise stop
at `ingested-magnitude` when a real consumer is observed to produce a
computed delta from it.

- **F1 — `probe_spell_effect_wiring`** (+ its pure helper
  `spell_key_is_wired`). Loads every spell record it can reach from the real
  on-disk `data/corpus/<book>/spell/*.json` corpus via a new
  `corpus_loader::load_spell_corpus` (the spell-side sibling of the existing
  `load_equipment_corpus`), selects the spell through
  `CharacterInput.chosen.spells_selected`, and runs it through
  `pilot_compute_corpus::compute_pilot_with_corpus` — the twin the player
  reads (`decisions.md §29.1`). A spell is wired only when it resolves (not
  in `unresolved_spell_ids`) AND lands in a real, non-empty
  `school_coverage` entry.
- **F2 — `probe_equipment_effect_wiring`** (+ its pure helper
  `equipment_key_is_wired`). Loads every equipment record it can reach from
  the real on-disk `data/corpus/<book>/equipment/*.json` corpus via the
  existing `corpus_loader::load_equipment_corpus`, equips the item alone,
  and runs it through the existing `equipment_effects::compute_equipment_effects`
  — the same pipeline `compute_pilot_with_corpus` already calls for a real
  character. An item is wired only when at least one of its per-item stat
  fields (`armor_class_bonus`, `max_dex`, `spell_failure`,
  `armor_check_penalty`, `skill_bonus`, `ability_bonus`,
  `weapon_enhancement_bonus`) is `Some` — i.e. its own corpus record carries
  a mechanical token the engine's existing resolvers read.

`classify()`'s `Kind::Spell` arm consults `facts.spell_effect_wired` before
falling through to `ingested-magnitude`; `Kind::Equipment`/
`Kind::EquipmentModifier` consults `facts.equipment_effect_wired` the same
way. Both checks happen strictly after the existing `text_only` check, and
the `text-complete` predicate itself (`unit.magnitude_token_count == 0`) is
untouched — E14 adds observation, it does not redefine any status.

**CORRECTION to the epic's own spec, verified before writing code:** the
spec's `decisions.md §10` dependency claim ("a probe over a CRB-only
`equipment_catalog.rs` can observe nothing for six other books") is stale on
two counts. First, `apps/desktop/src-tauri/src/equipment_catalog.rs` was
already widened to all six ingested books in `a92ae066`/`d44ea892` (per the
dispatching brief's own correction). Second, and more directly load-bearing
for this epic: the rules-core consumer this epic actually needed —
`equipment_effects::compute_equipment_effects` — was **already book-agnostic**
before this cycle touched it. It resolves against whatever
`SourcePackageContent` it is given and every per-category resolver
(`arms_armor`/`general`/`magic_items`/`equipmods`) already reads tokens
directly off the resolved `EquipmentRecord`, not off the CRB-only compiled
`equipment_tables()` store (confirmed by reading `equipment_effects.rs:194-236`
and its own doc comment). The real gate was not book-scoping at all — it was
that **no on-disk corpus existed** to resolve most SD-28 books' equipment
against. The desktop `equipment_catalog.rs` widening was a real, separate,
already-shipped change; it was not this epic's dependency.

## Books the widened harness can actually observe

`data/corpus/` (real on-disk Shape B v1 JSON, the only source
`corpus_loader.rs` can load from) holds exactly 6 books today:
`core_rulebook`, `advanced_players_guide`, `advanced_class_guide`,
`beastiary`, `advanced_race_guide`, `pathfinder_unchained`. Re-derived via
`ls data/corpus/` at cycle time, not transcribed from a prior figure.

Every one of the 4,050 `ingested-magnitude` units this epic targeted (per
`classify()`'s own gating: `Kind::Spell` only reaches `ingested-magnitude`
via `facts.spell_levels`, which is populated for `core_rulebook`,
`advanced_players_guide`, `advanced_class_guide` only; `Kind::Equipment`/
`Kind::EquipmentModifier` only via `facts.equipment_keys`, populated for
those three plus `bestiary_1`) is inside this reachable set. No unit in this
epic's own 4,050-unit population is blocked purely by book coverage; the
disposition below is entirely about whether the individual record carries an
observable computed delta.

## Before / after (re-derived, not transcribed)

Regenerated via `cargo run --locked --bin v06_work_inventory` against the
real `~/workspace/repos/pcgen/data` checkout. `generated_at` on the produced
`docs/work-inventory.json`: `2026-08-06T22:27:08Z`.

```bash
python3 -c "
import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter(x['status'] for x in u)
print(dict(c))
"
```

| status | before | after | delta |
|---|---|---|---|
| `grounded` | 301 | 1,541 | **+1,240** |
| `ingested-magnitude` | 4,050 | 2,810 | **-1,240** |

`before` is the file at the branch point this cycle claimed (`git show
HEAD:docs/work-inventory.json` before regeneration, same commit
`c12b1905`); `after` is the freshly regenerated file this cycle produced
and committed.

By evidence, all 1,240 promotions attributable to E14 alone:

```bash
python3 -c "
import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
promoted=[x for x in u if x['status']=='grounded' and x.get('evidence') in
    ('spell_effect_probe_observed_computed_delta','equipment_effect_probe_observed_computed_delta')]
print(len(promoted), collections.Counter(x['evidence'] for x in promoted))
"
```

→ `1240 {'spell_effect_probe_observed_computed_delta': 1067, 'equipment_effect_probe_observed_computed_delta': 173}`.

**Every one of the 1,067 spell units promoted (100%).** Every CRB/APG/ACG
spell with a resolved level also has a real on-disk `spell/*.json` record
and a real, recognized school string, so `spell_key_is_wired` observes a
`school_coverage` delta for all of them.

**173 of 2,983 equipment/equipment-modifier units promoted.** The remaining
2,810 stay `ingested-magnitude` — see OPEN_FINDINGS below.

## OPEN_FINDINGS — units that stay `ingested-magnitude`

All 2,810 remaining `ingested-magnitude` equipment/equipment-modifier units,
by book (re-derived from the regenerated `docs/work-inventory.json`):

| book | remaining ingested-magnitude |
|---|---|
| `core_rulebook` | 2,255 |
| `advanced_players_guide` | 337 |
| `advanced_class_guide` | 214 |
| `bestiary` (engine id `bestiary_1`) | 4 |
| **total** | **2,810** |

**Finding: every one of these resolves against the real on-disk corpus (the
harness reaches them — the book-coverage gap named above does not apply
here), but produces no per-item mechanical-token delta
`compute_equipment_effects` can read.** Two honest sub-causes, not
distinguished per-unit by this cycle (a further probe refinement, not a
blocker):

1. **Genuinely non-mechanical items.** The large majority: trade goods,
   containers, tools, alchemical ingredients, plot items, and other
   equipment whose real PF1 rules text carries no AC/max-Dex/spell-failure/
   armor-check-penalty/skill/ability-score/weapon-enhancement bonus at all.
   For these there is no computed delta to observe because the *rule*, not
   the harness, has none — the correct status is `ingested-magnitude`, not
   `grounded`, and it is not this epic's job to invent one.
2. **Thin on-disk records.** Per `corpus_loader.rs`'s own doc comment, a
   record without `raw_tokens`/`raw_bonus_chains` (not yet enriched by
   `scripts/enrich_equipment_raw_tokens.rs`, or a `web_second_source`/
   `same_book_fallback` record with no raw LST line to enrich from)
   reconstructs with `tokens`/`bonus_chains` empty — it resolves by name but
   the mechanical fields the resolvers read are genuinely absent from what
   is on disk, even for an item whose real rules text does carry a bonus.
   This is an ingestion-completeness gap (whether `raw_tokens` enrichment
   has reached that record), not a harness gap: E14's job was to WIRE a
   consumer to whatever tokens exist, not to re-run enrichment.

Neither sub-cause is a defect in the F1/F2 probes themselves (proven by the
negative tests below), and neither is masked: every one of the 2,810 stays
`ingested-magnitude`, none is promoted, and this file names the disposition
for the whole remaining population rather than only a sample.

## Anti-gaming (F3) evidence

- **Status/predicate untouched.** No change to `unit.magnitude_token_count == 0`
  (`text-complete`'s predicate), the `proven` formula, or any status
  definition/vocabulary string besides adding the two new `evidence` values
  named above.
- **Negative tests, proven to catch a permissive probe.** Both new probes
  ship with a negative unit test in `v06_work_inventory.rs`'s
  `e14_harness_tests` module:
  - `equipment_probe_never_promotes_a_text_only_item_with_no_mechanical_tokens`
    — a real, resolvable corpus record with no BONUS/ACCHECK token. Manually
    verified to FAIL when `equipment_key_is_wired` was temporarily replaced
    with a permissive "resolves => wired" check (`effects.per_item.first().is_some()`),
    then reverted; `cargo test` confirms the reverted, real probe passes it.
  - `spell_probe_never_promotes_a_spell_absent_from_the_on_disk_corpus` — a
    real `crb_spell_list::SPELL_LIST` key run against an intentionally empty
    corpus (stands in for "table says resolved, on-disk record absent").
  - `equipment_probe_never_promotes_an_item_absent_from_the_corpus` — same
    shape for an unresolvable equipment key.
- **Positive controls**, so the negatives are not merely "the probe found
  nothing at all": `equipment_probe_promotes_a_real_armor_item_with_real_ac_tokens`
  (CRB Padded Armor (Base), AC 1 / max-dex 8, matching
  `corpus_loader.rs`'s own existing test's real numbers) and
  `spell_probe_promotes_a_real_on_disk_spell` (CRB Animate Plants).

## Generator invocation (re-run to reproduce)

```bash
export PCGEN_CORPUS_ROOT=~/workspace/repos/pcgen/data   # default, shown for clarity
cargo run --locked --bin v06_work_inventory
```

Test invocation:

```bash
cargo test --bin v06_work_inventory e14_harness_tests
cargo test --lib rules_core::corpus_loader
```
