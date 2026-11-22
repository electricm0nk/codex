# Swashbuckler (#14) — Finesse Reuse + Deeds Scoping

> Task re-routed after the #11-15 stale-framing sweep found that Finesse's
> prerequisite-substitution half reuses Brawler's already-built Cunning
> idiom rather than needing a new mechanism.
>
> **Two corrections up front, one of them to my own sweep, and one
> load-bearing corpus bug that would silently produce a Swashbuckler with
> zero deeds at every level.**

Already grounded: Panache, Charmed Life, Nimble (`named_features_wired = 3`).

---

## Correction 1 (mine): Deeds are NOT a chooser

My sweep called Deeds "an independent narrowable chooser, same idiom as
Slayer Talents," and that wording propagated into the rewritten task
description. **It is wrong.** There is **no `BONUS:ABILITYPOOL` for
Swashbuckler deeds anywhere in the corpus.** Every deed is gated purely on a
level tier (`PREVARGTEQ:SwashbucklerDeedQualifyLVL,{1,3,7,11,15,19}`) and is
gained *automatically* at that tier — a swashbuckler gets all deeds of each
tier, unlike Rogue/Slayer Talents which really are selected.

I asserted the chooser framing from structural analogy without checking for
the pool token. This is the same failure mode as the Cavalier AC-penalty
overclaim — reasoning from a pattern instead of verifying the specific
claim.

**Net effect on scope: this is good news.** No chooser machinery, no
canonical-narrowing, and no explicit-choice/mechanism-B work is needed.
Each deed carrying a real magnitude simply grounds at its tier.

## Correction 2 (corpus): the deed gate variable is never set for a Swashbuckler

Searched every occurrence of `SwashbucklerDeedQualifyLVL` across the entire
PCGen tree. It is:

- `DEFINE:SwashbucklerDeedQualifyLVL|0` — defaults to 0
- read by all six deed tiers (`PREVARGTEQ:…,1/3/7/11/15/19`)
- set by **exactly one** token, in the whole tree:
  `BONUS:VAR|SwashbucklerDeedQualifyLVL|MagusLVL|TYPE=Base`

**It is never set from `SwashbucklerLVL`.** Only a Magus (via the archetype
that grants deeds) ever satisfies these gates. Taken literally, a pure
single-class Swashbuckler at *any* level has **zero deeds** — every tier
gate compares against 0.

This is a genuine incompleteness in PCGen's own ACG data, not a
misreading — the same category as Oracle's Interstellar Void, but
load-bearing rather than cosmetic, because a builder implementing the gate
faithfully would ship a Swashbuckler that correctly-per-corpus has no deeds
at all.

**The magnitudes are unaffected.** `SwashbucklerDeedsLVL` — the variable the
deed *formulas* actually use — **is** properly set
(`BONUS:VAR|SwashbucklerDeedsLVL|SwashbucklerLVL|TYPE=Base`). Only the
*gating* variable is broken.

**Recommended workaround:** gate the deed tiers on `SwashbucklerLVL` (or
`SwashbucklerDeedsLVL`) directly, with a comment recording that the corpus's
own `SwashbucklerDeedQualifyLVL` is unset for this class and why the
substitution is correct. RAW tier levels 1/3/7/11/15/19 are independently
confirmed by the gate constants themselves.

---

## Real missed wins

### Swashbuckler Finesse — reuse at the variable level, not the function level

`BONUS:VAR|CombatFeatIntRequirement|max(CHASCORE,INTSCORE)|TYPE=Base`

**Same target variable and same idiom** as Brawler's Cunning
(`max(13,INTSCORE)`), which is already built and wired as
`brawler_cunning_effective_intelligence_score` (`pilot_compute.rs:14344`,
consumed at `:14431`).

**Honest bound — the operands differ.** Brawler's floors INT at a constant
13; Swashbuckler's takes the max of two *ability scores* (CHA and INT). The
existing function takes a single score and hardcodes the 13. So this is a
genuine conceptual and variable-level reuse — the mechanism, the target
variable, and the consumer are all already there — but the function needs
generalizing to take the second operand rather than being callable as-is. I
am naming that explicitly rather than repeating my Cavalier mistake of
calling something a drop-in when only the shape matched.

### Swashbuckler Weapon Training — flat, self-scoped

`BONUS:VAR|SwashbucklerWeaponTrainingBonus|(SwashbucklerWeaponTrainingLVL-1)/4`
→ +1 at 5th, +2 at 9th, +3 at 13th, +4 at 17th.

### Bonus Feats — the Martial Training shape, already ruled

