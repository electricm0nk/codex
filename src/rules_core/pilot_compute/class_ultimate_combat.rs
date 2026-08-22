//! Ultimate Combat's three classes (Gunslinger, Ninja, Samurai) split out of
//! `pilot_compute.rs` (SD31-E4-F1-005), a pure code-move with unchanged behaviour.
//! `use super::*;` gives this submodule the same visibility into `pilot_compute`'s
//! private items its functions relied on before the move (child modules see a
//! parent's private items in Rust). Nothing here is called from outside
//! `pilot_compute`, except `compute_uc_class_chassis`, re-exported by `mod.rs`.

use super::*;

/// SD31-E4-F1-002 (epic-4-mechanism F1, Ultimate Combat's first class):
/// compute the base-attack-bonus / base-save chassis pillar for
/// Gunslinger, then ground the named features this cycle wires.
///
/// Structurally identical to `compute_acg_class_chassis`/
/// `compute_pu_class_chassis` -- same explanation ids, same
/// `class_chassis.unsupported` diagnostic shape, same "chassis first,
/// then per-class grounding" order. Only ever called from
/// `compute_class_chassis`'s single-class-only section, and
/// `UcClassId::from_class_id_str` is deliberately NOT registered with
/// `table_class_id`, matching every other book's own non-CRB dispatch
/// branch.
pub(super) fn compute_uc_class_chassis(
    class_id: UcClassId,
    class_id_str: &str,
    level: u8,
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> Option<(i16, BaseSaves)> {
    let Some(row) = uc::class_chassis_resolve(class_id, level, RuleSetId::Uc) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis has no {class_id_str} UC class_chassis_resolve row at \
                 level {level} (exceeds this class's real MAXLEVEL ceiling), so no chassis \
                 values were computed"
            ),
            claim_blocking: true,
        });
        return None;
    };

    let base_attack_bonus = row.base_attack_bonus;
    let base_saves = BaseSaves {
        fortitude: row.fort_save,
        reflex: row.ref_save,
        will: row.will_save,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_id_str} level {level} base attack bonus from \
             rules_tables::ultimate_combat::class_chassis_resolve's row for this class: \
             {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "{class_id_str} level {level} base Fortitude save from \
             rules_tables::ultimate_combat::class_chassis_resolve's row for this class: {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "{class_id_str} level {level} base Reflex save from \
             rules_tables::ultimate_combat::class_chassis_resolve's row for this class: {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "{class_id_str} level {level} base Will save from \
             rules_tables::ultimate_combat::class_chassis_resolve's row for this class: {}",
            base_saves.will
        ),
    });

    if class_id == UcClassId::Gunslinger {
        ground_or_block_gunslinger_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == UcClassId::Ninja {
        ground_or_block_ninja_class_features(
            input,
            level,
            ability_modifiers,
            explanations,
            diagnostics,
        );
    } else if class_id == UcClassId::Samurai {
        ground_or_block_samurai_class_features(level, explanations, diagnostics);
    }

    Some((base_attack_bonus, base_saves))
}

/// Gunslinger's Grit points gained at the start of each day: `max(1, WIS)`
/// on the base progression (`BONUS:VAR|GunslingerGritPoints|MAX(1,WIS)`,
/// `KEY:Gunslinger ~ Grit`), or `max(1, CHA)` when the Mysterious Stranger
/// archetype supersedes this slot (`BONUS:VAR|GunslingerGritPoints|
/// MAX(1,CHA)`, `KEY:MYSTERIOUS STRANGER ~ Grit`).
fn gunslinger_grit_points(ability_modifier: i16) -> i16 {
    ability_modifier.max(1)
}

/// Gunslinger's Grit limit (the ceiling grit normally cannot exceed):
/// `WIS` on the base progression, `CHA` under Mysterious Stranger. Unlike
/// the points formula this has no floor -- a genuinely low ability score
/// can produce a non-positive limit, transcribed as-is rather than
/// smoothed to a RAW assumption the corpus token does not state.
fn gunslinger_grit_limit(ability_modifier: i16) -> i16 {
    ability_modifier
}

/// Gunslinger's Nimble dodge bonus to AC: `(level+2)/4`, starting at 2nd
/// level (`BONUS:VAR|GunslingerDodgeBonus|(GunslingerLVL+2)/4`, `KEY:
/// Gunslinger ~ Nimble`). The real corpus row additionally conditions
/// this on wearing light or no armor (`PREVARLT` gated on medium+ armor
/// equipped) -- no armor-weight-class consumer exists anywhere in this
/// engine yet (verified: no `EQTYPE.ARMOR` reader in `pilot_compute.rs`),
/// so this grounds as a standalone flat fact, the same
/// missing-consumer-does-not-block-a-correct-number idiom Slayer's
/// Studied Target already establishes.
fn gunslinger_nimble_dodge_bonus(level: u8) -> i16 {
    (i16::from(level) + 2) / 4
}

/// Gunslinger's Gun Training count -- how many firearm types she has
/// gained the Dexterity-damage/reduced-misfire benefit for:
/// `(level-1)/4`, starting at 5th level (`BONUS:VAR|GunTrainingSelection|
/// (GunslingerLVL-1)/4`, `KEY:Gunslinger ~ Gun Training`). Grounds the
/// COUNT only -- which firearm TYPE(s) were picked is a chooser
/// (`BONUS:ABILITYPOOL|Gun Training Choice|...`) this engine does not
/// model, the same count-vs-choice split Slayer Talents already
/// establishes for its own pool.
fn gunslinger_gun_training_count(level: u8) -> i16 {
    (i16::from(level) - 1) / 4
}

