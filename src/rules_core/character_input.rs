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
    /// Character trait/drawback selections (AT-34-E4-002). A flat compound-
    /// string id (`"trait:trait_acrobat"`, matching the corpus filename
    /// slug -- the same idiom `selected_feats`' `"feat:weapon_focus"` uses),
    /// never a per-book enum. Before this field existed, no character
    /// trait/drawback selection surface existed anywhere in this crate --
    /// confirmed by a whole-tree grep for
    /// `selected_traits|character_traits|CharacterTrait\b` returning zero
    /// matches (`AT-34-E4-002_cycle_receipt_3.md`, folded at `782584b4b3`).
    /// `trait_pool.rs`'s existing `RaceTrait` machinery is a different
    /// mechanic entirely (an Adopted-Race option list; "nothing is
    /// computed... this loader only indexes them", its own doc comment) and
    /// is untouched by this field. Real effects are read through four
    /// `trait_effects` compute paths (`skill_bonuses_from_traits`,
    /// `skill_choice_bonuses_from_traits`,
    /// `family_choice_bonuses_from_traits`, `save_bonuses_from_traits`),
    /// currently covering 42-of-59 `ultimate_campaign` `trait_content`
    /// records: 40 whose corpus `BONUS` token is a flat, fixed-choice, or
    /// open-family-choice named-skill `SKILL` bonus, plus 2 whose token
    /// is a flat, named-save `SAVE` bonus (see that module's own doc
    /// comment for the exact shapes and what is deliberately not yet
    /// covered). Defaults to empty via every fixture
    /// that omits `trait=` lines and every pre-existing construction site --
    /// no pre-existing fixture or call site is broken by this addition.
    pub selected_traits: Vec<String>,
    /// Spells this character knows, has prepared, or has been granted.
    /// NEW (SD-19). Defaults to empty via fixtures that omit `spell=`
    /// lines — every pre-SD-19 fixture and construction site keeps
    /// compiling and passing unmodified.
    pub spells_selected: Vec<SpellSelection>,
    /// Combat-time class-ability activations declared for this specific
    /// computed snapshot (e.g. "is the barbarian currently raging").
    /// v0.6 alpha swarm, risks item 8 (combat-time activation state
    /// scoping). Defaults to empty via fixtures that omit `activation=`
    /// lines — every pre-existing fixture and construction site keeps
    /// compiling and passing unmodified.
    pub class_ability_activations: Vec<ClassAbilityActivation>,
}

/// One class ability's declared activation state and rounds-per-day
/// consumption for this specific computed snapshot (e.g. Barbarian Rage,
/// Bard Bardic Performance). Represented only; no ability effect is
/// computed here -- mirrors `EquipmentSelection`'s own
/// representation-only shape exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassAbilityActivation {
    /// e.g. "rage", "bardic_performance" -- the same flat compound-string
    /// id idiom as `feat:weapon_focus:weapon:longsword` elsewhere in this
    /// module, not a per-class enum. Any pillar that reads this field for
    /// a given `ability_id` must first confirm the character's own
    /// `class_levels` actually contains that ability's owning class
    /// before applying anything -- this type carries no such validation
    /// itself, the same way `SpellSelection.source_class_id` is a bare,
    /// unvalidated string here too.
    pub ability_id: String,
    /// Reuses `ActiveState` directly (the same enum `EquipmentSelection`
    /// uses) rather than introducing a class-ability-specific duplicate:
    /// `EquippedActive` means "active for this computed snapshot",
    /// `Absent` means no activation is declared at all, and
    /// `SelectedInactive` means chosen/available but not active this
    /// snapshot (mirrors Power Attack's own existing use of this variant).
    pub active_state: ActiveState,
    /// Rounds of the ability's own already-grounded rounds-per-day budget
    /// consumed so far today. `None` for abilities with no per-day budget
    /// (there are none among Rage/Bardic Performance, but this field
    /// should not assume every future ability has one).
    pub rounds_consumed_today: Option<u16>,
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
    selected_traits: Vec<String>,
    spells_selected: Vec<SpellSelection>,
    class_ability_activations: Vec<ClassAbilityActivation>,
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
                    selected_traits: parsed.selected_traits,
                    spells_selected: parsed.spells_selected,
                    class_ability_activations: parsed.class_ability_activations,
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
        "trait" => parsed.selected_traits.push(value.to_owned()),
        "skill" => apply_skill_allocation(value, parsed),
        "equipment" => apply_equipment_selection(value, parsed),
        "equipment_modifier" => apply_equipment_modifier(value, parsed),
        "choice" => apply_selected_choice(value, parsed),
        "spell" => apply_spell_selection(value, parsed),
        "activation" => apply_class_ability_activation(value, parsed),
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

