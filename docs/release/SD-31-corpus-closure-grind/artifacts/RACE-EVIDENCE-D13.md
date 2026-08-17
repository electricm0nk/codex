---
canonical: true
owner: sd31-register-race
cycle: SD31-D13-REG-001
authority: >
  decisions.md §13 AMENDMENT (2026-08-17 operator ruling): "devil is in the details.
  without examples of what you found, it's hard to give a ruling ... just to put it
  out there, the core rules might have a dwarf. then the arg maybe has something like
  a grey dwarf. those are two different things and deserve their own records. if it's
  just a dwarf and it says they can see 60 feet in the dark, and the later book says
  90 feet - go with 90 feet."
date: 2026-08-17
---

# SD-31 race supersession evidence table (`§13` amendment)

## 0. Read this first: no attribution changes here, PROPOSED classifications only

**This document changes nothing.** `RACE_TRUE_BOOK` and `RACE_NEWEST_PRINTING`
(`src/bin/v06_work_inventory.rs`) are untouched by this cycle -- outside this card's
write scope regardless. Race attribution stays **FROZEN** per `decisions.md §13`'s
amendment until the operator rules from this evidence. Every branch classification
below is **PROPOSED**, not decided.

This supersedes `SD31-ATTRIB-003-race-evidence.md` (2026-08-16) for the purpose of a
`§13` branch ruling: that document counted CITATIONS ("4/5 traits cited, 12 total
mentions") and never showed an actual field VALUE, so it could not answer the
question the operator actually asked. This document shows real values, real files,
real lines, for every field that could plausibly differ, and confirms -- rather than
assumes -- whether it does.

## 1. The structural fact, re-confirmed corpus-wide (not just two worked examples)

Every one of the 51 races under `core_essentials/races/<slug>/` has **exactly one
live base declaration** for both its `race`-kind object (the `RACE:` line) and every
one of its base `race_trait` abilities (Ability Scores / Type / Size / Speed /
Vision / etc, each its own `KEY:<Race> ~ <Trait>` ability). Every other book's own
`.lst` files carry only `.MOD` rows against those same keys.

**Corpus-wide scan, not a sample.** For every `<Race>.MOD` and every
`<Race> ~ <Trait>.MOD` row across all 37 in-mandate books (excluding
`core_essentials` itself and each book's own `_pfs/` Pathfinder-Society companion
directory, which duplicates content within the SAME book's release, not a second
printing):

```
python3 docs/release/SD-31-corpus-closure-grind/artifacts/race-evidence-scripts/race_evidence_gen.py   # full source in the sibling directory; re-derives from the pinned oracle
# -> 51 races with a base RACE: declaration
# -> 139 total RACE:.MOD citations across 47 multi-book races
# -> 0 citations carry a mechanical VALUE override (BONUS/VISION/MOVE/DESC-replace);
#    the only non-SOURCE*/TYPE tags found anywhere are 3 additive, non-overriding ones:
```

| race | citing book | non-citation tag found | what it does |
|---|---|---|---|
| Dwarf | `advanced_race_guide` | `TYPE:Core` | a taxonomy label, not a value |
| Half-Elf, Half-Orc, Human | `inner_sea_world_guide` | `TEMPLATE:Human Ethnicity` | attaches an OPTIONAL ethnicity template a player may apply; does not touch any existing field |

**Same corpus-wide result at the individual race-trait level**: of 799 distinct
`<Race> ~ <Trait>` KEYs declared under `core_essentials/races/`, **zero** in-mandate
`.MOD` row anywhere overrides `BONUS:`/`VISION:`/`MOVE:`/`DESC:` or any other
mechanical field -- confirmed by grepping every `.MOD` occurrence of every one of the
799 keys across all 37 books and checking for any tag beyond `SOURCE*`/`TYPE`. (Three
such overrides DO exist in the wider PCGen oracle -- `Drow ~ Weapon Familiarity` and
`Vishkanya ~ Weapon Familiarity` in `adventurers_armory_2`, `Orc ~ Weapon Familiarity`
in `orcs_of_golarion` -- but both are `player_companion` supplements **outside the
37-book mandate roster**, confirmed absent from `supersession_register_build.py`'s own
`BOOK_DIRS`. In-scope, the finding is unqualified: zero.)

**Consequence for `§13`'s branch test:** within the 37-book mandate, **no base race
object or base race trait has ever had its VALUE changed by a later book.** Every
later citation either (a) supplies only a page reference to the SAME shared
declaration (branch 1, identical -- the whole population in `§5`'s table), or (b)
introduces a genuinely NEW, separately-keyed object (an alternate racial trait, a
favored-class bonus, a favored-enemy entry) that coexists with the original rather
than replacing it (branch 2, a different thing).

## 2. The operator's own hypothetical, found for real: Dwarf darkvision, and why it is branch 2, not branch 3

The operator's illustrative case (*"if it's just a dwarf and it says they can see 60
feet in the dark, and the later book says 90 feet - go with 90 feet"*) does not occur
as a universal value change anywhere in-scope (§1). **The real corpus analog exists,
and it is structurally different from the hypothetical in a way that matters for the
ruling:**

`core_essentials/races/dwarf/dwarf_abilities_race.lst:15` (Dwarf's base declaration,
unmodified by any citing book):

```
Darkvision   KEY:Dwarf ~ Vision   CATEGORY:Special Ability   ...
  DESC:Dwarves can see in the dark up to 60 feet.
  BONUS:VAR|HasRacialVision|1   VISION:Darkvision (60)   SOURCEPAGE:p.21
```

`advanced_race_guide/arg_abilities_race.lst:39` (a SEPARATE, NEW key -- not a `.MOD`
of `Dwarf ~ Vision`):

```
Minesight   KEY:Dwarf ~ Minesight   CATEGORY:Special Ability
  PREMULT:1,[PREABILITY:1,...,Dwarf ~ Minesight],[!PREFACT:1,ABILITIES,Dwarf_ReplaceVision=true]
  DESC:Dwarves with this racial trait increase the range of their darkvision to 90
  feet; however, they are automatically dazzled in bright light and take a -2
  penalty on saving throws against effects with the light descriptor.
  DESC:This racial trait replaces darkvision.|!PREABILITY:...
  BONUS:VAR|HasRacialVision|1   VISION:Darkvision (90)   COST:0   SOURCEPAGE:p.12
  FACT:Dwarf_ReplaceVision|True
```

**This is not ARG updating every Dwarf's darkvision to 90 feet.** It is ARG (2012-06)
publishing an OPTIONAL alternate racial trait, `Dwarf ~ Minesight`, own `KEY`, own
`corpus_key`, own `race_trait` record, that a player may choose INSTEAD of the
standard `Dwarf ~ Vision` -- trading the light-sensitivity downside for the extended
range. `Dwarf ~ Vision` (60 ft) is untouched, still the default, still what every
Dwarf has unless the player picks Minesight. **This is branch 2 (a different thing,
own record, both stay), not branch 3 (the same thing, value replaced for everyone).**
`decisions.md §13`'s own text anticipates exactly this shape for ARG's Core Rulebook
chapters (*"ARG's core-race chapters add alternate racial traits -- they are not
identical"*) -- Minesight is the concrete, page-cited instance of it, not an inference.

**Corpus-wide, this pattern is not rare.** §5's table's "alternate traits added by
other books" column shows **48 of 51 races** have at least one book beyond their base
declaration's home contributing genuinely new, separately-keyed trait content -- **439
such new `race_trait` objects total**, none of them a `.MOD` of an existing key. If a
real branch-3 case exists anywhere in the in-scope corpus, it was not found by this
cycle's exhaustive `.MOD`-tag scan (§1); the operator should treat "no branch-3 cases
found" as this cycle's honest finding, not an assumption, and a future cycle finding
one should treat it as new evidence, not a contradiction of this scan's methodology.

## 3. THE CORRECTION: 5 "Bestiary 4" races were first published by Advanced Race Guide

`decisions.md §13`'s own text discusses Bestiary 4 as a **possible** exception
("Bestiary 4's 9 races: decided per race by the same comparison, not by book.
Re-derive."). Re-derived, with exact evidence:

| race | current attribution (`RACE_TRUE_BOOK`) | first `.MOD` citer by SOURCEDATE | file:line |
|---|---|---|---|
| Changeling | `bestiary_4` | **Advanced Race Guide (2012-06)** | `advanced_race_guide/arg_races.lst:36` |
| Kitsune | `bestiary_4` | **Advanced Race Guide (2012-06)** | `advanced_race_guide/arg_races.lst` |
| Nagaji | `bestiary_4` | **Advanced Race Guide (2012-06)** | `advanced_race_guide/arg_races.lst` |
| Samsaran | `bestiary_4` | **Advanced Race Guide (2012-06)** | `advanced_race_guide/arg_races.lst` |
| Wayang | `bestiary_4` | **Advanced Race Guide (2012-06)** | `advanced_race_guide/arg_races.lst` |

Verbatim, `advanced_race_guide/arg_races.lst:36`:

```
Changeling.MOD	TYPE:Uncommon	SOURCEPAGE:p.184
```

`advanced_race_guide/advanced_race_guide.pcc:18`: `SOURCEDATE:2012-06`.
`bestiary_4/_bestiary_4.pcc:21` and `_bestiary_4_for_players.pcc:21`:
`SOURCEDATE:2013-10`. **ARG is 16 months older.** Per §1, the base declaration ARG
and Bestiary 4 both cite is byte-identical at the data layer (both are pure
`SOURCEPAGE`-only citations of the SAME `core_essentials/races/changeling/` fields;
zero value differences). Under `§13` branch 1 (identical -> first print owns it),
**these 5 races' `race`-kind object belongs to Advanced Race Guide, not Bestiary 4.**

**This reverses the reasoning `RACE_NEWEST_PRINTING`'s own doc comment gives**
(`src/bin/v06_work_inventory.rs:1598-1606`), which explicitly computed this same
SOURCEDATE comparison (ARG 2012-06 vs Bestiary 4 2013-10) and concluded Bestiary 4 is
"the NEWER printing... the current attribution is already correct" -- **that
conclusion was correct under Decision 10's original "newest wins" direction and is
now backwards under `§13`'s correction.** Nothing in that file has been edited by this
cycle (frozen, outside write scope); this is flagged as evidence for the operator's
ruling, not fixed.

**The other 4 of Bestiary 4's 9 races are NOT affected** -- Kasatha, Trox, Wyrwood,
Wyvaran have no ARG citation at all (§5's table); Bestiary 4 genuinely is their first
and only pre-`inner_sea_races` printing.

## 4. Worked example: Catfolk, the operator's own §10 case, corrected under §13

`decisions.md §10`'s worked example (*"catfolk exists as a race in beastiary and
advanced race guide - thats a duplicate. most recent publish wins"*) was applied under
the OLD direction and moved Catfolk to ARG. Re-derived under `§13`:

| book | SOURCEDATE | citation | file:line |
|---|---|---|---|
| Bestiary 3 | **2012-01** | `Catfolk.MOD SOURCEPAGE:p.47` | `bestiary_3/b3_races_pc.lst:6` |
| Advanced Race Guide | 2012-06 | `Catfolk.MOD TYPE:Featured SOURCEPAGE:p.91` | `advanced_race_guide/arg_races.lst:18` |
| Inner Sea Races | 2015-09 | `Catfolk.MOD SOURCEPAGE:p.239` | `inner_sea_races/isr_races.lst` |

Bestiary 3 predates ARG by 5 months. Base fields identical (§1's global scan). Under
`§13` branch 1: **Catfolk belongs to Bestiary 3, not Advanced Race Guide** -- exactly
the outcome `§13`'s own text predicts (*"If ARG merely reprints the base traits,
Bestiary 3 owns it — the opposite of §10's recorded outcome"*), now confirmed with the
actual file citations rather than asserted.

## 5. Full per-race table -- every race printed by 2+ in-mandate books (47 of 51)

**Branch is PROPOSED for every row; the operator classifies.** "Base fields differ?"
is corpus-wide re-confirmed NO for all 47 (§1) -- no row in this table has ever shown
a value difference at the `.MOD` level; every citing book beyond the first only adds a
page reference (SOURCEPAGE) or, per the flagged rows, one additive non-value tag.
"Alternate traits added by other books" counts genuinely NEW `race_trait` KEYs (own
`corpus_key`, branch-2-shaped, already their own records in the corpus, not part of
this race-object comparison) -- shown so the operator can see how much real NEW
content each later book actually contributes versus pure re-citation.

| Race | Books citing the base declaration (SOURCEDATE, page) | Base fields differ across books? | Alternate traits added by other books (own KEY, own corpus_key -- branch 2) | Proposed branch |
|---|---|---|---|---|
| Aasimar | Bestiary (2009-10, p.7) **[FIRST]**<br>Advanced Race Guide (2012-06, p.85)<br>Inner Sea Races (2015-09, p.238) | NO (0 non-citation field diffs) | Advanced Race Guide +9; Inner Sea Races +2 | **1 (identical base — first print owns it)** |
| Android | Inner Sea Bestiary (2013-06, p.3) **[FIRST]**<br>Inner Sea Races (2015-09, p.239)<br>Bestiary 5 (2015-12, p.19) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Catfolk | Bestiary 3 (2012-01, p.47) **[FIRST]**<br>Advanced Race Guide (2012-06, p.91)<br>Inner Sea Races (2015-09, p.239) | NO (0 non-citation field diffs) | Advanced Race Guide +6; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Changeling | Advanced Race Guide (2012-06, p.184) **[FIRST]**<br>Bestiary 4 (2013-10, p.29)<br>Inner Sea Races (2015-09, p.240) | NO (0 non-citation field diffs) | Inner Sea Races +2 | **1 (identical base — first print owns it)** |
| Dhampir | Bestiary 2 (2010-12, p.89) **[FIRST]**<br>Advanced Race Guide (2012-06, p.97)<br>Inner Sea Races (2015-09, p.240) | NO (0 non-citation field diffs) | Advanced Race Guide +3; Inner Sea Races +2 | **1 (identical base — first print owns it)** |
| Drow | Bestiary (2009-10, p.114) **[FIRST]**<br>Advanced Race Guide (2012-06, p.103)<br>Inner Sea Races (2015-09, p.241) | NO (0 non-citation field diffs) | Advanced Race Guide +7; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Duergar | Bestiary (2009-10, p.117) **[FIRST]**<br>Advanced Race Guide (2012-06, p.186)<br>Inner Sea Races (2015-09, p.241) | NO (0 non-citation field diffs) | Advanced Race Guide +3; Monster Codex +2; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Dwarf | Core Rulebook (2009-08, p.21) **[FIRST]**<br>Advanced Race Guide (2012-06, p.10)<br>Inner Sea Races (2015-09, p.236) | NO (0 non-citation field diffs) | Advanced Race Guide +17; Advanced Player's Guide +8; Inner Sea Races +7; horror_adventures +6 | **1 (identical base — first print owns it)** |
| Elf | Core Rulebook (2009-08, p.22) **[FIRST]**<br>Advanced Race Guide (2012-06, p.20)<br>Inner Sea Races (2015-09, p.236) | NO (0 non-citation field diffs) | Advanced Race Guide +13; Advanced Player's Guide +7; horror_adventures +7; Inner Sea Races +7 | **1 (identical base — first print owns it)** |
| Fetchling | Bestiary 2 (2010-12, p.123) **[FIRST]**<br>Advanced Race Guide (2012-06, p.109)<br>Inner Sea Races (2015-09, p.242) | NO (0 non-citation field diffs) | Advanced Race Guide +5; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Gathlain | Bestiary 4 (2013-10, p.122) **[FIRST]**<br>Inner Sea Races (2015-09, p.243)<br>Ultimate Wilderness (2017-11, p.9) | NO (0 non-citation field diffs) | Ultimate Wilderness +7; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Ghoran | Inner Sea Bestiary (2013-06, p.14) **[FIRST]**<br>Inner Sea Races (2015-09, p.243)<br>Bestiary 5 (2015-12, p.119) | NO (0 non-citation field diffs) | Ultimate Wilderness +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Gillman | Inner Sea World Guide (2011-03, p.310) **[FIRST]**<br>Advanced Race Guide (2012-06, p.188)<br>Inner Sea Races (2015-09, p.243) | NO (0 non-citation field diffs) | Advanced Race Guide +3; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Gnome | Core Rulebook (2009-08, p.23) **[FIRST]**<br>Advanced Race Guide (2012-06, p.30)<br>Inner Sea Races (2015-09, p.237) | NO (0 non-citation field diffs) | Advanced Race Guide +12; Advanced Player's Guide +7; Inner Sea Races +6; horror_adventures +5 | **1 (identical base — first print owns it)** |
| Goblin | Bestiary (2009-10, p.156) **[FIRST]**<br>Advanced Race Guide (2012-06, p.115)<br>Inner Sea Races (2015-09, p.244) | NO (0 non-citation field diffs) | Advanced Race Guide +6; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Grippli | Bestiary 2 (2010-12, p.149) **[FIRST]**<br>Advanced Race Guide (2012-06, p.190)<br>Inner Sea Races (2015-09, p.244) | NO (0 non-citation field diffs) | Advanced Race Guide +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Half-Elf | Core Rulebook (2009-08, p.24) **[FIRST]**<br>Inner Sea World Guide (2011-03, ?) +['TEMPLATE:Human Ethnicity']<br>Advanced Race Guide (2012-06, p.40)<br>Inner Sea Races (2015-09, p.237) | NO (0 non-citation field diffs) | Advanced Race Guide +9; Inner Sea Races +7; Advanced Player's Guide +6; horror_adventures +5 | **1 for base, but SEE non-citation tag note** |
| Half-Orc | Core Rulebook (2009-08, p.25) **[FIRST]**<br>Inner Sea World Guide (2011-03, ?) +['TEMPLATE:Human Ethnicity']<br>Advanced Race Guide (2012-06, p.50)<br>Inner Sea Races (2015-09, p.237) | NO (0 non-citation field diffs) | Advanced Race Guide +14; Advanced Player's Guide +10; Inner Sea Races +7; horror_adventures +6 | **1 for base, but SEE non-citation tag note** |
| Halfling | Core Rulebook (2009-08, p.26) **[FIRST]**<br>Advanced Race Guide (2012-06, p.60)<br>Inner Sea Races (2015-09, p.238) | NO (0 non-citation field diffs) | Advanced Race Guide +13; Advanced Player's Guide +8; horror_adventures +7; Inner Sea Races +7 | **1 (identical base — first print owns it)** |
| Hobgoblin | Bestiary (2009-10, p.175) **[FIRST]**<br>Advanced Race Guide (2012-06, p.121)<br>Inner Sea Races (2015-09, p.244) | NO (0 non-citation field diffs) | Advanced Race Guide +9; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Human | Core Rulebook (2009-08, p.27) **[FIRST]**<br>Inner Sea World Guide (2011-03, ?) +['TEMPLATE:Human Ethnicity']<br>Advanced Race Guide (2012-06, p.70)<br>Inner Sea Races (2015-09, p.238) | NO (0 non-citation field diffs) | Advanced Race Guide +15; Inner Sea Races +13; horror_adventures +6; Advanced Player's Guide +4 | **1 for base, but SEE non-citation tag note** |
| Ifrit | Bestiary 2 (2010-12, p.160) **[FIRST]**<br>Advanced Race Guide (2012-06, p.127)<br>Inner Sea Races (2015-09, p.245) | NO (0 non-citation field diffs) | Advanced Race Guide +8; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Kasatha | Bestiary 4 (2013-10, p.174) **[FIRST]**<br>Inner Sea Races (2015-09, p.245) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Kitsune | Advanced Race Guide (2012-06, p.192) **[FIRST]**<br>Bestiary 4 (2013-10, p.175)<br>Inner Sea Races (2015-09, p.245) | NO (0 non-citation field diffs) | Advanced Race Guide +3; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Kobold | Bestiary (2009-10, p.183) **[FIRST]**<br>Advanced Race Guide (2012-06, p.133)<br>Inner Sea Races (2015-09, p.245) | NO (0 non-citation field diffs) | Advanced Race Guide +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Lashunta | Inner Sea Bestiary (2013-06, p.25) **[FIRST]**<br>Inner Sea Races (2015-09, p.246) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Merfolk | Bestiary (2009-10, p.204) **[FIRST]**<br>Advanced Race Guide (2012-06, p.194)<br>Inner Sea Races (2015-09, p.246) | NO (0 non-citation field diffs) | Advanced Race Guide +3; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Nagaji | Advanced Race Guide (2012-06, p.196) **[FIRST]**<br>Bestiary 4 (2013-10, p.196)<br>Inner Sea Races (2015-09, p.246) | NO (0 non-citation field diffs) | Advanced Race Guide +1; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Orc | Bestiary (2009-10, p.222) **[FIRST]**<br>Advanced Race Guide (2012-06, p.139)<br>Inner Sea Races (2015-09, p.246) | NO (0 non-citation field diffs) | Advanced Race Guide +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Oread | Bestiary 2 (2010-12, p.205) **[FIRST]**<br>Advanced Race Guide (2012-06, p.145)<br>Inner Sea Races (2015-09, p.247) | NO (0 non-citation field diffs) | Advanced Race Guide +8; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Ratfolk | Bestiary 3 (2012-01, p.231) **[FIRST]**<br>Advanced Race Guide (2012-06, p.151)<br>Inner Sea Races (2015-09, p.247) | NO (0 non-citation field diffs) | Advanced Race Guide +4; Monster Codex +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Samsaran | Advanced Race Guide (2012-06, p.198) **[FIRST]**<br>Bestiary 4 (2013-10, p.198)<br>Inner Sea Races (2015-09, p.247) | NO (0 non-citation field diffs) | Advanced Race Guide +1; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Skinwalker | Inner Sea Races (2015-09, p.248) **[FIRST]**<br>Bestiary 5 (2015-12, p.233) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Strix | Inner Sea World Guide (2011-03, p.313) **[FIRST]**<br>Advanced Race Guide (2012-06, p.200) | NO (0 non-citation field diffs) | Advanced Race Guide +5; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Suli | Bestiary 3 (2012-01, p.258) **[FIRST]**<br>Advanced Race Guide (2012-06, p.202)<br>Inner Sea Races (2015-09, p.250) | NO (0 non-citation field diffs) | Advanced Race Guide +5; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Svirfneblin | Bestiary (2009-10, p.261) **[FIRST]**<br>Advanced Race Guide (2012-06, p.204)<br>Inner Sea Races (2015-09, p.250) | NO (0 non-citation field diffs) | Advanced Race Guide +2; Inner Sea Races +2 | **1 (identical base — first print owns it)** |
| Sylph | Bestiary 2 (2010-12, p.258) **[FIRST]**<br>Advanced Race Guide (2012-06, p.157)<br>Inner Sea Races (2015-09, p.250) | NO (0 non-citation field diffs) | Advanced Race Guide +8; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Syrinx | Inner Sea Bestiary (2013-06, p.51) **[FIRST]**<br>Inner Sea Races (2015-09, p.251) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Tengu | Bestiary (2009-10, p.263) **[FIRST]**<br>Advanced Race Guide (2012-06, P.163)<br>Inner Sea Races (2015-09, p.251) | NO (0 non-citation field diffs) | Advanced Race Guide +4; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Tiefling | Bestiary (2009-10, p.264) **[FIRST]**<br>Advanced Race Guide (2012-06, P.169)<br>Inner Sea Races (2015-09, p.251) | NO (0 non-citation field diffs) | Advanced Race Guide +7; Inner Sea Races +3 | **1 (identical base — first print owns it)** |
| Trox | Bestiary 4 (2013-10, p.264) **[FIRST]**<br>Inner Sea Races (2015-09, p.253) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Undine | Bestiary 2 (2010-12, p.275) **[FIRST]**<br>Advanced Race Guide (2012-06, p.175)<br>Inner Sea Races (2015-09, p.253) | NO (0 non-citation field diffs) | Advanced Race Guide +9; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Vanara | Bestiary 3 (2012-01, p.280) **[FIRST]**<br>Advanced Race Guide (2012-06, p.206)<br>Inner Sea Races (2015-09, p.253) | NO (0 non-citation field diffs) | Advanced Race Guide +2; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Vishkanya | Bestiary 3 (2012-01, p.281) **[FIRST]**<br>Advanced Race Guide (2012-06, p.208)<br>Inner Sea Races (2015-09, p.253) | NO (0 non-citation field diffs) | Advanced Race Guide +2; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Wayang | Advanced Race Guide (2012-06, p.274) **[FIRST]**<br>Bestiary 4 (2013-10, p.274)<br>Inner Sea Races (2015-09, p.254) | NO (0 non-citation field diffs) | Advanced Race Guide +1; Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Wyrwood | Bestiary 4 (2013-10, p.280) **[FIRST]**<br>Inner Sea Races (2015-09, p.254) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |
| Wyvaran | Bestiary 4 (2013-10, p.281) **[FIRST]**<br>Inner Sea Races (2015-09, p.254) | NO (0 non-citation field diffs) | Inner Sea Races +1 | **1 (identical base — first print owns it)** |

**[†]** Half-Elf, Half-Orc, Human: `inner_sea_world_guide` additionally attaches
`TEMPLATE:Human Ethnicity` (§1) -- additive (an optional ethnicity template), not a
value override of any existing field; does not change the branch-1 classification of
the base race object.

## 6. Races printed in exactly one in-mandate book (not in §5's table)

- **Triaxian** — cited only by `inner_sea_races` in-mandate (a second citation exists
  in `player_companion/people_of_the_stars`, outside the 37-book roster). Single-book;
  no supersession question.
- **Rougarou** — its base `RACE:` line (`Rougarou.MOD`) is cited by **no book at all**,
  including its current home `bestiary_6`; `bestiary_6` instead cites Rougarou's
  individual TRAIT keys directly (`Rougarou ~ Ability Scores.MOD`, etc,
  `bestiary_6/b6_abilities_race_pc.lst:33-40`) without ever citing the RACE line
  itself. No other book cites any Rougarou trait either. Genuinely single-book;
  flagged here only because the RACE-line citation signal alone would have silently
  read as "0 books print this race" rather than "1 book, cited at trait level."

## 7. Methodology — exact commands, reproduce this from the pinned oracle

```
export PCGEN_ORACLE_SHA=$(grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env | cut -d= -f2)
export PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data

# 1. Per-race RACE:.MOD citations, SOURCEDATE, SOURCEPAGE, and any non-SOURCE*/TYPE tag:
python3 docs/release/SD-31-corpus-closure-grind/artifacts/race-evidence-scripts/race_evidence_gen.py   # writes race-citations.json next to itself

# 2. Alternate-trait (branch-2-shaped) NEW KEY declarations per race per book:
python3 docs/release/SD-31-corpus-closure-grind/artifacts/race-evidence-scripts/race_alt_traits.py     # writes race-alt-traits.json next to itself

# 3. Corpus-wide scan for ANY .MOD row on ANY of the 799 race-trait KEYs carrying a
#    mechanical tag beyond SOURCE*/TYPE (the branch-3 detector):
python3 docs/release/SD-31-corpus-closure-grind/artifacts/race-evidence-scripts/race_mod_scan.py       # 0 hits in-mandate
```

All three scripts are included verbatim in this artifact's companion directory
(`docs/release/SD-31-corpus-closure-grind/artifacts/race-evidence-scripts/`) so the
operator or a future cycle can re-run them unmodified against a later oracle pin.
`BOOK_DIRS` (the 37-book in-mandate roster) is imported directly from
`supersession_register_build.py`, never re-typed, so the two artifacts can never
silently disagree on which books are in scope.

## 8. What this document does NOT do

- Does not change `RACE_TRUE_BOOK`, `RACE_NEWEST_PRINTING`, or any unit's `book`
  attribution — frozen per `§13`'s amendment, and outside this card's write scope
  regardless.
- Does not classify any race — every branch column in §5 is **PROPOSED**, and §§3-4's
  named corrections are evidence for a ruling, not a ruling.
- Does not touch the Supersession Register (`SUPERSESSION-REGISTER.{md,json}`) —
  races are a SEPARATE population from that register's 116 objects (none of which are
  `kind: race`; the register's evidence bar requires two independent full `.lst` rows
  for the SAME key, which races structurally never have — see §1).
- Does not assert that zero branch-3 cases exist in the WHOLE PCGen oracle — only that
  none exist within the 37-book mandate roster, confirmed by an exhaustive tag scan,
  not a sample.
