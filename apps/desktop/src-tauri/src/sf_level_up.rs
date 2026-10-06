//! Starfinder 1e level-up (SD-37 E6.5, `docs/release/SD-37-starfinder-1e/epic-breakdown.md`
//! Epic E6): a saved Starfinder character takes its next character level in a class it holds or
//! in a new class, and is saved again as the `CharacterInput` [`crate::sf_adapter`] reads, at the
//! new level.
//!
//! # What a level asks for, and where each answer comes from
//!
//! | Asked | Offered / checked by | Saved as |
//! |---|---|---|
//! | the class | every class the character holds (advance by one), then every other class record with its key-ability options ([`crate::sf_creation::class_options`]) | `class_levels` (+1, or a new class at 1); a new class's key-ability choice as `(class id, STR)` |
//! | the ability increase, at character level 5, 10, 15 and 20 | four different scores ([`sf_abilities::INCREASE_LEVELS`], [`sf_abilities::SCORES_PER_INCREASE`]); each one's step is the engine's [`sf_abilities::increase_term`] (+2, or +1 at 17 or higher) | the final `ability_scores`, raised |
//! | this level's skill ranks | the package's base skills ([`crate::sf_creation::skill_options`]); ranks in one skill at most the character level (SRD Acquiring Skills) | `skill_allocations`, added |
//! | the picks the new level opens | the held set at the new level against the held set now ([`crate::sf_creation::discover`]): an offer (a record), a skill or an ability pick whose slot is new or whose count rose | rule choices `(slot, option)` |
//!
//! Picks the held set owes that this flow does not ask (feats from a feat pool, languages, ...)
//! are listed by the same discovery as "chosen on the sheet", as at creation.
//!
//! # What the level grants
//!
//! The preview is the engine's: the character computed now and computed at the new level
//! ([`sf_adapter::compute_sheet`]), and every `sf.*` row whose value differs is listed with both
//! values (`Hit Points 11 -> 18`). The class's printed stat-block rows (`Skill ranks per level:
//! 4`) are printed as the record states them; the skill-rank budget is not computed (it is not a
//! sheet total, `decisions.md §5`). A character level that is odd prints the feat the
//! Starfinder advancement table grants at it (SRD Table 2-4), a printed line, never a number.
//!
//! A level the rules refuse (no class, an increase not chosen or chosen wrong, a skill above
//! the character level, a pick not offered, the level-20 cap, a build the engine refuses) is a
//! named, blocking diagnostic and nothing is saved.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use codex::rules_core::character_input::{AbilityScores, CharacterClassLevel, CharacterInput, SelectedChoice, SkillAllocation};
use codex::rules_core::pilot_compute::sf_abilities::{self, INCREASE_LEVELS, SCORES_PER_INCREASE};
use codex::rules_core::pilot_compute::sf_chassis::{self, SfChassisRefusal};
use codex::rules_core::pilot_compute::sf_defense;
use codex::rules_core::sheet_rule::{Ability, ProseFamily, ProsePiece, SheetRulePackage};
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::SavedCharacterEnvelope;

use crate::character_hub::{DiagnosticDto, ExplanationDto};
use crate::sf_adapter::{self, SfSheet};
use crate::sf_creation::{
    self, ability_code, parse_ability, pick_problems, Discovery, SfCreateResponse, SfCreationSlotDto, SfOptionDto,
    SfPickDto, ABILITIES,
};
use crate::sf_sheet_print::HIT_DIE_ROW;

pub const REFUSED_NO_CLASS: &str = "sf_level_up.class_not_chosen";
pub const REFUSED_CLASS_NOT_OFFERED: &str = "sf_level_up.class_not_offered";
pub const REFUSED_LEVEL_CAP: &str = "sf_level_up.level_cap";
pub const REFUSED_KEY_ABILITY: &str = "sf_level_up.key_ability";
pub const REFUSED_INCREASE_NOT_CHOSEN: &str = "sf_level_up.increase_not_chosen";
pub const REFUSED_INCREASE_NOT_FOUR_DIFFERENT: &str = "sf_level_up.increase_not_four_different";
pub const REFUSED_INCREASE_NOT_DUE: &str = "sf_level_up.increase_not_due";
pub const REFUSED_SKILL_UNKNOWN: &str = "sf_level_up.skill_unknown";
pub const REFUSED_SKILL_RANKS: &str = "sf_level_up.skill_ranks";
pub const REFUSED_NOT_COMPUTED: &str = "sf_level_up.current_character_not_computed";

/// The Starfinder level cap (SRD Table 2-4: Character Advancement runs to 20th level).
pub const MAX_CHARACTER_LEVEL: u8 = 20;

const SRD_ADVANCEMENT: &str = "SRD Table 2-4: Character Advancement (https://www.aonsrd.com/Rules.aspx?ID=56)";
const SRD_ACQUIRING_SKILLS: &str = "SRD Acquiring Skills (https://www.aonsrd.com/Rules.aspx?ID=77)";

/// Ranks added to one skill at this level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfSkillRanksDto {
    pub skill: String,
    pub ranks: i64,
}