/// Parses `activation=<ability_id>:<state>[:<rounds>]`. Unlike
/// `apply_spell_selection`'s edge-parse (whose middle segment,
/// `source_class_id`, conventionally contains its own colon), `ability_id`
/// values ("rage", "bardic_performance") never do, so a plain
/// front-to-back split is unambiguous here.
fn apply_class_ability_activation(value: &str, parsed: &mut ParsedFixture) {
    let malformed = || {
        diagnostic(
            "class_ability_activations",
            format!(
                "invalid character input activation '{value}' must have at least 2 \
                 colon-separated parts (ability_id:state[:rounds_consumed_today])"
            ),
        )
    };

    let mut parts = value.splitn(3, ':');
    let Some(ability_id) = parts.next().filter(|s| !s.is_empty()) else {
        parsed.diagnostics.push(malformed());
        return;
    };
    let Some(state_token) = parts.next() else {
        parsed.diagnostics.push(malformed());
        return;
    };
    let Some(active_state) = active_state_from_token(state_token) else {
        parsed.diagnostics.push(diagnostic(
            "class_ability_activations",
            format!(
                "invalid character input activation '{value}' has an unsupported active state"
            ),
        ));
        return;
    };

    let rounds_consumed_today = match parts.next() {
        None => None,
        Some(rounds_text) => match rounds_text.parse::<u16>() {
            Ok(rounds) => Some(rounds),
            Err(_) => {
                parsed.diagnostics.push(diagnostic(
                    "class_ability_activations",
                    format!(
                        "invalid character input activation '{value}' has a non-numeric \
                         rounds_consumed_today"
                    ),
                ));
                return;
            }
        },
    };

    parsed.class_ability_activations.push(ClassAbilityActivation {
        ability_id: ability_id.to_owned(),
        active_state,
        rounds_consumed_today,
    });
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

#[cfg(test)]
mod selected_traits_tests {
    use super::*;

    const FIXTURE: &str = "\
case_id=case:test
source_package_id=ultimate_campaign
race_id=race:human
class_level=class:fighter:1
ability=strength:16
ability=dexterity:14
ability=constitution:14
ability=intelligence:10
ability=wisdom:12
ability=charisma:8
";

    /// A fixture with no `trait=` lines at all must default to a real,
    /// empty `selected_traits` -- not an absent field, since the type has
    /// no `Option` to be absent from. Proves every pre-existing fixture
    /// keeps compiling and passing unmodified (AT-34-E4-002).
    #[test]
    fn fixture_with_no_trait_lines_defaults_to_empty() {
        let result = load_character_input_fixture(FIXTURE);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert!(input.chosen.selected_traits.is_empty());
    }

    /// The core case: `trait=trait:trait_acrobat` parses to a real entry in
    /// `selected_traits`, verbatim.
    #[test]
    fn trait_line_parses_into_selected_traits() {
        let fixture = format!("{FIXTURE}trait=trait:trait_acrobat\n");
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.selected_traits, vec!["trait:trait_acrobat".to_string()]);
    }

    /// Two `trait=` lines both land, in order -- proves the field is a real
    /// accumulating list, not a single slot (a PF1 character normally takes
    /// two traits at creation).
    #[test]
    fn multiple_trait_lines_all_land_in_order() {
        let fixture = format!(
            "{FIXTURE}trait=trait:trait_acrobat\ntrait=trait:trait_ease_of_faith\n"
        );
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(
            input.chosen.selected_traits,
            vec!["trait:trait_acrobat".to_string(), "trait:trait_ease_of_faith".to_string()]
        );
    }
}

#[cfg(test)]
mod class_ability_activation_tests {
    use super::*;

    const FIXTURE: &str = "\
case_id=case:test
source_package_id=core_rulebook
race_id=race:human
class_level=class:barbarian:1
ability=strength:16
ability=dexterity:14
ability=constitution:14
ability=intelligence:10
ability=wisdom:12
ability=charisma:8
";

