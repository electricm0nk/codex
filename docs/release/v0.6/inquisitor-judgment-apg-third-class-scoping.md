# APG Inquisitor Judgment — Third APG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Self-directed after the choice-picker Path A closure, surveying the
> remaining 10 untouched ACG/APG classes for the next cheap win. Written
> up here and flagged to the lead before building, matching the
> established rhythm for every prior class-specific closure this
> session.
>
> **Course-correction, not a reversal, of the earlier scan verdict**
> (flagged explicitly per the lead's own request, same honesty bar as
> every other scoping doc's revision history this session): the original
> cheap-win scan grouped Inquisitor with Oracle/Summoner/Witch as
> "chooser-list-with-real-mechanical-variety, not a quick win." That
> verdict was correct about the FULL breadth (Judgment's 8 types, Domain,
> spellcasting, Bane/Monster Lore/Stern Gaze/etc. together). It did not
> anticipate narrowing to ONE canonical judgment type as the MVP -- the
> same move that already made Alchemist's Mutagen (one canonical stat
> choice, not all three tiers) and Cleric's domain (Good only, not the
> full domain list) tractable. Justice was picked specifically because
> it needs zero new engine state (see below); the other 7 judgment types
> remain exactly as costly as the original scan found.

## Corpus findings (verified against `apg_classes.lst` / `apg_abilities_class.lst`)