/// Grounds Gunslinger's Grit, Nimble, Gun Training and Gunslinger
/// Initiative with the real archetype-supersession `if let`/`else` shape
/// SD31-E4-F1's acceptance names, using
/// `archetype_resolver::archetype_claiming_slot_entry` against 2 of
/// Gunslinger's own 4 real archetypes (Pistolero supersedes Gun
/// Training; Mysterious Stranger supersedes both Grit and Nimble). See
/// this cycle's own `docs/release/.../artifacts/OPEN-ISSUES.md` entry for
/// the honest remainder this function does not yet ground.
fn ground_or_block_gunslinger_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Grit: superseded by Mysterious Stranger (uses CHA instead of WIS),
    // base progression otherwise (WIS).
    let grit_claim =
        archetype_resolver::archetype_claiming_slot_entry(input, "Gunslinger", "GunslingerGrit");
    let (grit_ability, grit_ability_name) = match grit_claim {
        Some(_) => (ability_modifiers.charisma, "Charisma"),
        None => (ability_modifiers.wisdom, "Wisdom"),
    };
    let grit_points = gunslinger_grit_points(grit_ability);
    let grit_limit = gunslinger_grit_limit(grit_ability);
    let grit_detail = match grit_claim {
        Some(entry) => {
            let own_grant = entry.grants.iter().find(|g| g.grants_feature_key.ends_with("~ Grit"));
            match own_grant.and_then(|g| g.description) {
                Some(text) => format!(
                    "Gunslinger Grit: superseded by the selected {} archetype (corpus KEY:{}), \
                     which replaces the base grant. {}'s own text: \"{text}\". At level {level}, \
                     {grit_ability_name} modifier {grit_ability} gives {grit_points} grit \
                     point(s) at the start of each day (max(1,{grit_ability_name})), with a \
                     grit limit of {grit_limit} ({grit_ability_name})",
                    entry.archetype_name, entry.key, entry.archetype_name
                ),
                None => format!(
                    "Gunslinger Grit: superseded by the selected {} archetype (corpus KEY:{}); \
                     its own replacement text is not resolved in this catalog entry. At level \
                     {level}, {grit_ability_name} modifier {grit_ability} gives {grit_points} \
                     grit point(s), with a grit limit of {grit_limit}",
                    entry.archetype_name, entry.key
                ),
            }
        }
        None => format!(
            "Gunslinger level {level} Grit: at the start of each day, a gunslinger gains \
             max(1, {grit_ability_name} modifier) grit points ({grit_ability_name} modifier \
             {grit_ability} -> {grit_points}), capped at a grit limit of {grit_ability_name} \
             modifier ({grit_limit}). Grit is spent on deeds and regained on a firearm critical \
             hit or killing blow -- this engine tracks no per-encounter combat log, so only the \
             daily-refresh points and the limit are grounded here, not the mid-combat regain \
             triggers"
        ),
    };
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.gunslinger.grit".to_owned(),
        value: grit_points,
        detail: grit_detail,
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.gunslinger.grit_limit".to_owned(),
        value: grit_limit,
        detail: format!(
            "Gunslinger level {level} Grit limit: {grit_ability_name} modifier = {grit_limit}"
        ),
    });

    // Nimble: superseded by Mysterious Stranger's Lucky (a Will-save luck
    // bonus with the identical (level+2)/4 formula, not an AC dodge
    // bonus).
    let nimble_bonus = gunslinger_nimble_dodge_bonus(level);
    let nimble_claim =
        archetype_resolver::archetype_claiming_slot_entry(input, "Gunslinger", "GunslingerNimble");
    if nimble_bonus > 0 || nimble_claim.is_some() {
        let nimble_detail = match nimble_claim {
            Some(entry) => {
                let own_grant =
                    entry.grants.iter().find(|g| g.grants_feature_key.ends_with("~ Lucky"));
                match own_grant.and_then(|g| g.description) {
                    Some(text) => format!(
                        "Gunslinger Nimble: superseded by the selected {} archetype (corpus \
                         KEY:{}), which replaces the base dodge-bonus grant with a Will-save \
                         luck bonus instead. {}'s own text: \"{text}\". At level {level} the \
                         shared (level+2)/4 formula gives +{nimble_bonus}",
                        entry.archetype_name, entry.key, entry.archetype_name
                    ),
                    None => format!(
                        "Gunslinger Nimble: superseded by the selected {} archetype (corpus \
                         KEY:{}); its own replacement text is not resolved in this catalog \
                         entry",
                        entry.archetype_name, entry.key
                    ),
                }
            }
            None => format!(
                "Gunslinger level {level} Nimble: a +{nimble_bonus} dodge bonus to AC while \
                 wearing light or no armor ((level+2)/4, first at 2nd level, max +5 at 20th). \
                 No armor-weight-class consumer exists in this engine yet, so the light-armor \
                 precondition is not modelled; the magnitude itself is grounded as a standalone \
                 flat fact"
            ),
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.gunslinger.nimble".to_owned(),
            value: nimble_bonus,
            detail: nimble_detail,
        });
    }

    // Gun Training: superseded by Pistolero (flat DEX damage bonus on
    // one-handed firearms instead of a firearm-type count).
    let gun_training_count = gunslinger_gun_training_count(level);
    let gun_training_claim = archetype_resolver::archetype_claiming_slot_entry(
        input,
        "Gunslinger",
        "GunslingerGunTraining",
    );
    if gun_training_count > 0 || gun_training_claim.is_some() {
        let (gun_training_value, gun_training_detail) = match gun_training_claim {
            Some(entry) => {
                let own_grant = entry
                    .grants
                    .iter()
                    .find(|g| g.grants_feature_key.ends_with("~ Pistol Training"));
                let detail = match own_grant.and_then(|g| g.description) {
                    Some(text) => format!(
                        "Gunslinger Gun Training: superseded by the selected {} archetype \
                         (corpus KEY:{}), which replaces gun training 1 to 4 with a single \
                         scaling one-handed-firearm damage bonus. {}'s own text: \"{text}\"",
                        entry.archetype_name, entry.key, entry.archetype_name
                    ),
                    None => format!(
                        "Gunslinger Gun Training: superseded by the selected {} archetype \
                         (corpus KEY:{}); its own replacement text is not resolved in this \
                         catalog entry",
                        entry.archetype_name, entry.key
                    ),
                };
                (0, detail)
            }
            None => (
                gun_training_count,
                format!(
                    "Gunslinger level {level} Gun Training: {gun_training_count} firearm \
                     type(s) selected ((level-1)/4, first at 5th level). Grounds the COUNT \
                     only -- which firearm type(s) were picked is a chooser \
                     (BONUS:ABILITYPOOL) this engine does not model, the same count-vs-choice \
                     split Slayer Talents already establishes"
                ),
            ),
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.gunslinger.gun_training".to_owned(),
            value: gun_training_value,
            detail: gun_training_detail,
        });
    }

    // Gunslinger Initiative (Deed, 3rd level): a flat +2 bonus on
    // initiative checks, conditioned on the gunslinger having at least 1
    // grit point at the moment of the check -- this engine tracks no
    // per-encounter grit-spend state, so the condition is named but not
    // gated on, the same standalone-fact idiom Slayer's own
    // opponent-conditional bonuses already use.
    if level >= 3 {
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.gunslinger.gunslinger_initiative".to_owned(),
            value: 2,
            detail: format!(
                "Gunslinger level {level} Gunslinger Initiative (Deed): a +2 bonus on \
                 initiative checks, as long as the gunslinger has at least 1 grit point (flat, \
                 not level-scaled -- BONUS:VAR|GunslingerInitiative|2). This engine tracks no \
                 per-encounter grit-spend state, so the grit>=1 precondition is named but not \
                 gated on; the magnitude itself is correct regardless"
            ),
        });
    }

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.uc.gunslinger.other_features_deferred.unsupported".to_owned(),
        message: "Gunslinger now grounds its base-attack-bonus/base-save chassis pillar, Grit \
             (points and limit), Nimble's dodge bonus, Gun Training's count, and Gunslinger \
             Initiative's flat bonus -- SD31-E4-F1-002's own new wiring, with Grit/Nimble/Gun \
             Training additionally wired through the real archetype_claiming_slot_entry \
             supersession primitive against 2 of Gunslinger's 4 real archetypes (Pistolero, \
             Mysterious Stranger). What stays deferred, honestly: (1) Gunsmith (the starting \
             battered firearm + Gunsmithing bonus feat) and Proficiencies are zero-magnitude \
             grant-only records not yet transcribed here; (2) the remaining Deeds (Quick Clear, \
             Startling Shot, Bleeding Wound, Menacing Shot, Dead Shot, Utility Shot) and True \
             Grit/Cheat Death/Slinger's Luck/Targeting/Lightning Reload/Expert \
             Loading/Stunning Shot/Pistol-Whip/Death's Shot -- Gunslinger's later-level named \
             features -- are not yet transcribed; (3) Gun Tank and Musket Master, Gunslinger's \
             other two real archetypes, are not yet added to the archetype-swap catalog. This \
             diagnostic is not claim-blocking for the features that ARE grounded above; it \
             carries the honest remainder"
            .to_owned(),
        claim_blocking: false,
    });
}

