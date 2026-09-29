// -- split from `class_armor_proficiency_tests` in src/rules_core/rules_tables/crb/weapon_tables.rs (pcgen-touching items only) --
mod class_armor_proficiency_tests {
    use codex::rules_core::rules_tables::crb::weapon_tables::*;
    use codex_ingest::pcgen_import::ingest_record;

    /// Every row's own claim, re-derived from the LIVE corpus record's own
    /// `ABILITY` tokens -- not merely asserted in the table above. RED if
    /// the corpus record ever changes which armor/shield tiers it grants.
    #[test]
    fn class_armor_proficiencies_match_their_own_corpus_records() {
        let dir = codex_ingest::repo_root()
            .join("data/corpus/core_rulebook/class_feature/weapon_and_armor_proficiency");
        let expectations: &[(&str, &str)] = &[
            ("class:bard", "Bard"),
            ("class:fighter", "Fighter"),
            ("class:paladin", "Paladin"),
            ("class:ranger", "Ranger"),
            ("class:rogue", "Rogue"),
        ];
        for (class_id, class_name) in expectations {
            let row = class_armor_proficiency(class_id)
                .unwrap_or_else(|| panic!("{class_id} must be a real row in CLASS_ARMOR_PROFICIENCIES"));
            let mut found_file = false;
            for entry in std::fs::read_dir(&dir).expect("weapon_and_armor_proficiency dir exists") {
                let entry = entry.expect("readable dir entry");
                let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
                let json: serde_json::Value =
                    serde_json::from_str(&text).expect("valid corpus json");
                let key = json["data"]["key"].as_str().unwrap_or_default();
                if key != format!("Weapon and Armor Proficiency ~ {class_name}") {
                    continue;
                }
                found_file = true;
                let ability_tokens: Vec<String> = ingest_record::token_values(&json, "ABILITY")
                    .into_iter()
                    .map(str::to_string)
                    .collect();
                let has = |needle: &str| ability_tokens.iter().any(|v| v.contains(needle));
                assert_eq!(has("Armor Prof ~ Light"), row.light, "{class_name} light armor");
                assert_eq!(has("Armor Prof ~ Medium"), row.medium, "{class_name} medium armor");
                assert_eq!(has("Armor Prof ~ Heavy"), row.heavy, "{class_name} heavy armor");
                assert_eq!(has("Shield Prof ~ Tower"), row.tower_shield, "{class_name} tower shield");
                // "Shield Prof" alone (not "Shield Prof ~ Tower") is the
                // Buckler/Light/Heavy-shield grant -- must be checked
                // without matching the Tower variant's own substring.
                let has_plain_shield_prof = ability_tokens
                    .iter()
                    .any(|v| v.split('|').any(|part| part == "Shield Prof"));
                assert_eq!(has_plain_shield_prof, row.shield, "{class_name} shield (non-tower)");
            }
            assert!(found_file, "no corpus record found for {class_name}");
        }
    }


}
