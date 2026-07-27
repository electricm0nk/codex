# Cleric Domains (#63) — Representative Ladder + Generalization Scoping

> Largest-leverage class-ladder family (292 records, 159 magnitude-bearing,
> one shared namespace). Two questions to answer: what the generalization
> actually requires, and whether the domain choice needs a canonical pick.
>
> **Answers: it is more than a level-variable swap — because the level
> variable is an *expression*, not a reference. And no, the domain choice
> needs no canonical pick; per-power grounding proceeds independently.**

## Finding 1 — Domains reach **five** classes, not three

`BONUS:VAR|DomainLVL|…` is fed by **six paths**:

```
ClericLVL      DruidLVL       InquisitorLVL
HunterLVL-2    PaladinLVL     PaladinLVL-3
```

So the family covers **Cleric, Druid, Inquisitor, Hunter and Paladin** —
more leverage than the brief assumed, and more than any other family
surveyed.

**But note the offsets.** Hunter is `HunterLVL-2`; one Paladin path is
`PaladinLVL-3`. **The level source is per-class arithmetic, not a plain
class-level reference.** A generalization that simply swaps which class's
level it reads would be silently wrong for Hunter and for one Paladin path —
computing a domain power two or three levels too strong.

That is the precise answer to the brief's open question: **not a
straightforward parameterization.** Close to one, but with a per-class
offset table that has to exist.

## Finding 2 — the generalization has three coupling points, and one is already done

Inspecting the shipped Cleric implementation:

| piece | state |
|---|---|
| **the formula** — `cleric_touch_of_good_bonus(level: u8) -> (level/2).max(1)` | **already generic.** Takes a bare level; no Cleric coupling at all. Zero work. |
| **the level lookup** — `active_cleric_touch_of_good_bonus` hardcodes `class_id == CLERIC_CLASS_ID` | needs per-class dispatch **plus the offset table above** |
| **the choice-set id** — `CLERIC_DOMAIN_CHOICE_ID` | needs to accept one domain choice-set per class |

So the good news is real: the arithmetic core was written class-agnostically
already. The work is entirely in *how the level and the choice are looked
up*, not in the domain logic itself.

## Finding 3 — the emitted-id namespace is the actual design decision

Current ids are `class_chassis.cleric.domain_power_good_touch_of_good_bonus`
— Cleric-namespaced. Two options:

1. **Per-class ids** (`class_chassis.inquisitor.domain_power_…`) — mirrors
   the existing shape, but multiplies records five ways for one shared
   mechanic.
2. **A shared `class_feature.domain.*` namespace** — exactly what the
   **familiar machinery already does** (`class_feature.familiar.master_hit_point_bonus`,
   class-agnostic, and directly reusable by Arcanist as #56 showed).

**Recommend option 2.** There is shipped precedent that it works, it is why
Arcanist's Familiar Exploit became near-free, and it is the difference
between this being a one-time investment and a five-times-repeated one.

## Recommended representative: **Good Domain / Touch of Good**

The shortest path to a genuine multi-class extension:

- **Already partially covered for Cleric**, so the choice-recognition,
  activation-gating and diagnostic paths all exist and work.
- **Its formula is already generic** — `max(level/2, 1)` — so extending it
  exercises exactly the level-lookup and choice-set generalization without
  any new arithmetic.
- Corpus: `Domain Power ~ Touch of Good` carries
  `BONUS:VAR|TouchofGoodTimes|DomainGoodTimes` — a uses/day pool, the
  idiom this codebase grounds routinely.

Healing / Rebuke Death is the alternative and is equivalent in shape
(`RebukeDeathTimes = DomainHealingTimes`), but Good is already the
further-along of the two.

## Answer to the brief's second question: **no canonical domain pick needed**

Per-power grounding proceeds independently of the 34-domain chooser. The
shipped code already recognizes Good and Healing individually and routes
everything else to an honest catch-all
(`unrecognized_other_domain_chosen` → stays Blocked). Extending to more
classes preserves that structure unchanged — **the chooser is already
handled; only the class dispatch is not.**

This is a meaningful contrast with Bloodlines, where the ladder arrives as
an automatic package and the bloodline pick *is* the unit of work.

## Build-time hazards

1. **Per-class level offsets** (Hunter −2, Paladin −3) — the single most
   likely thing to get silently wrong, since five of six paths are plain and
   two are not.
2. **Three key formats coexist in the domain namespace**:
   `<Name> Domain`, `<Name> Domain ~ <Power>`, and `Domain Power ~ <Power>`.
   A sweep matching only one shape sees a partial family.
3. **Duplicate records.** `Domain Power ~ Touch of Good` appears **twice**,
   and Rebuke Death appears under **both** `Domain Power ~ Rebuke Death` and
   `Healing Domain ~ Rebuke Death`. Dedup by power name, not by record —
   same shape as the `Resounding Blow` base/`.MOD` duplicate that produced
   the Inquisitor count dispute.
4. **A wrapper layer exists** — `Core Domain ~ Good Domain` /
   `Core Domain ~ Healing Domain`, carrying no tokens. Don't mistake it for
   the power records.

## Recommended scope

**Generalize the domain machinery using Touch of Good as the exercise**:
move the level lookup behind a per-class source (with the offset table),
accept per-class domain choice-set ids, and emit under a shared
`class_feature.domain.*` namespace on the familiar precedent.

**Expected payoff:** one build extends an existing, working Cleric feature to
**four more classes** (Druid, Inquisitor, Hunter, Paladin) — the largest
multi-class credit available anywhere on the roster, and it leaves the
remaining 32 domains cleanly deferred behind the catch-all that already
exists.

## Open question

**Shared `class_feature.domain.*` namespace, or per-class ids?** I recommend
shared, on the familiar precedent. It is the decision that determines
whether the other four classes are near-free afterwards or each cost a
repeat — worth ruling explicitly before the build rather than discovering it
at the fourth repetition.
