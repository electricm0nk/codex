# Corpus-wide work channels — rescoping input for SD-29 and SD-30

**Status:** analysis for operator review, not a decision. Written on `tranche/8`, 2026-08-10,
at operator request: *"instead of scoping them by book we should instead look at prerequisite work
and work channels across the entire corpus — both for started but unfinished books as well as those
we haven't started."*

**Author:** SD-28 orchestrator. Every figure derives from `docs/work-inventory.json` at `347dcf3c`,
post-`§61`. Nothing here is inherited from a planning document.

---

## 0. Reproduce every figure

```bash
cd ~/workspace/repos/codex

# kind × status matrix
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
by = collections.defaultdict(collections.Counter)
for u in U: by[u.get('kind')][u.get('status')] += 1
for k, c in sorted(by.items(), key=lambda x: -sum(x[1].values())):
    print(k, sum(c.values()), dict(c))
PY

# breadth: how many books have ANY ingested unit of each kind
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
P = {'grounded','text-complete','ingested-magnitude'}
tot, done = collections.defaultdict(set), collections.defaultdict(set)
for u in U:
    tot[u['kind']].add(u['book'])
    if u.get('status') in P: done[u['kind']].add(u['book'])
for k in sorted(tot, key=lambda x: -len(tot[x])):
    print(f'{k:20} books={len(tot[k]):3} with-any-ingested={len(done[k]):3}')
PY
```

---

## 1. The corpus, by kind

| kind | total | proven | ing-mag | not-ingested | not-started | unknown |
|---|---|---|---|---|---|---|
| class_feature | 15,472 | 109 | 0 | 9,078 | 3,293 | 2,958 |
| equipment | 6,227 | 426 | 4,453 | 370 | 978 | 0 |
| race_trait | 3,456 | 44 | 0 | 1,790 | 1,622 | 0 |
| monster_ability | 3,107 | 0 | 0 | 1,232 | 1,875 | 0 |
| spell | 2,843 | 22 | 1,067 | 815 | 939 | 0 |
| feat | 2,610 | 1,260 | 0 | 84 | 957 | 307 |
| companion | 1,683 | 0 | 0 | 1,363 | 320 | 0 |
| equipment_modifier | 1,580 | 307 | 442 | 603 | 228 | 0 |
| monster | 1,270 | 46 | 0 | 305 | 919 | 0 |
| class | 185 | 27 | 0 | 103 | 55 | 0 |
| race | 103 | 7 | 0 | 62 | 34 | 0 |

## 2. Breadth — the number that reframes everything

**How many books have *any* ingested unit of each kind:**

| kind | books with units | books with any ingested | ingested in |
|---|---|---|---|
| feat | 23 | **11** | broad |
| equipment | 29 | **9** | broad |
| equipment_modifier | 19 | 7 | broad |
| class_feature | 23 | 5 | CRB 65, PU 22, ACG 13, APG 7, ARG 1 |
| spell | 26 | **3** | CRB 652, APG 293, ACG 144 |
| race_trait | 27 | 3 | CE 39, UPsi 4, ARG 1 — see §3 |
| monster | 14 | **1** | bestiary 46 (of its own 330) |
| monster_ability | 24 | **0** | — |
| companion | 17 | **0** | — |

## 3. Four channels, not twenty-three books

**Channel A — proven path, wide adoption.** `feat` (11/23 books), `equipment` (9/29),
`equipment_modifier` (7/19). The method is settled: per-book table, `equipment_resolver`-style
chain, count sweep. SD-28 landed seven books of feats and four of equipment through it. **Adding a
book here is content work with a known cost.**

**Channel B — proven path, narrow adoption.** `spell` (3/26 books, 1,089 of 2,843 units) and
`monster` (**1/14 books, 46 of 1,270 units**). The path exists and has been exercised — but once,
or nearly. `monster` is the one that matters: it is SD-29's entire premise, and it has been run for
46 monsters in a single book. Extending it to 13 more books is not the same risk profile as adding
a twelfth feat book.

**Channel C — no path anywhere in the corpus.** `monster_ability` (3,107 units, 24 books, **zero
ingested**) and `companion` (1,683 units, 17 books, **zero ingested**). No engine table family
exists for either kind. `monster_ability` did not exist as a kind until SD-28 `§61` created it
last night. **This is 4,790 units of mechanism-building, not ingestion.**