- **Chassis**: 3/4 BAB, good Fortitude/Will, poor Reflex (matches
  `EXPECTED_LEVEL_1`'s `("class:inquisitor", 0, 2, 0, 2, 8)` row). HD 8.
  Already genuinely wired generically via `compute_apg_class_chassis`/
  `class_chassis_resolve` for all 6 APG classes -- confirmed zero other
  mentions of "inquisitor" anywhere in `pilot_compute.rs` before this
  closure (grepped directly), so this is the same "chassis grounded,
  named feature untouched" starting point every prior class-specific
  closure this session began from.
- **Judgment** (`KEY:Inquisitor ~ Sacred Judgment` / `~ Profane Judgment`,
  `apg_abilities_class.lst:297-298`): "pronounce judgment upon her foes
  as a swift action... receives a bonus or special ability based on the
  type of judgment made... use this ability %1 times per day... lasts
  until combat ends." Uses/day formula: `BONUS:VAR|InquisitorJudgmentTimes
  |1+(InquisitorLVL-1)/3` (line 354). Which of Sacred/Profane an
  inquisitor gets is alignment-gated (`!PREALIGN:LE,NE,CE` for Sacred,
  `!PREALIGN:LG,NG,CG` for Profane -- both restrictions admit any Neutral
  alignment, so a True/Lawful/Chaotic-Neutral inquisitor could get
  either), but every judgment TYPE'S numeric bonus is identical between
  the two variants (verified: both `Judgment (Sacred)` and `Judgment
  (Profane)` DESC blocks list the same 8 sub-types with the same
  formulas) -- so no alignment branching is needed for the VALUE, only
  the flavor name, which this closure does not attempt to pick correctly.
- **Justice** (`KEY:Sacred Judgment ~ Justice` / `KEY:Profane Judgment ~
  Justice`): "+%1 sacred [or profane] bonus on all attack rolls",
  `1+InqJudgeJusticeLVL/5`. At level 1: +1.
- **The other 7 judgment types**, why each is out of scope for this
  closure:
  - Destruction: `+%1 sacred/profane bonus on all weapon damage rolls`
    (`1+LVL/3`) -- this codebase has no damage-roll total anywhere to
    layer a bonus onto.
  - Healing: fast healing `1+LVL/3` -- needs ongoing hit-point/round-tick
    state this codebase doesn't model.
  - Resiliency: DR `1+LVL/5` (or DR/alignment past level 10) -- needs a
    damage-reduction facet that doesn't exist yet.
  - Resistance: energy resistance `2*(1+LVL/3)` against a chosen energy
    type -- needs an energy-resistance facet that doesn't exist yet.
  - Piercing, Purity, Protection, Smiting: each either opponent- or
    effect-type-dependent (bypassing specific DR/SR, bonus vs a chosen
    creature type, etc.) -- the same "genuinely unrepresented, not
    provably vacuous" bucket as Slayer's Studied Target or Monk's Deflect
    Arrows.
- **Domain**: grepped the full `KEY:Inquisitor ~ ...` list -- no "Domain"
  entry exists at all, unlike Cleric. Confirmed against known PF1 rules:
  Inquisitor domains grant spell-list access only, never a domain power.
  So there is no separate domain-power burden to name for Inquisitor
  the way Cleric's domain has one; it folds cleanly into the deferred
  spellcasting bucket instead.

## Why this is the same size/shape as Alchemist's Mutagen, not a new architecture

Judgment combines the activation-gating pattern (Barbarian/Skald/
Bloodrager Rage-shaped mechanics: a `class_ability_activations` entry,
active/inactive branches) with the choice-recognition pattern (Cleric's
domain choice / Alchemist's mutagen-stat choice: a `selected_choices`
entry naming which of several options is picked). This exact combination
was already proven once, for Alchemist's Mutagen -- this closure reuses
it, not a new interaction shape.

**Proposed combination logic** (mirrors Alchemist's Mutagen exactly): a
character with no `class_ability_activations` entry for `"judgment"`, or
one present but not `EquippedActive`, is a genuinely valid "not currently
judging" posture -- grounds an honest recognition record, no claim-block.
A character with an ACTIVE judgment activation but no recognized
`choice:inquisitor_judgment` selection naming `judgment:justice` (either
no selection, or a request for one of the other 7 unbuilt types) is a
genuine posture violation -- pronouncing judgment always requires
choosing a type first per the corpus's own sequencing -- and claim-blocks,
mirroring Alchemist's own "active but no recognized stat choice" shape.
Only a genuinely active activation PLUS the recognized Justice choice
together ground the real attack-roll bonus.

## Proposed scope

1. `is_supported_inquisitor_single_class` -- exact `ApgClassId::Inquisitor`
   match, mirroring Cavalier's/Alchemist's own gate exactly (third
   APG-side use).
2. `INQUISITOR_CLASS_ID`, `INQUISITOR_JUDGMENT_ABILITY_ID`,
   `INQUISITOR_JUDGMENT_CHOICE_ID`, `INQUISITOR_JUDGMENT_JUSTICE_SELECTION_ID`
   constants.
3. `inquisitor_judgment_uses_per_day(level) -> i16` (`1 + (level-1)/3`),
   genuinely enforced against `activation.rounds_consumed_today`,
   mirroring Rage's/Bloodrage's own enforced rounds-per-day budget
   exactly. **Correction (caught in team-lead review, 2026-07-25):** an
   earlier draft of this closure's own doc comment claimed this budget
   was informational-only, citing Rage/Bloodrage as precedent -- but a
   direct check of `ground_or_block_barbarian_rage` showed both of those
   actually DO enforce their budget with a real `rounds_exceeded`
   diagnostic. This closure was genuinely missing the same check, not
   deliberately scoping it out -- fixed before commit, with a new
   `uses_exceeded` diagnostic and test mirroring Bloodrager's own
   over-budget test exactly.
4. `inquisitor_justice_judgment_attack_bonus(level) -> i16`
   (`1 + level/5`).
5. `ground_or_block_inquisitor_judgment` mirroring
   `ground_or_block_alchemist_mutagen`'s three-branch shape (not-judging /
   active-with-recognized-Justice-choice / active-without-recognized-
   choice), applying the attack-roll bonus to the integrated baseline
   melee attack bonus (mirroring Cleric's Touch of Good's own
   self-application-only integration point at `compute_combat_baseline`,
   the closest existing precedent for a self-applied attack-roll bonus
   gated on both a choice and an activation).
6. New, narrower `class_feature.apg.inquisitor.other_features_deferred
   .unsupported` diagnostic replacing the generic one, naming
   Inquisitor's remaining named features (Bane, Cunning Initiative,
   Discern Lies, Exploit Weakness, Monster Lore, Solo Tactics, Stalwart,
   Stern Gaze, Track), the other 7 judgment types, and spellcasting
   (folding Domain in here too, since it has no separate power to name).
7. Tests mirroring the Rage-shaped/Alchemist-shaped closures' own shape
   (not-judging, actively-judging-with-recognized-Justice-choice,
   actively-claimed-but-unrecognized-choice stays blocked, spoofed-entry
   non-leak), plus the standard diagnostic-swap/positive-leak/
   negative-leak trio (now "the other 3 APG classes," Cavalier/Alchemist
   already admitted) in `apg_class_chassis_dispatch_tests`.

## What stays explicitly out of scope, named honestly

- The other 7 judgment types (Destruction, Healing, Piercing, Protection,
  Purity, Resiliency, Resistance/Smiting) -- each needs engine state
  (damage rolls, DR, energy resistance, opponent/effect-type dependency)
  that doesn't exist yet.
- Which of Sacred/Profane Judgment an inquisitor's own alignment grants
  -- the numeric value is identical either way, so this is a cosmetic
  gap only, not a correctness one.
- Inquisitor's own spellcasting (Domain spell-list access, Orisons,
  spells-known/per-day table) -- a separate, bigger partial-caster
  closure, not proposed here.
- Every other named Inquisitor feature (Bane, Cunning Initiative, Discern
  Lies, Exploit Weakness, Monster Lore, Solo Tactics, Stalwart, Stern
  Gaze, Track).
- Multiclass eligibility -- `ApgClassId::from_class_id_str` stays
  deliberately unregistered with `multiclass_class_level_supported`,
  same as every other APG class-specific closure this session.

## Reachability

Not proposing any `compose_character_input` seeding for Inquisitor as
part of this closure -- that's the separate choice-picker Path A surface
(`pf1_adapter.rs`), out of scope here. Like Cavalier/Alchemist, Inquisitor
does not reach `Computed` this slice regardless (spellcasting/other
named features stay permanently deferred), so there is no product-
reachability question to answer yet for this class -- unlike Sorcerer/
Cleric/Druid, which genuinely could reach `Computed` once their one
remaining gap closed.

## Result

Built and verified: `is_supported_inquisitor_single_class`,
`ground_or_block_inquisitor_judgment` (three-branch: not-judging /
active-with-Justice / active-without-recognized-choice),
`push_inquisitor_other_features_deferred_diagnostic`, wired into
`compute_apg_class_chassis` and `compute_combat_baseline`'s melee attack
bonus. 4 new tests in `pilot_compute::inquisitor_dispatch_widening_safety_tests`
plus 2 new dedicated tests and 2 fixed pre-existing tests in
`apg_class_chassis_dispatch_tests` (diagnostic-swap coverage, negative-
leak/positive-leak carve-outs). Full lib suite: 434/434. Both SD-24 audit
files updated and green. Desktop crate: 212/212 (unaffected, no
frontend-facing change). Inquisitor stays `Blocked` on the new, narrower
`other_features_deferred` diagnostic, same as Cavalier/Alchemist's own
shape -- no `Computed` claim made for this class this slice.

## Deepening (2026-07-26, task #3)

The "other 7 judgment types" exclusion above was re-verified directly
against the raw corpus rather than trusted as-is, and found wrong for
three of them: Protection (Armor Class bonus) and Purity (all-saves
bonus) are flat self-buffs with real live consumers already computed in
this codebase (`baseline_armor_class`, `total_saves`), not opponent/
effect-dependent at all -- the original grouping was a mistake, not a
re-derivation of new facts. Smiting (weapons count as magic for DR
bypass) is a flat, level-independent boolean fact grounded as a
standalone explanation record, mirroring Brawler's Strike DR-bypass fact.
All three now ground alongside Justice (4 of 8 judgment types); the
remaining 5 (Destruction, Healing, Piercing, Resiliency, Resistance)
genuinely still need engine state that doesn't exist. Stern Gaze (a
separate, unconditional, non-choice-gated named feature) was also
grounded onto Intimidate's total, Intimidate-half only. See
`docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_4/apg_inquisitor_coverage.md`'s
own update section and `docs/release/v0.6/risks-and-open-questions.md`
item 41 for the full record. `named_features_wired` rises from 1 to 2.
