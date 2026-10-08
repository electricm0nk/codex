//! Starfinder 1e feats, spells known and gear, chosen through the app (SD-37 E6.5a,
//! `docs/release/SD-37-starfinder-1e/epic-breakdown.md` Epic E6): the one set of choices the
//! creation form, the level-up dialog and the Starfinder sheet's "Feats, spells and gear" dialog
//! all edit, read from the converted Starfinder package and saved as the `CharacterInput`
//! [`crate::sf_adapter`] reads.
//!
//! # What is offered, and where each answer comes from
//!
//! | Choice | Offered / checked by | Saved as |
//! |---|---|---|
//! | feats | every `feat` record of the package; each one's prerequisite is the record's own `applies`, evaluated by the engine for this character (its held set, with the chassis's base attack bonus) | `selected_feats` (the record ids; the theme and every other held pick are kept) |
//! | a chosen feat's own pick (Weapon Focus's weapon type) | the held set's discovery ([`crate::sf_creation::discover`]): a slot whose choosing rule is a chosen feat | rule choices `(feat, option)` |
//! | spells known | for each class the character casts with, each castable spell level: the engine's spells-known total ([`sf_spells`]) and its terms, and every spell record on that class's list at that level ([`crate::sf_sheet_print::spell_level_on`]) | `spells_selected` (`Known`; a selection saved another way is kept) |
//! | gear | every `equipment` record of the package, printed with its own `Price` row | `equipment_selections`, one per item: equipped (worn armour, a wielded weapon, an installed augmentation) or carried |
//! | upgrades and fusions on an item | every `equipment_modifier` record that fits the item ([`crate::sf_sheet_print::modifier_fits`]) | that selection's `applied_modifiers` |
//!
//! # What is computed, and what is printed
//!
//! The totals these choices reach are the engine's: [`sf_adapter::compute_sheet`] over the
//! character with the choices applied, and the preview returns its EAC, KAC, credits and bulk
//! rows. A loadout over the character's credits, an upgrade that does not fit, two worn armours
//! or a spell off the class's list is the engine's own refusal, by name, and blocks the save.
//!
//! How many feats a character has is not a sheet total (`decisions.md §5`): the advancement
//! table's feat rule and every feat pool the held set carries are printed, never counted
//! against the feats chosen. A feat whose prerequisite the engine evaluates as not met, a feat
//! taken twice that is not repeatable, and more spells at a level than the engine's
//! spells-known total are refused by name; fewer spells than known are listed, not blocking.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use codex::rules_core::character_input::{ActiveState, AcquisitionMode, CharacterInput, EquipmentSelection, SelectedChoice, SpellSelection};
use codex::rules_core::pilot_compute::sf_chassis::SfChassisRefusal;
use codex::rules_core::pilot_compute::sf_defense::{self, SfHeld};
use codex::rules_core::pilot_compute::sf_spells;
use codex::rules_core::sheet_rule::{evaluate_applies, split_rule_id, EvalContext, SheetRule, SheetRulePackage};
use codex::saved_character::local_store::SavedCharacterStore;

use crate::character_hub::{DiagnosticDto, ExplanationDto};
use crate::sf_adapter;
use crate::sf_creation::{self, pick_problems, SfCreateResponse, SfCreationSlotDto, SfOptionDto, SfPickDto, NOT_CHOSEN_YET};
use crate::sf_sheet_print::{modifier_fits, spell_level_on, stat_row};

pub const REFUSED_NOT_A_FEAT: &str = "sf_choices.not_a_feat";
pub const REFUSED_FEAT_PREREQUISITE: &str = "sf_choices.feat_prerequisite";
pub const REFUSED_FEAT_TWICE: &str = "sf_choices.feat_twice";
pub const REFUSED_SPELL_NOT_OFFERED: &str = "sf_choices.spell_not_offered";
pub const REFUSED_SPELLS_OVER_KNOWN: &str = "sf_choices.spells_over_known";
pub const REFUSED_NOT_EQUIPMENT: &str = "sf_choices.not_equipment";
pub const REFUSED_MODIFIER_NOT_OFFERED: &str = "sf_choices.modifier_not_offered";

/// The totals the preview returns: the ones gear and feats reach on the sheet.
pub const TOTAL_ROWS: [&str; 6] = ["sf.eac", "sf.kac", "sf.credits.starting", "sf.credits.spent", "sf.credits.remaining", "sf.bulk"];

const SRD_FEATS: &str = "SRD Table 2-4: Character Advancement (https://www.aonsrd.com/Rules.aspx?ID=56)";

/// One spell known: the class it is known through and the spell record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfSpellPickDto {
    pub class_id: String,
    pub spell_id: String,
}

/// One item the character has: equipped (worn, wielded, installed) or carried, with the
/// upgrades and fusions applied to it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfGearDto {
    pub item_id: String,
    #[serde(default)]
    pub equipped: bool,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

/// The character's feats, the feats' own picks, spells known and gear.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfChoicesDto {
    #[serde(default)]
    pub feats: Vec<String>,
    #[serde(default)]
    pub feat_picks: Vec<SfPickDto>,
    #[serde(default)]
    pub spells: Vec<SfSpellPickDto>,
    #[serde(default)]
    pub gear: Vec<SfGearDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfFeatOptionDto {
    pub id: String,
    pub label: String,
    /// The engine's evaluation of the feat's prerequisite for this character.
    pub eligible: bool,
    pub repeatable: bool,
}

