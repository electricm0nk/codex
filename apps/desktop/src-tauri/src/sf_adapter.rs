//! `StarfinderAdapter` — the Starfinder 1e `RuleSystemAdapter` (SD-37 E4.6,
//! `docs/release/SD-37-starfinder-1e/epic-breakdown.md` Epic E4).
//!
//! # The id
//!
//! The rule-system id is `"starfinder-1e"` ([`STARFINDER_RULE_SYSTEM_ID`]):
//! `GameSystem::Starfinder1e.id()`, the desktop landing screen's `RuleSetId`
//! (`characterHubRuntime.ts`'s `resolveRuleSystemId` passes it through
//! unchanged) and the `game_system` a Starfinder save envelope carries. The
//! bare `"starfinder"` that `StubAdapter`'s own tests use names no system.
//!
//! # The character
//!
//! A saved Starfinder character is a `CharacterInput` read this way
//! ([`build_from_input`]):
//!
//! | `CharacterInput` | Starfinder build |
//! |---|---|
//! | `race_id`, `class_levels` | the race and class records (`core:race:human`, `core:class:soldier`) |
//! | `ability_scores` | the FINAL scores (race, theme, point buy and increases applied) |
//! | `selected_feats` | held records: the one whose package `pool` is `theme` is the theme; every other is a pick (feat, connection, racial pick) |
//! | `equipment_selections` | the one equipped record tagged `ARMOR` is the worn armour; every other selection not `Absent` is carried, one per selection; an equipped one is also held (its bonuses reach the totals); `applied_modifiers` are the upgrades and fusions on that selection (printed, E5.3) |
//! | `skill_allocations` | package skill id -> ranks |
//! | `selected_choices` | a choice whose set is one of the character's class ids is that class's key-ability choice (`STR`); every other is a rule choice (set = the choosing rule's id) |
//!
//! # The totals
//!
//! Every total comes from the E4.1–E4.5 readers over the converted package
//! (`data/starfinder-1e/sheet_rules`, read by [`package`]): `sf_chassis`,
//! `sf_defense`, `sf_skills`, `sf_spells`, `sf_loadout`. The adapter adds one
//! join those readers leave to the caller: the bulk condition's max-Dex cap
//! and −5 Strength/Dexterity check penalty ([`apply_bulk_condition`]; SRD
//! Bulk Limits, the worse of armour and bulk, not stacked).
//!
//! Each total is one `sf.*` [`ComputationExplanation`] row whose `detail`
//! lists every term added into it (`decisions.md §5`). A term a reader cannot
//! resolve is its named refusal, surfaced as one claim-blocking diagnostic.

use std::path::Path;

use codex::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use codex::rules_core::encumbrance::BulkCondition;
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::level_up::LevelUpPlan;
use codex::rules_core::pilot_compute::sf_attack::{self, SfAttacks};
use codex::rules_core::pilot_compute::sf_chassis::{
    self, ability_modifier, SfChassis, SfChassisBuild, SfChassisRefusal, SfTerm, SfTotal,
};
use codex::rules_core::pilot_compute::sf_defense::{self, SfBuild, SfDefense};
use codex::rules_core::pilot_compute::sf_loadout::{self, SfCarried, SfLoadout};
use codex::rules_core::pilot_compute::sf_skills::{self, SfSkills};
use codex::rules_core::pilot_compute::sf_spells::{self, SfSpellcasting};
use codex::rules_core::pilot_compute::{
    AbilityModifiers, BaseSaves, ComputationDiagnostic, ComputationExplanation,
    PilotBaseChassisComputation, SelectedSkillModifiers,
};
use codex::rules_core::sheet_rule::{split_rule_id, Ability, SheetRulePackage};
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::SavedCharacterEnvelope;

use crate::character_hub::{
    AbilityModifiersDto, AbilityScoresDto, BaseSavesDto, CharacterSummaryDto, CorpusDerivedDto,
    DiagnosticDto, EncumbranceDto, EquipmentEffectsDto, ExplanationDto,
    ListSavedCharactersResponse, LoadSavedCharacterResponse, PilotSnapshotDto,
    SelectedSkillModifiersDto,
};
use crate::characterHub::appendToCharacter::{
    AppendToCharacterResponse, AppendedCharacterDto, ItemToAppendDto,
};
use crate::characterHub::recomputeCharacter::{CharacterSnapshotDto, RecomputeCharacterResponse};
use crate::characterHub::reSaveCharacter::{re_save_character_at_root, ReSaveCharacterResponse};
use crate::rule_system_adapter::{ClassLevelDelta, RuleSystemAdapter};

/// The Starfinder 1e rule-system id: the wire id, the desktop `RuleSetId` and
/// the save envelope's `game_system`.
pub const STARFINDER_RULE_SYSTEM_ID: &str = GameSystem::Starfinder1e.id();

pub const REFUSED_PACKAGE_MISSING: &str = "sf_adapter.package_missing";
pub const REFUSED_TWO_THEMES: &str = "sf_adapter.two_themes";
pub const REFUSED_TWO_ARMORS: &str = "sf_adapter.two_armors";
pub const REFUSED_KEY_ABILITY_CHOICE: &str = "sf_adapter.key_ability_choice";
pub const REFUSED_NOT_STARFINDER: &str = "sf_adapter.not_a_starfinder_character";

const SRD_BULK: &str = "SRD Bulk Limits (https://www.aonsrd.com/Equipment.aspx)";
const SRD_CONDITIONS: &str = "SRD Conditions: Encumbered, Overburdened (https://www.aonsrd.com/Rules.aspx?ID=165)";

/// The id of the diagnostic every computed Starfinder chassis carries: the
/// Pathfinder fixed-posture fields of the shared result types are 0 here.
pub const PATHFINDER_FIELDS_DIAGNOSTIC: &str = "sf_adapter.pathfinder_fields_zero";
const PATHFINDER_FIELDS_MESSAGE: &str = "baseline_melee_attack_bonus, baseline_armor_class and \
     selected_skill_modifiers are Pathfinder fixed-posture totals with no Starfinder meaning and are 0; \
     the Starfinder sheet totals are the sf.* explanation rows (sf.eac, sf.kac, sf.skill.<skill>, ...)";

/// The Starfinder 1e adapter. Stateless: the package is the process-wide
/// Starfinder package [`package`] reads.
pub struct StarfinderAdapter;

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
}

/// The Starfinder package, read from the desktop's resolved root
/// ([`crate::authoring_workbench::codex_repo_root`]: `CODEX_REPO_ROOT`, then the packaged
/// app's bundled resources, then the dev checkout), so a packaged app reads the
/// `data/starfinder-1e/sheet_rules/` it ships (`tauri.conf.json` `bundle.resources`).
pub(crate) fn package() -> Result<&'static SheetRulePackage, SfChassisRefusal> {
    crate::character_hub::sheet_rule_package_for(GameSystem::Starfinder1e).as_ref().map_err(|reason| {
        refuse(
            REFUSED_PACKAGE_MISSING,
            format!(
                "the Starfinder package ({}) did not load: {reason}",
                GameSystem::Starfinder1e.sheet_rules_relative()
            ),
        )
    })
}

/// `STR`, `Dex`, `Strength` -> the ability.
fn parse_ability(option: &str) -> Option<Ability> {
    match option.trim().to_ascii_uppercase().as_str() {
        "STR" | "STRENGTH" => Some(Ability::Str),
        "DEX" | "DEXTERITY" => Some(Ability::Dex),
        "CON" | "CONSTITUTION" => Some(Ability::Con),
        "INT" | "INTELLIGENCE" => Some(Ability::Int),
        "WIS" | "WISDOM" => Some(Ability::Wis),
        "CHA" | "CHARISMA" => Some(Ability::Cha),
        _ => None,
    }
}

