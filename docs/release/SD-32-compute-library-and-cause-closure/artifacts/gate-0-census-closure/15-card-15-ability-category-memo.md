---
canonical: true
owner: card-15-ability-category
status: measurement-lane deliverable — decision memo, not a widening
date: 2026-08-22
---

# Card 15 decision memo — the `ability_category:*` buckets

**Scope:** kanban card 15 (`census-scope-closure`), the `ability_category:*` measurement lane
named in `decisions.md §12b`. This memo covers every `ability_category:*` key in
`artifacts/gate-0-census-closure/diff.json`'s `kind_unenumerable` — **26 categories, 5,886
units** (re-derive: `jq -r '.kind_unenumerable | to_entries[] | select(.key|startswith("ability_category:"))' artifacts/gate-0-census-closure/diff.json | jq -s 'map(.value) | add'`).
This is a measurement-and-decision lane: it produces this memo, not a change to
`docs/work-inventory.json`, `scripts/census_independent.py`, `scripts/shape_ledger.py`, or any
pinned-count file. Two sibling lanes are measuring the `class_feature` (18,231) and "everything
else" (3,551 + 179 `unclassified:<file>`) buckets concurrently; a single integration cycle applies
all three memos.

**Population note (`decisions.md §12c`):** every count in this memo is drawn from the
`kind_unenumerable` population of `artifacts/gate-0-census-closure/diff.json` — the census's own
discovered-but-not-tracked-kind set, 27,847 units across 44 buckets, of which the 26
`ability_category:*` buckets are 5,886. This is not the ledger's not-done 24,914, nor the
inventory's all-units 38,391.

## Method and the committed classifier

Every figure below is reproducible from one committed script,
`artifacts/gate-0-census-closure/15-card-15-ability-category-classify.py`, which re-walks the same
in-scope book set with `census_independent.py`'s own `classify_scope` /
`_classify_kind_by_filename` / `_row_category_tag` / `_parse_lst_rows` (imported, not
reimplemented) and adds a per-row disposition. It **fails closed**: it re-derives
`kind_unenumerable["ability_category:*"]` itself and refuses (non-zero exit) if its own totals
don't exactly match `diff.json`'s.

Re-derive command (pinned oracle SHA `7f818006e371188e5717fd18d74d18a420747fc6`,
`scripts/pcgen-oracle-pin.env`):

```bash
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-classify.py \
  --repo-root . \
  --corpus-root docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
  --inventory docs/work-inventory.json \
  --diff-json docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json \
  --output-jsonl docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-rows.jsonl \
  --output-summary-md docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-summary.md
```

Output: `rows: 5886` / `self-check: MATCH` (verified 2026-08-22, this cycle).

Every row gets one of four dispositions, applied in this priority order:

1. **`B-duplicate`** — the row's `KEY:` field (never a fallback to bare identity — a shared
   *display name* is not proof of a shared *thing*; see "the shared-name hazard" below) exactly
   matches a `KEY:` field already counted under a tracked kind (`feat`, `class`, `spell`,
   `monster`, `monster_ability`, `equipment`, `equipment_modifier`, `companion`, `race`,
   `race_trait`) elsewhere in the in-scope corpus.
2. **`A`** — not a duplicate, and the row carries at least one of `DEFINE:` / `BONUS*:` / `DESC:` /
   `ASPECT:` / `CSKILL:` / `MOVE:` / `AUTO:` / `TEMPLATE:` / `SPROP:` / `QUALITY:` / `SR:` / `DR:` /
   `SAB:` / `VISION:` — independent mechanical or narrative content, i.e. this row's effect does not
   depend on decoding some other row's grant token.
3. **`B-gateway`** — no independent content, but the row carries an
   `ABILITY:<Category>|AUTOMATIC|<target>` token whose target is a fully-defined object elsewhere
   (in a tracked kind or, more often, another `ability_category:*` row that is itself disposition
   `A`) — a wrapper/gateway row, not new content.
4. **`B-picklist`** — no independent content and no gateway token: a bare `CHOOSE:*` menu entry
   with nothing beyond `CATEGORY:`/`TYPE:`/`KEY:` — the value another (already-counted) ability's
   chooser selects from, not itself an object.

**Result** (`15-card-15-ability-category-summary.md`, this cycle):

