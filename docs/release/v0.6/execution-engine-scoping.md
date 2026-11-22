# Execution-Engine Scoping — does this product need execution engines?

> Scoped 2026-07-29 against `tranche/6` @ `2346527b`. Every count re-derived
> from code and corpus that day, not carried from any prior document. Lead
> independently verified the four load-bearing claims before acting (§6).

## Verdict

**No. Build none of them.**

The execution-engine framing is a category error. The ~252 deferrals that cite
a missing engine are, in overwhelming majority, **caveats attached to numbers
the engine already computes correctly**. The real gap is one layer downstream
and has nothing to do with simulation:

> **The engine's computed facts do not cross the IPC boundary into the sheet.**

`pilot_compute.rs` emits **636+ distinct `ComputationExplanation` ids**, each
corpus-cited. `LoadSavedCharacterResponse` (`character_hub.rs`) has **no
`explanations` field at all** — verified: the only matches in that file are a
local `discarded_explanations` binding and a doc comment; the frontend has one
hit, in an unrelated operator-triage panel.

Every sneak-attack die count, rage bonus, lay-on-hands pool, channel-energy
die, ki pool, bomb die, judgment bonus and hex DC is computed, tested,
corpus-cited — and then dropped at the boundary.

## 1. Inventory

252 occurrences / 129 distinct phrasings of `no <X> engine exists`
(continuation-normalised). By host construct — the decisive cut:

| where the phrase lives | count | meaning |
|---|---|---|
| In a `ComputationExplanation` **with a real computed magnitude** | **71** | the number exists; the caveat only says nothing consumes it |
| In a `ComputationExplanation` with `value: 0` | 40 | no magnitude exists in the corpus |
| In a `ComputationDiagnostic` | 29 | honest unmodelled-scope notices |
| In code comments (design rationale) | ~112 | not a deferral surface |

15% of raw hits are not compute code at all — 38 live in
`support_state_matrix.rs`, whose own module doc says it *"deliberately does not
compute character mechanics."*

## 2. The static / dynamic split

| category | share | disposition |
|---|---|---|
| Static, **already computed**, blocked only on transport | ~71 in-record + most of the 112 comments | close by shipping the sheet |
| Static, **genuinely missing**, computable today | ~8 sheet numbers (§3) | build — cheap arithmetic |
| No magnitude anywhere in the corpus | ~40 | **correctly deferred permanently**; description text is the complete representation |
| Genuinely dynamic (RNG / opponent / turn state) | attack-resolution 29, action-economy 20 | **not needed to print a sheet; defer permanently** |

**Not one of the 252 requires an execution engine for the correct number to
reach the player.**

## 3. The real static gaps — the buildable list

| missing sheet number | evidence | cost |
|---|---|---|
| **Caster level, every full caster** | only 3 `caster_level` ids exist: Monk abundant step, Paladin/Ranger partials. Wizard, Cleric, Sorcerer, Druid, Bard, Oracle, Witch, Shaman, Arcanist, Summoner have **none** | trivial — CL = class level |
| **CMB / CMD** | no total; only feat halves | `BAB+STR+size` / `10+BAB+STR+DEX+size`; `size.rs` exists |
| **Initiative total** | bonuses exist, no total | `DEX + bonuses` |
| **Touch AC / flat-footed AC** | zero ids | subsets of an AC already computed |
| **Concentration** | one id, and it is a deferral note | `CL + casting stat mod` |
| **HP above level 1** | `maxHitPoints()` lives in **TypeScript** | move to engine |
| **Speed total** | `base_land_speed_feet()` exists; Monk Fast Movement still defers on "no speed-total engine" | stale |

## 4. The dice ruling (operator, 2026-07-29)

> *"we don't need to do dice rolls. but we do need to calculate for a spell how
> many dice are able to be rolled based on the character level."*

**Class features: already satisfied.** 30 explanation records already carry a
level-scaled die expression — rogue sneak attack `(lvl+1)/2`d6, slayer
`lvl/3`d6, cleric/paladin channel dice, lay-on-hands, warpriest sacred weapon
die, alchemist bomb, monk unarmed strike, investigator studied strike. The
damage/healing/lay-on-hands clusters are therefore **static and already
computed** — they need transport, not arithmetic.

**Spells: blocked by the corpus, not the engine.** All 1093 spell records carry
only `key`, `school`, `level`, `description`, `full_text`. **PCGen's spell
schema has no damage/dice token at all** — `LstSpellRecord` extracts every
column upstream offers and none is damage. Fireball's record is prose:
*"1d6 per caster level [maximum 10d6]"*. 347 of 1093 descriptions contain a
dice expression; 121 contain dice **and** per-caster-level scaling; 47 name a cap.

Computing "10d6" therefore means NL extraction over 1093 prose records or
hand-authoring 121–347 formulas at this project's 2–3-source verification bar.
That is a **corpus-authoring project**, not an execution-engine one.

**Cheaper substitute capturing most of the value:** the Spells tab already
renders the full description, so *"1d6 per caster level (max 10d6)"* is already
on screen. The missing piece is the one number that makes it usable — **caster
level**. One record per caster class. ~1% of the effort for ~80% of the outcome.