#[cfg(test)]
mod gunslinger_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ComputationExplanation,
        PilotHeadlessReceipt,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const GUNSLINGER_CLASS_ID: &str = "class:gunslinger";

    /// WIS 12 (+1 mod), CHA 8 (-1 mod) -- the fixture's own real ability
    /// scores, unmodified.
    fn character(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: GUNSLINGER_CLASS_ID.to_owned(), level }];
        input
    }

    fn with_archetype(level: u8, key: &str) -> CharacterInput {
        let mut input = character(level);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: key.to_owned(),
        });
        input
    }

    fn find<'a>(receipt: &'a PilotHeadlessReceipt, id: &str) -> Option<&'a ComputationExplanation> {
        receipt.computation.explanations.iter().find(|e| e.id == id)
    }

    /// The base chassis pillar: full BAB, good Fort/Reflex, poor Will,
    /// matching `class_gunslinger.rs`'s own formulas.
    #[test]
    fn gunslinger_base_chassis_grounds_bab_and_saves() {
        let receipt = build_pilot_headless_receipt(&character(5));
        assert_eq!(find(&receipt, "class_chassis.base_attack_bonus").unwrap().value, 5);
        assert_eq!(find(&receipt, "class_chassis.base_save.fortitude").unwrap().value, 4);
        assert_eq!(find(&receipt, "class_chassis.base_save.reflex").unwrap().value, 4);
        assert_eq!(find(&receipt, "class_chassis.base_save.will").unwrap().value, 1);
    }

    /// Grit, base case, no archetype selected: WIS modifier (+1) drives
    /// both the points (max(1,1)=1) and the limit (1).
    #[test]
    fn grit_grounds_from_wisdom_with_no_archetype_selected() {
        let receipt = build_pilot_headless_receipt(&character(1));
        let grit = find(&receipt, "class_feature.uc.gunslinger.grit").expect("grit must ground");
        assert_eq!(grit.value, 1);
        assert!(grit.detail.contains("Wisdom"), "{grit:?}");
        assert!(!grit.detail.to_lowercase().contains("superseded"), "{grit:?}");
        let limit = find(&receipt, "class_feature.uc.gunslinger.grit_limit").expect("limit");
        assert_eq!(limit.value, 1);
    }

    /// Grit, superseded by Mysterious Stranger: CHA modifier (-1) drives
    /// both figures instead (max(1,-1)=1 points, -1 limit) and the
    /// archetype's own real corpus text is quoted, not the base grant's.
    #[test]
    fn grit_is_superseded_by_mysterious_stranger_using_charisma_instead_of_wisdom() {
        let input = with_archetype(1, "Gunslinger Archetype ~ Mysterious Stranger");
        let receipt = build_pilot_headless_receipt(&input);
        let grit = find(&receipt, "class_feature.uc.gunslinger.grit").expect("grit must ground");
        assert_eq!(grit.value, 1, "max(1,-1) = 1");
        assert!(grit.detail.contains("Mysterious Stranger"), "{grit:?}");
        assert!(grit.detail.contains("Instead of using her Wisdom"), "{grit:?}");
        let limit = find(&receipt, "class_feature.uc.gunslinger.grit_limit").expect("limit");
        assert_eq!(limit.value, -1, "CHA modifier -1, transcribed as-is");
    }

    /// Nimble, base case: +1 dodge bonus at level 2 ((2+2)/4).
    #[test]
    fn nimble_grounds_the_base_dodge_bonus_from_second_level() {
        let receipt = build_pilot_headless_receipt(&character(2));
        let nimble = find(&receipt, "class_feature.uc.gunslinger.nimble").expect("nimble");
        assert_eq!(nimble.value, 1);
        assert!(!nimble.detail.to_lowercase().contains("superseded"), "{nimble:?}");
        // Not yet granted at level 1.
        let receipt1 = build_pilot_headless_receipt(&character(1));
        assert!(find(&receipt1, "class_feature.uc.gunslinger.nimble").is_none());
    }

    /// Nimble, superseded by Mysterious Stranger's Lucky: same formula,
    /// different mechanic (Will save luck bonus, not AC dodge), and the
    /// archetype's own text is quoted.
    #[test]
    fn nimble_is_superseded_by_mysterious_strangers_lucky() {
        let input = with_archetype(2, "Gunslinger Archetype ~ Mysterious Stranger");
        let receipt = build_pilot_headless_receipt(&input);
        let nimble = find(&receipt, "class_feature.uc.gunslinger.nimble").expect("nimble");
        assert_eq!(nimble.value, 1);
        assert!(nimble.detail.contains("Mysterious Stranger"), "{nimble:?}");
        assert!(nimble.detail.contains("luck bonus"), "{nimble:?}");
    }

    /// Gun Training, base case: count of firearm types trained,
    /// (level-1)/4 starting at 5th level.
    #[test]
    fn gun_training_grounds_the_base_count_from_fifth_level() {
        let receipt = build_pilot_headless_receipt(&character(9));
        let gt = find(&receipt, "class_feature.uc.gunslinger.gun_training").expect("gun training");
        assert_eq!(gt.value, 2, "(9-1)/4 = 2");
        assert!(!gt.detail.to_lowercase().contains("superseded"), "{gt:?}");
        let receipt4 = build_pilot_headless_receipt(&character(4));
        assert!(find(&receipt4, "class_feature.uc.gunslinger.gun_training").is_none());
    }

    /// Gun Training, superseded by Pistolero: the base count is replaced
    /// by a flat 0-value record carrying Pistol Training's own real
    /// corpus text (a DEX-based damage bonus, not a firearm-type count),
    /// and it grounds even below the base feature's own 5th-level floor
    /// because Pistolero's own text says it replaces "gun training 1 to
    /// 4" -- the whole progression, not only levels 5+.
    #[test]
    fn gun_training_is_superseded_by_pistolero_even_before_the_base_floor() {
        let input = with_archetype(1, "Gunslinger Archetype ~ Pistolero");
        let receipt = build_pilot_headless_receipt(&input);
        let gt = find(&receipt, "class_feature.uc.gunslinger.gun_training")
            .expect("must ground even at level 1, superseded");
        assert_eq!(gt.value, 0);
        assert!(gt.detail.contains("Pistolero"), "{gt:?}");
        assert!(gt.detail.contains("Dexterity modifier"), "{gt:?}");
    }

    /// Gunslinger Initiative: flat +2 from 3rd level, not level-scaled.
    #[test]
    fn gunslinger_initiative_is_a_flat_bonus_from_third_level() {
        let receipt3 = build_pilot_headless_receipt(&character(3));
        assert_eq!(
            find(&receipt3, "class_feature.uc.gunslinger.gunslinger_initiative").unwrap().value,
            2
        );
        let receipt20 = build_pilot_headless_receipt(&character(20));
        assert_eq!(
            find(&receipt20, "class_feature.uc.gunslinger.gunslinger_initiative").unwrap().value,
            2,
            "flat, not level-scaled"
        );
        let receipt2 = build_pilot_headless_receipt(&character(2));
        assert!(find(&receipt2, "class_feature.uc.gunslinger.gunslinger_initiative").is_none());
    }
}