/// The Starfinder build and carried loadout a saved `CharacterInput` holds
/// (the table in this module's doc comment).
pub fn build_from_input(
    package: &SheetRulePackage,
    input: &CharacterInput,
) -> Result<(SfBuild, SfLoadout), SfChassisRefusal> {
    let chosen = &input.chosen;
    let classes: Vec<(String, u8)> =
        chosen.class_levels.iter().map(|c| (c.class_id.clone(), c.level)).collect();
    let scores = &chosen.ability_scores;
    let ability_scores = [
        scores.strength,
        scores.dexterity,
        scores.constitution,
        scores.intelligence,
        scores.wisdom,
        scores.charisma,
    ]
    .map(i64::from);

    let mut key_ability_choice = None;
    let mut choices: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for choice in &chosen.selected_choices {
        if classes.iter().any(|(class, _)| *class == choice.choice_set_id) {
            let ability = parse_ability(&choice.selection_id).ok_or_else(|| {
                refuse(
                    REFUSED_KEY_ABILITY_CHOICE,
                    format!("{}: key ability choice {:?} names no ability", choice.choice_set_id, choice.selection_id),
                )
            })?;
            if key_ability_choice.replace(ability).is_some_and(|earlier| earlier != ability) {
                return Err(refuse(
                    REFUSED_KEY_ABILITY_CHOICE,
                    format!("{}: two different key ability choices", choice.choice_set_id),
                ));
            }
        } else {
            choices.entry(choice.choice_set_id.clone()).or_default().push(choice.selection_id.clone());
        }
    }

    let mut theme = None;
    let mut picks = Vec::new();
    for id in &chosen.selected_feats {
        // A record the package does not hold stays a pick; `sf_defense::held` refuses it by name.
        if package.rule(id).is_some_and(|r| r.pool == "theme") {
            if let Some(first) = theme.replace(id.clone()) {
                return Err(refuse(REFUSED_TWO_THEMES, format!("two themes selected: {first}, {id}")));
            }
        } else {
            picks.push(id.clone());
        }
    }

    let is_armor = |s: &EquipmentSelection| {
        package.rule(&s.item_id).is_some_and(|r| r.tags.iter().any(|t| t == "ARMOR"))
    };
    let mut armor = None;
    let mut carried: Vec<(String, u32)> = Vec::new();
    let mut applied: Vec<String> = Vec::new();
    for selection in &chosen.equipment_selections {
        if selection.active_state == ActiveState::Absent {
            continue;
        }
        // Upgrades, fusions and special materials on a carried item were bought with it:
        // their price and bulk reach the carried totals (`sf_loadout`, SD-37 decisions.md §20).
        applied.extend(selection.applied_modifiers.iter().cloned());
        if selection.active_state == ActiveState::EquippedActive && is_armor(selection) {
            if let Some(first) = armor.replace(selection.item_id.clone()) {
                return Err(refuse(
                    REFUSED_TWO_ARMORS,
                    format!("two armours worn: {first}, {}", selection.item_id),
                ));
            }
            continue;
        }
        // An equipped (worn, installed) item is held: its bonus to a sheet total reaches the
        // total E4's readers add (an aeon stone's insight bonus to Perception). Carried but
        // not equipped, it only prints and costs credits and bulk.
        if selection.active_state == ActiveState::EquippedActive && !picks.contains(&selection.item_id) {
            picks.push(selection.item_id.clone());
        }
        match carried.iter_mut().find(|(id, _)| *id == selection.item_id) {
            Some((_, quantity)) => *quantity += 1,
            None => carried.push((selection.item_id.clone(), 1)),
        }
    }

    let build = SfBuild {
        chassis: SfChassisBuild {
            classes,
            race: chosen.race_id.clone(),
            ability_scores,
            key_ability_choice,
        },
        theme,
        armor,
        picks,
        skill_ranks: chosen
            .skill_allocations
            .iter()
            .map(|s| (s.skill_id.clone(), i64::from(s.ranks)))
            .collect(),
        choices,
    };
    Ok((build, SfLoadout { starting_credits: None, carried, applied }))
}

/// Every Starfinder sheet total of one character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfSheet {
    pub ability_scores: [i64; 6],
    /// Each class rule id and its level (the build's classes).
    pub classes: Vec<(String, u8)>,
    pub chassis: SfChassis,
    pub defense: SfDefense,
    pub skills: SfSkills,
    pub spells: Vec<SfSpellcasting>,
    pub carried: SfCarried,
    /// The melee and ranged attack bonus and each carried weapon's attack and damage bonus
    /// (`sf_attack`, SD-37 E7.1).
    pub attacks: SfAttacks,
    /// The printed lines: race, theme, class features, feats and every other held record
    /// (`sf_sheet_print`, E5.1), and the spells known (E5.2).
    pub lines: Vec<codex::rules_core::sheet_rule::SheetLine>,
}

/// The sheet totals of `input`, read from `package` (the Starfinder package).
pub fn compute_sheet(package: &SheetRulePackage, input: &CharacterInput) -> Result<SfSheet, SfChassisRefusal> {
    let (build, loadout) = build_from_input(package, input)?;
    let chassis = sf_chassis::compute(package, &build.chassis)?;
    let held = sf_defense::held(package, &build)?;
    let mut defense = sf_defense::compute_with(package, &build, &held)?;
    let mut skills = sf_skills::compute_with(package, &build, &held)?;
    let spells = sf_spells::compute_with(package, &build, &held)?;
    // The print path first: it refuses an applied modifier it cannot place (unknown record,
    // wrong item, slots exceeded) by its own name before the loadout totals its price.
    let lines = crate::sf_sheet_print::sheet_lines(package, &build, &held, &input.chosen.spells_selected, &input.chosen.equipment_selections)?;
    let carried = sf_loadout::compute(package, &build, &loadout)?;
    let attacks = sf_attack::compute_with(package, &held, &chassis.base_attack_bonus, &loadout.carried);
    apply_bulk_condition(&mut defense, &mut skills, carried.condition);
    Ok(SfSheet {
        ability_scores: build.chassis.ability_scores,
        classes: build.chassis.classes.clone(),
        chassis,
        defense,
        skills,
        spells,
        carried,
        attacks,
        lines,
    })
}

fn resum(total: &mut SfTotal) {
    total.total = total.terms.iter().map(|t| t.value).sum();
}

/// Folds the bulk condition into the totals it changes (SRD Bulk Limits):
/// the Dexterity term of EAC and KAC is capped at the condition's max Dex
/// (the lower of armour and bulk), and every Strength- or Dexterity-based
/// skill takes the condition's −5 -- or the armour check penalty, whichever
/// is worse; the two do not stack. Initiative (a Dexterity-based check) takes
/// the −5 too. Unencumbered changes nothing.
pub fn apply_bulk_condition(defense: &mut SfDefense, skills: &mut SfSkills, condition: BulkCondition) {
    if let Some(cap) = condition.max_dex_cap() {
        for ac in [&mut defense.eac, &mut defense.kac] {
            if let Some(dex) = ac.terms.iter_mut().find(|t| t.label.starts_with("Dexterity modifier")) {
                if dex.value > cap {
                    dex.label = format!("{} (max {cap} while {condition:?})", dex.label);
                    dex.value = cap;
                    dex.source = SRD_BULK.to_owned();
                }
            }
            resum(ac);
        }
    }
    let penalty = condition.check_penalty();
    if penalty == 0 {
        return;
    }
    // Initiative is a Dexterity-based check (d20 + Dex modifier, SRD Rules ID=96); armour
    // has no check penalty on it, so the condition's -5 applies alone.
    defense.initiative.terms.push(SfTerm {
        label: format!("{condition:?} penalty"),
        value: penalty,
        source: SRD_CONDITIONS.to_owned(),
    });
    resum(&mut defense.initiative);
    for skill in skills.skills.iter_mut().filter(|s| matches!(s.ability, Ability::Str | Ability::Dex)) {
        let Some(total) = skill.total.as_mut() else { continue };
        match total.terms.iter_mut().find(|t| t.label == "armor check penalty") {
            Some(acp) if acp.value <= penalty => {}
            Some(acp) => {
                acp.label = format!("{condition:?} penalty (worse than the armor check penalty {})", acp.value);
                acp.value = penalty;
                acp.source = SRD_BULK.to_owned();
            }
            None => total.terms.push(SfTerm {
                label: format!("{condition:?} penalty"),
                value: penalty,
                source: SRD_BULK.to_owned(),
            }),
        }
        resum(total);
    }
}

fn to_i16(value: i64) -> i16 {
    value.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16
}

fn explain(rows: &mut Vec<ComputationExplanation>, id: String, total: &SfTotal) {
    let detail = total
        .terms
        .iter()
        .map(|t| format!("{} {:+} ({})", t.label, t.value, t.source))
        .collect::<Vec<_>>()
        .join("; ");
    rows.push(ComputationExplanation { id, value: to_i16(total.total), detail });
}

const ABILITY_NAMES: [&str; 6] = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];

impl SfSheet {
    fn modifiers(&self) -> [i64; 6] {
        self.ability_scores.map(ability_modifier)
    }

    /// One `sf.*` explanation row per sheet total, plus one per ability score and per
    /// class level, so every number the Starfinder sheet shows is a row (E6.3).
    pub fn explanations(&self) -> Vec<ComputationExplanation> {
        let mut rows = Vec::new();
        for (i, name) in ABILITY_NAMES.iter().enumerate() {
            rows.push(ComputationExplanation {
                id: format!("sf.ability_score.{name}"),
                value: to_i16(self.ability_scores[i]),
                detail: "the saved score: race, theme, point buy and ability increases applied".to_owned(),
            });
        }
        for (class, level) in &self.classes {
            rows.push(ComputationExplanation {
                id: format!("sf.class_level.{}", split_rule_id(class).2),
                value: i16::from(*level),
                detail: format!("{class} level {level}"),
            });
        }
        for (i, name) in ABILITY_NAMES.iter().enumerate() {
            rows.push(ComputationExplanation {
                id: format!("sf.ability_modifier.{name}"),
                value: to_i16(self.modifiers()[i]),
                detail: format!("score {}: floor(score / 2) - 5 (Starfinder Table 2-1)", self.ability_scores[i]),
            });
        }
        let c = &self.chassis;
        for (id, total) in [
            ("sf.base_attack_bonus", &c.base_attack_bonus),
            ("sf.fortitude", &c.fortitude),
            ("sf.reflex", &c.reflex),
            ("sf.will", &c.will),
            ("sf.hit_points", &c.hit_points),
            ("sf.stamina", &c.stamina),
            ("sf.resolve", &c.resolve),
            ("sf.eac", &self.defense.eac),
            ("sf.kac", &self.defense.kac),
            ("sf.initiative", &self.defense.initiative),
            ("sf.attack.melee", &self.attacks.melee),
            ("sf.attack.ranged", &self.attacks.ranged),
        ] {
            explain(&mut rows, id.to_owned(), total);
        }
        // `sf.weapon.<equipment slug>.attack` / `.damage`: each carried weapon's attack bonus and
        // the number added to its damage dice (the dice print on the weapon's own line).
        for weapon in &self.attacks.weapons {
            let item = split_rule_id(&weapon.item).2;
            explain(&mut rows, format!("sf.weapon.{item}.attack"), &weapon.attack);
            explain(&mut rows, format!("sf.weapon.{item}.damage"), &weapon.damage);
        }
        for skill in &self.skills.skills {
            if let Some(total) = &skill.total {
                explain(&mut rows, format!("sf.skill.{}", skill.skill), total);
            }
        }
        for casting in &self.spells {
            let class = split_rule_id(&casting.class).2;
            for level in casting.levels.iter().filter(|l| l.castable()) {
                for (field, total) in [("per_day", &level.per_day), ("known", &level.known), ("save_dc", &level.save_dc)] {
                    if let Some(total) = total {
                        explain(&mut rows, format!("sf.spells.{class}.{}.{field}", level.level), total);
                    }
                }
            }
        }
        let carried = &self.carried;
        for (id, total) in [
            ("sf.credits.starting", &carried.starting_credits),
            ("sf.credits.spent", &carried.credits_spent),
            ("sf.credits.remaining", &carried.credits_remaining),
            ("sf.bulk", &carried.bulk),
        ] {
            explain(&mut rows, id.to_owned(), total);
        }
        let strength = self.ability_scores[0];
        rows.push(ComputationExplanation {
            id: "sf.bulk_limit.unencumbered_max".to_owned(),
            value: to_i16(carried.limits.unencumbered_max),
            detail: format!("half the Strength score {strength}, rounded down ({SRD_BULK})"),
        });
        rows.push(ComputationExplanation {
            id: "sf.bulk_limit.overburdened_above".to_owned(),
            value: to_i16(carried.limits.overburdened_above),
            detail: format!("the Strength score {strength} ({SRD_BULK})"),
        });
        rows
    }

