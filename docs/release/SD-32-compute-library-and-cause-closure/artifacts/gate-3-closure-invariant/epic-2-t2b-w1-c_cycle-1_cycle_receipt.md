# Cycle 1 — epic-2-cause-closure / T2b lane w1-c — `core_rulebook` / `advanced_players_guide` / `advanced_race_guide`

- **Card ID:** `epic-2-cause-closure` (row 11; T2b shape, lane scope: `core_rulebook`,
  `advanced_players_guide`, `advanced_race_guide` per dispatch brief `wf_c05d169f-70e-3`,
  104 nominal units per `card11-t2b-census-census.md §4`)
- **Actor:** `t2b-w1-c`
- **Base:** `45fef71f0` (pinned `PIN`), rebuilt via `git reset --hard` after this worktree was cut
  from a stray `site-publish` merge commit (footgun 1) with no `docs/`/`data/`/`scripts/` tree.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`), re-fetched fresh into this worktree via `scripts/fetch-pcgen-oracle.sh`,
  re-verified via `scripts/verify.sh --only preflight-oracle` → PASS.
- **Status:** measurement cycle — **0 units banked**, per `decisions.md §13`'s explicit
  authorization ("measurement... does not substitute for the work... a precursor to it. A
  measurement cycle that banks zero units but produces a real, re-derivable book/file census is a
  legitimate closed cycle"). **No production files touched** — `ingest_races.rs`,
  `ingest_race_traits.rs`, `race_catalog.rs` are unchanged by this cycle.
- **Files touched:** this receipt; `docs/retro/events/t2b-w1-c.jsonl` (3 corrections, appended by
  `scripts/retro.py`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (no production diff exists this cycle)
- **Wired-integration audit result:** `OK_NO_TOKENS` (no production diff exists this cycle)

## 0. What this cycle found

The dispatch brief characterized all three books as needing only "ingest-tool extension... roughly
3 files each," following `card11-t2b-census-census.md`'s "other (never-transcribed per-record
content)" classification. **Re-deriving each book's actual raw-corpus content against the pinned
oracle disproves that characterization for all three books.** None of the 104 nominal units is a
same-shaped "row exists in the `.lst`, was never copied into `data/corpus`" gap the way the
memo's worked examples (e.g. `inner_sea_races: Kasatha ~ Stealthy`) are. Full findings logged as
three `scripts/retro.py correction` events (`docs/retro/events/t2b-w1-c.jsonl`); summarized here.

### `core_rulebook` — 14 nominal, **0 real T2b work**

Re-derive: `python3 scripts/t2b_race_trait_census.py --dump-other 2>&1 | grep '^#   core_rulebook:'`

| Sub-shape | Units | Disposition |
|---|---:|---|
| PCGen sentinel/placeholder rows | 4 | Not work. `Region ~ None`/`Region ~ Unknown` sit under `cr_abilities_race.lst`'s own `###Block: Placeholder objects for no Human Ethnicities or Regional Affinities`; `No Race Trait Available` and `Remove Excess Points from Pool` sit under `###Block: Racial Trait Support` — both block headers name themselves as non-content. |
| Duplicate of already-ingested `class_feature` content | 4 | Not work. `Favored Enemy ~ Humanoid (Gnome/Halfling/Human/Orc)` are Ranger favored-enemy CHOOSE options, filed in `_abilities_race.lst` only because PCGen groups all `Special Ability`-category rows together; already correctly ingested as `kind: class_feature` at `data/corpus/core_rulebook/class_feature/favored_enemy/humanoid_{gnome,halfling,human,orc}.json`. |
| Duplicate of already-covered mechanism | 6 | Not work. `+2 Strength/Dex/Con/Int/Wis/Cha` are unattributed CHOOSE-target primitives for the flexible-ability-bonus mechanism. `race_creation.rs::floating_ability_bonus_points` already implements this mechanism as a point-count sourced from each race's own `*_ability_scores.json` (`data/corpus/core_rulebook/race_trait/human/human_ability_scores.json`, `.../half_elf/half_elf_ability_scores.json`); the picker offers the fixed six-ability choice directly and does not consult these generic rows. |

