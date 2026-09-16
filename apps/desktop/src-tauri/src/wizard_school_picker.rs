//! v0.8 B-11 -- `list_wizard_school_options`: the arcane schools a Wizard
//! can specialise in, as the ids the engine reads from
//! `choice:wizard_school_specialization` / `choice:wizard_opposed_schools`,
//! each carrying whether the engine can actually SAVE a character built on
//! it.
//!
//! The option universe is the engine's own pub table
//! (`class_feature_pool_catalog::WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER`, nine
//! `"<School> Wizard Spells"` keys). The support fact is NOT a list kept
//! here: it is obtained by running the real compute path a save runs
//! (`compose_character_input` -> `resolve_unified_pilot_snapshot`) on a
//! level-1 Wizard for every specialty + opposed-pair combination, and
//! reporting which combinations reach `Computed`. An unsupported school
//! carries the engine's own claim-blocking diagnostic as its reason, so the
//! picker's greyed explanation is the engine's sentence, not a paraphrase.
//! B-7's audit (`docs/release/v0.8/choice-pool-resolver-audit.md`) found the
//! engine grounds several schools' powers but the prepared-spellbook gate
//! (`unmet_wizard_spellbook_conditions`) accepts exactly one triple; this
//! command reports whatever that gate accepts today and will widen by
//! itself when the engine does.

use std::sync::OnceLock;

use serde::Serialize;

use codex::rules_core::class_feature_pool_catalog::WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER;

use crate::character_hub::{
    compose_character_input, resolve_unified_pilot_snapshot, AbilityScoresDto,
    CreateCharacterRequest, SelectedChoiceDto,
};
use crate::corpus_fixtures::corpus_fixture_bundle;

