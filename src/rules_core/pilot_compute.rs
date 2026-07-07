//! GE-06 pilot deterministic rules-core computation surface.
//!
//! Computes and explains the bounded set of outputs accumulated across the GE-06
//! pilot slices for the accepted PF1 Human Fighter level-1 deterministic input:
//!
//! - ability modifiers (`floor(score/2) - 5`)
//! - Fighter level-1 base attack bonus
//! - Fighter level-1 base saves
//! - baseline melee attack bonus for the deterministic Longsword loadout
//! - baseline armor class for the deterministic Chain Shirt / Dodge / no-shield posture
//! - total Fortitude / Reflex / Will saves (base save + relevant ability modifier)
//!
//! Each computed value carries a machine-checkable explanation record. This is
//! intentionally not a full rules engine: it does not compute feat-, item-, or
//! condition-based save modifiers, weapon damage, active Power Attack math,
//! initiative, skill modifiers, armor-check penalties, equipment effects beyond
//! the deterministic baseline, feat prerequisites, or any oracle parity. Support
//! is the bounded deterministic Human Fighter posture widened across the SD13-E3
//! milestone tranche from level 1 to levels 2 and 3 only: the level-2 bonus-feat
//! progression seam and the level-3 armor-training seam are surfaced explicitly,
//! but nothing here grounds level-4+ Fighter burden, a general feat-effect engine,
//! spellcasting, multiclassing, or non-Fighter positive support. The SD13-E3-F6 slice
//! additionally recognizes the deterministic Human Paladin level-1 and Human Ranger
//! level-1 hybrid chassis as direct runtime evidence, but keeps both explicitly
//! claim-blocked on their still-missing non-spell class-feature burden and later spell
//! burden; it grounds no hybrid class-feature or spell math. The SD13-E4-F7 slice
//! also recognizes the deterministic Human Sorcerer level-1 spell-bearing identity as
//! a direct runtime evidence, but keeps it explicitly claim-blocked on its
//! bloodline and spontaneous known-spell / slot posture burdens; it grounds no
//! bloodline power and no spell math. The SD13-E4-R3 slice further recognizes the
//! deterministic Human Wizard level-1 prepared arcane spell-bearing identity as
//! direct runtime evidence, but keeps it explicitly claim-blocked on its school
//! specialization burden and prepared spellbook / spells-prepared / spell-slot
//! posture burden; it grounds no spellbook content, no spells prepared, no spell
//! slots, no spell save DCs, no bonus spells, no school-opposition bookkeeping, and
//! no specialty school bonus. Unsupported input yields claim-blocking diagnostics
//! and withheld explanations rather than fabricated values.

use super::character_input::{AbilityScores, ActiveState, CharacterInput, SkillAllocation};

/// Result of the GE-06 pilot deterministic compute surface, accumulating the
/// base chassis, baseline combat, and total-save outputs proven across slices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotBaseChassisComputation {
    pub ability_modifiers: AbilityModifiers,
    /// Class/base attack bonus only. Zero when the chassis is unsupported.
    pub base_attack_bonus: i16,
    /// Class/base save bonuses only (no ability modifiers added to these).
    pub base_saves: BaseSaves,
    /// Baseline melee attack bonus for the deterministic Longsword loadout. Zero
    /// when the required deterministic combat posture is absent or unsupported.
    pub baseline_melee_attack_bonus: i16,
    /// Baseline armor class for the deterministic Chain Shirt / Dodge / no-shield
    /// posture. Zero when that posture is absent or unsupported.
    pub baseline_armor_class: i16,
    /// Total saving throws (Fighter base save + relevant ability modifier). Zero
    /// when the Fighter level-1 chassis is absent or unsupported.
    pub total_saves: BaseSaves,
    /// Selected deterministic Climb / Intimidate / Swim skill modifiers. All zero
    /// when the deterministic selected-skill or Chain Shirt posture is absent or
    /// widened beyond this slice.
    pub selected_skill_modifiers: SelectedSkillModifiers,
    pub explanations: Vec<ComputationExplanation>,
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Ability modifiers derived from chosen ability scores via `floor(score/2) - 5`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbilityModifiers {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

/// Base save bonuses from the grounded class chassis row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaseSaves {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Selected deterministic skill modifiers bounded to the GE-06 pilot slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectedSkillModifiers {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

/// A machine-checkable record explaining why a single computed value exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationExplanation {
    /// Stable id (e.g. `ability_modifier.strength`, `class_chassis.base_attack_bonus`).
    pub id: String,
    /// The computed value this record explains.
    pub value: i16,
    /// Human-auditable detail referencing the source input and formula.
    pub detail: String,
}

/// A diagnostic that blocks downstream claims when an input is unsupported here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationDiagnostic {
    /// Stable id for the diagnostic subject (e.g. `class_chassis.unsupported`).
    pub id: String,
    pub message: String,
    pub claim_blocking: bool,
}

const FIGHTER_CLASS_ID: &str = "class:fighter";

// SD13-E3-F6 hybrid chassis baseline identities. Paladin and Ranger are hybrid
// (martial + later spellcasting) classes; this slice recognizes only their bounded
// single-class level-1 chassis as direct runtime evidence and grounds no class-feature
// or spell math for either.
const PALADIN_CLASS_ID: &str = "class:paladin";
const RANGER_CLASS_ID: &str = "class:ranger";
const HYBRID_BASELINE_LEVEL: u8 = 1;

// SD13-E4-F7 spell-bearing baseline identity. Sorcerer is a spontaneous full arcane
// caster; this slice recognizes only its bounded single-class level-1 identity as direct
// runtime evidence and grounds no bloodline power and no spell math (spell slots, spells
// known, spell DCs, bonus spells, or prepared posture) for it.
const SORCERER_CLASS_ID: &str = "class:sorcerer";
const SORCERER_BASELINE_LEVEL: u8 = 1;

// SD13-E4-F7 spell-bearing baseline identity. Bard is a spontaneous arcane caster with a
// distinct chassis-class-feature burden (Bardic Knowledge and Bardic Music); this slice
// recognizes only its bounded single-class level-1 identity as direct runtime evidence and
// grounds no Bardic Knowledge check resolution, no Bardic Music / Inspire Courage execution,
// and no spell math (spells known, spells per day, spell DCs, bonus spells, school choice, or
// prepared posture) for it.
const BARD_CLASS_ID: &str = "class:bard";
const BARD_BASELINE_LEVEL: u8 = 1;

// Grounded SD13-E4-R3 Human Wizard level-1 prepared arcane spell-bearing baseline
// identities. The Wizard class is the canonical PF1 prepared arcane full caster;
// its class identity differs from Sorcerer in two ways that this bounded slice
// surfaces explicitly: the prepared posture (spellbook + spells prepared per day +
// spell slots per day) and the school specialization (one school chosen, two
// opposed schools locked, specialty school bonus at later levels).
const WIZARD_CLASS_ID: &str = "class:wizard";
const WIZARD_BASELINE_LEVEL: u8 = 1;

// SD13-E3 martial chassis baseline identity. Barbarian is a non-spell pure
// martial class; this slice recognizes only its bounded single-class level-1
// identity as direct runtime evidence and grounds no base-attack / base-save
// progression, no fast-movement +10 ft. speed extension, no illiteracy trait
// engine, no rage execution, no weapon familiarity, and no level-2+ martial
// progression.
const BARBARIAN_CLASS_ID: &str = "class:barbarian";
const MARTIAL_BASELINE_LEVEL: u8 = 1;

// SD13-E3 martial chassis baseline identity, mirroring the Barbarian pattern. Monk
// is a non-spell pure martial class with a distinct four-pillar bounded burden; this
// slice recognizes only its bounded single-class level-1 identity as direct runtime
// evidence and grounds no base-attack / base-save progression, no unarmed strike
// damage die, no Flurry of Blows execution, no AC Bonus computation, no level-1
// bonus feat grant, no ki pool, and no level-2+ martial progression.
const MONK_CLASS_ID: &str = "class:monk";

// Grounded SD13-E4 Human Cleric level-1 prepared divine spell-bearing baseline
// identity. Cleric is the canonical PF1 prepared divine full caster; unlike the
// arcane Sorcerer/Wizard/Bard baselines already recognized, its bounded burden
// is split across a domain / channel energy class-feature family (two domains
// chosen, domain spells, domain powers, channel energy) and a prepared divine
// spell posture family (spells prepared from the full Cleric list, spontaneous
// cure/inflict conversion, spell slots per day, bonus spells from a high Wisdom,
// spell save DCs).
const CLERIC_CLASS_ID: &str = "class:cleric";
const CLERIC_BASELINE_LEVEL: u8 = 1;

// Grounded SD13-E4 Human Druid level-1 prepared divine spell-bearing baseline
// identity. Druid is a prepared divine caster whose bounded burden splits across
// a nature bond / wild empathy class-feature family (nature bond choice between
// an animal companion and a domain, nature sense, wild empathy) and a prepared
// divine spell posture family (spells prepared from the full Druid list,
// spontaneous summon nature's ally conversion, spell slots per day, bonus spells
// from a high Wisdom, spell save DCs).
const DRUID_CLASS_ID: &str = "class:druid";
const DRUID_BASELINE_LEVEL: u8 = 1;


// Grounded Human pilot race seam identities. These name the already-accepted
// deterministic Human selections; this slice makes their pressure explicit but
// grounds no non-Human race semantics and no broader Human racial trait burden.
const HUMAN_RACE_ID: &str = "race:human";
const HUMAN_ABILITY_BONUS_CHOICE_ID: &str = "choice:human_ability_bonus";
const HUMAN_BONUS_FEAT_CHOICE_ID: &str = "choice:human_bonus_feat";
const ABILITY_SELECTION_PREFIX: &str = "ability:";