    /// A fixture with no `activation=` lines at all must default to a
    /// real, empty `class_ability_activations` -- not an absent field,
    /// since the type has no `Option` to be absent from. Proves every
    /// pre-existing fixture keeps compiling and passing unmodified.
    #[test]
    fn fixture_with_no_activation_lines_defaults_to_empty() {
        let result = load_character_input_fixture(FIXTURE);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert!(input.chosen.class_ability_activations.is_empty());
    }

    /// The core case: `activation=rage:active` parses to a real
    /// `ClassAbilityActivation` with `EquippedActive` and no rounds
    /// consumed.
    #[test]
    fn activation_line_with_no_rounds_parses_active_state_only() {
        let fixture = format!("{FIXTURE}activation=rage:active\n");
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.class_ability_activations.len(), 1);
        let activation = &input.chosen.class_ability_activations[0];
        assert_eq!(activation.ability_id, "rage");
        assert_eq!(activation.active_state, ActiveState::EquippedActive);
        assert_eq!(activation.rounds_consumed_today, None);
    }

    /// The third, optional segment: `activation=rage:active:3` carries a
    /// real rounds-consumed count.
    #[test]
    fn activation_line_with_rounds_parses_the_consumed_count() {
        let fixture = format!("{FIXTURE}activation=rage:active:3\n");
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.class_ability_activations[0].rounds_consumed_today, Some(3));
    }

    /// `selected_inactive` and `absent` both parse too -- the same three
    /// `ActiveState` variants `EquipmentSelection` already uses.
    #[test]
    fn activation_line_supports_all_three_active_states() {
        let fixture = format!(
            "{FIXTURE}\
activation=rage:selected_inactive\n\
activation=bardic_performance:absent\n"
        );
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.class_ability_activations.len(), 2);
        assert_eq!(
            input.chosen.class_ability_activations[0].active_state,
            ActiveState::SelectedInactive
        );
        assert_eq!(
            input.chosen.class_ability_activations[1].active_state,
            ActiveState::Absent
        );
    }

    /// Two activations both land, in order -- proves the field is a real
    /// accumulating list, not a single slot (e.g. a Barbarian/Bard
    /// multiclass with both abilities active).
    #[test]
    fn multiple_activation_lines_all_land_in_order() {
        let fixture = format!(
            "{FIXTURE}\
activation=rage:active:2\n\
activation=bardic_performance:active:1\n"
        );
        let result = load_character_input_fixture(&fixture);
        let input = result.character_input.expect("fixture must parse cleanly");

        assert_eq!(input.chosen.class_ability_activations.len(), 2);
        assert_eq!(input.chosen.class_ability_activations[0].ability_id, "rage");
        assert_eq!(
            input.chosen.class_ability_activations[1].ability_id,
            "bardic_performance"
        );
    }

    /// An unsupported active-state token is a real fixture-authoring
    /// error and must surface a diagnostic, not silently do nothing or
    /// default to some fallback state.
    #[test]
    fn activation_line_with_an_unsupported_state_is_a_diagnostic() {
        let fixture = format!("{FIXTURE}activation=rage:frenzied\n");
        let result = load_character_input_fixture(&fixture);

        assert!(result.character_input.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.subject_ref == "class_ability_activations"),
            "expected a diagnostic for the unsupported active state: {:?}",
            result.diagnostics
        );
    }

    /// A missing state segment (`activation=rage` alone) is malformed and
    /// must surface a diagnostic.
    #[test]
    fn activation_line_missing_the_state_segment_is_a_diagnostic() {
        let fixture = format!("{FIXTURE}activation=rage\n");
        let result = load_character_input_fixture(&fixture);

        assert!(result.character_input.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.subject_ref == "class_ability_activations"),
            "expected a diagnostic for the missing state segment: {:?}",
            result.diagnostics
        );
    }

    /// A non-numeric rounds_consumed_today segment is malformed and must
    /// surface a diagnostic, not silently drop the segment or default to
    /// 0.
    #[test]
    fn activation_line_with_non_numeric_rounds_is_a_diagnostic() {
        let fixture = format!("{FIXTURE}activation=rage:active:many\n");
        let result = load_character_input_fixture(&fixture);

        assert!(result.character_input.is_none());
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.subject_ref == "class_ability_activations"),
            "expected a diagnostic for the non-numeric rounds_consumed_today: {:?}",
            result.diagnostics
        );
    }
}