/// The choice set a Wizard's specialty school is recorded under -- the
/// same literal `compose_character_input` seeds and the engine reads.
pub const WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID: &str = "choice:wizard_school_specialization";
/// The choice set each opposed school is recorded under (two entries for
/// a specialist, none for a universalist).
pub const WIZARD_OPPOSED_SCHOOLS_CHOICE_ID: &str = "choice:wizard_opposed_schools";
const UNIVERSAL_SCHOOL_ID: &str = "school:universal";
const KEY_SUFFIX: &str = " Wizard Spells";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WizardSchoolOptionDto {
    /// `school:<slug>` -- exactly the `selection_id` to record under
    /// [`WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID`].
    pub id: String,
    pub name: String,
    pub choice_set_id: String,
    pub opposed_choice_set_id: String,
    /// `true` when at least one opposed-school combination reaches
    /// `Computed` through the real save path. A `false` school makes the
    /// character unsaveable today; grey it with `reason`.
    pub supported: bool,
    /// Every exact opposed-school id list the engine computes for this
    /// specialty (each is the full `selection_id` set to record under
    /// [`WIZARD_OPPOSED_SCHOOLS_CHOICE_ID`]; empty inner list = universalist
    /// with no opposed schools). Only these combinations save -- there is
    /// no free choice of opposed pair beyond what is listed.
    pub supported_opposed_pairs: Vec<Vec<String>>,
    /// For an unsupported school: the engine's own claim-blocking
    /// diagnostic message, verbatim. `None` when supported.
    pub reason: Option<String>,
    /// The id of that diagnostic, so the UI can key on it.
    pub blocking_diagnostic_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WizardSchoolOptionsResponse {
    pub options: Vec<WizardSchoolOptionDto>,
}

fn school_id(name: &str) -> String {
    format!("school:{}", name.to_ascii_lowercase())
}

/// A level-1 Human Wizard carrying exactly the given school choices --
/// the probe character every support fact is measured on.
fn probe_request(specialty: &str, opposed: &[String]) -> CreateCharacterRequest {
    let mut additional_choices = vec![SelectedChoiceDto {
        choice_set_id: WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID.to_owned(),
        selection_id: specialty.to_owned(),
    }];
    for school in opposed {
        additional_choices.push(SelectedChoiceDto {
            choice_set_id: WIZARD_OPPOSED_SCHOOLS_CHOICE_ID.to_owned(),
            selection_id: school.clone(),
        });
    }
    CreateCharacterRequest {
        character_id: "wizard-school-probe".to_owned(),
        display_label: "Wizard school probe".to_owned(),
        race_id: "race:human".to_owned(),
        class_id: "class:wizard".to_owned(),
        level: 1,
        ability_scores: AbilityScoresDto {
            strength: 10,
            dexterity: 14,
            constitution: 12,
            intelligence: 16,
            wisdom: 12,
            charisma: 8,
        },
        ability_bonus_target: "intelligence".to_owned(),
        selected_alternate_trait_keys: Vec::new(),
        companion_species: None,
        selected_traits: Vec::new(),
        trait_skill_choices: Vec::new(),
        additional_choices,
        saved_at: "2026-09-01T00:00:00Z".to_owned(),
    }
}

/// `Ok(())` when the combination computes; `Err((id, message))` with the
/// engine's own claim-blocking diagnostic (the prepared-spellbook one
/// when present, since that is the gate that decides school support).
fn probe(specialty: &str, opposed: &[String]) -> Result<(), (String, String)> {
    let input = compose_character_input(&probe_request(specialty, opposed));
    match resolve_unified_pilot_snapshot(&input, corpus_fixture_bundle()) {
        Ok(_) => Ok(()),
        Err(diagnostics) => {
            let blocking: Vec<_> = diagnostics.iter().filter(|d| d.claim_blocking).collect();
            let chosen = blocking
                .iter()
                .find(|d| d.id == "class_spell.wizard.prepared_spellbook.unsupported")
                .or_else(|| blocking.first())
                .map(|d| (d.id.clone(), d.message.clone()))
                .unwrap_or_else(|| {
                    (
                        "unknown".to_owned(),
                        "the engine reported Blocked with no claim-blocking diagnostic".to_owned(),
                    )
                });
            Err(chosen)
        }
    }
}

fn build_options() -> Vec<WizardSchoolOptionDto> {
    let names: Vec<&str> = WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER
        .iter()
        .filter_map(|(key, _)| key.strip_suffix(KEY_SUFFIX))
        .collect();
    let ids: Vec<String> = names.iter().map(|n| school_id(n)).collect();
    let opposable: Vec<&String> = ids.iter().filter(|id| id.as_str() != UNIVERSAL_SCHOOL_ID).collect();

    names
        .iter()
        .zip(ids.iter())
        .map(|(name, id)| {
            // The candidate opposed-school lists for this specialty: none for
            // a universalist; every unordered pair of the other schools for
            // a specialist. The engine, not this module, decides which
            // compute.
            let candidates: Vec<Vec<String>> = if id == UNIVERSAL_SCHOOL_ID {
                vec![Vec::new()]
            } else {
                let others: Vec<&String> = opposable.iter().copied().filter(|o| *o != id).collect();
                let mut pairs = Vec::new();
                for (i, a) in others.iter().enumerate() {
                    for b in &others[i + 1..] {
                        pairs.push(vec![(*a).clone(), (*b).clone()]);
                    }
                }
                pairs
            };
            let mut supported_opposed_pairs = Vec::new();
            let mut first_failure: Option<(String, String)> = None;
            for candidate in candidates {
                match probe(id, &candidate) {
                    Ok(()) => supported_opposed_pairs.push(candidate),
                    Err(failure) => {
                        first_failure.get_or_insert(failure);
                    }
                }
            }
            let supported = !supported_opposed_pairs.is_empty();
            let (blocking_diagnostic_id, reason) = if supported {
                (None, None)
            } else {
                match first_failure {
                    Some((id, message)) => (Some(id), Some(message)),
                    None => (None, None),
                }
            };
            WizardSchoolOptionDto {
                id: id.clone(),
                name: (*name).to_owned(),
                choice_set_id: WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID.to_owned(),
                opposed_choice_set_id: WIZARD_OPPOSED_SCHOOLS_CHOICE_ID.to_owned(),
                supported,
                supported_opposed_pairs,
                reason,
                blocking_diagnostic_id,
            }
        })
        .collect()
}

fn cached_options() -> &'static Vec<WizardSchoolOptionDto> {
    static TABLE: OnceLock<Vec<WizardSchoolOptionDto>> = OnceLock::new();
    TABLE.get_or_init(build_options)
}

