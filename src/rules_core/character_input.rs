//! Character input record shape for the GE04-E1-F1 rules-core slice.
//!
//! This module represents chosen character input only. It deliberately does not
//! compute derived values, evaluate effects, or interpret formulas.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInput {
    /// Optional case identity for the chosen-input record (e.g. the GE-06
    /// deterministic pilot case). Absent for fixtures that do not name one.
    pub case_id: Option<String>,
    pub source_package_id: String,
    pub chosen: ChosenCharacterState,
    pub selection_provenance: Vec<SelectionProvenance>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChosenCharacterState {
    pub race_id: String,
    pub class_levels: Vec<CharacterClassLevel>,
    pub ability_scores: AbilityScores,
    pub selected_feats: Vec<String>,
    pub skill_allocations: Vec<SkillAllocation>,
    pub equipment_selections: Vec<EquipmentSelection>,
    pub selected_choices: Vec<SelectedChoice>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterClassLevel {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AbilityScores {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillAllocation {
    pub skill_id: String,
    pub ranks: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentSelection {
    pub item_id: String,
    /// Backward-compatible flag: true only when the selection is equipped/active.
    /// Derived from `active_state`; retained for the GE-04 record shape.
    pub equipped_or_active: bool,
    /// Chosen active state of the selection for baseline outputs. Distinguishes
    /// equipped/active, absent, and selected-but-inactive. This represents the
    /// chosen state only; it does not compute any equipment effect.
    pub active_state: ActiveState,
}

/// The chosen active state of a selection for baseline outputs. Represented only;
/// no equipment effect, encumbrance, or inventory behavior is computed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveState {
    /// Equipped/worn/primary and active for baseline outputs.
    EquippedActive,
    /// Absent / none for this slice.
    Absent,
    /// Selected but inactive for baseline outputs (e.g. Power Attack).
    SelectedInactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedChoice {
    pub choice_set_id: String,
    pub selection_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionProvenance {
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInputLoadResult {
    pub character_input: Option<CharacterInput>,
    pub diagnostics: Vec<CharacterInputDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInputDiagnostic {
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub subject_ref: String,
    pub claim_blocking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    InvalidCharacterInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
}

#[derive(Default)]
struct ParsedFixture {
    case_id: Option<String>,
    source_package_id: Option<String>,
    race_id: Option<String>,
    class_levels: Vec<CharacterClassLevel>,
    ability_scores: AbilityScores,
    ability_scores_present: AbilityScorePresence,
    selected_feats: Vec<String>,
    skill_allocations: Vec<SkillAllocation>,
    equipment_selections: Vec<EquipmentSelection>,
    selected_choices: Vec<SelectedChoice>,
    selection_provenance: Vec<SelectionProvenance>,
    diagnostics: Vec<CharacterInputDiagnostic>,
}

#[derive(Default)]
struct AbilityScorePresence {
    strength: bool,
    dexterity: bool,
    constitution: bool,
    intelligence: bool,
    wisdom: bool,
    charisma: bool,
}

pub fn load_character_input_fixture(input: &str) -> CharacterInputLoadResult {
    let mut parsed = ParsedFixture::default();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            parsed.diagnostics.push(diagnostic(
                "fixture_line",
                format!("invalid character input line missing '=': {raw_line}"),
            ));
            continue;
        };

        apply_fixture_field(key.trim(), value.trim(), &mut parsed);
    }

    parsed.add_required_field_diagnostics();

    if parsed.diagnostics.is_empty() {
        CharacterInputLoadResult {
            character_input: Some(CharacterInput {
                case_id: parsed.case_id,
                source_package_id: parsed.source_package_id.expect("validated source package"),
                chosen: ChosenCharacterState {
                    race_id: parsed.race_id.expect("validated race"),
                    class_levels: parsed.class_levels,
                    ability_scores: parsed.ability_scores,
                    selected_feats: parsed.selected_feats,
                    skill_allocations: parsed.skill_allocations,
                    equipment_selections: parsed.equipment_selections,
                    selected_choices: parsed.selected_choices,
                },
                selection_provenance: parsed.selection_provenance,
            }),
            diagnostics: Vec::new(),
        }
    } else {
        CharacterInputLoadResult {
            character_input: None,
            diagnostics: parsed.diagnostics,
        }
    }
}

fn apply_fixture_field(key: &str, value: &str, parsed: &mut ParsedFixture) {
    match key {
        "case_id" => parsed.case_id = Some(value.to_owned()),
        "source_package_id" => parsed.source_package_id = Some(value.to_owned()),
        "race_id" => parsed.race_id = Some(value.to_owned()),
        "class_level" => apply_class_level(value, parsed),
        "ability" => apply_ability_score(value, parsed),
        "feat" => parsed.selected_feats.push(value.to_owned()),
        "skill" => apply_skill_allocation(value, parsed),
        "equipment" => apply_equipment_selection(value, parsed),
        "choice" => apply_selected_choice(value, parsed),
        "provenance" => parsed.selection_provenance.push(SelectionProvenance {
            source_ref: value.to_owned(),
        }),
        unknown => parsed.diagnostics.push(diagnostic(
            unknown,
            format!("invalid character input field '{unknown}' is unsupported"),
        )),
    }
}

fn apply_class_level(value: &str, parsed: &mut ParsedFixture) {
    let Some((class_id, level_text)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(diagnostic(
            "class_levels",
            format!("invalid character input class level '{value}' is missing a level"),
        ));
        return;
    };

    match level_text.parse::<u8>() {
        Ok(level) if level > 0 => parsed.class_levels.push(CharacterClassLevel {
            class_id: class_id.to_owned(),
            level,
        }),
        _ => parsed.diagnostics.push(diagnostic(
            "class_levels",
            format!("invalid character input class level '{value}' has an invalid level"),
        )),
    }
}

fn apply_ability_score(value: &str, parsed: &mut ParsedFixture) {
    let Some((ability, score_text)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(diagnostic(
            "ability_scores",
            format!("invalid character input ability score '{value}' is missing a score"),
        ));
        return;
    };

    let Ok(score) = score_text.parse::<i16>() else {
        parsed.diagnostics.push(diagnostic(
            "ability_scores",
            format!("invalid character input ability score '{value}' has a non-numeric score"),
        ));
        return;
    };

    match ability {
        "strength" => {
            parsed.ability_scores.strength = score;
            parsed.ability_scores_present.strength = true;
        }
        "dexterity" => {
            parsed.ability_scores.dexterity = score;
            parsed.ability_scores_present.dexterity = true;
        }
        "constitution" => {
            parsed.ability_scores.constitution = score;
            parsed.ability_scores_present.constitution = true;
        }
        "intelligence" => {
            parsed.ability_scores.intelligence = score;
            parsed.ability_scores_present.intelligence = true;
        }
        "wisdom" => {
            parsed.ability_scores.wisdom = score;
            parsed.ability_scores_present.wisdom = true;
        }
        "charisma" => {
            parsed.ability_scores.charisma = score;
            parsed.ability_scores_present.charisma = true;
        }
        unknown => parsed.diagnostics.push(diagnostic(
            "ability_scores",
            format!("invalid character input ability '{unknown}' is unsupported"),
        )),
    }
}

fn apply_skill_allocation(value: &str, parsed: &mut ParsedFixture) {
    let Some((skill_id, ranks_text)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(diagnostic(
            "skill_allocations",
            format!("invalid character input skill allocation '{value}' is missing ranks"),
        ));
        return;
    };

    match ranks_text.parse::<u8>() {
        Ok(ranks) => parsed.skill_allocations.push(SkillAllocation {
            skill_id: skill_id.to_owned(),
            ranks,
        }),
        Err(_) => parsed.diagnostics.push(diagnostic(
            "skill_allocations",
            format!("invalid character input skill allocation '{value}' has non-numeric ranks"),
        )),
    }
}

fn apply_equipment_selection(value: &str, parsed: &mut ParsedFixture) {
    let Some((item_id, state)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(diagnostic(
            "equipment_selections",
            format!("invalid character input equipment selection '{value}' is missing state"),
        ));
        return;
    };

    let Some(active_state) = active_state_from_token(state) else {
        parsed.diagnostics.push(diagnostic(
            "equipment_selections",
            format!(
                "invalid character input equipment selection '{value}' has an unsupported state"
            ),
        ));
        return;
    };

    parsed.equipment_selections.push(EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: matches!(active_state, ActiveState::EquippedActive),
        active_state,
    });
}

fn active_state_from_token(state: &str) -> Option<ActiveState> {
    match state {
        "equipped" | "active" | "equipped_worn_active" | "equipped_primary_active" => {
            Some(ActiveState::EquippedActive)
        }
        "selected_inactive" => Some(ActiveState::SelectedInactive),
        "absent" => Some(ActiveState::Absent),
        _ => None,
    }
}

fn apply_selected_choice(value: &str, parsed: &mut ParsedFixture) {
    let parts: Vec<&str> = value.split(':').collect();

    if parts.len() < 4 {
        parsed.diagnostics.push(diagnostic(
            "selected_choices",
            format!("invalid character input selected choice '{value}' is incomplete"),
        ));
        return;
    }

    parsed.selected_choices.push(SelectedChoice {
        choice_set_id: parts[0..2].join(":"),
        selection_id: parts[2..].join(":"),
    });
}

impl ParsedFixture {
    fn add_required_field_diagnostics(&mut self) {
        if self.source_package_id.as_deref().is_none_or(str::is_empty) {
            self.diagnostics.push(diagnostic(
                "source_package_id",
                "missing character input source package identity",
            ));
        }

        if self.race_id.as_deref().is_none_or(str::is_empty) {
            self.diagnostics.push(diagnostic(
                "race_id",
                "missing character input race selection",
            ));
        }

        if self.class_levels.is_empty() {
            self.diagnostics.push(diagnostic(
                "class_levels",
                "missing character input class and level selection",
            ));
        }

        self.add_missing_ability_score_diagnostics();
    }

    fn add_missing_ability_score_diagnostics(&mut self) {
        let required_scores = [
            ("strength", self.ability_scores_present.strength),
            ("dexterity", self.ability_scores_present.dexterity),
            ("constitution", self.ability_scores_present.constitution),
            ("intelligence", self.ability_scores_present.intelligence),
            ("wisdom", self.ability_scores_present.wisdom),
            ("charisma", self.ability_scores_present.charisma),
        ];

        for (ability, is_present) in required_scores {
            if !is_present {
                self.diagnostics.push(diagnostic(
                    format!("ability_scores.{ability}"),
                    format!("missing character input ability score '{ability}'"),
                ));
            }
        }
    }
}

fn diagnostic(
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) -> CharacterInputDiagnostic {
    CharacterInputDiagnostic {
        class: DiagnosticClass::InvalidCharacterInput,
        severity: DiagnosticSeverity::Error,
        message: message.into(),
        subject_ref: subject_ref.into(),
        claim_blocking: true,
    }
}
