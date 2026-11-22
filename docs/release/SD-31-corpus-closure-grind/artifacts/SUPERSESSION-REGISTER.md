---
canonical: true
owner: sd31-supersession
cycle: SD31-D10-REGISTER-001; direction CORRECTED by SD31-D13-REG-001 (2026-08-17)
authority: decisions.md Decision 10 (2026-08-16 operator ruling) + its 2026-08-16 amendment,
  direction CORRECTED by Decision 13 (2026-08-17 operator ruling) -- see §12
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

**Re-derived again at direction-correction time (2026-08-17, `SD31-D13-REG-001`):** `core_essentials`
re-attribution has landed partial progress since the paragraph above was written — the current
`docs/work-inventory.json` carries **128** residual `core_essentials` units, not 644 — and, as
predicted, some of the freed collisions now surface as `§7` candidates (7 new `monster_ability`
raw-line-not-found rows this build, including this section's own worked example,
`monster_ability:Kyton ~ Unnerving Gaze`). Still excluded from pairing either way; still not this
card's fix (lane 1's file territory). See `§12` for the full current-state re-derivation.

## 2. Guard 1 — a shared NAME is not a duplicate

Matching `(kind, name)` implicates units across owners that share nothing but a label. Re-derived
fresh at direction-correction time (2026-08-17, `SD31-D13-REG-001`) against the current
non-`core_essentials` units on the board:

| measure | value |
|---|---:|
| objects sharing `(kind, name)` across books | 8,610 units, 22.4 % of the strict 38,521-unit board |
| objects sharing `(kind, corpus_key)` across books | 749 |
| units involved | 1,555 (4.0 %) |

Worked example confirmed directly in this run's own data (the card's own cited case):
`class_feature` **"Flight"** is `Witch Hex ~ Flight` (`advanced_players_guide`), `Aegis ~ Flight`
(`ultimate_psionics`), **and** `Psychic ~ Flight` (`ultimate_psionics`) — three unrelated objects a
`(kind, name)` match would have implicated as duplicates of each other. **Every pairing in this
register matches on `(kind, corpus_key)`, never `(kind, name)`.**

(This re-derived 8,610/22.4% differs slightly from the 8,382/21.8% this section quoted at the
prior build, and from Decision 10's own quoted 8,738/22.7% before that — expected corpus drift as
waves land between builds (most recently, `core_essentials` re-attribution moving units to real
books); re-derive, don't transcribe, is the standing rule this program follows, and all three
figures land in the same ballpark and support the same conclusion: `(kind, name)` is unusable as a
duplicate signal. This direction-correction cycle did not change how Guard 1 is computed — only
`§6`'s survivor/superseded direction changed; this table's movement is corpus drift, re-derived
per the standing rule, not a Decision-13 effect.)

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
every one of the 584 non-`core_essentials`, non-variant-line candidate groups, this cycle fetched
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

