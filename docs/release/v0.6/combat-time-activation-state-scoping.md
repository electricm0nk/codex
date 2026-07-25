# Combat-Time Activation State (Risks Item 8, Barbarian/Bard) — Scoping Plan

> Requested by the lead after Ranger/Paladin/Sorcerer/Cleric/Druid's spell
> postures landed: those five classes' remaining gap was a genuine
> *build-time* fact (which spells you know/prepared today, same shape as
> an equipment loadout), so the existing `CharacterInput` snapshot model
> fit it directly. Barbarian's rage-execution and Bard's
> bardic-performance-execution gaps are a different *kind* of fact —
> combat-round, transient state — and this doc sizes what a real fix
> needs before any code is written, per the lead's request. Monk's
> bonus-feat gap is investigated too, and found NOT to be the same shape
> (see below) — flagged rather than force-fit into this plan.

## The central finding: a real precedent exists, but it's weaker than a first read suggests

> **Correction (adversarial review, 2026-07-25)**: the first version of
> this doc overstated how "de-risked" this is. Corrected here rather than
> carried forward silently, same as every other real finding this session
> has folded back into a doc once verified.

Verified directly: `character_input.rs`'s `ActiveState` enum
(`EquippedActive` / `Absent` / `SelectedInactive`) already represents
"a combat-time-toggleable thing, declared for this specific computed
snapshot," and it is already used for a non-equipment case — Power
Attack (a feat, not an item) is a synthetic entry in
`ChosenCharacterState.equipment_selections` keyed by the literal string
`"power_attack"` (`pilot_compute.rs`'s `POWER_ATTACK_ITEM_ID`).

**But the ONLY thing this codebase does with that state today is gate on
it** — `require_active_state(input, POWER_ATTACK_ITEM_ID,
ActiveState::SelectedInactive, &mut unmet)` requires the state equal a
specific value or the whole pillar blocks. There is **zero existing code
that conditionally ADDS a bonus when a state equals `EquippedActive`/
`Active`** — every current `ActiveState` consumer is a pass/fail gate,
never a "apply this value if active" branch. Rage's STR/CON/Will bonus
and AC penalty, and Inspire Courage's attack/damage/save bonus, need that
second shape, and it has **no working precedent anywhere in this
codebase** — this is genuinely new logic, not a proven pattern being
reused. The `ActiveState` enum's three variants are still a reasonable
foundation to build on; the doc's earlier framing implying the hard part
was already solved was wrong.

The other real gap stands as originally noted: Power Attack is a pure
on/off toggle, not a limited resource. Rage and Bardic Performance are
both limited resources (`class_chassis.barbarian.rage_rounds_per_day`,
`class_chassis.bard.bardic_performance_rounds_per_day`, both already
grounded, though currently only as text inside `ComputationExplanation`
records -- see the Rounds-Per-Day Enforcement section below for why that
matters) on top of the same on/off toggle shape.

## Proposed schema addition

A new field, sibling to `equipment_selections`, with clearer naming and
one addition (a consumed-resource counter) beyond what `ActiveState`
alone provides:

```rust
// character_input.rs
pub struct ChosenCharacterState {
    // ...existing fields unchanged...
    pub class_ability_activations: Vec<ClassAbilityActivation>,
}

pub struct ClassAbilityActivation {
    /// e.g. "rage", "bardic_performance" -- same "flat compound string
    /// id" idiom as `feat:weapon_focus:weapon:longsword` elsewhere in
    /// this codebase, not a per-class enum.
    pub ability_id: String,
    /// Reuses the existing three-variant `ActiveState` enum directly --
    /// `EquippedActive` means "active for this computed snapshot"
    /// (the name is equipment-flavored from its original use; a rename
    /// to something class-neutral, e.g. `Active`/`Inactive`/`Absent`, is
    /// a real but small consideration -- see Open Questions).
    pub active_state: ActiveState,
    /// Rounds of the ability's own already-grounded rounds-per-day
    /// budget consumed so far today. `None` for abilities with no
    /// per-day budget (there are none in this pair, but the field
    /// should not assume every future ability has one).
    pub rounds_consumed_today: Option<u16>,
}
```

This is additive only (mirrors exactly how `spells_selected` and
`applied_modifiers` were added in past cycles): every existing fixture
and construction site keeps compiling and passing unmodified.

> **Correction (adversarial review)**: the first version of this doc
> said this needed a "JSON schema addition." Wrong -- `CharacterInput`/
> `ChosenCharacterState` has no serde/JSON path at all. Both fixtures and
> saved characters go through the hand-rolled `key=value` line parser
> (`character_input.rs`'s `apply_*` functions, the same grammar
> `spells_selected`/`applied_modifiers` used). The "additive, zero blast
> radius" property holds for the real reason -- a struct-literal
> `Default`-equivalent construction path plus `Vec::new()` for the new
> field, not because of a JSON schema this type doesn't have. Dropped
> from the action list below.

Real wire-contract work needed alongside the Rust type:

- Fixture-text grammar: a new `activation=<ability_id>:<state>[:<rounds>]`
  line type, mirroring `apply_spell_selection`'s existing
  last-colon-split convention.
- Tauri DTO plumbing if the frontend needs to set this (out of scope for
  a first backend-only slice -- the DTO/UI work is its own follow-on,
  matching how spell selection landed backend-first).

## Required validation: an activation must be tied to the character's real class levels

> **Added (adversarial review)**: a real gap the first draft missed
> entirely, the same shape of finding the Ranger dispatch review caught
> (a false-`Computed`/false-grounding risk, not a theoretical one).

As specced, nothing stops a non-Barbarian character from carrying a
spoofed `class_ability_activations` entry with `ability_id: "rage"` and
getting Rage's bonuses applied with nothing to reject it, **unless the
application code explicitly checks class ownership first**. This must be
a required, explicit step, not an assumption: any pillar that reads
`class_ability_activations` for a given `ability_id` must first confirm
`class_levels` actually contains the owning class (Barbarian for
`"rage"`, Bard for `"bardic_performance"`) before applying anything.
Mirrors the existing spell-posture shape exactly -- Ranger's/Paladin's/
etc. validation functions are only ever invoked from inside a
`class_levels`-gated `if let Some(...) = ... .find(|cl| cl.class_id ==
RANGER_CLASS_ID)` block, so a non-Ranger character's stray
`spells_selected` entries are never read at all, not merely rejected
after the fact. The rage/performance application code must be
structured the same way -- gate-by-construction, not a bolt-on check.

## Rounds-per-day budget: behavior and real plumbing, both currently unspecified

> **Added (adversarial review)**: the first draft named "enforcement" as
> required but never defined what enforcement means, and the budget
> values aren't queryable as real values yet.

**Behavior, now specified**: exceeding the ability's rounds-per-day
budget must produce a claim-blocking diagnostic, matching every other
over-budget check landed this session (Ranger/Paladin/Cleric/Druid's
over-prepared-slot checks, Sorcerer's over-known check) -- never silent
capping and never silently ignoring the excess. Consistent with this
codebase's no-stub-mvp honesty doctrine: an over-budget activation is a
genuine posture violation, not a value to quietly clamp.

**Real plumbing needed**: `rage_rounds_per_day`/
`bardic_performance_rounds_per_day` currently exist only as `i16` values
interpolated into `ComputationExplanation.detail` text -- there is no
function returning the number as a real, callable Rust value today. The
correct fix mirrors the `_table`-extraction pattern used for every spell
math ladder this session (`ranger_base_spells_per_day_table`,
`cleric_base_spells_per_day_table`, etc.): pull the existing inline
formula (`4 + constitution_modifier + 2 * (level - 1)` for Rage) out into
a small pure function both the existing explanation-push code and the
new validation logic call directly, rather than either duplicating the
formula a second time or parsing it back out of explanation text.

## Which already-integrated pillars this touches, and how

Unlike the spell-posture slices (which only ever added NEW explanation/
diagnostic records, never touched an existing integrated total), this
concept requires **conditionally modifying pillars that are already
real and already tested**:

| Pillar | Barbarian (Rage) | Bard (Inspire Courage) |
|---|---|---|
| `compute_ability_modifiers` | +4/+6/+8 STR, +4/+6/+8 CON (tiered by Greater/Mighty Rage, already grounded as flat constants) applied only while `active_state == Active` | not touched |
| `compute_total_saves` | +2/+3/+4 Will (same tiering) | +1/+2/+3/+4 competence bonus on saves vs. fear (tiered, already grounded as `inspire_courage_bonus`) |
| `compute_combat_baseline` (AC) | -2 penalty while raging | not touched |
| `compute_combat_baseline` (attack/damage) | not touched (Rage doesn't bonus attack/damage directly in PF1 core) | +1/+2/+3/+4 competence bonus on attack and weapon damage rolls |
| Round-budget enforcement | consumed rounds vs. `rage_rounds_per_day` | consumed rounds vs. `bardic_performance_rounds_per_day` |
| Post-use state | Fatigued after rage ends (below Tireless Rage) -- a SECOND conditional state this codebase has no representation for at all yet | none (Bardic Performance has no post-use penalty) |

Each of these four compute functions currently has zero knowledge of
`class_ability_activations` and would need a new, explicit "if an active,
in-budget activation exists for this ability, add its grounded
constant(s)" branch — real, testable logic, not a stub, but genuinely new
surface in functions this session has otherwise only ever read from, not
modified.

> **Correction (adversarial review)**: `compute_ability_modifiers`
> (`pilot_compute.rs:4878`) takes `scores: &AbilityScores` only, not
> `CharacterInput` -- Rage's STR/CON bonus cannot land inside it as
> originally specced without a signature change (one call site,
> `pilot_compute.rs:4615`, so mechanically small either way -- but the
> plan should say so explicitly rather than imply a body-only edit).
>
> On reflection, threading `input` into `compute_ability_modifiers`
> itself is not the recommended shape: `compute_total_saves` already
> takes `input: &CharacterInput` directly and layers
> `feat_effects::save_bonuses_from_feats`'s own separately-computed
> result onto its base total (`pilot_compute.rs:19591-19597`) rather than
> baking feat bonuses into a lower-level modifier function. Recommend the
> same shape for Rage: a new, small `apply_rage_ability_bonuses(base:
> AbilityModifiers, input: &CharacterInput, explanations) ->
> AbilityModifiers` function, called immediately after
> `compute_ability_modifiers` returns at its one call site. This leaves
> the widely-used base function (called for every class, every request)
> completely untouched, and mirrors an idiom this codebase already
> trusts rather than inventing a new one.

**Barbarian's fatigue state is a second, smaller open question**: PF1
Rage causes fatigue for 2× the rounds raged, once fatigue-causing runs
out. Representing "is this character currently fatigued as a rage
aftereffect" is a related but separate transient-state fact this schema
doesn't yet cover, and fatigue itself has real mechanical effects
(-2 STR, -2 DEX, no run/charge) that would need their own conditional
application once grounded. Deferring this is fine as documented (honest,
not silent) -- **decision (adversarial review feedback): add a
non-blocking diagnostic** noting Fatigue isn't modeled while Rage is
active, the same "explanation-only, no fabricated mechanical value"
idiom used throughout every other class this session (e.g. every
"grant-only identity record, no execution engine" note). Cheap, keeps
the gap honest rather than silent, and costs nothing to add alongside
the rest of Barbarian's cycle.

## Monk's bonus-feat gap is NOT the same shape — investigated, not force-fit

`class_feature.monk.bounded_progression.bonus_feat.unsupported` names 7
restricted-list feats (Catch Off-Guard, Combat Reflexes, Deflect Arrows,
Dodge, Improved Grapple, Scorpion Style, Throw Anything). Checked each
against the activation-state model above:

- **Combat Reflexes** (bonus attacks of opportunity) is a passive, always-on
  bonus -- no activation state needed at all, just a flat number and an
  AoO-counting engine that doesn't exist.
- **Dodge** — **correction (adversarial review): this dismissal was
  factually wrong.** Dodge is already fully grounded in this codebase:
  an unconditional flat `+1` AC (`DODGE_AC_BONUS`,
  `pilot_compute.rs:19904`), gated only by `selected_feats` containing
  `DODGE_FEAT_ID` (`pilot_compute.rs:19959-19961`) -- verified directly,
  not assumed. PF1 Core Rulebook's own Dodge is untargeted (no
  "designated opponent" concept at all; that's 3.5e's version), so no
  target-selection concept exists or is needed here. Dodge is likely the
  **easiest** of Monk's 7 feats, not a harder one -- worth noting for
  whenever Monk's own scoping pass happens.
- **Deflect Arrows**, **Improved Grapple**, **Scorpion Style**, **Throw
  Anything**, **Catch Off-Guard** each require their own attack-resolution,
  grapple-check, or weapon-proficiency engine pieces that don't exist
  regardless of activation state.

Monk's real gap is "no general feat-effect/prerequisite engine exists,"
not "no activation-state schema exists" -- confirmed by direct
inspection, not assumed from the class-breadth doc's original one-line
characterization. It belongs in its own future scoping pass (likely
per-feat, since the 7 feats don't share one mechanical shape), not this
one. Feat-effects engine precedent (Toughness's flat +3 HP,
`feat_effects.rs`) is the closer analog to reuse for whichever of these 7
feats gets tackled first, not the activation-state concept above.

## Sequencing recommendation

1. **Schema first**: `ClassAbilityActivation` type + fixture-grammar line,
   following the exact discipline `spells_selected` used (additive,
   zero-blast-radius on existing fixtures, its own RED→GREEN round-trip
   tests in `character_input.rs` before any consumer exists). Includes
   the rounds-per-day pure-function extraction and the class-ownership
   validation gate as required parts of this first step, not deferred to
   Barbarian's own cycle.
2. **Barbarian rage next**: the more self-contained of the two (one
   ability, three already-grounded constant tiers, one already-grounded
   rounds-per-day budget, no target-selection concept needed). Leaves
   fatigue explicitly out of scope for this first cycle, named as a
   known follow-on.
3. **Bard's Inspire Courage after**: reuses the identical schema and the
   identical "conditionally add to combat_baseline/total_saves" shape,
   proven by Barbarian's cycle -- should be meaningfully cheaper the
   second time, matching how Ranger's spell-posture pattern got cheaper
   for Paladin/Sorcerer/Cleric/Druid.
4. **Countersong, Distraction, Versatile Performance, Soothing
   Performance, Frightening Tune, Inspire Heroics, Deadly Performance**
   (Bard's other named-but-unexecuted performance types) stay out of
   scope for the first Bard cycle -- Inspire Courage alone is the
   pillar-touching piece; the rest are additional performance TYPES that
   would reuse the same activation/round-tracking mechanism once it
   exists, each with its own distinct effect to ground.
5. **Monk, Cavalier's Challenge (APG), and any other combat-time-shaped
   gap** get their own separate scoping once actually started -- noted
   here as likely future parallels, not pre-scoped.

## Open questions for the lead

- **Naming**: keep `ActiveState`'s existing variant names
  (`EquippedActive`/`Absent`/`SelectedInactive`, equipment-flavored) for
  reuse across both equipment and class-ability activations, or
  introduce a class-neutral duplicate enum with the same three variants?
  Reusing directly is less new surface; a rename risks a large mechanical
  diff across every existing equipment-selection call site for a purely
  cosmetic gain.
- **Where Power Attack's existing hack lives**: worth a follow-up
  decision on whether to migrate Power Attack's synthetic
  `equipment_selections` entry into the new, properly-named
  `class_ability_activations` list once it exists (Power Attack is a
  feat activation, not equipment, and arguably belongs there too) --
  not required for Barbarian/Bard to land, but worth deciding rather
  than leaving two mechanisms for the same concept indefinitely.
- **Scale of adversarial review**: per the lead's own note, this sets the
  pattern for at least 2 classes' combat-time mechanics (Barbarian, Bard),
  likely more later (Cavalier). Recommend one review pass on this plan
  before Barbarian's implementation starts, none required for Bard's
  follow-on cycle once Barbarian's pattern is proven -- same "review the
  pattern-setter, not every repetition" discipline used for the Ranger
  dispatch-widening review.
