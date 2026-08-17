# SD31-E5-F1-002 — `class_feature` not-started breakdown

**Cycle:** `SD31-E5-F1-002` (`RETRO_ACTOR=sd31-cf-ground`), primary checkout, direct to `tranche/11`.
**Oracle:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
**Re-derived at:** `docs/work-inventory.json` as committed at this cycle's starting HEAD
(`a9426b760`), before any code change. Every figure below has its exact command; re-run them
rather than transcribing this document into a future dispatch.

## 0 — Board framing, re-derived

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature']
print(len(U))
"
# -> 15472
```

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature']
print(collections.Counter(u.get('status') for u in U).most_common())
"
# -> not-ingested 10475, unknown 3917, not-started 929, grounded 55, text-complete 43,
#    deferred-with-reason 34, literal-verified 19
```

`pf1e_dashboard_producer.doneness_verdict()` maps `status in {not-ingested, not-started}` to
`DONENESS_NOT_STARTED` **regardless of `wiring_class`** (`_doneness_verdict_uncapped`,
`scripts/observer/pf1e_dashboard_producer.py:3636-3641`), and `status == "unknown"` straight to
`DONENESS_UNMEASURABLE`. So:

- **not-started (board-level) = 10,475 + 929 = 11,404** — exactly the mandate's figure, confirmed,
  not transcribed.
- **unmeasurable = 3,917** — exactly `status == "unknown"`, confirmed.

Both are re-derived from the raw `status`/`evidence` fields the inventory already stamps, not from
a second instrument.

## 1 — The trace, end to end, for a real unit: `advanced_class_guide:class_feature:Slayer ~ Track`

**Step 1 — corpus row** (pinned oracle, `$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/
advanced_class_guide/acg_abilities_class.lst:1787`, quoted verbatim, not paraphrased):

```
Track	KEY:Slayer ~ Track	CATEGORY:Special Ability	TYPE:Slayer Class Feature.SpecialQuality.Extraordinary
DESC:You gain +%1 to Survival checks made to follow tracks.|SlayerTrackBonus
ABILITY:Internal|AUTOMATIC|Slayer Track Bonus
SOURCEPAGE:p.53	ASPECT:Ability Benefit|+%1|SlayerTrackBonus
```