**Channel D — blocked on engine work.** `class_feature`, 15,472 units (42% of the corpus), 109
proven. Blocked behind the archetype mechanism (SD-28 `§60`/`§63`) and per-class chassis. `§63`
established the sizing **cannot be extrapolated** — four hand-verified classes span 5%–70% of named
slots wired-able. Not schedulable until per-class measurement is funded.

**`race_trait` sits awkwardly and needs its own ruling.** 3,456 units, 44 "ingested" — but 39 are in
`core_essentials` (excluded from the dashboard by operator directive), 4 are UPsi's
name-coincidence false positives recorded in SD-28 `§56`, and 1 is ARG. **The real count of
legitimately grounded race traits is approximately one.** There is no per-book race-trait ingest
path; the only source `classify()` reads is CRB's own hardcoded table, and a non-CRB trait can reach
`grounded` only by coincidental name match. Treat `race_trait` as Channel C until that is fixed.

## 4. What this means for SD-29 and SD-30

**SD-29 (Bestiary line, 7 books) is not a book-ingest bundle.** Its content decomposes as:

| kind | units across SD-29's 7 books | channel |
|---|---|---|
| monster_ability | 1,869 | C — no path |
| monster | 1,143 | B — path used once |
| race_trait | 1,145 | C — no real path |
| companion | 334 | C — no path |

**Three of its four dominant kinds have no working ingest path**, and the fourth has been exercised
in one book. A per-book epic breakdown hides this completely: it reads as seven similar jobs, when
it is really one path-extension job (monster) plus three mechanism-builds.

The per-book distribution also has no representative case — Monster Codex has **2** monsters and 68
class_features; Bestiary 5 and 6 have **zero** monsters; Bestiary 3 is 799 `race_trait` while
Bestiary 4 is 768 `monster_ability`.

**SD-30 (16 books) inherits the same structure at larger scale** and should be scoped from the same
channel table rather than a book list.

## 5. Proposed shape

1. **Sequence by channel, not by package.** Channel C mechanisms (`monster_ability`, `companion`,
   `race_trait` path) are prerequisites for most of SD-29 *and* SD-30. Build each once, corpus-wide,
   before either package's content work — otherwise both packages queue behind the same missing
   mechanism and rediscover it independently.
2. **Channel A work can run continuously and in parallel**, across started and unstarted books
   alike. It needs no new mechanism and no scoping decision.
3. **Channel B — extend the monster path deliberately.** One more book end-to-end before committing
   to thirteen, to get the real per-book cost the way SD-28 got the archetype class-two delta.
4. **Channel D stays out of both packages** until per-class measurement is funded (`§63`).
5. **Started-but-unfinished books are not a separate category** under this scheme — a book is simply
   a set of (kind, book) cells, some of which are done. The channel table already covers them.

## 6. Open questions

- **Provenance.** Per-book receipts carry the OGL/licensing story. Channel-scoped work needs a
  different provenance record. **Blocking before the first channel runs.**
- **Cross-book KEY collisions** get easier to detect per channel but the check must move out of the
  per-book slice. It caught real duplication in three SD-28 books.
- ~~**Does `monster_ability` warrant an engine table at all?**~~ **RESOLVED, operator ruling
  2026-08-10: `monster_ability` is engine content and requires a table and real grounding**, not
  reference text terminating at `text-complete`. See §8.
- **`race_trait`'s missing path** is a real defect (SD-28 `§56`), not merely unbuilt — the current
  classifier can ground a non-CRB trait by name coincidence alone.

---

## 7. Product book list — operator rulings, 2026-08-10

Settled in conversation. These are decisions, not analysis; the reasoning is recorded so the next
reader can tell which parts were measured and which were judgment.

### 7.1 The list

**36 books in, 2 out. 36,907 units in scope, 2,202 proven (6.0%).**

```bash
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
EX = {'core_essentials', 'beginner_box'}
inc = [u for u in U if u['book'] not in EX]
P = {'grounded', 'text-complete'}
print(len({u['book'] for u in inc}), 'books', len(inc), 'units,',
      sum(1 for u in inc if u.get('status') in P), 'proven')
PY
```

