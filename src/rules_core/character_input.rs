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
    /// Spells this character knows, has prepared, or has been granted.
    /// NEW (SD-19). Defaults to empty via fixtures that omit `spell=`
    /// lines — every pre-SD-19 fixture and construction site keeps
    /// compiling and passing unmodified.
    pub spells_selected: Vec<SpellSelection>,
}

/// One spell a class knows, has prepared, or has been granted. SD-19.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellSelection {
    /// The corpus identity of this spell (spell `name`, since the PF1
    /// spell corpus carries no separate `KEY:` token — see
    /// `rules_tables::crb::spell_list`'s doc comment).
    pub spell_id: String,
    /// The class that provides this spell. Mirrors `CharacterClassLevel.class_id`
    /// (a plain string, not a typed enum) for consistency with the rest of
    /// this module's identity fields.
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionMode,
}

/// How a selected spell was acquired. Not yet consumed by the corpus-aware
/// seam (slot math is out of scope for SD-19); present at the type level so
/// a future slice can consume it without another `CharacterInput` change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquisitionMode {
    /// Spontaneous caster knows the spell; no preparation needed.
    Known,
    /// Prepared caster has prepared this specific spell in a slot today.
    Prepared,
    /// Granted by a class feature, domain, or other non-standard source.
    Granted,
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
    /// item_ids of equipmods-category items applied to this specific selection
    /// (e.g. "Special Ability ~ +1 ~ Weapon" on a Longsword selection), mirroring
    /// PCGen's own single-entry `CUSTOMIZATION:EQMOD=` convention: an applied
    /// equipmod has no separate top-level `equipment_selections` entry of its
    /// own -- it lives on the weapon/armor selection it modifies.
    pub applied_modifiers: Vec<String>,
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
    spells_selected: Vec<SpellSelection>,
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
                    spells_selected: parsed.spells_selected,
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
        "equipment_modifier" => apply_equipment_modifier(value, parsed),
        "choice" => apply_selected_choice(value, parsed),
        "spell" => apply_spell_selection(value, parsed),
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
        applied_modifiers: Vec::new(),
    });
}