/// One castable spell level of one class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfSpellLevelChoiceDto {
    pub class_id: String,
    pub class_label: String,
    pub level: u8,
    /// The engine's spells-known total at this level.
    pub known: i64,
    /// Its terms, in the engine's words (`Mystic spells known 4`, a connection's spell).
    pub known_terms: Vec<String>,
    /// The spells chosen at this level, in choice order.
    pub chosen: Vec<SfOptionDto>,
    /// Every spell on the class's list at this level.
    pub options: Vec<SfOptionDto>,
}

/// One item of the gear, by its position in [`SfChoicesDto::gear`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfGearLineDto {
    pub position: usize,
    pub item_id: String,
    pub label: String,
    pub equipped: bool,
    pub modifiers: Vec<SfOptionDto>,
    /// Every upgrade, fusion or material that fits this item.
    pub modifier_options: Vec<SfOptionDto>,
}

/// One equipment record the gear picker offers, with its own printed price.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfEquipmentOptionDto {
    pub id: String,
    pub label: String,
    /// The record's `Price` row as it states it (credits); `None` when it states none.
    pub price: Option<String>,
}

/// The choices' next state: the lists, the totals they reach and every problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfChoicesPreviewDto {
    /// The choices this preview is for (the saved character's when none were sent).
    pub chosen: SfChoicesDto,
    /// The feats the character's rules grant, printed: the advancement table's rule and every
    /// feat pool or feat pick the held set carries.
    pub feat_rules: Vec<String>,
    pub feats: Vec<SfOptionDto>,
    pub feat_options: Vec<SfFeatOptionDto>,
    /// The picks a chosen feat asks for (Weapon Focus: a weapon type).
    pub feat_slots: Vec<SfCreationSlotDto>,
    pub spell_levels: Vec<SfSpellLevelChoiceDto>,
    pub gear: Vec<SfGearLineDto>,
    /// [`TOTAL_ROWS`], the engine's values with the choices applied.
    pub totals: Vec<ExplanationDto>,
    pub problems: Vec<DiagnosticDto>,
}

fn problem(id: &str, message: impl Into<String>) -> DiagnosticDto {
    DiagnosticDto { id: id.to_owned(), message: message.into(), claim_blocking: true }
}

fn refusal_problem(refusal: &SfChassisRefusal) -> DiagnosticDto {
    problem(refusal.id, refusal.message.clone())
}

fn kind_of(id: &str) -> &str {
    split_rule_id(id).1
}

fn is_feat_id(id: &str) -> bool {
    !id.contains('#') && kind_of(id) == "feat"
}

fn label_of(package: &SheetRulePackage, id: &str) -> String {
    package.rule(id).map_or_else(|| id.to_owned(), |r| r.label.clone())
}

/// The choices a saved character holds.
pub fn choices_of(input: &CharacterInput) -> SfChoicesDto {
    let chosen = &input.chosen;
    let feats: Vec<String> = chosen.selected_feats.iter().filter(|id| is_feat_id(id)).cloned().collect();
    SfChoicesDto {
        feat_picks: chosen
            .selected_choices
            .iter()
            .filter(|c| feats.contains(&c.choice_set_id))
            .map(|c| SfPickDto { slot_id: c.choice_set_id.clone(), option_id: c.selection_id.clone() })
            .collect(),
        feats,
        spells: chosen
            .spells_selected
            .iter()
            .filter(|s| s.acquisition_mode == AcquisitionMode::Known)
            .map(|s| SfSpellPickDto { class_id: s.source_class_id.clone(), spell_id: s.spell_id.clone() })
            .collect(),
        gear: chosen
            .equipment_selections
            .iter()
            .filter(|s| s.active_state != ActiveState::Absent)
            .map(|s| SfGearDto {
                item_id: s.item_id.clone(),
                equipped: s.active_state == ActiveState::EquippedActive,
                modifiers: s.applied_modifiers.clone(),
            })
            .collect(),
    }
}

/// `input` with its feats, the feats' picks, its spells known and its gear replaced by
/// `choices`. Everything else -- the theme and every other held pick, a spell saved another
/// way, a selection marked absent -- is kept.
pub fn apply(input: &CharacterInput, choices: &SfChoicesDto) -> CharacterInput {
    let mut out = input.clone();
    let chosen = &mut out.chosen;
    let old_feats: BTreeSet<String> = chosen.selected_feats.iter().filter(|id| is_feat_id(id)).cloned().collect();
    chosen.selected_feats.retain(|id| !is_feat_id(id));
    chosen.selected_feats.extend(choices.feats.iter().cloned());
    chosen
        .selected_choices
        .retain(|c| !old_feats.contains(&c.choice_set_id) && !choices.feats.contains(&c.choice_set_id));
    chosen.selected_choices.extend(
        choices.feat_picks.iter().map(|p| SelectedChoice { choice_set_id: p.slot_id.clone(), selection_id: p.option_id.clone() }),
    );
    chosen.spells_selected.retain(|s| s.acquisition_mode != AcquisitionMode::Known);
    chosen.spells_selected.extend(choices.spells.iter().map(|s| SpellSelection {
        spell_id: s.spell_id.clone(),
        source_class_id: s.class_id.clone(),
        acquisition_mode: AcquisitionMode::Known,
    }));
    chosen.equipment_selections.retain(|s| s.active_state == ActiveState::Absent);
    chosen.equipment_selections.extend(choices.gear.iter().map(|g| {
        let active_state = if g.equipped { ActiveState::EquippedActive } else { ActiveState::SelectedInactive };
        EquipmentSelection {
            item_id: g.item_id.clone(),
            equipped_or_active: g.equipped,
            active_state,
            applied_modifiers: g.modifiers.clone(),
        }
    }));
    out
}