**Re-derived at direction-correction time (2026-08-17, `SD31-D13-REG-001`)** — the evidence bar
itself is unchanged by Decision 13 (only `§6`'s survivor/superseded direction flips); these counts
move only from ordinary corpus drift between builds:

| outcome | groups |
|---|---:|
| clean (non-`core_essentials`, non-variant-line) groups checked | 584 |
| **PROVEN same object (this register)** | **116** |
| differ materially — same key, NOT the same object (Guard-1-shaped false positive at the key level) | 433 |
| near-miss (similarity ≥ 0.90, not exact — candidates §7) | 21 |
| inconclusive — raw `.lst` row not found (candidates §7) | 14 |
| no usable `SOURCEDATE` for one side (left out entirely, per the standing rule never to guess an order) | 0 |

**433 of 584 same-key groups are genuinely different objects.** This is the same shape Guard 1 already
proved at the `(kind, name)` level, reproducing at the `(kind, corpus_key)` level: a shared identifier
is still not proof of duplication on its own, only a candidate worth checking. `Bullet (Firearm/Pitted)`
(`ultimate_combat` vs `ultimate_equipment`) is a concrete example — identical name and `COST`, but
`ultimate_equipment`'s `TYPE:` tag set adds an `Ammo` category `ultimate_combat`'s row does not carry;
real content difference, correctly excluded, not merely a coincidence of formatting.

## 5. Superseded sourcebooks

**None found.** No whole sourcebook's content is wholly (or even substantially) duplicated elsewhere.
**Re-derived under the Decision 13 direction correction — the identity of the "heaviest loser" book
flips along with `§6`'s direction**, since a "loser" is now whichever book prints the LATER reprint
of an identical object, not the earlier one:

| book | units superseded (this register) | book's own total units | % |
|---|---:|---:|---:|
| `adventurers_guide` (heaviest by absolute count) | 54 | 974 | 5.54 % |
| `bestiary_6` (heaviest by % of its own total) | 13 | 72 | 18.06 % |

`inner_sea_world_guide` — the pre-Decision-13 "heaviest loser" at 62/402 (15.4 %) — now loses only
**2** units (0.5 % of its own 400-unit total): under first-print-wins, ISWG (2011) is the *survivor*
against `adventurers_guide` (2017) for nearly every pair the two books share, the exact reversal
Decision 13 exists to produce (`decisions.md §13`: *"the 7 Core Rulebook races stay with the Core
Rulebook... ARG's core-race chapters add alternate racial traits"* is the same shape — an early book
regains ownership of material a later book had been credited with). Every other losing book is under
2% of its own total except `bestiary_6` (flagged above); 18.06% is real signal (Bestiary 6 (2017-05)
and Ultimate Wilderness (2017-11) are five months apart and share several reprinted domain powers) but
nowhere near "the whole book," so no `superseded_sourcebooks` entry is warranted.

## 6. Superseded objects — the register

**116 objects, 134 redundant units** (some objects span 3+ books; each superseded side counts once).
Full field-level evidence, raw `.lst` lines, and the exact command are in
`SUPERSESSION-REGISTER.json`; this table is mechanically rendered from that JSON. **Direction
corrected under Decision 13 (2026-08-17, `SD31-D13-REG-001`): for an identical pair the FIRST
print owns it -- `surviving` is now the OLDER book, `superseded` the later reprint(s), the
opposite of this section's pre-2026-08-17 direction. All 116 pairs re-verified: same object
identities, direction swapped, zero new or dropped entries -- see section 12.**

| # | kind | corpus_key | surviving (book, SOURCEDATE) | superseded (book, SOURCEDATE) |
|---:|---|---|---|---|
| 1 | class | Hellknight | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 2 | class_feature | Armored Casting ~ RMA | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 3 | class_feature | Artifice ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 4 | class_feature | Blood Mantis Form | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 5 | class_feature | Brand ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 6 | class_feature | Censor ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 7 | class_feature | Death Mantis Form | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 8 | class_feature | Discipline ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 9 | class_feature | Domain Power ~ Dragonbreath | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 10 | class_feature | Domain Power ~ Guarded Mind | inner_sea_world_guide (2011-03) | bestiary_4 (2013-10); bestiary_6 (2017-05); horror_adventures (2016-08) |
| 11 | class_feature | Domain Power ~ It Came From Beyond | inner_sea_world_guide (2011-03) | bestiary_4 (2013-10); bestiary_6 (2017-05); horror_adventures (2016-08) |
| 12 | class_feature | Domain Power ~ Part the Veil | inner_sea_world_guide (2011-03) | bestiary_4 (2013-10); bestiary_6 (2017-05); horror_adventures (2016-08) |
| 13 | class_feature | Domain Power ~ Serpent Companion | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 14 | class_feature | Domain Power ~ The Stars Are Right | inner_sea_world_guide (2011-03) | bestiary_4 (2013-10); bestiary_6 (2017-05); horror_adventures (2016-08) |
| 15 | class_feature | Domain Power ~ Venomous Saliva | bestiary_6 (2017-05) | ultimate_wilderness (2017-11) |
| 16 | class_feature | Fear ~ HKFow1 | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 17 | class_feature | Fear ~ HKFow2 | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 18 | class_feature | Fear ~ HKFow3 | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 19 | class_feature | Fearsomeness ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 20 | class_feature | Force of Will ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 21 | class_feature | Glory ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 22 | class_feature | Hellknight Armor Benefits | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 23 | class_feature | Inquisitor Domain ~ Dark Tapestry Subdomain | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); horror_adventures (2016-08) |
| 24 | class_feature | Inquisitor Domain ~ Dragon Subdomain | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 25 | class_feature | Inquisitor Domain ~ Scalykind | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 26 | class_feature | Inquisitor Domain ~ Stars Subdomain | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); horror_adventures (2016-08) |
| 27 | class_feature | Inquisitor Domain ~ Void | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); horror_adventures (2016-08) |
| 28 | class_feature | Knowledge ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 29 | class_feature | Law ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 30 | class_feature | Magic Warrior ~ Magic Warrior's Aspect | inner_sea_intrigue (2016-06) | adventurers_guide (2017-06) |
| 31 | class_feature | Magic Warrior ~ Nameless Anonymity | inner_sea_intrigue (2016-06) | adventurers_guide (2017-06) |
| 32 | class_feature | Magic Warrior ~ Nameless Mask | inner_sea_intrigue (2016-06) | adventurers_guide (2017-06) |
| 33 | class_feature | Magic ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 34 | class_feature | Magus Archetype ~ Magic Warrior | inner_sea_intrigue (2016-06) | adventurers_guide (2017-06) |
| 35 | class_feature | Mantis Doom | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 36 | class_feature | Mantis Form | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 37 | class_feature | Nobility ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 38 | class_feature | Onslaught ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 39 | class_feature | Pentamic Faith ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 40 | class_feature | Physical Enhancement ~ Constitution | core_rulebook (2009-08) | advanced_class_guide (2014-08) |
| 41 | class_feature | Physical Enhancement ~ Dexterity | core_rulebook (2009-08) | advanced_class_guide (2014-08) |
| 42 | class_feature | Physical Enhancement ~ Strength | core_rulebook (2009-08) | advanced_class_guide (2014-08) |
| 43 | class_feature | Protection ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 44 | class_feature | RMA Bonus Spells | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 45 | class_feature | RMA Weapon Proficiencies | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 46 | class_feature | Red Shroud | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 47 | class_feature | Resurrection Sense | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 48 | class_feature | Rune ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 49 | class_feature | Shackle ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 50 | class_feature | Strength ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 51 | class_feature | Summon Devil V ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 52 | class_feature | Summon Devil VI ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 53 | class_feature | Summon Devil VII ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 54 | class_feature | Summon Devil ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 55 | class_feature | Summon Mantis | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 56 | class_feature | Tempest Druid ~ Druid Domain | inner_sea_magic (2011-07) | adventurers_guide (2017-06) |
| 57 | class_feature | Travel ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 58 | class_feature | Vigilance ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 59 | class_feature | War ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 60 | class_feature | Wrack ~ HK | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 61 | companion | Chicken ~ Drift | bestiary_5 (2015-12) | ultimate_wilderness (2017-11) |
| 62 | companion | Companion Advancement ~ Giant Vulture | bestiary_3 (2012-01) | monster_codex (2014-11) |
| 63 | companion | Grab ~ Medium | bestiary_4 (2013-10) | ultimate_wilderness (2017-11) |
| 64 | companion | Penguin ~ Toboggan | bestiary_5 (2015-12) | ultimate_wilderness (2017-11) |
| 65 | equipment | Aldori Dueling Sword | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 66 | equipment | Blunderbuss | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 67 | equipment | Buckler Gun | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 68 | equipment | Culverin | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 69 | equipment | Do-maru | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 70 | equipment | Double Hackbut | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 71 | equipment | Fire Lance | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 72 | equipment | Four-mirror Armor | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 73 | equipment | Goz Mask | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 74 | equipment | Haramaki | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 75 | equipment | Katana | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 76 | equipment | Kikko Armor | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 77 | equipment | Kusari Gusoku | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 78 | equipment | Lamellar (Horn) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 79 | equipment | Lamellar (Iron) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 80 | equipment | Lamellar (Leather) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 81 | equipment | Lamellar (Steel) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 82 | equipment | Lamellar Cuirass | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 83 | equipment | Mask of the Mantis | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 84 | equipment | Mountain Pattern Armor | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 85 | equipment | Musket (Axe) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 86 | equipment | Musket (Double-Barreled) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 87 | equipment | Musket (Warhammer) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 88 | equipment | O-yoroi | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 89 | equipment | Pistol (Coat) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 90 | equipment | Pistol (Dagger) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 91 | equipment | Pistol (Double-Barreled) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 92 | equipment | Pistol (Dragon) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 93 | equipment | Pistol (Sword Cane) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 94 | equipment | Powder Keg | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 95 | equipment | Revolver | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 96 | equipment | Rifle | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 97 | equipment | Rifle (Pepperbox) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 98 | equipment | Shotgun | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 99 | equipment | Shotgun (Double-Barreled) | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 100 | equipment | Silken Ceremonial Armor | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 101 | equipment | Tatami-do | ultimate_combat (2011-01) | ultimate_equipment (2012-08) |
| 102 | equipment | Zoic Fetish (Amphibian) | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 103 | equipment | Zoic Fetish (Bird) | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 104 | equipment | Zoic Fetish (Fish) | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 105 | equipment | Zoic Fetish (Mammal) | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 106 | equipment | Zoic Fetish (Reptile) | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |
| 107 | equipment_modifier | STONE | ultimate_combat (2011-01) | inner_sea_world_guide (2011-03) |
| 108 | feat | Tribal Hunter | adventurers_guide (2017-06) | ultimate_wilderness (2017-11) |
| 109 | monster | Kami (Shikigami) | bestiary_3 (2012-01) | occult_adventures (2015-07) |
| 110 | race_trait | Shikigami ~ Improvised Weapon Mastery | bestiary_3 (2012-01) | occult_adventures (2015-07) |
| 111 | race_trait | Shikigami ~ Spell-Like Abilities | bestiary_3 (2012-01) | occult_adventures (2015-07) |
| 112 | race_trait | Vishkanya ~ Toxic ~ Vishkanya Venom | bestiary_3 (2012-01) | advanced_race_guide (2012-06); inner_sea_races (2015-09) |
| 113 | spell | Animal Growth (Reptiles Only) | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 114 | spell | Animal Shapes (Reptiles Only) | inner_sea_world_guide (2011-03) | bestiary_6 (2017-05); ultimate_wilderness (2017-11) |
| 115 | spell | Summon Demons (Nascent Demon Lord) | book_of_the_damned_volume_2 (2010-10) | inner_sea_world_guide (2011-03) |
| 116 | spell | Summon Mantis | inner_sea_world_guide (2011-03) | adventurers_guide (2017-06) |