// SD13-E6-F3a Human racial trait bundle (size, speed, senses, extra skill ranks).
// These name the remaining Human racial trait burden explicitly, classified
// against PF1 Core Rulebook Standard Human racial traits (source evidence only,
// not oracle-checked parity):
//   cr_races.lst race:human SIZE:MEDIUM        -> Medium size category
//   cr_races.lst race:human GAIT:WALK|30       -> 30 ft base land speed
//   cr_races.lst race:human                   -> no special senses (PCGen races
//                                                in the CRB only carry the SENSE
//                                                tag when a sense bonus exists;
//                                                Human has none for Standard Human)
//   cr_races.lst race:human BONUS:SKILL|...   -> 4 extra skill points at 1st
//                                                level and 1 extra skill rank
//                                                per level thereafter
//
// This constant set deliberately names the entire PF1 Standard Human racial
// trait surface — every line a Player's Handbook Human racial entry lists —
// so the explanation records can name each dimension explicitly instead of
// leaving it an incidental side-effect or a folklore claim.
//
// None of these ground a computed mechanical contribution to the existing
// NumericOutputs in this slice. They explain Human identity only; the chassis
// totals remain controlled by the bounded deterministic posture.
const HUMAN_SIZE_CATEGORY: &str = "Medium";
const HUMAN_BASE_SPEED_FEET: i16 = 30;
const HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1: u8 = 4;
const HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL: u8 = 1;

// Grounded deterministic combat-baseline contributors and posture identities.
const LONGSWORD_ITEM_ID: &str = "item:longsword";
const CHAIN_SHIRT_ITEM_ID: &str = "item:chain_shirt";
const SHIELD_ITEM_ID: &str = "item:shield";
const POWER_ATTACK_ITEM_ID: &str = "power_attack";
const DODGE_FEAT_ID: &str = "feat:dodge";
const WEAPON_FOCUS_FEAT_ID: &str = "feat:weapon_focus";
const FIGHTER_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat";
const WEAPON_FOCUS_LONGSWORD_SELECTION: &str = "feat:weapon_focus:weapon:longsword";

// SD13-E5-F9 canonical Human Fighter feat-choice seam. These name the exact accepted
// deterministic feat-choice selections on the level-1/2/3 seam. This slice preserves
// these selections and claim-blocks any deviation of the named slots; it grounds no
// general feat-effect or prerequisite engine and no alternative feat legality.
const LEVEL_1_CHARACTER_FEAT_CHOICE_ID: &str = "choice:level_1_character_feat";
const POWER_ATTACK_FEAT_SELECTION: &str = "feat:power_attack";
const TOUGHNESS_FEAT_SELECTION: &str = "feat:toughness";

// Grounded numeric contributors (source evidence only; not oracle-checked parity):
//   cr_equip_arms_armor.lst:40  Chain Shirt -> BONUS:COMBAT|AC|4|TYPE=Armor, MAXDEX:4
//   cr_feats.lst:53             Dodge       -> BONUS:COMBAT|AC|1|TYPE=Dodge
//   cr_feats.lst:184            Weapon Focus-> +1 to-hit with the selected weapon
const ARMOR_CLASS_BASE: i16 = 10;
const CHAIN_SHIRT_ARMOR_BONUS: i16 = 4;
const CHAIN_SHIRT_MAX_DEX: i16 = 4;
const DODGE_AC_BONUS: i16 = 1;
const WEAPON_FOCUS_TO_HIT_BONUS: i16 = 1;

// Grounded selected-skill contributors (source evidence only; not oracle-checked):
//   cr_skills.lst:10   Climb      -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Climb|3|TYPE=ClassSkill
//   cr_skills.lst:42   Intimidate -> KEYSTAT:CHA (no ACHECK), BONUS:SKILL|Intimidate|3|TYPE=ClassSkill
//   cr_skills.lst:102  Swim       -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Swim|3|TYPE=ClassSkill
//   cr_abilities_class.lst:2835   Fighter class skills include Climb, Intimidate, Swim
//   cr_equip_arms_armor.lst:40    Chain Shirt -> ACCHECK:-2
const CLIMB_SKILL_ID: &str = "skill:climb";
const INTIMIDATE_SKILL_ID: &str = "skill:intimidate";
const SWIM_SKILL_ID: &str = "skill:swim";
const SELECTED_SKILL_RANK: u8 = 1;
const CLASS_SKILL_BONUS: i16 = 3;
const CHAIN_SHIRT_ARMOR_CHECK_PENALTY: i16 = -2;

// Bounded SD13-E3 Fighter milestone widening. The accepted level-1 pilot is now
// joined by levels 2 and 3 only. Nothing here grounds level 4+ Fighter burden,
// repeated bonus-feat cadence, weapon training, later armor-training ranks, or any
// non-Fighter positive support.
const MAX_SUPPORTED_FIGHTER_LEVEL: u8 = 3;

// Fighter level-2 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 2; this slice surfaces the named selection as an explicit seam only
// and grounds no general feat-effect or prerequisite engine.
const FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_2";

// Fighter armor training 1, gained at level 3. It reduces the worn armor's
// armor-check penalty by 1 (to a minimum of 0) and raises its maximum Dexterity
// bonus by 1. Grounded from cr_abilities_class.lst Fighter armor training; not
// oracle-checked parity.
const FIGHTER_ARMOR_TRAINING_1_LEVEL: u8 = 3;
const ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION: i16 = 1;
const ARMOR_TRAINING_1_MAX_DEX_INCREASE: i16 = 1;

/// Simple integrated status for the GE-06 pilot headless receipt: whether the
/// path produced computed evidence or is blocked. This distinguishes evidence
/// from a blocker posture; it is not an oracle-checked parity verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadlessReceiptStatus {
    /// The integrated deterministic path produced computed evidence with no
    /// claim-blocking diagnostics.
    Computed,
    /// The integrated deterministic path is blocked; at least one claim-blocking
    /// diagnostic is present and no success state is fabricated.
    Blocked,
}

/// One bounded, library-first, headless receipt for the accepted deterministic
/// GE-06 pilot path. It preserves case and source-package identity, a simple
/// computed/blocked status, and the full underlying computation (already-grounded
/// outputs, explanations, and diagnostics) for later parity or UI consumers.
///
/// This is headless computed evidence only; it must not be relabeled as
/// oracle-checked parity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotHeadlessReceipt {
    /// Case identity carried from the loaded input (absent when the input names none).
    pub case_id: Option<String>,
    /// Source package identity carried from the loaded input.
    pub source_package_id: String,
    /// Whether the integrated path produced evidence or is blocked.
    pub status: HeadlessReceiptStatus,
    /// The underlying pilot computation, preserving the already-grounded outputs,
    /// explanation records, and claim-blocking diagnostics unchanged.
    pub computation: PilotBaseChassisComputation,
}

/// Build the GE-06 pilot headless receipt from a loaded character input.
///
/// This runs the existing deterministic compute surface and wraps it in one
/// bounded receipt shape, deriving the integrated status from the computation's
/// claim-blocking diagnostics: any claim-blocking diagnostic blocks the path,
/// otherwise the path is computed. It adds no new computed value, fabricates no
/// success state, and discards none of the existing explanations or diagnostics.
pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt {
    let computation = compute_pilot_base_chassis(input);

    let status = if computation.diagnostics.iter().any(|d| d.claim_blocking) {
        HeadlessReceiptStatus::Blocked
    } else {
        HeadlessReceiptStatus::Computed
    };

    PilotHeadlessReceipt {
        case_id: input.case_id.clone(),
        source_package_id: input.source_package_id.clone(),
        status,
        computation,
    }
}

