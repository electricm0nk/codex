# SD31-E5-F1-001 — `class_feature` lever measurement

Cycle `SD31-E5-F1-001`, card `epic-5-chassis-sweep` F1. All figures re-derived
directly against the source this cycle; the exact command is given beside
each table. HEAD started at `89846f5c982ade12458595d0e7d885f4a5d91f80`
(tranche/11 tip). Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`, confirmed via `./scripts/verify.sh --only preflight-oracle`).

## 1. Is there a shipped `rules_tables` module to dump, the way `ultimate_equipment` had one?

```
grep -rl "class_feature\|ClassFeature" src/rules_core/rules_tables/
```
→
```
src/rules_core/rules_tables/acg/mod.rs
src/rules_core/rules_tables/pathfinder_unchained/monk_features.rs
src/rules_core/rules_tables/pathfinder_unchained/barbarian_features.rs
src/rules_core/rules_tables/pathfinder_unchained/summoner_features.rs
src/rules_core/rules_tables/pathfinder_unchained/rogue_features.rs
src/rules_core/rules_tables/apg/mod.rs
src/rules_core/rules_tables/crb/cleric_spell_list.rs
src/rules_core/rules_tables/crb/weapon_tables.rs
```

**No.** Every one of these is class-CHASSIS *mechanism* code (Fighter Weapon
Training bonus math, Cleric domain spell selection, the four Pathfinder
Unchained per-class feature files backing the already-wired Barbarian/
Monk/Rogue/Summoner Unchained variants) — never a per-record data table
naming every class feature's key/description/citation the way
`ultimate_equipment::equipment_tables` does for equipment. **There is
nothing to dump.** `equip­ment`'s lever (dump an already-completed Rust
table) does not exist for `class_feature`; this cycle's generator instead
does book-agnostic LST-token transcription (see
`src/rules_core/cache_gen/class_feature.rs`'s module doc comment for why
that is the correct strategy here, not a `decisions.md §11.3` violation).

## 2. Which of the 23 in-scope books had a `data/corpus/<book>/class_feature/` directory before this cycle, and how many `class_feature` units does each carry?

```
ls -d data/corpus/*/class_feature 2>/dev/null   # → only data/corpus/pathfinder_unchained/class_feature (1 of 23)
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
U=[u for u in d['units'] if u.get('kind')=='class_feature']; \
c=collections.Counter(u.get('book') for u in U); \
[print(b,n) for b,n in sorted(c.items(), key=lambda x:-x[1])]"
```

| book | class_feature units | corpus dir before | records written this cycle | corpus dir after |
|---|---:|---|---:|---|
| advanced_class_guide | 2396 | absent | 2395 | present |
| advanced_players_guide | 2055 | absent | 2055 | present |
| ultimate_psionics | 1422 | absent | 0 (excluded, see §3) | absent |
| ultimate_combat | 1412 | absent | 991 | present |
| ultimate_magic | 1070 | absent | 1069 | present |
| occult_adventures | 979 | absent | 979 | present |
| core_rulebook | 959 | absent | 959 | present |
| ultimate_wilderness | 866 | absent | 753 | present |
| ultimate_intrigue | 777 | absent | 651 | present |
| adventurers_guide | 700 | absent | 651 | present |
| advanced_race_guide | 645 | absent | 643 | present |
| pathfinder_unchained | 577 | **present (64 hand-curated)** | 0 (excluded on purpose, not overwritten) | present |
| horror_adventures | 419 | absent | 165 | present |
| inner_sea_combat | 314 | absent | 306 | present |
| inner_sea_magic | 218 | absent | 198 | present |
| book_of_the_damned_volume_2 | 212 | absent | 205 | present |
| inner_sea_world_guide | 171 | absent | 142 | present |
| inner_sea_intrigue | 169 | absent | 158 | present |
| monster_codex | 68 | absent | 68 | present |
| bestiary_6 | 18 | absent | 18 | present |
| inner_sea_taverns | 11 | absent | 11 | present |
| book_of_the_damned_volume_1 | 10 | absent | 10 | present |
| bestiary_4 | 4 | absent | 4 | present |
| **total** | **15472** | **1 of 23** | **12431 written this cycle** | **22 of 23** |

Written-count is lower than the book's full unit count wherever
`v06_work_inventory::enumerate_book` recursively picked up nested
`support/*.lst` or `_pfs/*.lst` cross-book variant files this cycle
deliberately excludes (module doc comment §"Scope this cycle") — a named
shortfall (`OPEN-ISSUES.md`), not a silent gap.

`ultimate_psionics` is fully excluded this cycle: its non-Paizo path
(`pathfinder/dreamscarred_press/ultimate_psionics/<file>`, only 4 segments)
fails `corpus_literal_sweep::book_dir_of`'s hard 5-segment requirement
(`<system>/<publisher>/<line>/<book>/<file>`) — confirmed live by running
`cargo run --locked --bin corpus_literal_sweep` against a first attempt at
its dump (`OPEN-ISSUES.md`, this cycle's row). `book_dir_of` lives in
`src/bin/corpus_literal_sweep.rs`, outside this card's file territory.

## 3. Traced one unit end to end BEFORE writing any dump code

`core_rulebook:class_feature:rogue_sneak_attack`:

- Corpus row: `cr_abilities_class.lst:1615` — `Sneak Attack␉␉KEY:Rogue ~ Sneak
  Attack␉␉CATEGORY:Special Ability␉TYPE:...␉␉␉␉VISIBLE:DISPLAY␉DEFINE:
  RogueSneakAttackLVL|0␉...␉ABILITY:Special Ability|AUTOMATIC|Sneak
  Attack␉...␉BONUS:VAR|RogueSneakAttackLVL|RogueLVL` (no `DESC:` token).
- `docs/work-inventory.json`'s pre-cycle entry:
  `"status": "grounded", "evidence": "explanation_id_observed_in_a_real_computation",
  "wiring_class": "static", "wiring_class_reason": "literal_magnitudes_only"`
  — the engine ALREADY genuinely computes Sneak Attack (a real class-chassis
  feature), independent of any corpus JSON.
- `pf1e_dashboard_producer.doneness_verdict("static", "grounded", "class_feature")`
  → the `("static","derived")` branch's rule: `grounded` (not
  `literal-verified`) → `DONENESS_HELD`, not `done`. **The one thing missing
  was the `literal-verified` stamp**, which only `corpus_literal_sweep`
  finding this unit's `(book, source_file, source_line)` in its
  `sweep_verified` set can supply — and that set is built entirely from
  `data/corpus/**/*.json`, which `core_rulebook/class_feature/` did not
  have.
- After this cycle's dump + a guarded regen: same unit's `status` becomes
  `"literal-verified"` → `doneness_verdict` → `DONENESS_DONE`. Traced,
  proven, not assumed.

**The critical, negative finding this trace produced BEFORE writing the
generator:** `Kind::ClassFeature`'s `classify()` arm
(`src/bin/v06_work_inventory.rs:3412`) sets `status` from TWO paths only —
`class_feature_effect_wired` (an engine consumer-delta probe) or a matching
`explanation_id` from a real computation — **both require the engine to
already genuinely compute the feature.** Neither path reads
`data/corpus/**/*.json` at all. **A corpus-JSON dump cannot manufacture a
`grounded` status; it can only unlock the `literal-verified` STAMP for a
unit that is ALREADY `grounded`.** This is why the `ultimate_equipment`
lever's shape (dump → done, corpus-wide, for nearly the whole book) does
NOT generalize to `class_feature`: equipment's `status` is set by a
magnitude-token check the corpus itself satisfies; class_feature's
`status` is set by genuine engine wiring that Epic 4 (`epic-4-mechanism`)
has not yet built for any class this wave (`kanban.md` row: zero cycles
landed under `epic-4-mechanism` as of this cycle).

## 4. Re-derived the promotable population BEFORE writing any code

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('kind')=='class_feature']
c = collections.Counter((u.get('wiring_class'), u.get('status')) for u in U)
[print(k,v) for k,v in sorted(c.items(), key=lambda x:-x[1])]"
```