/// Ninja's Sneak Attack dice: `(NinjaLVL+1)/2`, from the real corpus row
/// (`KEY:Ninja ~ Sneak Attack`, `BONUS:VAR|SneakAttackDice|
/// (NinjaSneakAttackLVL+1)/2`, `NinjaSneakAttackLVL` fed by `NinjaLVL`)
/// -- the identical formula and progression Rogue's own Sneak Attack
/// uses, granted from 1st level.
fn ninja_sneak_attack_dice(level: u8) -> i16 {
    (i16::from(level) + 1) / 2
}

/// Ninja's Ki Pool size: `NinjaLVL/2 + Charisma modifier`, from the real
/// corpus (`class:ninja`'s own `DEFINE:KiPoolCha|0`/`BONUS:VAR|
/// KiPoolCha|1` flag selects the Charisma stat-choice branch of the
/// shared `Ki Pool Tracker` internal ability, whose own `BONUS:VAR|
/// KiPoints|KiPoolLVL/2` base formula is fed `KiPoolLVL` = `NinjaLVL`
/// via `KEY:Ninja ~ Ki Pool`'s own `BONUS:VAR|KiPoolLVL|NinjaLVL`) --
/// the same shared mechanism `class_chassis.monk.ki_pool_size` already
/// grounds for Monk, substituting Charisma for Wisdom per Ninja's own
/// corpus stat-choice flag. Granted from 2nd level
/// (`PREVARGTEQ:Ninja_CFP_Level,2`).
fn ninja_ki_pool_size(level: u8, charisma_modifier: i16) -> i16 {
    i16::from(level) / 2 + charisma_modifier
}

/// Ninja Trick count: `NinjaLVL/2`, from the real corpus row (`KEY:Ninja
/// ~ Ninja Trick`, `BONUS:ABILITYPOOL|Ninja Trick|NinjaTrickLVL/2`,
/// `NinjaTrickLVL` fed by `NinjaLVL`). Grounds the COUNT only -- WHICH
/// trick(s) were picked is a chooser (`BONUS:ABILITYPOOL`) this engine
/// does not model, the same count-vs-choice split Slayer Talents and
/// Gunslinger Gun Training already establish. Granted from 2nd level
/// (`PREVARGTEQ:Ninja_CFP_Level,2`).
fn ninja_trick_count(level: u8) -> i16 {
    i16::from(level) / 2
}

/// Ninja's No Trace bonus: `NinjaLVL/3`, from the real corpus row
/// (`KEY:Ninja ~ No Trace`, `BONUS:VAR|NoTraceBonus|NinjaNoTraceLVL/3`,
/// `NinjaNoTraceLVL` fed by `NinjaLVL`) -- an insight bonus to the DC to
/// track the ninja via Survival, and on Disguise/Stealth checks while
/// stationary. Granted from 3rd level (`PREVARGTEQ:Ninja_CFP_Level,3`).
fn ninja_no_trace_bonus(level: u8) -> i16 {
    i16::from(level) / 3
}

