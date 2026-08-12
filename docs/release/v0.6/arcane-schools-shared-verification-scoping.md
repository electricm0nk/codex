# Wizard Arcane Schools (#65) — Shared-vs-Parallel Verification + Scoping

> The open question from #62's triage, which flagged Schools "likely shared"
> without confirming it. Domains answered this one way, Bloodlines the other.
>
> **Verdict: genuinely shared, like Domains. Two base classes, one shared
> power namespace, both setter paths verified base-class.**
>
> **But my own #62 record counts were inflated by archetype content — the
> same error class as the Hunter/Paladin credit in #63.**

## Verdict: shared, and here is the actual chain

```
WizardLVL ───┐
             ├──→ ArcaneSchoolLVL ──→ <School>SchoolLVL ──→ <School> School ~ <Power>
ArcanistLvl ─┘      (shared spine)      (per-school)          (SHARED power records)
```

`Arcanist School ~ <SchoolName>` records are **wrappers**, not independent
content — each carries only `DEFINE:<School>SchoolLVL|0`, feeding the same
per-school variable Wizard's own power records read. So Arcanist does not
have a parallel school set the way Bloodrager has a parallel bloodline set;
it plugs into Wizard's.

**Both setter paths screened for archetypes and both are base-class:**

| setter | record | verdict |
|---|---|---|
| `ArcaneSchoolLVL \| WizardLVL` | `CATEGORY=Internal\|Arcane School Tr…` | **base** |
| `ArcaneSchoolLVL \| ArcanistLvl` | `Arcanist Exploit ~ School Understanding`, `TYPE:ArcanistExploit.SpecialQuality` | **base** |

I checked this *before* crediting rather than after — the #63 lesson applied
in the direction it was meant to work.

## Correction to my own #62 triage: 177/96 was inflated

The triage figure counted archetype and wrapper records as base-class
content. Screened properly:

| category | records | disposition |
|---|---|---|
| **base Wizard school powers** | **66** (58 magnitude-bearing) | **the real family** |
| Savant | 64 | **archetype** (`CATEGORY:Archetype`, `TYPE:Archetype.ArcanistArchetype…`) |
| Arcanist wrappers | 31 | plumbing — `DEFINE:<School>SchoolLVL` only |
| Focused Arcane School | 16 | archetype-shaped, excluded |

**The real base figure is 66 records / 58 magnitude-bearing**, not 177/96.
Same error I made crediting Hunter and Paladin for domains: counting
archetype content toward a base-class family. Two instances in one day says
this belongs in the triage method itself, not just in my attention.

## What the corrected numbers actually show — density, not size

| family | base magnitude-bearing | density | classes |
|---|---|---|---|
| Domain | 159 | ~54% | 3 (shared) |
| Bloodline | 149 | 61% | 1 per build (parallel) |
| **School** | **58** | **88%** | **2 (shared)** |
| Rage Powers | 18 | 30% | 2 (shared) |

Schools is **much smaller than #62 implied but the densest shared family on
the roster** — 29 schools × ~3 powers each, almost every record carrying a
magnitude, and a very regular shape (Abjuration, Conjuration, Enchantment,
Evocation, Illusion, Necromancy and Transmutation are all exactly 3-for-3).

That regularity is worth something: a generalization built for one school
transfers mechanically to the other 28.

## A sequencing coupling worth flagging

**Arcanist's access runs through `Arcanist Exploit ~ School Understanding`**
— which #55 scoped and **explicitly deferred** as a chooser-in-chooser.

So the two-class credit here is **not** available independently: grounding
schools gets Wizard immediately, but Arcanist only once School Understanding
is recognized. Either that Exploit comes into scope alongside this, or this
is honestly a one-class build with a second class pending.

That is a real dependency between two of my own scoping docs, and it should
be settled before the work is briefed rather than discovered mid-build.

## Recommended representative: **Abjuration School**

3 records, 3 magnitude-bearing — Resistance (`AbjurationResistanceBonus`),
Protective Ward (`AbjurationProtectiveWardTimes`), Energy Absorption
(`AbjurationEnergyAbsorption`) — plus a tokenless Immunity capstone. First
alphabetically, conventional, and structurally identical to six other
schools, so it exercises the general case rather than a special one.

**Existing coverage:** #62 counted 12 grounded `school` ids. Whether those
are Wizard-namespaced (like Cleric's domain ids) or class-agnostic (like the
familiar machinery) needs the same check Domains got — **and it is the same
question**, so whichever way #64 rules for domains should almost certainly
rule here too.

## Recommendation

**Worth doing, but after Domains (#64), and probably by the same hand.** The
two families pose an identical generalization question (per-class level
lookup into a shared power namespace) and Domains is both larger and already
scoped. Building domains first establishes the pattern; schools then follows
it at 29-schools-times-3-powers with unusually high density.

**Do not treat it as independent of #55.** Without School Understanding, the
Arcanist half of the credit is not real.

## Honest summary

Genuinely shared — the #62 flag was right. Substantially smaller than #62
claimed — the #62 *numbers* were wrong, by my own archetype-counting error.
Highest density of any shared family, and mechanically regular. A good
second target behind Domains, not a competitor to it.