Only `(static, grounded)` and `(derived, grounded)` cells can EVER reach
`done` via a corpus-JSON-dump lever (the `literal-verified`/
`fixture-verified` stamps), and only `static` is achievable from THIS
card's file territory (`fixture-verified` needs a
`derived_evaluator_fixture_check.rs` fixture per record, a different
file/lane). Pre-cycle counts: **`(static, grounded)` = 14**, `(derived,
grounded)` = 19 (not this card's lever), `(computed, grounded)` = 20
(already `done` — `computed`'s bar is `status == "grounded"` directly, no
corpus JSON needed at all), `(display, grounded)` = 54 (capped at `held`
by design, `display`'s bar is `text-complete`), `(ambiguous, grounded)` =
1 (capped at `held`, the lower-bound rule). Pre-cycle `done` = 5
(`static`+`literal-verified`, already-shipped `pathfinder_unchained`
records) + 20 (`computed`+`grounded`) = **25**, matching the dispatch
brief's own figure exactly.

Also re-derived and confirmed against the brief's own cited figures:
`not-started` doneness population (status `not-ingested` + `not-started`)
= **11476**; `unmeasurable` population (status `unknown`) = **3849**;
`deferred` population (status `deferred-with-reason`) = **34** (+2 `feat`
units elsewhere = the 36 the brief's F4 seed names). All match exactly.

## 5. Result

`(static, grounded)` = 14 units, all sourced from PRIMARY (non-nested)
files, all in books this cycle's dump covers (core_rulebook ×12,
advanced_class_guide ×1, ultimate_combat ×1) → all 14 promoted to `done`
via the new `literal-verified` stamp. Measured by the guarded regen +
`doneness_verdict` replay (see `progress.md`'s cycle receipt): board
`done` 7355 → **7369 (+14)**; `class_feature` `done` 25 → **39 (+14)**.

The other 12,417 records this cycle wrote are genuinely `not-started`/
`unknown` today (no engine wiring exists for their owning class) and move
**0** additional units to `done` — banked infrastructure for whichever
future `epic-4-mechanism` cycle wires that class's chassis, at which point
the `literal-verified`/corpus-JSON half of the work will already be done.
This is the honest, re-derived deliverable this card's brief asked for: a
proving trace showing the equipment-shaped lever does NOT generalize to
`class_feature`'s 99%-not-started mass, plus the narrow real lever it DOES
expose, pulled.
