# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism)

- **Commit SHA:** `f8a230d4f398e49e3288cf51cfe63542342bd934` (parent `0009b7ca2197c44caf30d664f0c71fa1e819389d`).
- **Files touched:**
  - `src/rules_core/class_feature_pool_catalog.rs` — one new, committed, passing regression
    test: `class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`.
    No production code changed. The test re-derives this mechanism's 346-unit `core_rulebook`
    population directly from the live `docs/work-inventory.json`, walks the SAME gates
    `load_class_feature_catalog`'s filter already runs (in the same order), and asserts the
    resulting sub-cause tally sums exactly to the population — a committed, re-runnable proof
    of this cycle's finding rather than a one-off investigation that decays.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS (the diff's only hits are pre-existing
  `placeholder` occurrences from prior cycles' own corpus-shape prose, none introduced this
  cycle — verified by `git diff --stat` showing only `src/rules_core/class_feature_pool_catalog.rs`
  as this cycle's change).
- **Acceptance criterion:** AT-34-E3-001 — bucket B closes: records reach their tables — this
  cycle owns exactly mechanism 4 of 9,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`.

## Re-derived population (do not trust the brief's number without checking)

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
→ **346** (matches the dispatch brief's own re-derived figure, `decisions.md §14`'s table, and
the `sd-34-dispatch.workflow.js` note verbatim — not merely quoted).

## Investigation: why none of the 346 is a narrow catalog-widening bug

`Kind::ClassFeature`'s "owner resolved" branch (`v06_work_inventory.rs`, the
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` evidence site) already
consults `facts.class_feature_pool_catalog_holds` before falling back to `engine-does-not-hold`
— `class_feature_pool_catalog::load_pool_catalog`'s `is_registered_pool_group` was ALREADY
widened (SD-32 T12) to accept ANY `" ~ "`-qualified key, so the catalog itself is not narrow.
Re-running that catalog's own filter, gate by gate and in the SAME order, against every one of
the 346 units (test above) produces an EXACT partition:

| Sub-cause | Units | Why not a catalog-widening fix |
|---|---:|---|
| `description_is_null_internal_bookkeeping` | 143 | The corpus record's `data.description` is `null` — no `DESC:` token exists at all (pure `ADD:SPELLCASTER`/`SPELLKNOWN`/`SPELLLEVEL` bookkeeping rows, e.g. `Sorcerer Class`, `Sorcerer Domain ~ Air`, `Wizard ~ Cantrips`). There is no player-facing prose to serve; `is_real_description_value` correctly never sees a real string. Real ingest work (a description does not exist upstream to serve) or a reclassification of these rows as non-player-facing, not a catalog fix. |
| `engine_effect_token_present` | 121 | Carries a real `ENGINE_EFFECT_TOKEN_KEYS` token (`ADD`, `ABILITY`, `AUTO`, `BONUS`, `DEFINE`, `SPELLS`, …) alongside its description — e.g. `Assassin ~ Weapon and Armor Proficiency`'s `AUTO`/`ABILITY` tokens. Decision 7 condition 1 ("prose only, not a mechanic") genuinely fails; these need real engine wiring (the mechanic computed), not a text serve. |
| `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` | 67 | Already present in `load_pool_catalog`'s index (every render-and-refuse gate passes) but `classify()`'s OWN promotion gate — `is_display_wiring_class_for_promotion(wc_class)` requiring `wc_class == "display"`, or `closure_states_universal_sheet_modifier`'s `"size bonus"` cue — blocks it. Hand-sampled: `Sorcerer Bonus Spell L1 ~ Bless` (`wiring_class: "computed"`, `tempbonus` signal); `Sorcerer Bonus Spell L4 ~ Elemental Body I` (`wiring_class: "display"` but its prose states "+2 size bonus to your Dexterity" four times, tripping `universal_sheet_modifier`). Both gates are deliberate, pre-existing Decision-7 correctness checks (`SD31-D7-PROSE-004`) — promoting either shape to `text-complete` would misreport a record that still needs a real per-character computation as merely displayed. |
| `class_specific_level_phrase` | 6 | Prose literally states a value scaling with the owning class's level (e.g. `Arcane Bond ~ Bonded Object`: "200 gp per wizard level") — Decision 7 condition 2 ("nothing to compute") genuinely fails. |
| `dropped_pcgen_args` | 5 | `render_pcgen_desc` reports an unresolved `%N` argument this catalog has no character to resolve — a real remaining computation. |
| `multi_desc_segment_not_regenerated` | 3 | Multiple `DESC:` rows, hand-checked this cycle: `Cleric ~ Spontaneous Casting` (alignment-conditional alternate branches), `Elemental Bloodline ~ Bloodline Arcana` and `Monk ~ Flurry of Blows` (`PREVARGTEQ`/level-banded mutually-exclusive branches). None is the `class_feature_option_pool` cycle's safe sequential-continuation shape — joining any of these would show every mutually-exclusive branch at once, the exact over-disclosure defect that gate exists to prevent. |
| `bare_percent_reference` | 1 | A `%N` reference with no `|`-tail this catalog cannot resolve. |
| **Total** | **346** | — |

