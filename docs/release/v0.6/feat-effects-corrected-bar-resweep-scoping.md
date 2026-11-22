# Feat-effects re-sweep under the corrected standalone-grounding bar

**Author:** featmate (v0.6 alpha swarm) · **Date:** 2026-07-26 · **Status:** scoping, no code written

## Why this pass exists

The standalone-grounding bar changed mid-session: a feature grounds on a real,
verified magnitude alone — a live consumer is a bonus, not a requirement. Several
feats were deferred under the older, stricter "must feed a computed total" bar and
were never re-examined. This pass re-reads all 185 CRB feat records end to end and
re-classifies them under the corrected bar plus the lead's Studied-Combat ruling
(always-on self-buffs ground; anything conditioned on an opponent/ally interaction
the engine cannot evaluate stays deferred).

## Method

Read all four catalog files in full (`feat_data/{general,combat,metamagic,item_creation}.rs`),
then re-verified every candidate magnitude against the **raw PCGen corpus**
(`~/workspace/repos/pcgen/.../core_rulebook/cr_feats.lst`), not this repo's
transcription — reading both the `BONUS:` token and the authoritative `BENEFIT:`
prose. That choice paid for itself twice (see *Findings that changed a verdict*).

## Denominators

| Table | Records | `effect: Some` | `effect: None` |
|---|---|---|---|
| General | 50 | 30 | 20 |
| Combat | 110 | 42 | 68 |
| Metamagic | 17 | 9 | 8 |
| ItemCreation | 8 | 0 | 8 |
| **Total** | **185** | **81** | **104** |

104 of 185 records carry no numeric token at all — those are a hard floor on what a
numeric engine can ever ground, not a backlog.

**Grounded today via the general catalog path** (`feat_effects.rs`, keyed on the real
catalog `key`): 16 feats — Toughness, Great Fortitude, Iron Will, Lightning Reflexes,
Athletic, Persuasive, Intimidating Prowess, Acrobatic, Alertness, Animal Affinity,
Deceitful, Deft Hands, Magical Aptitude, Self-Sufficient, Stealthy, Skill Focus.

**Structural finding — a second, separate grounding path exists.** Six more feats are
grounded *only* through Monk's bonus-feat choice slot, keyed on synthetic
`feat:<slug>` ids, gated on single-class Monk: Dodge, Catch Off-Guard, Throw Anything,
Combat Reflexes, Scorpion Style, Improved Grapple. A Fighter who picks Combat Reflexes
from the real catalog gets nothing; a Monk who picks it as a bonus feat gets a real
grounded record. That asymmetry is not a bug in either path, but it means the honest
tally is "16 via the catalog, 6 more Monk-only," and several Category A entries below
would close that gap as a side effect.

---

## Category A — real wins, buildable now, no new engine (17 feats)

All are flat or purely self-derived magnitudes, corpus-verified, no chooser, no
opponent dependency. Every one is a `feat_effects.rs` producer in my lane.

