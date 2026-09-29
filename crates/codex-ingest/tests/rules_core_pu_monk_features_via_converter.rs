// -- split from `tests` in src/rules_core/rules_tables/pathfinder_unchained/monk_features.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::rules_tables::pathfinder_unchained::monk_features::*;
    use codex_ingest::pcgen_import::ingest_record;
    use std::path::PathBuf;

    /// `Unchained Monk ~ Evasion` (`:465`) and `~ Improved Evasion` (`:472`)
    /// each carry a `DESC:` and **no** `BONUS:`/`DEFINE:` token. Before this
    /// they computed nothing; the numbers their prose states are 0% damage on
    /// a made Reflex save and 50% on a failed one.
    #[test]
    fn evasion_percentages_are_the_ones_the_two_rows_prose_states() {
        use prose_derived::{
            evasion_damage_percent_on_a_successful_reflex_save as made,
            improved_evasion_damage_percent_on_a_failed_reflex_save as failed,
        };
        for level in 0..EVASION_LEVEL {
            assert_eq!(made(level), None, "level {level}");
        }
        for level in EVASION_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(made(level), Some(0), "level {level}");
        }
        for level in 0..IMPROVED_EVASION_LEVEL {
            assert_eq!(failed(level), None, "level {level}");
        }
        for level in IMPROVED_EVASION_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(failed(level), Some(50), "level {level}");
        }

        for key in ["Unchained Monk ~ Evasion", "Unchained Monk ~ Improved Evasion"] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} must still carry no BONUS: token -- if it gained one, the prose-derived \
                 reading is no longer the only source and must be revisited"
            );
        }
        assert!(
            description_of("Unchained Monk ~ Evasion").contains(
                "against an attack that normally deals half damage on a successful save, he \
                 instead takes no damage"
            ),
            "Evasion prose changed; the 0% reading must be re-derived"
        );
        assert!(
            description_of("Unchained Monk ~ Improved Evasion")
                .contains("henceforth he takes only half damage on failed saves"),
            "Improved Evasion prose changed; the 50% reading must be re-derived"
        );
    }

    /// `Unchained Monk ~ Flawless Mind` (`:475`) and `~ Timeless Body`
    /// (`:474`) — same shape, same fix.
    #[test]
    fn flawless_mind_and_timeless_body_state_the_numbers_their_prose_carries() {
        use prose_derived::{
            flawless_mind_will_save_rolls as rolls,
            timeless_body_aging_ability_penalty as aging_penalty,
        };
        for level in 0..FLAWLESS_MIND_LEVEL {
            assert_eq!(rolls(level), None, "level {level}");
        }
        assert_eq!(rolls(FLAWLESS_MIND_LEVEL), Some(2));
        assert_eq!(rolls(MAX_SUPPORTED_LEVEL), Some(2));

        for level in 0..TIMELESS_BODY_LEVEL {
            assert_eq!(aging_penalty(level), None, "level {level}");
        }
        for level in TIMELESS_BODY_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(aging_penalty(level), Some(0), "level {level}");
        }

        for key in ["Unchained Monk ~ Flawless Mind", "Unchained Monk ~ Timeless Body"] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} must still carry no BONUS: token"
            );
        }
        assert!(
            description_of("Unchained Monk ~ Flawless Mind")
                .contains("Whenever he attempts a Will save, he can roll twice and take the better result"),
            "Flawless Mind prose changed; the two-rolls reading must be re-derived"
        );
        assert!(
            description_of("Unchained Monk ~ Timeless Body")
                .contains("a monk no longer takes penalties to his ability scores for aging"),
            "Timeless Body prose changed; the zero-aging-penalty reading must be re-derived"
        );
    }

    /// The two Unchained Monk features that genuinely state no number, in
    /// prose or in token. This pins the *reason* they compute nothing, so a
    /// later cycle cannot quietly invent a magnitude for them — and so that
    /// if the corpus ever grows one, this test fails and says so.
    #[test]
    fn purity_of_body_and_tongue_of_the_sun_and_moon_state_no_number_at_all() {
        for (key, grant_level_digits) in [
            ("Unchained Monk ~ Purity of Body", "5"),
            ("Unchained Monk ~ Tongue of the Sun and Moon", "13"),
        ] {
            assert!(
                bonus_tokens(&record_for(key)).is_empty(),
                "{key} carries no BONUS: token"
            );
            let description = description_of(key);
            let digits: String = description.chars().filter(|c| c.is_ascii_digit()).collect();
            assert_eq!(
                digits, grant_level_digits,
                "{key}'s only digits must still be its own grant level -- a new number in this \
                 prose means a magnitude is now derivable and must be modelled. \
                 Corpus says: {description}"
            );
        }
    }

    /// §24 compliance for every magnitude whose token the ingested record
    /// actually carries.
    #[test]
    fn every_modelled_formula_is_byte_exact_against_the_ingested_corpus_record() {
        assert_bonus_token("Unchained Monk ~ AC Bonus", "BONUS:VAR|MonkACLVL|MonkLVL|TYPE=Level");
        assert_bonus_token(
            "Unchained Monk ~ Bonus Feat",
            "BONUS:ABILITYPOOL|Unchained Monk Bonus Feat|1+max((MonkBonusFeatLVL+2)/4,0)",
        );
        assert_bonus_token("Unchained Monk ~ Bonus Feat", "BONUS:VAR|MonkBonusFeatLVL|MonkLVL");
        assert_bonus_token(
            "Unchained Monk ~ Fast Movement",
            "BONUS:VAR|MonkFastMovementBonus|10*floor(MonkFastMovementLVL/3)",
        );
        assert_bonus_token(
            "Unchained Monk ~ Fast Movement",
            "BONUS:VAR|MonkFastMovementLVL|MonkLVL",
        );
        assert_bonus_token("Unchained Monk ~ Ki Pool", "BONUS:VAR|KiPoolLVL|MonkLVL");
        assert_bonus_token(
            "Unchained Monk ~ Ki Powers",
            "BONUS:VAR|Pool_Unchained_Ki_Power|(MonkLVL-2)/2",
        );
        assert_bonus_token(
            "Unchained Monk ~ Style Strike",
            "BONUS:VAR|Pool_Unchained_Style_Strike|(MonkLVL-1)/4",
        );
        assert_bonus_token(
            "Unchained Monk ~ Stunning Fist",
            "BONUS:VAR|StunningFistMonkLVL|MonkLVL",
        );

        // SD-32 T12 row 21 cycle 2: Flurry of Blows' formulas live on a
        // separate `.MOD` block (raw `.lst` lines 492-504) that
        // `ingest_pu_classes.rs` used to read only the base row and silently
        // drop -- the exact `.MOD`-appended-row-loss defect row 21 fixed for
        // the generic `class_feature.rs` path, found live in THIS book's own
        // generator too and fixed here (the generator's token reader and its
        // declared-bonus-chain reader --
        // `pcgen_import::ingest_record::{token_pairs, bonus_chain_qualifiers}`
        // -- now read the full `.MOD` closure, matching
        // `out_of_record_formulas_are_byte_exact_against_the_real_lst_rows`
        // below, which independently pins the same raw `.lst` tokens). No
        // longer an honest absence -- the real tokens now ship.
        for token in [
            "BONUS:VAR|FlurryExtraAttacks|2+(Total_BAB>=11)+(Total_BAB>=6)+(Total_BAB>=11)+(Total_BAB>=16)",
            "BONUS:VAR|Total_BAB|BAB",
            "BONUS:VAR|FlurryAttacks|2+(Total_BAB>=6)+if(Total_BAB>=11,2,0)+(Total_BAB>=16)",
            "BONUS:VAR|FAB_1|FAB",
            "BONUS:VAR|FAB_2|FAB",
            "BONUS:VAR|FAB_3|FAB+if(FlurryAttacks==3,-5,0)",
            "BONUS:VAR|FAB_4|FAB-5",
            "BONUS:VAR|FAB_5|FAB-10",
            "BONUS:VAR|FAB_6|FAB-15",
            "BONUS:VAR|FAB_7|FAB",
        ] {
            assert_bonus_token("Unchained Monk ~ Flurry of Blows", token);
        }
    }

    fn assert_bonus_token(key: &str, token: &str) {
        let record = record_for(key);
        let tokens = bonus_tokens(&record);
        assert!(
            tokens.iter().any(|t| t == token),
            "{key} must carry the exact token {token:?}; it carries {tokens:?}"
        );
    }

    fn bonus_tokens(record: &serde_json::Value) -> Vec<String> {
        ingest_record::bonus_chain_qualifiers(record)
            .into_iter()
            .map(|parts| format!("BONUS:{}", parts.join("|")))
            .collect()
    }

    fn description_of(key: &str) -> String {
        record_for(key)["description"]
            .as_str()
            .unwrap_or_else(|| panic!("{key} must carry a rendered description"))
            .to_owned()
    }

    fn ingested_records() -> Vec<serde_json::Value> {
        let mut out = Vec::new();
        let dir = corpus_dir();
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Monk corpus dir {dir:?} must exist: {e}"));
        for entry in entries {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable corpus record");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("corpus record is valid JSON");
            out.push(value["data"].clone());
        }
        out
    }

    fn record_for(key: &str) -> serde_json::Value {
        ingested_records()
            .into_iter()
            .find(|r| r["key"] == key)
            .unwrap_or_else(|| panic!("no ingested record with KEY:{key}"))
    }

    fn corpus_dir() -> PathBuf {
        codex_ingest::repo_root()
            .join("data/corpus/pathfinder_unchained/class_feature/monk_unchained_class")
    }


}