### 7.2 Excluded — simplified subsets that conflict with core

| book | units | proven |
|---|---|---|
| core_essentials | 1,610 | 39 |
| beginner_box | 19 | 7 |

**Reason (operator):** Core Essentials is a lighter subset of the core rules and can conflict with
them. Beginner Box is the same category — a simplified introductory ruleset.

**Consequence worth noting:** this makes the product match an exclusion the dashboard *already*
applies (operator directive 2026-08-02, `pf1e_dashboard_producer.py:586`). Before this ruling the
metric excluded two books the product included; now they agree. Note also that 39 of the corpus's
44 "grounded" `race_trait` units live in `core_essentials` — so excluding it removes most of that
kind's apparent progress, which was never real (see §3 and SD-28 `§56`).

### 7.3 Included, with a modelling constraint — Pathfinder Unchained

**Ruling (operator):** Unchained variants are **distinct selectable options, not replacements** —
a player chooses "Rogue" or "Rogue (Unchained)"; Unchained never silently supersedes core Rogue.

**Constraint, from how the corpus actually models it.** `pathfinder_unchained` declares **zero
`CLASS:` objects** and contributes zero `class` units to the inventory (577 `class_feature`, 127
`race_trait`, 72 `monster_ability`, 42 `equipment_modifier`, 8 `feat`). `Barbarian ~ Unchained
Class` is a `CATEGORY:CLASS` *selection ability* layered over CRB's real `CLASS:Barbarian`; the
variant record's `hit_die`, `bab` and all three save columns are `null`, so it keeps CRB's chassis
unchanged (`rules_tables/pathfinder_unchained/barbarian_features.rs`, module doc).

**Therefore: implement as CRB chassis + variant feature set, distinguished by the selection
ability — not as a duplicated chassis table.** The existing module comment rejects inventing one as
*"a second, competing statement of a fact CRB already owns"*, which is the Decision 36 pattern this
program has now closed twenty-one times. The ruling's intent (two selectable options) is satisfied
by presentation and selection, not by duplicating chassis data.

**Open:** whether the *picker* currently surfaces both as distinct choices has not been verified.
That is the part of this ruling that may need work.

### 7.4 Included — Inner Sea line and Book of the Damned

**Ruling (operator):** include. 2,822 units across 10 books (`inner_sea_*` ×8,
`book_of_the_damned_volume_1/2`).

This settles the product-identity question raised in §4: the tool covers Golarion setting content,
not system-neutral rules only. Consequence for scoping — setting books carry `class_feature`,
`spell` and `race_trait` populations that fall in the same channels as everything else, so they are
not a separate workstream.

### 7.5 Included — Ultimate Psionics

**Ruling (operator):** include. 2,495 units, the only non-Paizo book in the set (Dreamscarred
Press). Self-consistent and additive rather than conflicting. SD-28 made it one of the
better-ingested books (221 feats `§50`, 439 equipment `§58`, 15 archetype records `§51`).

### 7.6 Checked and kept — Bonus Bestiary

Investigated as a suspected promo reprint of Bestiary 1; **the hypothesis was wrong.** Its 14
monsters have **zero** key overlap with Bestiary 1 (`Allip`, `Ascomoid`, `Axe Beak`, `Caryatid
Column`, …). Not redundant. Kept.

### 7.7 Considered and kept — Mythic Adventures

969 units. Layers a parallel advancement track *on top of* core rather than replacing any core
subsystem, so it is additive and does not raise the Core Essentials conflict. Kept.

### 7.8 What changes downstream

- **Denominators.** Every program-wide percentage should now be computed over 36,907, not 38,536.
  SD-28's closing figures use the wider number and predate this ruling.
- **`race_trait` looks worse, correctly.** Removing `core_essentials` drops 39 of 44 grounded race
  traits — units that were grounded by name coincidence against CRB's table, not by real support.
- **SD-29 and SD-30 book lists** must be re-derived against the 36-book set before either is cut
  into channels.


---

## 8. `monster_ability` ruling and its consequence for SD-29 (operator, 2026-08-10)

**Ruling:** `monster_ability` is **engine content**. It requires its own table family and a real
grounding path; it does not terminate at `text-complete` as reference text.