| Feat | Corpus token | Magnitude | Precedent |
|---|---|---|---|
| **Improved Initiative** | `BONUS:COMBAT\|INITIATIVE\|4` | flat +4 initiative | `inquisitor_cunning_initiative_bonus` already grounds an initiative bonus standalone |
| **Endurance** | `BONUS:VAR\|Feat_Endurance_SaveBonus\|4\|TYPE=Base` | flat +4 on a named list of environmental checks/saves | Investigator/Alchemist Poison Resistance (+2 vs one hazard type) |
| **Nimble Moves** | `BONUS:VAR\|Feat_NimbleMoves_Squares\|5` | move through 5 ft difficult terrain/round | — |
| **Acrobatic Steps** | `BONUS:VAR\|Feat_NimbleMoves_Squares\|15` | 15 ft, **explicitly stacking** with Nimble Moves to 20 ft total (stated verbatim in `BENEFIT:`) | — |
| **Fleet** | `BONUS:MOVEADD\|TYPE.Walk\|5\|PREVARLT:ENCUMBERANCE...` | +5 ft base speed in light/no armor, unencumbered | self-equipment condition only |
| **Improved Bull Rush / Disarm / Overrun / Sunder / Trip** (5) | `BONUS:VAR\|CMB_X,CMD_X\|2` | +2 offense **and** +2 defense per maneuver | `MONK_IMPROVED_GRAPPLE_BONUS = 2` (identical token) **and** `DWARF_STABILITY_CMD_BONUS` |
| **Greater Bull Rush / Disarm / Grapple / Overrun / Sunder / Trip** (6) | `BONUS:VAR\|CMB_X\|2` | +2 offense only; `BENEFIT:` explicitly says it stacks with the Improved version | same two precedents |
| **Stunning Fist** | `BONUS:VAR\|StunningFistDC\|10+(TL/2)+WIS` + `StunningFistAttack\|MonkLVL+floor((TL-MonkLVL)/4)` | save DC and uses/day | see below |

**Improved Grapple** is a 12th maneuver feat — already grounded on the Monk path, so
building the family would give it a general-catalog grounding too, at no extra cost.

**Stunning Fist deserves a specific call-out.** Its DC formula `10+(TL/2)+WIS` is
**byte-identical** to `monk_scorpion_style_dc` (`pilot_compute.rs:20098`), which is
already grounded. Monk's slice deferred Stunning Fist citing "no DC/save engine" — that
objection is now **stale under the corrected bar**, since Scorpion Style's DC was
grounded despite the same missing engine. This is a real inconsistency in the current
state, not a new judgment call.

**Why the maneuver family passes the Studied-Combat ruling.** The `CMB_X` half is a
bonus to *your own* check, not conditioned on any opponent property. The `CMD_X` half
is conditioned on the opponent's *action type* ("whenever an opponent tries to trip
you") — but that is a static defensive property of the character, exactly what Dwarf
Stability already grounds. Neither half requires evaluating an opponent's state.

## Category B — real magnitude **with a live consumer already computed** (4 feats)

Highest value in the sweep: these don't just record a fact, they change a number the
engine already computes and, in one case, already *enforces*.

| Feat | Corpus token | Live consumer (verified in code) |
|---|---|---|
| **Extra Rage** | `BONUS:VAR\|RageDuration\|6` | `barbarian_rage_rounds_per_day` — a genuinely enforced per-day budget (over-budget claim-blocks) |
| **Extra Performance** | `BONUS:VAR\|BardicPerformanceDuration\|6` | `bard_bardic_performance_rounds_per_day` / `class_chassis.bard.bardic_performance_rounds_per_day` |
| **Extra Ki** | `BONUS:VAR\|KiPoints\|2` | `class_chassis.monk.ki_pool_size` (`level/2 + WIS`) |
| **Extra Lay On Hands** | `BONUS:VAR\|LayOnHandsTimes\|2` | `class_chassis.paladin.lay_on_hands_uses_per_day` (`level/2 + CHA`) |

All four corpus tokens match their `BENEFIT:` prose exactly (6/6/2/2) — verified
individually, not assumed from the pattern.

**Two design constraints these carry, both real:**

1. **All four are `STACK:YES MULT:YES`** — genuinely repeatable feats. The producer must
   *count occurrences* in `selected_feats`, not test presence. Every existing producer in
   `feat_effects.rs` uses `.any(...)`; this family is the first that cannot.