/// Every equipment record the gear picker offers, in name order, each with its own `Price` row.
pub fn equipment_options(package: &SheetRulePackage) -> Vec<SfEquipmentOptionDto> {
    let mut out: Vec<SfEquipmentOptionDto> = package
        .rules_of_kind("equipment")
        .filter(|r| !r.id.contains('#'))
        .map(|r| SfEquipmentOptionDto { id: r.id.clone(), label: r.label.clone(), price: stat_row(r, "Price").map(|p| p.trim().to_owned()) })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

/// Every `equipment_modifier` record that fits `item`, in name order.
fn modifier_options(package: &SheetRulePackage, item: &SheetRule) -> Vec<SfOptionDto> {
    let mut out: Vec<SfOptionDto> = package
        .rules_of_kind("equipment_modifier")
        .filter(|m| !m.id.contains('#') && modifier_fits(item, m).is_ok())
        .map(|m| SfOptionDto { id: m.id.clone(), label: m.label.clone() })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

/// The feat options and their prerequisite gates, evaluated over the held set with the
/// chassis's base attack bonus (the readers' held set leaves the fact at 0; the chassis computes it).
fn feat_options(package: &SheetRulePackage, sf: &SfHeld, base_attack: i64) -> Vec<SfFeatOptionDto> {
    let mut facts = sf.facts.clone();
    facts.base_attack = base_attack;
    let mut out: Vec<SfFeatOptionDto> = package
        .rules_of_kind("feat")
        .filter(|r| !r.id.contains('#'))
        .map(|r| SfFeatOptionDto {
            id: r.id.clone(),
            label: r.label.clone(),
            eligible: evaluate_applies(&r.applies, &sf.held, package, &facts, EvalContext::default()).includes(),
            repeatable: r.repeatable,
        })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
    out
}

fn held_of(package: &SheetRulePackage, input: &CharacterInput) -> Result<(sf_defense::SfBuild, SfHeld), SfChassisRefusal> {
    let (build, _) = sf_adapter::build_from_input(package, input)?;
    let held = sf_defense::held(package, &build)?;
    Ok((build, held))
}

/// The choices' next state for `input`, the character with the choices applied ([`apply`]).
pub fn preview(package: &SheetRulePackage, input: &CharacterInput) -> SfChoicesPreviewDto {
    let chosen = choices_of(input);
    let mut out = SfChoicesPreviewDto {
        chosen: chosen.clone(),
        feat_rules: vec![format!("Character feats: one at 1st level and one at every odd character level ({SRD_FEATS})")],
        feats: chosen.feats.iter().map(|id| SfOptionDto { id: id.clone(), label: label_of(package, id) }).collect(),
        feat_options: Vec::new(),
        feat_slots: Vec::new(),
        spell_levels: Vec::new(),
        gear: Vec::new(),
        totals: Vec::new(),
        problems: Vec::new(),
    };

    // Gear: each item a package equipment record, each modifier one that fits it.
    for (position, gear) in chosen.gear.iter().enumerate() {
        let item = package.rule(&gear.item_id).filter(|r| kind_of(&r.id) == "equipment" && !r.id.contains('#'));
        let Some(item) = item else {
            out.problems.push(problem(REFUSED_NOT_EQUIPMENT, format!("{}: not a Starfinder equipment record", gear.item_id)));
            continue;
        };
        let options = modifier_options(package, item);
        for id in &gear.modifiers {
            if !options.iter().any(|o| &o.id == id) {
                out.problems.push(problem(REFUSED_MODIFIER_NOT_OFFERED, format!("{id}: not an upgrade, fusion or material that fits {}", item.label)));
            }
        }
        out.gear.push(SfGearLineDto {
            position,
            item_id: item.id.clone(),
            label: item.label.clone(),
            equipped: gear.equipped,
            modifiers: gear.modifiers.iter().map(|id| SfOptionDto { id: id.clone(), label: label_of(package, id) }).collect(),
            modifier_options: options,
        });
    }

    // Feats: each a package feat record, its prerequisite met, taken once unless repeatable.
    for id in &chosen.feats {
        if !package.rule(id).is_some_and(|r| is_feat_id(&r.id)) {
            out.problems.push(problem(REFUSED_NOT_A_FEAT, format!("{id}: not a Starfinder feat record")));
        }
    }
    let unknown = out.problems.iter().any(|p| p.id == REFUSED_NOT_A_FEAT || p.id == REFUSED_NOT_EQUIPMENT);
    let held = if unknown { None } else { Some(held_of(package, input)) };
    let sheet = if unknown { None } else { Some(sf_adapter::compute_sheet(package, input)) };
    match (&held, &sheet) {
        (Some(Ok((build, sf))), Some(sheet)) => {
            let base_attack = match sheet {
                Ok(s) => s.chassis.base_attack_bonus.total,
                Err(_) => sf_chassis_bab(package, build),
            };
            out.feat_options = feat_options(package, sf, base_attack);
            let mut seen = BTreeSet::new();
            for id in &chosen.feats {
                let Some(option) = out.feat_options.iter().find(|o| &o.id == id) else { continue };
                if !option.eligible {
                    out.problems.push(problem(REFUSED_FEAT_PREREQUISITE, format!("{}: its prerequisite is not met by this character", option.label)));
                }
                if !seen.insert(id.clone()) && !option.repeatable {
                    out.problems.push(problem(REFUSED_FEAT_TWICE, format!("{}: taken twice; it is not repeatable", option.label)));
                }
            }
            let counts = sf_creation::pool_counts(package, sf);
            let found = sf_creation::discover(package, sf, &counts);
            out.feat_rules.extend(found.chosen_on_the_sheet.iter().cloned());
            out.feat_slots = found.slots.into_iter().filter(|s| chosen.feats.contains(&s.slot_id)).collect();
            out.problems.extend(pick_problems(&chosen.feat_picks, &out.feat_slots));
            spell_levels(package, build, sf, &chosen, &mut out);
        }
        (Some(Err(refusal)), _) => out.problems.push(refusal_problem(refusal)),
        _ => {}
    }

    match sheet {
        Some(Ok(sheet)) => {
            let rows = sheet.explanations();
            out.totals = TOTAL_ROWS
                .iter()
                .filter_map(|id| rows.iter().find(|e| e.id == *id))
                .map(|e| ExplanationDto { id: e.id.clone(), value: e.value, detail: e.detail.clone() })
                .collect();
        }
        Some(Err(refusal)) if !out.problems.iter().any(|p| p.id == refusal.id) => out.problems.push(refusal_problem(&refusal)),
        Some(Err(_)) | None => {}
    }
    out
}

/// The chassis's base attack bonus when the full sheet refuses (an over-budget loadout still
/// shows which feats the character qualifies for).
fn sf_chassis_bab(package: &SheetRulePackage, build: &sf_defense::SfBuild) -> i64 {
    codex::rules_core::pilot_compute::sf_chassis::compute(package, &build.chassis).map_or(0, |c| c.base_attack_bonus.total)
}

/// The castable spell levels of every class the character casts with, the spells chosen at
/// each, and the problems: a spell not on a held class's list, more than the engine's total.
fn spell_levels(package: &SheetRulePackage, build: &sf_defense::SfBuild, sf: &SfHeld, chosen: &SfChoicesDto, out: &mut SfChoicesPreviewDto) {
    let casting = match sf_spells::compute_with(package, build, sf) {
        Ok(casting) => casting,
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return;
        }
    };
    let spells: Vec<&SheetRule> = package.rules_of_kind("spell").filter(|r| !r.id.contains('#')).collect();
    let mut placed: BTreeSet<usize> = BTreeSet::new();
    for class in &casting {
        let slug = split_rule_id(&class.class).2.to_string();
        let class_label = label_of(package, &class.class);
        for level in class.levels.iter().filter(|l| l.castable()) {
            let known = level.known.as_ref().expect("castable");
            let mut options: Vec<SfOptionDto> = spells
                .iter()
                .filter(|s| spell_level_on(s, &slug) == Some(level.level))
                .map(|s| SfOptionDto { id: s.id.clone(), label: s.label.clone() })
                .collect();
            options.sort_by(|a, b| a.label.cmp(&b.label).then(a.id.cmp(&b.id)));
            let mut picked = Vec::new();
            for (i, pick) in chosen.spells.iter().enumerate() {
                if pick.class_id == class.class && options.iter().any(|o| o.id == pick.spell_id) {
                    placed.insert(i);
                    picked.push(SfOptionDto { id: pick.spell_id.clone(), label: label_of(package, &pick.spell_id) });
                }
            }
            let n = picked.len() as i64;
            if n > known.total {
                out.problems.push(problem(
                    REFUSED_SPELLS_OVER_KNOWN,
                    format!("{class_label} level {} spells: {n} chosen; {} known", level.level, known.total),
                ));
            } else if n < known.total {
                out.problems.push(DiagnosticDto {
                    id: NOT_CHOSEN_YET.to_owned(),
                    message: format!("{class_label} level {} spells: choose {} ({n} chosen)", level.level, known.total),
                    claim_blocking: false,
                });
            }
            out.spell_levels.push(SfSpellLevelChoiceDto {
                class_id: class.class.clone(),
                class_label: class_label.clone(),
                level: level.level,
                known: known.total,
                known_terms: known.terms.iter().map(|t| format!("{} {}", t.label, t.value)).collect(),
                chosen: picked,
                options,
            });
        }
    }
    for (i, pick) in chosen.spells.iter().enumerate() {
        if !placed.contains(&i) {
            out.problems.push(problem(
                REFUSED_SPELL_NOT_OFFERED,
                format!("{}: not a spell this character's {} can know", pick.spell_id, label_of(package, &pick.class_id)),
            ));
        }
    }
}

/// Whether nothing in the preview blocks a save.
pub fn blocked(preview: &SfChoicesPreviewDto) -> bool {
    preview.problems.iter().any(|p| p.claim_blocking)
}

/// The sheet dialog's request: the saved character and, once the player has changed anything,
/// the whole set of choices.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfChoicesRequest {
    #[serde(default)]
    pub character_id: String,
    #[serde(default)]
    pub saved_at: String,
    #[serde(default)]
    pub choices: Option<SfChoicesDto>,
}