/// Grounds Ninja's Sneak Attack, Ki Pool, Ninja Trick count and No Trace
/// unconditionally, then Uncanny Dodge (4th level) and Improved Uncanny
/// Dodge (8th level) with the real archetype-supersession `if let`/
/// `else` shape SD31-E4-F1's acceptance names, using
/// `archetype_resolver::archetype_claiming_slot_entry` against Ninja's
/// one real archetype (Scout, which replaces both Uncanny Dodge slots
/// with Scout's Charge/Skirmisher -- see `archetype_tables.rs`'s own doc
/// comment for why `replaces` here is `FACT:`-derived rather than the
/// usual `TYPE:`-derived convention). See this cycle's own
/// `OPEN-ISSUES.md` entry for the honest remainder this function does
/// not yet ground (Poison Use, Light Steps, Hidden Master, Weapon
/// Proficiencies -- zero/flat-only grant-only records not yet
/// transcribed) and the row-96-shaped structural blocker
/// (`v06_work_inventory.rs`'s `modelled_class_books()` does not know
/// Ultimate Combat's classes at all, out of this cycle's file territory)
/// that caps every one of these at `held`/board-invisible regardless.
fn ground_or_block_ninja_class_features(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Sneak Attack: no archetype in the 23-book scope claims this slot
    // for Ninja (Scout's own `replaces` list names only the two Uncanny
    // Dodge slots), so this grounds unconditionally.
    let sneak_dice = ninja_sneak_attack_dice(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.ninja.sneak_attack".to_owned(),
        value: sneak_dice,
        detail: format!(
            "Ninja level {level} Sneak Attack: extra {sneak_dice}d6 precision damage \
             ((level+1)/2) anytime the target would be denied a Dexterity bonus to AC, or when \
             the ninja flanks her target. Ranged attacks count as sneak attacks only if the \
             target is within 30 feet"
        ),
    });

    // Ki Pool: granted from 2nd level, uses Charisma (Ninja's own
    // corpus stat-choice flag, unlike Monk's Wisdom).
    if level >= 2 {
        let ki_pool = ninja_ki_pool_size(level, ability_modifiers.charisma);
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.ninja.ki_pool".to_owned(),
            value: ki_pool,
            detail: format!(
                "Ninja level {level} Ki Pool: {ki_pool} ki points (level/2 + Charisma modifier \
                 {}, per the shared Ki Pool Tracker mechanism's Charisma stat-choice branch). As \
                 long as she has at least 1 point, she treats Acrobatics jump checks as if she \
                 had a running start; she can spend points for a bonus attack, +20 feet of \
                 speed, or a +4 insight bonus on Stealth checks",
                ability_modifiers.charisma
            ),
        });

        let trick_count = ninja_trick_count(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.ninja.ninja_trick_count".to_owned(),
            value: trick_count,
            detail: format!(
                "Ninja level {level} Ninja Trick: {trick_count} trick(s) known (level/2, first \
                 at 2nd level, one additional every 2 levels thereafter). Grounds the COUNT \
                 only -- which trick(s) were picked is a chooser (BONUS:ABILITYPOOL) this \
                 engine does not model, the same count-vs-choice split Slayer Talents and \
                 Gunslinger Gun Training already establish"
            ),
        });
    }

    // No Trace: granted from 3rd level.
    if level >= 3 {
        let no_trace = ninja_no_trace_bonus(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.ninja.no_trace".to_owned(),
            value: no_trace,
            detail: format!(
                "Ninja level {level} No Trace: +{no_trace} to the DC to track the ninja via \
                 Survival (level/3), and a +{no_trace} insight bonus on Disguise checks and on \
                 opposed Stealth checks while stationary and taking no action for at least 1 \
                 round"
            ),
        });
    }

    // Uncanny Dodge (4th level): superseded by Scout's Scout's Charge.
    if level >= 4 {
        let claim =
            archetype_resolver::archetype_claiming_slot_entry(input, "Ninja", "NinjaUncannyDodge");
        let (value, detail) = match claim {
            Some(entry) => {
                let own_grant =
                    entry.grants.iter().find(|g| g.grants_feature_key.ends_with("~ Scout's Charge"));
                let detail = match own_grant.and_then(|g| g.description) {
                    Some(text) => format!(
                        "Ninja Uncanny Dodge: superseded by the selected {} archetype (corpus \
                         KEY:{}) at 4th level, which replaces this base-class slot with Scout's \
                         Charge. {}'s own text: \"{text}\"",
                        entry.archetype_name, entry.key, entry.archetype_name
                    ),
                    None => format!(
                        "Ninja Uncanny Dodge: superseded by the selected {} archetype (corpus \
                         KEY:{}) at 4th level; its own replacement text is not resolved in this \
                         catalog entry",
                        entry.archetype_name, entry.key
                    ),
                };
                (0, detail)
            }
            None => (
                0,
                "Ninja level 4 Uncanny Dodge: \"You can react to danger before your senses \
                 would normally allow you to do so. You cannot be caught flat-footed, nor do \
                 you lose your Dexterity bonus to AC if the attacker is invisible. You still \
                 lose your Dexterity bonus to AC if immobilized. You can still lose your \
                 Dexterity bonus to AC if an opponent successfully uses the feint action \
                 against you.\" A bounded grant-only identity record (value 0, non-fabricated): \
                 the base row carries no BONUS: magnitude of its own beyond the shared \
                 UncannyDodgeFlankingLevel context-fact, which needs an opposing rogue's level \
                 to matter and is out of this engine's per-character scope"
                    .to_owned(),
            ),
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.ninja.uncanny_dodge".to_owned(),
            value,
            detail,
        });
    }

    // Improved Uncanny Dodge (8th level): superseded by Scout's
    // Skirmisher.
    if level >= 8 {
        let claim = archetype_resolver::archetype_claiming_slot_entry(
            input,
            "Ninja",
            "NinjaImprovedUncannyDodge",
        );
        let (value, detail) = match claim {
            Some(entry) => {
                let own_grant =
                    entry.grants.iter().find(|g| g.grants_feature_key.ends_with("~ Skirmisher"));
                let detail = match own_grant.and_then(|g| g.description) {
                    Some(text) => format!(
                        "Ninja Improved Uncanny Dodge: superseded by the selected {} archetype \
                         (corpus KEY:{}) at 8th level, which replaces this base-class slot with \
                         Skirmisher. {}'s own text: \"{text}\"",
                        entry.archetype_name, entry.key, entry.archetype_name
                    ),
                    None => format!(
                        "Ninja Improved Uncanny Dodge: superseded by the selected {} archetype \
                         (corpus KEY:{}) at 8th level; its own replacement text is not resolved \
                         in this catalog entry",
                        entry.archetype_name, entry.key
                    ),
                };
                (0, detail)
            }
            None => (
                0,
                "Ninja level 8 Improved Uncanny Dodge: \"You can no longer be flanked. This \
                 defense denies a rogue the ability to sneak attack you by flanking you, unless \
                 the attacker is a rogue of at least level X.\" A bounded grant-only identity \
                 record (value 0, non-fabricated): the flanking-rogue-level threshold is a \
                 context fact about an opposing character, out of this engine's per-character \
                 scope, the same unmodeled-precondition idiom Gunslinger Initiative's grit>=1 \
                 precondition already establishes"
                    .to_owned(),
            ),
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.ninja.improved_uncanny_dodge".to_owned(),
            value,
            detail,
        });
    }

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.uc.ninja.other_features_deferred.unsupported".to_owned(),
        message: "Ninja now grounds Sneak Attack's dice, Ki Pool's size, Ninja Trick's count, \
             No Trace's bonus, and Uncanny Dodge/Improved Uncanny Dodge -- SD31-E4-F1-003's own \
             new wiring, with Uncanny Dodge/Improved Uncanny Dodge additionally wired through \
             the real archetype_claiming_slot_entry supersession primitive against Ninja's one \
             real archetype (Scout). What stays deferred, honestly: (1) Poison Use, Light \
             Steps, Hidden Master and Weapon Proficiencies are zero/flat-only grant-only \
             records not yet transcribed here; (2) all 30 named Ninja Tricks are not yet \
             transcribed (only the trick COUNT is grounded); (3) this class is not registered \
             in v06_work_inventory.rs's modelled_class_books() (out of this cycle's file \
             territory -- reported to OPEN-ISSUES.md), so none of this wiring can reach `done` \
             or even `held` on the board yet regardless of how complete it is, the same \
             structural blocker SD31-E4-F1-002 found for Gunslinger. This diagnostic is not \
             claim-blocking for the features that ARE grounded above; it carries the honest \
             remainder"
            .to_owned(),
        claim_blocking: false,
    });
}