/// Compute the GE-06 pilot base chassis from a loaded character input.
pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation {
    let mut explanations = Vec::new();
    let mut diagnostics = Vec::new();

    let ability_modifiers =
        compute_ability_modifiers(&input.chosen.ability_scores, &mut explanations);

    let (base_attack_bonus, base_saves) =
        compute_fighter_chassis(input, &mut explanations, &mut diagnostics);

    let (baseline_melee_attack_bonus, baseline_armor_class) = compute_combat_baseline(
        input,
        &ability_modifiers,
        base_attack_bonus,
        &mut explanations,
        &mut diagnostics,
    );

    let total_saves = compute_total_saves(
        input,
        &ability_modifiers,
        &base_saves,
        &mut explanations,
        &mut diagnostics,
    );

    let selected_skill_modifiers = compute_selected_skill_modifiers(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_fighter_class_features(input, &mut explanations);

    explain_hybrid_level1_chassis(input, &mut explanations, &mut diagnostics);
    explain_barbarian_level1_chassis(input, &mut explanations, &mut diagnostics);
    explain_monk_level1_chassis(input, &mut explanations, &mut diagnostics);


    // SD13-E3/E4 Paladin-only decomposition: split the F6 hybrid class-feature
    // and spell-burden blockers into per-burden diagnostics so the chassis
    // burden is separable from the partial-caster spell burden on the runtime
    // path. This is an extension, never a downgrade, of the F6 surface.
    explain_paladin_level1_chassis_and_spell_burden_separation(
        input,
        &mut explanations,
        &mut diagnostics,
    );

    explain_sorcerer_level1_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_wizard_level1_prepared_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_cleric_level1_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_druid_level1_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_bard_level1_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_human_race_seam(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_human_trait_bundle(input, &mut explanations, &mut diagnostics);

    validate_fighter_feat_choice_legality(input, &mut diagnostics);

    PilotBaseChassisComputation {
        ability_modifiers,
        base_attack_bonus,
        base_saves,
        baseline_melee_attack_bonus,
        baseline_armor_class,
        total_saves,
        selected_skill_modifiers,
        explanations,
        diagnostics,
    }
}

fn compute_ability_modifiers(
    scores: &AbilityScores,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let abilities = [
        ("strength", scores.strength),
        ("dexterity", scores.dexterity),
        ("constitution", scores.constitution),
        ("intelligence", scores.intelligence),
        ("wisdom", scores.wisdom),
        ("charisma", scores.charisma),
    ];

    let mut modifiers = AbilityModifiers::default();
    for (ability, score) in abilities {
        let modifier = ability_modifier(score);
        explanations.push(ComputationExplanation {
            id: format!("ability_modifier.{ability}"),
            value: modifier,
            detail: format!(
                "{ability} ability modifier from chosen score {score}: floor({score} / 2) - 5 = {modifier}"
            ),
        });
        assign_modifier(&mut modifiers, ability, modifier);
    }

    modifiers
}

/// Pathfinder ability modifier: `floor(score / 2) - 5`. `div_euclid` gives true
/// floor division so negative scores would round down rather than toward zero.
fn ability_modifier(score: i16) -> i16 {
    score.div_euclid(2) - 5
}

fn assign_modifier(modifiers: &mut AbilityModifiers, ability: &str, modifier: i16) {
    match ability {
        "strength" => modifiers.strength = modifier,
        "dexterity" => modifiers.dexterity = modifier,
        "constitution" => modifiers.constitution = modifier,
        "intelligence" => modifiers.intelligence = modifier,
        "wisdom" => modifiers.wisdom = modifier,
        "charisma" => modifiers.charisma = modifier,
        _ => unreachable!("ability set is fixed and fully matched"),
    }
}

/// Make the already-grounded Human pilot race seam explicit instead of leaving it an
/// incidental side effect of the numeric outputs.
///
/// This adds no new computed mechanic and no new input surface. It derives strictly
/// from existing chosen input — the `race:human` identity and the named
/// `choice:human_ability_bonus` and `choice:human_bonus_feat` selections — and from the
/// already-computed deterministic outputs — the ability modifiers and the grounded
/// Dodge armor-class contribution. It thereby surfaces the named Human ability-bonus and
/// bonus-feat interaction pressure as legible explanation records.
///
/// Non-Human races receive only a bounded, non-claim-blocking note that their race
/// semantics remain unverified; this slice grounds no non-Human race truth and no
/// broader Human racial trait burden (size, speed, senses, extra skill ranks).
fn explain_human_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        diagnostics.push(ComputationDiagnostic {
            id: "race.semantics.unverified".to_owned(),
            message: format!(
                "race semantics are grounded only for {HUMAN_RACE_ID} on the deterministic pilot seam; \
                 chosen race {} has no grounded race semantics in this slice",
                input.chosen.race_id
            ),
            claim_blocking: false,
        });
        return;
    }

    // Human ability-bonus interaction: the named choice targets one ability. Surface its
    // pressure through the already-computed modifier for exactly that ability.
    if let Some(selection) = choice_selection(input, HUMAN_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.human.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Human ability-bonus selection ({HUMAN_ABILITY_BONUS_CHOICE_ID} -> {selection}) targets \
                 {ability}; the chosen {ability} score yields modifier {modifier:+}"
            ),
        });
    }

    // Human bonus-feat interaction: the named choice grants a feat. Surface the grounded
    // Dodge armor-class contribution the deterministic baseline already relies on.
    if let Some(selection) = choice_selection(input, HUMAN_BONUS_FEAT_CHOICE_ID) {
        let (value, detail) = if selection == DODGE_FEAT_ID {
            (
                DODGE_AC_BONUS,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) grants Dodge, \
                     the deterministic Dodge feat contributing {DODGE_AC_BONUS:+} to the baseline armor class"
                ),
            )
        } else {
            (
                0,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) is a named Human \
                     bonus feat, but only the deterministic Dodge grant has a grounded computed contribution"
                ),
            )
        };
        explanations.push(ComputationExplanation {
            id: "race.human.bonus_feat_grant".to_owned(),
            value,
            detail,
        });
    }

    // Bounded honesty: only the named seam is grounded. This is explicit but
    // non-claim-blocking so the deterministic pilot still reports computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.human.bounded_semantics".to_owned(),
        message: "Human race semantics are grounded for the deterministic pilot's named \
                  ability-bonus and bonus-feat selections, and the SD13-E6-F3a trait bundle \
                  (size, speed, senses, extra skill ranks) is classified explicitly; the \
                  remaining PF1 Standard Human racial trait surface (alternate Human racial \
                  traits, variant Humans, half-Human heritages, and any ruleset-level effects \
                  outside the named deterministic pilot) remains unverified"
            .to_owned(),
        claim_blocking: false,
    });
}

/// SD13-E6-F3a Human racial trait bundle explanation seam.
///
/// Surfaces each remaining PF1 Standard Human racial trait dimension (size,
/// speed, senses, extra skill ranks) as an explicit `ComputationExplanation`
/// record so the trait bundle is legible on the runtime path rather than left
/// as an incidental side-effect or a folklore claim. Three of the four
/// dimensions carry the grounded PF1 source value as a recognition record;
/// the senses dimension carries a bounded "no special senses" classification
/// because PF1 Standard Human grants no special sense bonus.
///
/// This function:
///   - runs only when `race_id == race:human`; non-Human races stay on the
///     existing `race.semantics.unverified` diagnostic from
///     `explain_human_race_seam`,
///   - adds no new computed mechanical contribution; each record carries the
///     grounded source value as recognition and contributes nothing to the
///     chassis totals, selected-skill modifiers, combat baseline, or AC,
///   - replaces the previous "Human size, speed, senses, extra skill ranks
///     remain unverified" non-claim-blocking note from
///     `race.human.bounded_semantics` with explicit per-dimension records,
///   - is bounded to the deterministic Human Fighter level-1/2/3 pilot
///     posture implicitly via the caller; it deliberately grounds no other
///     Human racial variant (alternate Human racial traits, variant Humans,
///     half-Humans), no other race, and no PF1 alternate ruleset.
fn explain_human_trait_bundle(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // ----- size -----
    // Recognition record only; carries the grounded Human size category name
    // as the recognition value so the explanation reads as the humanoid
    // identity rather than fabricating a numeric contribution.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Human racial trait bundle — size: PF1 Standard Human is {HUMAN_SIZE_CATEGORY} size \
             (cr_races.lst race:human SIZE:MEDIUM). This is a bounded recognition record naming \
             the Human size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    // Recognition record for the 30 ft base land speed. The bounded
    // selected-skill and combat baselines never consult base speed, so this
    // record is identity-only — no computed speed-derived value is fabricated.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.speed".to_owned(),
        value: HUMAN_BASE_SPEED_FEET,
        detail: format!(
            "Human racial trait bundle — speed: PF1 Standard Human has a base land speed of \
             {HUMAN_BASE_SPEED_FEET} ft (cr_races.lst race:human GAIT:WALK|{HUMAN_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the human base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Bounded "no special senses" classification. PF1 Standard Human grants
    // no special senses (darkvision, low-light, scent, etc.), so this
    // dimension is classified explicitly as no-effect rather than a silent
    // omission or a fabricated sense bonus.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.senses".to_owned(),
        value: 0,
        detail: format!(
            "Human racial trait bundle — senses: PF1 Standard Human grants no special senses \
             (cr_races.lst race:human carries no SENSE tag for Standard Human; darkvision, \
             low-light vision, scent, and other sense bonuses are absent). This is a bounded \
             no-effect classification record on the deterministic pilot seam; it carries no \
             fabricated sense bonus and contributes no computed value (+0)"
        ),
    });

    // ----- extra skill ranks -----
    // Recognition record for the extra-skill-ranks Human trait. PF1 Standard
    // Human grants 4 extra skill points at 1st level and 1 extra skill rank
    // per additional level thereafter; this slice surfaces both numbers as a
    // recognition record and explicitly does not propagate them through the
    // bounded selected-skill modifier computation (which controls the
    // deterministic Climb / Intimidate / Swim rank-1 posture only).
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.extra_skill_ranks".to_owned(),
        value: i16::from(HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL),
        detail: format!(
            "Human racial trait bundle — extra skill ranks: PF1 Standard Human gains \
             {HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1} extra skill points at 1st level and \
             {HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL} extra skill rank per additional level thereafter \
             (cr_races.lst race:human BONUS:SKILL|...). The recognition value \
             ({HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL:+}) carries the per-additional-level extra-rank \
             identity on the deterministic pilot seam; this slice does not propagate these \
             extra skill points/rank through the bounded Climb/Intimidate/Swim rank-1 selected \
             skill-modifier computation, so the bounded fighter-posture skill totals remain \
             grounded by the canonical rank-1 posture rather than by the unbounded Human extra \
             skill-rank rule"
        ),
    });
}

/// Return the selection id chosen for the named choice set, if present.
fn choice_selection<'a>(input: &'a CharacterInput, choice_set_id: &str) -> Option<&'a str> {
    input
        .chosen
        .selected_choices
        .iter()
        .find(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
}

/// Look up the already-computed modifier for a named ability. Unknown ability names
/// contribute nothing rather than fabricating a value.
fn ability_modifier_for(modifiers: &AbilityModifiers, ability: &str) -> i16 {
    match ability {
        "strength" => modifiers.strength,
        "dexterity" => modifiers.dexterity,
        "constitution" => modifiers.constitution,
        "intelligence" => modifiers.intelligence,
        "wisdom" => modifiers.wisdom,
        "charisma" => modifiers.charisma,
        _ => 0,
    }
}

/// The bounded Fighter milestone level this surface grounds, if any. Returns the
/// single Fighter level when the chosen input is exactly a single-class Fighter at
/// one of the supported milestone levels (1, 2, or 3). Returns `None` for no
/// Fighter, a non-Fighter class, a multiclass mix, or a level-4+ Fighter this slice
/// does not yet ground — each of which stays claim-blocked as before.
fn supported_fighter_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == FIGHTER_CLASS_ID
                && (1..=MAX_SUPPORTED_FIGHTER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Fighter armor-training profile for a given Fighter level. Armor training 1 is
/// gained at level 3; before that there is no armor-training effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FighterArmorTraining {
    /// Armor-training rank (0 before level 3, 1 from level 3 in this slice).
    rank: u8,
    /// Reduction applied to the worn armor's armor-check penalty (moves it toward 0).
    armor_check_reduction: i16,
    /// Increase applied to the worn armor's maximum Dexterity bonus.
    max_dex_increase: i16,
}

fn fighter_armor_training(level: u8) -> FighterArmorTraining {
    if level >= FIGHTER_ARMOR_TRAINING_1_LEVEL {
        FighterArmorTraining {
            rank: 1,
            armor_check_reduction: ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_1_MAX_DEX_INCREASE,
        }
    } else {
        FighterArmorTraining {
            rank: 0,
            armor_check_reduction: 0,
            max_dex_increase: 0,
        }
    }
}