/// The dialog's next state for the saved character at `root`.
pub fn preview_at_root(package: &SheetRulePackage, root: &Path, request: &SfChoicesRequest) -> Result<SfChoicesPreviewDto, String> {
    let envelope = sf_adapter::load_starfinder(root)?;
    let input = match &request.choices {
        Some(choices) => apply(&envelope.character_input, choices),
        None => envelope.character_input,
    };
    Ok(preview(package, &input))
}

/// Saves the choices on the saved character at `root`: the next revision, with every total
/// recomputed, or refused with every blocking problem and nothing saved.
pub fn save_at_root(package: &SheetRulePackage, root: &Path, request: &SfChoicesRequest) -> Result<SfCreateResponse, String> {
    let mut envelope = sf_adapter::load_starfinder(root)?;
    let choices = request.choices.clone().unwrap_or_else(|| choices_of(&envelope.character_input));
    let input = apply(&envelope.character_input, &choices);
    let shown = preview(package, &input);
    if blocked(&shown) {
        return Ok(SfCreateResponse::Blocked { diagnostics: shown.problems.into_iter().filter(|p| p.claim_blocking).collect() });
    }
    let sheet = match sf_adapter::compute_sheet(package, &input) {
        Ok(sheet) => sheet,
        Err(refusal) => return Ok(SfCreateResponse::Blocked { diagnostics: vec![refusal_problem(&refusal)] }),
    };
    let next = sf_adapter::next_revision_id(&envelope.character_id, &envelope.revision_id);
    envelope.revision_id = next.clone();
    envelope.latest_authoritative_revision_ref = next;
    envelope.saved_at = request.saved_at.clone();
    envelope.character_input = input;
    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;
    Ok(SfCreateResponse::Saved {
        summary: Box::new(sf_adapter::summary_dto(&envelope)),
        explanations: sheet.explanations().into_iter().map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail }).collect(),
    })
}