**Step 2 — shipped `data/corpus/` record** (`data/corpus/advanced_class_guide/class_feature/
slayer/track.json`, written by `SD31-E5-F1-001`'s wave-4 `gen_cache_class_feature`): `data.key
== "Slayer ~ Track"`, `data.description` carries the `DESC:` text verbatim, `wiring_class:
"display"` (`wiring_class_signals: ["display:no_magnitude_token"]` — the `%1` is a PCGen
placeholder resolved by a separate `ASPECT:`/`ABILITY:` formula variable, not a literal numeric
token the wiring classifier's `MAGNITUDE_TOKENS` scan recognizes in this row, so
`magnitude_token_count == 0` and `text_only == true` downstream).

**Step 3 — what the inventory reads.** `classify()`'s `Kind::ClassFeature` arm
(`src/bin/v06_work_inventory.rs`):

1. `facts.class_feature_effect_wired.get("Slayer ~ Track")` — `None` (Track is not one of the 28
   `CLASS_FEATURE_POOLS` option-pool prefixes; it is an automatic base-class feature).
2. `class_feature_owner("Slayer ~ Track", facts.class_books.keys())` (line 4174) — `owner =
   "slayer"` (ACG's `AcgClassId::Slayer` registers `"slayer" -> "advanced_class_guide"` in
   `modelled_class_books()`, line 5953). **This step succeeds** — Slayer has always been a
   modelled class; this is not row 96/118's registry gap.
3. `feature_slug = slug("Track") = "track"` (line 5255-5256).
4. **The leak, exact line, pre-fix** (line 5257-5260):
   ```rust
   let grounded = facts
       .explanation_ids
       .iter()
       .any(|id| id.contains(&format!(".{owner}.")) && id.ends_with(&feature_slug));
   ```
   `pilot_compute.rs`'s `ground_or_block_slayer_class_features` (line ~17295) pushes
   `ComputationExplanation { id: "class_feature.acg.slayer.track_bonus", value: track_bonus, .. }`
   **unconditionally** for any Slayer character of level ≥ 1 the engine sweeps
   (`SWEEP_LEVELS`) — the engine genuinely computes this every time. But
   `"class_feature.acg.slayer.track_bonus".ends_with("track")` is `false` in Rust: `ends_with` is
   a raw byte-suffix check, and `pilot_compute.rs`'s own established naming idiom appends a
   trailing magnitude-descriptor word (`_bonus`/`_count`/`_dc`/`_dice`/...) to many explanation
   ids. `grounded` evaluates `false` even though the exact fact this unit asks about was computed
   moments earlier in the same sweep.
5. `grounded` is `false`, so the diagnostic-quote branch (line ~5350, `facts.diagnostics.iter()`)
   is tried next — no diagnostic literally names `track` as unsupported (Track is a fully wired
   feature, not a deferred one), so that also misses.
6. Falls to the final branch (line 5352, `text_only == true`):
   ```rust
   return not_ingested("class_feature_owner_matched_by_name_but_record_not_held_by_engine");
   ```
   **This is the exact line that leaves the unit `not-started`.** `status: "not-ingested"`.

**Step 4 — `doneness_verdict`.** `_doneness_verdict_uncapped("display", "not-ingested")` → `status
in {"not-ingested","not-started"}` is checked FIRST, before `wiring_class` is even consulted
(`pf1e_dashboard_producer.py:3640-3641`) → `DONENESS_NOT_STARTED`.

**The gap, stated precisely:** the record IS grounded in reality (the engine computes Track's
bonus every sweep) and the corpus row IS shipped and PI-screened. The single point of failure is
a **measurement defect** in the inventory's own naming-match convention (row 78,
`OPEN-ISSUES.md`) — not a missing corpus record (wave 4 already shipped it) and not missing engine
wiring (`pilot_compute.rs` already computes it). This is Slayer's own case; 12 of its 13
not-started features share this exact shape (`OPEN-ISSUES.md` row 78's original finding).

## 2 — The 11,404 quantified, by distinct cause

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature' and u.get('status') in ('not-ingested','not-started')]
c=collections.Counter((u.get('evidence') or '').split(':')[0] for u in U)
for k,v in c.most_common(): print(v,k)
"
```

| evidence (verbatim `classify()` reason string) | count | share | meaning |
|---|---:|---:|---|
| `class_feature_option_pool_record_not_held_by_engine` | 4,520 | 39.6% | zero-magnitude option-pool member (Rage Power, Discovery, Domain Power, ...); no chooser-shaped mechanism holds this specific option's identity |
| `class_feature_of_unmodelled_corpus_class` | 2,152 | 18.9% | corpus declares a class by name that `modelled_class_books()` never registered |
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 2,002 | 17.6% | owner class IS modelled; zero-magnitude feature; no explanation id or diagnostic names it |
| `no_explanation_id_and_no_diagnostic_names_this_feature` | 1,801 | 15.8% | owner class IS modelled; real-magnitude feature; no explanation id or diagnostic names it |
| `no_compiled_rule_set_for_book` | 929 | 8.1% | the unit's whole BOOK has no compiled rule-set module at all |
| **total** | **11,404** | **100%** | |

**Sum verified**: 4520+2152+2002+1801+929 = 11,404. ✓

### 2a — `no_compiled_rule_set_for_book` (929), by book

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature' and u.get('evidence')=='no_compiled_rule_set_for_book']
print(collections.Counter(u.get('book') for u in U).most_common())
"
# -> adventurers_guide 700, inner_sea_magic 218, inner_sea_taverns 11
```

Not fixable from `v06_work_inventory.rs`: `RuleSetId`/`COMPILED_RULE_SETS`
(`src/bin/v06_work_inventory.rs:1913-1930`'s own doc comment names exactly this defect shape —
"Eleven days of delivered work read as untouched" was the ARG/PU precedent) has no variant for
these three books at all; needs new class-mechanism modules, lane 1's territory (or a fresh
book-onboarding cycle).

### 2b — `class_feature_of_unmodelled_corpus_class` (2,152), by named class

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature' and (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')]
c=collections.Counter(u['evidence'].split(':',1)[1] for u in U)
print(len(c),'distinct classes,', sum(c.values()),'units')
print(c.most_common(15))
"
```

84 distinct corpus-declared classes, 2,152 units. **120 of these (Gunslinger 42, Ninja 56, Samurai
22) are Ultimate Combat classes `pilot_compute.rs` already wires real chassis for**
(`OPEN-ISSUES.md` rows 96/118, `SD31-E4-F1-002`/`SD31-E4-F1-003`) — `modelled_class_books()` simply
never named their book. **This cycle fixes that 120-unit share** (§4 below). The remaining
**2,032 units across 81 classes** (Medium 143, Aegis 131, Psychic 125, Vigilante 117, Magus 101,
Shifter 95, Occultist 70, Mesmerist 70, Kineticist 61, ...) are genuinely blocked on zero engine
chassis — independently confirmed by `SD31-E3-F1-001`'s own no-proxy measurement, which found 23
of 24 supersession-shape classes (every OA/UPsi/newcomer class except Slayer) show **0
wired-able** mechanisms in `pilot_compute.rs` today. Not an inventory defect; needs real
chassis-building cycles (Epic 4/5's continuing work).

### 2c — the 3,803-unit "owner matched, no explanation id" population, split by real cause

The two evidence buckets `class_feature_owner_matched_by_name_but_record_not_held_by_engine`
(2,002) and `no_explanation_id_and_no_diagnostic_names_this_feature` (1,801) sum to **3,803**. Both
occur only after `class_feature_owner` already succeeded (the class IS modelled), so the remaining
question is purely: does the engine's `facts.explanation_ids` set (built by sweeping every
modelled class through the real compute pipeline, `src/bin/v06_work_inventory.rs:3942-3958`)
contain a fact that names this specific feature under a naming convention the exact
`ends_with` check misses?

Cross-referenced every one of the 3,803 units' `(owner, feature_slug)` pair against the literal
`ComputationExplanation`/`ComputationDiagnostic` ids `pilot_compute.rs` declares (637 explanation
ids, 97 diagnostic ids extracted by direct source scan — script preserved at this artifact's
sibling analysis, reproducible via `grep -n 'ComputationExplanation {' -A2
src/rules_core/pilot_compute.rs` cross-referenced against each unit's `corpus_key`):

| sub-cause | count | what it means | fixable from `v06_work_inventory.rs`? |
|---|---:|---:|---|
| **A — naming-mismatch, SAFE population** (base-class group, no archetype/variant qualifier, single known-suffix-word strip recovers an exact match) | **31** | row 78's naming bug, narrowly confirmed | **yes — fixed this cycle, §4** |
| **A′ — naming-mismatch, archetype/variant-qualified group** (e.g. `Sanctified Slayer ~ Sneak Attack`, `Unchained Monk ~ Perfect Self`) | 17 | same string shape as A, but the group names an ARCHETYPE/VARIANT, not the base class; crediting it off the base class's explanation would be the exact cross-variant conflation `decisions.md §10`'s AMENDMENT forbids (a variant is a different object) | **no — correctly left alone**, needs real per-archetype/variant wiring, lane 1 |
| **B — no explanation id matches even with generous stripping** | 3,744 | the specific feature genuinely has no computed fact anywhere in `pilot_compute.rs` (mostly archetype/output-qualified groups whose OWN mechanics were never wired, e.g. `Fighter Archetype ~ Martial Master`) | no — genuine engine gap, lane 1 |
| **C — exact match already succeeds today** (`"Fighter ~ Weapon Training"` and 3 variant-qualified siblings) | 4 | id.ends_with already true; the record is still not-started because the SWEEP that built `facts.explanation_ids` for THIS run never reached the level/precondition that emits it (a probe-coverage gap, not a naming gap) | no — different mechanism, named not fixed, see §5 |
| sum | 3,796 | (7 units untraceable to a specific `pilot_compute.rs` id by this method — malformed/no-separator corpus_keys like the bare `"Slayer"` record identified in §4's own safety check; correctly excluded from both A and A′) | — |

Full per-unit citation for row A (all 31, real `pilot_compute.rs` explanation id and the exact
suffix word stripped) and row A′ (all 17) is in this cycle's `progress.md` receipt — reproduced
here by class:

- **A (31, fixed):** Brawler (Knockout, Maneuver Training), Hunter (Animal Focus, Wild Empathy),
  Investigator (Alchemy, Poison Resistance, Studied Combat, Studied Strike, Trap Sense,
  Trapfinding), Skald (Bardic Knowledge, Song of Strength), Slayer (Master Slayer, Sneak Attack,
  Stalker, Studied Target, Track, Trap Sense, Trapfinding), Swashbuckler (Bleeding Wound, Precise
  Strike, Swashbuckler Initiative), Warpriest (Channel Energy), Alchemist (Bomb, Poison
  Resistance), Cavalier (Expert Trainer), Inquisitor (Cunning Initiative, Monster Lore, Track),
  Cleric (Channel Energy), Monk (Quivering Palm).

## 3 — The 3,917 unmeasurable (`status == "unknown"`), characterized

All 3,917 share **one** evidence reason: `class_feature_group_names_no_class_at_all` — i.e. the
group prefix matches neither a modelled class nor any corpus-declared class name, AND the unit
carries a real (non-zero) magnitude token, so `classify()` cannot even offer the conservative
"not held anywhere" verdict the zero-magnitude sibling population gets; it is `unknown` because no
owner can be derived to check anything against.

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='class_feature' and u.get('status')=='unknown']
print(len(U), collections.Counter(u.get('evidence') for u in U))
"
# -> 3917 {'class_feature_group_names_no_class_at_all': 3917}
```

This is the SAME shape as the `class_feature_option_pool_record_not_held_by_engine` population
(§2's 4,520) — option pools, archetypes and shared sub-choice sets — split from it only by whether
this specific unit's own record carries a magnitude token or not. Per Epic 3-F4's characterization
method (real chooser vs genuinely unreachable vs residual):

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
POOLS={'Rage Power','Unchained Rage Power','Discovery','Grand Discovery','Rogue Talent',
'Advanced Talents','Hex','Revelation','Mercy','Investigator Talent','Slayer Talent','Judgment',
'Inquisition','Blessing','Evolution','Bloodline','Bloodrager Bloodline','Domain','Order','Mystery',
'Curse','Spirit','Animal Focus','Favored Enemy','Favored Terrain','Versatile Performance',
'Arcane School','Focused Arcane School'}
U=[u for u in d['units'] if u.get('kind')=='class_feature' and u.get('status')=='unknown']
in_list=[u for u in U if u['corpus_key'].split(' ~ ')[0] in POOLS]
print('option-pool content with a real chooser wired:', len(in_list))
print('outside any wired chooser entirely:', len(U)-len(in_list))
"
```

- **Option-pool content with a real chooser wired: 191** (5 pools: Rage Power 37, Favored Enemy
  36, Inquisition 24, Unchained Rage Power 24, Discovery 22, Favored Terrain 18, Focused Arcane
  School 16, Versatile Performance 9, Judgment 5). The chooser SLOT is wired
  (`CLASS_FEATURE_POOLS`, `probe_class_feature_key`), but the consumer-delta probe found no fact
  attributable to THIS specific option — i.e. picking it produces no observable difference from a
  sibling pick. A real, narrower gap than "no mechanism at all": the count/slot exists, the
  per-option identity does not.
- **Genuinely unreachable / residual, outside any wired chooser: 3,726** across ~1,300+ distinct
  pool names (Domain Power 106, Refined Education 94, Hate-Monger 34, Forbidden Rites Domain 33,
  Wildcat 30, Ki Power 29, Insight 28, Blade Skill 27, ...). No `CLASS_FEATURE_POOLS` entry names
  these prefixes at all — this is Epic 3-F3/Epic 4-F2's designed-not-yet-built chooser-interaction
  primitive territory, not something the inventory can characterize further without inventing a
  mechanism.

**Combined with §2's 4,520 zero-magnitude option-pool population**, the full option-pool-shaped
share of the board is **8,437 units** (4,520 + 3,917), of which only **472 (5.6%)** touch an
already-wired chooser slot at all — directly re-derived, not inferred: **191** of the magnitude>0
(`unknown`) population plus **281** of the zero-magnitude 4,520 (`python3 -c "... u['corpus_key']
.split(' ~ ')[0] in POOLS ..."` against `class_feature_option_pool_record_not_held_by_engine`,
same command shape as above) — and **7,965 (94.4%)** across **1,847 distinct pool names** have no
chooser primitive naming them at all. This single
number is this artifact's clearest dispatch signal: the largest lever on `class_feature` closure
is not more corpus ingest (wave 4 already dumped 12,431 records) and not this inventory's naming
conventions — it is Epic 4-F2's chooser-interaction primitive, scaled from covering 28 pools to
covering roughly 1,875.

**Characterization only — no unit above was reclassified by this section.**

## 4 — What this cycle fixed (in-territory, `src/bin/v06_work_inventory.rs`)

Both fixes are additive, scoped, TDD'd (12 new tests, `src/bin/v06_work_inventory.rs`
`class_feature_id_magnitude_suffix_strip_tests` / `modelled_class_books_registry_tests` / 6 new
cases in `class_feature_text_complete_rung_tests`), zero regressions (211/211 bin tests, was
199/199).

**Fix 1 — `modelled_class_books()` registers `UcClassId`** (Gunslinger/Ninja/Samurai →
`"ultimate_combat"`), closing `OPEN-ISSUES.md` rows 96/118. `PuClassId` (Pathfinder Unchained) is
deliberately NOT added — its multi-word underscored names (`"unchained_rogue"`) need
`class_feature_owner`'s own matching logic reworked first, or its substring fallback would
mis-attribute PU's variant records to the BASE class (a `decisions.md §10` AMENDMENT hazard);
named, not fixed, this cycle.

**Fix 2 — a scoped known-magnitude-suffix fallback** for the `explanation_id.ends_with
(feature_slug)` check (`id_matches_feature_slug_after_known_magnitude_suffix_strip`,
`CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` — the exact 18-word list `OPEN-ISSUES.md` row 78's own regex
already named, reused verbatim, never widened). Fires ONLY when: (a) the exact check already
failed, (b) the unit's group prefix IS the bare class name (never an archetype/variant qualifier —
the `decisions.md §10` guard, proven by a dedicated test using `Sanctified Slayer ~ Sneak Attack`),
(c) `feature_slug != owner` (excludes the malformed no-`" ~ "`-separator shape, proven by a
dedicated test reproducing the real `"Slayer"` corpus_key collision found during this cycle's own
pre-flight check), and (d) stripping exactly ONE trailing known-suffix word (never two or three,
never an invented word) makes the remainder match. Verified zero cross-feature collisions
corpus-wide before landing (every `(owner, id)` pair checked against every candidate `feature_slug`
sharing that owner — one real collision found and closed by guard (c) before it could ship).