Every one of the seven gates above is pre-existing, load-bearing safety architecture built by
earlier cycles specifically to prevent the shortcut this mechanism's shape invites (serving a
genuinely mechanical or level-scaled record as if it were static flavor text). None of the 346
units can move to `text-complete`/`ingested-magnitude` without either (a) new engine wiring for a
genuinely mechanical/computed record (spellcaster grants, domain spell lists, bonus-feat lists,
proficiency grants, size-bonus shapeshifting effects), or (b) new ingest work writing a
description for a record that currently has none. Both are real, larger engine/ingest projects,
not something this narrowly-scoped catalog-consulting cycle can safely do without risking exactly
the "green code gate over a hole on the screen" defect Decision 7 exists to prevent.

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 346 — re-derive command above.
- **Sub-cause partition:** 143+121+67+6+5+3+1 = 346 — `cargo test --lib
  rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly
  -- --nocapture`.
- **Bucket B, `core_rulebook` (atlas-real partition):** unchanged this cycle at 762 of 6,701
  (no unit moved) — `python3 scripts/completion_atlas.py --by-book`.
- **`completion_atlas.py --check`:** `citation_failures=0` (no `v06_work_inventory.rs` line
  shifted this cycle; no corpus data regenerated; `docs/work-inventory.json` untouched).
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `violations=0`.

## Row-count command output

```
cargo test --lib rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly -- --nocapture
...
AT-34-E3-001 class_feature_owner_matched sub-cause: 1 | bare_percent_reference
AT-34-E3-001 class_feature_owner_matched sub-cause: 67 | catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion
AT-34-E3-001 class_feature_owner_matched sub-cause: 6 | class_specific_level_phrase
AT-34-E3-001 class_feature_owner_matched sub-cause: 143 | description_is_null_internal_bookkeeping
AT-34-E3-001 class_feature_owner_matched sub-cause: 5 | dropped_pcgen_args
AT-34-E3-001 class_feature_owner_matched sub-cause: 121 | engine_effect_token_present
AT-34-E3-001 class_feature_owner_matched sub-cause: 3 | multi_desc_segment_not_regenerated
test rules_core::class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly ... ok
test result: ok. 1 passed; 0 failed
```

## Build scope verified

- `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`)
  → exit 0, at this cycle's HEAD.