#[cfg(test)]
mod ninja_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ComputationExplanation,
        PilotHeadlessReceipt,
    };
    use crate::rules_core::character_input::{load_character_input_fixture, SelectedChoice};

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const NINJA_CLASS_ID: &str = "class:ninja";

    /// CHA 8 (-1 mod) -- the fixture's own real ability scores, unmodified.
    fn character(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: NINJA_CLASS_ID.to_owned(), level }];
        input
    }

    fn with_scout_archetype(level: u8) -> CharacterInput {
        let mut input = character(level);
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: crate::rules_core::archetype_resolver::ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: "Ninja Archetype ~ Scout".to_owned(),
        });
        input
    }

    fn find<'a>(receipt: &'a PilotHeadlessReceipt, id: &str) -> Option<&'a ComputationExplanation> {
        receipt.computation.explanations.iter().find(|e| e.id == id)
    }

    /// The base chassis pillar: 3/4 BAB, poor Fort, good Reflex, poor
    /// Will, matching `class_ninja.rs`'s own formulas.
    #[test]
    fn ninja_base_chassis_grounds_bab_and_saves() {
        let receipt = build_pilot_headless_receipt(&character(10));
        assert_eq!(find(&receipt, "class_chassis.base_attack_bonus").unwrap().value, 7);
        assert_eq!(find(&receipt, "class_chassis.base_save.fortitude").unwrap().value, 3);
        assert_eq!(find(&receipt, "class_chassis.base_save.reflex").unwrap().value, 7);
        assert_eq!(find(&receipt, "class_chassis.base_save.will").unwrap().value, 3);
    }

    /// Sneak Attack: (level+1)/2 dice, granted from 1st level, no
    /// archetype in scope claims this slot.
    #[test]
    fn sneak_attack_grounds_from_first_level() {
        let receipt1 = build_pilot_headless_receipt(&character(1));
        assert_eq!(find(&receipt1, "class_feature.uc.ninja.sneak_attack").unwrap().value, 1);
        let receipt5 = build_pilot_headless_receipt(&character(5));
        assert_eq!(find(&receipt5, "class_feature.uc.ninja.sneak_attack").unwrap().value, 3);
    }

    /// Ki Pool: level/2 + CHA modifier, granted from 2nd level. Fixture
    /// CHA modifier is -1, so at level 4: 4/2 + (-1) = 1.
    #[test]
    fn ki_pool_grounds_from_second_level_using_charisma() {
        let receipt1 = build_pilot_headless_receipt(&character(1));
        assert!(find(&receipt1, "class_feature.uc.ninja.ki_pool").is_none());
        let receipt4 = build_pilot_headless_receipt(&character(4));
        let ki = find(&receipt4, "class_feature.uc.ninja.ki_pool").expect("ki pool");
        assert_eq!(ki.value, 1, "4/2 + (-1) = 1");
        assert!(ki.detail.contains("Charisma"), "{ki:?}");
    }

    /// Ninja Trick count: level/2, granted from 2nd level alongside Ki
    /// Pool.
    #[test]
    fn ninja_trick_count_grounds_from_second_level() {
        let receipt2 = build_pilot_headless_receipt(&character(2));
        assert_eq!(find(&receipt2, "class_feature.uc.ninja.ninja_trick_count").unwrap().value, 1);
        let receipt8 = build_pilot_headless_receipt(&character(8));
        assert_eq!(find(&receipt8, "class_feature.uc.ninja.ninja_trick_count").unwrap().value, 4);
    }

    /// No Trace: level/3, granted from 3rd level.
    #[test]
    fn no_trace_grounds_from_third_level() {
        let receipt2 = build_pilot_headless_receipt(&character(2));
        assert!(find(&receipt2, "class_feature.uc.ninja.no_trace").is_none());
        let receipt6 = build_pilot_headless_receipt(&character(6));
        assert_eq!(find(&receipt6, "class_feature.uc.ninja.no_trace").unwrap().value, 2);
    }

    /// Uncanny Dodge, base case, no archetype selected: grounds at 4th
    /// level with the real base DESC text, not superseded.
    #[test]
    fn uncanny_dodge_grounds_the_base_grant_from_fourth_level_with_no_archetype() {
        let receipt3 = build_pilot_headless_receipt(&character(3));
        assert!(find(&receipt3, "class_feature.uc.ninja.uncanny_dodge").is_none());
        let receipt4 = build_pilot_headless_receipt(&character(4));
        let ud = find(&receipt4, "class_feature.uc.ninja.uncanny_dodge").expect("uncanny dodge");
        assert_eq!(ud.value, 0);
        assert!(ud.detail.contains("flat-footed"), "{ud:?}");
        assert!(!ud.detail.to_lowercase().contains("superseded"), "{ud:?}");
    }

    /// Uncanny Dodge, superseded by Scout: replaced with Scout's Charge
    /// at the same 4th-level gate, and the archetype's own real corpus
    /// text is quoted, not the base grant's.
    #[test]
    fn uncanny_dodge_is_superseded_by_scout_with_scouts_charge() {
        let receipt = build_pilot_headless_receipt(&with_scout_archetype(4));
        let ud = find(&receipt, "class_feature.uc.ninja.uncanny_dodge").expect("uncanny dodge");
        assert_eq!(ud.value, 0);
        assert!(ud.detail.contains("Scout"), "{ud:?}");
        assert!(ud.detail.contains("Scout's Charge"), "{ud:?}");
        assert!(ud.detail.contains("charge"), "{ud:?}");
    }

    /// Improved Uncanny Dodge, base case: grounds at 8th level, not
    /// superseded.
    #[test]
    fn improved_uncanny_dodge_grounds_the_base_grant_from_eighth_level_with_no_archetype() {
        let receipt7 = build_pilot_headless_receipt(&character(7));
        assert!(find(&receipt7, "class_feature.uc.ninja.improved_uncanny_dodge").is_none());
        let receipt8 = build_pilot_headless_receipt(&character(8));
        let iud = find(&receipt8, "class_feature.uc.ninja.improved_uncanny_dodge")
            .expect("improved uncanny dodge");
        assert_eq!(iud.value, 0);
        assert!(iud.detail.contains("flanked"), "{iud:?}");
        assert!(!iud.detail.to_lowercase().contains("superseded"), "{iud:?}");
    }

    /// Improved Uncanny Dodge, superseded by Scout: replaced with
    /// Skirmisher at the same 8th-level gate.
    #[test]
    fn improved_uncanny_dodge_is_superseded_by_scout_with_skirmisher() {
        let receipt = build_pilot_headless_receipt(&with_scout_archetype(8));
        let iud = find(&receipt, "class_feature.uc.ninja.improved_uncanny_dodge")
            .expect("improved uncanny dodge");
        assert_eq!(iud.value, 0);
        assert!(iud.detail.contains("Scout"), "{iud:?}");
        assert!(iud.detail.contains("Skirmisher"), "{iud:?}");
    }
}