/// The level-up dialog's state: the character and what the player has chosen for the level.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpRequest {
    #[serde(default)]
    pub character_id: String,
    #[serde(default)]
    pub saved_at: String,
    #[serde(default)]
    pub class_id: Option<String>,
    /// `STR` .. `CHA`, for a new class whose key ability is a choice.
    #[serde(default)]
    pub key_ability: Option<String>,
    /// `STR` .. `CHA`: the four scores the increase raises, when the new level has one.
    #[serde(default)]
    pub ability_increases: Vec<String>,
    #[serde(default)]
    pub picks: Vec<SfPickDto>,
    #[serde(default)]
    pub skill_ranks: Vec<SfSkillRanksDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpClassOptionDto {
    pub id: String,
    pub label: String,
    /// The class's level now; 0 for a class the character does not hold.
    pub current_level: u8,
    /// `STR` .. `CHA`. Asked only for a new class with more than one.
    pub key_ability_options: Vec<SfOptionDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpAbilityDto {
    /// `STR` .. `CHA`.
    pub ability: String,
    pub label: String,
    pub score: i64,
    /// The score after this level (the increase applied when chosen).
    pub new_score: i64,
    /// The increase's term when this score is raised (`5th level ability increase`).
    pub increase: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpSkillDto {
    /// Package skill id (`athletics`).
    pub skill: String,
    pub label: String,
    /// Ranks held now.
    pub ranks: i64,
    /// Ranks this level adds.
    pub added: i64,
    /// The most ranks one skill may hold at the new level (the character level).
    pub max_ranks: i64,
}

/// One `sf.*` row whose value the level changes; `None` where the row is absent on that side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpChangeDto {
    pub id: String,
    pub before: Option<i64>,
    pub after: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfLevelUpPreviewDto {
    pub character_level: u8,
    pub classes: Vec<SfLevelUpClassOptionDto>,
    /// `Soldier 2 — character level 2`; `None` until a class is chosen.
    pub level_line: Option<String>,
    /// The chosen class record's printed stat-block rows (`Skill ranks per level: 4`).
    pub class_lines: Vec<String>,
    /// Printed advancement rules for the new level (the odd-level feat), with their source.
    pub rule_lines: Vec<String>,
    pub increase_due: bool,
    pub abilities: Vec<SfLevelUpAbilityDto>,
    /// The picks the new level opens.
    pub slots: Vec<SfCreationSlotDto>,
    /// Picks the new level owes that this flow does not ask (feats, languages, ...).
    pub chosen_on_the_sheet: Vec<String>,
    pub skills: Vec<SfLevelUpSkillDto>,
    pub skill_rule: String,
    /// Every `sf.*` row whose value the level changes, engine values on both sides.
    pub changes: Vec<SfLevelUpChangeDto>,
    pub problems: Vec<DiagnosticDto>,
}

fn problem(id: &str, message: impl Into<String>) -> DiagnosticDto {
    DiagnosticDto { id: id.to_owned(), message: message.into(), claim_blocking: true }
}

fn refusal_problem(refusal: &SfChassisRefusal) -> DiagnosticDto {
    problem(refusal.id, refusal.message.clone())
}

fn character_level(input: &CharacterInput) -> u8 {
    input.chosen.class_levels.iter().map(|c| c.level).fold(0u8, u8::saturating_add)
}

fn finals(input: &CharacterInput) -> [i64; 6] {
    let a = &input.chosen.ability_scores;
    [a.strength, a.dexterity, a.constitution, a.intelligence, a.wisdom, a.charisma].map(i64::from)
}

fn ordinal(n: u8) -> String {
    let suffix = match (n % 10, n % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}

/// The classes offered: the held ones first (advance by one), then every other class record.
fn class_options(package: &SheetRulePackage, input: &CharacterInput) -> Vec<SfLevelUpClassOptionDto> {
    let all = sf_creation::class_options(package);
    let level_of = |id: &str| input.chosen.class_levels.iter().find(|c| c.class_id == id).map_or(0, |c| c.level);
    let mut held: Vec<SfLevelUpClassOptionDto> = Vec::new();
    let mut others: Vec<SfLevelUpClassOptionDto> = Vec::new();
    for class in all {
        let current_level = level_of(&class.id);
        let option = SfLevelUpClassOptionDto {
            id: class.id,
            label: class.label,
            current_level,
            key_ability_options: class.key_ability_options,
        };
        if current_level > 0 { held.push(option) } else { others.push(option) }
    }
    held.extend(others);
    held
}

/// The class record's printed stat-block rows, as the record states them (no hit die: a
/// Starfinder class has none, `sf_sheet_print::HIT_DIE_ROW`).
fn class_lines(package: &SheetRulePackage, class_id: &str) -> Vec<String> {
    let Some(class) = package.rule(class_id) else { return Vec::new() };
    class
        .prose
        .iter()
        .filter_map(|segment| {
            let ProseFamily::StatBlock(name) = &segment.family else { return None };
            let text: Option<String> = segment
                .pieces
                .iter()
                .map(|p| match p {
                    ProsePiece::Text(t) => Some(t.as_str()),
                    _ => None,
                })
                .collect();
            Some(format!("{name}: {}", text?.trim()))
        })
        .filter(|line| !line.starts_with(HIT_DIE_ROW))
        .collect()
}

/// What the held set of `input` asks for and owes.
fn discovery(package: &SheetRulePackage, input: &CharacterInput) -> Result<Discovery, SfChassisRefusal> {
    let (build, _) = sf_adapter::build_from_input(package, input)?;
    let sf = sf_defense::held(package, &build)?;
    let counts = sf_creation::pool_counts(package, &sf);
    Ok(sf_creation::discover(package, &sf, &counts))
}

/// The leveled input: the class level, the raised scores, the added ranks and the answered picks,
/// with every pool record the new level holds held and the package's key-ability pick answered
/// from the new class's key ability (as creation does), iterated until both settle.
fn compose(
    package: &SheetRulePackage,
    current: &CharacterInput,
    class_id: &str,
    key: Option<Ability>,
    scores: [i64; 6],
    request: &SfLevelUpRequest,
) -> Result<(CharacterInput, Discovery), SfChassisRefusal> {
    let mut input = current.clone();
    let chosen = &mut input.chosen;
    match chosen.class_levels.iter_mut().find(|c| c.class_id == class_id) {
        Some(class) => class.level += 1,
        None => {
            chosen.class_levels.push(CharacterClassLevel { class_id: class_id.to_owned(), level: 1 });
            if let Some(key) = key {
                chosen.selected_choices.push(SelectedChoice { choice_set_id: class_id.to_owned(), selection_id: ability_code(key).to_owned() });
            }
        }
    }
    let s = scores.map(|v| i16::try_from(v).unwrap_or(i16::MAX));
    chosen.ability_scores =
        AbilityScores { strength: s[0], dexterity: s[1], constitution: s[2], intelligence: s[3], wisdom: s[4], charisma: s[5] };
    for added in &request.skill_ranks {
        let ranks = u8::try_from(added.ranks).unwrap_or(u8::MAX);
        match chosen.skill_allocations.iter_mut().find(|a| a.skill_id == added.skill) {
            Some(a) => a.ranks = a.ranks.saturating_add(ranks),
            None => chosen.skill_allocations.push(SkillAllocation { skill_id: added.skill.clone(), ranks }),
        }
    }
    chosen.selected_choices.extend(
        request.picks.iter().map(|p| SelectedChoice { choice_set_id: p.slot_id.clone(), selection_id: p.option_id.clone() }),
    );
    let mut last = None;
    for _ in 0..5 {
        let found = discovery(package, &input)?;
        let mut changed = false;
        for p in &found.auto_picks {
            if !input.chosen.selected_feats.contains(p) {
                input.chosen.selected_feats.push(p.clone());
                changed = true;
            }
        }
        // A new class's own key-ability pick (`Class ~ Mystic`) answered from its key ability.
        let key_for_new_class = key.or_else(|| {
            package.rule(class_id).and_then(|c| match sf_chassis::key_ability_options(c).ok()?.as_slice() {
                [only] => Some(*only),
                _ => None,
            })
        });
        for (slot_id, options) in &found.key_ability_slots {
            let answered = input.chosen.selected_choices.iter().any(|c| &c.choice_set_id == slot_id);
            if answered {
                continue;
            }
            if let Some((_, option)) = options.iter().find(|(a, _)| Some(*a) == key_for_new_class) {
                input.chosen.selected_choices.push(SelectedChoice { choice_set_id: slot_id.clone(), selection_id: option.clone() });
                changed = true;
            }
        }
        if !changed {
            return Ok((input, found));
        }
        last = Some(found);
    }
    Ok((input, last.expect("the loop ran")))
}

/// The slots the new level opens: a slot that is new, or whose count rose (by the rise).
fn new_slots(before: &Discovery, after: &Discovery) -> Vec<SfCreationSlotDto> {
    after
        .slots
        .iter()
        .filter_map(|slot| match before.slots.iter().find(|b| b.slot_id == slot.slot_id) {
            None => Some(slot.clone()),
            Some(b) if slot.count > b.count => Some(SfCreationSlotDto { count: slot.count - b.count, ..slot.clone() }),
            Some(_) => None,
        })
        .collect()
}

fn changes(before: &SfSheet, after: &SfSheet) -> Vec<SfLevelUpChangeDto> {
    let old: std::collections::BTreeMap<String, i64> =
        before.explanations().into_iter().map(|e| (e.id, i64::from(e.value))).collect();
    let new: std::collections::BTreeMap<String, i64> =
        after.explanations().into_iter().map(|e| (e.id, i64::from(e.value))).collect();
    // Engine order: the new sheet's rows first, then any row the level removes.
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for e in after.explanations().into_iter().chain(before.explanations()) {
        if !seen.insert(e.id.clone()) {
            continue;
        }
        let (b, a) = (old.get(&e.id).copied(), new.get(&e.id).copied());
        if b != a {
            out.push(SfLevelUpChangeDto { id: e.id, before: b, after: a });
        }
    }
    out
}

/// The level-up dialog's next state for `current` (the saved character's input) and `request`,
/// and the leveled input when nothing blocks it.
pub fn preview(package: &SheetRulePackage, current: &CharacterInput, request: &SfLevelUpRequest) -> (SfLevelUpPreviewDto, Option<CharacterInput>) {
    let level = character_level(current);
    let new_level = level.saturating_add(1);
    let scores = finals(current);
    let skill_list = sf_creation::skill_options(package, &[]);
    let ranks_now = |skill: &str| {
        current.chosen.skill_allocations.iter().filter(|a| a.skill_id == skill).map(|a| i64::from(a.ranks)).sum::<i64>()
    };
    let added_to = |skill: &str| request.skill_ranks.iter().filter(|r| r.skill == skill).map(|r| r.ranks).sum::<i64>();
    let mut out = SfLevelUpPreviewDto {
        character_level: level,
        classes: class_options(package, current),
        level_line: None,
        class_lines: Vec::new(),
        rule_lines: Vec::new(),
        increase_due: INCREASE_LEVELS.contains(&new_level),
        abilities: ABILITIES
            .iter()
            .enumerate()
            .map(|(i, (_, code, name))| SfLevelUpAbilityDto {
                ability: (*code).to_owned(),
                label: (*name).to_owned(),
                score: scores[i],
                new_score: scores[i],
                increase: None,
            })
            .collect(),
        slots: Vec::new(),
        chosen_on_the_sheet: Vec::new(),
        skills: skill_list
            .iter()
            .map(|s| SfLevelUpSkillDto {
                skill: s.id.clone(),
                label: s.label.clone(),
                ranks: ranks_now(&s.id),
                added: added_to(&s.id),
                max_ranks: i64::from(new_level),
            })
            .collect(),
        skill_rule: format!("Ranks in one skill: at most the character level, {new_level} ({SRD_ACQUIRING_SKILLS})"),
        changes: Vec::new(),
        problems: Vec::new(),
    };

    let before_sheet = match sf_adapter::compute_sheet(package, current) {
        Ok(sheet) => sheet,
        Err(refusal) => {
            out.problems.push(problem(REFUSED_NOT_COMPUTED, format!("the character does not compute now: {}: {}", refusal.id, refusal.message)));
            return (out, None);
        }
    };
    if level >= MAX_CHARACTER_LEVEL {
        out.problems.push(problem(REFUSED_LEVEL_CAP, format!("character level {level}: Starfinder advancement ends at {MAX_CHARACTER_LEVEL} ({SRD_ADVANCEMENT})")));
        return (out, None);
    }
    let Some(class_id) = request.class_id.as_deref() else {
        out.problems.push(problem(REFUSED_NO_CLASS, format!("choose the class for character level {new_level}")));
        return (out, None);
    };
    let Some(class) = out.classes.iter().find(|c| c.id == class_id).cloned() else {
        out.problems.push(problem(REFUSED_CLASS_NOT_OFFERED, format!("{class_id}: not a Starfinder class this character can take")));
        return (out, None);
    };
    let key = if class.current_level == 0 && class.key_ability_options.len() > 1 {
        let chosen = request
            .key_ability
            .as_deref()
            .and_then(parse_ability)
            .filter(|a| class.key_ability_options.iter().any(|o| o.id == ability_code(*a)));
        if chosen.is_none() {
            out.problems.push(problem(
                REFUSED_KEY_ABILITY,
                format!(
                    "{}: choose the key ability ({})",
                    class.label,
                    class.key_ability_options.iter().map(|o| o.label.as_str()).collect::<Vec<_>>().join(" or ")
                ),
            ));
        }
        chosen
    } else {
        None
    };
    let class_level = class.current_level + 1;
    out.level_line = Some(format!(
        "{} {class_level}{} \u{2014} character level {new_level}",
        class.label,
        if class.current_level == 0 { " (new class)" } else { "" }
    ));
    out.class_lines = class_lines(package, class_id);
    if new_level % 2 == 1 {
        out.rule_lines.push(format!("Feat: one feat at {} level (every odd character level; {SRD_ADVANCEMENT})", ordinal(new_level)));
    }

    // The ability increase.
    let mut new_scores = scores;
    let chosen: Vec<Option<Ability>> = request.ability_increases.iter().map(|c| parse_ability(c)).collect();
    if out.increase_due {
        let valid: Vec<Ability> = chosen.iter().flatten().copied().collect();
        let distinct: BTreeSet<&str> = valid.iter().map(|a| ability_code(*a)).collect();
        if request.ability_increases.is_empty() {
            out.problems.push(problem(
                REFUSED_INCREASE_NOT_CHOSEN,
                format!("{} level raises {SCORES_PER_INCREASE} different ability scores: choose them", ordinal(new_level)),
            ));
        } else if valid.len() != chosen.len() || valid.len() != SCORES_PER_INCREASE || distinct.len() != SCORES_PER_INCREASE {
            out.problems.push(problem(
                REFUSED_INCREASE_NOT_FOUR_DIFFERENT,
                format!(
                    "the {} level increase chose {:?}: it raises {SCORES_PER_INCREASE} different scores",
                    ordinal(new_level),
                    request.ability_increases
                ),
            ));
        } else {
            for a in &valid {
                let i = ABILITIES.iter().position(|(x, _, _)| x == a).expect("one of the six");
                let term = sf_abilities::increase_term(scores[i], new_level);
                new_scores[i] = scores[i] + term.value;
                out.abilities[i].new_score = new_scores[i];
                out.abilities[i].increase = Some(term.label);
            }
        }
    } else if !request.ability_increases.is_empty() {
        out.problems.push(problem(
            REFUSED_INCREASE_NOT_DUE,
            format!("character level {new_level} has no ability increase (they come at {INCREASE_LEVELS:?})"),
        ));
    }

    // The skill ranks.
    for added in &request.skill_ranks {
        let Some(skill) = skill_list.iter().find(|s| s.id == added.skill) else {
            out.problems.push(problem(REFUSED_SKILL_UNKNOWN, format!("{}: not a Starfinder skill", added.skill)));
            continue;
        };
        if added.ranks <= 0 {
            out.problems.push(problem(REFUSED_SKILL_RANKS, format!("{}: {} ranks; a level adds ranks", skill.label, added.ranks)));
        }
    }
    for skill in &out.skills {
        if skill.added != 0 && skill.ranks + skill.added > skill.max_ranks {
            out.problems.push(problem(
                REFUSED_SKILL_RANKS,
                format!(
                    "{}: {} ranks at character level {new_level}; at most {} ({SRD_ACQUIRING_SKILLS})",
                    skill.label,
                    skill.ranks + skill.added,
                    skill.max_ranks
                ),
            ));
        }
    }

    let before = match discovery(package, current) {
        Ok(found) => found,
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return (out, None);
        }
    };
    let (leveled, after) = match compose(package, current, class_id, key, new_scores, request) {
        Ok(composed) => composed,
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return (out, None);
        }
    };
    out.slots = new_slots(&before, &after);
    out.problems.extend(pick_problems(&request.picks, &out.slots));
    let owed_before: BTreeSet<&String> = before.chosen_on_the_sheet.iter().collect();
    out.chosen_on_the_sheet = after.chosen_on_the_sheet.iter().filter(|l| !owed_before.contains(l)).cloned().collect();

    match sf_adapter::compute_sheet(package, &leveled) {
        Ok(after_sheet) => out.changes = changes(&before_sheet, &after_sheet),
        Err(refusal) => {
            out.problems.push(refusal_problem(&refusal));
            return (out, None);
        }
    }
    let blocked = out.problems.iter().any(|p| p.claim_blocking);
    (out, (!blocked).then_some(leveled))
}

/// Levels up the saved Starfinder character at `root`: saved at the new level with every
/// Starfinder total recomputed, or refused with every problem and nothing saved.
pub fn level_up_at_root(package: &SheetRulePackage, root: &Path, request: &SfLevelUpRequest) -> Result<SfCreateResponse, String> {
    let mut envelope: SavedCharacterEnvelope = sf_adapter::load_starfinder(root)?;
    let (preview, leveled) = preview(package, &envelope.character_input, request);
    let Some(leveled) = leveled else {
        return Ok(SfCreateResponse::Blocked { diagnostics: preview.problems.into_iter().filter(|p| p.claim_blocking).collect() });
    };
    let sheet = match sf_adapter::compute_sheet(package, &leveled) {
        Ok(sheet) => sheet,
        Err(refusal) => return Ok(SfCreateResponse::Blocked { diagnostics: vec![refusal_problem(&refusal)] }),
    };
    let next = sf_adapter::next_revision_id(&envelope.character_id, &envelope.revision_id);
    envelope.revision_id = next.clone();
    envelope.latest_authoritative_revision_ref = next;
    envelope.saved_at = request.saved_at.clone();
    envelope.character_input = leveled;
    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;
    Ok(SfCreateResponse::Saved {
        summary: Box::new(sf_adapter::summary_dto(&envelope)),
        explanations: sheet.explanations().into_iter().map(|e| ExplanationDto { id: e.id, value: e.value, detail: e.detail }).collect(),
    })
}

fn package_or_error() -> Result<&'static SheetRulePackage, String> {
    sf_adapter::package().map_err(|r| format!("{}: {}", r.id, r.message))
}

