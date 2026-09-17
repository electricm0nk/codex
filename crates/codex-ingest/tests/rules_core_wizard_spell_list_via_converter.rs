// -- split from `tests` in src/rules_core/rules_tables/crb/wizard_spell_list.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::rules_tables::crb::wizard_spell_list::*;
    use codex_ingest::pcgen_import::ingest_record;

    /// `AT-34-E3-001` wizard-opposition-school-spell-tracking sub-cause:
    /// proves [`wizard_school_zero_level_spells`] byte-for-byte against
    /// the 9 real, committed `"<School> Wizard Spells"` corpus records
    /// under `data/corpus/core_rulebook/class_feature/` -- not merely
    /// asserted in a doc comment. RED if either source table (or the
    /// corpus itself) ever drifts, which is exactly when this join must
    /// be revisited.
    #[test]
    fn wizard_school_zero_level_spells_matches_the_real_corpus_records() {
        use codex::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
        let repo_root = codex_ingest::repo_root();
        let cases: &[(&str, Pf1SchoolId)] = &[
            ("abjuration_wizard_spells", Pf1SchoolId::Abjuration),
            ("conjuration_wizard_spells", Pf1SchoolId::Conjuration),
            ("divination_wizard_spells", Pf1SchoolId::Divination),
            ("enchantment_wizard_spells", Pf1SchoolId::Enchantment),
            ("evocation_wizard_spells", Pf1SchoolId::Evocation),
            ("illusion_wizard_spells", Pf1SchoolId::Illusion),
            ("necromancy_wizard_spells", Pf1SchoolId::Necromancy),
            ("transmutation_wizard_spells", Pf1SchoolId::Transmutation),
            ("universal_wizard_spells", Pf1SchoolId::Universal),
        ];
        for (dir, school) in cases {
            let path = repo_root
                .join("data/corpus/core_rulebook/class_feature")
                .join(dir)
                .join(format!("{dir}.json"));
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("readable corpus json at {path:?}: {e}"));
            let json: serde_json::Value =
                serde_json::from_str(&text).expect("valid corpus json");
            let spellknown = ingest_record::first_token_value(&json, "SPELLKNOWN")
                .unwrap_or_else(|| panic!("{dir} carries a SPELLKNOWN token"));
            // `CLASS|Wizard=0|Spell One,Spell Two`
            let corpus_spells: Vec<&str> = spellknown
                .split('|')
                .nth(2)
                .unwrap_or_else(|| panic!("{dir}'s SPELLKNOWN token has a spell-list segment"))
                .split(',')
                .map(str::trim)
                .collect();
            let mut expected = corpus_spells.clone();
            expected.sort_unstable();
            let actual = wizard_school_zero_level_spells(*school);
            assert_eq!(
                actual, expected,
                "{dir}: engine join disagrees with the real corpus SPELLKNOWN token"
            );
        }
    }


}
