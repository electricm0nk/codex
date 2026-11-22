# Card 15 -- CATEGORY:Internal (`_abilities_class.lst`) per-row disposition

Rows found (independent re-walk, bucket=`row_dependent_class_feature`, cat=INTERNAL): **2614** (expected 2614 -- MATCH)

**Disposition of record: 2371 (A) / 243 (B)** (90.7% / 9.3%).

## Disposition (full content test -- see module docstring for the field list; disposition-of-record)

| disposition | count | meaning |
|---|---:|---|
| A | 2369 | proven independent mechanical/narrative content |
| A-unresolved-gateway | 2 | gateway token whose target this script could not resolve -- **not proven (B)**, so stays (A) per decisions.md §12b's burden of proof |
| B-gateway-resolved | 203 | proven facet -- gateway target resolves to an already-counted real object |
| B-picklist | 40 | proven inert -- zero content field, zero gateway token |
| B-duplicate | 0 | exact KEY: match on a tracked kind elsewhere |
| **TOTAL** | **2614** | |

## Content-test comparison (same 2,614 rows, four definitions)

| test | rows matching | rows NOT matching |
|---|---:|---:|
| base (ability_category classifier's own list) | 1034 | 1580 |
| extended (+ SPELLKNOWN*/TEMPBONUS -- verifier's 6 token families) | 2219 | 395 |
| full (+ CHOOSE/NATURALATTACKS/COMPANIONLIST/ADD/FOLLOWERS/UDAM/UMULT/SELECT/COST/MOVECLONE/SPELLS/SERVESAS/DEFINESTAT/UNENCUMBEREDMOVE/BENEFIT -- this script's disposition-of-record) | 2369 | 245 |
| formula-only (DEFINE/BONUS*, class_feature memo's/shape_ledger's test) | 677 | 1937 |

## Gateway resolution (own KEY:-or-identity join, scoped per target kind; resolution universe = tracked kinds + inventory's tracked class_feature units + ability_category lane's own disposition-A rows + sibling A-disposition rows within this same 2,614-row population)

- rows with an `ABILITY:...\|AUTOMATIC\|<target>` token: 437
- of those, target resolves to an already-counted unit: 382
- of those, target does NOT resolve (stays disposed A -- not proven B): 55

## Rows with neither full content nor a gateway token: 40

## Per-token presence (not mutually exclusive; a row may carry several)

| token | rows carrying it |
|---|---:|
| SPELLKNOWN | 1185 |
| BONUS | 605 |
| ABILITY | 437 |
| DESC | 167 |
| BENEFIT | 154 |
| DEFINE | 151 |
| CSKILL | 131 |
| CHOOSE | 117 |
| UDAM | 113 |
| UMULT | 113 |
| TEMPBONUS | 70 |
| SPELLLEVEL | 69 |
| AUTO | 38 |
| COST | 36 |
| TEMPLATE | 23 |
| COMPANIONLIST | 23 |
| FOLLOWERS | 18 |
| NATURALATTACKS | 15 |
| MOVE | 12 |
| SELECT | 10 |
| SERVESAS | 9 |
| ADD | 8 |
| SPELLS | 4 |
| DR | 3 |
| UNENCUMBEREDMOVE | 3 |
| MOVECLONE | 2 |
| DEFINESTAT | 2 |
| ASPECT | 1 |
| VISION | 1 |

## Per-book distribution of disposition A (A + A-unresolved-gateway)

| book | A rows |
|---|---:|
| ultimate_magic | 710 |
| core_rulebook | 505 |
| advanced_class_guide | 269 |
| advanced_players_guide | 205 |
| ultimate_intrigue | 154 |
| ultimate_psionics | 121 |
| ultimate_combat | 120 |
| ultimate_wilderness | 77 |
| advanced_race_guide | 63 |
| adventurers_guide | 52 |
| occult_adventures | 33 |
| pathfinder_unchained | 25 |
| inner_sea_intrigue | 11 |
| horror_adventures | 9 |
| inner_sea_magic | 6 |
| monster_codex | 4 |
| inner_sea_world_guide | 3 |
| bestiary_6 | 2 |
| bestiary_4 | 1 |
| inner_sea_combat | 1 |
