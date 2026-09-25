// -- split from `tests` in src/rules_core/race_creation.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::race_creation::*;
    use codex::rules_core::race_resolver::{ResolvedRace, ResolvedTrait};
    use codex::rules_core::size::SizeCategory;
    use codex::rules_core::race_resolver::{SizeSource, SpeedSource, TraitRole};

    /// A size that cannot be read is a refusal, never a defaulted Medium.
    #[test]
    fn a_race_with_no_readable_size_is_refused() {
        let mut race = bare_race("Sizeless");
        race.traits.push(ability_trait("Sizeless Ability Scores", &[("STAT", "STR", "2")]));
        race.size = None;
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("declares no readable creature size"), "unexpected reason: {err}");
    }

    /// A speed that cannot be read is a refusal too.
    #[test]
    fn a_race_with_no_readable_speed_is_refused() {
        let mut race = bare_race("Speedless");
        race.traits.push(ability_trait("Speedless Ability Scores", &[("STAT", "DEX", "2")]));
        race.walk_speed_ft = None;
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("declares no readable base land speed"), "unexpected reason: {err}");
    }

    /// A comma-separated `BONUS:STAT` code list credits every code — the
    /// Goblin `BONUS:STAT|STR,CHA|-2` shape.
    #[test]
    fn a_multi_code_bonus_stat_chain_credits_every_code() {
        let mut race = bare_race("Multi");
        race.traits.push(ability_trait("Multi Ability Scores", &[("STAT", "STR,CHA", "-2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments.get("strength"), Some(&-2));
        assert_eq!(chassis.ability_adjustments.get("charisma"), Some(&-2));
        assert_eq!(chassis.floating_bonus_points, 0);
    }

    /// A floating pool passes on its own, with no fixed modifier — the Human
    /// shape — and its magnitude comes from the row's own name.
    #[test]
    fn a_floating_ability_pool_passes_on_its_own() {
        let mut race = bare_race("Floater");
        race.traits.push(ability_trait(
            "+2 to One Ability Score",
            &[("ABILITYPOOL", "Ability Bonus", "1")],
        ));
        let chassis = race_creation_chassis(&race).expect("a floating pool is a real magnitude");
        assert!(chassis.ability_adjustments.is_empty());
        assert_eq!(chassis.floating_bonus_points, 2);
    }

    /// An ability-pool row that does not state its magnitude in its own name
    /// is an error, never a guessed magnitude.
    #[test]
    fn an_ability_pool_row_without_a_stated_magnitude_is_an_error() {
        let mut race = bare_race("Vague");
        race.traits
            .push(ability_trait("Some Bonus", &[("ABILITYPOOL", "Ability Bonus", "1")]));
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(err.contains("must state its magnitude in its own name"), "unexpected: {err}");
    }

    /// **The chassis NAMES the record its ability magnitude was read from.**
    ///
    /// `SD31-W15-RACETRAIT-001`. Without this, a second consumer asking "which
    /// `race_trait` record did the character-creation path actually read?" has
    /// to re-implement [`racial_ability_scores_trait`]'s selection rule and
    /// [`fixed_ability_adjustments`]' parsing — an instrument *asserting* this
    /// module's behaviour instead of *observing* it, which is exactly the
    /// failure this module's own header comment exists to prevent. The
    /// consumer reports what it read; nobody guesses.
    #[test]
    fn the_chassis_names_the_trait_record_its_ability_magnitude_came_from() {
        let mut race = bare_race("Named");
        race.traits.push(ability_trait("Named Ability Scores", &[("STAT", "CON,WIS", "2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments_source_trait_key, "Test ~ Named Ability Scores");
    }

    /// …and it names the row that really supplied the numbers, not merely the
    /// first trait the race applies. A positional answer would credit whichever
    /// record happened to sort first, which is the "credit resting on a
    /// DIFFERENT record" shape wave 12 demoted 251 units for.
    #[test]
    fn the_named_source_row_is_the_ability_scores_row_not_merely_the_first_trait() {
        let mut race = bare_race("Two");
        race.traits.push(plain_trait("Decoy"));
        race.traits.push(ability_trait("Two Ability Scores", &[("STAT", "STR", "2")]));
        let chassis = race_creation_chassis(&race).expect("a real magnitude is stated");
        assert_eq!(chassis.ability_adjustments_source_trait_key, "Test ~ Two Ability Scores");
        assert_eq!(chassis.ability_adjustments.get("strength"), Some(&2));
    }

    /// Modifiers that cancel to zero do not count as a magnitude: a race
    /// whose only `BONUS:STAT` chains sum to nothing is refused exactly as a
    /// race with no chain at all is.
    #[test]
    fn ability_adjustments_that_cancel_to_zero_do_not_count_as_a_magnitude() {
        let mut race = bare_race("Cancel");
        race.traits.push(ability_trait(
            "Cancel Ability Scores",
            &[("STAT", "STR", "2"), ("STAT", "STR", "-2")],
        ));
        let err = race_creation_chassis(&race).unwrap_err();
        assert!(
            err.contains("states neither a fixed ability modifier nor a floating ability pool"),
            "unexpected refusal reason: {err}"
        );
    }

    /// A trait carrying no `Racial Ability Scores` type token — the shape
    /// every ordinary racial trait has.
    fn plain_trait(name: &str) -> ResolvedTrait {
        let mut plain = ability_trait(name, &[("STAT", "STR", "9")]);
        plain.type_tokens = vec!["Special Quality".to_owned()];
        plain
    }

    fn ability_trait(name: &str, chains: &[(&str, &str, &str)]) -> ResolvedTrait {
        use codex_ingest::pcgen_import::bonus_chain_reader::declared_bonuses_from_chains;
        use codex_ingest::pcgen_import::ingest_payload::RawBonusChain;
        ResolvedTrait {
            key: format!("Test ~ {name}"),
            name: name.to_owned(),
            book_id: "test_book".to_owned(),
            role: TraitRole::Default,
            type_tokens: vec![RACIAL_ABILITY_SCORES_TYPE.to_owned()],
            description: None,
            source_page: None,
            declared_walk_speed_ft: None,
            declared_size: None,
            declared_vision: Vec::new(),
            declared_bonuses: declared_bonuses_from_chains(
                &chains
                    .iter()
                    .map(|(a, b, c)| RawBonusChain {
                        qualifiers: vec![(*a).to_owned(), (*b).to_owned(), (*c).to_owned()],
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }

    fn bare_race(race_key: &str) -> ResolvedRace {
        ResolvedRace {
            race_key: race_key.to_owned(),
            name: race_key.to_owned(),
            book_id: "test_book".to_owned(),
            size: Some(SizeCategory::Medium),
            chassis_size: Some(SizeCategory::Medium),
            size_source: SizeSource::Chassis,
            race_type: Some("Humanoid".to_owned()),
            chassis_walk_speed_ft: Some(30),
            walk_speed_ft: Some(30),
            speed_source: SpeedSource::Chassis,
            traits: Vec::new(),
            fired_flags: Vec::new(),
            suppressions: Vec::new(),
            unmatched_selections: Vec::new(),
            inert_flags: Vec::new(),
        }
    }


}