**Scale:** 3,107 units across 24 books, **zero ingested anywhere**, zero table files. The kind did
not exist until SD-28 `§61` created it by replacing `file_kind()`'s filename typing with row-content
classification.

**Why this matters more than the unit count suggests.** Under the alternative ruling (reference
content, terminal at `text-complete`) these 3,107 units would have become achievable by ingestion
alone — Channel A work at known cost. Under this ruling they are **net-new mechanism work**: a table
family, a grounding path, and a reach classification, none of which exist. This is now the largest
single piece of unbuilt engine machinery in the corpus.

**Consequence for SD-29, stated plainly.** Its four dominant kinds are:

| kind | units | status after this ruling |
|---|---|---|
| monster_ability | 1,869 | mechanism-build — table + grounding, none exists |
| monster | 1,143 | path exists, exercised in one book (46 units) |
| race_trait | 1,145 | no real path (`§56`); ~1 legitimately grounded corpus-wide |
| companion | 334 | no path, zero ingested anywhere |

**SD-29 is a mechanism bundle, not an ingest bundle.** Three of four dominant kinds need machinery
built before any content can land, and the fourth has been run once. This is the strongest argument
against the per-book epic breakdown it currently carries (`SD-29/decisions.md §36`): seven per-book
epics present this as seven similar content jobs, when the real critical path is three mechanism
builds that every book then depends on.

**Sequencing consequence.** The Channel C mechanisms must be built **once, corpus-wide, before**
SD-29's and SD-30's content work — not inside either package. Building `monster_ability` twice, or
discovering mid-SD-29 that SD-30 needs the same thing, is the failure this rescoping exists to
prevent.

**Still open after this ruling:**

- Whether `monster_ability` grounding is per-facet (`NaturalAttack` vs `SpecialQuality` vs
  `SpecialAttack` vs `Universal Monster Rule`) or uniform. The four facets were identified in
  SD-28 `§61` but never costed separately.
- Whether the monster path (Channel B, 46 units in one book) and `monster_ability` share machinery
  or are independent builds. They are adjacent in the corpus but that is not evidence they are
  adjacent in the engine.
- The `companion` and `race_trait` paths remain unruled — this decision covers `monster_ability`
  only, and it should not be read as settling the other two Channel C kinds by analogy.

---

## 9. Operator rulings, 2026-08-10 (second set)

### 9.1 Archetype epic sizing — FUNDED

Per-class hand-verification of the remaining ~24 classes is authorized. `§63` established the ratio
cannot be extrapolated (5%–70% across four hand-verified classes); the operator has chosen to buy
the real number rather than leave archetype-swap unschedulable. Deliverable: per-class
`wired-able / named`, a total slot count, and `total × ~33 lines` as the epic's production-wiring
size. **No proxies** — three id-scan iterations already failed on three different naming
assumptions.

### 9.2 `monster_ability` is player-facing character content — and it merges two channels

**Ruling:** *"there are situations where a character might actually be a monster, we need to treat
them like a race/class."*

This is the most structurally significant decision in this document, and it is **not** the ruling
§8 anticipated. §8 recorded "engine content, needs a table and grounding" — correct, but framed as a
standalone kind. The operator's reasoning goes further: monsters are **playable**, so
`monster` and `monster_ability` are not two kinds to ingest separately. They are **one system**:

```
monster          1,270 units  →  behaves like a race / class chassis
monster_ability  3,107 units  →  behaves like race_traits / class_features on that chassis
                 ─────
                 4,377 units in a single coherent build
```

**Consequences:**

- **Channels B and C merge for these two kinds.** The monster path (46 units, one book) was scoped
  as "ingest stat blocks for DM reference." Under this ruling it is the chassis half of a
  playable-race system, which is a different and larger design.
- **The existing precedent is race, not monster.** `race` (103 units) + `race_trait` (3,456) already
  have this exact shape — a chassis kind plus a features kind attached to it. Whatever mechanism
  serves monsters-as-races should be evaluated against that pairing rather than invented fresh.
- **The four `monster_ability` facets matter more now, not less.** `NaturalAttack`,
  `SpecialQuality`, `SpecialAttack` and `Universal Monster Rule` are player-facing mechanics under
  this ruling, so each needs real grounding rather than description. Whether that is one mechanism or
  four remains uncosted.
