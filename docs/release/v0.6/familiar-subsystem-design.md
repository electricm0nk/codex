# Familiar Subsystem — Design Pass (#11 Witch, #12 Shaman)

> Third of the roster's "architecture blockers," scoped with the same
> three-tier discipline as the Studied-Target pass, and with the lead's
> explicit instruction to *confirm* the Tier-2 analogy rather than assume it.
>
> **Confirming was the right call — my own first-look read was wrong.** I
> predicted Tier 2 on the reasoning that "a familiar has its own stat block,
> so unlike a studied target its properties really are inputs." For the part
> that actually reaches the character sheet, that is false: **the familiar's
> master benefit is a flat magnitude on the PC that needs nothing from the
> familiar's stat block at all** — the same shape as Studied Target, one
> level up.

---

## What Witch's and Shaman's records actually contain

Both are thin. `KEY:Witch ~ Familiar` carries exactly two tokens:

```
ABILITY:Internal|AUTOMATIC|Standard Familiar List
BONUS:VAR|FamiliarMasterLVL|WitchLVL|TYPE=Base
```

`KEY:Shaman ~ Spirit Animal` is the same shape plus a flag:

```
BONUS:VAR|FamiliarMasterLVL|ShamanLVL
BONUS:VAR|FamiliarIsSpiritAnimal|1
BONUS:VAR|SpiritAnimalLVL|FamiliarMasterLVL*FamiliarIsSpiritAnimal
ABILITY:Internal|AUTOMATIC|Standard Familiar List
```

Neither carries a creature stat block. Both delegate to a shared species
list, and the species is where the real content lives.

## Tier 0 — the master benefit. Flat, self-scoped, **and four land on real computed totals.**

The standard familiar table, from `core_essentials/ce_abilities_familiar_cr.lst`:

| familiar | master benefit | lands on |
|---|---|---|
| **Toad** | `BONUS:HP|CURRENTMAX|3` | **computed max HP** (`durability.rs`) |
| **Rat** | `BONUS:SAVE|Fortitude|2` | **computed saves** |
| **Weasel** | `BONUS:SAVE|Reflex|2` | **computed saves** |
| **Lizard** | `BONUS:SKILL|Climb|3` | **computed selected-skill modifier** |
| Bat | `BONUS:SKILL|Fly|3` | standalone |
| Cat | `BONUS:SKILL|Stealth|3` | standalone |
| Hawk / Owl | `BONUS:SKILL|Perception|3` | standalone |
| Monkey | `BONUS:SKILL|Acrobatics|3` | standalone |
| Raven | `BONUS:SKILL|Appraise|3` | standalone |
| Viper | `BONUS:SKILL|Bluff|3` | standalone |

**Every one is a flat bonus applied to the master.** None reads any property
of the familiar creature. Choosing a familiar is a chooser that narrows to
one canonical pick exactly like Animal Focus's Bull or Oracle's Life
Mystery — and unlike almost everything else scoped this segment, **four of
the options land on totals this engine already computes**, so this is not
standalone-only grounding.

**Recommended canonical: Toad.** Its `+3` goes into computed max HP — the
most load-bearing and most visible total in the app — making this a genuine
integration rather than an explanation record. Owl is the recognizability
alternative if you would rather optimise for "looks like a witch's
familiar," the same tradeoff as Quadruped-vs-Biped on the Eidolon.

## Tier 1 — the proximity condition. No work needed, one honest note.

The corpus DESC states the benefits "apply only when you and your familiar
are within 1 mile of each other." The corpus's own cancellation mechanism
for this is the negative `FamiliarGrantedBonus_N` setters — and those live
in `horror_adventures` and `familiar_folio`, **neither ingested here** (see
hazard 2). Within this repo's scope the bonus is unconditional, so Tier 1 is
a sentence in the detail text, not a build.

## Tier 2 — the familiar creature's own stat block. Correctly deferred.

