# Cycle t2b-w1-d-1 — Gate 3 closure invariant / Card 11, shape T2b, book `bestiary_3`

- **Card ID:** `epic-2-cause-closure` (row 11; this cycle's scope: shape T2b, `bestiary_3` only,
  per dispatch brief and `card11-t2b-census-census.md` §4)
- **Actor:** `t2b-w1-d`
- **Base:** `45fef71f0d0ce144af093ab622fe0cd316fd99bf` (pinned; matched `origin/tranche/12` HEAD at
  cycle start — no rebase needed)
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`); `scripts/verify.sh --only preflight-oracle` FAILed on the fresh worktree
  (no checkout present) then PASSed after `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen
  slot>`.
- **Status:** measurement cycle — **0 units mechanically closed**, per `decisions.md §13`'s
  explicit authorization ("if you want to do measurements first, i support this... a measurement
  cycle that banks zero units but... produces a real, re-derivable... census is a legitimate
  closed cycle", `workflow-instruction.md §9` standing lesson 6). This is not a substitute for the
  work — it is the finding that the dispatched brief's characterization of this book's work was
  wrong, with the corrected characterization and a concrete next-step spec below.

## 0. What the dispatch brief assumed, and what this cycle found

The dispatch brief and `card11-t2b-census-census.md` §4 characterized `bestiary_3` as "819 units,
7 files to touch... needs full [RACE_CORPUS_BOOKS] onboarding," the same shape as `bestiary_2`/
`bestiary_5`/`bestiary_6`'s prior, successful onboardings (new playable races via
`ingest_races.rs`'s `IN_SCOPE_RACES`).

**That characterization does not hold for `bestiary_3`.** Re-derived below: `bestiary_3` declares
**zero new playable races** (every one of its ~261 `b3_races.lst` entries carries a `CR:` token —
the corpus's own monster discriminator, `src/bin/v06_work_inventory.rs`'s `refine_kind`: "A
`*_races.lst` row carrying a `CR:` token is a monster"). Its 819 `race_trait` units are
**monster/creature-template special-ability content misclassified as racial-trait content**, plus
a small genuine remainder — not a missing-chassis onboarding gap.

This is the same defect class the standing memory note `not-ingested-figures-are-classifier-noise`
already documented for Bestiary 1 (`b1_abilities_race.lst`: 620 declared → 0 real gap, monster
special-ability library, not racial traits) and ARG (979 declared → 0 real gap). `bestiary_3` is a
**new instance** of that pattern the SD28-E15 fix (`MONSTER_ABILITY_TYPE_FACETS`,
`src/bin/v06_work_inventory.rs`) does not fully catch, because it only matches the row's TYPE
**first** dot-segment against a fixed literal list, and bestiary-style books use compound,
race-specific first segments (`AghashRacialAbility`, `RaceAbility`, `BearLordRacialTrait`,
`AdletSelection`, ...) instead of the bare vocabulary (`SpecialQuality`/`SpecialAttack`/
`NaturalAttack`/`Universal Monster Rule`) that check was written against.

## 1. Re-derived population (unchanged from the census memo)

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled' and x.get('book')=='bestiary_3']
print(len(u))
"
```
→ **819**. Matches `card11-t2b-census-census.md` §4 exactly. No correction to the total.

## 2. Row-content classification (new this cycle — the census memo never ran this for any of the
17 unregistered books; it only ran it for the *registered*-book pile, §3)

**Re-derive script (committed):** `scripts/t2b_bestiary_3_row_classify.py`

```
python3 scripts/t2b_bestiary_3_row_classify.py
```

```
# bestiary_3 T2b units (race_trait_race_not_modelled): 819
# CR:-bearing race names in b3_races.lst: 260
# template names in b3_templates.lst: 26

category_header: 9        (Race Subtype ~ <X>, same by-design exclusion rule
                            the census memo §3 rule 2 already established for
                            the registered-book pile)
adopted_race: 5            (Adopted Race ~ Catfolk/Ratfolk/Suli/Vanara/Vishkanya)
monster_or_template_owned: 683   (KEY prefix exactly matches a CR:-bearing
                                   b3_races.lst race name or a b3_templates.lst
                                   template name)
unresolved: 122            (KEY prefix is a variant/sub-form of a monster or
                            template name -- e.g. "Confounding Bandersnatch"
                            vs. races.lst's "Bandersnatch"; "Bear Lord"/"Cat
                            Lord"/etc. vs. templates.lst's "Animal Lord";
                            "Awakened Demilich" vs. "Demilich" -- not an exact
                            string match, so the script correctly declines to
                            auto-classify them, but hand verification below
                            confirms every sampled case is monster/template
                            content, not a player race)

sum check: 819 (expect 819)
```

