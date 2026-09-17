// -- split from `tests` in src/rules_core/rules_tables/pathfinder_unchained/barbarian_features.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::rules_tables::pathfinder_unchained::barbarian_features::*;
    use codex_ingest::pcgen_import::ingest_record;
    use std::path::PathBuf;

    /// `Unchained Barbarian ~ Tireless Rage` (`:295`) carries no `BONUS:` or
    /// `DEFINE:` token at all — its only number is the "1 minute" in its
    /// `DESC:`. Both halves of that claim are checked here.
    #[test]
    fn tireless_rage_lockout_is_the_one_minute_its_prose_states() {
        use prose_derived::tireless_rage_temporary_hit_point_lockout_rounds as lockout;
        for level in 0..TIRELESS_RAGE_LEVEL {
            assert_eq!(lockout(level), None, "level {level}");
        }
        for level in TIRELESS_RAGE_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(lockout(level), Some(10), "level {level}");
        }

        let record = record_for("Unchained Barbarian ~ Tireless Rage");
        assert!(
            bonus_tokens(&record).is_empty(),
            "Tireless Rage must still carry no BONUS: token -- if it gained one, the \
             prose-derived reading is no longer the only source and must be revisited"
        );
        let description = record["description"].as_str().expect("Tireless Rage carries a DESC:");
        assert!(
            description.contains(
                "If you enters a rage again within 1 minute of ending a rage, you don't gain any \
                 temporary hit points from your rage."
            ),
            "Tireless Rage prose changed; the 1-minute lockout reading must be re-derived. \
             Corpus says: {description}"
        );
    }

    /// Every level-scaling magnitude modelled here must be traceable to a
    /// `BONUS:` token that the ingested corpus record actually carries.
    /// This is the §24 compliance check: transcription, verified.
    #[test]
    fn every_modelled_formula_is_byte_exact_against_the_ingested_corpus_record() {
        assert_bonus_token(
            "Unchained Barbarian ~ Rage Powers",
            "BONUS:ABILITYPOOL|Unchained Rage Power|RagePowersLVL/2",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Rage Powers",
            "BONUS:VAR|RagePowersLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Danger Sense",
            "BONUS:VAR|TrapSenseBonus|BarbarianTrapSenseLVL/3",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Danger Sense",
            "BONUS:VAR|BarbarianTrapSenseLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Damage Reduction",
            "BONUS:VAR|BarbarianDR|(BarbarianDRLVL-4)/3",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Damage Reduction",
            "BONUS:VAR|BarbarianDRLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Fast Movement",
            "BONUS:MOVEADD|TYPE=Walk|10|PREVARLT:ENCUMBERANCE,2,var(\"COUNT[EQTYPE.ARMOR.EQUIPPED.IS.HEAVY]\"),1",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Rage",
            "BONUS:VAR|RageLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,5|PREVAREQ:Barbarian_CF_ImprovedUncannyDodge,0",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeFlankingLevel|BarbarianLVL|TYPE=EachClass.REPLACE|PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0",
        );

        // Indomitable Will has no BONUS token at all -- its magnitude is on
        // the row's ASPECT. Pinned separately, so that "no BONUS token" is a
        // checked fact rather than an omission.
        let indomitable = record_for("Unchained Barbarian ~ Indomitable Will");
        assert!(
            bonus_tokens(&indomitable).is_empty(),
            "Indomitable Will carries no BONUS chain in the corpus"
        );
        let aspect = ingest_record::first_token_value(&indomitable, "ASPECT")
            .expect("Indomitable Will must carry an ASPECT token");
        assert_eq!(
            aspect,
            "SaveBonus|+4 bonus to Will saves vs. Enchantment spells while raging"
        );
    }

    fn assert_bonus_token(key: &str, token: &str) {
        let record = record_for(key);
        let tokens = bonus_tokens(&record);
        assert!(
            tokens.iter().any(|t| t == token),
            "{key} must carry the exact token {token:?}; it carries {tokens:?}"
        );
    }

    /// The `BONUS:` chain qualifiers of one record, rejoined with `|` so
    /// they can be compared against the literal source token.
    fn bonus_tokens(record: &serde_json::Value) -> Vec<String> {
        ingest_record::bonus_chain_qualifiers(record)
            .into_iter()
            .map(|parts| format!("BONUS:{}", parts.join("|")))
            .collect()
    }

    fn corpus_dir() -> PathBuf {
        codex_ingest::repo_root()
            .join("data/corpus/pathfinder_unchained/class_feature/barbarian_unchained_class")
    }

    /// Every `data` block under the ingested Barbarian feature directory.
    fn ingested_records() -> Vec<serde_json::Value> {
        let mut out = Vec::new();
        let dir = corpus_dir();
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Barbarian corpus dir {dir:?} must exist: {e}"));
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


}