/// The effective Chain Shirt armor-check penalty at a Fighter level, after any
/// armor-training reduction. Capped at 0 so the reduction never turns the penalty
/// into a bonus.
fn effective_chain_shirt_armor_check_penalty(level: u8) -> i16 {
    (CHAIN_SHIRT_ARMOR_CHECK_PENALTY + fighter_armor_training(level).armor_check_reduction).min(0)
}

/// Compute the bounded Fighter base chassis for the supported milestone levels
/// (1, 2, or 3), or block the claim if the input is not a supported single-class
/// Fighter posture for this narrow slice.
fn compute_fighter_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, BaseSaves) {
    let Some(level) = supported_fighter_level(input) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis is only supported for a single-class {FIGHTER_CLASS_ID} at \
                 levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL}; chosen class levels {:?} do not provide it, \
                 so no chassis values were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
    };

    // Grounded Fighter base progression from cr_classes.lst:139, evaluated at the
    // chosen level:
    //   BONUS:COMBAT|BASEAB|classlevel                -> level (full base attack)
    //   BONUS:SAVE|BASE.Fortitude|classlevel/2+2      -> level/2 + 2 (good save)
    //   BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 -> level/3 (poor saves)
    let level_value = i16::from(level);
    let base_attack_bonus = level_value;
    let base_saves = BaseSaves {
        fortitude: level_value / 2 + 2,
        reflex: level_value / 3,
        will: level_value / 3,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Fighter level {level} base attack bonus from cr_classes.lst:139 \
             BONUS:COMBAT|BASEAB|classlevel = {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "Fighter level {level} base Fortitude save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Fortitude|classlevel/2+2 = {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "Fighter level {level} base Reflex save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "Fighter level {level} base Will save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = {}",
            base_saves.will
        ),
    });

    (base_attack_bonus, base_saves)
}

/// Make the bounded Fighter milestone class features for this slice explicit rather
/// than leaving them incidental: the level-2 bonus-feat progression seam and the
/// level-3 armor-training seam.
///
/// This adds no general feat-effect or prerequisite engine. The level-2 bonus-feat
/// seam names the chosen selection only and contributes no computed mechanical value.
/// The level-3 armor-training seam names the concrete armor-check-penalty reduction
/// and maximum-Dexterity increase that the bounded selected-skill and armor-class
/// outputs already apply, so the derived-output change is legible instead of folklore.
fn explain_fighter_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = supported_fighter_level(input) else {
        return;
    };

    if level >= 2
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_2_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 2 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    let armor_training = fighter_armor_training(level);
    if armor_training.rank > 0 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_1_LEVEL} Armor Training 1 (armor training, \
                 cr_abilities_class.lst Fighter): reduces the worn Chain Shirt armor-check penalty by \
                 {ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION} (from {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to \
                 {reduced_penalty:+}) and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_1_MAX_DEX_INCREASE} (from {CHAIN_SHIRT_MAX_DEX} to {raised_max_dex})"
            ),
        });
    }
}

/// The canonical Human Fighter feat-choice selections this slice preserves on the
/// deterministic level-1/2/3 seam, as `(choice_set_id, canonical_selection_id)` pairs.
/// Any named slot present but deviating from its canonical selection is claim-blocked.
/// A slot absent for the chosen level (e.g. the level-2 bonus feat at level 1) is not
/// fabricated.
const CANONICAL_FIGHTER_FEAT_CHOICES: [(&str, &str); 4] = [
    (
        LEVEL_1_CHARACTER_FEAT_CHOICE_ID,
        POWER_ATTACK_FEAT_SELECTION,
    ),
    (HUMAN_BONUS_FEAT_CHOICE_ID, DODGE_FEAT_ID),
    (
        FIGHTER_BONUS_FEAT_CHOICE_ID,
        WEAPON_FOCUS_LONGSWORD_SELECTION,
    ),
    (
        FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID,
        TOUGHNESS_FEAT_SELECTION,
    ),
];

/// Claim-block non-canonical feat-choice mutations on the deterministic Human Fighter
/// levels 1-3 seam, while preserving the accepted canonical selections exactly.
///
/// This is deliberately not a general feat legality or prerequisite engine. It only knows
/// the exact accepted deterministic feat-choice selections on the bounded Human Fighter
/// seam. When one of those named choice slots is present but deviates from its canonical
/// selection, it emits a claim-blocking diagnostic that names the offending choice identity
/// and states plainly that alternative feat/prerequisite legality is outside this bounded
/// proof without a general engine — instead of letting the non-canonical build ride through
/// as a fabricated computed success.
///
/// It runs only for a supported single-class Human Fighter (levels 1-3); any other posture
/// is already claim-blocked upstream and is left untouched here. It grounds no alternative
/// feat effect and does not touch the read-only canonical Human ability-bonus target.
fn validate_fighter_feat_choice_legality(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if supported_fighter_level(input).is_none() {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    for (choice_set_id, canonical_selection) in CANONICAL_FIGHTER_FEAT_CHOICES {
        let Some(selection) = choice_selection(input, choice_set_id) else {
            // The slot is absent for this level; do not fabricate a required choice.
            continue;
        };
        if selection != canonical_selection {
            diagnostics.push(ComputationDiagnostic {
                id: format!("feat_choice.non_canonical.{choice_set_id}"),
                message: format!(
                    "feat-choice slot {choice_set_id} on the deterministic Human Fighter levels \
                     1-{MAX_SUPPORTED_FIGHTER_LEVEL} seam must be the canonical {canonical_selection}; \
                     chosen selection {selection} is a non-canonical feat choice. This bounded slice \
                     preserves only the accepted canonical Human Fighter feat-choice path and grounds \
                     no general feat-effect or prerequisite engine, so alternative feat/prerequisite \
                     legality is outside this proof and the non-canonical build is claim-blocked \
                     rather than computed as a legal build"
                ),
                claim_blocking: true,
            });
        }
    }
}

/// A hybrid (martial + later spellcasting) class this slice recognizes at its bounded
/// single-class level-1 chassis boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HybridClass {
    Paladin,
    Ranger,
}

