# Stale-Deferral Sweep — Shipped Deferrals Predating Item 52

> Triggered by featmate's Monk audit (High Jump / Wholeness of Body deferred
> under the old bar) and risks item 61. Swept `pilot_compute.rs` for
> deferral language and cross-checked each hit's *stated reasoning* against
> the corrected bar, rather than trusting the comment's own claim.
>
> **Result: one clean family of stale deferrals, three genuinely new
> members, and ~7 confirmed correctly-deferred. The family is damage
> reduction, and the codebase is currently inconsistent with itself about
> it — 3 grounded, 2 deferred, identical shape.**

## The test applied

Item 52's line: a magnitude conditional on an **unmodelled circumstance**
grounds as a scoped standalone fact; one that needs an **unmodelled quantity
belonging to something else** stays deferred. Deferrals justified by *"no X
engine/total exists to apply this to"* are consumer-absence reasoning —
exactly what the corrected bar overturned, **provided the feature has a real
magnitude of its own.**

## The inconsistency

The same reasoning currently produces opposite outcomes on the same shape:

| feature | corpus token | status |
|---|---|---|
| Barbarian ~ Damage Reduction | `DR:BarbarianDR/-` | **grounded** standalone |
| Skald ~ Damage Reduction | `DR:SkaldDamageReduction…` | **grounded** standalone |
| Fighter ~ Armor Mastery | `DR:5/-` | **grounded** (`ARMOR_MASTERY_DAMAGE_REDUCTION = 5`) |
| **Paladin ~ Aura of Righteousness** | **`DR:5/Evil`** | **deferred** — identity record only |
| **Monk ~ Perfect Self** | **`DR:10/Chaotic`** | **ungrounded entirely** (0 code hits) |

Skald's own grounded record states the rationale plainly — *"never applied
to any incoming-damage total, since no damage-resolution engine or
incoming-damage total exists anywhere in this codebase"* — while Paladin's
deferral cites that same absence as the reason **not** to ground. Three
sites accept it, two reject it.

## Ranked findings

### 1. Paladin — Aura of Righteousness `DR 5/evil` (level 17). Cheapest and clearest.

Deferred as a bounded grant-only identity record because *"no
damage-reduction-application engine and no compulsion-immunity-check engine
exists anywhere in this codebase to apply this to."* The DR half is a flat,
self-scoped, structured `DR:5/Evil` token with three in-codebase precedents.

**Correctly stays deferred:** the compulsion immunity (no magnitude) and the
*"each ally within 10 feet gains a +4 morale bonus"* clause (ally-scoped).
Only the DR is the missed win — a clean split, not a wholesale reversal.

### 2. Monk — Perfect Self `DR 10/chaotic` (level 20). Same shape, zero coverage.

`grep perfect_self` returns **0 hits** — not deferred-with-a-record, simply
absent. Same flat structured token. **Monk is already task #36**, so this
folds in naturally rather than needing its own dispatch.

### 3. Bloodrager — `DR:BloodragerDR/-`. Known-remaining, now clearly same family.

Zero code hits. Flagged in the earlier Bloodrager pass as a remaining
feature; this sweep places it in the same proven-groundable family rather
than leaving it as an unclassified leftover.

### 4. Bloodline / spirit DR sub-entries. Lower priority, chooser-shaped.

`Aberrant Form 5/-`, `Soul of the Fey 10/cold iron`, `Deep One 10/piercing`,
`Strength of Stone 10/adamantine`, Shaman's `Stone`/`Bones` spirit DRs.
Each is a sub-entry under an already-counted chooser slot, so each narrows
to one canonical pick under standing precedent — real, but one mechanism per
class, not per record.

## Confirmed correctly deferred — no action

These came back clean on inspection, and their comments' own reasoning holds:

- **Ranger Camouflage** — comment states it outright: *"no numeric magnitude
  of its own."*
- **Bard Soothing Performance** — *"functions as mass cure serious wounds"*;
  the healing belongs to the spell, not the feature.
- **Ranger Evasion / Monk Improved Evasion** — Evasion has no magnitude
  ("take no damage on a successful save"); +0 identity records are right.
- **Ranger Master Hunter**, **Druid Wild Shape / A Thousand Faces** —
  capstone/execution features with no independent number.
- **Barbarian Indomitable Will** — *already* grounded as a flat while-raging
  magnitude; comment is accurate.
- **Fighter Armor Mastery** — *already* grounded with a real constant.
- **Skald's DR ally-extension** — correctly deferred as ally-scoped while the
  self-DR is grounded; a model example of the split.

**Monk Fast Movement / Maneuver Training** are genuinely stale (deferred on
*"no speed-total engine and no CMB/CMD engine exist… to attach either to"*,
while the same file elsewhere grounds movement facts standalone on that
exact basis) — but they are already task #36, so they are noted, not
re-reported.

## Recommendation

Fold **#1 (Paladin)** into a small closure — it is one flat value with three
precedents and a clean deferred/grounded split. Fold **#2 (Monk Perfect
Self)** into the in-flight #36. Handle **#3 (Bloodrager)** whenever
Bloodrager is next touched. **#4** only if a bloodline/spirit chooser pass
happens anyway.

**Worth doing regardless of the individual wins:** the DR family should be
made *consistent*. Right now a reader of this codebase cannot tell from the
comments whether "no damage-resolution engine exists" means ground-it or
defer-it, because both answers are shipped. Whichever way each feature goes,
the five should agree.

## Open question

Only one, and it is a scoping call rather than a corpus question: **is the
DR-consistency cleanup one task or three?** I would do Paladin standalone
(it is the only one not already attached to an in-flight or future class
task), let Monk ride #36, and leave Bloodrager to its own class pass — but
bundling all three as a single "make DR consistent" closure is also
defensible and would leave the codebase self-consistent in one step rather
than three.
