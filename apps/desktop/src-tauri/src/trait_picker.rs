//! Character trait selection surface (AT-34-E4-002). The desktop-facing
//! sibling of `race_trait_picker.rs`'s `list_alternate_racial_traits`
//! command, following the identical shape: a plain, no-argument
//! `#[tauri::command]` that hands the frontend a real, corpus-derived list
//! of options -- built directly from `codex::rules_core::trait_effects`'s
//! own two tables, never a hand-typed duplicate of either.
//!
//! Unlike `race_trait_picker`, a flat trait needs no "resolve" step: a
//! flat `BONUS:SKILL` trait has no alternate-swap exclusivity to
//! validate, no rendered-description formula, and no `held_feats`
//! dependency -- the option list itself, and the id the frontend echoes
//! back on `CreateCharacterRequest.selected_traits`, are the entire
//! contract. A **choice-based** trait (`skill_options` non-empty) needs one
//! more echo: the frontend's chosen `skill_options` entry is sent back as a
//! `SelectedChoiceDto { choice_set_id: <this option's own choice_set_id>,
//! selection_id: <the chosen skill_id> }` on
//! `CreateCharacterRequest.trait_skill_choices` -- the same generic
//! `SelectedChoice` channel `archetype_resolver.rs`'s own pool-choice
//! primitive already established, not a new mechanism.
use codex::rules_core::trait_effects::{
    family_choice_skill_options, trait_skill_choice_id, FAMILY_CHOICE_TRAIT_BONUSES,
    FLAT_SKILL_TRAIT_BONUSES, SAVE_TRAIT_BONUSES, SITUATIONAL_SKILL_TRAIT_BONUSES,
    SKILL_CHOICE_TRAIT_BONUSES,
};
use serde::Serialize;