**Measured movement — guarded regen, every unit individually diffed before/after by id, not
estimated** (`corpus_literal_sweep` → 24,583 examined/0 findings/CLEAN;
`derived_evaluator_fixture_check` → 998/999 cleared, the same pre-existing
`advanced_players_guide:equipment:spindle_of_perfect_knowledge` failure every prior wave names,
exit 0; `v06_work_inventory` → zero stamp loss, 38,540 total units both before and after):

```
python3 -c "
import json, sys; sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
b=json.load(open('/tmp/work-inventory-BEFORE.json')); a=json.load(open('docs/work-inventory.json'))
bmap={u['id']:u for u in b['units']}; amap={u['id']:u for u in a['units']}
for uid,au in amap.items():
    bu=bmap.get(uid)
    if not bu: continue
    bv=P.doneness_verdict(bu.get('wiring_class'),bu.get('status'),bu.get('kind'))
    av=P.doneness_verdict(au.get('wiring_class'),au.get('status'),au.get('kind'))
    if bv!=av: print(au['corpus_key'],au['book'],bv,'->',av)
"
```

**45 units moved, all `class_feature`, zero regressions anywhere on the board (board-wide
regression scan run both directions, 0 found; 0 movement in any other `kind`):**

| destination | count |
|---|---:|
| `not-started` → `done` | 23 |
| `not-started` → `held` | 21 |
| `not-started` → `deferred` | 1 |