fn apply_equipment_modifier(value: &str, parsed: &mut ParsedFixture) {
    // Same last-colon convention as `apply_equipment_selection`: item_id may
    // contain its own colon (e.g. "item:longsword"), the modifier item_id does
    // not, so the boundary is the final colon in the value.
    let Some((item_id, modifier_item_id)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(diagnostic(
            "equipment_selections",
            format!(
                "invalid character input equipment modifier '{value}' is missing a modifier item id"
            ),
        ));
        return;
    };

    let Some(selection) = parsed
        .equipment_selections
        .iter_mut()
        .find(|selection| selection.item_id == item_id)
    else {
        parsed.diagnostics.push(diagnostic(
            "equipment_selections",
            format!(
                "invalid character input equipment modifier '{value}' has no matching \
                 equipment selection for '{item_id}' -- the 'equipment_modifier' line must \
                 come after its 'equipment' line"
            ),
        ));
        return;
    };

    selection.applied_modifiers.push(modifier_item_id.to_owned());
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

fn apply_spell_selection(value: &str, parsed: &mut ParsedFixture) {
    // source_class_id conventionally contains its own colon (e.g.
    // "class:demo", mirroring "race:human"/"item:longsword" elsewhere in
    // this fixture grammar), so this can't be a flat 3-way split. Parse
    // from the edges instead: acquisition_mode is the last segment,
    // spell_id is the first segment, and everything between is
    // source_class_id verbatim.
    let malformed = || {
        diagnostic(
            "spells_selected",
            format!(
                "invalid character input spell selection '{value}' must have at least 3 \
                 colon-separated parts (spell_id:source_class_id:acquisition_mode)"
            ),
        )
    };

    let Some((rest, mode_token)) = value.rsplit_once(':') else {
        parsed.diagnostics.push(malformed());
        return;
    };
    let Some((spell_id, source_class_id)) = rest.split_once(':') else {
        parsed.diagnostics.push(malformed());
        return;
    };

    let Some(acquisition_mode) = acquisition_mode_from_token(mode_token) else {
        parsed.diagnostics.push(diagnostic(
            "spells_selected",
            format!(
                "invalid character input spell selection '{value}' has an unsupported \
                 acquisition mode"
            ),
        ));
        return;
    };

    parsed.spells_selected.push(SpellSelection {
        spell_id: spell_id.to_owned(),
        source_class_id: source_class_id.to_owned(),
        acquisition_mode,
    });
}

fn acquisition_mode_from_token(token: &str) -> Option<AcquisitionMode> {
    match token {
        "known" => Some(AcquisitionMode::Known),
        "prepared" => Some(AcquisitionMode::Prepared),
        "granted" => Some(AcquisitionMode::Granted),
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

#[cfg(test)]
mod applied_modifiers_tests {
    use super::*;

    const FIXTURE: &str = "\
case_id=case:test
source_package_id=core_rulebook
race_id=race:human
class_level=class:fighter:1
ability=strength:16
ability=dexterity:14
ability=constitution:14
ability=intelligence:10
ability=wisdom:12
ability=charisma:8
equipment=item:longsword:equipped
";

    /// An existing `equipment=` fixture line with no accompanying
    /// `equipment_modifier=` line must produce a real, empty
    /// `applied_modifiers` -- not an absent field, since the type has no
    /// `Option` to be absent from.
    #[test]
    fn equipment_selection_defaults_to_no_applied_modifiers() {
        let result = load_character_input_fixture(FIXTURE);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.equipment_selections.len(), 1);
        assert!(input.chosen.equipment_selections[0].applied_modifiers.is_empty());
    }

    /// The core case: an `equipment_modifier=` line attaches its modifier
    /// item id onto the matching `equipment=` selection's
    /// `applied_modifiers`, keyed by `item_id`. Uses an item_id that itself
    /// contains a colon (`item:longsword`) to prove the last-colon split
    /// correctly isolates the modifier id, not a substring of the item id.
    #[test]
    fn equipment_modifier_line_attaches_to_the_matching_equipment_selection() {
        let fixture = format!(
            "{FIXTURE}equipment_modifier=item:longsword:Special Ability ~ +1 ~ Weapon\n"
        );
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.equipment_selections.len(), 1);
        assert_eq!(
            input.chosen.equipment_selections[0].applied_modifiers,
            vec!["Special Ability ~ +1 ~ Weapon".to_string()]
        );
    }

    /// Two modifiers on the same selection must both land, in order --
    /// proves the field is a real accumulating list, not a single slot.
    #[test]
    fn multiple_equipment_modifier_lines_all_attach_to_the_same_selection() {
        let fixture = format!(
            "{FIXTURE}\
equipment_modifier=item:longsword:Special Ability ~ +1 ~ Weapon\n\
equipment_modifier=item:longsword:Special Ability ~ Flaming ~ Weapon\n"
        );
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(
            input.chosen.equipment_selections[0].applied_modifiers,
            vec![
                "Special Ability ~ +1 ~ Weapon".to_string(),
                "Special Ability ~ Flaming ~ Weapon".to_string(),
            ]
        );
    }

    /// An `equipment_modifier=` line naming an item_id with no matching
    /// prior `equipment=` line is a real fixture-authoring error and must
    /// surface a diagnostic, not silently do nothing.
    #[test]
    fn equipment_modifier_line_with_no_matching_selection_is_a_diagnostic() {
        let fixture = format!("{FIXTURE}equipment_modifier=item:shield:Masterwork\n");
        let result = load_character_input_fixture(&fixture);

        assert!(result.character_input.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.subject_ref == "equipment_selections"),
            "expected a diagnostic for the unmatched equipment_modifier line: {:?}",
            result.diagnostics
        );
    }
}
