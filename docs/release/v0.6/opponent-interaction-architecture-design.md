# Opponent-Interaction Architecture — Design Pass (#13 Slayer, #15 Monk)

> Assigned together as "the two genuine architecture gaps," under the
> operator's reversed policy that blockers are top priority regardless of
> cost (risks item 51).
>
> **They are not one problem, and the answers diverge sharply.** Slayer's
> pillar needs *far less* architecture than assumed — the largest part of it
> needs none at all, and this codebase's own dominant idiom already covers
> it. Monk's Deflect Arrows needs *much more*, and would still ground
> nothing, because no numeric magnitude for it exists anywhere in the
> corpus.

---

## The structural split

| | Slayer / Investigator / Cavalier | Monk's Deflect Arrows |
|---|---|---|
| what's missing | the identity of a **persistent declared target** | an **incoming-attack event** |
| shape | a noun — durable state between actions | an event — per-attack, reactive |
| magnitude in corpus | real, flat, level-derived | **none anywhere in the tree** |
| would Tier 2 opponent entities deliver it? | yes, but they aren't needed | **no** — needs attack resolution |

Grouping them as "opponent interaction" is what made both look equally
expensive. They share almost nothing.

---

# Part 1 — The Studied-Target pillar (#13)

## The central finding: no magnitude depends on the opponent

Every formula, re-derived from the corpus:

| feature | magnitude | opponent-dependent? |
|---|---|---|
| Slayer Studied Target | `SlayerStudiedTargetBonus = SlayerLVL/5+1` → +1/+2/+3/+4/+5 at 1/5/10/15/20 | **no — level only** |
| …simultaneous targets | `(SlayerLVL>0)+(SlayerLVL>6)` → 1, then 2 at 7th | no |
| Investigator Studied Combat | `InvestigatorLVL/2`; duration `max(1,INT)` rounds | no |
| Investigator Studied Strike | `min(9,(InvestigatorLVL-2)/2)` d6 | no |
| Cavalier Challenge | `+CavalierLVL` damage; `(LVL+2)/3` uses/day; `-2` AC | no |

**Not one of these reads any property of the opponent** — not its AC, HD,
type, or state. The opponent is a *scope condition*, not an input to the
computation. So "opponent tracking" in the sense of modelling creatures is
not required to compute any of these numbers. That reframes the whole task.

## The precedent that settles it: Slayer's own Sneak Attack

This codebase **already grounds opponent-relationship-conditional
magnitudes** as standalone records:

- `class_feature.acg.slayer.sneak_attack_dice` — sneak attack applies only
  when the target is flanked or denied its Dexterity bonus, an opponent
  relationship modelled nowhere here. It is grounded, with the detail text
  stating plainly: *"This codebase computes no sneak-attack-damage total to
  layer this onto; the flat dice count is grounded as a standalone record
  only."*
- `class_feature.acg.slayer.trap_sense_bonus` and `…trapfinding` — both
  conditional on circumstances (facing a trap; locating traps) the engine
  models nowhere. Both grounded, and Barbarian's and Rogue's Trap Sense
  before them.

**So Slayer's own class already contains both treatments of the same
shape:** Sneak Attack grounded, Studied Target deferred. That inconsistency
is the real finding here — the deferral wasn't a considered architectural
boundary, it was the same category of feature landing on the other side of a
line drawn before the standalone-grounding bar was corrected.

## Three tiers

### Tier 0 — ground the magnitudes as scoped standalone records. **No new architecture.**

Emit each magnitude as its own `ComputationExplanation`, never folded into
any total, with the scope named explicitly in the detail text — exactly as
Sneak Attack already does. A character sheet legitimately prints "Studied
Target: +2 attack and damage against your studied target"; that is a
complete, verified, non-fabricated fact.

- **Cost:** an ordinary closure. No type changes, no new state.
- **Unblocks:** Slayer's Studied Target (+ target count), Investigator's
  Studied Combat and Studied Strike, Cavalier's Challenge damage, and
  Investigator's Studied Defense talent — **four classes, one pass.**
- **Risk:** wording discipline only. The record must name the scope and
  state that it is in no total. Silence on scope is what would overstate.

### Tier 1 — a declared snapshot scope, if the numbers should reach totals

Reuse the **existing** `ClassAbilityActivation` primitive
(`ability_id` / `active_state` / `rounds_consumed_today`) with a new id such
as `studied_target` — the same mechanism Rage and Bloodrage already use
(`active_barbarian_rage_bonus`, `pilot_compute.rs:21505`). No new type is
required.

**But there is a genuine epistemic difference from Rage, and it is the
reason this tier needs more than a flag.** Rage's activation *fully
determines* the modifier: while raging, every attack gets it. Studied
Target's does not — in the same round, attacks against the studied creature
get the bonus and attacks against anything else do not. So folding it into a
computed attack total asserts something Rage's fold does not.

The honest form is therefore a **declared snapshot scope**, not just an
activation: the sheet states "computed against your studied target," the way
a VTT presents a toggle. Under that stated scope the totals are true.
Without it, they silently over-claim.

- **Cost:** small — reuses an existing primitive; the real work is the
  scope marker and presenting it honestly.
- **Prerequisite:** Tier 0, which stands alone regardless.

### Tier 2 — real opponent entities and per-attack targeting

