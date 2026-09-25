// -- split from `tests` in src/rules_core/rules_tables/pathfinder_unchained/rogue_features.rs (pcgen-touching items only) --
mod tests {
    use codex_ingest::pcgen_import::ingest_record;

    /// Debilitating Injury is the Unchained Rogue's headline feature and its
    /// corpus row (`:583`) carries **no** `BONUS:`, `DEFINE:` or any other
    /// formula token — its whole numeric content is three sentences. Both
    /// halves of that are checked here, because it is the fact the readings
    /// above depend on.
    #[test]
    fn debilitating_injury_carries_no_formula_token_only_prose() {
        let record = record_for("Unchained Rogue ~ Debilitating Injury");
        assert!(
            ingest_record::bonus_chain_qualifiers(&record).is_empty(),
            "Debilitating Injury must still carry no BONUS: chain -- if it gained one, the \
             prose-derived readings are no longer the only source and must be revisited"
        );
        let token_keys: Vec<String> =
            ingest_record::token_keys(&record).into_iter().map(str::to_owned).collect();
        assert_eq!(
            token_keys,
            vec!["KEY", "CATEGORY", "TYPE", "DESC"],
            "the row is a bare declaration plus prose; any new token changes where its numbers \
             come from"
        );

        let description = record["description"].as_str().expect("Debilitating Injury carries a DESC:");
        for sentence in [
            "causing it to take a penalty for 1 round",
            "The target becomes bewildered, taking a -2 penalty to AC.",
            "The target takes an additional -2 penalty to AC against all attacks made by the rogue.",
            "At 10th level and 16th level, the penalty to AC against attacks made by the rogue \
             increases by -2 (to a total maximum of -8).",
        ] {
            assert!(
                description.contains(sentence),
                "Debilitating Injury prose changed; its readings must be re-derived.\n\
                 expected to contain: {sentence}\ncorpus says: {description}"
            );
        }
    }

    /// `Unchained Rogue ~ Evasion` (`:584`) is the one Unchained Rogue record
    /// this book leaves genuinely empty: no `DESC:`, no `BONUS:`, no
    /// `DEFINE:`. It `SERVESAS` the shared Core Rulebook `Rogue ~ Evasion`
    /// record and grants it outright, so the magnitude — such as it is —
    /// belongs to that record and not to this book.
    ///
    /// Pinned so nobody later "fixes" it by copying the Unchained *Monk's*
    /// Evasion prose across, which would attribute a sentence to a row that
    /// does not carry it.
    #[test]
    fn rogue_evasion_delegates_to_the_shared_core_rulebook_record() {
        let record = record_for("Unchained Rogue ~ Evasion");
        assert!(
            record["description"].as_str().unwrap_or("").is_empty(),
            "the row carries no DESC: of its own"
        );
        assert!(
            ingest_record::bonus_chain_qualifiers(&record).is_empty(),
            "the row carries no BONUS: chain"
        );
        let tokens: Vec<(String, String)> = ingest_record::token_pairs(&record)
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        assert!(
            tokens.contains(&(
                "SERVESAS".to_owned(),
                "ABILITY=Special Ability|Rogue ~ Evasion".to_owned()
            )),
            "the row must still delegate to the shared Core Rulebook record; it carries {tokens:?}"
        );
    }

    /// The ingested `data` block for one Unchained Rogue record, read off
    /// disk. The prose-derived readings in this module are checked against
    /// this rather than against a copy of the sentence kept here — a pin that
    /// quotes itself pins nothing.
    fn record_for(key: &str) -> serde_json::Value {
        let dir = codex_ingest::repo_root()
            .join("data/corpus/pathfinder_unchained/class_feature/rogue_unchained_class");
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Rogue corpus dir {dir:?} must exist: {e}"));
        for entry in entries {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable corpus record");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("corpus record is valid JSON");
            if value["data"]["key"] == key {
                return value["data"].clone();
            }
        }
        panic!("no ingested record with KEY:{key}");
    }


}
