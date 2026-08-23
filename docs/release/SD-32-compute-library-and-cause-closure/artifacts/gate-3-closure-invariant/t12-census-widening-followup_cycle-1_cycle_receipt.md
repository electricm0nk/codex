# Cycle t12-census-widening-followup — Gate 3 (closure invariant) / Card 11 (row 15's T12 remainder: the psion cycle's census-widening discoveries)

- **Card ID:** 11 (`epic-2-cause-closure`), row 15's T12 sub-scope: the ten records the `psion` cycle's
  widened census surfaced (`epic-2-t12-psion-shape3-closure_cycle-1_cycle_receipt.md`, commit
  `1b2dbfcdbc`) — 7 sibling `"<ClassName> Manifesting"` records and 3 new `antipaladin` records — plus
  resolving the `PsionPowersKnown`/`PsionMaxPowerLevel` escalation, sizing the pool-shaped population
  generically, and re-deriving `no_record`/row 17.
- **Commit SHA:** see `git log -1` at push time (rebased onto `origin/tranche/12` before pushing, §5)
- **Files touched:**
  - `src/rules_core/rules_tables/apg/antipaladin_features.rs` — **new**: `aura_of_evil_strength_level`,
    `detect_good_caster_level`, `smite_good_uses_per_day`, `smite_good_attack_and_ac_bonus`,
    `smite_good_damage_bonus`
  - `src/rules_core/rules_tables/ultimate_psionics/{cryptic,dread,marksman,psychic_warrior,tactician,
    vitalist,wilder}_features.rs` — **new**: `<class>_power_points_total`, `<class>_powers_known`,
    `<class>_max_power_level` (21 functions total, 3 per class)
  - `src/rules_core/rules_tables/ultimate_psionics/psion_features.rs` — **new**: `psion_powers_known`,
    `psion_max_power_level`, resolving (not re-escalating) the prior cycle's `PsionPowersKnown`/
    `PsionMaxPowerLevel` combination-semantics blocker
  - `src/rules_core/pilot_compute/mod.rs` — `ground_antipaladin_class_features` extended; the 7 sibling
    `ground_<class>_class_features` functions extended and (where not already taking them) given
    `input: &CharacterInput`/`ability_modifiers: &AbilityModifiers` parameters; `ground_psion_class_features`
    extended and given `input`; dispatch call sites updated for all 8 changed signatures; one stale test
    renamed and rewritten (`antipaladin_level_1_has_none_of_the_seven_magnitudes_yet` →
    `..._has_none_of_the_original_seven_magnitudes_yet`, since 3 new min_level-1 magnitudes now
    legitimately appear at level 1); 7 existing level-20 wiring tests extended with new assertions
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 cycle entry prepended (row
    left `in-progress` per dispatch instruction)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (working-tree diff on this cycle's own touched files,
  `git diff --unified=0` against the pre-cycle HEAD: `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`
  — 0 hits outside this cycle's own doc-comment citations of the cycle id itself, matching the convention
  every prior T12 cycle used)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"`: 0 hits)