2. **All four carry a `PREABILITY:` prereq** requiring the underlying class feature
   (`TYPE.Rage`, `TYPE.Bardic Performance`, `TYPE.Ki Pool`, `TYPE.Lay on Hands`). Skipping
   that check would let a Fighter with "Extra Rage" in `selected_feats` claim rage rounds —
   the identical false-grounding shape the adversarial review caught for the Rage
   activation gate (finding #3). The ownership check is required, not optional.

The producers are mine; wiring them into the four budgets crosses into `pilot_compute.rs`
(backend's lane), same handoff shape as the last two slices.

## Category C — real, but needs the activation-state pattern (3 feats)

Self-only, activated trade-offs. `ClassAbilityActivation`/`ActiveState` is the existing
mechanism; Power Attack is already modeled this way, so these are pattern-following, not
new concepts — but the activation half is backend's lane.

- **Combat Expertise** — `CombatExpertiseModifier = floor(BAB/4)+1`, −attack / +AC dodge.
- **Deadly Aim** — `DeadlyAimModifier = floor(BAB/4)+1` penalty, `2*modifier` ranged damage.
- **Arcane Strike** — `min(1+ArcaneStrikeLVL/5,5)` damage, swift action.

## Category D — conditional on self *equipment* state (6 feats, moderate)

The engine does model equipment selections, so these are evaluable in principle, but each
needs a target dimension that may not exist yet.

- **Shield Focus**, **Greater Shield Focus** — +1 AC each while a shield is equipped,
  explicitly stacking. AC is computed (`compute_combat_baseline`), so these are the closest
  of this category to a real win.
- **Two-Weapon Defense** — +1 shield AC wielding two weapons (+2 when fighting defensively).
- **Improved / Greater Two-Weapon Fighting** — `BONUS:COMBAT|SECONDARYATTACKS|1`.
  Precedent: `monk_combat_reflexes_additional_attacks_of_opportunity` grounds an
  attack-count magnitude standalone.
- **Two-Weapon Fighting** — `TOHIT-PRIMARY|2` / `TOHIT-SECONDARY|6`. These are *penalty
  offsets* against the base two-weapon penalties, meaningless until that base is modeled.

## Category E — genuinely deferred, with the reason

- **Chooser / `%LIST` family (~12)** — Weapon Focus, Greater Weapon Focus, Weapon
  Specialization, Greater Weapon Specialization, Improved Critical, Penetrating Strike,
  Greater Penetrating Strike, Spell Focus, Greater Spell Focus, Master Craftsman, Exotic/
  Martial Weapon Proficiency, Rapid Reload. These belong to the ratified Focus-feat
  Mechanism B slice, not here.
- **No numeric magnitude at all (104 records)** — all 8 ItemCreation feats (crafting-rule
  prose, permanently out of scope for a numeric engine), 8 of the 9 real Metamagic feats,
  and the bulk of Combat feats whose benefit is an action-economy or positioning permission
  (Cleave, Whirlwind Attack, Spring Attack, Mobility, Vital Strike, Deflect Arrows,
  Snatch Arrows, the 10 Critical feats, …).
- **Opponent-dependent, per the Studied-Combat ruling** — Shatter Defenses, Improved/Greater
  Feint, Gorgon's Fist, Medusa's Wrath, Disruptive, Spellbreaker, Stand Still, Step Up, and
  every `… Critical` feat (all conditioned on landing a crit against an opponent).
- **Boolean, not a magnitude** — Improved Unarmed Strike (`UnarmedLethal|1|TYPE=Boolean`).
- **Ability-score substitutions, not additive magnitudes** — Agile Maneuvers
  (`CMB_STAT|DEX-STR`), Defensive Combat Training (`CMD_BAB|TL-BAB`), Weapon Finesse
  (`TOHIT.Finesseable|max(STR,DEX)-STR`). Each is computable but only meaningful against a
  CMB/CMD/attack base that isn't built. **Weapon Finesse ties directly to task #14**
  (Swashbuckler's feat-prereq-substitution mechanism) and should be sequenced with it.
- **Needs an absent subsystem** — Shield Master (equipment enhancement values), Double
  Slice, Far Shot, Point-Blank Shot, Manyshot (range/damage engines).
- **Extra Mercy** — `ABILITYPOOL|Mercy|1` is a chooser (which mercy), not a magnitude.
- **Leadership** — `LeadershipScore = TL + CHA` is a genuinely computable formula, but the
  feat's actual content is a cohort/follower subsystem. Borderline; the score alone would
  ground, the feat would remain substantially unmodeled. Flagging rather than deciding.

---

## Findings that changed a verdict

**1. A corpus token that does not match its own printed rule — Extra Channel.**
`feat_data/general.rs` transcribes `BONUS:ABILITYPOOL|Extra Channel|1`, which reads as a
magnitude of 1. The raw corpus `BENEFIT:` text says **"You can channel energy two
additional times per day."** The `1` is PCGen ability-pool bookkeeping, not the rule's
magnitude. Anything grounded from the transcribed token alone would have asserted a
specific, checkable, **wrong** number. Extra Channel therefore stays deferred (its real
magnitude lives only in prose, and no channel-uses/day dimension is built) — but the
general lesson is the load-bearing part: for this family, `BONUS:` alone is not sufficient
evidence. Every Category A/B magnitude above was cross-checked against `BENEFIT:` prose for
exactly this reason.

**2. A duplicate key in the shipped catalog — `Combat Expertise`.**
A `uniq -d` sweep over the `key:` field of all 185 records across all four tables returns
exactly one duplicate: **"Combat Expertise" appears twice** in `feat_data/combat.rs`
(lines 25 and 26), with different effect payloads. Root cause verified in the raw corpus:
`grep -c '^Combat Expertise\t' cr_feats.lst` returns **2** — a base record and a
`TYPE=Base` Monk-flurry-aware variant — and the generator emitted each as its own
`FeatTableEntry` under the same key. Consequence: any exact-match `key` lookup silently
takes the first and ignores the second; any "sum every effect for this key" logic would
double-count. **Not currently harmful** (nothing reads Combat Expertise today), but it is a
live landmine sitting directly under the Category C work, and it is a generator-level
defect — `feat_data/*.rs` is marked "do not hand-edit; regenerate if the corpus changes",
so the fix belongs to whoever owns the generator, not an inline patch.

## Recommended sequence

1. **Improved Initiative + Endurance** — the two cleanest, most obviously correct wins;
   Endurance also deepens Ranger immediately (Ranger already grants it at level 3 as a
   grant-only record with no magnitude).
2. **The 11-feat maneuver family** — one uniform table, double precedent, largest
   feat-count-per-effort ratio in the sweep.
3. **Stunning Fist** — closes the stale Monk deferral, reuses an already-shipped formula.
4. **Nimble Moves / Acrobatic Steps / Fleet** — small, self-contained movement facts.
5. **Category B (the four `Extra …` feats)** — highest value but genuinely more design
   (occurrence counting + class-ownership gating) and a cross-lane wiring handoff.

Steps 1–4 would take the general-catalog tally from **16 to 33 of 185**. Category B would
make it **37**, and would be the first feat effects to change an already-computed number
rather than record a standalone fact.

## Open questions for the lead

1. **Sequence** — concur with 1→5 above, or reorder? My read: the maneuver family is the
   single best value, but Improved Initiative + Endurance are worth landing first as a
   small proof that the corrected bar really does open this vein.
2. **Category B occurrence counting** — repeatable feats need `.filter(...).count()`, a
   real departure from every existing producer's `.any(...)`. Confirm you want the
   stacking semantics honored rather than capped at one, since `STACK:YES MULT:YES` is
   explicit in the corpus.
3. **Leadership** — ground the score (`TL + CHA`) alone and leave the cohort subsystem
   named-but-unproven, or defer the whole feat? I lean defer; grounding one number from a
   feat whose substance is a subsystem feels closer to overstating than to the
   Poison-Resistance precedent.
4. **The `Combat Expertise` duplicate** — worth routing to whoever owns the feat_data
   generator now, or leave it noted until Category C actually needs it?