/// Return the hybrid class when the chosen input is exactly a single-class Paladin or
/// Ranger at the bounded hybrid baseline level (1). Returns `None` for any other class,
/// a multiclass mix, or a level-2+ hybrid this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
fn hybrid_level1_class(input: &CharacterInput) -> Option<HybridClass> {
    match input.chosen.class_levels.as_slice() {
        [class_level] if class_level.level == HYBRID_BASELINE_LEVEL => {
            match class_level.class_id.as_str() {
                PALADIN_CLASS_ID => Some(HybridClass::Paladin),
                RANGER_CLASS_ID => Some(HybridClass::Ranger),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Surface direct SD13-E3-F6 runtime evidence for the deterministic Human Paladin
/// level-1 and Human Ranger level-1 hybrid chassis, while keeping both explicitly
/// claim-blocked on their still-missing burdens.
///
/// This deliberately does not compute a supported hybrid chassis. It grounds no base
/// attack/save progression, no smite / lay-on-hands / divine-grace / mercy execution,
/// no favored-enemy / combat-style / tracking execution, and no spell posture. It only:
/// - leaves one chassis-recognition explanation so the `class:paladin:1` / `class:ranger:1`
///   identity is acknowledged as a hybrid martial baseline rather than an undocumented
///   packet placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits two claim-blocking diagnostics naming the still-missing non-spell class-feature
///   burden family and the later hybrid spell burden explicitly, rather than hiding behind
///   a generic "unsupported hybrid" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks these inputs; this seam
/// keeps that blocked posture but makes the hybrid class identity and its named burdens
/// legible on the runtime path.
fn explain_hybrid_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(hybrid) = hybrid_level1_class(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    let (class_id, class_name, chassis_id, feature_id, feature_burden, spell_id) = match hybrid {
        HybridClass::Paladin => (
            PALADIN_CLASS_ID,
            "Paladin",
            "class_chassis.hybrid_baseline.paladin",
            "class_feature.hybrid.paladin.unsupported",
            "smite evil, lay on hands, divine grace, and mercy",
            "class_spell.hybrid.paladin.unsupported",
        ),
        HybridClass::Ranger => (
            RANGER_CLASS_ID,
            "Ranger",
            "class_chassis.hybrid_baseline.ranger",
            "class_feature.hybrid.ranger.unsupported",
            "favored enemy, combat style, and skill/tracking",
            "class_spell.hybrid.ranger.unsupported",
        ),
    };

    // Direct runtime evidence: recognize the deterministic Human hybrid level-1 chassis
    // identity. This is a recognition record only; it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {HYBRID_BASELINE_LEVEL} hybrid chassis: \
             the {class_id}:{HYBRID_BASELINE_LEVEL} class identity is acknowledged as a hybrid martial \
             baseline on the rules-core seam rather than an undocumented packet placeholder. This is a \
             bounded chassis-recognition record only; it grounds no {class_name} class-feature math and \
             no spell posture, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the non-spell class-feature burden family explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: feature_id.to_owned(),
        message: format!(
            "{class_name} level {HYBRID_BASELINE_LEVEL} remains blocked on its non-spell class-feature \
             burden: {feature_burden} are not implemented in this bounded hybrid chassis baseline, so no \
             {class_name} class-feature support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the later hybrid spell burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: spell_id.to_owned(),
        message: format!(
            "{class_name} remains blocked on its later hybrid spell burden: spell slots, spell source, \
             and spells known/prepared posture are out of scope for this level-{HYBRID_BASELINE_LEVEL} \
             chassis baseline and are deferred to the SD13-E4 spellcasting slice"
        ),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Paladin at the
/// bounded hybrid baseline level (1). Returns `false` for any other class, a
/// multiclass mix, the Ranger hybrid (which has its own F6 class-feature
/// decomposition lane), or any level-2+ Paladin this slice deliberately does
/// not recognize — each of which stays blocked exactly as before.
fn is_single_class_paladin_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == PALADIN_CLASS_ID
                && class_level.level == HYBRID_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E3/E4 runtime evidence for the deterministic Human
/// Paladin level-1 chassis and spell burden as a separable pair of diagnostics.
///
/// This sits on top of the accepted SD13-F6 hybrid baseline: F6 already proves
/// the deterministic Human Paladin level-1 hybrid identity is acknowledged on
/// the compute seam and emits a single combined non-spell class-feature
/// blocker plus a single combined later-spell blocker. This slice proves the
/// per-burden separation Paladin actually needs:
///
/// - one explicit claim-blocking diagnostic per still-missing non-spell
///   class-feature burden:
///   * `smite evil` — alignment / target resolution, smite attack rolls and
///     damage bonus, and smite-uses-per-day resource accounting
///   * `lay on hands` — healing resource accounting, charismabased pool, and
///     use-against-conditions behavior
///   * `divine grace` — charisma-to-saves bonus, including the typing and
///     proc-time posture that divines saves even at the deterministic seam
///   * `mercy` — conditional save-effect and disease/poison removal mechanics
///     that only begin at level 6 and must therefore be claimed honestly as
///     still-blocked rather than silently allowed
///
/// - one explicit claim-blocking diagnostic for the partial-caster spell
///   burden, distinct from the non-spell class-feature blockers:
///   * Paladin is a divine partial caster in PF1 Core Rulebook (effective
///     caster level = paladin level − 2, slots begin at level 2); the blocker
///     names this partial-caster posture so the later SD13-E4 spell-burden
///     closure cannot collapse Paladin into a full divine caster shape
///     (Cleric / Druid) and so partial-caster pressure stays visible on the
///     runtime path.
///
/// This deliberately does not compute a supported spell surface. It grounds
/// no smite math, no lay-on-hands resource handling, no divine-grace
/// computation, no mercy handling, no spell slots, no spell source lineage,
/// no spells known or prepared posture, no deity resolution, no domain
/// mechanics, no alignment-target resolution, and no healing accounting. It
/// only emits the per-burden blockers that prove the F6 surface remains
/// separable on the runtime path.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input;
/// the F6 hybrid chassis emission already preserves a single class-feature
/// blocker and a single spell blocker. This seam adds per-burden granularity
/// next to the F6 surface, never replacing it, so the F6 acceptance test
/// continues to pass.
fn explain_paladin_level1_chassis_and_spell_burden_separation(
    input: &CharacterInput,
    _explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_paladin_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // The per-burden blockers ride alongside the F6 hybrid blockers. They are
    // intentionally distinct diagnostic ids so the chassis burden is separable
    // from the spell burden on the runtime path.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.paladin.smite_evil.unsupported".to_owned(),
        message: format!(
            "Paladin level {HYBRID_BASELINE_LEVEL} remains blocked on its smite evil burden: \
             smite attack-roll bonuses, smite damage bonus, smite-uses-per-day resource \
             accounting, and the underlying alignment-target resolution are not implemented in \
             this bounded chassis baseline, so no Paladin smite execution is claimed"
        ),
        claim_blocking: true,
    });

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.paladin.lay_on_hands.unsupported".to_owned(),
        message: format!(
            "Paladin level {HYBRID_BASELINE_LEVEL} remains blocked on its lay on hands burden: \
             the charisma-based healing pool, the per-level heal amount, the use against poison / \
             disease behavior, and any heal-resource accounting are not implemented in this \
             bounded chassis baseline, so no Paladin lay on hands support is claimed"
        ),
        claim_blocking: true,
    });

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.paladin.divine_grace.unsupported".to_owned(),
        message: format!(
            "Paladin level {HYBRID_BASELINE_LEVEL} remains blocked on its divine grace burden: \
             the charisma-to-saving-throw bonus and the typing/proc-time posture that applies it \
             are not implemented in this bounded chassis baseline, so no Paladin divine grace \
             save bonus support is claimed"
        ),
        claim_blocking: true,
    });

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.paladin.mercy.unsupported".to_owned(),
        message: format!(
            "Paladin level {HYBRID_BASELINE_LEVEL} remains blocked on its mercy burden: mercy \
             is a level-6 class-feature, so the conditional save-effect and disease/poison \
             removal mechanics are not implemented here; no Paladin mercy behavior is claimed"
        ),
        claim_blocking: true,
    });

    // The partial-caster spell burden is its own blocker, distinct from the
    // four non-spell class-feature blockers above. Paladin is a divine partial
    // caster in PF1 Core Rulebook (effective caster level = paladin level - 2;
    // first spell access at level 2 with a slowed slot progression), and the
    // blocker must name that partial-caster posture so the later SD13-E4
    // spell-burden closure cannot confuse Paladin with a full divine caster
    // (Cleric / Druid).
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.paladin.partial_caster.unsupported".to_owned(),
        message: format!(
            "Paladin remains blocked on its divine partial-caster spell burden: Paladin is a \
             partial caster (effective caster level = paladin level - 2, with spell slots first \
             available at level 2 in PF1 Core Rulebook), so spell-source lineage, spells known \
             or prepared posture, spells-per-day progression, bonus spell slots, and spell save \
             DCs are deferred to the SD13-E4 spellcasting slice; no partial-caster spell \
             execution is fabricated in this bounded chassis baseline"
        ),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Sorcerer at the bounded
/// spell baseline level (1). Returns `false` for any other class, a multiclass mix, or a
/// level-2+ Sorcerer this slice deliberately does not recognize — each of which stays
/// blocked exactly as before.
fn is_single_class_sorcerer_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == SORCERER_CLASS_ID
                && class_level.level == SORCERER_BASELINE_LEVEL
    )
}

/// A pure martial (non-hybrid, non-spell) class this slice recognizes at its bounded
/// single-class level-1 chassis boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MartialClass {
    Barbarian,
}

