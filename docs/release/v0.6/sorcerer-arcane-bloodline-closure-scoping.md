# Sorcerer Arcane Bloodline Closure (Risks Item 8 Follow-On) — Scoping Plan

> Requested by the lead after Barbarian/Bard/Monk's incremental slices landed
> (all session-sized, none of which finish a whole class): a comparative
> scope check across Sorcerer's bloodline-power blocker, Cleric's
> domain-powers blocker, and Druid's animal-companion/nature-bond blocker,
> since closing any ONE of the three would take that whole class to genuine
> `Computed` — a materially different payoff than another incremental Monk
> feat. This doc is the result of that check: it finds Sorcerer's Arcane
> bloodline is the cheapest of the three, and sizes exactly what closing it
> requires, per the lead's request to scope before any code.

## The central finding: Sorcerer's Arcane bloodline has two provably-vacuous
## pieces, the same shape as Monk's Catch Off-Guard/Throw Anything — not a
## coincidence, a structural property of this codebase

Verified directly (`pilot_compute.rs`, `character_input.rs`, and two
independent primary sources per the session's standing discipline):

- **Bloodline Arcana**: "Whenever you apply a metamagic feat to a spell that
  increases the slot used by at least one level, increase the spell's DC by
  +1" (d20pfsrd, cross-checked against the same wording via web search
  aggregation). This precondition — *a metamagic feat was applied to a
  specific known spell, raising its effective slot* — cannot be represented
  by any input this codebase currently accepts: `SpellSelection` (the only
  struct naming a known/prepared spell) carries `spell_id`,
  `source_class_id`, and `acquisition_mode` only — no metamagic field, no
  slot-level-override concept, confirmed by direct inspection. The
  precondition is not merely unclaimed; it is unrepresentable, hence
  provably never satisfied by any character this bounded slice can compute.
- **Arcane Bond** (1st-level bloodline power): "you gain an arcane bond, as
  a wizard equal to your sorcerer level... Once per day, your bonded item
  allows you to cast any one of your spells known" (d20pfsrd). The
  "cast a spell" half of this benefit requires a spell-casting-resolution
  engine — confirmed absent not just for Sorcerer but for EVERY class in
  this codebase (`grep` for `cast_a_spell`/`spell_casting_execution`/
  `spell_resolution`/`fn.*cast` across `pilot_compute.rs`: zero hits). This
  codebase computes character STATS, never resolves an actual spell being
  cast, for any class — so "cast a spell via your bonded item" can never
  be triggered here regardless of build, the same structural absence that
  made Catch Off-Guard/Throw Anything's improvised-weapon precondition
  provably vacuous.