Model creatures with identity so the engine can verify *which* creature is
studied. **Not justified by this evidence:** none of the four features'
magnitudes need any opponent property, so Tier 2 buys only verification of a
condition the player declares anyway. It is justified by a decision to build
real combat resolution — a product question far larger than these features.

## Recommendation

**Tier 0 now** — it is a normal closure, it unblocks four classes, and it
requires no architecture at all. **Tier 1 as a follow-on** if the operator
wants those numbers inside computed totals. **Tier 2 not on this evidence.**

Note this makes #13's premise ("design opponent-tracking pillar for Studied
Target, *then* ground Slayer Talents") wrong twice over: Slayer Talents was
already decoupled in the last sweep, and the pillar itself turns out not to
gate Studied Target either.

## Build-time hazards

1. **Cavalier's Challenge `-2` AC penalty has the *inverse* scope.** It
   applies against *everyone except* the challenge target. A Tier-1
   "computed against your studied/challenged target" scope would get this
   exactly backwards if applied uniformly. The two scopes must be
   distinguished, not merged.
2. **Investigator's Studied Defense redirects the same bonus to AC** instead
   of attack, at the player's choice — one magnitude, two destinations. That
   is mechanism-B (explicit recorded choice) territory, not a canonical
   default, by the same reasoning ratified for Skill Focus.
3. **Studied Combat's duration is `max(1,INT)` *rounds*.** This engine has
   no round clock, so unlike a per-day budget the duration can be grounded
   as a fact but **cannot be enforced**. Do not build a budget check that
   silently does nothing — name it as unenforced, the way Bloodrage's
   post-rage fatigue already is.
4. **Slayer maintains 2 simultaneous studied targets from 7th level.** At
   Tier 1 the scope is therefore not a single boolean at higher levels.
   Tier 0 sidesteps this entirely.
5. **Studied Target's bonus also covers Bluff, Knowledge, Sense Motive,
   Perception and Survival against that opponent** — not just attack and
   damage. Intimidate is not among them, so this does not collide with the
   three computed selected-skill modifiers.

---

# Part 2 — Deflect Arrows (#15)

## There is no magnitude, anywhere

I searched every `Deflect Arrows` record across the entire PCGen tree, not
just CRB:

- The CRB feat carries **zero `BONUS` tokens** — only `TYPE:Combat`, its two
  prerequisites (Improved Unarmed Strike; Dex 13), and DESC text.
- Every other record (`Zen Monk ~ Deflect Arrows`, `Unchained Monk Bonus
  Feat ~ …`, `Guard Dedication ~ …`) is a feat *grant* or a selection flag.
- The only variable bearing the name anywhere is
  `MonkBonusFeat_DeflectArrows` — a bonus-feat-selection flag, not a
  quantity.

The whole mechanic ("once per round, negate one ranged attack that hits
you") exists only as RAW prose. There is no corpus token to derive.

## What the architecture would actually cost

Deflect Arrows is reactive to an **incoming attack event**, which is a
different thing from target identity. Tier 2 opponent entities would not
deliver it. It needs an attack-resolution pipeline: attack rolls against the
PC, AC comparison, ranged-vs-melee classification, and a per-round action
economy to enforce "once per round." That is the largest single piece of
machinery proposed anywhere in this segment.

## Recommendation: keep it deferred — and the policy reversal does not reach it

This needs stating precisely, because the operator's reversal was aimed at
exactly this kind of row. **The reversal removed *cost* as a reason to
defer. Deflect Arrows does not fail on cost — it fails because there is no
fact to compute.** Building the entire pipeline would ground no number,
because none exists in the corpus.

That is the same test already applied consistently and repeatedly this
segment: Nature Training (zero `BONUS` token → correctly deferred, not a
missed win), Brawler's Awesome Blow, and the 34 of 100 Oracle revelations
with no numeric token. Deflect Arrows is that category, and the only one of
the roster's "architecture blockers" that is.

**What would justify the attack-event pipeline** is a product decision to do
real combat resolution — iterative attacks, AC-vs-attack, action economy.
That is worth its own question, and Deflect Arrows would come along free
with it. It is not worth building for one tokenless feat.

---

## Open questions for the lead

1. **Tier 0 or Tier 1 for the Slayer pillar?** I recommend Tier 0 now — it
   is a normal-sized closure unblocking four classes with no architecture —
   and Tier 1 only if you want the numbers in computed totals.
2. **Tier 0 reverses your earlier Studied Target ruling, and should be
   explicit about it.** That ruling was right under the framing available at
   the time; what changed is the Sneak Attack precedent being noticed in the
   *same class*. Worth recording as a deliberate correction rather than a
   quiet drift, so the line stays legible: *conditional on an unmodelled
   circumstance* grounds (Sneak Attack, Trap Sense, Studied Target);
   *needing an unmodelled quantity* does not.
3. **Confirm Deflect Arrows stays deferred on "no magnitude" grounds
   rather than cost** — so the row stops reading as an architecture backlog
   item and starts reading as a closed question, with the attack-resolution
   pipeline tracked separately as the product decision it really is.
4. Should the Familiar subsystem (#11/#12), queued behind these, get the
   same three-tier treatment? On a first look it is a genuine Tier-2-shaped
   problem — a familiar has its own stat block, so unlike a studied target
   its properties really are inputs — but the Eidolon pass already
   established the bounded-companion shape it would follow.
