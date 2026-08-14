# Receipt — the `class_feature` consumer-delta probe

Actor `closure-cf`. Card `probe-computed-class-feature`. Branch `tranche/9`, base `e702fa80`
(rebased onto the sibling `probe-race-and-class` work mid-run).

Filed here rather than in `docs/release/SD-30-class-feature-archetype-bundle/progress.md`
because the sibling agent `sd30-refresh` holds that package concurrently. That package's
docs are untouched by this run.

## Headline

**`done` movement: +2. class_feature 18 → 20. Total DONE 3,464 → 3,466.**

Reporting `done`, never `grounded`, as the card requires. `grounded` moved +4 — twice the
`done` figure — because two of the four newly-grounded units are `wiring_class: static`,
and `static` has no `done` rung at any status. They are reported as grounded and not as
done.

```
python3 docs/retro/closure-derived-doneness-delta.py 66a6804d d6964374
#   66a6804d  DONE=3464   class_feature 18
#   d6964374  DONE=3466   class_feature 20
#   DELTA vs 66a6804d: +2 done  {'class_feature': 2}
```

Per-kind: class_feature 18 → 20. Every other kind unchanged.

## Corrections to the brief

### 1. class_feature has no `held` bucket at all — its recoverable mass is `unknown`

The brief's central premise is that ~9,455 `held` units have real engine data awaiting a
confirming instrument, and that class_feature's 4,178 `computed` units are the biggest
single lever. The 4,178 figure is right. The premise behind it is not.

```
python3 -c "…Counter(status for kind==class_feature, wiring_class==computed)…"
#   not-ingested 1859 · unknown 1804 · not-started 467 · deferred-with-reason 30 · grounded 18
```

There is **no `ingested-magnitude` class_feature unit anywhere in the corpus**, so there is
no "engine holds a magnitude, consumer unconfirmed" population to sweep — which is exactly
what the spell probe converted:

```
python3 -c "…Counter(kind for status=='ingested-magnitude')…"
#   equipment 4802 · spell 637 · equipment_modifier 456     # and nothing else
```

Of the 4,178, **2,326 are behind ingestion** (`not-ingested` + `not-started`), which the
card explicitly excludes; 30 are engine-diagnostic deferrals; 18 were already grounded.
The only addressable mass is the **1,804 `unknown`**, whose group prefix names an option
pool rather than a class. Corpus-wide that bucket is 3,218 units — 91% of the whole
`unknown` population the brief calls "unmeasurable; likely partly unreachable".

### 2. The engine models the pools, but 431 of them count picks rather than apply records

Of 15,225 distinct class_feature corpus keys, 672 map to an option-pool choice slot the
engine really declares. The probe's verdict on them:

```
./target/release/v06_work_inventory --class-feature-probe
#   keys examined 15,225
#   wired                                      4
#   delta_not_attributable_to_the_record     431
#   no_consumer_delta                        237
#   no_choice_slot_offers_it              14,553
```

The 431 are the number this run declined, and they are the most important figure in this
receipt. A probe comparing only against a baseline would have promoted every one:

```
declined as not-attributable, by pool:
  169 Rage Power · 130 Rogue Talent · 54 Unchained Rage Power · 36 Favored Enemy
   18 Favored Terrain · 15 Mercy · 9 Versatile Performance

the facts those selections moved identically for both members:
  223 class_chassis.barbarian.rage_power_choice
  130 class_chassis.rogue.talent_choice
   36 class_chassis.ranger.favored_enemy_choice
   18 class_chassis.ranger.favored_terrain_choice
   15 class_chassis.paladin.mercy_choice
    9 class_chassis.bard.versatile_performance_choice
```

Every one of those explanation records carries `value: 0` and names the chosen power only
in free-text `detail`. `observable_facts` reads `(id, value)` and never `detail`. The
engine says so about itself, in the two seams that own the largest two buckets:

- `BARBARIAN_RAGE_POWER_SLOTS`: *"open-ended recognitions fabricating NOTHING about any
  power's effect"*.
- the paladin mercy seam: *"the mercy's own effect is not computed, since no lay-on-hands
  execution engine exists anywhere in this codebase"*.

Crediting those 431 would have reported a number no player can see. The control — a
**different real member of the same corpus pool**, never a synthetic id an open-ended slot
would simply echo — is what makes the refusal mechanical rather than a judgement call.

### 3. Two defects in the probe's own first draft, both of which UNDER-reported

The first run of this probe returned **0 wired** and would have shipped a clean, confident,
wrong ceiling reading "the engine applies no option-pool record". Both defects were caught
by tests written against engine source rather than by inspection:

1. **Namespaced selection ids.** `choice_selection(input, "choice:cleric_domain")` matches
   `domain:good`, never bare `good`. The draft passed bare slugs, so every namespaced
   consumer silently ignored them. `every_namespaced_pool_uses_a_namespace_the_engine_
   source_writes` failed on `mercy:` (genuinely open-ended) and caught `focus:` where the
   engine writes `animal_focus:`.
2. **Canonical seeds pre-occupying the slot under test.** `canonical_seeds_for` already
   seeds `choice:cleric_domain -> domain:good`, `choice:witch_hex -> hex:flight` and ten
   more, so the baseline already carried the pool's effect and the record under test could
   add nothing. Pinned by `canonical_seeds_really_do_occupy_probed_slots`.

Fixing them moved the count 0 → 4. **The correction is the only reason this is not a false
ceiling**, and it is the direction of error worth flagging: an instrument bug looks
identical to a real ceiling from the outside.

