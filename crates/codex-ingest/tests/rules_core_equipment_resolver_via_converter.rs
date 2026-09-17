// -- split from `tests` in src/rules_core/equipment_resolver.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::equipment_resolver::*;
    use codex::rules_core::source_content::SourcePackageContent;
    use codex::rules_core::rules_tables::RuleSetId;
    use codex_ingest::pcgen_import::ir_converter::convert_equipment_record;
    use codex_ingest::pcgen_import::lst_parser::equipment::parse_equipment_entries;
    use codex::rules_core::source_content::SourceRef;

    /// Regression test: KEY-less records whose only distinguishing
    /// content is inside parentheses (e.g. the real corpus's
    /// "Improvised Weapon (1d2)" through "(2d10)" damage-die variants)
    /// must resolve to themselves exactly, not to whichever sibling the
    /// lossy normalized-name fallback happens to hit first.
    #[test]
    fn key_less_records_distinguished_only_by_parenthesized_content_resolve_exactly() {
        let text = "\
Improvised Weapon (1d2)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d3)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:1
Improvised Weapon (1d4)\tTYPE:Weapon.Melee.Improvised\tCOST:0\tWT:2
";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d3)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d3)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d3)");

        let (record, _) = equipment_id_resolve("Improvised Weapon (1d2)", RuleSetId::Crb, &corpus)
            .expect("expected 'Improvised Weapon (1d2)' to resolve");
        assert_eq!(record.name, "Improvised Weapon (1d2)");
    }

    /// A needle must resolve to the record whose corpus IDENTITY it is, never
    /// to a record that merely DISPLAYS that name while being identified as
    /// something else.
    ///
    /// Both orders are asserted, and that is the whole point: before the
    /// identity pass existed the answer was first-match-wins over the corpus in
    /// `read_dir` order, so which record answered was a property of the
    /// filesystem rather than of the data — stable on one machine, different in
    /// another checkout of the same corpus. Sorting that scan (which this cycle
    /// also did) flipped `core_rulebook:equipment:shoes` from
    /// `ingested-magnitude` to a FALSE `grounded`, because
    /// `equipment_key_is_wired` then read the modifier's tokens and reported a
    /// mechanical effect for an item that has none. Determinism alone would
    /// have frozen the wrong answer; this rule makes it the right one either
    /// way.
    #[test]
    fn a_needle_resolves_to_the_record_whose_identity_it_is_not_to_a_name_twin() {
        for (label, text) in [
            ("item first", format!("{SHOES_ITEM}{SHOES_MODIFIER}")),
            ("modifier first", format!("{SHOES_MODIFIER}{SHOES_ITEM}")),
        ] {
            let corpus = corpus_from(&text);
            let (record, _) = equipment_id_resolve("Shoes", RuleSetId::Crb, &corpus)
                .expect("expected 'Shoes' to resolve");
            assert_eq!(
                record.identity, "Shoes",
                "[{label}] 'Shoes' must resolve to the KEY-less item whose identity is 'Shoes', \
                 not to the modifier identified as \"Artisan's Tools (Shoes)\""
            );
            assert!(
                !record.is_modifier,
                "[{label}] resolved the wrong record: the EQUIP item, not the EQUIPMOD twin"
            );
        }
    }

    /// The other half of the same rule: the name-twin is still reachable, by
    /// its own identity. Fixing the collision must not make a real record
    /// unresolvable.
    #[test]
    fn the_name_twin_is_still_reachable_by_its_own_corpus_key() {
        for (label, text) in [
            ("item first", format!("{SHOES_ITEM}{SHOES_MODIFIER}")),
            ("modifier first", format!("{SHOES_MODIFIER}{SHOES_ITEM}")),
        ] {
            let corpus = corpus_from(&text);
            let (record, _) =
                equipment_id_resolve("Artisan's Tools (Shoes)", RuleSetId::Crb, &corpus)
                    .expect("expected the modifier to resolve by its own KEY");
            assert_eq!(
                record.identity, "Artisan's Tools (Shoes)",
                "[{label}] the modifier must still be reachable by its identity"
            );
        }
    }

    /// The widest instance of the same shape in the real corpus: CRB's
    /// `general/potion.json` (the empty flask, KEY-less) against fifty-odd
    /// `Potion of ...` / `Oil of ...` magic items that all display as `Potion`.
    /// Asking for `Potion` must yield the flask, not whichever potion the disk
    /// offered first.
    #[test]
    fn a_keyless_generic_wins_over_its_many_specific_name_twins() {
        let text = "\
Potion\tKEY:Potion of Fly\tTYPE:Magic.Potion\tCOST:750
Potion\tTYPE:Item.Potion\tCOST:0\tWT:0
Potion\tKEY:Potion of Blur\tTYPE:Magic.Potion\tCOST:300
";
        let corpus = corpus_from(text);
        let (record, _) = equipment_id_resolve("Potion", RuleSetId::Crb, &corpus)
            .expect("expected 'Potion' to resolve");
        assert_eq!(
            record.identity, "Potion",
            "'Potion' must resolve to the KEY-less flask whose identity is 'Potion'"
        );
        // ... and each specific potion stays reachable by its own identity.
        let (fly, _) = equipment_id_resolve("Potion of Fly", RuleSetId::Crb, &corpus)
            .expect("expected 'Potion of Fly' to resolve");
        assert_eq!(fly.identity, "Potion of Fly");
    }

    /// Control: the legacy `"item:longsword"`-style fixture namespace
    /// must still resolve via the normalized-name fallback, since it
    /// predates corpus-linkage and never matches the corpus's exact name.
    #[test]
    fn legacy_item_prefix_fixture_namespace_still_resolves_via_normalized_fallback() {
        let text = "Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\n";
        let corpus = corpus_from(text);

        let (record, _) = equipment_id_resolve("item:longsword", RuleSetId::Crb, &corpus)
            .expect("expected 'item:longsword' to resolve via the normalized fallback");
        assert_eq!(record.name, "Longsword");
    }

    const SHOES_MODIFIER: &str =
        "Shoes\tKEY:Artisan's Tools (Shoes)\tTYPE:EQMODARTISAN\tCOST:0\tVISIBLE:QUALITY\n";

    /// The two real CRB records behind the `Shoes` defect, as their `.lst`
    /// rows: the item (KEY-less, so its corpus identity is its name, and it
    /// carries no mechanical token) and the equipment MODIFIER that merely
    /// displays the same name while being identified as
    /// `Artisan's Tools (Shoes)`.
    const SHOES_ITEM: &str = "Shoes\tTYPE:Feet.Shoes\tCOST:0\tWT:0\tSLOTS:2\tMODS:REQUIRED\n";

    fn corpus_from(text: &str) -> SourcePackageContent<'static> {
        let result = parse_equipment_entries("test.lst", text);
        let source_ref = SourceRef {
            lst_file: "test.lst".to_string(),
            line: 1,
        };
        let mut corpus = SourcePackageContent::empty("test", source_ref);
        for record in result.entries {
            let record: &'static codex_ingest::pcgen_import::lst_parser::equipment::EquipmentRecord =
                Box::leak(Box::new(record));
            corpus.push(convert_equipment_record(record));
        }
        corpus
    }


}
