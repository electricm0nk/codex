# Roster Survey Round 2 (#51) — Next Scoping Candidate

> Run with the fully corrected methodology this round established. **Two of
> the four checks changed the answer, and one found a fifth prefix.**
>
> **Headline: single-class scoping is essentially exhausted. Every remaining
> gap of any size is a shared chooser family, and the highest-leverage one is
> Rage Powers — 60 active records, zero magnitudes grounded, serving two
> classes including a marquee CRB one.**

## Methodology results (the four checks)

**1. Prefix enumeration — there is a fifth class-bearing prefix.**

```
class_feature 707 | class_chassis 349 | class_spell 117 | class 12 | ability_modifier 4
```

`ability_modifier.<class>.*` (e.g. `ability_modifier.skald.inspired_rage_bonus_applied`)
is a genuine fifth, and it is precisely a *consumer-integration* record —
the category check 2 exists to catch. Re-enumerating found it; the
previously-corrected four-prefix list would have missed it. **The rule keeps
paying: enumerate, never name.**

**2. Consumer-integrated features (Stern Gaze / item 67)** — applied;
folded into the counts below by separating diagnostics from real features
rather than counting raw ids.

**3. Name-collision check (item 69)** — applied; this is what confirmed
Rage Powers is genuinely ungrounded rather than merely present-by-name.

**4. Corrected parser + `#`-exclusion** — applied to the Rage Power count:
**60 raw, 60 active**, no disabled records inflating it. (Worth stating
positively — the `#` bug does not touch this family.)

## Grounded real features per class (diagnostics excluded)

| tier | classes |
|---|---|
| **4** | Inquisitor, Hunter — *both already queued (#47, #44)* |
| **7-8** | Skald *(#50)*, Summoner, Arcanist, Slayer |
| 9-11 | Witch, Bloodrager *(#42)*, Sorcerer, Alchemist, Cavalier, Shaman |
| 13-17 | Rogue, Wizard, Investigator, Oracle, Brawler, Cleric, Druid, Swashbuckler, Warpriest, Barbarian |
| 21-59 | Paladin, Fighter, Bard, Monk, Ranger |

**Every class at the thin end is already scoped or building.** The
un-queued leaders are Arcanist and Slayer at 8 — and inspecting both shows
their remainders are choosers, not flat features:

- **Slayer** — `sneak_attack_dice`, `studied_target_bonus`,
  `studied_target_count`, **`talent`, `talent_count`**, `track_bonus`,
  `trap_sense_bonus`, `trapfinding_bonus`. Slayer Talents **already
  grounded**. Effectively complete.
- **Arcanist** — remainder is **Exploits**: 46 records, exactly one
  (Metamagic Knowledge) grounded.

## The real gap: shared chooser families

| family | records | classes | grounded |
|---|---|---|---|
| **Rage Powers** | **60 active** | **Barbarian (CRB) + Skald (ACG)** | **0 magnitudes** — only `class_chassis.barbarian.rage_power_choice`, a choice-recognition record |
| Arcanist Exploits | 46 | Arcanist | 1 (Metamagic Knowledge) |
| Rogue Talents | 13 pool-grants | Rogue + Investigator (own 14 copies) | 0 |
| Bloodlines | 5 pool-grants | Sorcerer + Bloodrager | 0 |

**Rage Powers wins on every axis**: most records, zero magnitude coverage,
two classes, and one of them is Barbarian — a marquee CRB class sitting at
17 grounded features with this as its main remaining hole. Both grant pools
are verified: Barbarian `RagePowersLVL/2`, Skald `RagePowersLVL/3`.

**Timing is favourable too:** Skald's own rage-power *pool count* is already
in #50's scope, so a canonical rage-power narrowing would complete that
thread rather than opening a new one.

## Recommendation

**Scope the Rage Power family next**, canonical-narrowing style (one
representative power grounded, the other 59 named and deferred) — the same
shape as Oracle's Mystery, Hunter's Animal Focus, and Cavalier's Order.

Expected: one build credits **two** classes, and closes Barbarian's largest
remaining gap.

**Second choice: Arcanist Exploits** — larger per-class effect (46 records,
1 grounded) but single-class, so lower leverage.

## What this survey does not claim

- Grounded counts separate diagnostics by name matching, so a class using an
  unusual diagnostic id could be off by one.
- Per check 2, counts remain a **floor**: consumer-integrated features
  without explanation records are invisible, and the fifth prefix found this
  round is exactly that category — so more may exist.
- Chooser families are keyed outside the `KEY:<Class> ~` namespace, so the
  class-level counts under-represent them by construction. That is the whole
  reason the answer is a family rather than a class.