- **`Kind::Monster` = 1,270 stat blocks stays the right model.** SD-28 `§61` deliberately kept
  `MonsterAbility` a separate kind rather than broadening `Monster`. This ruling vindicates that:
  chassis and features are genuinely different things, exactly as `race`/`race_trait` are.

### 9.3 `companion` and `race_trait` — same engine-content ruling, defect fixed alongside

**Ruling:** both are engine content, same as `monster_ability`. The `race_trait` name-coincidence
defect is fixed **alongside** the path, not before it and not after.

`companion`: 1,683 units, 17 books, zero ingested, no table family.

`race_trait`: 3,456 units, and the current state is worse than unbuilt — it is **defective**.
`classify()`'s only source is CRB's own hardcoded table, so a non-CRB trait reaches `grounded` by
coincidental name match alone (SD-28 `§56`; UPsi's `Blue ~ Keen Senses` matching Elf's, and three
others). Of the 44 "grounded" race traits corpus-wide, 39 are in the now-excluded `core_essentials`,
4 are those false positives, and 1 is ARG — **approximately one legitimate grounded race trait in
the whole 36-book product.**

> **FIXED 2026-08-11** (SD-29 card `epic-6-race-trait-lane-pilot`, actor `sd29-e6-racetrait-pilot`).
> `v06_work_inventory`'s `Kind::RaceTrait` arm now grounds a record against **its own race**, parsed
> from the corpus key's `~`-qualifiers (`modelled_race_of_race_trait`), instead of pairing the trait
> slug with every race the engine models. The same gate was applied to the twin in
> `EngineFacts::holds_key`.
>
> **The paragraph above undercounts the defect, corrected here rather than silently.** The false
> positives were **23, not 4**. Grounded race traits corpus-wide went **44 → 21** (re-derived by
> regenerating `docs/work-inventory.json` and counting `kind == "race_trait" && status ==
> "grounded"`). The 19 the count above missed were *intra-`core_essentials`* cross-race
> coincidences, not cross-book ones — `Aquatic Elf ~ Elven Magic` scoring off `elf.elven_magic`,
> `Svirfneblin ~ Stonecunning` off `dwarf.stonecunning`, `Drow ~ Keen Senses` off `elf.keen_senses`,
> and so on. §9.3 looked only for the cross-book form.
>
> The surviving 21 are 20 genuine `core_essentials` CRB traits plus ARG's `Saltbeard ~ Dwarf ~
> Greed` (its base race sits in an inner `~`-qualifier) — which does confirm the paragraph's headline
> claim: **exactly one** legitimate grounded race trait outside `core_essentials`.
>
> One further true finding the fix surfaced: `Dwarf ~ Hatred` was previously grounded off
> `gnome.hatred`. CRB's `race_traits()` table carries `Hatred` for Gnome only, so the Dwarf record
> now correctly reports `not-ingested` — a real gap in the hardcoded table, not a regression.

"Alongside" is the right call: building a per-book race-trait path while leaving the name-match
grounding in place would produce a path whose own success criterion is untrustworthy.

### 9.4 SD-29 / SD-30 re-cut — deferred [SUPERSEDED TWICE — see SD-29 `decisions.md §37` and `§38`]

**Superseded, 2026-08-10, twice.** First by `SD-29/decisions.md §37` (the kind-lane partitioning
re-cut, executed the same day as this deferral was written) and second by `SD-29/decisions.md §38`
(the corpus-wide re-scope, retiring SD-29's seven-book boundary entirely — "both those we have
touched and those we have not touched," operator directive 2026-08-10). SD-30 is untouched by
either supersession; `§38.5` records the resulting SD-29/SD-30 book-list collision as an open item
for the operator, not resolved by either supersession.

**Original deferral, preserved below as historical record:**

Explicitly deferred until 9.1–9.3 are settled, as its own conversation. Neither package is to be
re-cut yet. `SD-29/decisions.md §36` and `epic-breakdown.md`'s superseded-in-part note stand as
recorded, but no epics are to be re-scoped against them until the Channel C mechanism shape is known
— which 9.2 has just changed substantially.

**Why the deferral is correct rather than merely cautious:** 9.2 turned two separate channel entries
into one merged system, and 9.3 added a defect fix to a path that did not previously have one.
Re-cutting epics before those builds are shaped would produce a breakdown that is wrong in the same
way the per-book one was.

---

## 10. Book-list ruling REVERSED — Core Essentials returns (operator, 2026-08-10)

**§7.2 is superseded. Core Essentials is back in scope.** Final list: **37 books in, 1 out
(`beginner_box`, 19 units). 38,517 units, 2,253 proven.**

### 10.1 Why the original exclusion was wrong

§7.2 excluded `core_essentials` (1,610 units) as "a lighter subset of the core rules that can
conflict." Two pieces of evidence, both found after that ruling, overturn it.

**First — CRB does not define the core races. It patches them.**

```
core_rulebook/cr_races.lst  —  8 rows total
  SOURCELONG:Core Rulebook
  Dwarf.MOD  Elf.MOD  Gnome.MOD  Half-Elf.MOD  Half-Orc.MOD  Halfling.MOD  Human.MOD