**Representation note:** `ComputationExplanation.value` is `i16` and
`PrintedSheetCellValue` is `Number(i16) | Blocked` — neither can hold "10d6".
A `DiceExpression { count, die_size }` already exists in `damage_total.rs` but
is confined to weapons; class features work around it by splitting count and
size across two records. Promoting it to a first-class receipt value is the
correct enabling change, and it is small.

## 5. Architecture — why execution conflicts

The engine is a deterministic calculator whose trust rests on three properties:
every output is a corpus-cited `ComputationExplanation`; `claim_blocking`
diagnostics gate `Computed`/`Blocked` so absence renders honestly; tests assert
byte-exact values across all 20 levels.

Randomness has no representation in `value: i16`. Opponent state has none in
`CharacterInput`. Turn sequencing has none anywhere. Introducing any of the
three inside `pilot_compute.rs` breaks determinism, which breaks the test
strategy, which is the only reason the engine is trustworthy. A simulator
producing a plausible uncited number is exactly what the no-stub doctrine
forbids.

**If execution is ever built it must be a separate layer above the calculator**,
consuming finished receipts, with its own test strategy (seeded RNG, property
tests).

**The codebase already solved "state without simulation."**
`ClassAbilityActivation { ability_id, active_state, rounds_consumed_today }`
lets the *player declare* "I am raging"; the engine then computes
deterministically under that declared scope. Already consumed by rage, mutagen,
judgment and challenge. Declared scope is cheaper than simulation and more
honest than folding a conditional bonus into a total.

**Four deferral strings are factually false against today's code** and ship in
user-visible `detail` text: *no skill-check-total engine* (`SkillTotals`
exists), *no saving-throw-total engine* (`total_saves` exists), *no
Armor-Class-total engine* (`baseline_armor_class` exists — the code even
corrects itself nearby), *no speed-total engine* (`base_land_speed_feet()`
exists). Same disease `stale-deferral-sweep.md` found in the DR family.

## 6. Leverage

```
[A] IPC boundary carries explanations   -> unlocks 636+ records, 27 classes, at once
     +-- [B] missing static totals (§3)  -- arithmetic, no new data
     +-- [C] weapon line on the sheet    -- data complete in engine today
     +-- [D] delete frontend rules duplication
     +-- [E] per-spell dice tables       -- corpus authoring; largest, lowest leverage
```

**[A] is the smallest slice with the largest unlock.** Nothing in the dynamic
column unlocks anything but itself.

**This defect has been diagnosed and one-off-fixed four times already** — Feats
tab (raw ids), Spells tab (441 spells invisible), AC-by-source panel, Pets tab
(companion computed at all 20 levels, no field to travel in). [A] is the
structural fix.

Supporting evidence that the sheet is the weak point: the Weapons tab
unconditionally prints *"No weapons added yet."* with no row-rendering path,
while `damage_total.rs` returns a full corpus-cited per-weapon breakdown. The
Actions tab renders a hand-authored `CLASS_FEATURES` table with bare labels and
no magnitudes against 411 cited `class_feature.*` records. Spells/day is a
hardcoded Wizard-only table. `maxHitPoints()` is computed in TypeScript. In the
engine: **332 occurrences of "standalone" against 5 of "INTEGRATED into."**

## 7. Recommendation

Build zero execution engines. Reclassify the category from architecture backlog
to **closed question**, per the precedent `opponent-interaction-architecture-design.md`
already set for Deflect Arrows.

Start **Receipt-to-Sheet** instead. Thesis: *the engine's job is to produce
every number a player needs in order to roll; the sheet's job is to show all of
them; nothing rolls.*

### Slice 1 — the explanation channel and the weapon line

1. Carry `explanations: Vec<ExplanationDto { id, value, detail }>` across IPC.
   **`detail` verbatim** — it is the engine's corpus-cited derivation; a rewrite
   would be a second unverified source of rules prose.
2. Render a Class Features section from `class_feature.*` / `class_chassis.*`.
   **Delete `CLASS_FEATURES` and `WIZARD_SPELLS_PER_DAY`** from
   `characterProgression.ts` — hand-authored rules data in the frontend is the
   debt the no-stub doctrine forbids.
3. Ship the weapon line: add `weaponDamage` DTO, render rows. Present the
   breakdown as separate columns — **do not invent a summed damage total**;
   `contract.rs`'s existing boundary note stands.
4. Ground `class_chassis.<class>.caster_level` for every full caster.
5. Resweep the four stale phrases in §5.

**Acceptance:** a level-11 Rogue's sheet shows **Sneak Attack 6d6**; a level-10
Wizard shows **Caster Level 10** and per-level spells/day; an equipped longsword
produces a populated Weapons row; `grep CLASS_FEATURES\|WIZARD_SPELLS_PER_DAY
apps/desktop/src` returns zero. No RNG dependency, no opponent field, no turn state.

**Non-goals:** dice rolling, attack/save/damage resolution, opponent modelling,
turn clock, per-spell dice tables.

### Follow-on

- **Slice 2** — the missing static totals (§3); move `maxHitPoints` into the engine.
- **Slice 3** — promote `DiceExpression` to a first-class receipt value; migrate
  the 30 split/prose dice records onto it.
- **Slice 4, only on explicit request** — per-spell dice tables (121–347 records,
  hand-authored, 2–3-source bar). Recommend deferring: with caster level on the
  sheet, the already-rendered description text is a complete answer for a paper sheet.