/// The Starfinder level-up dialog's next state: the classes, the increase, the skills, the
/// picks the level opens and every sheet total it changes.
#[tauri::command]
pub fn preview_starfinder_level_up(app: tauri::AppHandle, request: SfLevelUpRequest) -> Result<SfLevelUpPreviewDto, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    let envelope = sf_adapter::load_starfinder(&root)?;
    Ok(preview(package_or_error()?, &envelope.character_input, &request).0)
}

/// Levels up a saved Starfinder 1e character by one character level.
#[tauri::command]
pub fn level_up_starfinder_character(app: tauri::AppHandle, request: SfLevelUpRequest) -> Result<SfCreateResponse, String> {
    let root = crate::character_hub::resolve_character_root(&app, &request.character_id)?;
    level_up_at_root(package_or_error()?, &root, &request)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::sf_creation::tests::{seed_requests, tempdir};

    const SEED_BUILDS: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-builds.md";
    /// The totals a level changes that no seed's gear or feat touches: compared with the seed's
    /// own hand-valued input (`sf_adapter::tests::seeds`, 160 of 160 = `seed-hand-values.md`, E4.6).
    const LEVEL_ROWS: [&str; 13] = [
        "sf.hit_points",
        "sf.stamina",
        "sf.resolve",
        "sf.base_attack_bonus",
        "sf.fortitude",
        "sf.reflex",
        "sf.will",
        "sf.ability_score.strength",
        "sf.ability_score.dexterity",
        "sf.ability_score.constitution",
        "sf.ability_score.intelligence",
        "sf.ability_score.wisdom",
        "sf.ability_score.charisma",
    ];

    fn package() -> &'static SheetRulePackage {
        sf_adapter::package().expect("the Starfinder package loads")
    }

    /// The seed's 5th-level increase from `seed-builds.md` (the abilities whose cell in the
    /// `5th-level increase` row is not empty): bytes the engine does not read.
    fn seed_increase(section: &str) -> Vec<String> {
        let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(SEED_BUILDS)).expect("seed-builds.md");
        let body: String = text.split("\n## ").find(|s| s.starts_with(section)).expect("seed section").to_owned();
        let Some(line) = body.lines().find(|l| l.starts_with("| 5th-level increase")) else { return Vec::new() };
        line.split('|')
            .skip(2)
            .take(6)
            .zip(["STR", "DEX", "CON", "INT", "WIS", "CHA"])
            .filter(|(cell, _)| !cell.trim().is_empty())
            .map(|(_, code)| code.to_owned())
            .collect()
    }

    /// Creates `seed` at 1st level through the creation flow (`sf_creation`), at `root`.
    fn create_seed(seed: &str, root: &Path) {
        let (_, mut r, _) = seed_requests().into_iter().find(|(s, _, _)| *s == seed).expect("seed request");
        r.character_id = seed.to_ascii_lowercase();
        let created = sf_creation::create_at_root(package(), root, &r, "test".into()).expect("creates");
        assert!(matches!(created, SfCreateResponse::Saved { .. }), "{seed}: {created:?}");
    }

    fn saved(root: &Path) -> SavedCharacterEnvelope {
        SavedCharacterStore::load(root).expect("loads")
    }

    fn request(class: &str) -> SfLevelUpRequest {
        SfLevelUpRequest {
            character_id: "sf-level-up".into(),
            saved_at: "2026-10-05T00:00:00Z".into(),
            class_id: Some(class.into()),
            ..SfLevelUpRequest::default()
        }
    }

    fn values(sheet: &SfSheet) -> BTreeMap<String, i64> {
        sheet.explanations().into_iter().map(|e| (e.id, i64::from(e.value))).collect()
    }

    /// Levels the character at `root` one level in `class`, adding toward `target_ranks` as many
    /// ranks as the new level allows, with `increase` when the level has one.
    fn level_once(root: &Path, class: &str, target_ranks: &BTreeMap<String, i64>, increase: &[String]) -> SfLevelUpPreviewDto {
        let current = saved(root).character_input;
        let new_level = i64::from(character_level(&current)) + 1;
        let mut r = request(class);
        for (skill, want) in target_ranks {
            let now = current.chosen.skill_allocations.iter().filter(|a| &a.skill_id == skill).map(|a| i64::from(a.ranks)).sum::<i64>();
            let add = (*want).min(new_level) - now;
            if add > 0 {
                r.skill_ranks.push(SfSkillRanksDto { skill: skill.clone(), ranks: add });
            }
        }
        if INCREASE_LEVELS.contains(&u8::try_from(new_level).unwrap()) {
            r.ability_increases = increase.to_vec();
        }
        let (shown, _) = preview(package(), &current, &r);
        let done = level_up_at_root(package(), root, &r).expect("levels up");
        assert!(matches!(done, SfCreateResponse::Saved { .. }), "level {new_level}: {done:?} ({:?})", shown.problems);
        shown
    }

    /// The four seeds, each created at 1st level through the creation flow and leveled to its
    /// level through this level-up (skill ranks added level by level, the 5th-level increase
    /// from `seed-builds.md`): every total a level changes equals the seed's hand-valued input's
    /// (52 of 52: 4 seeds x 13 rows), and the saved ranks are the seed's.
    #[test]
    fn the_seeds_leveled_through_the_level_up_have_their_totals() {
        let characters = tempdir("sf-level-up-seeds");
        let sections = [("SF-Soldier-3", "1. SF-Soldier-3"), ("SF-Mystic-5", "2. SF-Mystic-5"), ("SF-Technomancer-5", "3. SF-Technomancer-5"), ("SF-Envoy-3", "4. SF-Envoy-3")];
        let mut checked = 0;
        for (seed, class_slug, want) in crate::sf_adapter::tests::seeds() {
            let section = sections.iter().find(|(s, _)| *s == seed).expect("section").1;
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
            let leveled = saved(&root).character_input;
            assert_eq!(character_level(&leveled), target_level, "{seed}");
            let ranks: BTreeMap<String, i64> =
                leveled.chosen.skill_allocations.iter().map(|a| (a.skill_id.clone(), i64::from(a.ranks))).collect();
            assert_eq!(ranks, target_ranks, "{seed}: saved skill ranks");
            let got = values(&sf_adapter::compute_sheet(package(), &leveled).expect("leveled computes"));
            let expected = values(&sf_adapter::compute_sheet(package(), &want).expect("seed computes"));
            for row in LEVEL_ROWS {
                assert_eq!(got.get(row), expected.get(row), "{seed}: {row}");
                assert!(got.contains_key(row), "{seed}: {row} present");
                checked += 1;
            }
        }
        assert_eq!(checked, 52);
        std::fs::remove_dir_all(&characters).ok();
    }

    /// The preview of Soldier 1 -> 2: the held class first, every other class after it; the
    /// level line; the class's printed skill-rank row and no hit die; the totals the level
    /// changes with the engine's values on both sides.
    #[test]
    fn the_preview_lists_the_class_the_level_and_every_total_it_changes() {
        let root = tempdir("sf-level-up-preview").join("soldier");
        create_seed("SF-Soldier-3", &root);
        let current = saved(&root).character_input;
        let (open, none) = preview(package(), &current, &SfLevelUpRequest::default());
        assert!(none.is_none());
        assert_eq!(open.character_level, 1);
        assert_eq!((open.classes[0].id.as_str(), open.classes[0].current_level), ("core:class:soldier", 1));
        assert!(open.classes.iter().any(|c| c.id == "core:class:mystic" && c.current_level == 0));
        assert_eq!(open.problems.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), [REFUSED_NO_CLASS]);

        let (shown, leveled) = preview(package(), &current, &request("core:class:soldier"));
        assert!(shown.problems.iter().all(|p| !p.claim_blocking), "{:?}", shown.problems);
        assert_eq!(shown.level_line.as_deref(), Some("Soldier 2 \u{2014} character level 2"));
        assert_eq!(shown.class_lines, ["Skill ranks per level: 4"]);
        assert!(!shown.increase_due);
        assert!(shown.rule_lines.is_empty(), "level 2 is even: no feat line");
        let leveled = leveled.expect("leveled input");
        let before = values(&sf_adapter::compute_sheet(package(), &current).unwrap());
        let after = values(&sf_adapter::compute_sheet(package(), &leveled).unwrap());
        let changed: Vec<&str> = shown.changes.iter().map(|c| c.id.as_str()).collect();
        for change in &shown.changes {
            assert_eq!(change.before, before.get(&change.id).copied(), "{}", change.id);
            assert_eq!(change.after, after.get(&change.id).copied(), "{}", change.id);
            assert_ne!(change.before, change.after);
        }
        for id in ["sf.hit_points", "sf.stamina", "sf.base_attack_bonus", "sf.class_level.soldier"] {
            assert!(changed.contains(&id), "{id} in {changed:?}");
        }
        let unchanged = before.iter().filter(|(id, v)| after.get(*id) == Some(*v)).count();
        assert_eq!(changed.len() + unchanged, after.len().max(before.len()), "every row is listed or unchanged");
        // The level is not saved by a preview.
        assert_eq!(character_level(&saved(&root).character_input), 1);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }

    /// A new class is offered at 1 with an odd character level's feat line; the engine's own
    /// multiclass refusal (`sf_chassis` reads no multiclass key ability, E4) is shown by name and
    /// blocks the level -- the level-up never computes around it.
    #[test]
    fn a_new_class_shows_its_level_and_the_engines_multiclass_refusal() {
        let root = tempdir("sf-level-up-multiclass").join("soldier");
        create_seed("SF-Soldier-3", &root);
        level_once(&root, "core:class:soldier", &BTreeMap::new(), &[]);
        let current = saved(&root).character_input;
        let (shown, leveled) = preview(package(), &current, &request("core:class:mystic"));
        assert_eq!(shown.level_line.as_deref(), Some("Mystic 1 (new class) \u{2014} character level 3"));
        assert_eq!(shown.rule_lines.len(), 1);
        assert!(shown.rule_lines[0].starts_with("Feat: one feat at 3rd level"), "{:?}", shown.rule_lines);
        assert!(leveled.is_none());
        assert!(
            shown.problems.iter().any(|p| p.claim_blocking && p.id == sf_chassis::REFUSED_MULTICLASS_KEY_ABILITY),
            "{:?}",
            shown.problems
        );
        // A class the package does not hold is not offered; a new class whose key ability is a
        // choice asks for it.
        let (asks, _) = preview(package(), &current, &request("core:class:soldier_x_not_a_class"));
        assert!(asks.problems.iter().any(|p| p.id == REFUSED_CLASS_NOT_OFFERED));
        let soldier_choice = shown.classes.iter().find(|c| c.id == "core:class:soldier").expect("soldier");
        assert_eq!(soldier_choice.current_level, 2);
        let fresh = tempdir("sf-level-up-key").join("envoy");
        create_seed("SF-Envoy-3", &fresh);
        let envoy = saved(&fresh).character_input;
        let two_keys = shown.classes.iter().find(|c| c.current_level == 0 && c.key_ability_options.len() > 1).expect("a class with a key-ability choice");
        let (asks, none) = preview(package(), &envoy, &request(&two_keys.id));
        assert!(none.is_none());
        assert!(asks.problems.iter().any(|p| p.id == REFUSED_KEY_ABILITY), "{}: {:?}", two_keys.id, asks.problems);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
        std::fs::remove_dir_all(fresh.parent().unwrap()).ok();
    }

    /// Every level the rules refuse is blocked by name and nothing is saved.
    #[test]
    fn a_level_the_rules_refuse_is_blocked_and_nothing_is_saved() {
        let characters = tempdir("sf-level-up-refused");
        let root = characters.join("mystic");
        create_seed("SF-Mystic-5", &root);
        for _ in 1..4 {
            level_once(&root, "core:class:mystic", &BTreeMap::new(), &[]);
        }
        let revision = saved(&root).revision_id;
        let blocked = |r: &SfLevelUpRequest, id: &str| {
            let done = level_up_at_root(package(), &root, r).expect("answers");
            let SfCreateResponse::Blocked { diagnostics } = done else { panic!("{id}: {done:?}") };
            assert!(diagnostics.iter().any(|d| d.id == id), "{id}: {diagnostics:?}");
            assert_eq!(saved(&root).revision_id, revision, "{id}: nothing saved");
        };
        let mystic = request("core:class:mystic");
        blocked(&SfLevelUpRequest { class_id: None, ..mystic.clone() }, REFUSED_NO_CLASS);
        blocked(&request("core:class:fighter"), REFUSED_CLASS_NOT_OFFERED);
        // Level 5 has the increase: none, three, or one score twice is refused.
        blocked(&mystic, REFUSED_INCREASE_NOT_CHOSEN);
        let three = ["DEX", "INT", "WIS"].map(str::to_owned).to_vec();
        blocked(&SfLevelUpRequest { ability_increases: three, ..mystic.clone() }, REFUSED_INCREASE_NOT_FOUR_DIFFERENT);
        let twice = ["DEX", "DEX", "WIS", "CHA"].map(str::to_owned).to_vec();
        blocked(&SfLevelUpRequest { ability_increases: twice, ..mystic.clone() }, REFUSED_INCREASE_NOT_FOUR_DIFFERENT);
        let four = ["DEX", "INT", "WIS", "CHA"].map(str::to_owned).to_vec();
        // Ranks above the new character level (5), and a skill the package does not hold.
        let six = vec![SfSkillRanksDto { skill: "bluff".into(), ranks: 6 }];
        blocked(&SfLevelUpRequest { ability_increases: four.clone(), skill_ranks: six, ..mystic.clone() }, REFUSED_SKILL_RANKS);
        let unknown = vec![SfSkillRanksDto { skill: "spellcraft".into(), ranks: 1 }];
        blocked(&SfLevelUpRequest { ability_increases: four.clone(), skill_ranks: unknown, ..mystic.clone() }, REFUSED_SKILL_UNKNOWN);
        // The increase at a level that has none.
        let lower = characters.join("soldier");
        create_seed("SF-Soldier-3", &lower);
        let done = level_up_at_root(package(), &lower, &SfLevelUpRequest { ability_increases: four.clone(), ..request("core:class:soldier") }).unwrap();
        assert!(matches!(&done, SfCreateResponse::Blocked { diagnostics } if diagnostics.iter().any(|d| d.id == REFUSED_INCREASE_NOT_DUE)), "{done:?}");
        // The increase raises each chosen score by the engine's step: Wis 18 -> 19 (17 or higher), the others +2.
        let (shown, _) = preview(package(), &saved(&root).character_input, &SfLevelUpRequest { ability_increases: four, ..mystic.clone() });
        let wis = shown.abilities.iter().find(|a| a.ability == "WIS").unwrap();
        assert_eq!((wis.score, wis.new_score), (18, 19));
        assert_eq!(wis.increase.as_deref(), Some("5th level ability increase (+1: score 17 or higher)"));
        let dex = shown.abilities.iter().find(|a| a.ability == "DEX").unwrap();
        assert_eq!((dex.score, dex.new_score), (12, 14));
        // A Pathfinder save is not leveled here.
        let pf = characters.join("pf");
        let mut envelope = saved(&root);
        envelope.game_system = "pf1".into();
        SavedCharacterStore::save(&envelope, &pf).unwrap();
        let err = level_up_at_root(package(), &pf, &mystic).unwrap_err();
        assert!(err.starts_with(sf_adapter::REFUSED_NOT_STARFINDER), "{err}");
        std::fs::remove_dir_all(&characters).ok();
    }

    /// Character level 20 is the cap.
    #[test]
    fn character_level_20_is_the_cap() {
        let root = tempdir("sf-level-up-cap").join("envoy");
        create_seed("SF-Envoy-3", &root);
        let mut current = saved(&root).character_input;
        current.chosen.class_levels[0].level = MAX_CHARACTER_LEVEL;
        let (shown, none) = preview(package(), &current, &request("core:class:envoy"));
        assert!(none.is_none());
        assert!(shown.problems.iter().any(|p| p.id == REFUSED_LEVEL_CAP), "{:?}", shown.problems);
        std::fs::remove_dir_all(root.parent().unwrap()).ok();
    }
}