/// Samurai's Challenge uses-per-day: `min((SamuraiChallengeLVL+2)/3, 7)`,
/// from the real corpus row (`KEY:Samurai ~ Challenge`,
/// `BONUS:VAR|SamuraiChallengeTimes|min((SamuraiChallengeLVL+2)/3,7)`,
/// `SamuraiChallengeLVL` fed by `SamuraiLVL`). Integer division applies
/// before the 7-use ceiling, matching the real corpus token exactly.
fn samurai_challenge_uses_per_day(level: u8) -> i16 {
    ((i16::from(level) + 2) / 3).min(7)
}

/// Samurai's Challenge damage bonus: `SamuraiChallengeLVL`, i.e. flat
/// class level, from the real corpus row's
/// `BONUS:VAR|SamuraiChallengeDam|SamuraiChallengeLVL` token -- extra
/// damage the samurai's melee attacks deal against the target of his
/// current challenge.
fn samurai_challenge_damage_bonus(level: u8) -> i16 {
    i16::from(level)
}

/// Samurai's Resolve uses-per-day: `(SamuraiResolveLVL+1)/2`, from the
/// real corpus row (`KEY:Samurai ~ Resolve`,
/// `BONUS:VAR|SamuraiResolveTimes|(SamuraiResolveLVL+1)/2`,
/// `SamuraiResolveLVL` fed by `SamuraiLVL`).
fn samurai_resolve_uses_per_day(level: u8) -> i16 {
    (i16::from(level) + 1) / 2
}

/// Samurai's Bonus Feat count: `SamuraiLVL/6`, from the real corpus row
/// (`KEY:Samurai ~ Bonus Feat`, `BONUS:VAR|SamuraiBonusFeat|SamuraiLVL/6`
/// base formula; `BONUS:ABILITYPOOL|Samurai Feat|SamuraiBonusFeat`
/// dispenses that count as pool slots), granted at 6th level and every
/// six levels thereafter. Grounds the COUNT only -- WHICH bonus feat(s)
/// were picked is a chooser this engine does not model, the same
/// count-vs-choice split Slayer Talents, Gunslinger Gun Training and
/// Ninja Trick already establish.
///
/// Named boundary, deliberately not modelled: the real corpus row also
/// carries three `.MOD` rows (`PRECLASS:1,Samurai=06/12/18` gated
/// `-1` adjustments keyed to `Samurai_CF_BonusFeat6/12/18` flags) that
/// decrement this count under a condition this engine tracks nowhere
/// (a prior-choice flag, not a character stat) -- the base formula this
/// function grounds is the un-decremented one the corpus row itself
/// states as its `DEFINE:SamuraiBonusFeat|0` starting point.
fn samurai_bonus_feat_count(level: u8) -> i16 {
    i16::from(level) / 6
}

/// Grounds Samurai's Challenge (uses/day and damage bonus), Resolve
/// (uses/day) and Bonus Feat (count) unconditionally -- Samurai has no
/// real archetype content anywhere in the 23-book scope
/// (`class_samurai.rs`'s own doc comment carries the full citation), so
/// unlike Gunslinger and Ninja this function has no
/// `archetype_claiming_slot_entry` supersession branch to build; every
/// slot below grounds the base progression outright. See this cycle's
/// own `OPEN-ISSUES.md` entry for the honest remainder this function
/// does not yet ground (Mount, Order, Weapon Expertise, Mounted Archer,
/// Banner and the later-level Resolve-spending abilities -- not yet
/// transcribed) and the row-96-shaped structural blocker
/// (`v06_work_inventory.rs`'s `modelled_class_books()` does not know
/// Ultimate Combat's classes at all, out of this cycle's file territory)
/// that caps every one of these at `held`/board-invisible regardless.
fn ground_or_block_samurai_class_features(
    level: u8,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    // Challenge: granted from 1st level, no archetype in the 23-book
    // scope exists to claim this slot.
    let challenge_uses = samurai_challenge_uses_per_day(level);
    let challenge_damage = samurai_challenge_damage_bonus(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.samurai.challenge_uses".to_owned(),
        value: challenge_uses,
        detail: format!(
            "Samurai level {level} Challenge: {challenge_uses} time(s) per day \
             (min((level+2)/3, 7)), as a swift action the samurai can challenge one target \
             within sight; his melee attacks deal {challenge_damage} extra damage against that \
             target (flat class level) until the target is dead, unconscious, or combat ends. \
             While a challenge is active the samurai takes a -2 penalty to Armor Class, except \
             against the target of his challenge"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.samurai.challenge_damage_bonus".to_owned(),
        value: challenge_damage,
        detail: format!(
            "Samurai level {level} Challenge damage bonus against the target of an active \
             challenge: +{challenge_damage} (flat class level)"
        ),
    });

    // Resolve: granted from 1st level, no archetype in the 23-book scope
    // exists to claim this slot.
    let resolve_uses = samurai_resolve_uses_per_day(level);
    explanations.push(ComputationExplanation {
        id: "class_feature.uc.samurai.resolve_uses".to_owned(),
        value: resolve_uses,
        detail: format!(
            "Samurai level {level} Resolve: {resolve_uses} use(s) per day ((level+1)/2), spent \
             to endure devastating wounds and afflictions; regained whenever the samurai \
             defeats the target of his current challenge, up to this daily maximum"
        ),
    });

    // Bonus Feat: granted from 6th level.
    if level >= 6 {
        let bonus_feat_count = samurai_bonus_feat_count(level);
        explanations.push(ComputationExplanation {
            id: "class_feature.uc.samurai.bonus_feat_count".to_owned(),
            value: bonus_feat_count,
            detail: format!(
                "Samurai level {level} Bonus Feat: {bonus_feat_count} bonus combat feat(s) \
                 (level/6, first at 6th level, one additional every 6 levels thereafter), on \
                 top of those gained from normal advancement. Grounds the COUNT only -- which \
                 feat(s) were picked is a chooser this engine does not model, the same \
                 count-vs-choice split Slayer Talents, Gunslinger Gun Training and Ninja Trick \
                 already establish"
            ),
        });
    }

    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.uc.samurai.other_features_deferred.unsupported".to_owned(),
        message: "Samurai now grounds Challenge's uses/day and damage bonus, Resolve's \
             uses/day, and Bonus Feat's count -- SD31-E4-F1-004's own new wiring. No \
             archetype-supersession branch exists because Samurai has zero real archetype \
             content in this package's 23-book IN-SCOPE set (re-verified this cycle: a \
             full-oracle-tree grep for \"Samurai Archetype\" actually returns 17 hits across \
             7 files, 5 of them real swappable archetype records -- but all 5 live in \
             out-of-scope player_companion books, so the in-scope conclusion holds; see \
             class_samurai.rs's doc comment for the corrected evidence and the forward-scope \
             note). What stays deferred, honestly: (1) Mount, Order, \
             Weapon Expertise, Mounted Archer and Banner are not yet transcribed; (2) the \
             later-level Resolve-spending abilities (Determined, Resolute, Unstoppable, \
             Greater Resolve, Honorable Stand, True Resolve, Last Stand) and Demanding \
             Challenge/Greater Banner are not yet transcribed; (3) this class is not \
             registered in v06_work_inventory.rs's modelled_class_books() (out of this cycle's \
             file territory -- reported to OPEN-ISSUES.md), so none of this wiring can reach \
             `done` or even `held` on the board yet regardless of how complete it is, the same \
             structural blocker SD31-E4-F1-002 and SD31-E4-F1-003 found for Gunslinger and \
             Ninja. This diagnostic is not claim-blocking for the features that ARE grounded \
             above; it carries the honest remainder"
            .to_owned(),
        claim_blocking: false,
    });
}

