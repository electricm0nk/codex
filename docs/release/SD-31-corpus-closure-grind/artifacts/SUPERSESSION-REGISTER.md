---
canonical: true
owner: sd31-supersession
cycle: SD31-D10-REGISTER-001
authority: decisions.md Decision 10 (2026-08-16 operator ruling) + its 2026-08-16 amendment
started: 2026-08-16
---

# SD-31 — Supersession Register

Tracks, in writing, every superseded sourcebook and superseded object this cycle could prove —
"prove" meaning field-level evidence that two records are the SAME object, not merely that their
keys match. Per Decision 10: *"we need to track, in writing, all the superseded objects and the
sourcebooks. i dont want duplicates falsely adding to the denominator. if a duplicate is found, the
most recent publishing takes precedence and the older one is flagged as supersceded/out of scope."*

**This is a standing rule, not a per-entry operator signature** (unlike the Structural Exclusion
Register, `decisions.md §3`) — which makes the evidence bar below, not a signature, the only thing
protecting the mandate denominator. Read that as the whole job of this document and its gate.

## 0. Reproduce this document

Everything below is re-derived, not transcribed, by one script against `docs/work-inventory.json`
and the pinned PCGen oracle:

```
export PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data   # bootstrap: scripts/fetch-pcgen-oracle.sh
python3 docs/release/SD-31-corpus-closure-grind/artifacts/supersession_register_build.py
```