/// The nine arcane schools with, per school, the exact opposed-school
/// combinations the engine can save a character on. See the module doc.
#[tauri::command]
pub fn list_wizard_school_options() -> WizardSchoolOptionsResponse {
    WizardSchoolOptionsResponse { options: cached_options().clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Vec<WizardSchoolOptionDto> {
        list_wizard_school_options().options
    }

    fn option(id: &str) -> WizardSchoolOptionDto {
        options().into_iter().find(|o| o.id == id).unwrap_or_else(|| panic!("{id} must be offered"))
    }

    /// Nine schools, ids in the exact shape the engine's `school:` literals
    /// use, names from the pub table's own keys.
    #[test]
    fn offers_the_nine_schools_the_engine_table_names() {
        let ids: Vec<String> = options().iter().map(|o| o.id.clone()).collect();
        assert_eq!(
            ids,
            [
                "school:abjuration",
                "school:conjuration",
                "school:divination",
                "school:enchantment",
                "school:evocation",
                "school:illusion",
                "school:necromancy",
                "school:transmutation",
                "school:universal",
            ]
        );
        assert_eq!(option("school:evocation").name, "Evocation");
        for o in options() {
            assert_eq!(o.choice_set_id, "choice:wizard_school_specialization");
            assert_eq!(o.opposed_choice_set_id, "choice:wizard_opposed_schools");
        }
    }

    /// The one combination the engine's spellbook gate accepts today.
    #[test]
    fn evocation_is_supported_with_exactly_the_necromancy_transmutation_opposed_pair() {
        let evocation = option("school:evocation");
        assert!(evocation.supported);
        assert_eq!(
            evocation.supported_opposed_pairs,
            vec![vec!["school:necromancy".to_owned(), "school:transmutation".to_owned()]]
        );
        assert!(evocation.reason.is_none());
    }

    /// Every other school is reported unsupported, with the engine's own
    /// claim-blocking diagnostic as the reason -- never a bare `false`.
    #[test]
    fn every_other_school_is_unsupported_with_the_engines_own_reason() {
        for o in options().into_iter().filter(|o| o.id != "school:evocation") {
            assert!(!o.supported, "{}", o.id);
            assert!(o.supported_opposed_pairs.is_empty(), "{}", o.id);
            let reason = o.reason.as_deref().unwrap_or_else(|| panic!("{} needs a reason", o.id));
            assert!(
                reason.contains("Evocation"),
                "{}'s reason should be the engine's spellbook-gate sentence, got: {reason}",
                o.id
            );
            assert!(o.blocking_diagnostic_id.is_some(), "{}", o.id);
        }
    }

    /// Free opposed-school pairs are NOT selectable: the same specialty
    /// with any other pair is not in its supported list. Pinned so the
    /// command never implies a freedom the engine does not grant.
    #[test]
    fn a_supported_school_lists_only_the_pairs_the_engine_computes() {
        let evocation = option("school:evocation");
        assert_eq!(evocation.supported_opposed_pairs.len(), 1);
        assert!(!evocation
            .supported_opposed_pairs
            .contains(&vec!["school:abjuration".to_owned(), "school:illusion".to_owned()]));
    }

    /// The support fact is the same one the real save path applies: a
    /// character created through `create_character_at_root` with a
    /// supported combination saves, and with an unsupported one is Blocked.
    #[test]
    fn supported_means_the_real_create_path_saves_and_unsupported_means_it_blocks() {
        use crate::character_hub::{
            create_character_at_root, AbilityScoresDto, CreateCharacterRequest,
            CreateCharacterResponse, SelectedChoiceDto,
        };
        let request = |spec: &str, opposed: &[&str]| {
            let mut choices = vec![SelectedChoiceDto {
                choice_set_id: "choice:wizard_school_specialization".to_owned(),
                selection_id: spec.to_owned(),
            }];
            for o in opposed {
                choices.push(SelectedChoiceDto {
                    choice_set_id: "choice:wizard_opposed_schools".to_owned(),
                    selection_id: (*o).to_owned(),
                });
            }
            CreateCharacterRequest {
                character_id: "char-wizard-school-probe".to_owned(),
                display_label: "Wizard".to_owned(),
                race_id: "race:human".to_owned(),
                class_id: "class:wizard".to_owned(),
                level: 1,
                ability_scores: AbilityScoresDto {
                    strength: 10,
                    dexterity: 14,
                    constitution: 12,
                    intelligence: 16,
                    wisdom: 12,
                    charisma: 8,
                },
                ability_bonus_target: "intelligence".to_owned(),
                selected_alternate_trait_keys: Vec::new(),
                companion_species: None,
                selected_traits: Vec::new(),
                trait_skill_choices: Vec::new(),
                additional_choices: choices,
                saved_at: "2026-09-01T00:00:00Z".to_owned(),
            }
        };
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("codex-wizard-school-{unique}"));

        let saved = create_character_at_root(
            &root.join("ok"),
            &request("school:evocation", &["school:necromancy", "school:transmutation"]),
            "test".to_owned(),
        )
        .expect("create should not error");
        assert!(matches!(saved, CreateCharacterResponse::Saved { .. }), "got {saved:?}");

        let blocked = create_character_at_root(
            &root.join("blocked"),
            &request("school:abjuration", &["school:necromancy", "school:transmutation"]),
            "test".to_owned(),
        )
        .expect("create should not error");
        assert!(matches!(blocked, CreateCharacterResponse::Blocked { .. }), "got {blocked:?}");

        std::fs::remove_dir_all(&root).ok();
    }
}