/// One skill this trait's `%LIST` choice can be resolved to -- see
/// [`CharacterTraitOptionDto::skill_options`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraitSkillOptionDto {
    /// The `skill:` wire id to echo back as `SelectedChoiceDto.selection_id`.
    pub skill_id: String,
    /// Display name (`"Intimidate"`), built the same way
    /// [`CharacterTraitOptionDto::skills`] already is.
    pub name: String,
}

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
    /// needs a second, hand-maintained skill-name table. Empty for a
    /// choice-based trait (`skill_options` non-empty instead) -- which
    /// skill applies is not known until the player picks one.
    pub skills: Vec<String>,
    pub bonus: i8,
    /// Non-empty **only** for a fixed-choice `%LIST` trait
    /// (`trait_effects::SKILL_CHOICE_TRAIT_BONUSES`): the concrete skills
    /// the player may pick between. Empty for a flat trait, whose target
    /// skill(s) are already fixed in `skills` above.
    #[serde(default)]
    pub skill_options: Vec<TraitSkillOptionDto>,
    /// `Some(<choice_set_id>)` only for a choice-based trait -- the
    /// `SelectedChoiceDto.choice_set_id` the frontend must echo back,
    /// paired with whichever `skill_options` entry the player picked, on
    /// `CreateCharacterRequest.trait_skill_choices`. `None` for a flat
    /// trait, which needs no such echo.
    #[serde(default)]
    pub choice_set_id: Option<String>,
    /// `Some(<save name>)` (`"Fortitude"`, `"Reflex"`, or `"Will"`) only
    /// for a flat `BONUS:SAVE` trait (`trait_effects::SAVE_TRAIT_
    /// BONUSES`) -- fourth slice, `AT-34-E4-002`. `None` for every
    /// skill-pillar trait, whose `skills`/`skill_options` above already
    /// say what it affects. A save-bonus option's `skills` is always
    /// empty and `bonus` still carries the flat integer, same as a flat
    /// skill trait -- only which pillar it targets differs.
    #[serde(default)]
    pub save: Option<String>,
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
/// supports -- `ultimate_campaign`'s 31 flat `BONUS:SKILL` traits, its 5
/// fixed-choice `BONUS:SKILL|%LIST` traits, its 4 open-family
/// `BONUS:SKILL|%LIST` traits, its 2 flat `BONUS:SAVE` traits, and its 3
/// `BONUS:SITUATION` traits -- every option returned genuinely grants its
/// stated bonus when selected (and, for a choice-based option, a valid
/// `skill_options` choice recorded) and passed through `create_character`/
/// `compose_character_input` (`trait_effects::skill_bonuses_from_traits`
/// + `trait_effects::skill_choice_bonuses_from_traits` +
/// `trait_effects::family_choice_bonuses_from_traits` +
/// `trait_effects::save_bonuses_from_traits` +
/// `pilot_compute::ground_orphan_trait_facts`'s situational-fact channel +
/// `trait_effects::situational_flat_skill_bonuses_from_traits`), never a
/// rendered option with no compute path behind it.
///
/// **Note**: the fifth and sixth slices' traits (`INITIATIVE_TRAIT_
/// BONUSES`, `CONCENTRATION_TRAIT_BONUSES`, `ABILITY_DIFF_SKILL_TRAIT_
/// BONUSES` -- Tactician, Arcane Temper, Desperate Resolve, Bruising
/// Intellect, Planar Savant, Pragmatic Activator, Precise Treatment) are
/// NOT in this roster yet, a pre-existing gap this cycle found but did not
/// introduce and does not own fixing (see this cycle's own receipt).
#[tauri::command]
pub fn list_available_character_traits() -> Vec<CharacterTraitOptionDto> {
    let flat = FLAT_SKILL_TRAIT_BONUSES.iter().map(|entry| CharacterTraitOptionDto {
        id: entry.trait_id.to_owned(),
        name: entry.name.to_owned(),
        description: entry.description.to_owned(),
        skills: entry.skills.iter().map(|skill_id| skill_display_name(skill_id)).collect(),
        bonus: entry.bonus,
        skill_options: Vec::new(),
        choice_set_id: None,
        save: None,
    });
    let choice = SKILL_CHOICE_TRAIT_BONUSES.iter().map(|entry| CharacterTraitOptionDto {
        id: entry.trait_id.to_owned(),
        name: entry.name.to_owned(),
        description: entry.description.to_owned(),
        skills: Vec::new(),
        bonus: entry.bonus,
        skill_options: entry
            .skill_options
            .iter()
            .map(|skill_id| TraitSkillOptionDto {
                skill_id: (*skill_id).to_owned(),
                name: skill_display_name(skill_id),
            })
            .collect(),
        choice_set_id: Some(trait_skill_choice_id(entry.trait_id)),
        save: None,
    });
    // Third slice (`AT-34-E4-002`): the 4 open-subtype-family `%LIST`
    // traits -- `skill_options` here is the resolved Craft/Perform/
    // Profession family UNION (`family_choice_skill_options`), not a
    // hand-typed literal, so the frontend's own generic
    // `skillOptions.length > 0` picker rendering (`CreateCharacterForm.
    // tsx`) needs no change at all to support this new shape.
    let family_choice = FAMILY_CHOICE_TRAIT_BONUSES.iter().map(|entry| CharacterTraitOptionDto {
        id: entry.trait_id.to_owned(),
        name: entry.name.to_owned(),
        description: entry.description.to_owned(),
        skills: Vec::new(),
        bonus: entry.bonus,
        skill_options: family_choice_skill_options(entry)
            .iter()
            .map(|skill_id| TraitSkillOptionDto {
                skill_id: (*skill_id).to_owned(),
                name: skill_display_name(skill_id),
            })
            .collect(),
        choice_set_id: Some(trait_skill_choice_id(entry.trait_id)),
        save: None,
    });
    // Fourth slice (`AT-34-E4-002`): the 2 flat `BONUS:SAVE` traits --
    // needs no `choice_set_id` (no player choice, same as a flat skill
    // trait) and no `skills` (the frontend's generic display logic checks
    // `save` for this shape instead, the same "one new field, zero new
    // mechanism" pattern the family-choice slice established for
    // `skill_options`).
    let save_bonus = SAVE_TRAIT_BONUSES.iter().map(|entry| CharacterTraitOptionDto {
        id: entry.trait_id.to_owned(),
        name: entry.name.to_owned(),
        description: entry.description.to_owned(),
        skills: Vec::new(),
        bonus: entry.bonus,
        skill_options: Vec::new(),
        choice_set_id: None,
        save: Some(entry.save.to_owned()),
    });
    // Seventh slice (`AT-34-E4-002`): the 3 `BONUS:SITUATION` traits.
    // Reuses the SAME `skills`/`bonus` fields the flat slice already
    // established -- every clause on these 3 records happens to share one
    // identical bonus magnitude (proven by
    // `every_situational_option_has_a_uniform_bonus_across_its_clauses`
    // below), so no new DTO field is needed: `skills` lists every skill a
    // clause names (Trustworthy's separate flat Diplomacy token included),
    // and the option's own `description` states each clause's real
    // circumstance in full prose, exactly as it already does for every
    // earlier slice's non-uniform prose detail.
    let situational = SITUATIONAL_SKILL_TRAIT_BONUSES.iter().map(|entry| {
        let mut skills: Vec<String> =
            entry.situational.iter().map(|&(skill_name, _, _)| skill_name.to_owned()).collect();
        let mut bonus = entry.situational.first().map(|&(_, _, bonus)| bonus).unwrap_or(0);
        if let Some((skill_id, flat_bonus)) = entry.flat_skill {
            skills.push(skill_display_name(skill_id));
            if entry.situational.is_empty() {
                bonus = i16::from(flat_bonus);
            }
        }
        CharacterTraitOptionDto {
            id: entry.trait_id.to_owned(),
            name: entry.name.to_owned(),
            description: entry.description.to_owned(),
            skills,
            bonus: bonus as i8,
            skill_options: Vec::new(),
            choice_set_id: None,
            save: None,
        }
    });
    flat.chain(choice)
        .chain(family_choice)
        .chain(save_bonus)
        .chain(situational)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The command must return exactly the 31 flat-slice traits plus the 5
    /// fixed-choice-slice traits plus the 4 family-choice-slice traits
    /// plus the 2 flat-save-slice traits plus the 3 situational-slice
    /// traits (45 total), never a subset or a hand-typed duplicate that
    /// could drift from `trait_effects`' own tables.
    #[test]
    fn returns_every_flat_and_choice_skill_trait() {
        let options = list_available_character_traits();
        assert_eq!(
            options.len(),
            FLAT_SKILL_TRAIT_BONUSES.len()
                + SKILL_CHOICE_TRAIT_BONUSES.len()
                + FAMILY_CHOICE_TRAIT_BONUSES.len()
                + SAVE_TRAIT_BONUSES.len()
                + SITUATIONAL_SKILL_TRAIT_BONUSES.len()
        );
        assert_eq!(options.len(), 45);
    }

    /// Every situational-slice option's `skills` list is a real, non-empty
    /// set of skill display names, and every clause on the SAME record
    /// shares one identical bonus magnitude -- the precondition that lets
    /// this DTO reuse the flat `bonus: i8` field instead of needing a
    /// per-clause shape.
    #[test]
    fn every_situational_option_has_a_uniform_bonus_across_its_clauses() {
        for entry in SITUATIONAL_SKILL_TRAIT_BONUSES {
            let mut magnitudes: Vec<i16> =
                entry.situational.iter().map(|&(_, _, bonus)| bonus).collect();
            if let Some((_, flat_bonus)) = entry.flat_skill {
                magnitudes.push(i16::from(flat_bonus));
            }
            assert!(!magnitudes.is_empty(), "{} has no bonus clauses at all", entry.corpus_key);
            assert!(
                magnitudes.iter().all(|&m| m == magnitudes[0]),
                "{} carries non-uniform bonus magnitudes {:?} -- the picker's single `bonus` \
                 field would misreport at least one clause",
                entry.corpus_key,
                magnitudes
            );
        }
    }

    /// Every situational-slice option's id round-trips through the real
    /// `trait_effects::situational_skill_trait_magnitude_is_grounded_for_
    /// corpus_key` compute path (looked up via the option's OWN table
    /// entry, since the DTO itself carries no `corpus_key` field) --
    /// never a rendered option with no compute path behind it.
    #[test]
    fn every_situational_option_id_is_recognized_by_the_compute_path() {
        for option in list_available_character_traits() {
            let Some(entry) =
                SITUATIONAL_SKILL_TRAIT_BONUSES.iter().find(|e| e.trait_id == option.id)
            else {
                continue; // not a situational-slice option
            };
            assert!(
                codex::rules_core::trait_effects::situational_skill_trait_magnitude_is_grounded_for_corpus_key(
                    entry.corpus_key
                )
                .is_some(),
                "{} was returned by the picker but did not ground via real fixture execution",
                option.id
            );
        }
    }

    /// The Almost Human trait (seventh slice: single `BONUS:SITUATION`
    /// clause) reaches the DTO with its real corpus data verbatim.
    #[test]
    fn almost_human_option_carries_its_real_corpus_situational_data() {
        let almost_human = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_almost_human")
            .expect("Almost Human must be in the roster");
        assert_eq!(almost_human.name, "Almost Human");
        assert_eq!(almost_human.bonus, 4);
        assert_eq!(almost_human.skills, vec!["Disguise".to_string()]);
        assert!(almost_human.description.contains("appear human"));
        assert!(almost_human.skill_options.is_empty());
        assert_eq!(almost_human.choice_set_id, None);
        assert_eq!(almost_human.save, None);
    }

    /// The Trustworthy trait (seventh slice: a situational clause PLUS a
    /// separate flat token) reaches the DTO listing BOTH skills its two
    /// tokens name, never just one.
    #[test]
    fn trustworthy_option_lists_both_its_situational_and_flat_skills() {
        let trustworthy = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_trustworthy")
            .expect("Trustworthy must be in the roster");
        assert_eq!(trustworthy.bonus, 1);
        assert_eq!(trustworthy.skills, vec!["Bluff".to_string(), "Diplomacy".to_string()]);
    }

    /// Every flat-skill-slice option's id round-trips: it is exactly what
    /// `trait_effects::skill_bonuses_from_traits` recognizes with no
    /// choice recorded, so a frontend that echoes it back on
    /// `selected_traits` gets a real, computed bonus, not a
    /// silently-ignored unknown id.
    #[test]
    fn every_flat_option_id_is_recognized_by_the_compute_path() {
        for option in list_available_character_traits() {
            if option.choice_set_id.is_some() || option.save.is_some() {
                // covered by the choice-slice and save-slice tests
                // instead -- neither shape reaches
                // `skill_bonuses_from_traits`.
                continue;
            }
            if SITUATIONAL_SKILL_TRAIT_BONUSES.iter().any(|entry| entry.trait_id == option.id) {
                // covered by `every_situational_option_id_is_recognized_by_
                // the_compute_path` instead -- a situational option's
                // standalone-fact half never reaches
                // `skill_bonuses_from_traits` (only its optional separate
                // flat-skill half might, and that is a different table:
                // `situational_flat_skill_bonuses_from_traits`).
                continue;
            }
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

    /// Every save-slice option's id round-trips through the real
    /// `trait_effects::save_bonuses_from_traits` compute path, landing on
    /// exactly the save it declares -- never a rendered option with no
    /// compute path behind it.
    #[test]
    fn every_save_option_id_is_recognized_by_the_compute_path() {
        for option in list_available_character_traits() {
            let Some(save) = option.save.clone() else {
                continue; // not a save-slice option
            };
            let bonuses = codex::rules_core::trait_effects::save_bonuses_from_traits(&[
                option.id.clone(),
            ]);
            let landed = match save.as_str() {
                "Fortitude" => bonuses.fortitude,
                "Reflex" => bonuses.reflex,
                "Will" => bonuses.will,
                other => panic!("{other} is not a real save name"),
            };
            assert_eq!(
                landed, option.bonus as i16,
                "{} declares a {save} save bonus of {} but \
                 save_bonuses_from_traits landed {landed}",
                option.id, option.bonus
            );
        }
    }

    /// Every choice-slice option (both the fixed-list second slice and
    /// the open-family third slice) carries a non-empty `skill_options`, a
    /// real `choice_set_id`, and -- once the player's echoed choice is
    /// recorded exactly the way `create_character` will record it --- one
    /// of the two choice-based compute paths genuinely grants a bonus
    /// (each trait id is a member of exactly one, per
    /// `no_trait_id_appears_in_more_than_one_table`, so summing both
    /// paths' results is always safe and never double-applies). Never a
    /// rendered picker with no compute path behind the choice it offers.
    #[test]
    fn every_choice_option_round_trips_through_the_compute_path() {
        for option in list_available_character_traits() {
            let Some(choice_set_id) = option.choice_set_id.clone() else {
                continue; // a flat option, covered by the test above instead
            };
            assert!(
                !option.skill_options.is_empty(),
                "{} declares a choice_set_id but offers no skill_options",
                option.id
            );
            let picked = &option.skill_options[0];
            let selected_choice = codex::rules_core::character_input::SelectedChoice {
                choice_set_id,
                selection_id: picked.skill_id.clone(),
            };
            let mut bonuses = codex::rules_core::trait_effects::skill_choice_bonuses_from_traits(
                &[option.id.clone()],
                &[selected_choice.clone()],
            );
            for (skill_id, bonus) in codex::rules_core::trait_effects::family_choice_bonuses_from_traits(
                &[option.id.clone()],
                &[selected_choice],
            ) {
                *bonuses.entry(skill_id).or_insert(0) += bonus;
            }
            assert_eq!(
                bonuses.get(&picked.skill_id),
                Some(&option.bonus),
                "{} (choice {}) did not ground a bonus via either choice-based compute path",
                option.id,
                picked.skill_id
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
        assert!(acrobat.skill_options.is_empty());
        assert_eq!(acrobat.choice_set_id, None);
    }

    /// The Criminal trait's real corpus content reaches the DTO verbatim,
    /// as a choice-based option: no fixed `skills`, but three real
    /// `skill_options` and a non-empty `choice_set_id`.
    #[test]
    fn criminal_option_carries_its_real_corpus_choice_data() {
        let criminal = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_criminal")
            .expect("Criminal must be in the roster");
        assert_eq!(criminal.name, "Criminal");
        assert_eq!(criminal.bonus, 1);
        assert!(criminal.skills.is_empty());
        assert_eq!(
            criminal.choice_set_id.as_deref(),
            Some("trait_choice:trait:trait_criminal")
        );
        let option_ids: Vec<&str> =
            criminal.skill_options.iter().map(|o| o.skill_id.as_str()).collect();
        assert_eq!(
            option_ids,
            vec!["skill:disable_device", "skill:intimidate", "skill:sleight_of_hand"]
        );
        assert_eq!(criminal.skill_options[0].name, "Disable Device");
    }

    /// The Artisan trait (third slice: `TYPE=Craft` open-subtype family)
    /// reaches the DTO with a real, corpus-derived multi-entry
    /// `skill_options` list (all 23 `Craft (<subtype>)` ids), not a
    /// hand-typed stand-in and not an empty "not built yet" list.
    #[test]
    fn artisan_option_carries_the_full_craft_family_as_skill_options() {
        let artisan = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_artisan")
            .expect("Artisan must be in the roster");
        assert_eq!(artisan.name, "Artisan");
        assert_eq!(artisan.bonus, 2);
        assert!(artisan.skills.is_empty());
        assert_eq!(
            artisan.choice_set_id.as_deref(),
            Some("trait_choice:trait:trait_artisan")
        );
        assert_eq!(artisan.skill_options.len(), 23);
        assert!(artisan.skill_options.iter().any(|o| o.skill_id == "skill:craft_weapons"));
        assert!(!artisan.skill_options.iter().any(|o| o.skill_id.starts_with("skill:perform")));
    }

    /// The Mentored trait (three-family union: Craft + Perform +
    /// Profession) reaches the DTO with every one of the three families'
    /// members, not just the first-listed family.
    #[test]
    fn mentored_option_carries_the_union_of_all_three_named_families() {
        let mentored = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_mentored")
            .expect("Mentored must be in the roster");
        assert_eq!(mentored.skill_options.len(), 23 + 9 + 31);
        assert!(mentored.skill_options.iter().any(|o| o.skill_id == "skill:craft_alchemy"));
        assert!(mentored.skill_options.iter().any(|o| o.skill_id == "skill:perform_sing"));
        assert!(mentored.skill_options.iter().any(|o| o.skill_id == "skill:profession_scribe"));
    }

    /// The Life of Toil trait (fourth slice: flat `BONUS:SAVE`) reaches
    /// the DTO with its real corpus data verbatim, as a save-bonus
    /// option: no `skills`, no `skill_options`, no `choice_set_id`, but a
    /// real `save` naming Fortitude.
    #[test]
    fn life_of_toil_option_carries_its_real_corpus_save_data() {
        let life_of_toil = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_life_of_toil")
            .expect("Life of Toil must be in the roster");
        assert_eq!(life_of_toil.name, "Life of Toil");
        assert_eq!(life_of_toil.bonus, 1);
        assert_eq!(life_of_toil.save.as_deref(), Some("Fortitude"));
        assert!(life_of_toil.skills.is_empty());
        assert!(life_of_toil.skill_options.is_empty());
        assert_eq!(life_of_toil.choice_set_id, None);
        assert!(life_of_toil.description.contains("Fortitude"));
    }

    /// The Indomitable Faith trait reaches the DTO with a real `save`
    /// naming Will, not Fortitude -- proves the two save-slice entries
    /// are not accidentally aliased to the same save.
    #[test]
    fn indomitable_faith_option_carries_its_real_corpus_save_data() {
        let indomitable_faith = list_available_character_traits()
            .into_iter()
            .find(|o| o.id == "trait:trait_indomitable_faith")
            .expect("Indomitable Faith must be in the roster");
        assert_eq!(indomitable_faith.save.as_deref(), Some("Will"));
        assert_eq!(indomitable_faith.bonus, 1);
    }
}