/// Return the martial class when the chosen input is exactly a single-class Barbarian
/// at the bounded martial baseline level (1). Returns `None` for any other class, a
/// multiclass mix, or a level-2+ Barbarian this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
fn martial_level1_class(input: &CharacterInput) -> Option<MartialClass> {
    match input.chosen.class_levels.as_slice() {
        [class_level] if class_level.level == MARTIAL_BASELINE_LEVEL => {
            match class_level.class_id.as_str() {
                BARBARIAN_CLASS_ID => Some(MartialClass::Barbarian),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Surface direct SD13-E3 runtime evidence for the deterministic Human Barbarian
/// level-1 martial chassis, while keeping it explicitly claim-blocked on the four
/// still-missing named burdens.
///
/// This deliberately does not compute a supported martial chassis. It grounds no
/// base-attack progression, no base-save progression, no fast-movement +10 ft. speed
/// extension, and no illiteracy trait engine. It grounds no rage execution, no
/// weapon familiarity, and no level-2+ martial progression. It only:
/// - leaves one chassis-recognition explanation so the `class:barbarian:1` identity
///   is acknowledged as a non-hybrid martial baseline rather than an undocumented
///   packet placeholder (direct runtime evidence, carrying no fabricated mechanical
///   value), and
/// - emits four claim-blocking diagnostics naming the four still-missing pillar
///   burdens (base-attack, base-save, fast-movement, illiteracy) explicitly, rather
///   than hiding behind a single generic "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Barbarian martial identity and its four
/// named pillar burdens legible on the runtime path.
fn explain_barbarian_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(martial) = martial_level1_class(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Only the Barbarian is in this slice today; the match is exhaustive-by-design so
    // a future addition needs an explicit arm here. The SD13-E3 Monk martial-chassis
    // slice landed as its own dedicated recognition function (`explain_monk_level1_chassis`)
    // rather than a new `MartialClass` arm, since its four named burdens are unrelated
    // in content to Barbarian's.
    let MartialClass::Barbarian = martial;
    let class_id = BARBARIAN_CLASS_ID;
    let class_name = "Barbarian";
    let chassis_id = "class_chassis.barbarian.bounded_progression";

    // Direct runtime evidence: recognize the deterministic Human Barbarian level-1
    // martial chassis identity. This is a recognition record only; it fabricates no
    // mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {MARTIAL_BASELINE_LEVEL} martial chassis: \
             the {class_id}:{MARTIAL_BASELINE_LEVEL} class identity is acknowledged as a pure non-hybrid \
             martial baseline on the rules-core seam rather than an undocumented packet placeholder. This \
             is a bounded chassis-recognition record only; it grounds no {class_name} base-attack or \
             base-save progression, no fast-movement +10 ft. speed extension, no illiteracy trait engine, \
             no rage execution, and no level-2+ martial progression, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // Still blocked (1/4): name the base-attack progression burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.barbarian.bounded_progression.base_attack.unsupported".to_owned(),
        message: format!(
            "{class_name} level {MARTIAL_BASELINE_LEVEL} remains blocked on its base attack progression: \
             the full-BAB +{MARTIAL_BASELINE_LEVEL} base-attack bonus and the higher-level BAB cadence are \
             not implemented in this bounded martial chassis baseline, so no {class_name} base-attack \
             support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/4): name the base-save progression burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.barbarian.bounded_progression.base_save.unsupported".to_owned(),
        message: format!(
            "{class_name} level {MARTIAL_BASELINE_LEVEL} remains blocked on its base save progression: \
             the good Fortitude progression (classlevel/2+2, +{} at level {MARTIAL_BASELINE_LEVEL}) and \
             the poor Reflex / poor Will base-save cadence are not implemented in this bounded martial \
             chassis baseline, so no {class_name} base-save support is claimed",
            i16::from(MARTIAL_BASELINE_LEVEL) / 2 + 2
        ),
        claim_blocking: true,
    });

    // Still blocked (3/4): name the fast-movement speed-extension burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.barbarian.bounded_progression.fast_movement.unsupported".to_owned(),
        message: format!(
            "{class_name} level {MARTIAL_BASELINE_LEVEL} remains blocked on its fast movement burden: \
             the +10 ft. land speed extension applied while wearing no heavy armor is not implemented in \
             this bounded martial chassis baseline, so no {class_name} fast-movement support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (4/4): name the illiteracy trait burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.barbarian.bounded_progression.illiteracy.unsupported".to_owned(),
        message: format!(
            "{class_name} level {MARTIAL_BASELINE_LEVEL} remains blocked on its illiteracy trait: \
             the trait that prevents literate reading and writing of non-native languages without additional \
             training is not implemented in this bounded martial chassis baseline, so no {class_name} \
             illiteracy-trait support is claimed"
        ),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Monk at the bounded
/// martial baseline level (1). Returns `false` for any other class, a multiclass mix,
/// or a level-2+ Monk this slice deliberately does not recognize — each of which stays
/// blocked exactly as before.
fn is_single_class_monk_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == MONK_CLASS_ID
                && class_level.level == MARTIAL_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E3 runtime evidence for the deterministic Human Monk level-1
/// martial chassis, mirroring the Barbarian level-1 baseline pattern, while keeping it
/// explicitly claim-blocked on the four still-missing named burdens.
///
/// This deliberately does not compute a supported martial chassis. It grounds no
/// base-attack progression (3/4 BAB), no base-save progression (good Fortitude,
/// Reflex, and Will), no unarmed strike damage die, no Flurry of Blows execution, no
/// AC Bonus (Wisdom-to-AC) computation, and no level-1 bonus feat grant from the
/// restricted Monk feat list. It grounds no ki pool and no level-2+ martial
/// progression. It only:
/// - leaves one chassis-recognition explanation so the `class:monk:1` identity is
///   acknowledged as a non-hybrid martial baseline rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits four claim-blocking diagnostics naming the four still-missing pillar
///   burdens (base-attack, base-save, unarmed-strike/flurry, AC-bonus/bonus-feat)
///   explicitly, rather than hiding behind a single generic "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Monk martial identity and its four named
/// pillar burdens legible on the runtime path.
fn explain_monk_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_monk_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Monk level-1 martial
    // chassis identity. This is a recognition record only; it fabricates no mechanical
    // value.
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.bounded_progression".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Monk level {MARTIAL_BASELINE_LEVEL} martial chassis: \
             the {MONK_CLASS_ID}:{MARTIAL_BASELINE_LEVEL} class identity is acknowledged as a pure \
             non-hybrid martial baseline on the rules-core seam rather than an undocumented packet \
             placeholder. This is a bounded chassis-recognition record only; it grounds no Monk \
             base-attack or base-save progression, no unarmed strike damage die, no Flurry of \
             Blows execution, no AC Bonus computation, no level-1 bonus feat grant, no ki pool, \
             and no level-2+ martial progression, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/4): name the base-attack progression burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.base_attack.unsupported".to_owned(),
        message: format!(
            "Monk level {MARTIAL_BASELINE_LEVEL} remains blocked on its base attack progression: \
             the 3/4-BAB progression is not implemented in this bounded martial chassis baseline, \
             so no Monk base-attack support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/4): name the base-save progression burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.base_save.unsupported".to_owned(),
        message: format!(
            "Monk level {MARTIAL_BASELINE_LEVEL} remains blocked on its base save progression: \
             the good Fortitude, Reflex, and Will base-save progressions are not implemented in \
             this bounded martial chassis baseline, so no Monk base-save support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (3/4): name the unarmed strike / Flurry of Blows burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.unarmed_strike_and_flurry.unsupported"
            .to_owned(),
        message: format!(
            "Monk level {MARTIAL_BASELINE_LEVEL} remains blocked on its unarmed strike and \
             Flurry of Blows burden: the unarmed strike damage die and the Flurry of Blows extra \
             attack are not implemented in this bounded martial chassis baseline, so no Monk \
             unarmed strike or Flurry of Blows support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (4/4): name the AC Bonus / level-1 bonus feat burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.ac_bonus_and_bonus_feat.unsupported"
            .to_owned(),
        message: format!(
            "Monk level {MARTIAL_BASELINE_LEVEL} remains blocked on its AC Bonus and bonus feat \
             burden: the AC Bonus (Wisdom modifier added to AC while unarmored and unencumbered) \
             and the level-1 bonus feat grant from the restricted Monk feat list are not \
             implemented in this bounded martial chassis baseline, so no Monk AC Bonus or bonus \
             feat support is claimed"
        ),
        claim_blocking: true,
    });
}

/// Surface direct SD13-E4-F7 runtime evidence for the deterministic Human Sorcerer
/// level-1 spell-bearing baseline, while keeping it explicitly claim-blocked on its two
/// still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no bloodline
/// power, no bloodline arcana, and no spell math whatsoever — no spell slots, spells
/// known, spell DCs, bonus spells, prepared posture, or school choice. It only:
/// - leaves one recognition explanation so the `class:sorcerer:1` identity is acknowledged
///   as a spontaneous arcane spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits two distinct claim-blocking diagnostics naming the bloodline burden and the
///   spontaneous known-spell / slot posture burden explicitly, rather than hiding behind a
///   generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Sorcerer spell-bearing identity and its two
/// named burdens legible on the runtime path.
fn explain_sorcerer_level1_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_sorcerer_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Sorcerer level-1
    // spell-bearing identity. This is a recognition record only; it fabricates no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.sorcerer".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Sorcerer level {SORCERER_BASELINE_LEVEL} spell-bearing \
             baseline: the {SORCERER_CLASS_ID}:{SORCERER_BASELINE_LEVEL} class identity is acknowledged \
             as a spontaneous arcane spell-bearing class on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it grounds no \
             bloodline power and no spell math (spell slots, spells known, spell DCs, bonus spells, or \
             prepared posture), so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the bloodline burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.sorcerer.bloodline.unsupported".to_owned(),
        message: format!(
            "Sorcerer level {SORCERER_BASELINE_LEVEL} remains blocked on its bloodline burden: the \
             bloodline selection, its level-1 bloodline power, bloodline arcana, and bloodline bonus \
             spells/feats/skills are not implemented in this bounded spell baseline, so no Sorcerer \
             bloodline support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the spontaneous known-spell / slot posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.sorcerer.spontaneous.unsupported".to_owned(),
        message:
            "Sorcerer remains blocked on its spontaneous known-spell / slot posture burden: \
             spontaneous casting, spells known, spell slots per day, bonus spell slots from a high \
             ability score, and spell save DCs are out of scope for this level-1 spell baseline and \
             no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Wizard at the bounded
/// prepared spell baseline level (1). Returns `false` for any other class, a multiclass
/// mix, or a level-2+ Wizard this slice deliberately does not recognize — each of which
/// stays blocked exactly as before.
fn is_single_class_wizard_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == WIZARD_CLASS_ID
                && class_level.level == WIZARD_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E4-R3 runtime evidence for the deterministic Human Wizard
/// level-1 prepared arcane spell-bearing baseline, while keeping it explicitly
/// claim-blocked on its two still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no
/// spellbook content, no spells prepared, no spell slots per day, no spell save
/// DCs, no bonus spell slots from a high Intelligence, no school-opposition
/// bookkeeping, and no specialty school bonus. It only:
/// - leaves one recognition explanation so the `class:wizard:1` identity is
///   acknowledged as a prepared arcane spell-bearing class rather than an
///   undocumented packet placeholder (direct runtime evidence, carrying no
///   fabricated mechanical value), and
/// - emits two distinct claim-blocking diagnostics naming the school
///   specialization burden (chosen school, two opposed schools, specialty school
///   bonus) and the prepared spellbook / spells-prepared / spell-slot posture
///   burden explicitly, rather than hiding behind a generic "unsupported caster"
///   label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this
/// seam keeps that blocked posture but makes the Wizard prepared spell-bearing
/// identity and its two named burdens legible on the runtime path. The matrix
/// file row transition (Unverified/Observed → Blocked/Computed) is recorded by
/// the merge receipt only and is NOT applied to the in-source carrier here.
fn explain_wizard_level1_prepared_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_wizard_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Wizard level-1
    // prepared arcane spell-bearing identity. This is a recognition record only;
    // it fabricates no spell math and no school-opposition / specialty school
    // bonus math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.wizard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Wizard level {WIZARD_BASELINE_LEVEL} prepared arcane \
             spell-bearing baseline: the {WIZARD_CLASS_ID}:{WIZARD_BASELINE_LEVEL} class identity \
             is acknowledged as a prepared arcane spell-bearing class on the rules-core seam \
             rather than an undocumented packet placeholder. This is a bounded recognition record \
             only; it grounds no spellbook content, no spells prepared per day, no spell slots \
             per day, no spell save DCs, no bonus spell slots from a high Intelligence, no school \
             specialization mechanics, no opposed-school bookkeeping, and no specialty school \
             bonus, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the school specialization burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.wizard.school_specialization.unsupported".to_owned(),
        message: format!(
            "Wizard level {WIZARD_BASELINE_LEVEL} remains blocked on its school specialization \
             burden: the chosen school, two opposed schools, the school's opposed schools list, \
             and the specialty school bonus (additional spell slots / spells known at later \
             levels) are not implemented in this bounded prepared spell baseline, so no Wizard \
             school specialization support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared spellbook / spells-prepared /
    // spell-slot posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.wizard.prepared_spellbook.unsupported".to_owned(),
        message:
            "Wizard remains blocked on its prepared spellbook / spells prepared / spell slot \
             posture burden: spellbook content, spells prepared per day, spell slots per day, \
             bonus spell slots from a high Intelligence, and spell save DCs are out of scope for \
             this level-1 prepared spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Cleric at the bounded
/// prepared divine spell baseline level (1). Returns `false` for any other class, a
/// multiclass mix, or a level-2+ Cleric this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
fn is_single_class_cleric_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == CLERIC_CLASS_ID
                && class_level.level == CLERIC_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E4 runtime evidence for the deterministic Human Cleric level-1
/// prepared divine spell-bearing baseline, while keeping it explicitly claim-blocked on
/// its two still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no domain
/// selection, no domain spells, no domain powers, no channel energy execution, no
/// spellbook posture, no spells prepared, no spontaneous cure/inflict conversion, no
/// spell slots per day, no spell save DCs, and no bonus spell slots from a high Wisdom.
/// It only:
/// - leaves one recognition explanation so the `class:cleric:1` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits two distinct claim-blocking diagnostics naming the domain / channel energy
///   class-feature burden and the prepared divine spell posture burden explicitly,
///   rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Cleric prepared divine spell-bearing
/// identity and its two named burdens legible on the runtime path.
fn explain_cleric_level1_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_cleric_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Cleric level-1
    // prepared divine spell-bearing identity. This is a recognition record only; it
    // fabricates no domain power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.cleric".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Cleric level {CLERIC_BASELINE_LEVEL} prepared divine \
             spell-bearing baseline: the {CLERIC_CLASS_ID}:{CLERIC_BASELINE_LEVEL} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; it grounds \
             no domain selection, no domain spells, no domain powers, no channel energy execution, no \
             spellbook posture, no spells prepared per day, no spontaneous cure/inflict conversion, no \
             spell slots per day, no spell save DCs, and no bonus spell slots from a high Wisdom, so it \
             carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the domain / channel energy class-feature burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.cleric.domain_and_channel_energy.unsupported".to_owned(),
        message: format!(
            "Cleric level {CLERIC_BASELINE_LEVEL} remains blocked on its domain and channel energy \
             burden: the two chosen domains, their domain spells, their domain powers, and channel \
             energy (positive/negative energy burst, uses per day, save DC) are not implemented in \
             this bounded prepared divine spell baseline, so no Cleric domain or channel energy \
             support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared divine spell posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.cleric.prepared_divine.unsupported".to_owned(),
        message:
            "Cleric remains blocked on its prepared divine spell posture burden: spells prepared \
             from the full Cleric spell list, spontaneous cure/inflict conversion, spell slots per \
             day, bonus spell slots from a high Wisdom, and spell save DCs are out of scope for this \
             level-1 spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Druid at the bounded
/// prepared divine spell baseline level (1). Returns `false` for any other class, a
/// multiclass mix, or a level-2+ Druid this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
fn is_single_class_druid_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == DRUID_CLASS_ID
                && class_level.level == DRUID_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E4 runtime evidence for the deterministic Human Druid level-1
/// prepared divine spell-bearing baseline, while keeping it explicitly claim-blocked on
/// its two still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no nature
/// bond selection, no nature bond power execution (animal companion or domain), no
/// wild empathy check resolution, no spellbook posture, no spells prepared, no
/// spontaneous summon nature's ally conversion, no spell slots per day, no spell save
/// DCs, and no bonus spell slots from a high Wisdom. It only:
/// - leaves one recognition explanation so the `class:druid:1` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits two distinct claim-blocking diagnostics naming the nature bond / wild
///   empathy class-feature burden and the prepared divine spell posture burden
///   explicitly, rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Druid prepared divine spell-bearing
/// identity and its two named burdens legible on the runtime path.
fn explain_druid_level1_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_druid_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Druid level-1
    // prepared divine spell-bearing identity. This is a recognition record only; it
    // fabricates no nature-bond power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.druid".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Druid level {DRUID_BASELINE_LEVEL} prepared divine \
             spell-bearing baseline: the {DRUID_CLASS_ID}:{DRUID_BASELINE_LEVEL} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; it grounds \
             no nature bond selection, no nature bond power execution, no wild empathy check \
             resolution, no spellbook posture, no spells prepared per day, no spontaneous summon \
             nature's ally conversion, no spell slots per day, no spell save DCs, and no bonus spell \
             slots from a high Wisdom, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the nature bond / wild empathy class-feature burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.druid.nature_bond_and_wild_empathy.unsupported".to_owned(),
        message: format!(
            "Druid level {DRUID_BASELINE_LEVEL} remains blocked on its nature bond and wild empathy \
             burden: the nature bond choice (an animal companion or a domain), nature sense, and wild \
             empathy (the animal-diplomacy check) are not implemented in this bounded prepared divine \
             spell baseline, so no Druid nature bond or wild empathy support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared divine spell posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.druid.prepared_divine.unsupported".to_owned(),
        message:
            "Druid remains blocked on its prepared divine spell posture burden: spells prepared \
             from the full Druid spell list, spontaneous summon nature's ally conversion, spell slots \
             per day, bonus spell slots from a high Wisdom, and spell save DCs are out of scope for \
             this level-1 spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Return `true` when the chosen input is exactly a single-class Bard at the bounded
/// spell baseline level (1). Returns `false` for any other class, a multiclass mix, or a
/// level-2+ Bard this slice deliberately does not recognize — each of which stays
/// blocked exactly as before.
fn is_single_class_bard_level1(input: &CharacterInput) -> bool {
    matches!(
        input.chosen.class_levels.as_slice(),
        [class_level]
            if class_level.class_id == BARD_CLASS_ID
                && class_level.level == BARD_BASELINE_LEVEL
    )
}

/// Surface direct SD13-E4-F7 runtime evidence for the deterministic Human Bard
/// level-1 spontaneous arcane spell-bearing baseline, while keeping it explicitly
/// claim-blocked on its two still-missing burdens.
///
/// This deliberately does not compute a supported Bard chassis. It grounds no
/// Bardic Knowledge check resolution, no Bardic Music / Inspire Courage execution, and
/// no spell math whatsoever — no spells known, no spells per day, no spell DCs, no
/// bonus spells, no prepared posture, no school choice. It only:
/// - leaves one recognition explanation so the `class:bard:1` identity is acknowledged
///   as a spontaneous arcane spell-bearing class with its named Bardic-class-feature
///   burden rather than an undocumented packet placeholder (direct runtime evidence,
///   carrying no fabricated mechanical value), and
/// - emits two distinct claim-blocking diagnostics naming the Bardic Knowledge + Bardic
///   Music chassis-class-feature burden and the spontaneous known-spell / slot posture
///   burden explicitly, rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Bard spell-bearing identity and its two
/// named burdens legible on the runtime path.
fn explain_bard_level1_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if !is_single_class_bard_level1(input) {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Bard level-1
    // spell-bearing identity. This is a recognition record only; it fabricates no
    // Bardic-class-feature math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.bard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Bard level {BARD_BASELINE_LEVEL} spell-bearing \
             baseline: the {BARD_CLASS_ID}:{BARD_BASELINE_LEVEL} class identity is acknowledged \
             as a spontaneous arcane spell-bearing class with its named Bardic Knowledge and \
             Bardic Music chassis-class-feature burden on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it \
             grounds no Bardic Knowledge check resolution, no Bardic Music / Inspire Courage \
             execution, and no spell math (spells known, spells per day, spell DCs, bonus \
             spells, or prepared posture), so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the Bardic Knowledge + Bardic Music chassis-class-feature
    // burden explicitly. Bardic Knowledge grants a competence bonus on Knowledge checks
    // equal to half the Bard level (minimum 1) plus the Bard's INT modifier; Bardic Music
    // grants the Inspire Courage performance and the rest of the performance family. This
    // slice grounds neither check resolution nor performance execution.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.bard.bardic_knowledge_and_music.unsupported".to_owned(),
        message: format!(
            "Bard level {BARD_BASELINE_LEVEL} remains blocked on its bardic knowledge and \
             bardic music chassis-class-feature burden: the bardic knowledge competence bonus \
             on Knowledge checks (half Bard level + INT modifier) and the bardic music \
             performance family (inspire courage and later performances) are not implemented \
             in this bounded spell baseline, so no Bard class-feature support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the spontaneous known-spell / slot posture burden
    // explicitly. Bard spells known and spells per day are gated by Bard level and CHA
    // modifier on the Bard spell list; this slice grounds no spells known, no spells per
    // day, no spell DCs, and no bonus spells from a high casting stat.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.bard.spontaneous_known_and_per_day.unsupported".to_owned(),
        message:
            "Bard remains blocked on its spontaneous known-spell / slot posture burden: \
             spontaneous casting, spells known (from the Bard list), spells per day (from \
             the Bard table plus CHA modifier), bonus spell slots from a high casting stat, \
             and spell save DCs are out of scope for this level-1 spell baseline and no \
             spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Compute total saving throws as the grounded Fighter level 1–3 base save plus the
/// relevant ability modifier, or block the claim if a supported Fighter chassis
/// (levels 1–3) is absent.
///
/// This is intentionally narrow: it adds only the single ability modifier each
/// save uses (Fortitude/CON, Reflex/DEX, Will/WIS). It does not add feat-, item-,
/// or condition-based save modifiers.
fn compute_total_saves(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_saves: &BaseSaves,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> BaseSaves {
    if supported_fighter_level(input).is_none() {
        diagnostics.push(ComputationDiagnostic {
            id: "defense.total_save.unsupported".to_owned(),
            message: format!(
                "total saving throws are only computed from the grounded {FIGHTER_CLASS_ID} \
                 levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} base saves; chosen class levels {:?} do not \
                 provide them, so no total saves were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return BaseSaves::default();
    }

    let total_saves = BaseSaves {
        fortitude: base_saves.fortitude + ability_modifiers.constitution,
        reflex: base_saves.reflex + ability_modifiers.dexterity,
        will: base_saves.will + ability_modifiers.wisdom,
    };

    explanations.push(ComputationExplanation {
        id: "defense.total_save.fortitude".to_owned(),
        value: total_saves.fortitude,
        detail: format!(
            "Total Fortitude save: Fighter base Fortitude save (+{}) + Constitution modifier (+{}) = {}",
            base_saves.fortitude, ability_modifiers.constitution, total_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.reflex".to_owned(),
        value: total_saves.reflex,
        detail: format!(
            "Total Reflex save: Fighter base Reflex save (+{}) + Dexterity modifier (+{}) = {}",
            base_saves.reflex, ability_modifiers.dexterity, total_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.will".to_owned(),
        value: total_saves.will,
        detail: format!(
            "Total Will save: Fighter base Will save (+{}) + Wisdom modifier (+{}) = {}",
            base_saves.will, ability_modifiers.wisdom, total_saves.will
        ),
    });

    total_saves
}

/// Compute the selected deterministic Climb / Intimidate / Swim skill modifiers,
/// or block the claim if the selected-skill or Chain Shirt posture is absent or
/// widened beyond this slice.
///
/// This is intentionally not a skill engine. It computes only the three selected
/// Fighter class skills from the accepted deterministic rank allocations, applying
/// the already-grounded Chain Shirt armor-check penalty to the armor-check skills
/// (Climb, Swim) only. It does not handle other skills, arbitrary classes,
/// feat/racial/item skill bonuses, encumbrance, or speed-dependent adjustments.
/// Any deviation from the exact supported posture is refused with a claim-blocking
/// diagnostic and withheld selected-skill explanations rather than fabricated
/// totals.
fn compute_selected_skill_modifiers(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> SelectedSkillModifiers {
    let unmet = unmet_selected_skill_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "skill.selected_modifier.unsupported".to_owned(),
            message: format!(
                "selected skill modifiers are only computed for the exact GE-06 deterministic \
                 Fighter level-1 Climb/Intimidate/Swim rank-1 posture with the grounded Chain Shirt \
                 armor-check penalty; unmet conditions: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return SelectedSkillModifiers::default();
    }

    let rank = i16::from(SELECTED_SKILL_RANK);

    // The Chain Shirt armor-check penalty applied to Climb/Swim is reduced by Fighter
    // armor training from level 3, so the armor-check skills rise at that milestone.
    // The posture check above guarantees a supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let armor_check_penalty = effective_chain_shirt_armor_check_penalty(level);
    let armor_check_detail = if fighter_armor_training(level).armor_check_reduction > 0 {
        format!(
            "Chain Shirt armor-check penalty ({armor_check_penalty:+}, reduced from \
             {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} by Fighter armor training)"
        )
    } else {
        format!("Chain Shirt armor-check penalty ({armor_check_penalty:+})")
    };

    // Climb (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    let climb = rank + ability_modifiers.strength + CLASS_SKILL_BONUS + armor_check_penalty;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.climb".to_owned(),
        value: climb,
        detail: format!(
            "Selected Climb modifier: rank {rank} + Strength modifier ({:+}) + class-skill bonus \
             ({:+}) + {armor_check_detail} = {climb}",
            ability_modifiers.strength, CLASS_SKILL_BONUS
        ),
    });

    // Intimidate (CHA, not an armor-check skill): rank + CHA + class-skill.
    let intimidate = rank + ability_modifiers.charisma + CLASS_SKILL_BONUS;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.intimidate".to_owned(),
        value: intimidate,
        detail: format!(
            "Selected Intimidate modifier: rank {rank} + Charisma modifier ({:+}) + class-skill \
             bonus ({:+}) = {intimidate}",
            ability_modifiers.charisma, CLASS_SKILL_BONUS
        ),
    });

    // Swim (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    let swim = rank + ability_modifiers.strength + CLASS_SKILL_BONUS + armor_check_penalty;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.swim".to_owned(),
        value: swim,
        detail: format!(
            "Selected Swim modifier: rank {rank} + Strength modifier ({:+}) + class-skill bonus \
             ({:+}) + {armor_check_detail} = {swim}",
            ability_modifiers.strength, CLASS_SKILL_BONUS
        ),
    });

    SelectedSkillModifiers {
        climb,
        intimidate,
        swim,
    }
}

/// Return the list of unmet conditions for the exact deterministic selected-skill
/// posture. An empty list means the posture is fully supported.
///
/// The bounded posture requires a Fighter level 1–3 chassis, exactly the three
/// selected class skills (Climb, Intimidate, Swim) each at rank 1 with no other
/// skill allocations, and the grounded Chain Shirt armor-check posture that the
/// Climb/Swim totals depend on.
fn unmet_selected_skill_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let allocations = &input.chosen.skill_allocations;
    let mut unmet = Vec::new();

    if supported_fighter_level(input).is_none() {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} chassis"
        ));
    }

    let expected = [CLIMB_SKILL_ID, INTIMIDATE_SKILL_ID, SWIM_SKILL_ID];
    for skill_id in expected {
        require_selected_skill_rank(allocations, skill_id, &mut unmet);
    }

    // Refuse any widening beyond exactly the three selected skills.
    for allocation in allocations {
        if !expected.contains(&allocation.skill_id.as_str()) {
            unmet.push(format!(
                "skill allocation {} is outside the selected Climb/Intimidate/Swim slice",
                allocation.skill_id
            ));
        }
    }

    // Climb and Swim totals depend on the grounded Chain Shirt armor-check posture.
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );

    unmet
}

/// Record an unmet condition unless the named skill is allocated exactly the
/// supported deterministic rank.
fn require_selected_skill_rank(
    allocations: &[SkillAllocation],
    skill_id: &str,
    unmet: &mut Vec<String>,
) {
    let actual = allocations
        .iter()
        .find(|a| a.skill_id == skill_id)
        .map(|a| a.ranks);
    if actual != Some(SELECTED_SKILL_RANK) {
        unmet.push(format!(
            "{skill_id} must be allocated rank {SELECTED_SKILL_RANK} for the selected-skill slice, got {actual:?}"
        ));
    }
}

/// Compute the deterministic baseline melee attack bonus and armor class, or
/// block the claim if the input is not the exact supported pilot posture.
///
/// This is intentionally not a combat engine. It computes only the GE-06
/// deterministic Longsword/Chain Shirt/Dodge/no-shield baseline. Any deviation
/// from that exact posture is refused with a claim-blocking diagnostic rather
/// than fabricating combat totals.
fn compute_combat_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_attack_bonus: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, i16) {
    let unmet = unmet_combat_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "combat.baseline_unsupported".to_owned(),
            message: format!(
                "baseline combat totals are only computed for the exact GE-06 deterministic \
                 Longsword/Chain Shirt/Dodge/no-shield posture; unmet conditions: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return (0, 0);
    }

    // Baseline melee attack bonus: Fighter BAB + STR modifier + Weapon Focus
    // (Longsword). Power Attack is selected but inactive, contributing 0.
    let strength_modifier = ability_modifiers.strength;
    let melee_attack_bonus = base_attack_bonus + strength_modifier + WEAPON_FOCUS_TO_HIT_BONUS;

    explanations.push(ComputationExplanation {
        id: "combat.baseline_melee_attack_bonus".to_owned(),
        value: melee_attack_bonus,
        detail: format!(
            "Baseline melee attack bonus for the Longsword: Fighter base attack bonus (+{base_attack_bonus}) \
             + Strength modifier (+{strength_modifier}) + Weapon Focus (Longsword) (+{WEAPON_FOCUS_TO_HIT_BONUS}); \
             Power Attack is selected but inactive (+0) = {melee_attack_bonus}"
        ),
    });

    // Baseline armor class: 10 + Chain Shirt armor bonus + capped DEX + Dodge,
    // with no shield (absent posture contributes 0). Fighter armor training from
    // level 3 raises the Chain Shirt maximum Dexterity bonus; the posture check
    // above guarantees a supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let effective_max_dex = CHAIN_SHIRT_MAX_DEX + fighter_armor_training(level).max_dex_increase;
    let dexterity_modifier = ability_modifiers.dexterity;
    let dexterity_contribution = dexterity_modifier.min(effective_max_dex);
    let armor_class =
        ARMOR_CLASS_BASE + CHAIN_SHIRT_ARMOR_BONUS + dexterity_contribution + DODGE_AC_BONUS;

    explanations.push(ComputationExplanation {
        id: "defense.baseline_armor_class".to_owned(),
        value: armor_class,
        detail: format!(
            "Baseline armor class: base {ARMOR_CLASS_BASE} + Chain Shirt armor bonus (+{CHAIN_SHIRT_ARMOR_BONUS}) \
             + Dexterity contribution (+{dexterity_contribution}, DEX modifier +{dexterity_modifier} within MAXDEX:{effective_max_dex}) \
             + Dodge (+{DODGE_AC_BONUS}); shield is absent (+0) = {armor_class}"
        ),
    });

    (melee_attack_bonus, armor_class)
}

/// Return the list of unmet conditions for the exact deterministic combat
/// posture. An empty list means the posture is fully supported.
fn unmet_combat_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let chosen = &input.chosen;
    let mut unmet = Vec::new();

    if supported_fighter_level(input).is_none() {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} chassis"
        ));
    }

    require_active_state(
        input,
        LONGSWORD_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(input, SHIELD_ITEM_ID, ActiveState::Absent, &mut unmet);
    require_active_state(
        input,
        POWER_ATTACK_ITEM_ID,
        ActiveState::SelectedInactive,
        &mut unmet,
    );

    if !chosen.selected_feats.iter().any(|f| f == DODGE_FEAT_ID) {
        unmet.push(format!("missing selected feat {DODGE_FEAT_ID}"));
    }
    if !chosen
        .selected_feats
        .iter()
        .any(|f| f == WEAPON_FOCUS_FEAT_ID)
    {
        unmet.push(format!("missing selected feat {WEAPON_FOCUS_FEAT_ID}"));
    }

    let fighter_bonus_selection = choice_selection(input, FIGHTER_BONUS_FEAT_CHOICE_ID);
    if fighter_bonus_selection != Some(WEAPON_FOCUS_LONGSWORD_SELECTION) {
        unmet.push(format!(
            "{FIGHTER_BONUS_FEAT_CHOICE_ID} selection must be {WEAPON_FOCUS_LONGSWORD_SELECTION}, got {fighter_bonus_selection:?}"
        ));
    }

    unmet
}

/// Record an unmet condition unless the named item has exactly `expected` state.
fn require_active_state(
    input: &CharacterInput,
    item_id: &str,
    expected: ActiveState,
    unmet: &mut Vec<String>,
) {
    let actual = input
        .chosen
        .equipment_selections
        .iter()
        .find(|e| e.item_id == item_id)
        .map(|e| e.active_state);
    if actual != Some(expected) {
        unmet.push(format!(
            "{item_id} must be {expected:?} for the deterministic baseline, got {actual:?}"
        ));
    }
}