- `apps/desktop/src-tauri`: `cargo test --locked --no-run` (separate workspace,
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`) → exit 0.
- `cargo test --lib rules_core::class_feature_pool_catalog::` → 30 passed, 0 failed (includes
  every pre-existing test in this file plus this cycle's new one).

## Sweep population

N/A — no corpus record added, regenerated, or otherwise touched this cycle.
`corpus_literal_sweep`'s examined population is therefore unchanged from this bundle's baseline
(`48,699 of 51,473`).

## Oracle pin

N/A — no figure in this receipt was derived from the pinned PCGen oracle corpus; every number
is derived from `docs/work-inventory.json` and the live `data/corpus/core_rulebook/` checkout.

- **Status:** partial. This cycle closes **0 of 346** units (bucket B, `core_rulebook`,
  unchanged at 762 of 6,701) and **names every one of the remaining 346 by an exact, mutually
  exclusive, sum-exact sub-cause**, proven by a committed, passing regression test rather than
  asserted in prose alone. Every remaining unit needs real engine wiring or new ingest work — a
  larger scope than this narrowly-bounded catalog-consulting cycle can safely take on without
  risking a Decision-7 over-claim. AT-34-E3-001 as a whole does NOT close this cycle: 3 of 9
  named mechanisms remain unstarted or unfinished by prior cycles (`class_feature_option_pool_record_not_held_by_engine`
  55 remaining, `companion_absent_from_core_rulebook_companion_tables` 28 remaining,
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine` 333) plus this
  mechanism's own 346, all still open.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0 (no unit's evidence string changed).
- **Reachability:** 0 (no new engine table or fallback consulted; `class_feature_pool_catalog`
  and `class_feature_standalone_catalog` were already wired into `classify()` by earlier
  cycles).
- **Instrument-correction:** 0 (no prior figure was found wrong; the population re-derived
  cleanly to the same 346 the dispatch brief already carried).

## Notes

- The obvious-looking shortcut — relax `class_feature_pool_catalog`'s render-and-refuse gates
  so more of the 346 pass — was deliberately NOT taken. Every one of those gates (engine-effect
  token, class-specific-level-phrase, archetype-lock, multi-DESC, bare-`%N`, dropped-pcgen-arg)
  was hand-built by an earlier cycle against a real, hand-verified corpus finding (this file's
  own doc comments cite each). Loosening any of them to close more of this mechanism would
  reopen the exact defect they exist to prevent: serving a genuinely mechanical or level-scaled
  record to a player as if it were static, complete prose.
- The 67-unit `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` sub-cause
  looked the most promising for a real code fix (the record IS already in the catalog). Two
  hand-sampled records showed both of `classify()`'s two gates firing correctly and for the
  right, Decision-7-mandated reason (a `wiring_class` other than `display`, and a genuine
  per-character "size bonus" cue) — not a bug in either gate, so no code change was made.
- No `docs/work-inventory.json` regeneration was run this cycle (no verdict changed for any
  unit), so no `--allow-stamp-loss` risk and no `completion_atlas.py` citation re-derivation was
  needed.

## Next-cycle plan

This mechanism's 346 units split into two real follow-on projects, neither of which this cycle
started (naming them is this cycle's deliverable, not a promise of future scope):

1. **143 null-description internal-bookkeeping rows** — an ingest-territory question first:
   should `ADD:SPELLCASTER`/`SPELLKNOWN`/`SPELLLEVEL`-only rows with `VISIBLE:NO`/no `DESC:`
   even be enumerated as player-facing `class_feature` content units at all, or reclassified as
   a kind bucket A/pure-internal shape? Needs an operator-scoped ruling on whether "engine holds
   a record with no description" can ever satisfy bucket B for a unit with no prose to show —
   or whether these should never have been counted as bucket-B-eligible content units in the
   first place.
2. **121 engine-effect-token + 67 wiring-class-gated + 6 level-phrase + 5 dropped-args + 3
   multi-desc + 1 bare-percent (203 units)** — genuine engine wiring: bonus-spell-known grants
   (`SPELLKNOWN`), domain spell-list grants (`SPELLLEVEL`), bonus-feat-list grants (`ABILITY`),
   proficiency grants (`AUTO`), and level-scaled item-cost/size-bonus computations. Each is a
   real, separately-scoped feature-computation project (comparable in size to a single Epic 2
   simple-kind table), not a one-cycle catalog widening. A future cycle should pick ONE of these
   five shapes (e.g. proficiency grants, the largest clean sub-shape within the 121) and build
   real wiring for it, the same way Epic 2 built one table per kind.
