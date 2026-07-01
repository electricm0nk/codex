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
//! spellcasting, multiclassing, or non-Fighter classes. Unsupported input yields
//! claim-blocking diagnostics and withheld explanations rather than fabricated
//! values.

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

// Grounded Human pilot race seam identities. These name the already-accepted
// deterministic Human selections; this slice makes their pressure explicit but
// grounds no non-Human race semantics and no broader Human racial trait burden.
const HUMAN_RACE_ID: &str = "race:human";
const HUMAN_ABILITY_BONUS_CHOICE_ID: &str = "choice:human_ability_bonus";
const HUMAN_BONUS_FEAT_CHOICE_ID: &str = "choice:human_bonus_feat";
const ABILITY_SELECTION_PREFIX: &str = "ability:";

// Grounded deterministic combat-baseline contributors and posture identities.
const LONGSWORD_ITEM_ID: &str = "item:longsword";
const CHAIN_SHIRT_ITEM_ID: &str = "item:chain_shirt";
const SHIELD_ITEM_ID: &str = "item:shield";
const POWER_ATTACK_ITEM_ID: &str = "power_attack";
const DODGE_FEAT_ID: &str = "feat:dodge";
const WEAPON_FOCUS_FEAT_ID: &str = "feat:weapon_focus";
const FIGHTER_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat";
const WEAPON_FOCUS_LONGSWORD_SELECTION: &str = "feat:weapon_focus:weapon:longsword";

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

    explain_human_race_seam(input, &ability_modifiers, &mut explanations, &mut diagnostics);

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
    if let Some(selection) = human_choice_selection(input, HUMAN_ABILITY_BONUS_CHOICE_ID) {
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
    if let Some(selection) = human_choice_selection(input, HUMAN_BONUS_FEAT_CHOICE_ID) {
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
        message: "Human race semantics are grounded only for the deterministic pilot's named \
                  ability-bonus and bonus-feat selections; Human size, speed, senses, extra skill \
                  ranks, and the remaining racial trait burden remain unverified"
            .to_owned(),
        claim_blocking: false,
    });
}

/// Return the selection id chosen for the named choice set, if present.
fn human_choice_selection<'a>(input: &'a CharacterInput, choice_set_id: &str) -> Option<&'a str> {
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
    let mut fighter_level = None;
    for class_level in &input.chosen.class_levels {
        if class_level.class_id == FIGHTER_CLASS_ID
            && (1..=MAX_SUPPORTED_FIGHTER_LEVEL).contains(&class_level.level)
        {
            fighter_level = Some(class_level.level);
        } else {
            return None;
        }
    }
    fighter_level
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

    if level >= 2 {
        if let Some(selection) =
            human_choice_selection(input, FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID)
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

/// Compute total saving throws as the grounded Fighter level-1 base save plus the
/// relevant ability modifier, or block the claim if the Fighter level-1 chassis
/// is absent.
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
/// The bounded posture requires the Fighter level-1 chassis, exactly the three
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
    let armor_class = ARMOR_CLASS_BASE
        + CHAIN_SHIRT_ARMOR_BONUS
        + dexterity_contribution
        + DODGE_AC_BONUS;

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
    if !chosen.selected_feats.iter().any(|f| f == WEAPON_FOCUS_FEAT_ID) {
        unmet.push(format!("missing selected feat {WEAPON_FOCUS_FEAT_ID}"));
    }

    let fighter_bonus_selection = chosen
        .selected_choices
        .iter()
        .find(|c| c.choice_set_id == FIGHTER_BONUS_FEAT_CHOICE_ID)
        .map(|c| c.selection_id.as_str());
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