## What moved, in full

```
advanced_class_guide:class_feature:bloodrager_bloodline_arcane   computed  unknown      -> grounded  -> DONE
advanced_players_guide:class_feature:discovery_feral_mutagen     computed  unknown      -> grounded  -> DONE
advanced_class_guide:class_feature:slayer_talent_foil_scrutiny   static    not-ingested -> grounded  -> held (no done rung)
core_rulebook:class_feature:rogue_talent_resiliency              static    not-ingested -> grounded  -> held (no done rung)
```

`grounded` 5,349 → 5,353 · `not-ingested` 17,209 → 17,206 · `unknown` 3,547 → 3,546.
**0 wiring_class changes. 0 unit ids added or removed** (38,521 both sides, `beginner_box`
excluded). `git diff --stat -- data/` empty.

Baseline discipline: `docs/work-inventory.json` was regenerated at the parent commit
*before* the classify change, and changed only `generated_at` (0 non-timestamp lines), so
the four-unit diff is attributable to this change and to nothing else.

Count-pin sweep across `tests/ src/ apps/ scripts/` for all four moved figures, old **and**
new values: no hardcoded assertion on any of them.

One near-miss worth recording: the first baseline regeneration appeared to revert a sibling
agent's evidence strings on eleven ACG classes. It was a **stale release binary** — built
before the rebase that brought their `classify` change in. Rebuilding reproduced their
output exactly. A stale binary is the `CARGO_TARGET_DIR` cross-tree hazard wearing different
clothes: it produced a plausible wrong diff rather than an error.

## The real ceiling

**This instrument can reach at most 4 keys, and that is not a coverage problem — it is the
engine's actual state.** The 431 refusals are not units awaiting a better probe; they are
units where the engine deliberately implements a slot count and explicitly declines to
model the record's effect. Widening the pool table cannot move them. Only building the
per-record effects can, and that is content work, not instrument work.

The three walls, each with the command behind it:

- **431 pool members are slot-count only.** The engine records *that* a pick was spent
  (`…rage_power_choice`, value 0) and never *which*. Fixing this means implementing 169
  rage powers, 130 rogue talents, and so on — one execution engine at a time.
- **1,804 `computed` class_feature units are `unknown` and 14,553 keys have no slot at
  all.** Their group prefix names an archetype or a shared sub-choice set, not a pool this
  engine offers. This is the archetype mechanism **SD-30 epic-5 owns**, and it is the real
  gate on class_feature's remaining ~15,450 units. Splitting command:
  `Counter(key.split(" ~ ")[0] for unknown computed class_feature)` → 691 distinct group
  prefixes, top ones `Refined Education` 94, `Favored Enemy Bonus` 37, `Forbidden Rites
  Domain` 33, `Wildcat` 29, `Rage Power` 25.
- **2,326 are behind per-book ingest**, which this card excluded by instruction.

A fourth, narrower ceiling worth naming for whoever takes the next class_feature card:
Hunter's Animal Focus **does** have a real per-record magnitude
(`hunter_animal_focus_bull_bonus`, +2/+4/+6 by level), but it is activation-gated — it
needs a `class_ability_activations` entry in `EquippedActive`, which this probe's posture
does not build. Its own doc comment caps the prize at one or two units: *"Bull was picked
as the one canonical focus of the 13 real options this closure grounds … the other 11 stay
named-but-unbuilt."* Worth one cycle, not worth a lane.

## Gate

`./scripts/verify.sh` (full, all 16 stages), exit code captured directly, not through a
pipe: **`VERIFY_EXIT=0`, `RESULT: PASS`**.

```
passed: 16  preflight-disk pi-sweep audit-selftest reclaim-selftest driver-selftest
            corpus-sweep-selftest root-lib root-full desktop reach corpus-sweep
            frontend-install frontend-test frontend-typecheck clippy class-dump
root-full   6390 measured (baseline floor 6371)
clippy      root:46 desktop:7 warnings, 0 errors
class-dump  31/31 computing
corpus-sweep 3516 records examined, 8903 digests checked, 0 findings
```

Two baseline notes, neither a failure and neither touched by this run:
`BASELINE_ROOT_FULL_TESTS` is stale (6371 recorded, 6390 measured — this run added 9 tests)
and `BASELINE_CLIPPY_WARNINGS_ROOT` is loose (54 recorded, 46 measured). Left for a
deliberate `--show-actuals` baseline commit rather than folded into a content change.

## Tests

9 new tests in `class_feature_consumer_delta_tests`, written first and observed red on the
missing type, table and functions (E0433/E0425) before any implementation existed.

```
cargo test --locked --bin v06_work_inventory
#   75 passed; 0 failed; 0 ignored
```

Three pin `classify_class_feature_delta`'s branches directly (promote / refuse-as-pick-count
/ no-delta); two pin the pool table against engine source; one pins the canonical-seed
overlap so a future seed cannot silently reintroduce defect 2; three pin the refusal paths.

## Anti-gaming statement

No threshold, bucket definition, status word, `doneness_meaning` or `status_vocabulary` was
touched. No unit was reclassified into an easier `wiring_class` (0 wiring_class changes). No
corpus or fixture data was authored. No check was weakened, skipped or `#[ignore]`d. The
`--class-feature-probe` mode writes nothing and moves no unit; the grounding is a separate
commit whose entire inventory diff is the four units named above.

The honest gain is +2.