| bucket | total | A (real, distinct) | B-gateway | B-picklist | B-duplicate |
|---|---:|---:|---:|---:|---:|
| `ability_category:Special Ability` | 3436 | 3363 | 5 | 61 | 7 |
| `ability_category:Internal` | 839 | 685 | 76 | 78 | 0 |
| `ability_category:Words of Power` | 369 | 330 | 0 | 39 | 0 |
| `ability_category:Ability Focus` | 272 | 0 | 0 | 272 | 0 |
| `ability_category:Spell-Like Ability` | 165 | 145 | 0 | 20 | 0 |
| `ability_category:Path Dabbling` | 128 | 0 | 128 | 0 | 0 |
| `ability_category:Class Skill` | 102 | 102 | 0 | 0 | 0 |
| `ability_category:Intelligent Item` | 100 | 22 | 0 | 78 | 0 |
| `ability_category:Background` | 72 | 72 | 0 | 0 | 0 |
| `ability_category:Afflictions` | 70 | 70 | 0 | 0 | 0 |
| `ability_category:Save Bonus` | 58 | 58 | 0 | 0 | 0 |
| `ability_category:Aligned Class` | 52 | 52 | 0 | 0 | 0 |
| `ability_category:Eldritch Heritage Bloodline` | 31 | 27 | 0 | 4 | 0 |
| `ability_category:Class` | 29 | 20 | 1 | 8 | 0 |
| `ability_category:Racial Traits` | 28 | 27 | 0 | 0 | 1 |
| `ability_category:Archetype` | 27 | 27 | 0 | 0 | 0 |
| `ability_category:Builder` | 24 | 24 | 0 | 0 | 0 |
| `ability_category:Equipment` | 21 | 21 | 0 | 0 | 0 |
| `ability_category:Mythic Weapon Training` | 16 | 16 | 0 | 0 | 0 |
| `ability_category:UNKNOWN` | 15 | 15 | 0 | 0 | 0 |
| `ability_category:Raging Blood Feat Bloodline` | 10 | 10 | 0 | 0 | 0 |
| `ability_category:Racial Size` | 9 | 9 | 0 | 0 | 0 |
| `ability_category:Arcanist Bloodline Development` | 8 | 8 | 0 | 0 | 0 |
| `ability_category:Blood Arcanist Bloodline` | 2 | 2 | 0 | 0 | 0 |
| `ability_category:Condition` | 2 | 2 | 0 | 0 | 0 |
| `ability_category:Natural Attack` | 1 | 1 | 0 | 0 | 0 |
| **TOTAL** | **5886** | **5108** | **210** | **560** | **8** |

**Bottom line: of the 5,886 units, 5,108 (86.8%) are disposition (A) — real, distinct, currently
uncounted objects that must be enumerated as a new tracked kind and classified into a shape family.
778 (13.2%) are disposition (B) — 8 exact-`KEY:` duplicates of a unit already counted under a
tracked kind, 210 gateway/wrapper rows, and 560 bare pick-list entries — proven by the class-wide
command above, not by per-instance assertion.**

## The shared-name hazard (why identity-string collision was not used as evidence)

An earlier pass in this cycle checked identity-*string* collisions between `ability_category:*`
rows and tracked-kind identities and found rates from 0% up to 88.2% (`Ability Focus`,
`Spell-Like Ability`). Per-record inspection showed every one of those is coincidental name reuse,
not a shared object: an `ability_category:Spell-Like Ability` row named "Brand" (`CATEGORY:Spell-Like
Ability`, `DEFINE:SLA_Brand_LVL|0` and four sibling `DEFINE:SLA_Brand_*` tokens tracking its own
uses/DC) is a completely different PCGen record from the `spell` kind's "Brand" (level/school/range
fields, no `SLA_*` tokens) — the two live in disjoint files, have no `KEY:` in common, and model
different game facts (an innate racial/monster ability vs. a castable spell). The **only** field
PCGen itself uses to resolve a cross-reference is `KEY:` (or a bare identity used consistently as an
implicit key *within* one file's own object population — never across two different `.lst` files'
populations). The classifier above therefore joins on `KEY:` only, never falling back to identity —
this is what keeps `B-duplicate` at 8, not the hundreds a naive name match would have produced.

## Per-bucket detail

### `Special Ability` — 3,436 units — **split: 3,363 A / 5 B-gateway / 61 B-picklist / 7 B-duplicate**

