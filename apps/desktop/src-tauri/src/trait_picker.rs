//! Character trait selection surface (AT-34-E4-002). The desktop-facing
//! sibling of `race_trait_picker.rs`'s `list_alternate_racial_traits`
//! command, following the identical shape: a plain, no-argument
//! `#[tauri::command]` that hands the frontend a real, corpus-derived list
//! of options -- built directly from `codex::rules_core::trait_effects
//! ::FLAT_SKILL_TRAIT_BONUSES`, never a hand-typed duplicate of it.
//!
//! Unlike `race_trait_picker`, this surface needs no "resolve" step: a
//! flat `BONUS:SKILL` trait has no alternate-swap exclusivity to
//! validate, no rendered-description formula, and no `held_feats`
//! dependency -- the option list itself, and the id the frontend echoes
//! back on `CreateCharacterRequest.selected_traits`, are the entire
//! contract.

use codex::rules_core::trait_effects::FLAT_SKILL_TRAIT_BONUSES;
use serde::Serialize;

/// One selectable trait option, as the frontend picker renders it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterTraitOptionDto {
    /// Echoed back verbatim on `CreateCharacterRequest.selected_traits`.
    pub id: String,
    pub name: String,
    pub description: String,
    /// The skill(s) this trait's flat bonus applies to, as display names
    /// (`"Acrobatics"`, not the `skill:` wire id) -- built from each
    /// `skill:` id by title-casing its own segments, so the UI never
    /// needs a second, hand-maintained skill-name table.
    pub skills: Vec<String>,
    pub bonus: i8,
}

fn skill_display_name(skill_id: &str) -> String {
    skill_id
        .trim_start_matches("skill:")
        .split('_')
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The full roster of traits this cycle's real compute-and-apply path
/// supports (`ultimate_campaign`'s 31 flat `BONUS:SKILL` traits) -- every
/// option returned genuinely grants its stated bonus when selected and
/// passed through `create_character`/`compose_character_input`
/// (`trait_effects::skill_bonuses_from_traits`), never a rendered option
/// with no compute path behind it.
#[tauri::command]
pub fn list_available_character_traits() -> Vec<CharacterTraitOptionDto> {
    FLAT_SKILL_TRAIT_BONUSES
        .iter()
        .map(|entry| CharacterTraitOptionDto {
            id: entry.trait_id.to_owned(),
            name: entry.name.to_owned(),
            description: entry.description.to_owned(),
            skills: entry.skills.iter().map(|skill_id| skill_display_name(skill_id)).collect(),
            bonus: entry.bonus,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The command must return exactly the 31 flat-slice traits, never a
    /// subset or a hand-typed duplicate that could drift from
    /// `trait_effects`' own table.
    #[test]
    fn returns_every_flat_skill_trait() {
        let options = list_available_character_traits();
        assert_eq!(options.len(), FLAT_SKILL_TRAIT_BONUSES.len());
        assert_eq!(options.len(), 31);
    }

    /// Every returned id round-trips: it is exactly what
    /// `trait_effects::skill_bonuses_from_traits` recognizes, so a
    /// frontend that echoes it back on `selected_traits` gets a real,
    /// computed bonus, not a silently-ignored unknown id.
    #[test]
    fn every_returned_id_is_recognized_by_the_compute_path() {
        for option in list_available_character_traits() {
            let bonuses = codex::rules_core::trait_effects::skill_bonuses_from_traits(&[
                option.id.clone(),
            ]);
            assert!(
                !bonuses.is_empty(),
                "{} was returned by the picker but is not recognized by \
                 skill_bonuses_from_traits -- a selectable option with no real effect",
                option.id
            );
        }
    }

    #[test]
    fn skill_display_name_title_cases_each_segment() {
        assert_eq!(skill_display_name("skill:acrobatics"), "Acrobatics");
        assert_eq!(skill_display_name("skill:knowledge_nobility"), "Knowledge Nobility");
        assert_eq!(skill_display_name("skill:craft_alchemy"), "Craft Alchemy");
    }

    /// The Acrobat trait's real corpus content reaches the DTO verbatim.
    #[test]
    fn acrobat_option_carries_its_real_corpus_data() {
        let acrobat = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_acrobat")
            .expect("Acrobat must be in the roster");
        assert_eq!(acrobat.name, "Acrobat");
        assert_eq!(acrobat.bonus, 1);
        assert_eq!(acrobat.skills, vec!["Acrobatics".to_string()]);
        assert!(acrobat.description.contains("Acrobatics"));
    }
}