**Corrected population: core_rulebook contributes 0 units to T2b, not 14.**

### `advanced_players_guide` — 37 nominal, **0 real T2b work**

Re-derive: `sed -n '112,122p;249,306p' <oracle>/advanced_players_guide/apg_abilities_race.lst`

All 37 sit inside `apg_abilities_race.lst`'s own `###Block: Favored Classes` and `###Block:
Favored Class helper abilities` — PCGen's internal Favored-Class-Bonus engine plumbing:
per-class enablement rows (`Alchemist`, `Cavalier`, `Inquisitor`, `Oracle`, `Summoner`, `Witch` —
`BONUS:ABILITYPOOL|Favored Class Bonus|var("CL=<class>")`), per-class-per-spell-level bonus-known
trackers (`Bard/Inquisitor/Oracle/Sorcerer Spell Level 0-N` — `BONUS:SPELLKNOWN|CLASS=...`), and
misc internal counters (`Elf Barbarian Counter`, `CATEGORY:Internal`). **None of the 37 names or
applies to a race** — `corpus_key` for every one of them is a bare class name or an internal
counter label, never a `<Race> ~ ...` compound key. The census's own header-pattern classifier
correctly excludes `^Favored Class Bonus ~ ` rows but this "Favored Classes"/helper-abilities
block predates that prefix convention and was swept into "other" by omission, not by evidence of
being race content.

**Corrected population: advanced_players_guide contributes 0 units to T2b, not 37.**

### `advanced_race_guide` — 53 nominal, **7 real (new-mechanism) units, 46 not T2b work**

Re-derive: `grep -n 'Mystic Past Life\|Racial Subtype ~ None\|CATEGORY:Adoptive Parentage\|Heart of the' <oracle>/advanced_race_guide/arg_abilities_race.lst`

| Sub-shape | Units | Disposition |
|---|---:|---|
| Blocked — race not in `IN_SCOPE_RACES` | 6 | `Changeling ~ {Annis Hag Mist Child, Green Hag Object of Desire, Sea Hag Ocean's Daughter}` (3), `Dhampir ~ {Dayborn, Fangs, Vampiric Empathy}` (3). `ingest_race_traits.rs`'s own `in_scope_roster_is_exactly_the_34_races...` test asserts `Changeling`/`Dhampir`/`Samsaran` "still deferred and must not be in scope." Building a chassis for either race is a new-race-onboarding epic, not an ingest-tool extension, and is out of this lane's granted scope. |
| Blocked — race not in scope **and** needs the formula interpreter | 29 | `Samsaran ~ Mystic Past Life` (1) + its 28 `Mystic Past Life {Add,Base} ~ <class>` CHOOSE-target primitives (14 classes × 2). Samsaran is deferred (as above); the mechanism itself resolves ability-score-dependent formulas (`BONUS:VAR|MysticPastLifeINTBonus|(MysticPastLifeScoreINT-10)/2`) that `decisions.md §24`'s formula-interpreter ban forbids resolving here even if the race were in scope. |
| Duplicate selector-shim for already-ingested content | 8 | `Heart of the {Fields, Mountain, Sea, Slums, Snows, Streets, Sun, Wilderness}`. The real Human alternate traits (`KEY:Human ~ Heart of the X`) are already ingested at `data/corpus/advanced_race_guide/race_trait/human/human_heart_of_the_*.json`, sourced from `arg_abilities_race.lst:261-268`. The residual rows are a *second*, unkeyed block at `:279-286` (`###Block: Selections for Mixed Heritage`) whose only function is `ABILITY:Human Racial Trait|AUTOMATIC|Human ~ Heart of the X` — an `AUTOMATIC` grant-link to the trait already ingested, not new content. |
| `.MOD` overlay, zero new magnitude | 1 | `Vishkanya ~ Toxic ~ Vishkanya Venom` — `docs/work-inventory.json` marks this unit `origin: "mod_only"`, `magnitude_token_count: 0`; it modifies the already-ingested `Vishkanya ~ Toxic` ability (`data/corpus/advanced_race_guide/race_trait/vishkanya/vishkanya_toxic.json`), not a standalone new trait. |
| Sentinel / no race attribution | 2 | `Racial Subtype ~ None` ("No Racial Subtype" / "You have chosen no racial subtype.") and `Fins to Feet` (a shared `CATEGORY:Spell-Like Ability` primitive with no `KEY`, referenced by other races' `Racial SLA ~ ...` rows via `ABILITY:...AUTOMATIC`, not itself race-scoped). |
| **Real, closeable — but a new mechanism, not an ingest-tool row extension** | 7 | `Drow`, `Dwarf`, `Elf`, `Gnome`, `Halfling`, `Orc`, `Grippli` under `###Block: Adoptive Parentage Options` (`arg_abilities_race.lst:291-297`). All seven races are in `IN_SCOPE_RACES`. This is a genuine "Adopted Race" selector-plus-grant-link gap — the **same shape** `card11-t2b-census-census.md §3` names as needing new capture logic for the 9 `Adopted Race ~ <X>` units in `bestiary_2`/`bestiary_5`/`bestiary_6` ("real work, `ingest_races.rs`'s flat standard-trait loop never captures this shape"), just under a different corpus-key naming convention (no `Adopted Race ~ ` prefix here). Building the selector is out of this cycle's scope for two reasons: (a) it touches the same shared `ingest_races.rs`/`race_catalog.rs` surface sibling T2b lanes are concurrently editing for the bestiary_2/5/6 half of the identical mechanism, and a half-built duplicate mechanism from two lanes at once is a collision risk `workflow-instruction.md §5`'s protocol does not cover; (b) it is new-mechanism engineering, not the "≈3 files, ingest-tool extension only" scope this lane was dispatched under. |

**Corrected population: advanced_race_guide contributes at most 7 units to T2b (the Adoptive
Parentage selector gap), not 53. The other 46 are not T2b work under this shape's own definition
(a corpus record that names a race and was never transcribed).**

## 1. Why nothing was ingested this cycle

`decisions.md §3` (fixture discipline, non-negotiable) and `decisions.md §1a` (anti-gaming: "a
placeholder predicate ... cannot manufacture false coverage") both forbid writing a `race_trait`
corpus record for a row that does not, in fact, describe a race trait belonging to a modelled
race. Every one of the 97 non-Adoptive-Parentage nominal units across these three books fails that
bar for a specific, cited reason above — sentinel, duplicate, `.MOD` overlay, or blocked on scope
this lane was not granted (a new race chassis, or the formula interpreter). Force-ingesting any of
them would either (a) fabricate race content the corpus does not actually assert, or (b) write a
byte-duplicate of a record that already exists, silently doubling the corpus and breaking the
resolver's own `duplicate resolved trait` panic guard (the exact hazard `ingest_race_traits.rs`'s
own APG investigation, cited in its `BOOK_SOURCES` doc comment, already hit and reverted once).

## 2. Disposition (Blocker Discipline, `AGENTS.md`)

This is a **raise-your-hand** escalation on the 97 non-closeable units, not a deferral filing.
What is needed to clear it:

1. **core_rulebook (14), advanced_players_guide (37):** no ruling needed to *close* them — they
   are not T2b work at all. What is needed is an operator/consolidation-cycle decision on how
   `docs/work-inventory.json`'s classifier should stop tagging PCGen's internal Favored-Class-Bonus
   plumbing and Ranger favored-enemy duplicates as `kind: race_trait, evidence:
   race_trait_race_not_modelled` — this is a **census/classifier defect upstream of T2b**, not a
   T2b closure gap. Until that classifier fix lands, these 51 units will keep showing up in T2b's
   count on every re-run of `scripts/t2b_race_trait_census.py`.
2. **advanced_race_guide's 46 non-Adoptive-Parentage units:** 6 + 29 need a ruling on whether
   Changeling/Dhampir/Samsaran chassis-building is in this bundle's scope (it reads as a new-race
   epic, not book onboarding); the 29 additionally need the formula interpreter, itself
   `decisions.md §24`-forbidden here. The 8 Heart-of-the-X + 1 Vishkanya + 2 sentinel units need
   the same classifier fix as item 1.
3. **advanced_race_guide's 7 Adoptive Parentage units:** real, closeable work, but is a new
   selector mechanism shared with the bestiary_2/5/6 `Adopted Race ~ <X>` gap (9 units, a sibling
   T2b lane's scope). Recommend a single follow-up cycle builds the "Adopted Race" selector once,
   scoped explicitly across all four books together, rather than two lanes race-conditioning the
   same `ingest_races.rs` mechanism independently.

## 3. Verification run this cycle

- `scripts/verify.sh --only preflight-oracle` → PASS (oracle `7f818006e371188e5717fd18d74d18a420747fc6`).
- `python3 scripts/t2b_race_trait_census.py --dump-other 2>&1 | grep -c '^#   <book>:'` → 53 / 37 /
  14 for `advanced_race_guide` / `advanced_players_guide` / `core_rulebook` respectively, matching
  `card11-t2b-census-census.md §4` exactly (no correction to the *nominal* population — the
  correction is entirely about what fraction of each is real T2b work).
- Every disposition above is backed by a `grep`/`find`/`python3 -c` command against the pinned
  oracle checkout or the committed `data/corpus`/`docs/work-inventory.json`, run live this cycle
  (not quoted from a prior receipt).
- No `cargo build`/`cargo test` run: no Rust source was touched.

## 4. RED → GREEN evidence

Not applicable — no code changed. Per `workflow-instruction.md §8`'s self-heal posture and
`decisions.md §13`'s explicit authorization, a measurement cycle that produces a real,
re-derivable finding and forwards it is a legitimate closed cycle in its own right.

## 5. Discovery forwards

- **DISCOVERED — work-inventory.json's T2b classifier tags non-race content as `race_trait`.**
  `evidence: "race_trait_race_not_modelled"` fires for any unmatched row in a book's
  `*_abilities_race.lst`, including rows that never named a race in the first place (PCGen's
  shared Favored-Class-Bonus plumbing, Ranger favored-enemy options, ability-bonus CHOOSE
  primitives, `.MOD` overlays, sentinel placeholders). At minimum `core_rulebook` (14) and
  `advanced_players_guide` (37) — 51 units, 2.1% of the nominal 2,472 T2b population — are
  affected. Forwarded for a consolidation cycle or operator ruling; not something this lane's
  granted scope (ingest-tool extension for 3 named books) can fix, since the fix site is the
  classifier that *produces* `docs/work-inventory.json`, not the ingest tools that consume it.
- **DISCOVERED — the "Adopted Race" selector mechanism spans 4 books, not the 3 the census
  memo split it across.** `bestiary_2`/`bestiary_5`/`bestiary_6`'s 9 `Adopted Race ~ <X>` units and
  `advanced_race_guide`'s 7 `CATEGORY:Adoptive Parentage` units are the same PCGen mechanism
  (a selector ability that `AUTOMATIC`-grants an adopted race's standard traits), named
  differently only because ARG predates the `Adopted Race ~ ` KEY-prefix convention the later
  books use. Recommend one cycle builds it once, scoped across all four books.

## 6. Next-cycle plan

None for this lane on these three books — 0 bankable units remain after the above corrections
(the 7 Adoptive Parentage units belong to the cross-book selector-mechanism follow-up named in
§2 item 3, not to a same-shaped re-run of this lane). Escalating per `AGENTS.md` Blocker
Discipline disposition 2.
