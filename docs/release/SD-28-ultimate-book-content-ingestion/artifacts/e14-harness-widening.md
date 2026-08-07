# SD28-E14 — Observation-harness widening (spell + equipment consumers)

Cycle: `epic-14-harness`, actor `epic-14-harness`. Verified HEAD stated at
close (`progress.md` receipt carries the exact commit).

**Correction recorded, 2026-08-06 (`docs/retro/events/epic-14-harness.jsonl`):**
this cycle originally shipped a spell probe that promoted 1,067 units to
`grounded`. Independent review (team-lead) found the probe measured spell
*resolution*, not a *magnitude*, and the promotions were reverted before
this receipt's numbers below. The corrected disposition is: **F2 (equipment)
shipped as designed; F1 (spell) did not ship a promoting probe at all, and
that absence is itself the recorded finding.** See "F1 — what actually
happened" below for the full account; it is kept rather than deleted so the
next agent does not re-attempt the same shape.

## What landed

One new probe in `src/bin/v06_work_inventory.rs`, the same shape as the
existing `probe_feat_effect_wiring`: it runs a unit through the **real
compute pipeline** and only promotes a unit `classify()` would otherwise
stop at `ingested-magnitude` when a real consumer is observed to produce a
computed delta from it.

- **F2 — `probe_equipment_effect_wiring`** (+ its pure helper
  `equipment_key_is_wired`). Loads every equipment record it can reach from
  the real on-disk `data/corpus/<book>/equipment/*.json` corpus via the
  existing `corpus_loader::load_equipment_corpus`, equips the item alone,
  and runs it through the existing `equipment_effects::compute_equipment_effects`
  — the same pipeline `pilot_compute_corpus::compute_pilot_with_corpus`
  already calls for a real character. An item is wired only when at least
  one of its per-item stat fields (`armor_class_bonus`, `max_dex`,
  `spell_failure`, `armor_check_penalty`, `skill_bonus`, `ability_bonus`,
  `weapon_enhancement_bonus`) is `Some` — i.e. its own corpus record carries
  a mechanical token the engine's existing resolvers read.

`classify()`'s `Kind::Equipment`/`Kind::EquipmentModifier` arm consults
`facts.equipment_effect_wired` before falling through to
`ingested-magnitude`, strictly after the existing `text_only` check. The
`Kind::Spell` arm is **unchanged from before this epic** — see below.

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
`forward-scope-register.md` C3.1 corrected accordingly.

## F1 — what actually happened, and why no spell probe shipped

The first version of this cycle built `probe_spell_effect_wiring`: load real
on-disk spell JSON via a new `corpus_loader::load_spell_corpus` (kept — see
below), select the spell through `CharacterInput.chosen.spells_selected`,
run it through `compute_pilot_with_corpus`, and promote when the spell
resolved (not in `unresolved_spell_ids`) and landed in a non-empty
`school_coverage` entry. It promoted **1,067 of 1,067** targeted spell units
(100%).

**That is the tell, and independent review caught it.** Reading
`pilot_compute_corpus.rs:189-205`: `school_coverage` is populated by
resolving each selected spell and reading its `school` string through
`Pf1SchoolId::from_corpus_str` — nothing else. No spell field that varies by
content (level, save DC, duration, ...) is read into any number a consumer
produces. The predicate "resolves AND has a recognized school" is true for
**every** real PF1 spell (all nine school strings are recognized, and every
CRB/APG/ACG spell with a resolved level also has a real on-disk record) —
so the probe restated `ingested-magnitude`'s own existing evidence
(`spell_list_entry_with_resolved_level`) through a compute call, not an
observation of a magnitude moving. It is precisely the gaming failure mode
F3 exists to catch, and it passed the negative test that was written
(`...never_promotes_a_spell_absent_from_the_on_disk_corpus`) because that
test only pinned a *resolution* property (spell absent from disk), not a
*magnitude* property — it could not have caught this, by construction.

**Is there a real spell-magnitude consumer anywhere in this repo?** Yes, but
it is not wired to a surface the player reads. `spellbook::compute_spellbook_coverage`
reads each resolved spell's real `level` into `SpellEffect.level` and
computes `spell_save_dc`/`slots_total`/`slots_used` from it — genuine
magnitude-bearing output. It is wired into `contract::PilotReceipt.spellbook`
(`contract.rs:397`), and from there into `sheet.spellbook.*` cells
(`contract.rs:794-810`). But `contract::build_pilot_receipt` is **never
called** by `apps/desktop/src-tauri/src/pf1_adapter.rs` or `character_hub.rs`
— confirmed: `grep -rn build_pilot_receipt apps/desktop/src-tauri/src`
returns nothing. `pf1_adapter::resolve_unified_pilot_snapshot` (the function
the desktop app actually gates on) never calls it either. This is exactly
the "twin problem" `decisions.md §29.1`/`§29.2` already names by shape: a
real, magnitude-bearing computation that never reaches the surface the
player's sheet is built from. `contract.rs` is a third, disconnected twin.