**Redundant excess: 134** (116 objects; 14 of them span 3+ books, contributing 2 superseded units
each -- the direct tally is `sum(len(superseded)) for each object` = 134, matching
`SUPERSESSION-REGISTER.json`'s own `denominator.count_removed`, which the gate independently
cross-checks every run).

## 7. Candidates needing record-level comparison (NOT in the register)

35 groups where either the field similarity is high but not exact (21, similarity >= 0.90 -- mostly a
single added/reordered classification tag between the two printings) or the raw `.lst` row could not
be located this pass (14 -- a companion/monster_ability/spell key resolving to a monster-block
cross-reference rather than its own file, needing a hand lookup; 7 of these are new since the
register was last regenerated -- see section 12 -- surfaced by `core_essentials` re-attribution
progressing from 644 to 128 residual units between builds, which exposed `monster_ability`
cross-book collisions `core_essentials`'s exclusion previously hid, including this card's own
worked example, `monster_ability:Kyton ~ Unnerving Gaze`). **None of these count toward the
denominator change.** Full field diffs are in `SUPERSESSION-REGISTER.json`.

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
| 28 | monster_ability | Clockwork ~ Difficult to Create | bestiary_3, inner_sea_world_guide | raw_line_not_found | — |
| 29 | monster_ability | Clockwork ~ Swift Reactions | bestiary_3, inner_sea_world_guide | raw_line_not_found | — |
| 30 | monster_ability | Clockwork ~ Winding | bestiary_3, inner_sea_world_guide | raw_line_not_found | — |
| 31 | monster_ability | Immunity to Permanent Wounds | bestiary, inner_sea_bestiary | raw_line_not_found | — |
| 32 | monster_ability | Kyton ~ Unnerving Gaze | bestiary, bestiary_3 | raw_line_not_found | — |
| 33 | monster_ability | Rakshasa ~ Detect Thoughts | bestiary, bestiary_3 | raw_line_not_found | — |
| 34 | monster_ability | Water Walk ~ Constant | bestiary_2, bestiary_4 | raw_line_not_found | — |
| 35 | spell | Quickened Lightning Bolt | bestiary, bestiary_4 | raw_line_not_found | — |

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