Writes `SUPERSESSION-REGISTER.json` (this document's machine-readable twin — every table below is
rendered from it). Oracle pin at generation time: `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), confirmed via `./scripts/verify.sh --only preflight-oracle`.

## 1. Sequencing — this register runs after core_essentials is EXCLUDED, not after it dissolves

`core_essentials` is not a book (`decisions.md` Decision 9) and produces phantom cross-book
collisions until its content is re-attributed — the worked example named in this card's own brief,
`monster_ability:kyton_unnerving_gaze` colliding between `bestiary` and `core_essentials`, is exactly
this shape. **Every unit with `book == "core_essentials"` is excluded from pairing in this pass**,
confirmed as its own step before any candidate grouping runs.

**Re-derived, not the count decisions.md Decision 9 quoted:** the committed `docs/work-inventory.json`
this cycle read (generated `2026-08-16T11:04:45Z`, tip `2ae22bdae`, wave-6-merged) still carries
**644 units at `book == "core_essentials"`** (`monster_ability` 378, `race_trait` 258, `race` 8) —
the *same* 644 Decision 9 diagnosed, not a smaller residual, and byte-identical to `OPEN-ISSUES.md`
row 98's own independent measurement at HEAD `5d0cd1595` (`SD31-ATTRIB-002`). **Row 98 already carries
the full root-cause and remedy** — `resolve_true_book_for_core_essentials()`'s `SOURCELONG:` scan
covers only a file's first 5 lines, but `ce_abilities_race.lst` (which carries this card's own worked
example, `core_essentials:monster_ability:kyton_unnerving_gaze`) declares `SOURCELONG:` per-row-group
at 11 mid-file directive lines instead, so 516 of its 545 residual units are further resolvable by
walking to the nearest preceding directive; the exact fix (source-line-aware resolution, synced with
`corpus_literal_sweep.rs`'s `short_book_of`) is specified there, not repeated here. This cycle's
contribution is confirming, independently and at a fresh tip, that the defect **is still unfixed** —
logged as `OPEN-ISSUES.md` row 110 (below), a confirmation of row 98, not a new finding — and NOT
attempted here either way: `resolve_true_book_for_core_essentials()` is the re-attribution logic,
lane 1's file territory, outside this card's write scope (which may not edit it — see this card's own
brief). **What matters for this register**: whether 644 or the eventual ~29 remain unresolved, every
one of them is excluded from pairing here either way, so the defect does not risk a phantom collision
reaching the register — it only means the "post-dissolution pass" this card's brief asked to be named
has more work queued than a first read of Decision 9 alone would suggest.

**Deferred to that post-dissolution pass** (not paired this cycle): every `(kind, corpus_key)` group
that would include a `core_essentials`-labelled unit once it resolves to a real book. The clearest
named example is `monster_ability:kyton_unnerving_gaze` itself (`core_essentials:monster_ability:
kyton_unnerving_gaze`, `ce_abilities_race.lst:2333` — once re-attributed to `bestiary`, it becomes
a candidate `(kind=monster_ability, corpus_key="Kyton ~ Unnerving Gaze")` pair against the existing
`bestiary:monster_ability:kyton_unnerving_gaze` record). A full accounting of every such deferred pair
requires the re-attribution to land first; re-run this script once it has.

## 2. Guard 1 — a shared NAME is not a duplicate

Matching `(kind, name)` implicates units across owners that share nothing but a label. Re-derived
fresh against the 37,896 non-`core_essentials` units on the board:

| measure | value |
|---|---:|
| objects sharing `(kind, name)` across books | 2,313 |
| units involved | 8,382 (21.8 % of the strict 38,521-unit board) |
| objects sharing `(kind, corpus_key)` across books | 743 |
| units involved | 1,543 (4.0 %) |

Worked example confirmed directly in this run's own data (the card's own cited case):
`class_feature` **"Flight"** is `Witch Hex ~ Flight` (`advanced_players_guide`), `Aegis ~ Flight`
(`ultimate_psionics`), **and** `Psychic ~ Flight` (`ultimate_psionics`) — three unrelated objects a
`(kind, name)` match would have implicated as duplicates of each other. **Every pairing in this
register matches on `(kind, corpus_key)`, never `(kind, name)`.**

(This re-derived 8,382/21.8% differs slightly from Decision 10's own quoted 8,738/22.7% — expected
corpus drift across the six waves between that decision landing and this cycle; re-derive, don't
transcribe, is the standing rule this program follows, and the two figures still land in the same
ballpark and support the same conclusion: `(kind, name)` is unusable as a duplicate signal.)

## 3. Guard 2 — a later VARIANT is not a reprint

`pathfinder_unchained` and `mythic_adventures` are variant lines (`decisions.md` Decision 10's
amendment, restating `SD-28-ultimate-book-content-ingestion/decisions.md:1855-1858`: *"Unchained
variants are distinct classes, not replacements, at the data layer"*). **No record from either line
enters a pair without record-level proof that it is a reprint, not a variant; the default for both is
VARIANT.**

| measure | value |
|---|---:|
| `(kind, corpus_key)` groups touching `pathfinder_unchained` or `mythic_adventures` | 165 |
| units in those groups | 331 |
| groups admitted to the register with `reprint_proof` this pass | **0** |

All 165 groups are blanket-excluded, including the 95 `core_rulebook`↔`mythic_adventures` pairs named
in Decision 10 (e.g. `feat:weapon_focus`, `feat:improved_channel`) — confirmed still present in the
excluded set, not silently dropped from the count. **The 805-unit "redundant excess" figure Decision
10 itself flagged as an upper bound is not quoted as an outcome anywhere in this document**, per the
amendment's explicit instruction.

## 4. Evidence bar — what actually promotes a candidate to the register

`(kind, corpus_key)` match is Guard 1's fix, not proof of duplication by itself — decisions.md's own
caution ("evidence... is only that the keys match" does not qualify) is taken literally here. For
every one of the 578 non-`core_essentials`, non-variant-line candidate groups, this cycle fetched
**each side's raw `.lst` row** (via its `source_file`/`source_line`) from the pinned oracle and
compared them field-by-field:

- provenance/pricing tokens are stripped before comparing (`SOURCE*`, `COST`, `OUTPUTNAME`, `KEY`,
  `NAMEISPI`) — a later printing legitimately re-prices or re-cites its own source without being a
  different object;
- multi-value tags (`TYPE:A.B.C`) are compared as **order-insensitive sets** — a pure re-ordering of
  the same tag set is not evidence of a different object, but an ADDED or REMOVED tag is;
- **only an exact match after that normalization promotes the pair.** Anything else — including a
  0.90+ "near miss" that differs by one added classification tag — goes to §6
  (`candidates_needing_record_level_comparison`), never into the register itself.

| outcome | groups |
|---|---:|
| clean (non-`core_essentials`, non-variant-line) groups checked | 578 |
| **PROVEN same object (this register)** | **117** |
| differ materially — same key, NOT the same object (Guard-1-shaped false positive at the key level) | 433 |
| near-miss (similarity ≥ 0.90, not exact — candidates §6) | 21 |
| inconclusive — raw `.lst` row not found (candidates §6) | 7 |
| no usable `SOURCEDATE` for one side (left out entirely, per the standing rule never to guess an order) | 0 |

**433 of 578 same-key groups are genuinely different objects.** This is the same shape Guard 1 already
proved at the `(kind, name)` level, reproducing at the `(kind, corpus_key)` level: a shared identifier
is still not proof of duplication on its own, only a candidate worth checking. `Bullet (Firearm/Pitted)`
(`ultimate_combat` vs `ultimate_equipment`) is a concrete example — identical name and `COST`, but
`ultimate_equipment`'s `TYPE:` tag set adds an `Ammo` category `ultimate_combat`'s row does not carry;
real content difference, correctly excluded, not merely a coincidence of formatting.

## 5. Superseded sourcebooks

**None found.** No whole sourcebook's content is wholly (or even substantially) duplicated elsewhere.
The heaviest single loser among the 117 proven pairs:

| book | units superseded (this register) | book's own total units | % |
|---|---:|---:|---:|
| `inner_sea_world_guide` | 62 | 402 | 15.4 % |

Every other losing book is under 2% of its own total (`ultimate_combat` 35/2,056 = 1.7%, next
heaviest). 15.4% is real signal (Adventurer's Guide (2017) republishes a meaningful slice of Inner Sea
World Guide's (2011) Hellknight-order material as its own appendix) but nowhere near "the whole book,"
so no `superseded_sourcebooks` entry is warranted.

## 6. Superseded objects — the register

**117 objects, 135 redundant units** (some objects span 3+ books; each superseded side counts once).
Full field-level evidence, raw `.lst` lines, and the exact command are in
`SUPERSESSION-REGISTER.json`; this table is mechanically rendered from that JSON.

| # | kind | corpus_key | surviving (book, SOURCEDATE) | superseded (book, SOURCEDATE) |
|---:|---|---|---|---|
| 1 | class | Hellknight | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 2 | class_feature | Armored Casting ~ RMA | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 3 | class_feature | Artifice ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 4 | class_feature | Blood Mantis Form | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 5 | class_feature | Brand ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 6 | class_feature | Censor ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 7 | class_feature | Death Mantis Form | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 8 | class_feature | Discipline ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 9 | class_feature | Domain Power ~ Dragonbreath | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 10 | class_feature | Domain Power ~ Guarded Mind | bestiary_6 (2017-05) | bestiary_4 (2013-10); horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 11 | class_feature | Domain Power ~ It Came From Beyond | bestiary_6 (2017-05) | bestiary_4 (2013-10); horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 12 | class_feature | Domain Power ~ Part the Veil | bestiary_6 (2017-05) | bestiary_4 (2013-10); horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 13 | class_feature | Domain Power ~ Serpent Companion | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 14 | class_feature | Domain Power ~ The Stars Are Right | bestiary_6 (2017-05) | bestiary_4 (2013-10); horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 15 | class_feature | Domain Power ~ Venomous Saliva | ultimate_wilderness (2017-11) | bestiary_6 (2017-05) |
| 16 | class_feature | Fear ~ HKFow1 | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 17 | class_feature | Fear ~ HKFow2 | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 18 | class_feature | Fear ~ HKFow3 | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 19 | class_feature | Fearsomeness ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 20 | class_feature | Force of Will ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 21 | class_feature | Glory ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 22 | class_feature | Hellknight Armor Benefits | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 23 | class_feature | Inquisitor Domain ~ Dark Tapestry Subdomain | bestiary_6 (2017-05) | horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 24 | class_feature | Inquisitor Domain ~ Dragon Subdomain | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 25 | class_feature | Inquisitor Domain ~ Scalykind | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 26 | class_feature | Inquisitor Domain ~ Stars Subdomain | bestiary_6 (2017-05) | horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 27 | class_feature | Inquisitor Domain ~ Void | bestiary_6 (2017-05) | horror_adventures (2016-08); inner_sea_world_guide (2011-03) |
| 28 | class_feature | Knowledge ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 29 | class_feature | Law ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 30 | class_feature | Magic Warrior ~ Magic Warrior's Aspect | adventurers_guide (2017-06) | inner_sea_intrigue (2016-06) |
| 31 | class_feature | Magic Warrior ~ Nameless Anonymity | adventurers_guide (2017-06) | inner_sea_intrigue (2016-06) |
| 32 | class_feature | Magic Warrior ~ Nameless Mask | adventurers_guide (2017-06) | inner_sea_intrigue (2016-06) |
| 33 | class_feature | Magic ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 34 | class_feature | Magus Archetype ~ Magic Warrior | adventurers_guide (2017-06) | inner_sea_intrigue (2016-06) |
| 35 | class_feature | Mantis Doom | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 36 | class_feature | Mantis Form | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 37 | class_feature | Nobility ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 38 | class_feature | Onslaught ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 39 | class_feature | Pentamic Faith ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 40 | class_feature | Physical Enhancement ~ Constitution | advanced_class_guide (2014-08) | core_rulebook (2009-08) |
| 41 | class_feature | Physical Enhancement ~ Dexterity | advanced_class_guide (2014-08) | core_rulebook (2009-08) |
| 42 | class_feature | Physical Enhancement ~ Strength | advanced_class_guide (2014-08) | core_rulebook (2009-08) |
| 43 | class_feature | Protection ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 44 | class_feature | RMA Bonus Spells | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 45 | class_feature | RMA Weapon Proficiencies | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 46 | class_feature | Red Shroud | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 47 | class_feature | Resurrection Sense | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 48 | class_feature | Rune ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 49 | class_feature | Shackle ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 50 | class_feature | Strength ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 51 | class_feature | Summon Devil V ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 52 | class_feature | Summon Devil VI ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 53 | class_feature | Summon Devil VII ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 54 | class_feature | Summon Devil ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 55 | class_feature | Summon Mantis | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 56 | class_feature | Tempest Druid ~ Druid Domain | adventurers_guide (2017-06) | inner_sea_magic (2011-07) |
| 57 | class_feature | Travel ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 58 | class_feature | Vigilance ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 59 | class_feature | War ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 60 | class_feature | Wrack ~ HK | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 61 | companion | 1 [†] | ultimate_magic (2011-05) | book_of_the_damned_volume_1 (2009-10) |
| 62 | companion | Chicken ~ Drift | ultimate_wilderness (2017-11) | bestiary_5 (2015-12) |
| 63 | companion | Companion Advancement ~ Giant Vulture | monster_codex (2014-11) | bestiary_3 (2012-01) |
| 64 | companion | Grab ~ Medium | ultimate_wilderness (2017-11) | bestiary_4 (2013-10) |
| 65 | companion | Penguin ~ Toboggan | ultimate_wilderness (2017-11) | bestiary_5 (2015-12) |
| 66 | equipment | Aldori Dueling Sword | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 67 | equipment | Blunderbuss | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 68 | equipment | Buckler Gun | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 69 | equipment | Culverin | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 70 | equipment | Do-maru | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 71 | equipment | Double Hackbut | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 72 | equipment | Fire Lance | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 73 | equipment | Four-mirror Armor | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 74 | equipment | Goz Mask | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 75 | equipment | Haramaki | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 76 | equipment | Katana | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 77 | equipment | Kikko Armor | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 78 | equipment | Kusari Gusoku | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 79 | equipment | Lamellar (Horn) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 80 | equipment | Lamellar (Iron) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 81 | equipment | Lamellar (Leather) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 82 | equipment | Lamellar (Steel) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 83 | equipment | Lamellar Cuirass | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 84 | equipment | Mask of the Mantis | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 85 | equipment | Mountain Pattern Armor | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 86 | equipment | Musket (Axe) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 87 | equipment | Musket (Double-Barreled) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 88 | equipment | Musket (Warhammer) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 89 | equipment | O-yoroi | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 90 | equipment | Pistol (Coat) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 91 | equipment | Pistol (Dagger) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 92 | equipment | Pistol (Double-Barreled) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 93 | equipment | Pistol (Dragon) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 94 | equipment | Pistol (Sword Cane) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 95 | equipment | Powder Keg | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 96 | equipment | Revolver | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 97 | equipment | Rifle | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 98 | equipment | Rifle (Pepperbox) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 99 | equipment | Shotgun | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 100 | equipment | Shotgun (Double-Barreled) | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 101 | equipment | Silken Ceremonial Armor | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 102 | equipment | Tatami-do | ultimate_equipment (2012-08) | ultimate_combat (2011-01) |
| 103 | equipment | Zoic Fetish (Amphibian) | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 104 | equipment | Zoic Fetish (Bird) | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 105 | equipment | Zoic Fetish (Fish) | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 106 | equipment | Zoic Fetish (Mammal) | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 107 | equipment | Zoic Fetish (Reptile) | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |
| 108 | equipment_modifier | STONE | inner_sea_world_guide (2011-03) | ultimate_combat (2011-01) |
| 109 | feat | Tribal Hunter | ultimate_wilderness (2017-11) | adventurers_guide (2017-06) |
| 110 | monster | Kami (Shikigami) | occult_adventures (2015-07) | bestiary_3 (2012-01) |
| 111 | race_trait | Shikigami ~ Improvised Weapon Mastery | occult_adventures (2015-07) | bestiary_3 (2012-01) |
| 112 | race_trait | Shikigami ~ Spell-Like Abilities | occult_adventures (2015-07) | bestiary_3 (2012-01) |
| 113 | race_trait | Vishkanya ~ Toxic ~ Vishkanya Venom | inner_sea_races (2015-09) | advanced_race_guide (2012-06); bestiary_3 (2012-01) |
| 114 | spell | Animal Growth (Reptiles Only) | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 115 | spell | Animal Shapes (Reptiles Only) | ultimate_wilderness (2017-11) | bestiary_6 (2017-05); inner_sea_world_guide (2011-03) |
| 116 | spell | Summon Demons (Nascent Demon Lord) | inner_sea_world_guide (2011-03) | book_of_the_damned_volume_2 (2010-10) |
| 117 | spell | Summon Mantis | adventurers_guide (2017-06) | inner_sea_world_guide (2011-03) |

**[†] Row 61** — `corpus_key: "1"` is not a transcription error: both books' raw rows read literally
`1\tABILITY:FEAT|AUTOMATIC|CMB Output` — a PCGen internal chargen-scaffold row (an automatic-feat
output helper), not a player-facing companion. Kept in the register because the evidence bar is
genuinely met (byte-identical content, both books), flagged here so a reviewer isn't confused by the
bare key.

**Redundant excess: 135** (117 objects; 12 of them span 3 books, contributing 2 superseded units each
— `117 + 12 = ` doesn't map 1:1 to unit count, so this is the direct tally: `sum(len(superseded)) for
each object` = 135, matching `SUPERSESSION-REGISTER.json`'s own `denominator.count_removed`, which the
gate independently cross-checks every run).

## 7. Candidates needing record-level comparison (NOT in the register)

28 groups where either the field similarity is high but not exact (21, similarity ≥ 0.90 — mostly a
single added/reordered classification tag between the two printings) or the raw `.lst` row could not
be located this pass (7 — a companion/spell/equipment key resolving to a monster-block cross-reference
rather than its own file, needing a hand lookup). **None of these count toward the denominator
change.** Full field diffs are in `SUPERSESSION-REGISTER.json`.

| # | kind | corpus_key | books | reason | similarity |
|---:|---|---|---|---|---:|
| 1 | companion | Companion (Giant Vulture) | bestiary_3, monster_codex | near_miss_field_similarity | 0.957 |
| 2 | equipment | Hellknight Plate | adventurers_guide, inner_sea_world_guide | near_miss_field_similarity | 0.941 |
| 3 | class_feature | Envy (Abjuration) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 4 | class_feature | Gluttony (Necromancy) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 5 | class_feature | Greed (Transmutation) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 6 | class_feature | Lust (Enchantment) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 7 | class_feature | Pride (Illusion) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 8 | class_feature | Sloth (Conjuration) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 9 | class_feature | Wrath (Evocation) School | adventurers_guide, inner_sea_magic | near_miss_field_similarity | 0.933 |
| 10 | equipment_modifier | Material ~ Bone | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.933 |
| 11 | equipment_modifier | Material ~ Bronze | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.933 |
| 12 | equipment_modifier | Material ~ Gold | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.933 |
| 13 | equipment_modifier | Material ~ Obsidian | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.933 |
| 14 | equipment_modifier | Material ~ Darkleaf Cloth ~ Armor / Light | advanced_race_guide, ultimate_equipment | near_miss_field_similarity | 0.929 |
| 15 | equipment_modifier | Material ~ Darkleaf Cloth ~ Armor / Medium | advanced_race_guide, ultimate_equipment | near_miss_field_similarity | 0.929 |
| 16 | equipment | Lamellar (Stone) | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.923 |
| 17 | equipment | Wakizashi | ultimate_combat, ultimate_equipment | near_miss_field_similarity | 0.909 |
| 18 | equipment_modifier | Material ~ Darkleaf Cloth ~ Item | advanced_race_guide, ultimate_equipment | near_miss_field_similarity | 0.909 |
| 19 | class_feature | Tracker ~ HK | adventurers_guide, inner_sea_world_guide | near_miss_field_similarity | 0.9 |
| 20 | equipment | Robe of Arcane Heritage | advanced_players_guide, ultimate_equipment | near_miss_field_similarity | 0.9 |
| 21 | equipment_modifier | Material ~ Darkleaf Cloth ~ Clothing | advanced_race_guide, ultimate_equipment | near_miss_field_similarity | 0.9 |
| 22 | companion | Familiar (Fox) | advanced_players_guide, bestiary_3 | raw_line_not_found | — |
| 23 | companion | Familiar (Goat) | bestiary_3, ultimate_magic | raw_line_not_found | — |
| 24 | companion | Familiar (Parrot) | advanced_players_guide, advanced_race_guide, ultimate_wilderness | raw_line_not_found | — |
| 25 | companion | Familiar (Pig) | bestiary_3, ultimate_magic | raw_line_not_found | — |
| 26 | companion | Parrot | advanced_players_guide, ultimate_wilderness | raw_line_not_found | — |
| 27 | equipment | Poison (Violet Venom) | bestiary, ultimate_wilderness | raw_line_not_found | — |
| 28 | spell | Quickened Lightning Bolt | bestiary, bestiary_4 | raw_line_not_found | — |

**Why these stay out.** Per this card's own instruction: *"if your evidence for a pair is only that
the keys match, that pair does not go in the register."* Row 2 (`Hellknight Plate`) is illustrative —
`adventurers_guide` and `inner_sea_world_guide` agree on every field except one added `TYPE:` tag,
almost certainly the same armor under an expanded taxonomy, but "almost certainly" is not the bar this
card sets, so it stays a candidate.

## 8. Denominator change — reported as its own number

| | value |
|---|---:|
| mandate denominator before (`decisions.md §5`, re-derived) | **38,521** |
| redundant excess this register proves | **135** |
| proposed denominator after | **38,386** |

**Status: PROPOSED, NOT YET APPLIED.** This register does not itself edit `docs/work-inventory.json`
or the live dashboard denominator computation — both are outside this card's write scope
(`src/bin/v06_work_inventory.rs`'s doneness/rung path belongs to lane 3; the card's own brief says to
report, not edit, if consumption needs that file).

**The precise change needed to apply it** (reported here and at `OPEN-ISSUES.md` row 111, not
made): `v06_work_inventory.rs` (or its consumer, `scripts/observer/pf1e_dashboard_producer.py`) needs
an `EXCLUDED_UNIT_IDS` set — built by loading `SUPERSESSION-REGISTER.json`'s `objects[].superseded[].id`
values — and every corpus-wide denominator/rollup computation that currently filters only on
`unit.book not in EXCLUDED_BOOKS` needs the same unit also excluded when
`unit.id in EXCLUDED_UNIT_IDS`. This is additive and mechanical (same shape `EXCLUDED_BOOKS` already
has, just keyed on unit id instead of book), and does not touch any `done`/`held`/verdict logic — a
superseded unit simply stops being counted in either the numerator or the denominator, exactly like an
`EXCLUDED_BOOKS` book already does not count today.

**Correction, caught before this section shipped (`retro.py correction`, `SD31-D10-REGISTER-001`):**
a first draft of this section claimed none of the 135 superseded units are `done`, checked against
each unit's raw `status` field alone. That is not the doneness verdict — re-checked properly against
the real `pf1e_dashboard_producer.doneness_verdict(wiring_class, status, kind)` function:

| | value |
|---|---:|
| of the 135 superseded units, currently `done` | **36** (`equipment` 34 — the `ultimate_combat` firearm/armor reprints — plus 1 `companion`, 1 `monster`) |
| currently `not-started` / `unmeasurable` / `held` / `in-progress` | 99 |
| mandate numerator if applied | 9,488 → **9,452** |
| mandate denominator if applied | 38,521 → **38,386** |
| mandate headline if applied | 24.6307 % → **24.6236 %** (moves DOWN slightly, not up) |

**Applying this register would move both the numerator and the denominator**, not only the
denominator as a first pass here assumed. The direction is still correct and non-gaming — these 36
units keep their `done` credit on the SURVIVING side of each pair (e.g. `ultimate_equipment:equipment:
blunderbuss` stays `done`; only `ultimate_combat:equipment:blunderbuss`, the superseded duplicate,
stops being counted at all) — but the headline percentage moves down by ~0.007 points, not up, because
the superseded population's own done-rate (36/135 = 26.7%) is slightly ABOVE the board's overall
24.6% rate. Reported precisely rather than the more flattering (and wrong) "denominator only" framing
a first pass assumed.

## 9. The gate

`scripts/supersession_register_gate.py`, wired as the `supersession-gate` stage in `scripts/verify.sh`
(FULL only, immediately after `corpus-sweep`; hermetic self-test `supersession-gate-selftest` in BOTH
stage sets). For every `objects[]` entry it:

1. **Re-derives** both sides' raw `.lst` row from the pinned oracle at gate time (never trusts the
   register's own cached `raw_lines`) and REFUSES the entry if they are not still field-identical.
2. REFUSES any entry naming `pathfinder_unchained`/`mythic_adventures` on either side without a
   non-empty `reprint_proof` string.
3. REFUSES any entry naming `book == "core_essentials"` on either side (Decision 9).
4. REFUSES a backwards `SOURCEDATE` order (surviving older than something it supposedly supersedes).
5. REFUSES a `denominator.count_removed` that does not match the register's own tally.

**Proven able to fail, both required shapes, by mutation test**
(`scripts/tests/test_supersession_register_gate.py`, 12 cases, hermetic fake corpus tree, no oracle
dependency):

- a materially-different pair (same key, different `BONUS` magnitude) → refused;
- a `pathfinder_unchained` entry with no `reprint_proof` → refused; the SAME entry with a real,
  non-empty `reprint_proof` → passes (the guard is proof-gated, not an unconditional ban); an entry
  with a present-but-blank `reprint_proof` (`"   "`) → still refused (closes the trivial bypass);
  `mythic_adventures` on the *superseded* side → also refused;
- `core_essentials` on either side → refused; backwards `SOURCEDATE` order → refused; a
  `count_removed` mismatch → refused;
- a genuinely identical pair (including one with re-ordered `TYPE:` tags, proving the
  order-insensitivity is intentional, not a hole) → passes.

Also **live-mutation-tested against the wired stage itself**, not only the unit tests: a bad entry
(a proven pair's surviving side hand-edited to `pathfinder_unchained`) was seeded into the real
`SUPERSESSION-REGISTER.json`, `./scripts/verify.sh --only supersession-gate` was run and produced
`FAIL` (3 violations: the variant guard, the material-difference guard, and the count mismatch, all
firing correctly), then the real register was restored and the stage re-run to confirm `PASS`
(117 objects, all clean) — see `progress.md`'s receipt for this cycle for the full transcript.

## 10. What this cycle did NOT do (by design)

- Did not re-attribute `core_essentials` content (lane 1's card, this card's own brief says "you are
  not first").
- Did not wire the denominator change into the live pipeline (out of this card's file territory;
  precisely specified in §8 and `OPEN-ISSUES.md` row 111 instead).
- Did not promote any of the 21 near-miss or 7 inconclusive candidates to the register on similarity
  or plausibility alone — every one stayed in §7.
- Did not enter any `pathfinder_unchained`/`mythic_adventures` record into the register — 0 this pass,
  by design; the default is variant.