`BONUS:VAR|FighterWeaponQualifyLVL|SwashbucklerLVL` — swashbuckler levels
count as fighter levels for feat qualification. Identical in kind to
Brawler's Martial Training, which you just ruled: **ground the fact, defer
the `feat_prereqs` wiring.** Same treatment applies here without needing a
fresh ruling.

### Six deeds carry real magnitudes (of 21)

| deed | tier | magnitude |
|---|---|---|
| Derring-Do | 1 | `DerringDoTimes = DEX` |
| Dodging Panache | 1 | `DodgingPanacheBonus = CHA` (dodge AC) |
| Precise Strike | 3 | `= SwashbucklerDeedsLVL`, double `= 2*LVL` (damage) |
| Bleeding Wound | 11 | `BleedingWoundDamage = DEX` (bleed) |
| Deadly Stab | 19 | DC `= LVL/2 + 10 + DEX` |
| Stunning Stab | 19 | DC `= LVL/2 + 10 + DEX` |

Derring-Do, Dodging Panache and Precise Strike are the ones that matter for
a low-level build (tiers 1 and 3). Precise Strike is the Bomb/Sacred-Weapon
weapon-damage idiom; Dodging Panache is a flat self-buff whose magnitude
needs nothing about the attacker, so it clears the bar under the same
reasoning as Oracle's Deaf.

## Correctly blocked

- **15 of the 21 deeds carry zero `BONUS`/`DEFINE` tokens of any kind** —
  Opportune Parry and Riposte, Kip-Up, Menacing Swordplay, Swashbuckler
  Initiative, Swashbuckler's Grace, Superior Feint, Targeted Strike, Subtle
  Blade, Evasive, Dizzying Defense, Perfect Thrust, Swashbuckler's Edge,
  Cheat Death, plus Swashbuckler Weapon Mastery. Genuine no-ops in the
  Nature Training sense.
- **Finesse's other half** — "gains the benefits of the Weapon Finesse feat
  with light or one-handed piercing melee weapons" has no token and needs
  real attack mechanics. Stays deferred, as the task description already says.

## Build-time hazards

1. **The `SwashbucklerDeedQualifyLVL` gate bug** (above) — the one that
   would silently produce zero deeds. Substitute `SwashbucklerLVL`.
2. **`KEY:Deed ~ …` is a *different* class's record set.** There are 16 such
   records (Sleuth's Initiative, Clandestine Expertise, Silence Is Golden,
   Perfect Throw, Blind Shot, Targeted Throw, …) belonging to
   Investigator-Sleuth and firearm archetypes. **Swashbuckler's own deeds
   are keyed `Swashbuckler ~ <Name>`, not `Deed ~ <Name>`.** Searching the
   corpus for "Deed" finds the wrong set entirely — same trap family as
   Oracle's `Winds Mystery` / `Wind Mystery ~ …` key mismatch.
3. **`SwashbucklerDeedsLVL` is also fed from `CavalierLVL` and `MonkLVL`**
   (archetype cross-grants). Harmless for a single-class Swashbuckler, but
   do not assume the variable equals swashbuckler level unconditionally.
4. **Deadly Stab and Stunning Stab share a byte-identical DC formula** —
   two records, one mechanism, relevant to the `named_features_wired`
   count.
5. **The Finesse function needs a second operand** (hazard restated because
   it is easy to mis-plan as a zero-cost call).

## Recommended bounded MVP

Finesse's prereq substitution (generalize the Brawler function), Weapon
Training, Bonus Feats' level-equivalence fact, and the three low-tier deeds
with real magnitudes (Derring-Do, Dodging Panache, Precise Strike) — with
the two tier-19 DCs and Bleeding Wound as optional adds since they are the
same formulas at higher gates.

**Honest status expectation:** stays Blocked — 15 tokenless deeds and
Finesse's Weapon-Finesse half keep the `other_features_deferred` diagnostic
alive. `named_features_wired` 3 → roughly 8-9, subject to your
cluster-collapsing call on the two identical stab DCs.

## Open question for the lead

**Is the `SwashbucklerDeedQualifyLVL` substitution acceptable, or does it
cross the fabrication line?** My read is that it is clearly on the right
side: the tier constants 1/3/7/11/15/19 are read directly from the corpus's
own gates, and substituting the correct level source for a variable the
corpus simply forgot to populate is a transcription fix, not an invention.
But it *is* the first time this segment we would knowingly deviate from what
the corpus literally says, so it should be your call rather than my
assumption — and it wants a code comment recording the reasoning either way.
