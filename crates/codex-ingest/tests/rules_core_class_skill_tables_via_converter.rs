//! class_skill_tables::class_skill_list_tests::class_skill_lists_match_their_own_corpus_records, E1-evicted from src/rules_core/rules_tables/crb/class_skill_tables.rs (SD-36 Epic A).

use codex_ingest::pcgen_import::ingest_record;

/// Every base-class row's own claim, re-derived from the LIVE corpus
/// record's own `CSKILL` token — not merely asserted in the table
/// above. RED if the corpus record ever changes its class-skill list.
#[test]
fn class_skill_lists_match_their_own_corpus_records() {
    let dir = codex_ingest::repo_root()
        .join("data/corpus/core_rulebook/class_feature/class_skills");
    let expectations: &[(&str, &str)] = &[
        ("class:barbarian", "Barbarian"),
        ("class:bard", "Bard"),
        ("class:cleric", "Cleric"),
        ("class:druid", "Druid"),
        ("class:fighter", "Fighter"),
        ("class:monk", "Monk"),
        ("class:paladin", "Paladin"),
        ("class:ranger", "Ranger"),
        ("class:rogue", "Rogue"),
    ];
    for (owner_id, class_name) in expectations {
        let row = codex::rules_core::rules_tables::crb::class_skill_tables::class_skill_list(owner_id)
            .unwrap_or_else(|| panic!("{owner_id} must be a real row in CLASS_SKILL_LISTS"));
        assert!(!row.all_skills, "{class_name} is a named list, not the ALL row");
        let mut found_file = false;
        for entry in std::fs::read_dir(&dir).expect("class_skills dir exists") {
            let entry = entry.expect("readable dir entry");
            let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
            let json: serde_json::Value =
                serde_json::from_str(&text).expect("valid corpus json");
            let key = json["data"]["key"].as_str().unwrap_or_default();
            if key != format!("Class Skills ~ {class_name}") {
                continue;
            }
            found_file = true;
            let cskill =
                ingest_record::first_token_value(&json, "CSKILL").unwrap_or_default();
            let expected: Vec<&str> = cskill.split('|').collect();
            // Rebuild the record's own token spelling from the typed
            // entries. This is both the transcription check it has always
            // been and, since SD-35 `AT-35-E6-003-SWEEP` cycle 7, the proof
            // that typing the family wildcard lost nothing: a `Family`
            // entry must reproduce the record's element exactly.
            let rebuilt: Vec<String> = row
                .skills
                .iter()
                .map(|entry| match entry {
                    codex::rules_core::rules_tables::crb::class_skill_tables::ClassSkillEntry::Named(name) => (*name).to_string(),
                    codex::rules_core::rules_tables::crb::class_skill_tables::ClassSkillEntry::Family(family) => format!("TYPE={family}"),
                })
                .collect();
            assert_eq!(rebuilt, expected, "{class_name} CSKILL list");
        }
        assert!(found_file, "no corpus record found for {class_name}");
    }
}

/// `"Jack of All Trades ~ Class Skills"`'s own `CSKILL:ALL` grant is a
/// different shape (no enumerable list) — verified separately.
#[test]
fn jack_of_all_trades_is_the_all_skills_row() {
    let row = codex::rules_core::rules_tables::crb::class_skill_tables::class_skill_list("class_feature:jack_of_all_trades")
        .expect("jack_of_all_trades must be a real row");
    assert!(row.all_skills);
    assert!(row.skills.is_empty());

    let path = codex_ingest::repo_root().join(
        "data/corpus/core_rulebook/class_feature/jack_of_all_trades/jack_of_all_trades_class_skills.json",
    );
    let text = std::fs::read_to_string(&path).expect("readable corpus json");
    let json: serde_json::Value = serde_json::from_str(&text).expect("valid corpus json");
    let cskill = ingest_record::first_token_value(&json, "CSKILL").unwrap_or_default();
    assert_eq!(cskill, "ALL");
}