The largest bucket, by construction the one the operator flagged as most likely to double-count
(brief: "Special Ability at 3,436 units is the one most likely to be a double-count and the most
expensive to get wrong"). It is not: 97.9% is real, uncounted content.

**Books:** `horror_adventures` 546, `mythic_adventures` 384, `inner_sea_gods` 383,
`pathfinder_unchained` 364, `ultimate_psionics` 270, `ultimate_campaign` 257,
`advanced_race_guide` 251, `inner_sea_faiths` 152, `inner_sea_magic` 152,
`advanced_players_guide` 139, `core_rulebook` 123, `adventurers_guide` 108,
`inner_sea_races` 96, `advanced_class_guide` 87, `ultimate_equipment` 40, `core_essentials` 35,
`bestiary_3` 30, `ultimate_magic` 17, `book_of_the_damned_volume_2` 2. (Re-derive:
`jq '.[] | select(.bucket=="ability_category:Special Ability") | .book_id' -r
15-card-15-ability-category-rows.jsonl | sort | uniq -c | sort -rn`.)

**Why it is not the `class_feature` grant rows in disguise.** The walker only routes a bare
`*abilities.lst`/`*abilities_other.lst` row here when the *filename* does not match `_class` or
`_race`. `class_feature`-tracked files (`*_abilities_class*.lst`) hold **grant** rows — e.g.
`acg_abilities_class.lst` line for "Believer" has a bare
`ABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands,...` token that says "grant whichever ability
matches this TYPE" — while the mechanical *definition* those grants resolve to
(`KEY:Believer's Hands ~ Lay on Hands`, `CATEGORY:Special Ability`, `DEFINE:LayOnHandsTimes|0` /
`DEFINE:LayOnHandsDice|0` / `DEFINE:LayOnHandsLVL|0`, `BONUS:VAR|LayOnHandsDice|max(1,TL/4)`) lives
only in `acg_abilities_other.lst`, this bucket. Verified: `grep -rl --include="*_abilities_class*.lst"
"Believer's Hands ~ Lay on Hands" <corpus>` returns nothing — the grant and the definition are
disjoint records, not the same row counted twice.

**The 7 genuine duplicates** (exact `KEY:` match against `race_trait`/`monster_ability`, both
already tracked kinds) — real cross-book content reuse, not walker error: `core_essentials`
literally re-includes four of `advanced_players_guide`'s "Adopted Race ~ <Race>" rows verbatim
inside its own per-race `*_abilities_race.lst` files, and `bonus_bestiary`/`inner_sea_world_guide`
similarly re-include two `bestiary_3`/`book_of_the_damned_volume_2` rows:

```
Adopted Race ~ Half-Elf    -> also in race_trait (core_essentials/races/half_elf/halfelf_abilities_race.lst)
Adopted Race ~ Half-Orc    -> also in race_trait (core_essentials/races/half_orc/halforc_abilities_race.lst)
Adopted Race ~ Halfling    -> also in race_trait (core_essentials/races/halfling/halfling_abilities_race.lst)
Adopted Race ~ Human       -> also in race_trait (core_essentials/races/human/human_abilities_race.lst)
Faerie Dragon ~ Breath Weapon -> also in monster_ability (bonus_bestiary/bb_abilities_race.lst)
Nascent Demon Lord ~ Aligned Strike -> also in race_trait (inner_sea_world_guide/iswg_abilities_race.lst)
Nascent Demon Lord ~ Grant Spells   -> also in race_trait (inner_sea_world_guide/iswg_abilities_race.lst)
```

Re-derive: `jq 'select(.disposition=="B-duplicate" and (.bucket=="ability_category:Special Ability"))'
15-card-15-ability-category-rows.jsonl`.

**The 5 B-gateway + 61 B-picklist (66 units)** are the same two shapes documented generically
below: 5 rows are pure `ABILITY:...|AUTOMATIC|<target>` wrappers around another already-real
`Special Ability` row (e.g. "Add a Class Skill", `CHOOSE:ABILITYSELECTION|Class Skill|ANY`,
`ABILITY:Class Skill|AUTOMATIC|%LIST` — a real object in its own right, disposition A, whose
existence is what makes `ability_category:Class Skill` a pick-list target rather than orphaned
data); 61 rows are dead/placeholder markers — e.g. six `KEY:Adoptive Race ~ <Race>` rows literally
named `"Obsolete - Remove ~ <Race>"` with `PREVAREQ:CannotUse,1` (an unsatisfiable prerequisite —
PCGen's own convention for "this row is disabled, do not use").

**Shape-family sample (10 real, `A`-disposition records, canonical `F0`-`F10` vocabulary from
`artifacts/gate-1-shape-closure/family-vocabulary.md`, produced by
`scripts/shape_ledger.classify_formula` + `extract_formula_segment`, not a reimplementation):**

| identity | file | DEFINE/BONUS segment families |
|---|---|---|
| Lay on Hands | acg_abilities_other.lst | F1, F1, F1, F8 |
| Feat ~ Cosmopolitan (Internal, for comparison) | — | — |
| Champion of the Unbound Coven | acg_abilities_other.lst | (no DEFINE/BONUS — F0, narrative-only per DESC) |
| Aberrant Bloodline (Eldritch Heritage Bloodline, cross-ref) | apg_abilities.lst | F2, F1×5, F4×5, F8 |

(Full 15-row sample and command in "Method" above; every `A`-disposition row with a `DEFINE:`/
`BONUS*:` token classifies via the same reused function — no row was hand-picked to look good.)

**Ruling: enumerate as kind `ability` (or fold into an existing kind only if a future card shows a
cleaner join — none was found here). Count: 3,363. Books: as above. Shape families: predominantly
F1 (flat-constant `DEFINE:X|0` trackers) and F8 (residual/multi-term), per the classify_formula
sample; a majority of individual rows carry no DEFINE/BONUS at all and are F0 (text-only DESC
content) — that is a legitimate family per `family-vocabulary.md`, not a gap.**

### `Internal` — 839 units — **split: 685 A / 76 B-gateway / 78 B-picklist / 0 B-duplicate**

Genuinely mixed, as flagged in the dispatch brief — checked on its own terms, not by analogy.
`CATEGORY:Internal` is PCGen's own convention for non-player-facing bookkeeping abilities, and the
population matches that: pick-lists (`Elemental Fist ~ Acid/Cold/Electricity/Fire`,
`Outsider Plane Choice ~ <29 planes>`, `Size` choices — bare `TYPE:*Choice` rows, zero content, 78
units), gateway/tracker wrappers (`Racial Traits ~ Archon (Harbinger)` grants
`ABILITY:Special Ability|AUTOMATIC|Harbinger Archon ~ Blades|...|...`, bundling several already-real
Special Ability sub-features under one grantable package, 76 units), and real content (685 units:
e.g. `Aspect Combat Bonus ~ Low Profile`, `ASPECT:CombatBonus|+1 dodge bonus to AC against ranged
attacks...`; `Animal Companion / Primary`, an `F4` named-pool-variable tracker with its own
`BONUS:VAR`).

**Books:** `core_essentials` 253, `ultimate_magic` 101, `core_rulebook` 85,
`advanced_race_guide` 84, `inner_sea_gods` 74, `advanced_players_guide` 60,
`inner_sea_faiths` 47, `ultimate_psionics` 38, `adventurers_guide` 29, `mythic_adventures` 19,
`horror_adventures` 18, `bestiary_3` 15, `ultimate_equipment` 7, `ultimate_combat` 4,
`ultimate_campaign` 3, `bestiary_2` 2.

**Ruling: split the bucket. 685 units enumerate as kind `ability` (same kind as `Special Ability`
above — they are the same PCGen object type, `CATEGORY:Internal` is just a different value of the
same field). 154 units (76 gateway + 78 picklist) are not objects — proven by class via the
committed classifier's `has_content`/`has_gateway` predicates, both derived from the row's own raw
token set, not asserted.**

### `Words of Power` — 369 units — **split: 330 A / 0 B-gateway / 39 B-picklist / 0 B-duplicate**

Checked on its own terms per the dispatch brief. All 369 rows live in one file,
`ultimate_magic/um_abilities_wordsofpower.lst` — Ultimate Magic's alternate spellcasting subsystem.
330 are real, distinct "Words" (e.g. "Personal", `TYPE:WordsOfPowerOutput.TargetLevel0`,
`DESC:Spell targets only the caster.`; "Barrier", `DESC:Spell creates a barrier within Close range.
Boost: Larger barrier, any vertical shape`) or bloodline/class synergy grants ("Extra Word - Bard
0..6" etc., `BONUS:ABILITYPOOL|0-level Bard Word|1`, classify_formula → F1). The 39 `B-picklist`
rows are literal `Placeholder2`..`Placeholder9` (`TYPE:MetaLevelN`, `VISIBLE:NO`) and
per-class "Starter Word" markers with no fields beyond `CATEGORY:`/`TYPE:`/`VISIBLE:NO` — PCGen's
own placeholder convention, not content.

**Ruling: enumerate 330 as kind `ability` (Words-of-Power system content — text-only magnitude in
many rows' `DESC:` "Boost (N)" prose is a legitimate F0 shape, per the "text-only features are
complete" precedent — the boost cost is real mechanical content even without a `DEFINE:`/`BONUS:`
token). 39 `Placeholder*`/`Starter Word` rows are not objects.**

### `Ability Focus` — 272 units — **100% B-picklist**

Every row is `<Name>\tCATEGORY:Ability Focus\tTYPE:Ability Focus` with **zero** other fields —
verified: `grep -c "DEFINE:\|BONUS" <file>` → 0 across all 272. These are the enumerated valid
choices (e.g. "Breath Weapon", "Gaze", "Confusion", "Acid") for the real `feat` "Ability Focus"
(`core_essentials/ce_feats.lst`: `CHOOSE:ABILITY|Ability Focus|PC,TYPE=Ability Focus` — the feat's
own chooser pulls its pick-list directly from this category). The feat itself is already counted
under the tracked `feat` kind (1 unit). **Proof by class, not instance:** every row in the
population has exactly the same 2-field shape (`CATEGORY:`/`TYPE:`), confirmed by the committed
classifier's `has_content`=False and `has_gateway`=False for all 272 (`jq 'select(.bucket==
"ability_category:Ability Focus")' 15-card-15-ability-category-rows.jsonl | jq -s 'map(.disposition)
| group_by(.) | map({(.[0]): length}) | add'`).

**Ruling: 0 enumerable — 272 units are a facet (the chooser pick-list) of the already-tracked
`feat` "Ability Focus". Not in scope for a new kind.**

### `Spell-Like Ability` — 165 units — **split: 145 A / 0 B-gateway / 20 B-picklist / 0 B-duplicate**

Real, distinct content despite the 87.9% identity-string collision rate with `spell` (see "the
shared-name hazard" above): each row carries its own 5+ `DEFINE:SLA_<Name>_*` tokens
(`_LVL`, `_SpellLVL`, `_Times`, `_DCMod`, `_DC`) tracking a monster/race's specific spell-like-
ability usage independent of the underlying spell's own record. classify_formula sample (10 real
records): `Brand`, `Memory Lapse`, `Putrefy Food and Drink`, `Sift`, `Spark`, `Unwitting Ally`,
`Cloak of Shade`, `Feather Step`, `Hydraulic Push`, `Ill Omen` → each classifies as
`[F1×7, F2, F1, F4, F4, F2]` or `[F1×7, F4, F1×2, F3, F2]` across its `DEFINE:` segments (dominant:
F1 flat-constant trackers, F2/F4 for level-scaling and pool variables). 20 `B-picklist` rows are
bare category markers with no `DEFINE:`/`DESC:` content.

**Books:** `core_rulebook` 142, `advanced_players_guide` 18, `ultimate_magic` 3,
`monster_codex` 1, `ultimate_combat` 1.

**Ruling: enumerate 145 as kind `ability`, F1/F2/F4-dominant per the sample. 20 bare markers are
not objects.**

### `Path Dabbling` — 128 units — **100% B-gateway**

All 128 rows live in `mythic_adventures/ma_abilities.lst`. **Proof by class:** every row carries an
`ABILITY:Special Ability|AUTOMATIC|<KEY>` token, and every one of those 128 targets resolves to an
existing `KEY:` on a `CATEGORY:Special Ability` row in the *same file* — e.g. "Abundant Casting"
(`CATEGORY:Path Dabbling`, `ABILITY:Special Ability|AUTOMATIC|Mythic Path Ability ~ Abundant
Casting`) targets the real definition (`KEY:Mythic Path Ability ~ Abundant Casting`,
`CATEGORY:Special Ability`, already counted in this memo's `Special Ability` A-count). Verified:
128/128 matched (script in "Method"; the classifier's `has_gateway`=True for all 128, `has_content`
=False for all 128).

**Ruling: 0 enumerable — every unit is a gateway to a `Special Ability` row this memo already
counts. Not in scope for a new kind.**

### `Class Skill` — 102 units — **100% A**

Corrected mid-cycle from an initial facet call: each row carries its own `CSKILL:<Skill>` token
(e.g. `Acrobatics`, `CATEGORY:Class Skill`, `CSKILL:Acrobatics`) — a real, independently-acting
grant (makes that skill count as a class skill wherever applied), unlike `Ability Focus`'s pick-list
items, whose chosen value carries no token of its own and does nothing without the granting feat's
separate effect logic. `Class Skill` *is* referenced as a chooser target elsewhere (`core_rulebook`
"Add a Class Skill", `CHOOSE:ABILITYSELECTION|Class Skill|ANY`; `heroes_of_the_wild`'s "Fey
Thoughts Skill Choice"), but the chosen row itself still carries independent content, so it is
counted, not folded into the referencing row.

**Books:** `core_rulebook` 102 (the full core skill list, including `Craft`/`Perform`/`Profession`
sub-specialty expansions).

**Ruling: enumerate 102 as kind `ability`, shape family F0 (no `DEFINE:`/`BONUS:` token — the
`CSKILL:` field is outside `shape_ledger`'s current extraction rule, an honest gap logged below,
not silently absorbed into F0's "no formula content" framing without a note).**

### `Intelligent Item` — 100 units — **split: 22 A / 0 B-gateway / 78 B-picklist / 0 B-duplicate**

78 units are the `INT`/`WIS`/`CHA` stat-value pick-list (e.g. `INT 10`..`INT 20`, `KEY:Intelligent
Item Stat INT ~ 10`, `CATEGORY:Intelligent Item`, `TYPE:IntelligentItemStat`, no other field) — a
chooser pick-list for setting an intelligent item's ability scores, zero independent content. 22
units carry real content (e.g. `Abilities`, `KEY:Intelligent Item ~ Stat`,
`DESC:Intelligence %1, Wisdom %2, Charisma %3, Ego Score %1|IntItemStatINT|IntItemStatWIS|...`).

**Books:** `core_rulebook` 100.

**Ruling: enumerate 22 as kind `ability`. 78 stat-value picklist entries are not objects.**

### `Background` — 72 units — **100% A**

`inner_sea_world_guide` 64, `inner_sea_races` 8. Human-ethnicity/regional-background markers
(`Mwangi`, `Arcadian`, ..., `NAMEISPI:YES`, `CATEGORY:Background`, `TYPE:HumanEthnicity...`,
`VISIBLE:DISPLAY`) — real, distinct campaign-setting content (each ethnicity/background is its own
named, documented entity in Inner Sea World Guide), just not formula-bearing (F0).

**Ruling: enumerate 72 as kind `ability`, F0.**

### `Afflictions` — 70 units — **100% A**

`core_rulebook` 48, `ultimate_magic` 19, `ultimate_psionics` 2, `bestiary` 1. Real named diseases,
curses, and poisons with full mechanical prose in `DESC:` (e.g. "Rabies", `DESC:Disease, injury;
Fort DC 14; Onset 2d6 weeks; Frequency 1/day; Effect 1 Con damage plus 1d3 Wis damage...`) —
magnitude is encoded as text (DC/onset/frequency/effect), matching the "text-only features are
complete" precedent, not a `DEFINE:`/`BONUS:` formula. F0.

**Ruling: enumerate 70 as kind `ability`, F0.**

### `Save Bonus` — 58 units — **100% A**

`core_essentials` 58. Real content via `ASPECT:AllSaveBonus|+%1 vs. poison|SaveBonus_vs_Poison`
(a display-text field carrying a real variable-name substitution, `SaveBonus_vs_Poison`).

**Ruling: enumerate 58 as kind `ability`. `ASPECT:` is not one of `shape_ledger`'s current
`DEFINE`/`BONUS*` extraction keys, so these classify as F0 under the current extraction rule — an
honest gap (the variable substitution *is* real magnitude, just not surfaced by the shape ledger's
present field list), logged as a `scripts/retro.py correction` below.**

### `Aligned Class` — 52 units — **100% A**

`inner_sea_gods` 52. One row per PF1e base/prestige class (`Barbarian`, `Bard`, ..., `Arcane
Archer`, `Assassin`, `Dragon Disciple`, ...), each with `PREABILITY:1,CATEGORY=Class,Barbarian`
(references the tracked `class` kind by exact prereq, not duplicating it) — real content: which
deities/alignments each class is compatible with in Inner Sea setting terms. classify_formula
sample: 2 `F2` segments per row (references to that class's own level variable in a prerequisite
formula).

**Ruling: enumerate 52 as kind `ability`, F2.**

### `Eldritch Heritage Bloodline` — 31 units — **split: 27 A / 0 B-gateway / 4 B-picklist / 0 B-duplicate**

`ultimate_magic` 31. 27 are the 13 base sorcerer bloodlines + related content, each with real
`BONUS:VAR` mechanics (sample "Aberrant Bloodline": `F2, F1×5, F4×5, F8` across its `DEFINE`/`BONUS`
segments). 4 are `Level 3/9/15/20 Ability` PRE-guard tracker rows with no content beyond a
self-referential `!PREABILITY` (prevents re-taking the same tier twice) — bare trackers.

**Ruling: enumerate 27 as kind `ability` (F1/F2/F4-dominant). 4 tracker rows are not objects.**

### `Class` — 29 units — **split: 20 A / 1 B-gateway / 8 B-picklist / 0 B-duplicate**

`advanced_players_guide` 15, `advanced_class_guide` 10, `ultimate_combat` 3, `ultimate_magic` 1.
20 units (e.g. "Bloodrager", `DEFINE:BloodrageLVL|0`, `BONUS:VAR|BloodrageLVL|BloodragerLVL` — F1,
F2) carry real per-class tracker content used by cross-class synergy feats. 8 are bare class-name
markers (e.g. "Arcanist", `CATEGORY:Class` only) used purely as `PREABILITY:1,CATEGORY=Class,<X>`
prerequisite targets elsewhere.

**Ruling: enumerate 20 as kind `ability` (F1/F2). 8 bare class-name markers and 1 gateway row are
not objects.**

### `Racial Traits` — 28 units — **split: 27 A / 0 B-gateway / 0 B-picklist / 1 B-duplicate**

`advanced_race_guide` 27, `core_essentials` 1. Creature-type markers ("Humanoid", "Aberration", ...,
each with `TEMPLATE:<Type>` and real `PREFACT`/`PREMULT` gating) — real content, F1-dominant. One
exact-`KEY:` duplicate: "Skinwalker ~ Ability Scores" is defined identically in both
`core_essentials/races/skinwalker/skinwalker_abilities_arg.lst` (this bucket) and
`skinwalker_abilities_race.lst` (already counted as `race_trait`).

**Ruling: enumerate 27 as kind `ability`, F1. 1 unit already counted as `race_trait`.**

### `Archetype` — 27 units — **100% A**

`ultimate_magic` 13, `ultimate_psionics` 10, `adventurers_guide` 2, `core_rulebook` 2. Full
archetype-swap description rows (e.g. "Augmented Ninja", `KEY:Ninja Archetype ~ Augmented Ninja`,
real `DESC:` and `PRECLASS:`/`PREMULT:` gating, plus inline `ABILITY:...` grants of its own
sub-features which are separately-defined `class_feature`-tracked rows, not this row itself
duplicated). Genuinely distinct content — the archetype's own swap-eligibility rules and
description, not a restatement of the features it swaps in.

**Ruling: enumerate 27 as kind `ability`.**

### `Builder` — 24 units — **100% A**

`advanced_race_guide` 24. Race-builder point-buy components (e.g. `+2 Strength`, `KEY:
AbilityScoreBoost +2 ~ Strength`, `PREMULT` self-exclusion gating) — real, structured content
supporting the Advanced Race Guide's race-building subsystem.

**Ruling: enumerate 24 as kind `ability`.**

### `Equipment` — 21 units — **100% A**

`core_rulebook` 20, `ultimate_equipment` 1. The mechanical-effect definitions for specific magic
items' special powers (e.g. "Pearl of the Sirines ~ Swim Speed", `MOVE:Swim,60`) — same
grant/definition split as `Special Ability`: the base item ("Pearl of the Sirines",
`cr_equip_magic_items.lst`, already counted as `equipment`) carries only
`ABILITY:Equipment|AUTOMATIC|Pearl of the Sirines ~ Swim Speed`; the *only* place the actual
`MOVE:Swim,60` mechanical payload exists is this bucket's record. Verified zero `KEY:` collision
with the `equipment` kind.

**Ruling: enumerate 21 as kind `ability` — this is real, otherwise-uncounted magnitude (the base
`equipment` record for "Pearl of the Sirines" carries no `MOVE:` token itself).**

### `Mythic Weapon Training` — 16 units — **100% A**

`mythic_adventures` 16. Weapon-group pick-list values (e.g. "Axes Weapon Group") that at first
glance looked like a bare picklist (no `DEFINE`/`BONUS`/`DESC`), but each row carries
`AUTO:WEAPONPROF|TYPE.Weapon Group Axes` — an independent, real grant (automatic weapon
proficiency in that group) that fires without needing the referencing Mythic Path ability's own
logic. (The real "Mythic Weapon Training" Special Ability, already counted in this memo's `Special
Ability` A-count, pulls its `CHOOSE:ABILITYSELECTION` pick-list from this category — but the picked
row still does its own work via `AUTO:`.)

**Ruling: enumerate 16 as kind `ability`.**

### `UNKNOWN` — 15 units — **100% A, true category `Special Ability` (mislabeled by identity syntax)**

`ultimate_campaign` 15, all in `uca_abilities_retraining.lst`. These are `.COPY=` derivations whose
identity field uses the literal `CATEGORY=Special Ability|<Base>.COPY=<New>` syntax instead of a
separate tab-delimited `CATEGORY:` field — `_row_category_tag` (correctly) finds no `CATEGORY:`
token and falls through to `UNKNOWN`. Each is a real, distinct `.COPY=`-derived object (its own
`KEY:`, own `DESC:`, own `BONUS:ABILITYPOOL|...|N`): the base "Skill Ranks" (`KEY:Retrain ~ Skill
Ranks`, `CATEGORY:Special Ability`, line 15 of the same file — already counted in this memo's
`Special Ability` A-count) is `.COPY=`d into 15 numbered variants ("Retrain ~ 1 Skill Rank" through
"Retrain ~ 15 Skill Rank"), each with its own DESC ("You retrained N skill ranks.") and its own
`BONUS:ABILITYPOOL|Retraining Skill Ranks Removed|N`.

**Ruling: enumerate 15 as kind `ability`, true category `Special Ability` (a labeling correction,
not a new kind) — F1 (the `BONUS:ABILITYPOOL|...|N` values are flat-constant per variant).**

### `Raging Blood Feat Bloodline` — 10 units — **100% A**

`advanced_class_guide` 10. Real Bloodrager-bloodline-power synergy content (e.g. "Aberrant
Bloodline", `BONUS:VAR|Bloodrager_Aberrant_BloodlinePower1Times|3+CHA+BloodragerBloodlinePower1TimesBonus`
— a genuine F2/F3-blend formula, not a bare marker).

**Ruling: enumerate 10 as kind `ability`.**

### `Racial Size` — 9 units — **100% A**

`core_essentials` 9. Race-size-selection entries (`Fine`, `Diminutive`, ..., `KEY:Race Size ~ F`)
each carry `TEMPLATE:SIZE_F` — an independent, real grant (applies the size template) fired
directly from this row, not merely a label a chooser fills in blank.

**Ruling: enumerate 9 as kind `ability`.**

### `Arcanist Bloodline Development` / `Blood Arcanist Bloodline` — 8 + 2 = 10 units — **100% A**

`advanced_class_guide` 8 + 2. Real `BONUS:VAR` bloodline-power content
(`Bloodrager_Phoenix_BloodlineLVL`, etc.), same shape as `Raging Blood Feat Bloodline`.

**Ruling: enumerate 10 as kind `ability`.**

### `Condition` — 2 units — **100% A**

`core_rulebook` 2. Output-text trackers for accumulated ability damage/penalty
(`DESC:You have accumulated points of ability damage: %1 STR, ...`, six `DEFINE`-backed
substitutions) — real content.

**Ruling: enumerate 2 as kind `ability`.**

### `Natural Attack` — 1 unit — **100% A**

`core_rulebook` 1. "Natural Attack ~ Shield Bash", six `DEFINE:` tokens
(`ShieldBashingDieSizeStep`, `NaturalShieldReach`, `ShieldBashAttackBonus`, `ShieldDamageDice`,
`ShieldDamageSize`, ...) — real, fully mechanical content.

**Ruling: enumerate 1 as kind `ability`.**

## Sum the piles

```
5,886 total ability_category units (diff.json total_kind_unenumerable_units for these 26 keys)
  = 5,108 disposition A  (enumerate as kind `ability`, classify into a shape family)
  +   210 disposition B-gateway   (facet of an A-disposition row counted above)
  +   560 disposition B-picklist  (bare chooser value, not an object)
  +     8 disposition B-duplicate (exact-KEY match already counted as race_trait/monster_ability)
```

Re-derive the sum: `jq -s 'map(.disposition) | group_by(.) | map({(.[0]): length}) | add'
15-card-15-ability-category-rows.jsonl` → `{"A": 5108, "B-duplicate": 8, "B-gateway": 210,
"B-picklist": 560}`, and `5108+210+560+8 = 5886` ✓ against
`jq '[.kind_unenumerable | to_entries[] | select(.key | startswith("ability_category:")) | .value]
| add' artifacts/gate-0-census-closure/diff.json` → `5886` ✓.

## Discoveries / forwards for the integration cycle

1. **New kind `ability` needed.** All disposition-A rows across every category above are the same
   PCGen object type (an `ABILITY:` LST record whose `CATEGORY:` field happens to vary) — none of
   this memo's evidence supports splitting them into per-category kinds. The integration cycle
   should add `ability` as an eleventh tracked kind in `docs/work-inventory.json`'s vocabulary, with
   **5,108** units at this pin, sourced from the specific `(book, file, identity)` triples in
   `15-card-15-ability-category-rows.jsonl` where `disposition == "A"`.
2. **`shape_ledger.py`'s extraction rule doesn't see `CSKILL:`/`MOVE:`/`AUTO:`/`TEMPLATE:`/
   `SPROP:`/`QUALITY:`/`SR:`/`DR:`/`SAB:`/`VISION:`/`ASPECT:` tokens**, only `DEFINE:`/`BONUS*:`.
   For several categories in this memo (`Class Skill`, `Save Bonus`, `Equipment`, `Mythic Weapon
   Training`, `Racial Size`, `Background`, `Afflictions`) that is the *only* content signal present,
   so those units will classify as F0 ("no formula content") under the current extraction rule even
   though they carry a real, non-formula mechanical or narrative effect. This is not a
   miscount — F0 is a legitimate family — but it is a proof-width gap worth a
   `scripts/retro.py correction` (below) so a future card doesn't read "F0" as "the walker found
   nothing" for these units.
3. **8 exact-duplicate units** (`ability_category:Special Ability` ×7, `ability_category:Racial
   Traits` ×1) are cross-book content reuse already counted under `race_trait`/`monster_ability`.
   The integration cycle's `ability`-kind seed should exclude these 8 by the `(book, file,
   identity)` triples this memo names, not re-derive its own duplicate check.
4. **`UNKNOWN` (15 units) is a `census_independent.py` labeling gap**, not a real "unknown"
   category — its rows' true `CATEGORY:` is `Special Ability`, expressed via the nonstandard
   `CATEGORY=Special Ability|<Base>.COPY=<New>` identity syntax that `_row_category_tag` doesn't
   parse. Worth a one-line fix in a future card (out of this memo's write scope): recognize
   `CATEGORY=<X>|` as a leading identity prefix and extract `<X>` as the category before falling
   through to `UNKNOWN`.

## Retrospective events logged this cycle

Logged: `docs/retro/events/card-15-ability-category.jsonl`, id
`1787448814998-card-15-ability-category-4a1508` (`scripts/retro.py correction`) — the
identity-string-collision-as-double-count-signal error caught and corrected before this memo's
final draft, per "the shared-name hazard" section above.
