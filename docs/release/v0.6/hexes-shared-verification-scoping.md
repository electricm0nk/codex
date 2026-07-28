# Hexes (#71) — Shared-vs-Parallel Verification + Scoping

> The last un-scoped ladder family. Shared-vs-parallel answered by **direct
> corpus check**, not by inference from the other three (Domains and Schools
> came out shared; Bloodlines parallel — the pattern predicts nothing).
>
> **Verdict: PARALLEL, like Bloodlines. No two-class leverage.**
>
> **But Witch's hexes are 100% magnitude-bearing — the densest family
> measured on the roster.**

## The verification

**Separate KEY namespaces:**

```
Witch Hex ~ …          19        Shaman Hex ~ …            13
Witch Major Hex ~ …     8        Shaman Spirit Hex ~ …     49
Witch Grand Hex ~ …     5        Shaman Wandering Hex ~ …  49
```

Separate namespaces alone prove nothing — **Arcanist's schools looked
separate too**, and turned out to be wrappers plugging into Wizard's
namespace. So I applied the distinguishing test directly:

**Are Shaman's records wrappers, or real content?** `Arcanist School ~ X`
carried **only** `DEFINE:<School>SchoolLVL|0` — pure plumbing. **Shaman's
hex records carry their own magnitude tokens: 10 of 13 `Shaman Hex` records
and 40 of 49 `Shaman Spirit Hex` records.** They are independent content,
not a redirect into Witch's namespace.

**Name overlap is duplication, not sharing.** Seven names appear in both
sets — Charm, Evil Eye, Fortune, Healing, Misfortune, Tongues, **Ward** —
but as separate records with their own tokens. That is exactly the
Bloodlines shape (10 overlapping bloodline names, entirely distinct
records), and exactly the trap that made Bloodlines *look* shared.

**Conclusion: parallel. A canonical pick serves one class.**

## Corrected counts

Applying the key-format and wrapper lessons from Domains and Schools:

| set | records | magnitude-bearing | note |
|---|---|---|---|
| **Witch** (Hex + Major + Grand) | **32** | **32 — 100%** | densest family measured |
| **Shaman** (Hex + Spirit Hex) | **62** | **50 — 81%** | independent content |
| `Shaman Wandering Hex` | 49 | **0** | **wrappers/selectors** for the temporary-hex mechanic — excluded |
| `Hex Channeler ~ …` | 1 | — | **archetype** by the KEY-prefix rule (not a base class name) |

**#62 reported 143 records / 82 magnitude-bearing.** The **82 was correct**;
the 143 included the 49 wandering-hex wrappers. Counting magnitudes rather
than records was what saved that figure — worth noting, since the same
report's Domains and Schools record counts were both wrong.

## What the density means

**Witch's 32-for-32 is the highest ratio of any family measured**, alongside
Ranger's Favored Enemy/Terrain. Every single Witch hex record carries a real
magnitude — no tokenless filler at all, unlike Rage Powers (18 of 60) or
Rogue Talents (10 of 67).

Shaman's 81% is second-best among the ladders.

So this family is *small but almost entirely substantive* — the inverse of
Rogue Talents, which was large and mostly empty.

## Current coverage and the canonical pick

**Witch's Ward hex is already grounded** (flat `+2/+2` deflection/resistance,
scaling to `+4`). That is 1 of 32.

**Only one Witch hex lands on a computed total: Flight** —
`BONUS:SKILL|Swim|4|TYPE=Racial`, and Swim is one of the three computed
selected-skill modifiers.

**Caveats, stated rather than glossed:** Flight is gated (`PREMULT:1,…`) and
is a multi-tier hex — swim at 1st, then feather fall, levitate and fly at
higher tiers — of which **only the first tier carries a token.** So
grounding it means grounding the `+4` Swim and honestly naming the later
tiers as unmodelled, the same split used for Breath Weapon and Bomb.

Every other Witch hex grounds standalone.

## Honest value assessment

**Parallel, so no leverage — one class per build**, same as Bloodlines and
unlike Domains/Schools. That is the single biggest strike against it.

Against that: **highest density on the roster**, Witch is a still-Blocked
class with only one hex grounded of 32, and the records are small and
regular. It is a real slice, just not a multiplied one.

**Ranking against what's been built:** better than Rogue Talents (denser,
more records), worse than Domains or Schools (no shared leverage), comparable
to Bloodlines but much smaller per-unit — a hex is one record, not a
six-power ladder.

## Recommendation

**Ground Flight as the Witch canonical** (`+4` Swim into the computed total,
later tiers named and deferred), and treat Shaman's set as a **separate
follow-on** rather than a free rider — the parallel structure means it earns
nothing automatically from the Witch build.

**If capacity is tight, this is deferrable.** It is a genuine slice with no
multiplier, and the queue already holds work with better leverage. I would
not displace anything currently queued for it.

## Method note

The distinguishing test — *do the second class's records carry their own
magnitudes, or only `DEFINE` plumbing?* — is what separated Schools (shared)
from Bloodlines and now Hexes (parallel). Namespace shape alone predicts
nothing; three families with separate-looking namespaces split two-to-one on
this test.
