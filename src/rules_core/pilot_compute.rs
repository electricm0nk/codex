//! GE06-E2-F2a base rules-core computation slice.
//!
//! Computes only the first base outputs from a loaded [`CharacterInput`]:
//! ability modifiers, Fighter level-1 base attack bonus, and Fighter level-1
//! base saves, each with a machine-checkable explanation record.
//!
//! This is intentionally not the rules engine. It does not compute armor class,
//! attack bonus, skill modifiers, equipment effects, feat prerequisites, or any
//! oracle parity. Class chassis support is limited to Fighter level 1; any other
//! input yields a claim-blocking diagnostic rather than fabricated values.

use super::character_input::{AbilityScores, CharacterInput};

/// Result of the GE06-E2-F2a base chassis computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotBaseChassisComputation {
    pub ability_modifiers: AbilityModifiers,
    /// Class/base attack bonus only. Zero when the chassis is unsupported.
    pub base_attack_bonus: i16,
    /// Class/base save bonuses only (no ability modifiers added in this slice).
    pub base_saves: BaseSaves,
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

/// Compute the GE-06 pilot base chassis from a loaded character input.
pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation {
    let mut explanations = Vec::new();
    let mut diagnostics = Vec::new();

    let ability_modifiers =
        compute_ability_modifiers(&input.chosen.ability_scores, &mut explanations);

    let (base_attack_bonus, base_saves) =
        compute_fighter_chassis(input, &mut explanations, &mut diagnostics);

    PilotBaseChassisComputation {
        ability_modifiers,
        base_attack_bonus,
        base_saves,
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

/// Compute the Fighter level-1 base chassis, or block the claim if the input is
/// not the supported Fighter level 1 chassis for this narrow slice.
fn compute_fighter_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, BaseSaves) {
    let has_fighter_level_1 = input
        .chosen
        .class_levels
        .iter()
        .any(|cl| cl.class_id == FIGHTER_CLASS_ID && cl.level == 1);

    if !has_fighter_level_1 {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis is only supported for {FIGHTER_CLASS_ID} level 1; \
                 chosen class levels {:?} do not provide it, so no chassis values were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
    }

    // Grounded Fighter level-1 base values from cr_classes.lst:139.
    //   BONUS:COMBAT|BASEAB|classlevel              -> 1
    //   BONUS:SAVE|BASE.Fortitude|classlevel/2+2    -> 2
    //   BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 -> 0
    let base_attack_bonus = 1;
    let base_saves = BaseSaves {
        fortitude: 2,
        reflex: 0,
        will: 0,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: "Fighter level 1 base attack bonus from cr_classes.lst:139 \
                 BONUS:COMBAT|BASEAB|classlevel = 1"
            .to_owned(),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: "Fighter level 1 base Fortitude save from cr_classes.lst:139 \
                 BONUS:SAVE|BASE.Fortitude|classlevel/2+2 = 2"
            .to_owned(),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: "Fighter level 1 base Reflex save from cr_classes.lst:139 \
                 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = 0"
            .to_owned(),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: "Fighter level 1 base Will save from cr_classes.lst:139 \
                 BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = 0"
            .to_owned(),
    });

    (base_attack_bonus, base_saves)
}