A familiar really is a creature: HD, saves, skills and Intelligence all
scale off master level, and *there* its own properties are genuine inputs.
That part is Tier-2-shaped, and my original instinct holds — **for that
slice only.** It is not required for anything above, and if it is ever
built, the Eidolon MVP is the template (fixed canonical species, chassis
math as standalone records, defer the rest).

---

## A decoupling finding that changes #11's shape

**Witch's spellcasting is not mechanically gated on the familiar.** The DESC
says "the familiar stores all the spells that you know," which reads like a
hard dependency — but there is **no `PRE` gate on `Witch ~ Cantrips` or
`Witch ~ Patron Spells`, and zero `PREABILITY` references to
`Witch ~ Familiar` anywhere in the file.** It is flavour text with no
mechanical enforcement in the corpus.

So #11's two halves are independent. **The 324-spell list module can be
built now, with no Familiar work at all** — and given the sweep already
established it is 100% ingested, that is a straightforward reuse job whose
only real cost is table size.

## Build-time hazards

1. **Provenance looks like a reachability blocker and is not.** The master
   bonuses live under `core_essentials/`, which is not one of the four books
   this repo ingests — I was one step from flagging this as a
   Bloodrager-style unreachable-content problem. It isn't: the file itself
   declares `SOURCELONG:Core Rulebook  SOURCESHORT:CR`, and the `_cr`
   filename suffix means Core Rulebook. `core_essentials` is PCGen's own
   *organisational* directory, not a separate rulebook. These are CRB rules.
2. **`FamiliarGrantedBonus_N` has negative setters elsewhere in the tree**
   (`-3`, `-2`, some gated on `PREVAREQ:FamiliarBondFeatActive,1`). They live
   in `horror_adventures` and `familiar_folio` — **neither ingested here**,
   so they are provably vacuous by the same check that cleared Alchemist's
   Ultimate-Magic Bomb override. Within `core_essentials` the value is
   cleanly `3` (and `2` for the save variants). Verify rather than inherit
   this conclusion if the ingested-book set ever widens.
3. **Toad's bonus writes to a computed total**, so it needs real consumer
   wiring in `durability.rs`, not just an explanation record — a higher bar
   than the standalone facts most of this segment produced. The same applies
   to Rat/Weasel (saves) and Lizard (Climb).
4. **Do not assume Witch and Shaman share one implementation.** RAW gives a
   shaman's spirit animal an ability tied to her *spirit*, whereas the corpus
   record routes it through the same `Standard Familiar List`. Those may
   disagree; check which the build should follow before writing shared code.
5. **`SpiritAnimalLVL = FamiliarMasterLVL * FamiliarIsSpiritAnimal`** is a
   multiply-by-flag idiom that silently yields 0 when the flag is unset.
   Correct for Shaman; do not copy it onto Witch's path, where the flag is
   never set.

## Recommendation

**Tier 0 for both classes** — one canonical familiar (Toad recommended),
master benefit only, wired into its real consumer. Tier 1 is a note. Tier 2
stays deferred with the Eidolon precedent recorded as its template.

**And build Witch's spell-list module independently, now** — it is not
blocked by any of this.

**Honest status expectation:** both classes stay **Blocked** — Witch on
hexes and the familiar creature, Shaman on the other nine spirits and its
own spellcasting. `named_features_wired` +1 each for the familiar slot.

## Open questions for the lead

1. **Toad or Owl as the canonical familiar?** I recommend Toad for the live
   max-HP consumer; Owl is the more recognisable pick if you would rather
   optimise for that.
2. **Should Witch's spell-list module be split into its own task now that
   it is confirmed decoupled from the Familiar?** #11 currently reads as one
   unit and they are genuinely independent — the list is buildable
   immediately, the familiar is a separate small slice.
3. **Hazard 4 wants a ruling before backend writes shared code**: follow the
   corpus (spirit animal uses the standard species list) or RAW (spirit
   animal grants a spirit-linked ability)? This codebase's convention is
   corpus-first, which would mean one shared implementation — but it is worth
   saying so deliberately rather than discovering the divergence mid-build.
