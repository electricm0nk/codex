# Skald (#7) — Remaining-Features Scoping, with the Consumer-Existence Check Applied

> Directed by the lead after Inquisitor (#3) landed with a real correction
> to my prior comparative sizing: a formula existing in the corpus isn't
> sufficient to ground a feature — it also needs a **live consumer already
> computed** in `pilot_compute.rs`. Of my 4 flat Inquisitor features, only
> Stern Gaze (lands on the computed Intimidate total) grounded; Monster
> Lore / Cunning Initiative / Track were deferred (Knowledge / Initiative /
> Survival totals don't exist). Re-scoping Skald's three candidates
> (Bardic Knowledge, Well-Versed, Lore Master pool) with that check applied
> explicitly — and surfacing a real cross-closure **inconsistency** the
> check exposed, because it changes the answer for Bardic Knowledge and
> reaches beyond Skald.

## The consumer-existence check, made precise (and a real inconsistency it exposed)

Applying the check honestly required first pinning down what "groundable"
has actually meant in this codebase. Reading the shipped code, there are
**two** distinct grounding shapes, not one:

1. **Integration into a computed total** — e.g. Stern Gaze added to the
   Intimidate total (`pilot_compute.rs:26331`), Justice into the melee
   attack total. Requires the target total to exist. Intimidate exists
   only because it is one of the three "selected skills"
   (Climb/Intimidate/Swim) that `compute_selected_skill_modifiers`
   computes; Knowledge, Survival, Initiative, AC, damage, etc. have **no
   computed total**.
2. **Standalone magnitude record** — a flat value emitted as its own
   explanation, honestly labelled "never applied to any total because no
   X-resolution engine exists." This is a **deeply established idiom**: the
   phrase/pattern appears **31 times** in `pilot_compute.rs`, and the code
   itself names the lineage — "mirroring the Fighter Bravery / Rogue Trap
   Sense / Barbarian Trap Sense / Monk Still Mind idiom" (`:24903`).

**The inconsistency:** Inquisitor's just-landed closure deferred Monster
Lore, Cunning Initiative, and Track on the grounds that their totals don't
exist — but structurally identical features are **already grounded as
standalone magnitude records** elsewhere in the shipped codebase:

- **Bard Bardic Knowledge** — `class_chassis.bard.bardic_knowledge`
  (`pilot_compute.rs:24728-24742`), value `max(level/2,1)`, a competence
  bonus on Knowledge checks, explicitly "grounds only the flat
  Knowledge-check competence bonus; it is not a full Knowledge-check
  resolution engine." **Identical shape to Inquisitor's deferred Monster
  Lore** (a flat bonus to Knowledge checks).
- **Slayer Track** — `class_feature.acg.slayer.track_bonus`
  (`pilot_compute.rs:10569`), `max(level/2,1)` on Survival. **Identical
  shape to Inquisitor's deferred Track.**
- **Bard Well-Versed** (`class_feature.bard.well_versed`, `:24882`),
  **Swashbuckler Nimble**, Fighter Bravery, Barbarian/Rogue Trap Sense —
  all flat/situational bonuses grounded standalone with no consumer total.

So the "live consumer" bar that Inquisitor applied is **stricter than, and
inconsistent with, the dominant shipped idiom.** The Inquisitor deferrals
aren't wrong under the strict bar — but calling them "correctly deferred"
sits uneasily next to Bard's and Slayer's identical-shape features that
*are* grounded. This is a real codebase-consistency question, and it
reaches past Skald (it retroactively questions either the Inquisitor
deferrals or the older standalone groundings). **The repo's own
`no-stub-mvp-doctrine` / `wired-integration-discipline` arguably favors the
strict bar** (don't emit a number that connects to nothing) — but the
older records are honestly labelled as not-integrated, so they are defended
partial facts, not deceptive stubs. Both stances are coherent; **what isn't
coherent is grounding Bard's Bardic Knowledge while deferring Skald's.**
This is the lead's call to make, and it directly decides one of Skald's
three candidates.

## Skald's three candidates, each checked independently

### Well-Versed — groundable under BOTH bars (cheapest, strongest)

- Corpus: `BONUS:VAR|SkaldWellVersedBonus|4` (flat +4 on saves vs bardic
  performance / sonic / language-dependent effects).
- **Direct precedent already shipped:** `class_feature.bard.well_versed`
  (`:24882`) grounds the identical +4 as a standalone situational-save
  magnitude (`BARD_WELL_VERSED_BONUS = 4` — same value). And even the
  strict bar grounds situational-save riders: Inquisitor's own **Purity**
  judgment (a save-vs-curses/disease/poison rider) grounded. So Well-Versed
  is safe either way — it is a self-contained situational magnitude, not a
  bonus to a missing general total.
- **Groundable.** Build-time note: confirm Skald's own grant level (Bard's
  is level 2) against Skald's own record rather than assuming Bard's gate.

### Lore Master pool-size — groundable under BOTH bars (thin but real)

- Corpus: `BONUS:VAR|SkaldLoreMasterUsesPerDay|min((SkaldLVL-1)/6,…)` — a
  uses-per-day pool (take 10/20 on a Knowledge check).
- The pool **size** is a self-contained quantity, the same shape as
  Swashbuckler Panache (`class_feature.acg.swashbuckler.panache_max`,
  `:10661`) and Warpriest Blessing uses/day (`:10334`) — both grounded as
  standalone pool facts needing no consumer total. **Groundable.** The
  take-10/20 *effect* isn't modeled (no Knowledge-check engine), so ground
  the pool size only, named honestly — exactly the Panache "pool size, use
  not modelled" precedent.

### Bardic Knowledge — THE decision point (grounds under the established idiom, defers under the strict bar)

- Corpus: `BONUS:VAR|BardicKnowledgeSkillBonus|max(1,SkaldLVL/2)` on all
  Knowledge skills — **structurally identical to Bard's own grounded
  Bardic Knowledge and to Inquisitor's deferred Monster Lore.**
- Under the **established standalone-magnitude idiom**: grounds directly,
  mirroring `class_chassis.bard.bardic_knowledge` (same `max(level/2,1)`
  formula). This is not a "dead consumer" — Bard's own identical feature is
  grounded and shipped.
- Under the **strict Inquisitor bar**: defers, exactly like Monster Lore.
- **Not my call to pick** — it depends entirely on which standard the lead
  makes canonical. My prior sizing pass called it a clean win (correct
  under the established idiom); the lead's suspicion that it's a
  "dead-consumer case" is correct *only under the new strict bar*. The
  honest answer is: it's groundable-or-deferred depending on a consistency
  decision that isn't Skald-specific.

## The other ~15 remaining Skald features — defer regardless of the bar

Rage Powers (a chooser-list), the higher-level Raging Song variants (Song
of Marching / Strength / Dirge of Doom, etc. — situational party effects),
Spell Kenning (cast from other classes' lists — complex), Bonus Feats
(feat grants), Damage Reduction/Improved uncanny dodge, etc. All are
chooser-lists or need engine state that doesn't exist. Deferred under
either bar — unchanged from the prior pass.

## Net sizing (corrected)

Skald is **thinner than my prior pass implied**, and the exact count hinges
on the consistency decision:

- **Strict bar:** 2 groundable (Well-Versed, Lore Master pool) — a small
  slice, `named_features_wired += 2`.
- **Established idiom:** 3 groundable (+ Bardic Knowledge) — still small.

Either way Skald is a modest slice (2–3 flat/pool facts), not the richer
win the label "18 remaining features" might suggest — the bulk is
choosers/complex. This matches the "cheap but small" tier from the
comparative pass, now with the Bardic Knowledge caveat made precise.

## Open questions for the lead

1. **The consistency decision (bigger than Skald):** is the strict
   consumer-existence bar (Inquisitor's) the go-forward canonical standard
   — in which case Skald's Bardic Knowledge defers, and the ~31 existing
   standalone-magnitude records (Bard Bardic Knowledge, Slayer Track,
   Nimble, Fighter Bravery, …) are grandfathered / flagged for later
   review — **or** is the established standalone-magnitude idiom canonical,
   in which case Skald's Bardic Knowledge grounds like Bard's own? (I lean
   toward naming the strict bar canonical for *new* work since it aligns
   with `wired-integration-discipline`, but that implies the older records
   are a real, if honestly-labelled, inconsistency worth a tracked
   follow-up — your call.)
2. Given the decision, is a **2–3-feature Skald slice** (Well-Versed +
   Lore Master pool, ± Bardic Knowledge) worth building now, or does its
   thinness push it below Hunter (#2), whose spellcasting reuse adds a
   whole pillar? (This may reorder the Skald-then-Hunter sequence you set.)
3. Should the Inquisitor Monster Lore / Track / Cunning Initiative
   deferrals be revisited for consistency with Bard/Slayer once you've
   decided #1 — or left as-is under the new bar?
