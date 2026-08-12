# Rogue Talents (#57) — Canonical-Narrowing Scoping

> Third shared chooser family. Scoped with the brief's standing instruction:
> **treat "no legality constraint" as something to disprove, not assume** —
> the pattern has now held three times running.
>
> **Recommendation: Resiliency.** It is the only tokened talent that is
> simultaneously in both classes' lists, ungated, formula-simple, and landing
> on a computed total.

## The constraint is real — third time in a row

**Investigator does not get all 67 Rogue Talents.** It carries its own
explicit whitelist of **40** `KEY:Investigator ~ Rogue Talent ~ …` records.
A canonical pick outside that list would silently collapse the two-class
leverage that motivates this task — exactly the failure Skald's rage-power
restriction and Arcanist's Reservoir cost would each have caused.

**Correction to my own earlier work:** the Investigator scoping doc (#45)
said "14 `Rogue Talent ~ X` records". The real whitelist is **40**. The 14
was an undercount on my part; the figure to use is 40.

## Corpus inventory

- **67 active `Rogue Talent ~` records** (CRB 25, ACG/APG 42).
- **Only 10 carry a numeric token** — the family is overwhelmingly
  non-numeric (Fast Stealth, Ledge Walker, Rogue Crawl and the like grant
  abilities, not magnitudes). Low yield relative to Rage Powers' 18-of-60.

The ten tokened talents, and why most are unavailable:

| talent | token | verdict |
|---|---|---|
| **Resiliency** | `ResiliencyHitPoints = RogueTalentLVL` | **recommended** |
| Trap Spotter | `TrapSpotterDistance = 10` | runner-up |
| Minor Magic | `RogueCasterLevel = RogueTalentLVL` | **blocked** — `PRESTAT:1,INT=10` gate + spell-like machinery |
| Combat Trick | `ABILITYPOOL\|Combat Trick Feat\|1` | bonus-feat count, no consumer |
| Weapon Training | `ABILITYPOOL\|Weapon Training\|1` | bonus-feat count, no consumer |
| Advanced Talent / Regular Talent | `ABILITYPOOL\|…\|1` | **meta** — grant another talent slot, chooser-in-chooser |
| Dispelling Attack, Slippery Mind | — | **Advanced Talents**, gated behind the Advanced Talent slot |
| Guileful Polyglot | language pool | languages, no computed consumer |

## Recommended canonical: **Resiliency**

```
BONUS:VAR|ResiliencyHitPoints|RogueTalentLVL
PRE: (none)
DESC: "Once per day, you can gain N temporary hit points. Activating this
       ability is an immediate action that can only be performed when you
       are brought to below 0 hit points."
```

Why it wins:

1. **In both classes' lists** — present in Investigator's 40-talent
   whitelist, so the two-class leverage genuinely holds.
2. **No `PRE` gate at all** — available from the first talent slot.
3. **Simplest formula available** — `= RogueTalentLVL`, no arithmetic, so no
   divisor/offset transcription risk.
4. **Lands on a computed total** — temporary hit points are computed in
   `durability.rs` (max/current/temp HP all exist). This is a real
   integration, not another standalone record.

The once-per-day limit is a genuine budget of the kind this codebase models
routinely, and the "when brought below 0 hit points" trigger is an
unmodelled circumstance — so under the corrected bar the **magnitude**
grounds as a scoped fact while the trigger is named and not simulated. Same
treatment as every activation-gated pool this session.

## Runner-up: **Trap Spotter**

`TrapSpotterDistance = 10` — flat, ungated, and also on Investigator's
whitelist. Ranked second purely because it has **no computed consumer**
(Perception is not a computed total here), so it grounds standalone. Simpler
than Resiliency, worth less.

## Honest assessment of this family's value

**This is the weakest of the three chooser families surveyed**, and the
numbers say so plainly:

| family | records | tokened | classes | best pick lands on |
|---|---|---|---|---|
| Rage Powers (#54) | 60 | 18 | 2 | computed skill ×2 |
| Arcanist Exploits (#56) | 46 | 21 | 1 | computed max HP |
| **Rogue Talents** | **67** | **10** | **2** | computed temp HP |

Most records, fewest magnitudes. Resiliency is a genuinely good pick — but
the family behind it is thin, and after this one there is very little left
in it worth grounding.

## Explicitly deferred

The 57 tokenless talents (Nature Training family); the two Advanced Talents
(behind a slot-grant); Minor/Major Magic (INT prereq plus spell-like
machinery); the bonus-feat and meta-slot pools; and the 27 ACG/APG talents
outside Investigator's whitelist, which are Rogue-only and so carry no
shared leverage.

## Open question

**Is this family worth building at all right now?** Resiliency is a clean
pick and the two-class credit is real, but the family is thinner than either
predecessor and Rogue is already at 13 grounded features. If #54 and #56 are
still in flight, I would let those land first and reassess — the marginal
value here is lower than either, and I would rather say so than present
three equally-weighted options.
