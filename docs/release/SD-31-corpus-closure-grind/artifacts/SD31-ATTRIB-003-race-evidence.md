---
canonical: true
cycle: SD31-ATTRIB-003
purpose: >
  Per-race book-citation evidence for the operator's own question ("should the Core
  Rulebook's 7 races move to the Advanced Race Guide under Decision 10, and is Bestiary
  4 an exception?"). This is the deliverable itself, not a summary of one -- no
  attribution is changed by this document.
date: 2026-08-16
---

# SD-31 per-race citation evidence table

## 0. Read this first: no attribution changes here

**This document changes nothing.** `RACE_TRUE_BOOK` and `RACE_NEWEST_PRINTING`
(`src/bin/v06_work_inventory.rs`) are untouched by this cycle. The operator has not
ruled on the CRB-vs-ARG or Bestiary-4-exception question; this table exists so the next
ruling is made against evidence, not against a re-quoted headline.

## 1. The decisive structural fact, re-derived this cycle

Every race in the 43-race `RACE_TRUE_BOOK` roster (see §5's correction — not 44) has
**exactly one real base declaration in the whole corpus**, and it always lives under
`core_essentials/races/<slug>/<slug>_races.lst`. Every other book, **including the
book currently credited as the race's "true book"**, supplies only `.MOD` citation rows
layered on top of that one shared declaration — never a second independent base
declaration.

Verified for two worked examples, one CRB-attributed and one Bestiary-4-attributed:

```
grep -n "^Dwarf\b" core_essentials/races/dwarf/dwarf_races.lst
#   6:Dwarf   SORTKEY:a_base_pc ... SOURCEPAGE:p.xx FACT:IsPC|true
grep -n "^Dwarf\.MOD" core_rulebook/cr_races.lst
#   7:Dwarf.MOD   SOURCEPAGE:p.21
grep -rn "^Changeling\b" core_essentials/races/changeling/*.lst | grep -v '\.MOD'
#   changeling_races.lst:6:Changeling  SORTKEY:a_base_pc ... SOURCEPAGE:p.xx FACT:IsPC|True
grep -rn "^Changeling\b" bestiary_4/*.lst | grep -v '\.MOD'
#   (no output -- bestiary_4 has NO base Changeling declaration at all)
```

**Consequence: "which book prints this race" is not a question of two competing full
stat blocks.** It is a question of which book's `.MOD` citation currently supplies the
real `SOURCEPAGE` the shared record renders (the home file itself carries the `p.xx`
placeholder until a citing book's `.MOD` row overwrites it -- the mechanism the dispatch
brief names, confirmed here for every race checked, not just the one worked example).
Decision 10's "newest printing wins" is being applied to *citation precedence*, not to
two independently-authored printings of the same object.

## 2. A second structural fact this table surfaces: Inner Sea Races distorts a naive read

`campaign_setting/inner_sea_races/isr_abilities_race.lst` (`SOURCELONG:Inner Sea Races`,
`SOURCEDATE:2015-09`) supplies full 4/5-or-5/5 base-trait `.MOD` citations for **every
one of the 43 `RACE_TRUE_BOOK` races**, all with real page numbers (Dwarf `p.236`, etc.)
-- and `isr_races.lst` itself is **100% `.MOD` rows, zero base `RACE:` declarations**:

```
grep -c "^RACE:" campaign_setting/inner_sea_races/isr_races.lst   # -> 0
wc -l campaign_setting/inner_sea_races/isr_races.lst               # -> 62, all .MOD/comment lines
```

Applied literally, "most recent `SOURCEDATE` among books that cite the base traits"
would move **every** CRB/Bestiary-1/2/3 race (and most of ISWG's) to Inner Sea Races
(2015-09), not to the Advanced Race Guide (2012-06) — because ISR is itself newer than
ARG. Inner Sea Races is a genuine, real Paizo sourcebook (a citation-and-cross-reference
compendium of the whole Inner Sea setting's races), not a PCGen-internal construct like
`core_essentials` — so Decision 9's "not a book" answer does not apply to it — but it
never independently *prints* a race the way the dispatch's own worked example
(Changeling in Bestiary 4) does. **Every table below reports the newest-citer verdict
both including and excluding Inner Sea Races**, so the operator can see the effect
directly rather than have it silently decided by whichever a script happened to prefer.
This is a new open question this table surfaces, not a resolved one: logged at
`OPEN-ISSUES.md` row 129.

## 3. Methodology — exact commands, re-run this to reproduce

Oracle pin: `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), confirmed via `./scripts/verify.sh --only preflight-oracle`.

- **Roster (51 races):** the 43-entry `RACE_TRUE_BOOK` table + the 8 `core_essentials`-
  ambiguous races named in `decisions.md` Decision 9 (`android`, `aquatic_elf`,
  `gathlain`, `ghoran`, `monkey_goblin`, `lashunta`, `syrinx`, `triaxian`), copied
  verbatim from `src/bin/v06_work_inventory.rs` and re-verified against
  `docs/work-inventory.json`'s own `kind=='race'` units.
- **Book roster (37 books):** every distinct `book` value in `docs/work-inventory.json`
  that resolves to a real directory on the pinned oracle (`roleplaying_game/`,
  `campaign_setting/`, or `player_companion/` under
  `$PCGEN_CORPUS_ROOT/pathfinder/paizo/`) — not a hand-typed list, and not a blind
  directory walk (which would also sweep in unrelated lines like `adventure_path`).
- **Per-book citation count:** for each race, each book's own TOP-LEVEL
  `*abilities_race*.lst` file(s) (excluding nested `_pfs/`/`support/` compatibility
  files) are scanned. A row counts as citing the race only when its FIELD-1 token,
  split on the last `|` and then on ` ~ `, has its race-name half **exactly equal** to
  the race name — not a substring test. (§4 explains why this matters and what it
  caught.) Of those rows, the ones shaped `<Name> ~ <BaseTrait>.MOD` count toward the
  5 base traits (Ability Scores, Type, Size, Speed, Vision).
- **SOURCEDATE:** each book's own top-level `.pcc` (`_for_players` variant preferred for
  the Bestiary line, matching Decision 9's own precedent; falls back to the plain
  `<book>.pcc`), `SOURCEDATE:` field, read directly — never dated from memory.
- **Script:** written this cycle, `scratch_race_table.py` (scratch, not committed —
  the output below and its regeneration command are what's committed). Re-run:
  ```
  python3 scratch_race_table.py /tmp/race_evidence.json docs/work-inventory.json
  python3 scratch_race_table_md.py /tmp/race_evidence.json
  ```

## 4. A real bug this cycle found and fixed in its own draft — worth recording

A first-draft version matched race names as a plain substring (later, a single-character
word-boundary regex, which is still not enough). Both wrongly matched **"Goblin" inside
"Monkey Goblin ~ Ability Scores.MOD"** (Bestiary 6's own, unrelated race — the space
before "Goblin" satisfies a naive word-boundary check) and reported Bestiary 6 as citing
the CRB/Bestiary-1 **Goblin**, which would have shown Goblin "moving" to Bestiary 6 under
strict newest-wins with **zero real evidence** (`grep -n "Goblin ~"
b6_abilities_race.lst` returns nothing; only `Monkey Goblin ~` lines exist). The same
shape would have silently contaminated **Elf** (via "Half-Elf ~ .../Aquatic Elf ~ …")
and **Orc** (via "Half-Orc ~ …"). Fixed by anchoring to the actual PCGen field grammar
(§3) rather than any character-level regex. Re-verified after the fix: Goblin now
correctly shows **no** Bestiary 6 citation, and Monkey Goblin's own citation is
unaffected. Retro `correction` filed against this cycle's own first draft
(`--verified-by` the `grep -n "Goblin ~" b6_abilities_race.lst` command above).

**A second, expected correction:** the dispatch brief's own pre-verified sanity check
(Samsaran 30, Kitsune 21, Changeling 20, Wayang 17, Nagaji 16, Kasatha/Trox/Wyvaran 1,
Wyrwood 0 — reproduced exactly by this table's first draft via a looser whole-file
`grep -c "<Name>"`) is **not** what the tables below report. That looser count also
counted the race name's appearance in unrelated fields (DESC text, TYPE tokens, etc.),
not just genuine citation rows. The field-anchored count below is tighter and more
defensible against the brief's own stated ask — "how many rows each supplies" — but it
is a smaller number for every race (e.g. Samsaran: 30 -> 9). Both counts are legitimate
answers to slightly different questions; this table reports the citation-row count, not
the whole-file mention count, and says so here rather than silently disagreeing with the
dispatch's own figure.

**A third correction:** the dispatch/prior receipts describe `RACE_TRUE_BOOK` as a
"44-entry table." Counted directly from the live const in `src/bin/v06_work_inventory.rs`
(§3), it is **43** (7 CRB + 11 Bestiary 1 + 7 Bestiary 2 + 5 Bestiary 3 + 9 Bestiary 4 +
2 Inner Sea World Guide + 1 Bestiary 5 + 1 Bestiary 6). This does not change any
attribution — it is a transcription-figure correction, recorded here per the mandate's
"correcting this package's stated figures is expected."

---

### Core Rulebook's 7 races

| Race | Current attribution | Home base traits declared | Citing book | Base traits cited (n/5) | Total mentions | Book SOURCEDATE |
|---|---|---|---|---|---|---|
| Dwarf | core_rulebook | 4/5 | Core Rulebook | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2012-06 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2015-09 |
| Elf | core_rulebook | 4/5 | Core Rulebook | 4/5 (Ability Scores,Size,Speed,Vision) | 9 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 4/5 (Ability Scores,Size,Speed,Vision) | 13 | 2012-06 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Size,Speed,Vision) | 9 | 2015-09 |
| Gnome | core_rulebook | 4/5 | Core Rulebook | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2012-06 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2015-09 |
| Half-Elf | core_rulebook | 4/5 | Core Rulebook | 4/5 (Ability Scores,Size,Speed,Vision) | 10 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 4/5 (Ability Scores,Size,Speed,Vision) | 12 | 2012-06 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Size,Speed,Vision) | 10 | 2015-09 |
| Half-Orc | core_rulebook | 4/5 | Core Rulebook | 4/5 (Ability Scores,Size,Speed,Vision) | 9 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 4/5 (Ability Scores,Size,Speed,Vision) | 9 | 2012-06 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Size,Speed,Vision) | 9 | 2015-09 |
| Halfling | core_rulebook | 3/5 | Core Rulebook | 3/5 (Ability Scores,Size,Speed) | 9 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 3/5 (Ability Scores,Size,Speed) | 9 | 2012-06 |
|  |  | /5 | Inner Sea Races | 3/5 (Ability Scores,Size,Speed) | 9 | 2015-09 |
| Human | core_rulebook | 3/5 | Core Rulebook | 3/5 (Ability Scores,Size,Speed) | 6 | 2009-08 |
|  |  | /5 | Advanced Race Guide | 3/5 (Ability Scores,Size,Speed) | 6 | 2012-06 |
|  |  | /5 | Inner Sea Races | 3/5 (Ability Scores,Size,Speed) | 6 | 2015-09 |


### Bestiary 4's 9 races

| Race | Current attribution | Home base traits declared | Citing book | Base traits cited (n/5) | Total mentions | Book SOURCEDATE |
|---|---|---|---|---|---|---|
| Changeling | bestiary_4 | 5/5 | Advanced Race Guide | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 12 | 2012-06 |
|  |  | /5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 12 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 12 | 2015-09 |
| Kitsune | bestiary_4 | 5/5 | Advanced Race Guide | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2012-06 |
|  |  | /5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2015-09 |
| Nagaji | bestiary_4 | 5/5 | Advanced Race Guide | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2012-06 |
|  |  | /5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2015-09 |
| Samsaran | bestiary_4 | 5/5 | Advanced Race Guide | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2012-06 |
|  |  | /5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2015-09 |
| Wayang | bestiary_4 | 5/5 | Advanced Race Guide | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2012-06 |
|  |  | /5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 10 | 2015-09 |
| Kasatha | bestiary_4 | 4/5 | Bestiary 4 | 4/5 (Ability Scores,Type,Size,Speed) | 11 | 2013-10 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Type,Size,Speed) | 11 | 2015-09 |
| Trox | bestiary_4 | 5/5 | Bestiary 4 | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2013-10 |
|  |  | /5 | Inner Sea Races | 5/5 (Ability Scores,Type,Size,Speed,Vision) | 9 | 2015-09 |
| Wyrwood | bestiary_4 | 4/5 | Bestiary 4 | 4/5 (Ability Scores,Type,Size,Speed) | 7 | 2013-10 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Type,Size,Speed) | 7 | 2015-09 |
| Wyvaran | bestiary_4 | 4/5 | Bestiary 4 | 4/5 (Ability Scores,Type,Size,Speed) | 9 | 2013-10 |
|  |  | /5 | Inner Sea Races | 4/5 (Ability Scores,Type,Size,Speed) | 9 | 2015-09 |


### All 51 races -- summary verdict


| Race | Current attribution | Newest citer incl. ISR | Newest citer excl. ISR | Would move if strict newest-wins (excl. ISR)? |
|---|---|---|---|---|
| Aasimar | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Android | *(unattributed)* | Bestiary 5 (2015-12) | Bestiary 5 (2015-12) | *(ambiguous, no current attribution)* |
| Aquatic Elf | *(unattributed)* | Inner Sea Races (2015-09) | - | *(ambiguous, no current attribution)* |
| Catfolk | bestiary_3 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Changeling | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Dhampir | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Drow | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Duergar | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Dwarf | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Elf | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Fetchling | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Gathlain | *(unattributed)* | Ultimate Wilderness (2017-11) | Ultimate Wilderness (2017-11) | *(ambiguous, no current attribution)* |
| Ghoran | *(unattributed)* | Ultimate Wilderness (2017-11) | Ultimate Wilderness (2017-11) | *(ambiguous, no current attribution)* |
| Gillman | inner_sea_world_guide | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Gnome | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Goblin | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Grippli | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Half-Elf | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Half-Orc | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Halfling | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Hobgoblin | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Human | core_rulebook | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Ifrit | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Kasatha | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Kitsune | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Kobold | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Lashunta | *(unattributed)* | Inner Sea Races (2015-09) | Inner Sea Bestiary (2013-06) | *(ambiguous, no current attribution)* |
| Merfolk | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Monkey Goblin | *(unattributed)* | Bestiary 6 (2017-05) | Bestiary 6 (2017-05) | *(ambiguous, no current attribution)* |
| Nagaji | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Orc | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Oread | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Ratfolk | bestiary_3 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Rougarou | bestiary_6 | Bestiary 6 (2017-05) | Bestiary 6 (2017-05) | no |
| Samsaran | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Skinwalker | bestiary_5 | Bestiary 5 (2015-12) | Bestiary 5 (2015-12) | no |
| Strix | inner_sea_world_guide | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Suli | bestiary_3 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Svirfneblin | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Sylph | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Syrinx | *(unattributed)* | Inner Sea Races (2015-09) | Inner Sea Bestiary (2013-06) | *(ambiguous, no current attribution)* |
| Tengu | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Tiefling | bestiary | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Triaxian | *(unattributed)* | Inner Sea Races (2015-09) | - | *(ambiguous, no current attribution)* |
| Trox | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Undine | bestiary_2 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Vanara | bestiary_3 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Vishkanya | bestiary_3 | Inner Sea Races (2015-09) | Advanced Race Guide (2012-06) | **YES** |
| Wayang | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Wyrwood | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |
| Wyvaran | bestiary_4 | Inner Sea Races (2015-09) | Bestiary 4 (2013-10) | no |

---

## 5. Reading the "would move" column

"Would move" compares each race's CURRENT `RACE_TRUE_BOOK`/`RACE_NEWEST_PRINTING`
attribution (already landed, `decisions.md` Decision 10) against the newest book that
supplies at least 1 of the 5 base traits, **excluding Inner Sea Races** (§2). It is **not
a recommendation** — it is what a strict, mechanical "latest SOURCEDATE among genuine
base-trait citers" rule would produce, so the operator can see where the currently-landed
attribution already agrees with that rule (Bestiary 4's 9 races, Bestiary 5's Skinwalker,
Bestiary 6's Rougarou — all show `no`, i.e. already correctly attributed to their own
book because no citer is newer) and where it does not (every CRB/Bestiary-1/2/3/ISWG
race not already moved to ARG shows `no` too, because the already-landed
`RACE_NEWEST_PRINTING` table already moved them — this table independently reproduces
that same 32-race set from raw evidence, not by reading the table it means to check).

The `*(ambiguous, no current attribution)*` rows are the 8 `core_essentials`-residual
races Decision 9 already declined to force an attribution for (genuinely multi-book-
native). This table adds citation evidence for them too, but does not propose resolving
their ambiguity — that stays out of scope per Decision 9's own standard ("say so and
leave it" where no single true book is provable).

## 6. What this table does NOT do

- It does not change `RACE_TRUE_BOOK`, `RACE_NEWEST_PRINTING`, or any unit's `book`
  field. `src/bin/v06_work_inventory.rs` is lane 1's file and untouched.
- It does not rule on Inner Sea Races. That is a new open question (§2,
  `OPEN-ISSUES.md` row 129), not a settled one.
- It does not re-litigate Decision 9's 8 ambiguous races.