#[cfg(test)]
mod samurai_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, ComputationExplanation,
        PilotHeadlessReceipt,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const SAMURAI_CLASS_ID: &str = "class:samurai";

    fn character(level: u8) -> CharacterInput {
        let mut input = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE)
            .character_input
            .expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SAMURAI_CLASS_ID.to_owned(), level }];
        input
    }

    fn find<'a>(receipt: &'a PilotHeadlessReceipt, id: &str) -> Option<&'a ComputationExplanation> {
        receipt.computation.explanations.iter().find(|e| e.id == id)
    }

    /// The base chassis pillar: full BAB, good Fort, poor Reflex, poor
    /// Will, matching `class_samurai.rs`'s own formulas.
    #[test]
    fn samurai_base_chassis_grounds_bab_and_saves() {
        let receipt = build_pilot_headless_receipt(&character(10));
        assert_eq!(find(&receipt, "class_chassis.base_attack_bonus").unwrap().value, 10);
        assert_eq!(find(&receipt, "class_chassis.base_save.fortitude").unwrap().value, 7);
        assert_eq!(find(&receipt, "class_chassis.base_save.reflex").unwrap().value, 3);
        assert_eq!(find(&receipt, "class_chassis.base_save.will").unwrap().value, 3);
    }

    /// Challenge: min((level+2)/3, 7) uses/day, flat-level damage bonus,
    /// granted from 1st level.
    #[test]
    fn challenge_grounds_from_first_level_and_caps_uses_at_seven() {
        let receipt1 = build_pilot_headless_receipt(&character(1));
        assert_eq!(find(&receipt1, "class_feature.uc.samurai.challenge_uses").unwrap().value, 1);
        assert_eq!(
            find(&receipt1, "class_feature.uc.samurai.challenge_damage_bonus").unwrap().value,
            1
        );

        let receipt20 = build_pilot_headless_receipt(&character(20));
        assert_eq!(find(&receipt20, "class_feature.uc.samurai.challenge_uses").unwrap().value, 7);
        assert_eq!(
            find(&receipt20, "class_feature.uc.samurai.challenge_damage_bonus").unwrap().value,
            20
        );
    }

    /// Resolve: (level+1)/2 uses/day, granted from 1st level.
    #[test]
    fn resolve_grounds_from_first_level() {
        let receipt1 = build_pilot_headless_receipt(&character(1));
        assert_eq!(find(&receipt1, "class_feature.uc.samurai.resolve_uses").unwrap().value, 1);
        let receipt9 = build_pilot_headless_receipt(&character(9));
        assert_eq!(find(&receipt9, "class_feature.uc.samurai.resolve_uses").unwrap().value, 5);
    }

    /// Bonus Feat: level/6 count, granted from 6th level, not before.
    #[test]
    fn bonus_feat_count_grounds_from_sixth_level_only() {
        let receipt5 = build_pilot_headless_receipt(&character(5));
        assert!(find(&receipt5, "class_feature.uc.samurai.bonus_feat_count").is_none());
        let receipt6 = build_pilot_headless_receipt(&character(6));
        assert_eq!(find(&receipt6, "class_feature.uc.samurai.bonus_feat_count").unwrap().value, 1);
        let receipt18 = build_pilot_headless_receipt(&character(18));
        assert_eq!(
            find(&receipt18, "class_feature.uc.samurai.bonus_feat_count").unwrap().value,
            3
        );
    }

    /// Every grounded record's detail text names the real corpus formula
    /// rather than an unbacked number, the same self-check
    /// `ninja_tests`/`gunslinger_tests` run for their own records.
    #[test]
    fn samurai_features_carry_no_archetype_superseded_claim() {
        let receipt = build_pilot_headless_receipt(&character(10));
        for id in [
            "class_feature.uc.samurai.challenge_uses",
            "class_feature.uc.samurai.challenge_damage_bonus",
            "class_feature.uc.samurai.resolve_uses",
        ] {
            let record = find(&receipt, id).unwrap_or_else(|| panic!("expected {id} to ground"));
            assert!(
                !record.detail.to_lowercase().contains("superseded"),
                "Samurai has no archetype content in scope, so no record should ever claim a \
                 supersession: {record:?}"
            );
        }
    }
}