- **Acceptance criterion:** close the 10 records the psion cycle's widened census named (7 sibling
  Manifesting records + 3 antipaladin records) with real compute functions, wired through
  `pilot_compute`, RED→GREEN at both altitudes; check the escalated `PsionPowersKnown`/
  `PsionMaxPowerLevel` claim against this repo's own PCGen-sourced resolution before accepting it; size
  (not just psion's 32) the whole pool-shaped exclusion class; re-derive `no_record` and row 17.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/fetch-pcgen-oracle.sh`
  bootstrap into this fresh worktree's repo-local slot; every formula below re-derived directly against it)
- **Status:** complete (this cycle's scope: the 10 named records, the escalation check, the pool sizing,
  and the re-derivation). The pool-shaped population's own closure (thousands of records) is sized, not
  closed — see §5.
- **Discovery forwards:** `## DISCOVERED` — the pool-shaped exclusion class (`class_feature_pool_catalog.rs`)
  is far larger than psion's 32 records: ~1,913 distinct `" ~ "`-group-qualified names across the corpus,
  ~16,350 total records, ~6,131 magnitude-bearing, of which the catalog's own `REGISTERED_POOL_GROUPS`
  currently models only 2 groups (Rogue Talent, Rage Power — ~71 magnitude-bearing records). The
  remaining scope needs a dedicated sizing/closure cycle and reconciliation against other mechanisms that
  may already close parts of it (e.g. Domain Power per `decisions.md §23a`).
- **Next-cycle plan:** dedicated pool-catalog sizing/closure cycle — reconcile the raw ~6,131 figure
  against records already closed by other mechanisms (Domain Power via §23a, Rage Power/Rogue Talent via
  the existing catalog), then close remaining pool groups by class, largest first (Domain Power 172,
  Inquisitor Domain 124, Warpriest Bonus Feat 432, Aegis 126, ...). Sibling `monster_ability`
  (`no_record` 56) and PI (`declared_pi_shipping_audit` 65) lanes untouched, per territory.

---

## 1. The 10 named records: closed (§17a — re-derived, not inherited)

### Antipaladin's 3 new records (`aura_of_evil`, `detect_good`, `smite_good`)

Re-derived directly against `core_rulebook/cr_abilities_class.lst` (`Aura of Evil`) and
`advanced_players_guide/apg_abilities_globalvar.lst` (`Detect Good`, `Smite Good`), confirmed by the
already-ingested corpus records (`data/corpus/core_rulebook/class_feature/aura_of_evil/aura_of_evil.json`,
`data/corpus/advanced_players_guide/class_feature/{detect_good,smite_good}/*.json`), all three min_level 1
per the roster fixture:

- **Aura of Evil**: `BONUS:VAR|AuraEvilLVL|AlignmentAuraLVL`, a pure class-level pass-through selecting
  one of four `PREVAR`-gated `DESC` tiers. Grounded as `aura_of_evil_strength_level(level)`.
- **Detect Good**: `SPELLS:Class|TIMES=ATWILL|CASTERLEVEL=DetectGoodLVL|Detect Good,11+WIS` — an at-will
  spell-like ability, `DetectGoodLVL` a pure level pass-through. Grounded as
  `detect_good_caster_level(level)`.
- **Smite Good**: three `BONUS:VAR` tokens — `SmiteGoodTimes|min((SmiteGoodLVL+2)/3,7)`,
  `SmiteGoodAttackBonus,SmiteGoodACBonus|max(CHA,0)` (one token sets both), `SmiteGoodDamageBonus|
  SmiteGoodLVL` (doubled at the call site for the record's own `%4 = SmiteGoodDamageBonus*2`
  substitution against a good outsider/dragon/cleric-or-paladin). Grounded as
  `smite_good_uses_per_day`, `smite_good_attack_and_ac_bonus`, `smite_good_damage_bonus`.

### The 7 sibling `"<ClassName> Manifesting"` records

Every sibling shares the exact shape-3 grant convention the psion cycle documented
(`ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName> Manifesting`, no `" ~ "` group prefix), backed
by a `<ClassName> Manifesting Variables` record (`PowersKnown`/`MaxPowerLevel`) and a
`<ClassName> Power Points` record (`BasePowerPoints` ladder + `BonusPowerPoints`), re-derived directly
against each class's own `up_abilities_class.lst` block:

| Class | Ladder shape | Powers Known shape | Ability |
|---|---|---|---|
| Cryptic | half-manifester (1,1,2,2,2,4×5,8×5,12×5) | single term = PKL | INT |
| Dread | same half-manifester ladder | single term = PKL | CHA |
| Psychic Warrior | same half-manifester ladder | single term = PKL | WIS |
| Marksman | slower (1,1,1,2×4,3×3,4×3,5×4,6×3) | **TWO terms, SUM** (see §2) | WIS |
| Tactician | full-manifester (psion's own ladder) | single term = PKL | INT |
| Vitalist | full-manifester ladder | `1+floor((PKL+1)/2)` | WIS |
| Wilder | full-manifester ladder | `1+floor(PKL/2)` | CHA |

`MaxPowerLevel` is a single `BONUS:VAR` term for every sibling except Marksman (whose single term
carries its own `PREVARGTEQ:MarksmanMPL,2` gate — 0 below level 2), so no combination question for any
of the six single-term classes. `PowerPoints` = ladder value + `(ability_modifier * level) / 2`
(`BonusPowerPoints`, always a single unambiguous term).

## 2. Marksman's `MarksmanPowersKnown`: the same two-term shape psion escalated, resolved the same way

`MarksmanPowersKnown` carries `min(9,floor((3*MarksmanPKL-1)/4))` (unconditional) plus
`floor((MarksmanLVL-13)/2)|PREVARGTEQ:MarksmanPKL,15` (gated) — structurally identical to
`PsionPowersKnown`'s two-term shape. Resolved by the same SUM semantics §3 below documents (not
re-escalated): below level 15 only the base term is active; from level 15 both sum.
`marksman_powers_known(20) = min(9,14)=9 + floor(7/2)=3 = 12`.

## 3. `PsionPowersKnown`/`PsionMaxPowerLevel`: checked against the claim, resolved (§27b: check before accepting)

The brief's instruction: *"Check that claim before accepting it... Look for PCGen's own definition of
how those `BONUS:VAR` terms combine."* This worktree's pinned oracle checkout
(`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/`) is
**data-only** (no `src/` Java tree — `ls` confirms only `data/`, `system/`, build files), so PCGen's own
Java source is not directly present in this checkout. **It did not need to be**: this repo's own
`src/rules_core/pilot_compute/bonus_stack_reader.rs` module doc (point 2) already cites real PCGen
source directly — `pcgen/core/PlayerCharacter.java:2136` → `BonusManager.java`'s `getTotalBonusTo`/
`sumActiveBonusMap` — documenting that **multiple `BONUS:VAR` entries sharing one target variable SUM**,
each gated by its own currently-passing `PREVARGTEQ`. This is not a guess or this reader's own policy
choice; it is the real PCGen aggregation, already load-bearing elsewhere in this codebase (`WitchWardBonus`,
cited in that same module doc).

Applied to `PsionPowersKnown`: below `PsionPKL>=11` only the unconditional term is active; from
`PsionPKL>=11` both terms sum. This is the exact "sum" reading the escalating cycle had already
identified as the plausible one (versus "replace", which produces an implausible level-11 drop from 21
to 1) — the citation confirms it as PCGen's documented real behavior rather than leaving it a coin flip.
**Closed, not re-escalated**: `psion_powers_known(11) = 22`, `psion_powers_known(20) = 36`.
`PsionMaxPowerLevel` has only ONE `BONUS:VAR` term on the record — no combination question at all — closed
as `psion_max_power_level(level, int_score) = min(9, floor((level+1)/2), int_score-10)`.

A mutation-proof test (`psion_powers_known_mutation_proof_replace_semantics_would_drop_at_level_eleven`)
documents the "replace" reading's implausible value (1) is NOT what this function returns, making the
"why sum and not replace" reasoning independently checkable, not just asserted in prose.

## 4. RED→GREEN, proven live at both altitudes (§1a)

Mutated `marksman_powers_known`'s base term (`+99`) and `smite_good_damage_bonus`'s level term (`+99`):
```
marksman_powers_known_only_the_base_term_below_level_fifteen ... FAILED (left: Some(99), right: Some(0))
marksman_powers_known_sums_both_terms_from_level_fifteen ... FAILED (left: Some(109), right: Some(10))
marksman_level_20_reaches_every_real_magnitude_with_a_real_dexterity_score ... FAILED (left: 111, right: 12)

smite_good_damage_bonus_equals_class_level_and_the_call_site_doubles_it ... FAILED (left: Some(104), right: Some(5))
antipaladin_level_20_reaches_every_real_magnitude_with_a_real_charisma_score ... FAILED (left: 119, right: 20)
```
Both the unit-level compute tests AND the end-to-end wiring tests (through
`compute_pilot_base_chassis → compute_class_chassis → ground_<class>_class_features`) failed for the
intended reason in both cases. Reverted; re-ran the full targeted scope:
```bash
cargo test --locked --lib -- cryptic dread marksman psychic_warrior tactician vitalist wilder psion \
  antipaladin kineticist medium_features mesmerist_features occultist_features spiritualist_features \
  magus_features shifter_features vigilante_features aegis soulknife each_new_class_lacks \
  a_class_with_no_roster_data
```
`212/212 green`, no regressions in any pre-existing class's test.

## 5. Sizing the whole pool-shaped exclusion class (§27b: size the whole class, not just psion's 32)

The psion cycle sized only its own 32-record discipline/archetype pool. The brief's instruction: *"If
`Vigilante Talent` and `Magus Arcana` share this shape, they are part of the same population — size the
whole class of it, not just psion's 32."*

**Method** (ad-hoc script, not committed — re-derivable, command below): scan every
`data/corpus/*/class_feature/**/*.json` record whose own `key` contains `" ~ "` (a group-qualified name,
the same marker `census_untabled_base_class_feature_roster.py`'s own module doc names as the
pool-exclusion signal). This is a **raw upper bound**, not a precise "unclosed pool-catalog population" —
it includes records some OTHER already-built mechanism may already close (e.g. `Domain Power`'s 172
records are the subject of `decisions.md §23a`'s separate generator-input-extension fix, not necessarily
this catalog's own construction discipline).

```bash
python3 scratch_pool_size.py   # script content in this receipt's own commit message context; re-derivable
                                # by scanning data/corpus/*/class_feature/**/*.json for `" ~ "`-qualified
                                # keys and counting BONUS/DESC-%N-bearing records per group
```

**Result:**

| | Count |
|---|---:|
| Distinct `" ~ "`-group-qualified names (candidate pool groups) | ~1,913 |
| Total group-qualified records | ~16,350 |
| Magnitude-bearing among them (has `BONUS` or `%N`-substituted `DESC`) | ~6,131 |
| Currently modeled by `class_feature_pool_catalog.rs::REGISTERED_POOL_GROUPS` | 2 groups (Rogue Talent, Rage Power) — ~71 magnitude-bearing records |

Largest unmodeled groups by magnitude-bearing record count: Domain Power (148), Inquisitor Domain (106),
Refined Education (94), Social Grace (85), Aegis (88), Warpriest Bonus Feat (32), Rage Power (38,
already modeled), Rogue Talent (33, already modeled), Discovery (31), Shaman Spirit Hex (46), Divine
Scion (41), Vigilante Talent (40), Shifter Aspect (37), Investigator (37), Witch Hex (35), Medium (42).

**Not closed this cycle** — sized and named, per `§27b` ("novelty of shape is grounds for sizing, not
exclusion"). The true remaining scope is on the order of **thousands of records**, not the 32 psion's
own cycle found, and needs a dedicated cycle to (a) reconcile this raw count against records other
mechanisms already close and (b) build the pool-catalog closure for the residual, largest groups first.

## 6. Re-derivation (§17a — re-derive, don't inherit the brief's own figures)

```bash
python3 scripts/row17_census.py --check
```
```
row 17 census (decisions.md §27a/§27b, kanban.md row 17) — population 34397
  §27 provisional default            23   (corpus-wide total incl. done units: 24)
  ROW 17 HONEST SIZE                 23
  not_ingested (no_record)           56   -- row 17 starts only after this reaches 0
```
`23` matches the brief exactly; `--check` exits with the marker well-formed both before and after (no
new stamps applied this cycle — this cycle closed compute functions, not shape defaults). `56`
`no_record` matches the brief exactly, all `monster_ability`, sibling lane's own territory, untouched
this cycle (confirmed: `git status --porcelain -- data/corpus/**/monster_ability/**` shows no changes).

**Re-checking previously-closed classes for further gaps** (the brief's own highest-value ask): the
census-widening BFS the psion cycle ran was scoped to `CLASS:Psion`'s own block only. A corpus-wide
generic re-run of the SAME shape-3 marker (`ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName
Manifesting>`, no `" ~ "` prefix) against every OTHER already-"complete" T12 class found **zero**
additional shape-3 hits beyond the 7 siblings + psion already closed — the roster fixture's own
before/after diff (`235 → 246` entries, `+11`: the 3 antipaladin + 7 siblings + psion, all now closed)
is stable under a second re-run with no further growth:
```bash
python3 scripts/census_untabled_base_class_feature_roster.py --summary 2>&1 | tail -5
```
Actually re-ran (not assumed): `total records: 246`, all 19 classes report at least one record
(`classes with NO .MOD-shaped own-named grant found: []`), and `git status --porcelain` shows the
committed `tests/fixtures/rules_core/untabled-base-class-feature-roster.json` **unchanged** by the
re-run — the fixture the psion cycle produced (`235 → 246`, `+11`: the 3 antipaladin + 7 siblings +
psion, all now closed) is confirmed stable under a fresh oracle-grounded regeneration, not merely
assumed. No further "108/108 was wrong" gaps found for the 9 classes cycle 4 closed (Kineticist, Medium,
Mesmerist, Occultist, Psychic, Spiritualist, Magus, Shifter, Vigilante) or the earlier 10 (Antipaladin,
Cryptic, Dread, Marksman, Psychic Warrior, Soulknife, Aegis, Tactician, Vitalist, Wilder) beyond the 10
this cycle closed — the roster mechanism's own population is stable at 246 entries / 118 magnitude-bearing
(108 original + 10 this cycle) across 19 classes. §5's pool-shaped population is the real remaining T12
scope, not further roster-mechanism gaps.

## 7. Sweep (§3)

`grep -rn "108 of 108\|108/108\|235 total\|246 total" tests/ src/ scripts/ apps/` — no count-pinning hits
outside doc-comment prose this cycle's own receipt supersedes (no test/script/app asserted the literal
`108`/`235` figures as a hard pin; the roster fixture itself is regenerated, not hand-pinned, and this
cycle did not regenerate it — the 10 new magnitudes are grounded via existing roster entries whose
`text_only` flag the census already set correctly).

## 8. Scope discipline

Did not attempt: closing the pool-shaped population itself (sized in §5, real work for a dedicated
cycle). Did not touch `data/corpus/**/monster_ability/**`, `scripts/transcribe_monster_tables.py`,
`monster_chassis.rs` (the sibling `monster_ability` lane), or `scripts/pi_scrub.py`/
`declared_pi_shipping_audit`/PI screening in `cache_gen::{acg,apg,beastiary1}.rs` (the sibling PI lane).
Row 11 left `in-progress` per dispatch instruction.

`df -h /`: reported in the dispatch's final report.