    /// The shared chassis result: BAB, base and total saves, ability
    /// modifiers, the `sf.*` rows and the Pathfinder-fields diagnostic.
    pub fn to_chassis(&self) -> PilotBaseChassisComputation {
        let m = self.modifiers().map(to_i16);
        let c = &self.chassis;
        let total_saves = BaseSaves {
            fortitude: to_i16(c.fortitude.total),
            reflex: to_i16(c.reflex.total),
            will: to_i16(c.will.total),
        };
        PilotBaseChassisComputation {
            ability_modifiers: AbilityModifiers {
                strength: m[0],
                dexterity: m[1],
                constitution: m[2],
                intelligence: m[3],
                wisdom: m[4],
                charisma: m[5],
            },
            base_attack_bonus: to_i16(c.base_attack_bonus.total),
            // Each total save is the class's base save plus one ability modifier (Con/Dex/Wis).
            base_saves: BaseSaves {
                fortitude: total_saves.fortitude - m[2],
                reflex: total_saves.reflex - m[1],
                will: total_saves.will - m[4],
            },
            baseline_melee_attack_bonus: 0,
            baseline_armor_class: 0,
            total_saves,
            selected_skill_modifiers: SelectedSkillModifiers::default(),
            explanations: self.explanations(),
            diagnostics: vec![ComputationDiagnostic {
                id: PATHFINDER_FIELDS_DIAGNOSTIC.to_owned(),
                message: PATHFINDER_FIELDS_MESSAGE.to_owned(),
                claim_blocking: false,
            }],
            sheet_lines: self.lines.clone(),
        }
    }
}

/// The chassis of a character the readers refuse: every number 0 and the
/// refusal as one claim-blocking diagnostic (`id` = the refusal's stable id).
fn refused_chassis(refusal: SfChassisRefusal) -> PilotBaseChassisComputation {
    PilotBaseChassisComputation {
        ability_modifiers: AbilityModifiers::default(),
        base_attack_bonus: 0,
        base_saves: BaseSaves::default(),
        baseline_melee_attack_bonus: 0,
        baseline_armor_class: 0,
        total_saves: BaseSaves::default(),
        selected_skill_modifiers: SelectedSkillModifiers::default(),
        explanations: Vec::new(),
        diagnostics: vec![ComputationDiagnostic {
            id: refusal.id.to_owned(),
            message: refusal.message,
            claim_blocking: true,
        }],
        sheet_lines: Vec::new(),
    }
}

fn compute_chassis(input: &CharacterInput) -> PilotBaseChassisComputation {
    match package().and_then(|p| compute_sheet(p, input)) {
        Ok(sheet) => sheet.to_chassis(),
        Err(refusal) => refused_chassis(refusal),
    }
}

/// The blocking messages of a chassis, or `None` when it computed.
fn blocking(chassis: &PilotBaseChassisComputation) -> Option<String> {
    let messages: Vec<String> = chassis
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .map(|d| format!("{}: {}", d.id, d.message))
        .collect();
    (!messages.is_empty()).then(|| messages.join("; "))
}

/// The envelope at `root`, refused unless it is a Starfinder character.
pub(crate) fn load_starfinder(root: &Path) -> Result<SavedCharacterEnvelope, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;
    if envelope.game_system != STARFINDER_RULE_SYSTEM_ID {
        return Err(format!(
            "{REFUSED_NOT_STARFINDER}: {} is a {:?} character, not {STARFINDER_RULE_SYSTEM_ID}",
            envelope.character_id, envelope.game_system
        ));
    }
    Ok(envelope)
}

pub(crate) fn summary_dto(envelope: &SavedCharacterEnvelope) -> CharacterSummaryDto {
    CharacterSummaryDto {
        character_id: envelope.character_id.clone(),
        display_label: envelope.display_label.clone(),
        game_system: envelope.game_system.clone(),
        schema_version: envelope.schema_version,
        saved_at: envelope.saved_at.clone(),
        race_id: envelope.character_input.chosen.race_id.clone(),
        class_summary: envelope
            .character_input
            .chosen
            .class_levels
            .iter()
            .map(|c| format!("{}:{}", c.class_id, c.level))
            .collect::<Vec<_>>()
            .join(", "),
    }
}

fn snapshot_dto(chassis: &PilotBaseChassisComputation) -> PilotSnapshotDto {
    let m = chassis.ability_modifiers;
    let saves = |s: BaseSaves| BaseSavesDto { fortitude: s.fortitude, reflex: s.reflex, will: s.will };
    PilotSnapshotDto {
        ability_modifiers: AbilityModifiersDto {
            strength: m.strength,
            dexterity: m.dexterity,
            constitution: m.constitution,
            intelligence: m.intelligence,
            wisdom: m.wisdom,
            charisma: m.charisma,
        },
        base_attack_bonus: chassis.base_attack_bonus,
        base_saves: saves(chassis.base_saves),
        baseline_melee_attack_bonus: chassis.baseline_melee_attack_bonus,
        baseline_armor_class: chassis.baseline_armor_class,
        total_saves: saves(chassis.total_saves),
        selected_skill_modifiers: SelectedSkillModifiersDto { climb: 0, intimidate: 0, swim: 0 },
        damage_reduction: None,
        companion: None,
        spellbook: None,
    }
}

/// The corpus-derived block for a Starfinder sheet. Its Pathfinder corpus
/// fields (school coverage, resolved equipment, pound/gold-piece weights)
/// are empty or 0; the Starfinder bulk condition fills `level` and the load
/// caps, and the worn armour's penalty fills `armor_check_penalty_total`.
fn corpus_derived_dto(sheet: &SfSheet) -> CorpusDerivedDto {
    let condition = sheet.carried.condition;
    CorpusDerivedDto {
        school_coverage: Vec::new(),
        equipped_items: Vec::new(),
        equipment_effects: EquipmentEffectsDto {
            per_item: Vec::new(),
            armor_class_delta: 0,
            armor_check_penalty_total: to_i16(sheet.defense.armor_check_penalty),
            max_dex_cap: None,
            spell_failure_chance: None,
            attack_bonus_delta: None,
            spell_resistance_total: None,
        },
        encumbrance: EncumbranceDto {
            total_carried_weight_lbs: 0.0,
            total_carried_cost_gp: 0.0,
            light_max_lbs: 0.0,
            medium_max_lbs: 0.0,
            heavy_max_lbs: 0.0,
            level: format!("{condition:?}").to_ascii_lowercase(),
            load_max_dex_cap: condition.max_dex_cap().map(to_i16),
            load_armor_check_penalty: to_i16(condition.check_penalty()),
            per_item: Vec::new(),
            unresolved_item_ids: Vec::new(),
        },
        unresolved_spell_ids: Vec::new(),
        unresolved_equipment_item_ids: Vec::new(),
    }
}

/// The next `<id>.rev.<n>` revision (the same scheme `Pf1Adapter`'s mutations use).
pub(crate) fn next_revision_id(character_id: &str, current: &str) -> String {
    let prefix = format!("{character_id}.rev.");
    let next = current
        .strip_prefix(prefix.as_str())
        .and_then(|n| n.parse::<u64>().ok())
        .map_or(1, |n| n + 1);
    format!("{prefix}{next}")
}