- **Fix 2 (suffix-strip fallback) alone: 31 units** across ACG/APG/CRB (Brawler, Hunter,
  Investigator, Skald, Slayer, Swashbuckler — including 2 not hand-enumerated in §2c's static
  analysis, `Swashbuckler ~ Deadly Stab`/`Stunning Stab`, correctly caught by the same production
  code path — , Warpriest, Alchemist, Cavalier, Inquisitor, Cleric, Monk).
- **Fix 1 (`UcClassId` registration) alone or combined with Fix 2: 14 units** — Gunslinger (Grit,
  Gun Training, Gunslinger Initiative, Nimble — all 4 exact-match `done` the moment the class
  became modelled), Ninja (Ki Pool `done`, Ninja Trick `done` via the suffix-strip fallback firing
  on newly-modelled UC content too, No Trace/Sneak Attack `held`, `Ninja Trick ~ Feat` correctly
  routed to `deferred` by an existing UC diagnostic), Samurai (Bonus Feat/Challenge `done`, Resolve
  `held`).

**Board-wide: `done` 10,759 → 10,782 (+23, 27.9302% → 27.9899%), `held` +21, `deferred` +1,
`not-started` −45.** `class_feature` alone: `done` 82 → 105 (+23), `held` 35 → 56 (+21), `deferred`
34 → 35 (+1), `not-started` 11,404 → 11,359 (−45), `unmeasurable` unchanged at 3,917 (neither fix
touches the `unknown`-status population, correctly — §3 characterizes it, does not reclassify it).