**Disposition:** all 1,067 targeted spell units stay `ingested-magnitude`.
No spell probe was shipped, because no honest one can promote any of them
today. `probe_spell_effect_wiring`/`spell_key_is_wired` and their tests were
deleted rather than left as dead or misleading code — a future cycle that
wires `contract.rs`'s spellbook output into `pf1_adapter::resolve_unified_pilot_snapshot`
would then have a real magnitude (`spellbook.slots_total`/`spell_save_dc`,
keyed by the spell's own level) to build a genuine probe against; the
`corpus_loader::load_spell_corpus` on-disk loader added this cycle is kept
(tested, real, reusable infrastructure) for exactly that future probe to use.

**Remedy, named for the next cycle:** wire `contract::build_pilot_receipt`'s
`spellbook` output (or a narrower seam that does the same job) into
`pf1_adapter::resolve_unified_pilot_snapshot`, then build a probe that
selects a spell and confirms `spellbook.slots_total`/`spell_save_dc` moves
by an amount attributable to that specific spell's `level` — not merely that
some entry appears. That is engine-wiring work (`contract.rs`/`pf1_adapter.rs`),
outside this harness-widening epic's file scope.

## Books the widened harness can actually observe

`data/corpus/` (real on-disk Shape B v1 JSON, the only source
`corpus_loader.rs` can load from) holds exactly 6 books today:
`core_rulebook`, `advanced_players_guide`, `advanced_class_guide`,
`beastiary`, `advanced_race_guide`, `pathfinder_unchained`. Re-derived via
`ls data/corpus/` at cycle time, not transcribed from a prior figure. Every
equipment unit `classify()` can reach via `Kind::Equipment`/
`Kind::EquipmentModifier` is inside this set (`facts.equipment_keys` covers
`core_rulebook`/`advanced_players_guide`/`advanced_class_guide`/`bestiary_1`
only), so no equipment unit in this epic's population is blocked purely by
book coverage.

## Before / after (re-derived, not transcribed)

Regenerated via `cargo run --locked --bin v06_work_inventory` against the
real `~/workspace/repos/pcgen/data` checkout, **after** the F1 revert.
`generated_at` on the produced `docs/work-inventory.json`:
`2026-08-06T23:05:21Z`.

```bash
python3 -c "
import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter(x['status'] for x in u)
print(dict(c))
"
```

| status | before (branch point) | after (corrected) | delta |
|---|---|---|---|
| `grounded` | 301 | 474 | **+173** |
| `ingested-magnitude` | 4,050 | 3,877 | **-173** |

`before` is the file at the branch point this cycle claimed (`git show
HEAD:docs/work-inventory.json` before regeneration, commit `c12b1905`);
`after` is the corrected, freshly regenerated file this cycle committed.
(The intermediate, superseded run with the spell probe live produced
`grounded` 1,541 / `ingested-magnitude` 2,810 — never committed as final;
recorded here only so the number does not reappear unexplained in a log.)

```bash
python3 -c "
import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
promoted=[x for x in u if x['status']=='grounded' and x.get('evidence')=='equipment_effect_probe_observed_computed_delta']
spell=[x for x in u if x['status']=='grounded' and x.get('evidence')=='spell_effect_probe_observed_computed_delta']
print('equipment promoted:', len(promoted), '| spell promoted:', len(spell))
"
```

→ `equipment promoted: 173 | spell promoted: 0`.

**173 of 2,983 equipment/equipment-modifier units promoted (5.8%).** The
remaining 2,810 stay `ingested-magnitude` — see OPEN_FINDINGS below.
**0 of 1,067 spell units promoted** — see F1 above.

## OPEN_FINDINGS — units that stay `ingested-magnitude`

### Equipment / equipment-modifier: 2,810 units

By book (re-derived from the regenerated `docs/work-inventory.json`):

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
   `grounded`.
2. **Thin on-disk records.** Per `corpus_loader.rs`'s own doc comment, a
   record without `raw_tokens`/`raw_bonus_chains` (not yet enriched by
   `scripts/enrich_equipment_raw_tokens.rs`, or a `web_second_source`/
   `same_book_fallback` record with no raw LST line to enrich from)
   reconstructs with `tokens`/`bonus_chains` empty — it resolves by name but
   the mechanical fields the resolvers read are genuinely absent from what
   is on disk. This is an ingestion-completeness gap, not a harness gap.

### Spell: 1,067 units

**Finding: no currently-wired consumer reads a spell's magnitude at all.**
Full account above ("F1 — what actually happened"). Remedy: wire
`contract::build_pilot_receipt`'s `spellbook` output into
`pf1_adapter::resolve_unified_pilot_snapshot`, then build a probe against
the real magnitude that unlocks (`spellbook.slots_total`/`spell_save_dc`).

## Anti-gaming (F3) evidence

- **Status/predicate untouched.** No change to `unit.magnitude_token_count == 0`
  (`text-complete`'s predicate), the `proven` formula, or any status
  definition/vocabulary string besides adding the one new `evidence` value
  named above (`equipment_effect_probe_observed_computed_delta`).
- **Negative tests, proven to catch a permissive probe.** The F2 probe ships
  with negative unit tests in `v06_work_inventory.rs`'s `e14_harness_tests`
  module:
  - `equipment_probe_never_promotes_a_text_only_item_with_no_mechanical_tokens`
    — a real, resolvable corpus record with no BONUS/ACCHECK token. Manually
    verified to FAIL when `equipment_key_is_wired` was temporarily replaced
    with a permissive "resolves => wired" check (`effects.per_item.first().is_some()`),
    then reverted; `cargo test` confirms the reverted, real probe passes it.
  - `equipment_probe_never_promotes_an_item_absent_from_the_corpus` — same
    shape for an unresolvable equipment key.
- **Positive control**: `equipment_probe_promotes_a_real_armor_item_with_real_ac_tokens`
  (CRB Padded Armor (Base), AC 1 / max-dex 8, matching `corpus_loader.rs`'s
  own existing test's real numbers).
- **The F1 spell probe itself is the anti-gaming criterion's other proof
  point, the negative case.** It shipped, promoted 100% of its target, was
  caught by review (not by its own negative test — see "F1" above for
  exactly why that test could not have caught it), and was fully reverted
  rather than kept partially. `corpus_loader::load_spell_corpus` stays (real,
  tested, reusable), but no promoting logic tied to it remains anywhere in
  `classify()`.

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