fn package_or_error() -> Result<&'static SheetRulePackage, String> {
    sf_adapter::package().map_err(|r| format!("{}: {}", r.id, r.message))
}

/// The "Feats, spells and gear" dialog's next state for a saved Starfinder character.
#[tauri::command]
pub fn preview_starfinder_choices(app: tauri::AppHandle, request: SfChoicesRequest) -> Result<SfChoicesPreviewDto, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    preview_at_root(package_or_error()?, &root, &request)
}

/// Saves a saved Starfinder character's feats, spells known and gear.
#[tauri::command]
pub fn save_starfinder_choices(app: tauri::AppHandle, request: SfChoicesRequest) -> Result<SfCreateResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    save_at_root(package_or_error()?, &root, &request)
}

/// Every Starfinder equipment record the gear picker offers.
#[tauri::command]
pub fn list_starfinder_equipment_options() -> Result<Vec<SfEquipmentOptionDto>, String> {
    Ok(equipment_options(package_or_error()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::sf_level_up::tests::{create_seed, level_once, seed_increase};
    use crate::sf_creation::tests::tempdir;

    fn package() -> &'static SheetRulePackage {
        sf_adapter::package().expect("the Starfinder package loads")
    }

    fn saved(root: &Path) -> CharacterInput {
        SavedCharacterStore::load(root).expect("loads").character_input
    }

    fn values(input: &CharacterInput) -> BTreeMap<String, i64> {
        sf_adapter::compute_sheet(package(), input)
            .expect("computes")
            .explanations()
            .into_iter()
            .map(|e| (e.id, i64::from(e.value)))
            .collect()
    }

    /// The seed's feats, feat picks, spells known and gear as the dialog sends them: every spell
    /// the seed knows (a mystic's connection spells included, chosen from the class list) as
    /// a choice.
    fn seed_choices(seed: &CharacterInput) -> SfChoicesDto {
        let mut choices = choices_of(seed);
        choices.spells = seed
            .chosen
            .spells_selected
            .iter()
            .map(|s| SfSpellPickDto { class_id: s.source_class_id.clone(), spell_id: s.spell_id.clone() })
            .collect();
        choices
    }

    const SECTIONS: [(&str, &str); 4] =
        [("SF-Soldier-3", "1. SF-Soldier-3"), ("SF-Mystic-5", "2. SF-Mystic-5"), ("SF-Technomancer-5", "3. SF-Technomancer-5"), ("SF-Envoy-3", "4. SF-Envoy-3")];

    /// The hand-valued fields the loadout reaches (`seed-hand-values.md`, `E4.5-loadout-hand-values.md`).
    const LOADOUT_FIELDS: [(&str, &str); 6] = [
        ("EAC", "sf.eac"),
        ("KAC", "sf.kac"),
        ("Starting credits", "sf.credits.starting"),
        ("Credits spent", "sf.credits.spent"),
        ("Credits remaining", "sf.credits.remaining"),
        ("Bulk", "sf.bulk"),
    ];

    /// Creates `seed` at 1st level through the creation flow and levels it to its level through
    /// the level-up (E6.5); returns the character's root.
    fn build_to_level(characters: &Path, seed: &str, class_slug: &str, want: &CharacterInput) -> std::path::PathBuf {
        let section = SECTIONS.iter().find(|(s, _)| *s == seed).expect("section").1;
        let root = characters.join(seed.to_ascii_lowercase());
        create_seed(seed, &root);
        let class = format!("core:class:{class_slug}");
        let target_level = want.chosen.class_levels.iter().find(|c| c.class_id == class).expect("seed class").level;
        let target_ranks: BTreeMap<String, i64> =
            want.chosen.skill_allocations.iter().map(|a| (a.skill_id.clone(), i64::from(a.ranks))).collect();
        let increase = seed_increase(section);
        for _ in 1..target_level {
            level_once(&root, &class, &target_ranks, &increase);
        }
        root
    }

    /// The four seeds, each created at 1st level, leveled through the level-up and given its
    /// feats, spells known and gear through this dialog's save: the six loadout hand values of
    /// each (EAC, KAC, starting/spent/remaining credits, bulk: 24 of 24) equal E0.4's, and every
    /// sheet total equals the seed's hand-valued input's (`sf_adapter::tests::seeds`, 160 of 160).
    #[test]
    fn the_seeds_given_their_loadout_through_the_dialog_have_their_hand_values() {
        let characters = tempdir("sf-choices-seeds");
        let hand = crate::sf_adapter::tests::hand_values();
        let mut checked = 0;
        let mut rows = 0;
        for (seed, class_slug, want) in crate::sf_adapter::tests::seeds() {
            let root = build_to_level(&characters, seed, class_slug, &want);
            let request = SfChoicesRequest { character_id: seed.to_ascii_lowercase(), saved_at: "2026-10-06T00:00:00Z".into(), choices: Some(seed_choices(&want)) };
            let shown = preview_at_root(package(), &root, &request).expect("previews");
            assert!(!blocked(&shown), "{seed}: {:?}", shown.problems);
            let done = save_at_root(package(), &root, &request).expect("saves");
            assert!(matches!(done, SfCreateResponse::Saved { .. }), "{seed}: {done:?}");
            let got = values(&saved(&root));
            for (field, row) in LOADOUT_FIELDS {
                let (_, _, value) = hand.iter().find(|(s, f, _)| s == seed && f == field).unwrap_or_else(|| panic!("{seed} {field} hand value"));
                assert_eq!(got.get(row).copied(), *value, "{seed}: {field}");
                let previewed = shown.totals.iter().find(|t| t.id == row).map(|t| i64::from(t.value));
                assert_eq!(previewed, *value, "{seed}: previewed {field}");
                checked += 1;
            }
            let expected = values(&want);
            assert_eq!(got, expected, "{seed}: every sheet total");
            rows += expected.len();
            // The printed lines: the seed's feats, spells and gear print as the hand-built seed's do.
            // (The creation picks -- fighting style, archetype -- print too; the hand-built seed
            // names only the records that feed a total, so the comparison is these choices' kinds.)
            let line_ids = |input: &CharacterInput| -> BTreeSet<String> {
                sf_adapter::compute_sheet(package(), input)
                    .unwrap()
                    .lines
                    .into_iter()
                    .map(|l| l.id)
                    .filter(|id| matches!(kind_of(id), "feat" | "spell" | "equipment" | "equipment_modifier"))
                    .collect()
            };
            assert_eq!(line_ids(&saved(&root)), line_ids(&want), "{seed}: printed lines");
        }
        assert_eq!(checked, 24);
        println!("sf-choices seeds: {checked} loadout hand values, {rows} sheet totals");
        std::fs::remove_dir_all(&characters).ok();
    }

    /// The feat list is the package's feats, each with the engine's prerequisite verdict for this
    /// character: Quick Draw (BAB +1) is open to a 1st-level soldier (BAB +1) and closed to a
    /// 1st-level mystic (BAB +0); Deflect Projectiles (BAB +8) is closed to both. Choosing a feat
    /// whose prerequisite is not met, or a non-repeatable feat twice, is refused by name.
    #[test]
    fn the_feat_list_carries_the_engines_prerequisite_verdict() {
        let characters = tempdir("sf-choices-feats");
        let soldier = characters.join("soldier");
        create_seed("SF-Soldier-3", &soldier);
        let mystic = characters.join("mystic");
        create_seed("SF-Mystic-5", &mystic);
        let eligible = |root: &Path, id: &str| {
            let shown = preview(package(), &saved(root));
            shown.feat_options.iter().find(|o| o.id == id).unwrap_or_else(|| panic!("{id} offered")).eligible
        };
        assert!(eligible(&soldier, "core:feat:quick_draw"));
        assert!(!eligible(&mystic, "core:feat:quick_draw"));
        assert!(!eligible(&soldier, "core:feat:deflect_projectiles"));
        let shown = preview(package(), &saved(&soldier));
        let on_disk = package().rules_of_kind("feat").filter(|r| !r.id.contains('#')).count();
        assert_eq!(shown.feat_options.len(), on_disk, "every feat record offered");
        assert!(shown.feat_rules[0].starts_with("Character feats: one at 1st level"), "{:?}", shown.feat_rules);

        let try_feats = |root: &Path, feats: &[&str]| {
            let mut choices = choices_of(&saved(root));
            choices.feats = feats.iter().map(|f| f.to_string()).collect();
            save_at_root(package(), root, &SfChoicesRequest { character_id: "x".into(), saved_at: "t".into(), choices: Some(choices) }).expect("runs")
        };
        let ids = |r: SfCreateResponse| match r {
            SfCreateResponse::Blocked { diagnostics } => diagnostics.into_iter().map(|d| d.id).collect::<Vec<_>>(),
            SfCreateResponse::Saved { .. } => vec!["saved".to_owned()],
        };
        assert_eq!(ids(try_feats(&mystic, &["core:feat:quick_draw"])), [REFUSED_FEAT_PREREQUISITE]);
        assert_eq!(ids(try_feats(&soldier, &["core:feat:quick_draw", "core:feat:quick_draw"])), [REFUSED_FEAT_TWICE]);
        assert_eq!(ids(try_feats(&soldier, &["core:feat:no_such_feat"])), [REFUSED_NOT_A_FEAT]);
        assert_eq!(saved(&soldier).chosen.selected_feats.iter().filter(|f| is_feat_id(f)).count(), 0, "a refused save saves nothing");
        assert_eq!(ids(try_feats(&soldier, &["core:feat:quick_draw", "core:feat:weapon_focus"])), ["saved"]);
        let after = saved(&soldier);
        assert!(after.chosen.selected_feats.contains(&"core:ability:mercenary".to_owned()), "the theme is kept");
        assert_eq!(choices_of(&after).feats, ["core:feat:quick_draw", "core:feat:weapon_focus"]);
        // Weapon Focus asks for its weapon type (the engine's discovery), optional.
        let shown = preview(package(), &after);
        let slot = shown.feat_slots.iter().find(|s| s.slot_id == "core:feat:weapon_focus").expect("Weapon Focus's pick");
        assert!(!slot.options.is_empty() && !slot.required);
        std::fs::remove_dir_all(&characters).ok();
    }

    /// Spells known: each castable level of each casting class with the engine's total and terms
    /// and the class's list at that level; more than the total, or a spell off the list, is
    /// refused; fewer is listed, not blocking.
    #[test]
    fn spells_known_are_offered_per_level_from_the_class_list_up_to_the_engines_total() {
        let characters = tempdir("sf-choices-spells");
        let root = characters.join("technomancer");
        create_seed("SF-Technomancer-5", &root);
        let shown = preview(package(), &saved(&root));
        let levels: Vec<(u8, i64)> = shown.spell_levels.iter().map(|l| (l.level, l.known)).collect();
        let casting = sf_spells::compute(package(), &sf_adapter::build_from_input(package(), &saved(&root)).unwrap().0).unwrap();
        let expected: Vec<(u8, i64)> =
            casting[0].levels.iter().filter(|l| l.castable()).map(|l| (l.level, l.known.as_ref().unwrap().total)).collect();
        assert_eq!(levels, expected);
        let first = shown.spell_levels.iter().find(|l| l.level == 1).expect("1st-level spells at technomancer 1");
        assert!(first.options.iter().any(|o| o.id == "core:spell:magic_missile"));
        for option in &first.options {
            assert_eq!(spell_level_on(package().rule(&option.id).unwrap(), "technomancer"), Some(1), "{}", option.id);
        }
        assert!(shown.problems.iter().any(|p| p.id == NOT_CHOSEN_YET && !p.claim_blocking), "{:?}", shown.problems);

        let with_spells = |spells: Vec<&str>| {
            let mut choices = choices_of(&saved(&root));
            choices.spells = spells
                .into_iter()
                .map(|s| SfSpellPickDto { class_id: "core:class:technomancer".into(), spell_id: s.to_owned() })
                .collect();
            preview(package(), &apply(&saved(&root), &choices))
        };
        let over: Vec<&str> = first.options.iter().take(first.known as usize + 1).map(|o| o.id.as_str()).collect();
        let shown_over = with_spells(over);
        assert!(shown_over.problems.iter().any(|p| p.id == REFUSED_SPELLS_OVER_KNOWN && p.claim_blocking), "{:?}", shown_over.problems);
        assert!(with_spells(vec!["core:spell:mystic_cure_level_1"]).problems.iter().any(|p| p.id == REFUSED_SPELL_NOT_OFFERED && p.claim_blocking));
        let ok = with_spells(vec!["core:spell:magic_missile"]);
        assert!(!blocked(&ok), "{:?}", ok.problems);
        assert_eq!(ok.spell_levels.iter().find(|l| l.level == 1).unwrap().chosen.len(), 1);
        std::fs::remove_dir_all(&characters).ok();
    }

    /// Gear: the equipment list is the package's equipment records with their own price rows; an
    /// item's upgrade list is exactly the modifiers the print path places on it; the engine's
    /// totals move with the gear, and its refusals (over budget, an upgrade that does not fit)
    /// block the save by name.
    #[test]
    fn gear_and_upgrades_come_from_the_package_and_reach_the_engines_totals() {
        let options = equipment_options(package());
        assert_eq!(options.len(), package().rules_of_kind("equipment").filter(|r| !r.id.contains('#')).count());
        let squad = options.iter().find(|o| o.id == "core:equipment:defiance_series_squad").expect("listed");
        assert_eq!(squad.price.as_deref(), Some("1220"));

        let characters = tempdir("sf-choices-gear");
        let root = characters.join("soldier");
        create_seed("SF-Soldier-3", &root);
        let base = preview(package(), &saved(&root));
        let total = |p: &SfChoicesPreviewDto, id: &str| p.totals.iter().find(|t| t.id == id).map(|t| i64::from(t.value));
        let mut choices = choices_of(&saved(&root));
        choices.gear.push(SfGearDto { item_id: "core:equipment:second_skin".into(), equipped: true, modifiers: Vec::new() });
        let worn = preview(package(), &apply(&saved(&root), &choices));
        assert!(!blocked(&worn), "{:?}", worn.problems);
        assert!(total(&worn, "sf.kac") > total(&base, "sf.kac"), "armour worn raises KAC");
        assert!(total(&worn, "sf.credits.remaining") < total(&base, "sf.credits.remaining"), "its price is spent");
        let line = &worn.gear[0];
        assert!(!line.modifier_options.is_empty());
        let item = package().rule("core:equipment:second_skin").unwrap();
        let fits: Vec<String> = package()
            .rules_of_kind("equipment_modifier")
            .filter(|m| !m.id.contains('#') && modifier_fits(item, m).is_ok())
            .map(|m| m.id.clone())
            .collect();
        assert_eq!(line.modifier_options.iter().map(|o| o.id.clone()).collect::<BTreeSet<_>>(), fits.into_iter().collect());
        assert!(line.modifier_options.iter().all(|o| o.id != "core:equipment_modifier:weapon_flaming"), "no fusion on armour");

        let mut fusion = choices.clone();
        fusion.gear[0].modifiers = vec!["core:equipment_modifier:weapon_flaming".into()];
        assert!(preview(package(), &apply(&saved(&root), &fusion)).problems.iter().any(|p| p.id == REFUSED_MODIFIER_NOT_OFFERED));
        let mut broke = choices.clone();
        broke.gear = vec![SfGearDto { item_id: "core:equipment:defiance_series_squad".into(), equipped: true, modifiers: Vec::new() }];
        let refused = save_at_root(package(), &root, &SfChoicesRequest { character_id: "x".into(), saved_at: "t".into(), choices: Some(broke) }).unwrap();
        match refused {
            SfCreateResponse::Blocked { diagnostics } => {
                assert!(diagnostics.iter().any(|d| d.id == codex::rules_core::pilot_compute::sf_loadout::REFUSED_OVER_BUDGET), "{diagnostics:?}")
            }
            other => panic!("1,220 credits of armour on 1,000: {other:?}"),
        }
        assert!(saved(&root).chosen.equipment_selections.is_empty(), "nothing saved");
        let mut unknown = choices;
        unknown.gear.push(SfGearDto { item_id: "core:feat:quick_draw".into(), equipped: false, modifiers: Vec::new() });
        assert!(preview(package(), &apply(&saved(&root), &unknown)).problems.iter().any(|p| p.id == REFUSED_NOT_EQUIPMENT));
        std::fs::remove_dir_all(&characters).ok();
    }

    /// Creation and the level-up carry the same choices: a 1st-level technomancer created with a
    /// feat, two spells and a pistol saves them; a soldier leveled to 3rd takes the feat the level
    /// owes in the level-up dialog.
    #[test]
    fn creation_and_level_up_save_the_choices_they_are_given() {
        let characters = tempdir("sf-choices-flows");
        let (_, mut request, _) =
            crate::sf_creation::tests::seed_requests().into_iter().find(|(s, _, _)| *s == "SF-Technomancer-5").expect("seed");
        request.character_id = "tech".into();
        request.choices = Some(SfChoicesDto {
            feats: vec!["core:feat:spell_penetration".into()],
            feat_picks: Vec::new(),
            spells: ["magic_missile", "detect_magic"]
                .iter()
                .map(|s| SfSpellPickDto { class_id: "core:class:technomancer".into(), spell_id: format!("core:spell:{s}") })
                .collect(),
            gear: vec![SfGearDto { item_id: "core:equipment:laser_pistol_azimuth".into(), equipped: false, modifiers: Vec::new() }],
        });
        let (shown, _) = crate::sf_creation::preview(package(), &request);
        let choices = shown.choices.expect("choices previewed at creation");
        assert_eq!(choices.feats.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["core:feat:spell_penetration"]);
        assert_eq!(choices.totals.iter().find(|t| t.id == "sf.credits.spent").map(|t| t.value), Some(350));
        let root = characters.join("tech");
        let done = crate::sf_creation::create_at_root(package(), &root, &request, "test".into()).unwrap();
        assert!(matches!(done, SfCreateResponse::Saved { .. }), "{done:?}");
        assert_eq!(choices_of(&saved(&root)), request.choices.clone().unwrap());

        let soldier = characters.join("soldier");
        create_seed("SF-Soldier-3", &soldier);
        level_once(&soldier, "core:class:soldier", &BTreeMap::new(), &[]);
        let mut level = crate::sf_level_up::SfLevelUpRequest {
            character_id: "soldier".into(),
            saved_at: "t".into(),
            class_id: Some("core:class:soldier".into()),
            ..Default::default()
        };
        let mut owed = choices_of(&saved(&soldier));
        owed.feats.push("core:feat:coordinated_shot".into());
        level.choices = Some(owed);
        let (preview3, leveled) = crate::sf_level_up::preview(package(), &saved(&soldier), &level);
        assert!(preview3.rule_lines.iter().any(|l| l.starts_with("Feat: one feat at 3rd level")));
        assert!(preview3.choices.as_ref().is_some_and(|c| c.feats.iter().any(|f| f.id == "core:feat:coordinated_shot")));
        assert!(leveled.is_some(), "{:?}", preview3.problems);
        let done = crate::sf_level_up::level_up_at_root(package(), &soldier, &level).unwrap();
        assert!(matches!(done, SfCreateResponse::Saved { .. }), "{done:?}");
        assert!(saved(&soldier).chosen.selected_feats.contains(&"core:feat:coordinated_shot".to_owned()));
        std::fs::remove_dir_all(&characters).ok();
    }
}