```

Every core race in CRB is a **`.MOD` row** — a modification to a base record defined elsewhere. The
definitions live in `core_essentials/races/`. That is why the classifier reports `core_rulebook`
with **zero** `race` units and `core_essentials` with **51**: it is correctly seeing that CRB
modifies races it does not define. Excluding CE would have left seven `.MOD` rows patching records
that no longer exist, and a product with no definition of Dwarf, Elf or Human.

This is the fourth `.MOD`-semantics finding in this program (SD-28 `§46` unconditional recovery,
`§48` conditional variant, `§58` `.COPY=` aliasing, `§47`'s cross-book injection) — now at the level
of race definitions rather than feat text.

**Second — 99% of Core Essentials is unique, not duplicated.**

```bash
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
others = collections.defaultdict(set)
for u in U:
    if u['book'] != 'core_essentials': others[(u['kind'], u.get('corpus_key'))].add(u['book'])
uniq, dup = collections.Counter(), collections.Counter()
for u in (x for x in U if x['book'] == 'core_essentials'):
    (dup if (u['kind'], u.get('corpus_key')) in others else uniq)[u['kind']] += 1
print('CE-only', sum(uniq.values()), dict(uniq)); print('duplicated', sum(dup.values()))
PY
```

| kind | CE-only | also elsewhere |
|---|---|---|
| race_trait | 884 | 0 |
| monster_ability | 373 | 7 |
| companion | 140 | 5 |
| spell | 108 | 1 |
| race | 50 | 1 |
| class | 23 | 0 |
| feat | 15 | 0 |
| equipment | 2 | 1 |
| **total** | **1,595** | **15** |

**1,595 of 1,610 units exist nowhere else.** The book is not a redundant lighter subset; it is the
largest single block of unique content outside CRB/APG/ACG, and it holds the entire racial-trait
pool (884 units) for the races it defines.

### 10.2 Corrected denominators

Every figure computed under §7 used 36,907. **The correct in-scope denominator is 38,517.**

| scope | books | units | proven |
|---|---|---|---|
| product (final) | 37 | 38,517 | 2,253 |
| excluded | 1 (`beginner_box`) | 19 | 0 |
| whole corpus | 38 | 38,536 | 2,253 |

`core_essentials` returns 1,610 units and 46 proven to scope.

### 10.3 Consequence for the dashboard — a live defect, not just a doc correction

`pf1e_dashboard_producer.py:586` still carries `excluded = {"core_essentials", "beginner_box"}`
(operator directive 2026-08-02). Under this ruling that exclusion is **wrong and actively hiding
1,595 unique units** from every metric the dashboard reports — including the `work_inventory`
panel's `total_units`/`proven_units` and every per-kind percentage.

**Not fixed here.** It is a live defect in territory B requiring an operator decision to change a
dated directive, and it is recorded rather than acted on.

### 10.4 What this does NOT change

The `race_trait` correction from §7.8 still stands and gets *worse*, not better: 39 of the corpus's
44 grounded race traits are in `core_essentials`, and they are grounded by **name coincidence**
against CRB's hardcoded table (SD-28 `§56`), not by real support. Returning the book to scope
returns those 39 as *apparent* progress that is not real. The `race_trait` path defect must be fixed
alongside the path (§9.3) precisely so this does not read as 39 units of working content.