`docs/work-inventory.json` restored per the wave rule (`git checkout -- docs/work-inventory.json`,
confirmed clean).

## 5 — Named, not fixed (out of file territory, reported for lane 1)

- **§2c row B (3,744)** and **row A′ (17)**: genuine engine gaps / correctly-guarded variant
  content. `pilot_compute.rs`/`archetype_resolver.rs`, lane 1.
- **§2c row C (4 — `Fighter ~ Weapon Training` and 3 archetype-qualified Fighter siblings)**: the
  exact-suffix check ALREADY succeeds (`"class_feature.fighter.weapon_training".ends_with
  ("weapon_training")` is true) but the unit is still `not-started` — meaning
  `facts.explanation_ids` (built by sweeping every modelled class at `SWEEP_LEVELS`,
  `src/bin/v06_work_inventory.rs:3942-3958`) never actually contained this id in this run. Weapon
  Training requires Fighter level ≥ 5 (a `PRE:` guard); if `SWEEP_LEVELS` never probes a level that
  high, the fact is real but never observed. Not investigated further this cycle — a genuinely
  different mechanism (probe LEVEL coverage, not naming), named precisely so a future cycle does
  not conflate it with row A's fix.
- **§3's 3,726 + 7,965 combined figure** (unwired option-pool content): Epic 4-F2's
  chooser-interaction primitive, designed (Epic 3-F3) but not built. The single largest lever on
  `class_feature`'s remaining 55% share of the board.
- **§2b's 2,032** (genuinely zero-chassis classes): needs real chassis-building cycles, independent
  of this artifact.
- **§2a's 929** (`no_compiled_rule_set_for_book`): needs new class-mechanism modules for
  `adventurers_guide`/`inner_sea_magic`/`inner_sea_taverns`, or a book-onboarding decision.

## 6 — Reproducibility

Every number above has its exact command inline. The `pilot_compute.rs` explanation/diagnostic id
extraction (§2c, §4) was done by direct source scan (`grep`/Python over the committed file, not a
runtime dump) and cross-checked against a live `cargo test` run of the 12 new tests added this
cycle, which exercise the real `classify()` function against the real corpus shapes named above —
not a standalone script asserting its own logic. Re-run `cargo test --locked --bin
v06_work_inventory -- class_feature_id_magnitude_suffix_strip_tests
modelled_class_books_registry_tests` to reproduce Fix 1/2's correctness independent of this
document.
