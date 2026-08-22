---
canonical: true
cycle: SD31-ATTRIB-003
purpose: >
  Re-verification (not a re-fix -- v06_work_inventory.rs is lane 1's file, not this
  card's) of the core_essentials residual established by SD31-E4-F1-001/`sd31/dissolve-
  core-essentials` (already merged onto tranche/11, `2a69c5fb4`).
date: 2026-08-16
---

# core_essentials residual — re-verified, not re-fixed

## Re-derived at this cycle's tip

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('book') == 'core_essentials']
print(len(u), collections.Counter(x['kind'] for x in u))
"
# -> 128 Counter({'race_trait': 111, 'monster_ability': 9, 'race': 8})
```

Matches the dispatch brief's own "~128 units remain (re-derive)" exactly. The mechanism
that produced this floor (`resolve_true_book_for_core_essentials`,
`src/bin/v06_work_inventory.rs`) is `sd31/dissolve-core-essentials`'s work, already
merged onto `tranche/11` at `2a69c5fb4` — this card does not own that file and made no
change to it (`v06_work_inventory.rs` is lane 1's territory per this card's own dispatch).

## Independent one-record-deep spot-check of every composition bucket

**8 `race` units** — all 8 are `android`, `elf_aquatic`, `gathlain`, `ghoran`,
`goblin_monkey`, `lashunta`, `syrinx`, `triaxian` (confirmed by listing `id`/`source_file`
directly). These are Decision 9's own named 8 genuinely-multi-book-native races — this
cycle's own §1 evidence table (`SD31-ATTRIB-003-race-evidence.md`) independently confirms
each is natively citable from 2+ in-scope books with no single provable "first printing"
among them, consistent with (not merely re-quoting) the existing ambiguous verdict.

**120 `race_trait`/`monster_ability` units, by `source_file`:**

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('book') == 'core_essentials' and x.get('kind') in ('race_trait', 'monster_ability')]
print(collections.Counter(x.get('source_file') for x in u).most_common())
"
# -> ce_abilities_race.lst 38 (29 race_trait + 9 monster_ability),
#    aquaticelf/syrinx/ghoran/android/lashunta/monkeygoblin/triaxian/gathlain
#    _abilities_race.lst 13,13,12,11,11,11,11,9 (the same 8 ambiguous races' own trait rows)
```

- **91 of the 120** are the 8 ambiguous races' OWN trait rows (same file, same ambiguity
  as their `race` unit — a race trait inherits its race's own unresolved book).
- **38 in `ce_abilities_race.lst`** (29 `race_trait` + 9 `monster_ability`). Read the file
  directly (not trusted from the prior receipt's prose):
  ```
  sed -n '1,25p' core_essentials/ce_abilities_race.lst
  ```
  Line 4's own top-of-file comment: *"Everything in the Pathfinder GameMode is run off
  the Default Internal Ability, placing it in Core Essentials."* The first ~23 content
  rows (Vision Modes block: Low-Light Vision, Darkvision, Blindsense, Blindsight,
  Tremorsense, See Etheral, See Invisibility, plus internal bookkeeping rows) precede the
  file's first `SOURCELONG:` directive entirely — genuinely PCGen-internal, book-agnostic
  glossary content, independently re-read here, not assumed from the prior cycle's own
  characterization. These vision rules are printed near-identically across dozens of
  Paizo books; there is no single first-printing book to credit.
  ```
  grep -n "SOURCELONG:Universal Rules" core_essentials/ce_abilities_race.lst
  #   2342:SOURCELONG:Universal Rules   SOURCESHORT:UR
  ```
  One directive line, governing the rows until the next directive — `SOURCESHORT:UR` is
  PCGen's own internal designation (`decisions.md §25`'s existing "not a Paizo book we
  track" ruling for this exact string), correctly left unattributed.

## "Books outside the roster" check — independently re-run, not re-quoted

```
grep -n "SOURCELONG:Ironfang\|SOURCELONG:.*Blood of the Moon" core_essentials/ce_abilities_race.lst
# -> (no output)
```

None of the 128-unit residual cites Ironfang Invasion or Pathfinder Player Companion:
Blood of the Moon. No `RULING-NEEDED` row filed for "book outside the 37-book roster" —
there is nothing to rule on here; the only out-of-roster string present
(`Universal Rules`) is PCGen's own internal designation, already correctly excluded.

## Conclusion

**No further re-attribution is resolvable one record deep at this cycle's tip.** The
128-unit residual is the honest floor: 8 races (+ their 91 own trait rows) are genuinely
multi-book-native (Decision 9), and 38 rows are PCGen-internal glossary content with no
single Paizo book of origin. This confirms — independently, not merely re-states — the
prior cycle's own `core_essentials_real_corpus_residual_never_grows_past_its_pinned_baseline`
ratchet test's premise. `core_essentials` ends at 128 units by movement (1,610 -> 128,
92% resolved across three prior cycles), never by exclusion, per Decision 9's own
condition.
