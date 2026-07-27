# Rage Powers (#53) — Canonical-Narrowing Scoping

> Shared chooser family: 60 active records, zero magnitudes grounded. Scoped
> canonical-narrowing style — ground one representative power's real
> magnitude, name and defer the other 59.
>
> **Recommendation: Raging Climber.** Simplest formula on the roster
> (`= RagePowersLVL`, no arithmetic), purely passive, lands on a **computed
> total**, and its twin makes a second power nearly free.

## Corpus inventory

**60 active records** (60 raw, 60 active — no `#`-disabled inflation).
**18 carry a numeric token; 42 are tokenless** and correctly deferrable as
the Nature Training family.

**The level variable is fed by three classes, not two:**

```
BONUS:VAR|RagePowersLVL|BarbarianLVL
BONUS:VAR|RagePowersLVL|SkaldLVL
BONUS:VAR|RagePowersLVL|BloodragerLVL
```

My survey reported two classes because only **Barbarian** (`RagePowersLVL/2`)
and **Skald** (`RagePowersLVL/3`) carry verified `BONUS:ABILITYPOOL|Rage
Power` grants. Bloodrager feeds the level var without a confirmed pool grant
— likely for prerequisite resolution or an archetype path. **Stated
precisely: three classes set the level, two have confirmed grants.** Worth
checking Bloodrager's path before claiming three-class credit.

## The selection constraint the brief didn't mention

**Skald cannot take every rage power**, and its own DESC says so:

> "This cannot be a rage power that requires the creature to spend a standard
> action or rounds of rage to activate it. For example, the skald cannot
> choose terrifying howl (which requires a standard action)…"

So a canonical pick that isn't Skald-legal collapses the two-class leverage
that motivated this task. **Terrifying Howl is corpus-confirmed excluded**
(its own DESC: "You unleash a terrifying howl as a standard action"), and it
would otherwise have looked attractive — it has a clean flat DC
(`10 + RagePowersLVL/2 + STR`).

This rules out the activation-cost powers: Terrifying Howl and Renewed Vigor
(standard action), Powerful Blow / Surprise Accuracy / Strength Surge
(once-per-rage), and puts Guarded Stance / Rolling Dodge (move action) in a
gray zone.

## Recommended canonical: **Raging Climber**

```
BONUS:SKILL|Climb|RagingClimberBonus|PREVAREQ:Raging,1
BONUS:VAR|RagingClimberBonus|RagePowersLVL
DESC: "While raging you add a +N enhancement bonus to all Climb skill checks."
```

Why it wins on every criterion this session has used:

1. **Simplest formula on the roster** — `= RagePowersLVL`, no arithmetic, so
   no divisor/offset transcription risk (the Sacred Weapon and Raging Song
   failure mode).
2. **Purely passive** — "While raging you add…", no action cost at all, so
   unambiguously Skald-legal rather than gray-zone.
3. **Lands on a computed total** — Climb is one of the three computed
   selected-skill modifiers, so this is a real integration, not another
   standalone record. Rare for a chooser pick.
4. **The gate is already modelled for both classes** — Barbarian's `rage`
   activation and Skald's Inspired Rage both already exist as
   `class_ability_activations`, so `PREVAREQ:Raging,1` maps onto shipped
   infrastructure rather than needing new state.

**Bonus: Raging Swimmer is the identical twin** —
`BONUS:SKILL|Swim|RagingSwimmerBonus`, same `= RagePowersLVL` formula — and
**Swim is also one of the three computed skills.** Grounding Climb makes
Swimmer a near-zero-marginal-cost second power. Worth taking both unless you
want the narrowing to stay strictly single.

## Alternative if a defensive pick is preferred: **Superstition**

`SuperstitionSaveBonus = 2 + RagePowersLVL/4` → +2 at 1st, +3 at 4th, +4 at
8th, rising to +7 at 20th; a morale bonus on saves vs spells, supernatural
abilities and spell-like abilities. Also passive, also lands on a computed
total (saves), and **Skald's own DESC names superstition explicitly** as a
grantable power, so its legality is corpus-confirmed rather than inferred.

Ranked second only because it carries arithmetic and a narrower situational
scope than Climb's unconditional-while-raging bonus.

## The other 16 tokened powers — named, deferred

Increased Damage Reduction (belongs with the DR family, #39/#41/#46),
Guarded Stance, Rolling Dodge, Powerful Blow, Surprise Accuracy, Strength
Surge, Renewed Vigor, Swift Foot, Terrifying Howl, Raging Leaper, and the
five bloodline-gated Blood powers (Abyssal/Celestial/Draconic/Elemental ×2 —
these are sub-choosers gated on a Bloodrager bloodline, a chooser inside a
chooser).

**Swift Foot** deserves a note as the closest runner-up after Superstition:
`BONUS:MOVEADD|TYPE=Walk|5`, flat and passive — but movement has no computed
total, so it would ground standalone rather than integrating.

## Sequencing

Per the brief: **after or alongside #50**, not before — Skald's rage-power
*pool count* is in #50's scope, and two agents on Skald's rage-power surface
concurrently is the shared-checkout hazard this swarm has already hit twice.

## Honest expectation

One canonical power credits **Barbarian and Skald** (and possibly Bloodrager,
pending the pool-grant check above). Barbarian's largest remaining hole
closes; both classes stay Blocked on the other 59 named-and-deferred powers.

## Open question

**One or two powers?** Raging Climber alone is the clean single narrowing
matching every prior precedent. But Raging Swimmer is the same formula on a
sibling computed skill, so taking both costs almost nothing and doubles the
computed-total integration. I lean toward both; strict precedent says one.