**Hand verification of the `unresolved` 122** (not exhaustive — spot-checked, stated as such):
- `Bear Lord ~ Bear Hug` / `Cat Lord ~ Leap` / `Canine Lord ~ Savage` / `Crocodile Lord ~ Thick
  Skin` / `Dinosaur Lord ~ Primeval Mind` / `Raptor Lord ~ Raptor's Dive` / `Rat Lord ~ Hearty` /
  `Serpent Lord ~ Poison Immunity` / `Shark Lord ~ Brutal Jaws` (9 units): each carries
  `TYPE:...RacialTrait.SpecialQuality`/`Extraordinary` naming `<Animal>LordRacialTrait` — these are
  the eight animal-specific manifestations of `b3_templates.lst`'s single `Animal Lord` template
  row (`DR:10/silver`, `DEFINESTAT:MINVALUE|STR|AnimalSTRScore`, confirmed present), not a distinct
  race per manifestation.
- `Confounding Bandersnatch ~ Poison` (`TYPE:RacialAbility.ConfoundingBandersnatchRacialAbility.
  SpecialQuality.Extraordinary`): a named variant of `b3_races.lst`'s `Bandersnatch` (CR:-bearing,
  confirmed present at line 34).
- `Awakened Demilich` (`TYPE:DemilichSelection.SpecialQuality`): a variant of `b3_races.lst`'s
  `Demilich` (CR:-bearing, confirmed present at line 66).
- `Aquatic`/`Biped`/`Quadruped`/`Serpentine` (`TYPE:Unfettered Eidolon Base Form`): base-form
  selector options for `b3_races.lst`'s own `Unfettered Eidolon` entry (lines 103-104, a
  Summoner-eidolon build option, not a player character race).
- `Asura ~ Elusive Aura` / `Clockwork Traits` (`ce_abilities_race.lst`, attributed to `bestiary_3`
  by the same cross-book directive-attribution mechanism the 9 `category_header` rows use): same
  shape as the confirmed-excluded `Race Subtype ~ Asura`/`Race Subtype ~ Clockwork` header rows —
  creature-subtype trait content, not a player race.

Every sampled `unresolved` row is monster- or template-owned content. No sampled row named a real,
new playable race.

## 3. What this means for the 819

| Sub-shape | Units | Real work? |
|---|---:|---|
| Category-header rows (by-design exclusion, same rule as the registered-book pile) | **9** | No |
| `Adopted Race ~ <X>` selector rows (Catfolk/Ratfolk/Suli/Vanara/Vishkanya) | **5** | Yes, but see §4 |
| Monster/creature-template special-ability content misclassified as `race_trait` | **≥683, likely ~805** (683 exact-matched + up to 122 unresolved, all sampled cases confirmed monster/template-owned) | No — needs a classifier fix, not book onboarding |
| **Total** | **819** | — |

