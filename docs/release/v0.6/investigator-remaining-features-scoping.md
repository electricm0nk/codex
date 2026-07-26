# Investigator (#8) — Fresh Remaining-Features Pass (corrected standalone-grounding bar)

> Directed by the lead: do a fresh Investigator pass under the corrected
> canonical bar (a feature grounds as a standalone record whenever its
> magnitude is real and verifiable — no live consumer required), since its
> own deferred features might hide the same class of missed win Animal
> Focus and Bardic Knowledge turned out to be. Investigator's flat features
> (Trapfinding / Trap Sense / Inspiration pool) already landed (commit
> `46eb6c3d`); task #8 as written is the spellcasting subsystem. This pass
> keeps that and re-examines every *other* deferred Investigator feature
> under the corrected bar. **Result: two clear missed wins (Poison
> Resistance, Alchemy), one opponent-dependent judgment call (Studied
> Combat/Strike) that should be ruled consistently with Slayer #13, and the
> rest correctly deferred.**

## The main task body (#8) — spellcasting subsystem, unchanged

Confirmed still accurate from the earlier Investigator scoping: closing
spellcasting means building the shared **Alchemist formula spell-list** — a
new `alchemist_spell_list.rs` `(&str, u8)` mapping ingested from the **104
real `Alchemist=N` records** in `apg_spells.lst` — then wiring
Investigator's prepared-caster validation (`SPELLSTAT:INT MEMORIZE:YES
SPELLBOOK:YES`, reusing the Wizard/Arcanist prepared shape). ~Warpriest-
sized, and high-leverage: the list also unblocks Alchemist's own deferred
spellcasting (#4). Unchanged by the corrected bar.

## Missed wins under the corrected bar (were lumped into "defer the rest")

### Poison Resistance — groundable standalone (situational-save magnitude)

- Corpus (re-derived directly, correcting an earlier secondary-source
  slip in this doc): `BONUS:VAR|InvestigatorPoisonResistanceBonus|2` adds
  +2 per `InvestigatorPoisonLVL` step, and `InvestigatorPoisonLVL` itself
  increments at `PREVARGTEQ:InvestigatorLVL,2/5/8`, with the level-10 gate
  (`PREVARGTEQ:InvestigatorLVL,10`, `Investigator_CF_PoisonImmunity`)
  flipping to **full "Immunity to Poison"**. So the real progression is:
  **None below level 2, +2 (levels 2–4), +4 (5–7), +6 (8–9), immunity at
  10+** — a bonus on saves vs poison. **There is no +8 tier, and immunity
  arrives at level 10, not 20** (an earlier draft of this doc mis-stated
  "+2/+4/+6/+8, immunity at 20th" from secondary-source memory; corrected
  here after a direct corpus re-derivation — the exact failure mode the
  corpus-first discipline exists to catch).
- A situational-save standalone magnitude — **the exact shape as Bard
  Well-Versed and Inquisitor's grounded Purity judgment**. Grounds cleanly
  under the corrected bar as `class_feature.acg.investigator.poison_resistance`,
  no consumer total needed. **Clear missed win.**
- **Shared-leverage bonus:** Poison Resistance is *also* one of Alchemist's
  deferred features (task #4 names it). A single standalone-magnitude
  helper could serve both classes, the same shared-investment logic as the
  Alchemist formula list.

### Alchemy — groundable standalone (flat Craft-check magnitude)

- Corpus: `BONUS:VAR|InvestigatorAlchemyCreationBonus|InvestigatorLVL`
  (DESC: "highly trained in the creation of mundane [alchemical items]") —
  a flat +level bonus to create mundane alchemical items.
- A flat skill-check magnitude, the **same shape as Bard Bardic Knowledge**
  (a flat competence bonus on checks with no computed total). Grounds
  standalone under the corrected bar. Niche (Craft-alchemy is not a
  computed skill), but a real, verifiable magnitude — **a genuine missed
  win**, not a defer.

## Opponent-dependent judgment call — rule consistently with Slayer #13

### Studied Combat / Studied Strike

- Corpus: Studied Combat `BONUS:VAR|InvestigatorStudiedCombatBonus|InvestigatorLVL/2`
  (an insight bonus on attack/damage vs a studied target, duration
  `max(1,INT)`); Studied Strike `InvestigatorStudiedStrikeDice|min(9,…)`,
  die size 6 (bonus damage dice vs a studied target).
- The magnitudes ARE flat and verifiable — but both are **opponent-
  dependent**: the bonus only exists in the context of a studied-enemy
  interaction the engine models nowhere. This is the **same wall as
  Slayer's Studied Target** (task #13: "design opponent-tracking pillar…").
- **This needs a consistent ruling, not a solo call.** Under the corrected
  bar one *could* ground "Studied Combat insight bonus = +level/2 (applies
  vs a studied target)" as a conditional standalone magnitude — but that
  states a bonus whose activation condition the engine can't evaluate,
  which is a weaker kind of honesty than a self-applied always-on bonus
  (Poison Resistance, Nimble). My read: **keep opponent-dependent bonuses
  deferred** pending the opponent-tracking pillar (#13), and apply the same
  rule to Investigator Studied Combat/Strike and Slayer Studied Target
  together — but flagging it explicitly as the lead's call, since it's the
  one place the corrected bar has a genuine gray zone.

## Correctly deferred — no magnitude exists (not missed wins)

Verified each carries **no numeric `BONUS`** (like Hunter's Nature
Training) — nothing to ground even standalone:
- **Keen Recollection** — makes all Knowledge checks untrained (a
  permission, not a bonus).
- **Poison Lore** — identify poisons (identification capability, no
  numeric magnitude).
- **Swift Alchemy** — craft alchemical items in half time (a time
  modifier, no magnitude).
- **True Inspiration** — level-20 capstone, folds into the Inspiration
  mechanic (no separate magnitude).

## Correctly deferred — chooser-lists (unchanged)

Investigator Talents, the Discovery sub-list, and the Rogue-Talent
sub-list (the bulk of the 95 KEY records) — real chooser-lists, deferred
under either bar, as before.

## Net (corrected)

Task #8's spellcasting subsystem is unchanged (~Warpriest-sized, unblocks
Alchemist). **The corrected bar adds two cheap standalone features that
were previously lumped into "defer the rest": Poison Resistance
(situational save, also shared with Alchemist) and Alchemy (flat Craft
bonus).** These could bundle as a cheap pre-slice before the spellcasting
lift, or land alongside it — either way they raise `named_features_wired`
for real. Studied Combat/Strike stay a flagged judgment call tied to the
Slayer #13 opponent-tracking decision.

## Open questions for the lead

1. Bundle **Poison Resistance + Alchemy** (two cheap standalone missed
   wins) into task #8 — as a quick pre-slice before the spellcasting
   subsystem, or alongside it? Poison Resistance is shared with Alchemist
   (#4), so a shared helper is worth considering.
2. **Opponent-dependent ruling:** keep Studied Combat/Studied Strike (and,
   consistently, Slayer's Studied Target) deferred pending the
   opponent-tracking pillar (#13) — my recommendation — or ground their
   flat magnitudes now as conditional standalone facts under the corrected
   bar? This is the one genuine gray zone in the corrected bar and wants a
   single consistent rule across both classes.
3. Spellcasting-subsystem sequencing (unchanged from prior): build the
   Alchemist formula list under #8 (unblocking both Investigator and
   Alchemist), or split the ingestion into its own task both depend on?