impl RuleSystemAdapter for StarfinderAdapter {
    fn rule_system_id(&self) -> &'static str {
        STARFINDER_RULE_SYSTEM_ID
    }

    fn chassis_resolve(&self, input: &CharacterInput) -> PilotBaseChassisComputation {
        compute_chassis(input)
    }

    /// `LevelUpPlan`'s grants cite a Pathfinder `rules_tables` cell
    /// (`Grant::source_table: TableCellRef`, whose `rule_set` is a Pathfinder
    /// book), which cannot name a Starfinder package row, so the plan is the
    /// empty plan `level_up::compute_level_up_grants` returns for a class it
    /// does not dispatch. A Starfinder level-up is a re-save at the new level:
    /// the new totals are `chassis_resolve`'s. The desktop's Starfinder level-up is
    /// `crate::sf_level_up` (SD-37 E6.5), which saves the leveled `CharacterInput`.
    fn level_up(&self, _character: &CharacterInput, _deltas: &[ClassLevelDelta]) -> LevelUpPlan {
        LevelUpPlan::default()
    }

    fn save_character(
        &self,
        root: &Path,
        expected_revision_id: &str,
        saved_at: &str,
    ) -> Result<ReSaveCharacterResponse, String> {
        load_starfinder(root)?;
        re_save_character_at_root(root, expected_revision_id, saved_at)
    }

    /// Appends each item as one carried selection, after checking every item
    /// is a Starfinder equipment record and that the character still computes
    /// with them (a loadout over the character's credits is refused); either
    /// every item is appended or none.
    fn append_to_character(
        &self,
        root: &Path,
        items_to_append: &[ItemToAppendDto],
        saved_at: &str,
    ) -> Result<AppendToCharacterResponse, String> {
        let mut envelope = load_starfinder(root)?;
        let package = package().map_err(|r| format!("{}: {}", r.id, r.message))?;
        let failed = |error: String| AppendToCharacterResponse { success: false, character: None, error: Some(error) };
        if let Some(unknown) = items_to_append.iter().find(|item| {
            item.item_id.contains('#')
                || split_rule_id(&item.item_id).1 != "equipment"
                || package.rule(&item.item_id).is_none()
        }) {
            return Ok(failed(format!("equipment_not_found: {}", unknown.item_id)));
        }
        for item in items_to_append {
            let active_state: ActiveState = item.active_state.into();
            envelope.character_input.chosen.equipment_selections.push(EquipmentSelection {
                item_id: item.item_id.clone(),
                equipped_or_active: active_state == ActiveState::EquippedActive,
                active_state,
                applied_modifiers: Vec::new(),
            });
        }
        let sheet = match compute_sheet(package, &envelope.character_input) {
            Ok(sheet) => sheet,
            Err(refusal) => {
                return Ok(failed(format!("recompute_blocked: {}: {}", refusal.id, refusal.message)));
            }
        };
        let next = next_revision_id(&envelope.character_id, &envelope.revision_id);
        envelope.revision_id = next.clone();
        envelope.latest_authoritative_revision_ref = next;
        envelope.saved_at = saved_at.to_owned();
        SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;
        Ok(AppendToCharacterResponse {
            success: true,
            character: Some(AppendedCharacterDto {
                summary: summary_dto(&envelope),
                snapshot: snapshot_dto(&sheet.to_chassis()),
                corpus_derived: corpus_derived_dto(&sheet),
            }),
            error: None,
        })
    }

    fn recompute(&self, root: &Path, character_id: &str) -> RecomputeCharacterResponse {
        let envelope = match load_starfinder(root) {
            Ok(envelope) => envelope,
            Err(error) => {
                let error = if error.starts_with(REFUSED_NOT_STARFINDER) { error } else { "character_not_found".to_owned() };
                return RecomputeCharacterResponse { success: false, character: None, error: Some(error) };
            }
        };
        let chassis = compute_chassis(&envelope.character_input);
        if let Some(messages) = blocking(&chassis) {
            return RecomputeCharacterResponse {
                success: false,
                character: None,
                error: Some(format!("character_not_computable: {messages}")),
            };
        }
        let saves = |s: BaseSaves| crate::characterHub::recomputeCharacter::BaseSavesDto {
            fortitude: s.fortitude,
            reflex: s.reflex,
            will: s.will,
        };
        RecomputeCharacterResponse {
            success: true,
            character: Some(CharacterSnapshotDto {
                character_id: character_id.to_owned(),
                base_attack_bonus: chassis.base_attack_bonus,
                base_saves: saves(chassis.base_saves),
                baseline_melee_attack_bonus: chassis.baseline_melee_attack_bonus,
                baseline_armor_class: chassis.baseline_armor_class,
                total_saves: saves(chassis.total_saves),
                damage_reduction: None,
            }),
            error: None,
        }
    }

    /// Every saved character under `characters_root` whose `game_system` is
    /// `starfinder-1e`.
    fn list_saved_characters(&self, characters_root: &Path) -> Result<ListSavedCharactersResponse, String> {
        let listing = SavedCharacterStore::list_all(characters_root).map_err(|err| err.message)?;
        Ok(ListSavedCharactersResponse {
            characters: listing
                .characters
                .iter()
                .filter(|s| s.game_system == STARFINDER_RULE_SYSTEM_ID)
                .map(crate::character_hub::map_summary_dto)
                .collect(),
            unreadable_count: listing.unreadable_entries.len(),
        })
    }

    /// The saved character at `root`, recomputed fresh: the chassis snapshot,
    /// the `sf.*` explanation rows, the diagnostics and the saved selections.
    /// The Pathfinder-only blocks (weapon damage, racial-trait picker, feat
    /// targets) are empty; the Starfinder sheet lines are `sf_sheet_print`'s.
    fn load_saved_character(&self, root: &Path) -> Result<LoadSavedCharacterResponse, String> {
        let envelope = load_starfinder(root)?;
        let input = &envelope.character_input;
        let sheet = package().and_then(|p| compute_sheet(p, input));
        let chassis = match &sheet {
            Ok(sheet) => sheet.to_chassis(),
            Err(refusal) => refused_chassis(refusal.clone()),
        };
        let scores = &input.chosen.ability_scores;
        Ok(LoadSavedCharacterResponse {
            summary: summary_dto(&envelope),
            snapshot: sheet.as_ref().ok().map(|_| snapshot_dto(&chassis)),
            diagnostics: chassis
                .diagnostics
                .iter()
                .map(|d| DiagnosticDto { id: d.id.clone(), message: d.message.clone(), claim_blocking: d.claim_blocking })
                .collect(),
            corpus_derived: match &sheet {
                Ok(sheet) => corpus_derived_dto(sheet),
                Err(_) => CorpusDerivedDto {
                    school_coverage: Vec::new(),
                    equipped_items: Vec::new(),
                    equipment_effects: EquipmentEffectsDto {
                        per_item: Vec::new(),
                        armor_class_delta: 0,
                        armor_check_penalty_total: 0,
                        max_dex_cap: None,
                        spell_failure_chance: None,
                        attack_bonus_delta: None,
                        spell_resistance_total: None,
                    },
                    encumbrance: EncumbranceDto {
                        total_carried_weight_lbs: 0.0,
                        total_carried_cost_gp: 0.0,
                        light_max_lbs: 0.0,
                        medium_max_lbs: 0.0,
                        heavy_max_lbs: 0.0,
                        level: String::new(),
                        load_max_dex_cap: None,
                        load_armor_check_penalty: 0,
                        per_item: Vec::new(),
                        unresolved_item_ids: Vec::new(),
                    },
                    unresolved_spell_ids: Vec::new(),
                    unresolved_equipment_item_ids: Vec::new(),
                },
            },
            selected_feats: input.chosen.selected_feats.clone(),
            spells_selected: crate::character_hub::map_spells_selected_dto(&input.chosen.spells_selected),
            chosen_feat_targets: Vec::new(),
            explanations: chassis
                .explanations
                .iter()
                .map(|e| ExplanationDto { id: e.id.clone(), value: e.value, detail: e.detail.clone() })
                .collect(),
            weapon_damage: Vec::new(),
            selected_alternate_trait_keys: Vec::new(),
            selected_traits: input.chosen.selected_traits.clone(),
            // The race-trait picker reads the Pathfinder race corpus; its
            // "could not be served" shape is a non-empty `errors`.
            resolved_racial_traits: crate::race_trait_picker::RaceSelectionResponse {
                race_id: input.chosen.race_id.clone(),
                race_key: input.chosen.race_id.clone(),
                race_name: String::new(),
                book: String::new(),
                applied_traits: Vec::new(),
                suppressions: Vec::new(),
                fired_flags: Vec::new(),
                inert_flags: Vec::new(),
                unmatched_selections: Vec::new(),
                blocked_alternates: Vec::new(),
                conflicting_selections: Vec::new(),
                rendered_trait_descriptions: Vec::new(),
                display_value_feats: Vec::new(),
                errors: vec![format!(
                    "{}: the race-trait picker reads the Pathfinder race corpus; a Starfinder race's traits are its sheet lines",
                    input.chosen.race_id
                )],
            },
            ability_scores: AbilityScoresDto {
                strength: scores.strength,
                dexterity: scores.dexterity,
                constitution: scores.constitution,
                intelligence: scores.intelligence,
                wisdom: scores.wisdom,
                charisma: scores.charisma,
            },
            skill_allocations: crate::character_hub::map_skill_allocations_dto(input),
            equipment_selections: crate::character_hub::map_equipment_selections_dto(input),
            sheet_lines: crate::character_hub::map_sheet_lines_dto(&chassis.sheet_lines),
            sheet_rules_unavailable_reason: None,
            feat_skill_bonuses: Default::default(),
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use codex::rules_core::character_input::{
        AbilityScores, AcquisitionMode, CharacterClassLevel, ChosenCharacterState, SelectedChoice,
        SkillAllocation, SpellSelection,
    };
    use codex::saved_character::{SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION};

    use crate::character_hub::ActiveStateDto;

    /// The SRD hand-value files the seed fixtures read (E0.4, E4.2, E4.4, E4.5).
    const HAND_VALUE_FILES: [&str; 4] = [
        "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md",
        "docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.2-initiative-hand-values.md",
        "docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.4-spell-dc-hand-values.md",
        "docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.5-loadout-hand-values.md",
    ];

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
    }

    /// `None` = "untrained (trained only)": no total printed.
    pub(crate) fn hand_values() -> Vec<(String, String, Option<i64>)> {
        let mut out = Vec::new();
        for rel in HAND_VALUE_FILES {
            let path = repo_root().join(rel);
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            for line in text.lines().filter(|l| l.starts_with('|')) {
                let cells: Vec<&str> = line.split('|').map(str::trim).collect();
                if cells.len() < 6 || !cells[1].starts_with("SF-") {
                    continue;
                }
                assert!(cells[4].contains("https://"), "hand value without a source URL: {line}");
                let value = if cells[3].starts_with("untrained") {
                    None
                } else {
                    let v = cells[3].replace('\u{2212}', "-").replace(',', "");
                    Some(v.trim_start_matches('+').parse::<i64>().unwrap_or_else(|_| panic!("unparsed hand value: {line}")))
                };
                out.push((cells[1].to_owned(), cells[2].to_owned(), value));
            }
        }
        out
    }

    fn item(id: &str, active_state: ActiveState) -> EquipmentSelection {
        EquipmentSelection {
            item_id: format!("core:equipment:{id}"),
            equipped_or_active: active_state == ActiveState::EquippedActive,
            active_state,
            applied_modifiers: Vec::new(),
        }
    }

    fn choice(set: &str, selection: &str) -> SelectedChoice {
        SelectedChoice { choice_set_id: set.to_owned(), selection_id: selection.to_owned() }
    }

    /// One seed as a saved `CharacterInput` (`seed-builds.md` §1–§4): final scores, theme
    /// and picks as `selected_feats`, worn armour equipped, carried gear inactive.
    fn seed_input(
        (class, level, race): (&str, u8, &str),
        [strength, dexterity, constitution, intelligence, wisdom, charisma]: [i16; 6],
        feats: &[&str],
        ranks: &[(&str, u8)],
        armor: &str,
        carried: &[&str],
        choices: Vec<SelectedChoice>,
    ) -> CharacterInput {
        let mut equipment = vec![item(armor, ActiveState::EquippedActive)];
        equipment.extend(carried.iter().map(|id| item(id, ActiveState::SelectedInactive)));
        CharacterInput {
            case_id: None,
            source_package_id: STARFINDER_RULE_SYSTEM_ID.to_owned(),
            chosen: ChosenCharacterState {
                race_id: format!("core:race:{race}"),
                class_levels: vec![CharacterClassLevel { class_id: format!("core:class:{class}"), level }],
                ability_scores: AbilityScores { strength, dexterity, constitution, intelligence, wisdom, charisma },
                selected_feats: feats.iter().map(|f| f.to_string()).collect(),
                skill_allocations: ranks
                    .iter()
                    .map(|(s, r)| SkillAllocation { skill_id: s.to_string(), ranks: *r })
                    .collect(),
                equipment_selections: equipment,
                selected_choices: choices,
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    /// The seed's spells known (`seed-builds.md` §2, §3 "Spells known") as the save records
    /// them: every spell from the class table `Known`, a mystic's connection spells `Granted`.
    fn with_spells(mut input: CharacterInput, class: &str, known: &[&str], granted: &[&str]) -> CharacterInput {
        let selection = |spell: &str, acquisition_mode| SpellSelection {
            spell_id: format!("core:spell:{spell}"),
            source_class_id: format!("core:class:{class}"),
            acquisition_mode,
        };
        input.chosen.spells_selected = known
            .iter()
            .map(|s| selection(s, AcquisitionMode::Known))
            .chain(granted.iter().map(|s| selection(s, AcquisitionMode::Granted)))
            .collect();
        input
    }

    /// The four SD-37 Starfinder seeds, keyed by seed id, with each seed's class slug.
    pub(crate) fn seeds() -> Vec<(&'static str, &'static str, CharacterInput)> {
        vec![
            (
                "SF-Soldier-3",
                "soldier",
                crate::rule_system_adapter::tests::sf_soldier_3_input(),
            ),
            (
                "SF-Mystic-5",
                "mystic",
                with_spells(seed_input(
                    ("mystic", 5, "lashunta"),
                    [10, 14, 8, 14, 19, 15],
                    &[
                        "core:ability:priest",
                        "core:ability:empath",
                        "core:ability:2_racial_bonus_to_skill",
                        "core:ability:lashunta_subrace_damaya",
                        "core:feat:spell_penetration",
                        "core:feat:spell_focus",
                        "core:feat:quick_draw",
                    ],
                    &[
                        ("bluff", 5),
                        ("culture", 5),
                        ("diplomacy", 5),
                        ("life_science", 5),
                        ("medicine", 5),
                        ("mysticism", 5),
                        ("perception", 5),
                        ("sense_motive", 5),
                    ],
                    "lashunta_tempweave_basic",
                    &["laser_pistol_azimuth", "baton_tactical", "battery", "medkit_basic", "serum_of_healing_mk_1", "serum_of_healing_mk_1"],
                    vec![
                        choice("core:ability:2_racial_bonus_to_skill", "diplomacy"),
                        choice("core:ability:2_racial_bonus_to_skill", "medicine"),
                    ],
                ),
                "mystic",
                &[
                    "detect_affliction",
                    "detect_magic",
                    "ghost_sound",
                    "grave_words",
                    "stabilize",
                    "telepathic_message",
                    "charm_person",
                    "command",
                    "mystic_cure_level_1",
                    "share_language",
                    "hold_person",
                    "remove_condition",
                    "status",
                ],
                &["detect_thoughts", "zone_of_truth"]),
            ),
            (
                "SF-Technomancer-5",
                "technomancer",
                with_spells(seed_input(
                    ("technomancer", 5, "android"),
                    [10, 16, 14, 19, 13, 8],
                    &[
                        "core:ability:scholar",
                        "core:feat:spell_penetration",
                        "core:feat:spell_focus",
                        "core:feat:mobility",
                        "core:feat:quick_draw",
                    ],
                    &[
                        ("computers", 5),
                        ("engineering", 5),
                        ("life_science", 5),
                        ("mysticism", 5),
                        ("physical_science", 5),
                        ("piloting", 5),
                        ("sleight_of_hand", 5),
                        ("perception", 5),
                    ],
                    "d_suit_i",
                    &["laser_pistol_azimuth", "battery", "battery", "medkit_basic", "serum_of_healing_mk_1", "serum_of_healing_mk_1"],
                    vec![choice(
                        "core:ability:scholar_theme_benefit_theme_knowledge",
                        "core:pool_option:scholar_theme_chosen_skill_physical_science",
                    )],
                ),
                "technomancer",
                &[
                    "dancing_lights",
                    "detect_magic",
                    "energy_ray",
                    "mending",
                    "token_spell",
                    "transfer_charge",
                    "detect_tech",
                    "magic_missile",
                    "overheat",
                    "supercharge_weapon",
                    "invisibility",
                    "knock",
                    "mirror_image",
                ],
                &[]),
            ),
            (
                "SF-Envoy-3",
                "envoy",
                seed_input(
                    ("envoy", 3, "ysoki"),
                    [8, 13, 12, 12, 10, 18],
                    &["core:ability:icon", "core:feat:mobility", "core:feat:quick_draw"],
                    &[
                        ("bluff", 3),
                        ("computers", 3),
                        ("culture", 3),
                        ("diplomacy", 3),
                        ("engineering", 3),
                        ("intimidate", 3),
                        ("perception", 3),
                        ("sense_motive", 3),
                        ("stealth", 3),
                    ],
                    "carbon_skin_graphite",
                    &["semi_auto_pistol_tactical", "baton_tactical", "serum_of_healing_mk_1", "serum_of_healing_mk_1"],
                    Vec::new(),
                ),
            ),
        ]
    }

    fn ordinal_level(text: &str) -> u8 {
        match text {
            "0" => 0,
            "1st" => 1,
            "2nd" => 2,
            "3rd" => 3,
            other => other.trim_end_matches("th").parse().unwrap_or_else(|_| panic!("spell level {other:?}")),
        }
    }

    /// Every SRD hand value of the four seeds (160 rows: `seed-hand-values.md` 126,
    /// initiative 4, spell DC 6, loadout 24) against the `sf.*` explanation row the
    /// adapter's chassis carries for it. A trained-only skill without ranks has no row.
    #[test]
    fn sf_seed_every_hand_value_equals_the_adapters_explanation_row() {
        let adapter = StarfinderAdapter;
        let package = package().expect("the Starfinder package loads");
        let hand = hand_values();
        let mut compared = 0usize;
        let mut mismatches = Vec::new();
        for (seed, class, input) in seeds() {
            let chassis = adapter.chassis_resolve(&input);
            assert!(blocking(&chassis).is_none(), "{seed}: {:?}", chassis.diagnostics);
            assert!(chassis.diagnostics.iter().all(|d| !crate::rule_system_adapter::tests::is_would_message(&d.message)
                && d.id != crate::rule_system_adapter::tests::STUB_NOT_YET_IMPLEMENTED));
            let sheet = compute_sheet(package, &input).expect("the seed computes");
            let rows: BTreeMap<&str, i16> = chassis.explanations.iter().map(|e| (e.id.as_str(), e.value)).collect();
            for (s, field, value) in hand.iter().filter(|(s, _, _)| s == seed) {
                // The row `Profession` stands for every `Profession (…)` skill the package
                // holds (as E4.2's `sf_skills` seed test reads it).
                if field == "Skill: Profession" {
                    let professions: Vec<&str> = sheet
                        .skills
                        .skills
                        .iter()
                        .filter(|k| k.label.starts_with("Profession ("))
                        .map(|k| k.skill.as_str())
                        .collect();
                    assert!(!professions.is_empty(), "{s}: no Profession skill in the package");
                    compared += 1;
                    for slug in professions {
                        let engine = rows.get(format!("sf.skill.{slug}").as_str()).map(|v| i64::from(*v));
                        if engine != *value {
                            mismatches.push(format!("{s} {field} ({slug}): adapter {engine:?}, SRD {value:?}"));
                        }
                    }
                    continue;
                }
                let id = match field.as_str() {
                    "BAB" => "sf.base_attack_bonus".to_owned(),
                    "Fort" => "sf.fortitude".to_owned(),
                    "Ref" => "sf.reflex".to_owned(),
                    "Will" => "sf.will".to_owned(),
                    "HP" => "sf.hit_points".to_owned(),
                    "Stamina" => "sf.stamina".to_owned(),
                    "Resolve" => "sf.resolve".to_owned(),
                    "EAC" => "sf.eac".to_owned(),
                    "KAC" => "sf.kac".to_owned(),
                    "Initiative" => "sf.initiative".to_owned(),
                    "Starting credits" => "sf.credits.starting".to_owned(),
                    "Credits spent" => "sf.credits.spent".to_owned(),
                    "Credits remaining" => "sf.credits.remaining".to_owned(),
                    "Bulk" => "sf.bulk".to_owned(),
                    "Bulk limit unencumbered" => "sf.bulk_limit.unencumbered_max".to_owned(),
                    "Bulk limit overburdened" => "sf.bulk_limit.overburdened_above".to_owned(),
                    f if f.starts_with("Skill: ") => {
                        let label = &f["Skill: ".len()..];
                        let skill = sheet.skills.by_label(label).unwrap_or_else(|| panic!("{s}: no skill {label:?}"));
                        format!("sf.skill.{}", skill.skill)
                    }
                    f if f.starts_with("Spells per day: ") => {
                        format!("sf.spells.{class}.{}.per_day", ordinal_level(&f["Spells per day: ".len()..]))
                    }
                    f if f.starts_with("Spells known: ") => {
                        format!("sf.spells.{class}.{}.known", ordinal_level(&f["Spells known: ".len()..]))
                    }
                    f if f.starts_with("Spell DC: ") => {
                        format!("sf.spells.{class}.{}.save_dc", ordinal_level(&f["Spell DC: ".len()..]))
                    }
                    other => panic!("{s}: hand-value field {other:?} has no sf.* row mapping"),
                };
                compared += 1;
                let engine = rows.get(id.as_str()).map(|v| i64::from(*v));
                if engine != *value {
                    mismatches.push(format!("{s} {field} ({id}): adapter {engine:?}, SRD {value:?}"));
                }
            }
        }
        assert!(mismatches.is_empty(), "{} mismatches:\n{}", mismatches.len(), mismatches.join("\n"));
        assert_eq!(compared, 160, "every hand-value row of the four seeds is compared");
    }

    /// E6.3: the Starfinder sheet prints no number the engine did not send as a row. The
    /// two sheet numbers that were build inputs rather than rows — each ability score and
    /// each class's level — are `sf.ability_score.<ability>` and `sf.class_level.<class>`.
    #[test]
    fn every_seed_carries_its_ability_score_and_class_level_rows() {
        for (seed, class, input) in seeds() {
            let rows = StarfinderAdapter.chassis_resolve(&input).explanations;
            let value = |id: &str| rows.iter().find(|e| e.id == id).map(|e| i64::from(e.value));
            let s = &input.chosen.ability_scores;
            let scores = [s.strength, s.dexterity, s.constitution, s.intelligence, s.wisdom, s.charisma];
            for (name, score) in ABILITY_NAMES.iter().zip(scores) {
                assert_eq!(value(&format!("sf.ability_score.{name}")), Some(i64::from(score)), "{seed} {name}");
            }
            let level = input.chosen.class_levels[0].level;
            assert_eq!(value(&format!("sf.class_level.{class}")), Some(i64::from(level)), "{seed}");
        }
    }

    /// The directory of the frontend Starfinder sheet fixtures (E6.3).
    fn sheet_fixture_dir() -> PathBuf {
        repo_root().join("apps/desktop/src/characterHub/__tests__/starfinderSheetFixtures")
    }

    /// E6.3: the frontend Starfinder sheet test (`starfinderSheet.test.ts`) renders each
    /// seed's real `load_saved_character` response. This keeps those files equal to what the
    /// adapter returns today, so the frontend proof never runs on a stale engine answer.
    /// Regenerate: `SF_SHEET_FIXTURES_WRITE=1 cargo test --bin codex-desktop
    /// the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses`.
    #[test]
    fn the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses() {
        let write = std::env::var_os("SF_SHEET_FIXTURES_WRITE").is_some();
        let characters = tempdir("sheet-fixtures");
        let mut stale = Vec::new();
        for (seed, _, input) in seeds() {
            let root = characters.join(seed);
            SavedCharacterStore::save(&envelope(seed, STARFINDER_RULE_SYSTEM_ID, input), &root).expect("saves");
            let loaded = StarfinderAdapter.load_saved_character(&root).expect("loads");
            let json = serde_json::to_string_pretty(&loaded).expect("serialises") + "\n";
            let path = sheet_fixture_dir().join(format!("{seed}.json"));
            if write {
                std::fs::create_dir_all(sheet_fixture_dir()).expect("fixture dir");
                std::fs::write(&path, &json).expect("writes the fixture");
            } else if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str()) {
                stale.push(path.display().to_string());
            }
        }
        std::fs::remove_dir_all(&characters).ok();
        assert!(
            stale.is_empty(),
            "stale or missing Starfinder sheet fixtures {stale:?}; regenerate with SF_SHEET_FIXTURES_WRITE=1 cargo test --bin codex-desktop the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses"
        );
    }

    /// The `CharacterInput` -> build table of this module's doc comment, on SF-Soldier-3:
    /// the `theme`-pool record is the theme (not a pick), the equipped `ARMOR` record is the
    /// worn armour, repeated selections are one carried item with a quantity, the class-id
    /// choice is the key ability and every other choice is a rule choice. (No seed total
    /// reads `facts.theme` alone, so the seed rows cannot catch a theme read as a pick.)
    #[test]
    fn build_from_input_reads_theme_armour_loadout_and_choices() {
        let package = package().expect("the Starfinder package loads");
        let (build, loadout) =
            build_from_input(package, &crate::rule_system_adapter::tests::sf_soldier_3_input()).expect("maps");
        assert_eq!(build.theme.as_deref(), Some("core:ability:mercenary"));
        assert_eq!(
            build.picks,
            [
                "core:ability:2_racial_stat_bonus",
                "core:feat:weapon_focus",
                "core:feat:quick_draw",
                "core:feat:deadly_aim",
                "core:feat:coordinated_shot",
                // The rifle is equipped (the soldier's primary weapon), so it is held (E5.3).
                "core:equipment:laser_rifle_azimuth",
            ]
        );
        assert_eq!(build.armor.as_deref(), Some("core:equipment:defiance_series_squad"));
        assert_eq!(build.chassis.key_ability_choice, Some(Ability::Str));
        assert_eq!(build.choices, BTreeMap::from([("core:ability:2_racial_stat_bonus".to_owned(), vec!["STR".to_owned()])]));
        assert_eq!(build.skill_ranks.get("athletics"), Some(&3));
        assert_eq!(
            loadout.carried,
            [
                ("core:equipment:laser_rifle_azimuth".to_owned(), 1),
                ("core:equipment:baton_tactical".to_owned(), 1),
                ("core:equipment:battery".to_owned(), 2),
                ("core:equipment:serum_of_healing_mk_1".to_owned(), 2),
            ]
        );
        assert_eq!(loadout.starting_credits, None);
    }

    /// The bulk condition's max-Dex cap and −5 Str/Dex penalty are folded in (E4.5's
    /// discovery, `progress.md`): SF-Soldier-3's armour already caps Dex at +1, so
    /// encumbered (+2) leaves EAC/KAC 16/19 and overburdened (+0) makes them 15/18;
    /// Athletics (Str, ACP −4) takes the worse −5 (+6 → +5), Piloting (Dex, no ACP) takes
    /// −5 (+8 → +3), Intimidate (Cha) is untouched (+6).
    #[test]
    fn bulk_condition_caps_dex_and_takes_the_worse_penalty_on_str_and_dex_skills() {
        let package = package().expect("the Starfinder package loads");
        let sheet = compute_sheet(package, &crate::rule_system_adapter::tests::sf_soldier_3_input()).expect("computes");
        assert_eq!(sheet.carried.condition, BulkCondition::Unencumbered);
        let skill = |skills: &SfSkills, slug: &str| {
            skills.skills.iter().find(|s| s.skill == slug).and_then(|s| s.total.as_ref()).map(|t| t.total)
        };
        for (condition, eac, kac) in [(BulkCondition::Encumbered, 16, 19), (BulkCondition::Overburdened, 15, 18)] {
            let (mut defense, mut skills) = (sheet.defense.clone(), sheet.skills.clone());
            apply_bulk_condition(&mut defense, &mut skills, condition);
            assert_eq!((defense.eac.total, defense.kac.total), (eac, kac), "{condition:?}");
            assert_eq!(skill(&skills, "athletics"), Some(5), "{condition:?}");
            assert_eq!(skill(&skills, "piloting"), Some(3), "{condition:?}");
            assert_eq!(skill(&skills, "intimidate"), Some(6), "{condition:?}");
            // An initiative check is a Dexterity-based check (SRD Rules ID=96: d20 + Dex
            // modifier), and both conditions give "a -5 penalty to Strength- and
            // Dexterity-based checks" (SRD Conditions, Rules ID=165). Soldier +2 -> -3.
            assert_eq!(sheet.defense.initiative.total, 2);
            assert_eq!(defense.initiative.total, -3, "{condition:?}");
        }
        let (mut defense, mut skills) = (sheet.defense.clone(), sheet.skills.clone());
        apply_bulk_condition(&mut defense, &mut skills, BulkCondition::Unencumbered);
        assert_eq!((defense, skills), (sheet.defense.clone(), sheet.skills.clone()));
    }

    /// E5.3 (`decisions.md §20`): an upgrade installed in the worn armour and a fusion on the
    /// rifle (each selection's `applied_modifiers`) reach `SfLoadout::applied`, and an
    /// installed augmentation no longer refuses the sheet. The upgrade's price and bulk and the
    /// augmentation's price move the carried totals; the fusion and the augmentation add no bulk.
    #[test]
    fn applied_modifiers_and_an_augmentation_reach_the_credits_and_bulk_totals() {
        let package = package().expect("the Starfinder package loads");
        let base = crate::rule_system_adapter::tests::sf_soldier_3_input();
        let before = compute_sheet(package, &base).expect("computes").carried;
        let mut input = base.clone();
        for selection in &mut input.chosen.equipment_selections {
            match selection.item_id.as_str() {
                "core:equipment:defiance_series_squad" => {
                    selection.applied_modifiers = vec!["core:equipment_modifier:armor_automated_loader".to_owned()]
                }
                "core:equipment:laser_rifle_azimuth" => {
                    selection.applied_modifiers = vec!["core:equipment_modifier:weapon_ominous".to_owned()]
                }
                _ => {}
            }
        }
        input.chosen.equipment_selections.push(item("cybernetic_vocal_modulator", ActiveState::EquippedActive));
        let mut absent = item("baton_tactical", ActiveState::Absent);
        absent.applied_modifiers = vec!["core:equipment_modifier:weapon_adamantine_alloy".to_owned()];
        input.chosen.equipment_selections.push(absent);
        let (_, loadout) = build_from_input(package, &input).expect("maps");
        assert_eq!(
            loadout.applied,
            ["core:equipment_modifier:armor_automated_loader", "core:equipment_modifier:weapon_ominous"],
            "an absent item's modifiers are not carried"
        );
        let after = compute_sheet(package, &input).unwrap_or_else(|r| panic!("the augmented soldier computes: {r:?}")).carried;
        assert_eq!(after.credits_spent.total, before.credits_spent.total + 750 + 125, "{:?}", after.credits_spent);
        assert_eq!(after.credits_remaining.total, before.credits_remaining.total - 875);
        assert_eq!(after.bulk.total, before.bulk.total + 1, "{:?}", after.bulk);
    }

    /// A build the readers refuse is one claim-blocking diagnostic carrying the
    /// refusal's id and message, and every number 0.
    #[test]
    fn a_refused_build_is_a_named_blocking_diagnostic() {
        let mut input = crate::rule_system_adapter::tests::sf_soldier_3_input();
        input.chosen.race_id = "core:race:no_such_race".to_owned();
        let chassis = StarfinderAdapter.chassis_resolve(&input);
        assert_eq!(chassis.base_attack_bonus, 0);
        assert!(chassis.explanations.is_empty());
        assert_eq!(chassis.diagnostics.len(), 1);
        assert!(chassis.diagnostics[0].claim_blocking);
        assert!(chassis.diagnostics[0].message.contains("core:race:no_such_race"), "{:?}", chassis.diagnostics);

        let mut two_armours = crate::rule_system_adapter::tests::sf_soldier_3_input();
        two_armours.chosen.equipment_selections.push(item("carbon_skin_graphite", ActiveState::EquippedActive));
        let chassis = StarfinderAdapter.chassis_resolve(&two_armours);
        assert_eq!(chassis.diagnostics[0].id, REFUSED_TWO_ARMORS);
    }

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time after the epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codex-sf-adapter-test-{label}-{}-{unique}", std::process::id()));
        std::fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn envelope(character_id: &str, game_system: &str, character_input: CharacterInput) -> SavedCharacterEnvelope {
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: "2026-10-04T00:00:00Z".to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: game_system.to_owned(),
            game_system: game_system.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: format!("{character_id} label"),
            character_input,
        }
    }

    /// E5.2: a saved Starfinder caster keeps its spells known. Each seed saves and loads with
    /// the same `spells_selected` (rule ids `core:spell:<slug>`, classes `core:class:<slug>`),
    /// and the loaded sheet prints the same spell lines the in-memory build prints.
    #[test]
    fn a_saved_starfinder_caster_loads_with_its_spells_known() {
        let adapter = StarfinderAdapter;
        let characters = tempdir("spells");
        for (seed, _, input) in seeds() {
            let root = characters.join(seed);
            SavedCharacterStore::save(&envelope(seed, STARFINDER_RULE_SYSTEM_ID, input.clone()), &root).expect("saves");
            let loaded = adapter.load_saved_character(&root).expect("loads");
            assert!(loaded.diagnostics.iter().all(|d| !d.claim_blocking), "{seed}: {:?}", loaded.diagnostics);
            let spells = |lines: Vec<(String, String)>| lines.into_iter().filter(|(kind, _)| kind == "spell").map(|(_, id)| id).collect::<Vec<_>>();
            let want = spells(adapter.chassis_resolve(&input).sheet_lines.into_iter().map(|l| (l.kind, l.id)).collect());
            let got = spells(loaded.sheet_lines.iter().map(|l| (l.kind.clone(), l.id.clone())).collect());
            assert_eq!(got, want, "{seed}");
            assert_eq!(want.len(), input.chosen.spells_selected.len(), "{seed}");
        }
        std::fs::remove_dir_all(&characters).ok();
    }

    /// The persistence methods over a real `SavedCharacterStore`: a saved SF-Soldier-3
    /// round-trips, lists (a Pathfinder character beside it does not), loads with its
    /// `sf.*` rows, recomputes, appends (a real item: saved; an unknown or over-budget
    /// item: refused, nothing saved), re-saves, and a Pathfinder envelope is refused.
    #[test]
    fn persistence_methods_work_on_a_saved_starfinder_character() {
        let adapter = StarfinderAdapter;
        let characters = tempdir("persistence");
        let root = characters.join("sf-soldier");
        SavedCharacterStore::save(
            &envelope("sf-soldier", STARFINDER_RULE_SYSTEM_ID, crate::rule_system_adapter::tests::sf_soldier_3_input()),
            &root,
        )
        .expect("an SF envelope saves");
        let pf_root = characters.join("pf-fighter");
        let pf_input = crate::character_hub::compose_character_input(&crate::character_hub::CreateCharacterRequest {
            character_id: "pf-fighter".to_owned(),
            display_label: "PF".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: "class:fighter".to_owned(),
            level: 1,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            selected_alternate_trait_keys: Vec::new(),
            companion_species: None,
            selected_traits: Vec::new(),
            trait_skill_choices: Vec::new(),
            additional_choices: Vec::new(),
            additional_levels: Vec::new(),
            hit_point_levels: Vec::new(),
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            selected_spells: Vec::new(),
            selected_equipment: Vec::new(),
            price_mode: crate::character_hub::PriceMode::Standard,
            saved_at: "2026-10-04T00:00:00Z".to_owned(),
        });
        SavedCharacterStore::save(&envelope("pf-fighter", "pf1", pf_input), &pf_root).expect("a PF envelope saves");

        let listing = adapter.list_saved_characters(&characters).expect("lists");
        let ids: Vec<&str> = listing.characters.iter().map(|c| c.character_id.as_str()).collect();
        assert_eq!(ids, ["sf-soldier"]);

        let loaded = adapter.load_saved_character(&root).expect("loads");
        assert_eq!(loaded.summary.game_system, STARFINDER_RULE_SYSTEM_ID);
        assert!(loaded.diagnostics.iter().all(|d| !d.claim_blocking), "{:?}", loaded.diagnostics);
        let row = |id: &str| loaded.explanations.iter().find(|e| e.id == id).map(|e| e.value);
        assert_eq!((row("sf.hit_points"), row("sf.stamina"), row("sf.eac")), (Some(25), Some(24), Some(16)));
        assert_eq!(loaded.snapshot.as_ref().map(|s| s.base_attack_bonus), Some(3));
        assert_eq!(loaded.corpus_derived.encumbrance.level, "unencumbered");
        // E5.1: the loaded sheet prints the race, theme and class-feature lines.
        let printed = |id: &str| loaded.sheet_lines.iter().find(|l| l.id == id).map(|l| (l.label.as_str(), l.prose.as_str()));
        assert_eq!(printed("core:race:human").map(|(_, prose)| prose), Some("Speed: Walk 30 ft."));
        assert_eq!(printed("core:ability:mercenary").map(|(label, _)| label), Some("Mercenary"));
        assert_eq!(printed("core:ability:soldier_class_feature_gear_boost").map(|(label, _)| label), Some("Gear Boost"));

        let recomputed = adapter.recompute(&root, "sf-soldier");
        assert!(recomputed.success, "{:?}", recomputed.error);
        let snapshot = recomputed.character.expect("a snapshot");
        assert_eq!((snapshot.base_attack_bonus, snapshot.total_saves.fortitude), (3, 4));
        assert_eq!(adapter.recompute(&characters.join("missing"), "x").error.as_deref(), Some("character_not_found"));

        let appended = adapter
            .append_to_character(
                &root,
                &[ItemToAppendDto { item_id: "core:equipment:battery".to_owned(), active_state: ActiveStateDto::SelectedInactive }],
                "2026-10-04T01:00:00Z",
            )
            .expect("append runs");
        assert!(appended.success, "{:?}", appended.error);
        let after = SavedCharacterStore::load(&root).expect("reloads");
        assert_eq!(after.revision_id, "sf-soldier.rev.2");
        assert_eq!(
            after.character_input.chosen.equipment_selections.iter().filter(|s| s.item_id == "core:equipment:battery").count(),
            3
        );

        let unknown = adapter
            .append_to_character(
                &root,
                &[ItemToAppendDto { item_id: "core:equipment:no_such_item".to_owned(), active_state: ActiveStateDto::SelectedInactive }],
                "2026-10-04T02:00:00Z",
            )
            .expect("append runs");
        assert_eq!(unknown.error.as_deref(), Some("equipment_not_found: core:equipment:no_such_item"));
        let over_budget: Vec<ItemToAppendDto> = (0..10)
            .map(|_| ItemToAppendDto { item_id: "core:equipment:laser_rifle_azimuth".to_owned(), active_state: ActiveStateDto::SelectedInactive })
            .collect();
        let refused = adapter.append_to_character(&root, &over_budget, "2026-10-04T03:00:00Z").expect("append runs");
        assert!(!refused.success);
        assert!(refused.error.as_deref().is_some_and(|e| e.starts_with("recompute_blocked: sf_loadout.over_budget")), "{:?}", refused.error);
        assert_eq!(SavedCharacterStore::load(&root).expect("reloads").revision_id, "sf-soldier.rev.2", "a refused append saves nothing");

        let saved = adapter.save_character(&root, "sf-soldier.rev.2", "2026-10-04T04:00:00Z").expect("re-saves");
        assert!(saved.success, "{:?}", saved.error);
        assert_eq!(saved.revision_id.as_deref(), Some("sf-soldier.rev.3"));

        let err = adapter.save_character(&pf_root, "pf-fighter.rev.1", "2026-10-04T05:00:00Z").expect_err("a PF character is refused");
        assert!(err.starts_with(REFUSED_NOT_STARFINDER), "{err}");
        assert!(adapter.load_saved_character(&pf_root).is_err());

        std::fs::remove_dir_all(&characters).ok();
    }

    /// The three Tauri command paths route `"starfinder-1e"` to this adapter.
    #[test]
    fn the_command_paths_route_starfinder_1e_to_the_starfinder_adapter() {
        let characters = tempdir("commands");
        let root = characters.join("sf-soldier");
        SavedCharacterStore::save(
            &envelope("sf-soldier", STARFINDER_RULE_SYSTEM_ID, crate::rule_system_adapter::tests::sf_soldier_3_input()),
            &root,
        )
        .expect("saves");
        let recomputed =
            crate::characterHub::recomputeCharacter::recompute_character_via_rule_system(STARFINDER_RULE_SYSTEM_ID, &root, "sf-soldier");
        assert!(recomputed.success, "{:?}", recomputed.error);
        assert_eq!(recomputed.character.map(|c| c.base_attack_bonus), Some(3));
        let appended = crate::characterHub::appendToCharacter::append_to_character_via_rule_system(
            STARFINDER_RULE_SYSTEM_ID,
            &root,
            &[ItemToAppendDto { item_id: "core:equipment:battery".to_owned(), active_state: ActiveStateDto::SelectedInactive }],
            "2026-10-04T01:00:00Z",
        )
        .expect("append runs");
        assert!(appended.success, "{:?}", appended.error);
        let saved = crate::characterHub::reSaveCharacter::re_save_character_via_rule_system(
            STARFINDER_RULE_SYSTEM_ID,
            &root,
            "sf-soldier.rev.2",
            "2026-10-04T02:00:00Z",
        )
        .expect("re-saves");
        assert!(saved.success, "{:?}", saved.error);
        std::fs::remove_dir_all(&characters).ok();
    }

    /// SD-37 E6.1: the `load_saved_character` command loads a saved character through the
    /// adapter its envelope names. A Starfinder save is served by `StarfinderAdapter` (its
    /// `sf.*` rows, its SRD Stamina); a Pathfinder save beside it is served exactly as
    /// `character_hub::load_saved_character_at_root` serves it.
    #[test]
    fn the_load_command_path_routes_a_starfinder_save_to_the_starfinder_adapter() {
        let characters = tempdir("load-route");
        let sf_root = characters.join("sf-soldier");
        SavedCharacterStore::save(
            &envelope("sf-soldier", STARFINDER_RULE_SYSTEM_ID, crate::rule_system_adapter::tests::sf_soldier_3_input()),
            &sf_root,
        )
        .expect("an SF envelope saves");
        let loaded = crate::rule_system_adapter::load_saved_character_via_envelope(&sf_root).expect("loads");
        let direct = StarfinderAdapter.load_saved_character(&sf_root).expect("the adapter loads it");
        assert_eq!(
            serde_json::to_string(&loaded).expect("serialises"),
            serde_json::to_string(&direct).expect("serialises"),
            "the command path is StarfinderAdapter's load"
        );
        let stamina = loaded.explanations.iter().find(|e| e.id == "sf.stamina").map(|e| e.value);
        assert_eq!(stamina, Some(24), "SF-Soldier-3's SRD Stamina comes through the command path");

        let pf_root = characters.join("pf-fighter");
        let mut pf_input = crate::rule_system_adapter::tests::sf_soldier_3_input();
        pf_input.chosen.race_id = "race:human".to_owned();
        pf_input.chosen.class_levels = vec![CharacterClassLevel { class_id: "class:fighter".to_owned(), level: 1 }];
        SavedCharacterStore::save(&envelope("pf-fighter", "pf1", pf_input), &pf_root).expect("a PF envelope saves");
        assert_eq!(
            serde_json::to_string(&crate::rule_system_adapter::load_saved_character_via_envelope(&pf_root)).expect("serialises"),
            serde_json::to_string(&crate::character_hub::load_saved_character_at_root(&pf_root)).expect("serialises"),
            "a Pathfinder save loads exactly as before"
        );
        std::fs::remove_dir_all(&characters).ok();
    }

    /// SD-37 E6.1: the `list_saved_characters` command lists through the selected system's
    /// adapter: Starfinder selected lists the Starfinder save only; Pathfinder (or no id, every
    /// caller outside the character hub) lists exactly what `Pf1Adapter` lists.
    #[test]
    fn the_list_command_path_routes_through_the_selected_rule_system() {
        let characters = tempdir("list-route");
        SavedCharacterStore::save(
            &envelope("sf-soldier", STARFINDER_RULE_SYSTEM_ID, crate::rule_system_adapter::tests::sf_soldier_3_input()),
            &characters.join("sf-soldier"),
        )
        .expect("an SF envelope saves");
        SavedCharacterStore::save(
            &envelope("pf-fighter", "pf1", crate::rule_system_adapter::tests::sf_soldier_3_input()),
            &characters.join("pf-fighter"),
        )
        .expect("a PF envelope saves");
        let ids = |listing: ListSavedCharactersResponse| {
            let mut ids: Vec<String> = listing.characters.into_iter().map(|c| c.character_id).collect();
            ids.sort();
            ids
        };
        let starfinder = crate::rule_system_adapter::list_saved_characters_via_rule_system(Some(STARFINDER_RULE_SYSTEM_ID), &characters)
            .expect("lists");
        assert_eq!(ids(starfinder), vec!["sf-soldier".to_owned()], "Starfinder lists its own saves only");
        let pathfinder_before = ids(crate::pf1_adapter::Pf1Adapter.list_saved_characters(&characters).expect("lists"));
        for id in [Some("pf1"), None] {
            let listed = crate::rule_system_adapter::list_saved_characters_via_rule_system(id, &characters).expect("lists");
            assert_eq!(ids(listed), pathfinder_before, "{id:?} lists as Pf1Adapter does");
        }
        std::fs::remove_dir_all(&characters).ok();
    }

    /// The variable that turns [`starfinder_package_from_a_packaged_resource_root_probe`] on: the
    /// staged resource root its parent test built.
    const PACKAGED_ROOT_PROBE: &str = "CODEX_SF_PACKAGED_ROOT_PROBE";

    /// SD-37 E6.1: a packaged app reads the Starfinder package from its bundled resources, not
    /// from the build machine's checkout. A resource root is staged the way `tauri.conf.json`
    /// bundles it (`data/corpus/`, `data/starfinder-1e/sheet_rules/` holding one converted
    /// class file) and a child process of this test binary, with `CODEX_REPO_ROOT` unset and
    /// `CODEX_DESKTOP_RESOURCE_DIR` naming the staged root, loads this adapter's package: it
    /// must hold exactly the staged file's rules (the checkout's package holds thousands).
    #[test]
    fn a_packaged_app_reads_the_starfinder_package_from_its_resource_root() {
        let staged = tempdir("packaged-root");
        std::fs::create_dir_all(staged.join(GameSystem::Pathfinder1e.corpus_relative())).expect("corpus dir");
        let class_dir = staged.join(GameSystem::Starfinder1e.sheet_rules_relative()).join("core/class");
        std::fs::create_dir_all(&class_dir).expect("sheet rules dir");
        std::fs::copy(
            repo_root().join(GameSystem::Starfinder1e.sheet_rules_relative()).join("core/class/soldier.json"),
            class_dir.join("soldier.json"),
        )
        .expect("a converted class file copies");
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .args(["sf_adapter::tests::starfinder_package_from_a_packaged_resource_root_probe", "--exact", "--ignored", "--nocapture"])
            .env_remove("CODEX_REPO_ROOT")
            .env("CODEX_DESKTOP_RESOURCE_DIR", &staged)
            .env(PACKAGED_ROOT_PROBE, &staged)
            .output()
            .expect("the probe runs");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_dir_all(&staged).ok();
        assert!(stdout.contains("1 passed"), "the probe must run and pass:\n{stdout}\n{stderr}");
    }

    /// Runs only as the child of [`a_packaged_app_reads_the_starfinder_package_from_its_resource_root`].
    #[test]
    #[ignore = "child process of a_packaged_app_reads_the_starfinder_package_from_its_resource_root"]
    fn starfinder_package_from_a_packaged_resource_root_probe() {
        let Some(staged) = std::env::var_os(PACKAGED_ROOT_PROBE) else { return };
        let staged_dir = PathBuf::from(staged).join(GameSystem::Starfinder1e.sheet_rules_relative());
        let expected = codex::rules_core::corpus_loader::load_sheet_rules(&staged_dir).package.rules.len();
        assert!(expected > 0, "the staged file carries rules");
        let loaded = package().expect("the Starfinder package loads").rules.len();
        assert_eq!(loaded, expected, "the package is the staged resource root's, not the checkout's");
    }

}