**This is the same KIND of finding as Monk's second pass, not the same kind
as the Monk feats that were ruled out** (Combat Reflexes/Deflect Arrows/
Improved Grapple/Scorpion Style — those depend on opponent-controlled
events this snapshot model has zero representation of, a "whole subsystem
doesn't exist" gap). Metamagic-slot-tracking and spell-casting-resolution
are similarly whole subsystems that don't exist — but critically, *nothing
about this bounded slice's own inputs can ever trigger them*, the same
"provably zero, not merely unmodeled" distinction that justified Catch
Off-Guard/Throw Anything. Recording this reasoning explicitly here so a
future reviewer can check it against the Combat Reflexes counter-example
this session already worked out, rather than re-deriving it from scratch.

## What's NOT vacuous, and needs real (small) work

- **Arcane Bond's own identity** (familiar vs. bonded item, which specific
  one) is a real, representable choice this bounded slice CAN recognize —
  it just isn't recognized yet. No fixture-grammar support exists for it
  (no `choice:sorcerer_arcane_bond` or equivalent). This needs: a new
  choice-selection constant pair (mirroring `SORCERER_BLOODLINE_CHOICE_ID`/
  `ARCANE_BLOODLINE_SELECTION_ID`'s own shape), recognized as a bounded
  +0 identity record — no new schema field needed, `choice_selection`
  already handles arbitrary `choice:X -> Y` pairs via the existing
  fixture-grammar `choice=` line, the same mechanism every other choice
  this session used.
- **The once-per-day budget** itself (1 use/day, flat, no formula) is
  trivial to ground as a flat magnitude — no Constitution/Charisma-modifier
  arithmetic needed, unlike Rage/Bardic Performance's rounds-per-day.
  Whether tracking "was it consumed today" needs the full
  `ClassAbilityActivation` schema, or a simpler boolean, is an open
  question below — Arcane Bond has no per-round activation state (it isn't
  "active/inactive this snapshot" the way Rage/Inspire Courage are; it's a
  static "chosen bond type + daily use budget"), so the heavier schema may
  be overkill.
- **Bonus spells and bonus feats at 3rd+ level**: correctly absent at
  Sorcerer level 1 (the only level this bounded slice's chassis/spell-list
  functions currently cover per `supported_sorcerer_level` / the sorcerer
  spell-list access ladder) — mirrors every other class's "correctly
  absent below the level gate" idiom exactly. Not vacuous in general, just
  vacuous AT the specific level this slice computes; extending Sorcerer
  levels 2+ (a separate, already-anticipated future widening, same as
  every other class's level-range extensions this session) will need to
  revisit this.
- **Bloodline class skill grant**: already grounded separately per the
  existing code comment ("the bloodline class skill grant... is grounded
  separately above as a recognition record and is no longer part of this
  blocker") — confirmed not part of what needs closing.

## Why this beats Cleric's and Druid's paths on cost

- **Cleric domain-powers**: the existing fixture already recognizes Good +
  Healing domains specifically, and already grounds Good's Touch of Good
  flat magnitude (`max(level/2, 1)`) and both domains' uses-per-day
  (`3 + Wis modifier`). But Touch of Good's actual application ("touch a
  creature, granting it a bonus on ONE attack roll, skill check, ability
  check, OR saving throw of the player's choice") needs a NEW "which roll
  type does this bonus apply to" schema concept that doesn't exist
  anywhere yet (broader than Rage/Inspire Courage's fixed-target bonuses).
  Healing domain's Rebuke Death additionally needs a real heal-amount
  (dice roll) applied to a real HP/healing total — this codebase's
  durability model computes max HP only, never a mid-session heal
  application. Both of Cleric's two already-recognized domains need real
  new work, not a vacuous correction.
- **Druid nature bond**: the animal-companion path needs a full companion
  stat-block/advancement subsystem (Strength/Dexterity/Constitution by
  companion type, HD progression, natural attacks, tricks) — the largest
  of the three by a wide margin. The domain-bond alternative isn't even
  recognized as an input today (no `bond:domain`-shaped constant exists),
  and would inherit Cleric's own not-yet-closed domain-powers cost above
  once built. Neither path is cheap.

Sorcerer's Arcane bloodline is the only one of the three where the
REMAINING gap, after the vacuous corrections above, reduces to "recognize
one new choice, ground one flat 1/day number" — genuinely comparable in
size to Barbarian's Rage or Bard's Inspire Courage, not a new subsystem.

## Proposed approach

1. **Bloodline Arcana correction**: ground a `class_feature.sorcerer.
   bloodline_arcana_absent` (or similar) recognition record documenting
   that this codebase has no metamagic-to-spell association concept, so
   the +1 DC precondition never arises — mirrors the Barbarian illiteracy
   / Monk Catch-Off-Guard "vacuous under this bounded scope" idiom exactly,
   +0 value, no fabrication.
2. **Arcane Bond identity recognition**: new `SORCERER_ARCANE_BOND_CHOICE_ID`
   constant (`choice:sorcerer_arcane_bond`) and two selection values
   (`bond:familiar`, `bond:bonded_object`), recognized the same way
   `SORCERER_BLOODLINE_CHOICE_ID` already is — a bounded +0 identity
   record naming which was chosen, fabricating no familiar/item stat block.
3. **Daily-use budget**: ground the flat `1` use/day as an explanation
   record (mirrors `class_chassis.bard.bardic_performance_rounds_per_day`'s
   own informational-only shape) — no formula needed, always exactly 1.
   Whether "was today's use already consumed" needs real tracking is an
   open question below; if yes, a simple boolean-shaped
   `ClassAbilityActivation`-style entry (or an even simpler dedicated
   field) suffices — Arcane Bond has no "active this round" state to
   toggle, unlike Rage/Inspire Courage.
4. **Retire or condition the diagnostic**: once 1-3 land, the
   `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported`
   diagnostic's stated reasons (Arcane Bond, bloodline arcana, bonus
   spells/feats at 3rd+) are ALL either genuinely resolved or genuinely
   vacuous at Sorcerer level 1 — the diagnostic should retire entirely for
   a Human single-class Sorcerer at level 1 with a valid, in-budget known-
   spell posture, the same "reaches Computed" outcome Barbarian achieved.
5. **Gate-ordering check**: `explain_sorcerer_level1_spell_baseline` was
   already hoisted above the Human/single-class gate in an earlier cycle
   (per its own doc comment, "checked BEFORE the single-class-only/Human
   gate below, mirroring the Ranger/Paladin fix") — confirm the new
   Arcane Bond recognition logic is added to that SAME already-hoisted
   block, not re-introducing the gate-ordering bug this session has fixed
   five times now.

## Open questions for the lead / adversarial review

- **Does "once per day" need real consumption tracking, or is
  recognition-only sufficient to retire the diagnostic?** Every other
  closed burden this session (Rage, Inspire Courage, Dodge) required the
  BENEFIT to be genuinely, conditionally correct — an over-budget Rage
  claim-blocks, for example. Arcane Bond's actual benefit (casting a known
  spell) can never be exercised in this codebase regardless (per the
  vacuous finding above), so there may be nothing FOR a consumption count
  to gate — recommend recognition-only (bond type chosen, 1/day budget
  named) with no consumption-tracking schema addition, since tracking
  "was it used" would gate a benefit that can never actually apply to
  anything. Flagging this as the one place this closure's shape genuinely
  differs from Rage/Inspire Courage, worth a second opinion before coding.
- **Scale of the `choice:sorcerer_arcane_bond` addition**: purely additive
  (new constant pair + one recognition branch), same "zero blast radius on
  existing fixtures" property every prior choice addition had — low risk,
  but confirming before code per the lead's ask.
- **QA-facing wave size**: unlike Barbarian (25 files) and Bard (32 files),
  this touches ONE diagnostic whose message currently only varies on
  whether Arcane bloodline (vs. an unrecognized bloodline) was chosen —
  a much narrower likely wave, worth confirming against `tests/sd13_
  sorcerer_*`/`tests/sd18_sorcerer_*` once scoped, same as prior classes.