**This whole section is a historical snapshot from an earlier wave** (mandate ~24.6%, before six
further waves of real work). Decision 13's direction correction does not change WHICH units are
superseded (the object set is unchanged, `§6`), only which SIDE of each pair is `surviving` vs
`superseded` — but that DOES change the specific superseded unit ids `EXCLUDED_UNIT_IDS` would name,
and each id's own doneness state (a unit that used to be counted `surviving`/kept may now be the
`superseded` side dropped, or vice versa). **Re-derived against today's board in `§12`; do not apply
this section's stale numerator/denominator figures.**

## 9. The gate

`scripts/supersession_register_gate.py`, wired as the `supersession-gate` stage in `scripts/verify.sh`
(FULL only, immediately after `corpus-sweep`; hermetic self-test `supersession-gate-selftest` in BOTH
stage sets). For every `objects[]` entry it:

1. **Re-derives** both sides' raw `.lst` row from the pinned oracle at gate time (never trusts the
   register's own cached `raw_lines`) and REFUSES the entry if they are not still field-identical.
2. REFUSES any entry naming `pathfinder_unchained`/`mythic_adventures` on either side without a
   non-empty `reprint_proof` string.
3. REFUSES any entry naming `book == "core_essentials"` on either side (Decision 9).
4. REFUSES a backwards `SOURCEDATE` order — **direction corrected under Decision 13 (2026-08-17):
   surviving must be the FIRST print, so the gate now refuses surviving being NEWER than something
   it supposedly supersedes** (pre-Decision-13 this bullet read the opposite way — "surviving older
   than…" — see `§12`).
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

## 11. Wave-7 integration amendment (`SD31-W7-INTEGRATE-001`, 2026-08-16)

Adversarial review of this card found the gate in §9 was **unable to catch a fabricated entry** —
its "re-derive from the pinned oracle" branch (refusal 1) was dead code, because no shipped entry
ever carried `source_file`/`source_line` on its `surviving`/`superseded` sides (only the builder's
own separately-cached `raw_lines` did), so every entry fell through to a `None`-vs-cached-line
comparison that a `None == None` coincidence let pass silently. A second, independent defect: one
entry (`companion` corpus_key `"1"`) paired two unrelated PCGen class-continuation rows (a bare
level number, not an object identity) that happened to share the literal text
`1\tABILITY:FEAT|AUTOMATIC|CMB Output`.

**Both fixed, TDD, mutation-proven:**

1. `supersession_register_build.py` now emits `source_file`/`source_line` on every `surviving`/
   `superseded` side (the data was already in hand — `recs[b]["source_file"]`/`["source_line"]` —
   just never carried into the entry).
2. `supersession_register_gate.py`'s refusal-1 branch no longer falls back to the cached
   `raw_lines` at all: a side missing `source_file`/`source_line`, or whose citation the oracle
   cannot resolve, is now a **hard violation**. Re-ran the exact three fabrication mutations named
   by the review (planted-nonsense `raw_lines` on both sides; emptied `raw_lines`; a wholly invented
   entry with `evidence: "trust me"` and no `raw_lines` at all) against the fixed gate: **all three
   now exit 1**, where the pre-fix gate exited 0 on all three.
3. `supersession_register_gate.py`'s `FileFinder.BOOK_DIRS` synced to the builder's full 38-book
   table (previously missing `beginner_box`/`core_essentials`/`inner_sea_faiths`/
   `inner_sea_taverns`/`inner_sea_temples`, a second way an unresolvable citation could silently
   read as "equal").
4. A new builder guard refuses any group whose `corpus_key` is a bare integer before it can ever
   reach the material-difference comparison (which cannot distinguish two different continuation
   rows that happen to share a level number); the same guard was added to the gate as defense in
   depth. The `companion` `"1"` entry no longer builds.
5. 5 new self-test cases (16 total, up from 12 named in §9), all green; the 2 pre-existing
   `CleanEntryPassesTest` cases had their own latent fixture bug exposed and fixed in the same pass
   (`_entry()`'s file defaults were positional, not book-aware, and had silently been masked by the
   exact `None == None` hole being fixed here).

**Regenerated register, corrected figures:**

| | before (finding 1/2 unfixed) | after (this amendment) |
|---|---:|---:|
| objects | 117 | **116** |
| redundant units (`denominator.count_removed`) | 135 | **134** |
| proposed denominator if applied | 38,386 | **38,387** |
| of the redundant units, currently `done` | 36 (34 equipment, 1 companion, 1 monster) | **36** (33 equipment, 2 companion, 1 monster) |
| numerator if applied | 9,452 | **9,452** |
| mandate headline if applied | 24.6236% | **24.6229%** |

`python3 scripts/supersession_register_gate.py --corpus-root "$PCGEN_CORPUS_ROOT"` against the
regenerated register: `116 objects checked … OK: every entry proves same-object field equality and
clears both guards`, exit 0 — the gate now GENUINELY proves this, not merely reports it.

**Status: still PROPOSED, NOT YET APPLIED.** The gate defect that made this unsafe to wire is fixed
and the one bad entry is gone, but wiring `EXCLUDED_UNIT_IDS` into the live denominator computation
(§8's own spec) remains a separate, dedicated change this integration cycle chose not to make
inline under wave pressure — see `progress.md`'s `SD31-W7-INTEGRATE-001` receipt and the followups
list for the exact next step.


## 12. Decision 13 direction correction (`SD31-D13-REG-001`, 2026-08-17)

**What changed.** Decision 10 (2026-08-16) recorded the operator's original direction as *"the
most recent publishing takes precedence and the older one is flagged as superseded"*. Decision 13
(2026-08-17, verbatim: *"if they are identical - first print owns it"*) corrected that: **for an
IDENTICAL pair the FIRST print owns it, the later printing is superseded.** This section is the
required re-derivation `decisions.md §13` names (*"every existing entry's direction must be
re-derived... the register stays PROPOSED until it is re-derived under this rule"*).

**What this cycle touched.**

1. `supersession_register_build.py` — the survivor pick flipped from `max(books, key=date)`
   (newest) to `min(books, key=date)` (first print). Evidence bar, guards, and every other rule
   are byte-for-byte unchanged.
2. `supersession_register_gate.py` — the SOURCEDATE-ordering refusal flipped from *"surviving
   older than superseded is refused"* to *"surviving NEWER than superseded is refused"*, TDD:
   `scripts/tests/test_supersession_register_gate.py` was edited FIRST (3 tests now encode the
   corrected direction), confirmed to fail against the pre-fix gate (proving the tests exercise
   real behavior, not tautologies), then the gate was fixed and the full 16-case suite reconfirmed
   green.
3. `SUPERSESSION-REGISTER.json` was regenerated end-to-end against the current
   `docs/work-inventory.json` and the pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).
4. `SUPERSESSION-REGISTER.md` (this document) — §§1, 2, 4, 5, 6, 7, 8, 9 updated with re-derived
   numbers; this section added.

**Proof the swap is exact, not a rebuild that happened to look similar.** Every one of the 116
proven objects in the pre-correction register maps to the identical `(kind, corpus_key)` in the
post-correction register, with `surviving`/`superseded` book sets exactly swapped (surviving's
book moves to the superseded set and vice versa) — verified programmatically, 0 mismatches, 0 new
objects, 0 dropped objects:

```
# pre-D13 register backed up before regen: `git show HEAD:docs/release/SD-31-corpus-closure-grind/artifacts/SUPERSESSION-REGISTER.json > /tmp/old-register.json`
python3 -c "
import json
old = json.load(open('/tmp/old-register.json'))
new = json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SUPERSESSION-REGISTER.json'))
oldmap = {(o['kind'], o['corpus_key']): o for o in old['objects']}
newmap = {(o['kind'], o['corpus_key']): o for o in new['objects']}
assert set(oldmap) == set(newmap)
for k in oldmap:
    o, n = oldmap[k], newmap[k]
    os_, ns_ = o['surviving']['book'], n['surviving']['book']
    ob, nb = {s['book'] for s in o['superseded']}, {s['book'] for s in n['superseded']}
    assert ns_ in ob and os_ in nb
print('116/116 clean swaps, 0 mismatches')
"
```

**Did the register SHRINK, per `decisions.md §13`'s expectation?** No, and this is a deliberately
re-verified finding, not an assumption: `supersession_register_build.py`'s evidence bar (§4)
already refused any pair that was not field-identical after normalization — a Dwarf/Grey-Dwarf
shape (branch 2, "a different thing") or a darkvision-60-vs-90 shape (branch 3, "same thing,
changed values") would already fail the exact-match test and land in §7's candidates, never in
the register. **The 116 objects here were always branch-1-shaped (identical); §13's branch 2/3
split does not remove any of them — it only reversed which side of each identical pair is
`surviving`.** The register's OWN size is unaffected by this correction; what moves is 134 unit
ids' worth of `EXCLUDED_UNIT_IDS` membership (§8) and the specific unit each id names, not the
count.

**Gate re-run against the corrected, regenerated register:**

```
python3 scripts/supersession_register_gate.py --corpus-root "$PCGEN_CORPUS_ROOT"
# -> supersession_register_gate: 116 objects checked
#    OK: every entry proves same-object field equality and clears both guards
```

**Gate proven able to fail on the OLD (pre-correction) register**, confirming the direction check
is a real, live guard and not decorative:

```
python3 scripts/supersession_register_gate.py --register /tmp/old-register.json \
  --corpus-root "$PCGEN_CORPUS_ROOT"
# -> supersession_register_gate: 116 objects checked
#    FAIL: 134 violation(s) — one per superseded side of every proven pair (matching
#    objects_redundant_excess exactly, since each entry's `surviving` date is checked
#    against EVERY one of its `superseded` sides individually), e.g.:
#    class:Hellknight: surviving adventurers_guide (2017-06) is NEWER than superseded
#    inner_sea_world_guide (2011-03) -- Decision 13: for an identical pair the FIRST
#    print owns it, the later printing is superseded
```

**Re-derived current-board figures (today's `docs/work-inventory.json`, mandate denominator
`decisions.md §5`), superseding every numerator/denominator figure quoted in §8 (that section's
own numbers are six waves stale):**

| | value |
|---|---:|
| mandate denominator before | **38,521** |
| redundant excess this register proves | **134** |
| proposed denominator after (still PROPOSED, NOT APPLIED) | **38,387** |
| mandate numerator before | **11,829** |
| of the 134 superseded units, currently `done` (real `doneness_verdict()`) | **38** (not-started 89, unmeasurable 6, in-progress 1) |
| mandate numerator if applied | 11,829 → **11,791** |
| mandate headline if applied | 30.7079 % → **30.7161 %** (moves UP slightly this time — the superseded population's own done-rate, 38/134 = 28.4%, is now slightly BELOW the board's 30.71% rate, the opposite sign from §8's wave-7 snapshot, because the population itself changed under the direction swap) |

**Candidates (§7) grew from 28 to 35, clean groups checked from 578 to 584 — NOT a Decision 13
effect.** Both are ordinary corpus drift: `core_essentials` re-attribution (lane 1) progressed
from 644 to 128 residual units between the two builds, freeing several `monster_ability`
cross-book collisions `core_essentials`'s exclusion had been hiding (7 new `raw_line_not_found`
rows, §7 rows 28-34) — including this register's own `§1` worked example,
`monster_ability:Kyton ~ Unnerving Gaze`. None of the 7 are Decision 13's doing; they were always
candidates once `core_essentials` stopped hiding them, and none has been promoted to the register
without the same field-level identity proof every other entry needs.

**Status: still PROPOSED, NOT APPLIED.** This cycle changes no unit's `book` attribution (frozen
pending the operator's race ruling, §13's amendment) and edits neither `v06_work_inventory.rs` nor
the producer — §8's `EXCLUDED_UNIT_IDS` wiring spec is unchanged and still not implemented. Race
attribution stays frozen; see
`docs/release/SD-31-corpus-closure-grind/artifacts/RACE-EVIDENCE-D13.md` for the worked-example
table `decisions.md §13`'s amendment asked for, still separately awaiting the operator's
branch-1/2/3 ruling per race.