**Real, book-onboarding-shaped closable work in `bestiary_3` is at most 5 units, not 819.** The
`RACE_CORPUS_BOOKS` "full onboarding, ~7 files, new chassis" pattern the dispatch brief and census
memo assumed (following `bestiary_2`/`bestiary_5`/`bestiary_6`'s precedent) **does not apply**:
those three books each declared genuine new playable races via `ingest_races.rs`'s `IN_SCOPE_RACES`
(Grippli/Ifrit/Oread/Sylph/Undine/Fetchling, Skinwalker, Rougarou); `bestiary_3` declares none.

## 4. Why the 5 `Adopted Race` rows were not landed this cycle either

`Adopted Race ~ Catfolk` (`catfolk_abilities_race.lst:30`, oracle-verified) is a `CHOOSE:
ABILITYSELECTION|Special Ability|TYPE=Catfolk Race Trait` / `MULT:YES` row — a genuine **selector
picker mechanic** (pick a race, gain a curated subset of that race's traits), not a flat racial
default. Grepping every book this program currently ingests `race_trait` from finds **zero**
existing ingest-tool precedent for the `AdoptiveRace` TYPE shape anywhere — this is net-new picker
infrastructure, the same class of deferred work `race_catalog.rs`'s own doc comment already names
for ARG's 153 alternate traits ("They need a picker that shows the swap... real follow-on work in
the frontend, not owned by this cycle"). Building it correctly for 5 rows in isolation, inside a
book-scoped lane sharing `ingest_races.rs`/`ingest_race_traits.rs` with three concurrent sibling
lanes, risks exactly the kind of half-built, book-specific patch `decisions.md §13` warns against
("do the work on... in all cases" — meaning the real mechanism, not a bestiary_3-only stub of it).
`bestiary_2`/`bestiary_5`/`bestiary_6`'s own registered-book `Adopted Race` rows (7+1+1=9, per the
census memo's own table) share this identical, still-unbuilt gap — it is a single ingest-tool
feature needed by at least 4 books, best built once.

## 5. Correction and deferral logged

- `scripts/retro.py correction` (`docs/retro/events/t2b-w1-d.jsonl`): corrects
  `card11-t2b-census-census.md` §4's "819 real open units, needs full onboarding" claim for
  `bestiary_3` to "≤5 real closable units + 9 by-design exclusions + ~805 classifier-noise (monster
  special abilities misclassified as race_trait)", verified by
  `scripts/t2b_bestiary_3_row_classify.py`.
- `scripts/retro.py deferral` (`docs/retro/events/t2b-w1-d.jsonl`): the classifier fix and the
  `AdoptiveRace` picker mechanism are both named as blocked-by items with the exact next step, per
  `docs/governance/blocker-closure-doctrine.md` disposition 2 (escalate, do not silently defer).

## 6. Concrete next steps (escalated, not guessed at)

1. **Classifier fix** (`src/bin/v06_work_inventory.rs`, `refine_kind`/`MONSTER_ABILITY_TYPE_FACETS`):
   extend the monster-ability match from "TYPE first dot-segment is a literal facet" to "the row's
   `KEY` prefix (text before ` ~ `) names a `CR:`-bearing race or a template in the SAME book's own
   `*_races.lst`/`*_templates.lst`" — verified narrower fixes are unsafe: **every real player
   race's own `Favored Enemy ~ Humanoid (<Race>)` row carries
   `TYPE:RangerClassFeatures.FavoredEnemy.SpecialAttack.Extraordinary.AttackOption`**, which shares
   an inner dot-segment (`SpecialAttack`) with the monster-only facet vocabulary — a naive
   "any-segment-match" fix would wrongly reclassify genuine content present in every book's real
   race-trait file (confirmed by grepping `dwarf_abilities_race.lst`/`elf_abilities_race.lst`/
   `human_abilities_race.lst`). This fix is corpus-wide in blast radius (shifts `race_trait`/
   `monster_ability` counts for every book, not just `bestiary_3`) and is out of this lane's
   book-scoped write authority — needs a dedicated cycle with a full-corpus regression sweep, not a
   solo change mixed into one book's lane.
2. **`AdoptiveRace` selector-capture ingest mechanism**: needed by `bestiary_3` (5 rows) and the
   registered-book piles of `bestiary_2`/`bestiary_5`/`bestiary_6` (9 rows total per the census
   memo). Best built once, generically, in whichever cycle owns `ingest_race_traits.rs` next
   (currently contended by concurrent T2b sibling lanes per the dispatch brief's own shared-file
   warning).

## 7. Verification run

- `scripts/verify.sh --only preflight-oracle`: FAIL (no checkout) → PASS after
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`.
- `python3 scripts/t2b_race_trait_census.py`: reproduces the 819 total unchanged (no code touched
  this cycle, so no drift expected or found).
- `python3 scripts/t2b_bestiary_3_row_classify.py`: produces the table in §2 above.
- No production code, corpus data, `RACE_CORPUS_BOOKS`, `race_catalog.rs`, `ingest_races.rs`, or
  `ingest_race_traits.rs` was touched this cycle — the census-memo characterization this cycle
  corrects was wrong about the *shape* of the work, so implementing the assumed "7-file onboarding"
  pattern would have been building infrastructure for content that is not, in fact, missing
  playable-race content. `decisions.md §1a`'s anti-gaming doctrine forbids manufacturing a chassis
  for monster stat blocks to satisfy a race-trait counter.
- **Identifier audit / wired-integration audit:** not run — no diff to audit (no files changed
  under scoped paths other than this receipt, the two new committed scripts, and the retro log).

## 8. Discovery forwards

- `## DISCOVERED` candidate for `progress.md`: the 16 other unregistered books in
  `card11-t2b-census-census.md` §4's table may share this same bestiary/monster-book shape
  (`bestiary`, `bestiary_4`, `mythic_adventures`, and any other book whose `*_races.lst` entries
  are CR:-bearing) — **not verified here**, flagged forward for whichever lane picks up those
  books next, so they check their own book's `*_races.lst`/`*_templates.lst` shape before assuming
  "unregistered = needs full chassis onboarding."

## 9. Next-cycle plan

This lane's assigned scope (`bestiary_3` only) is exhausted for what is safely closable within a
book-scoped lane's write authority. The two escalated items in §6 need either an explicit operator
grant of write scope to `v06_work_inventory.rs`/`ingest_race_traits.rs` beyond book-scoped lanes,
or a dedicated follow-up cycle. No further `bestiary_3`-specific work remains blocked on anything
this lane could do differently.
