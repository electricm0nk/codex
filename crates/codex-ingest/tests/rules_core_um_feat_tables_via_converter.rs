// -- split from `tests` in src/rules_core/rules_tables/ultimate_magic/feat_tables.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::rules_tables::ultimate_magic::feat_tables::*;
    use codex_ingest::pcgen_import::feat_effect_tokens::um_feat_carries_effect;
    use codex::rules_core::rules_tables::RuleSetId;

    #[test]
    fn every_record_carries_real_content() {
        for (i, e) in feat_tables().iter().enumerate() {
            assert!(
                e.description.is_some() || e.benefit.is_some() || carries_effect(i, e),
                "{} has no DESC:, BENEFIT:, or BONUS: -- genuinely empty, should have been excluded",
                e.key
            );
        }
    }

    #[test]
    fn no_record_is_deferred() {
        assert_eq!(
            feat_tables()
                .iter()
                .enumerate()
                .filter(|(i, e)| e.description.is_none()
                    && e.benefit.is_none()
                    && !carries_effect(*i, e))
                .count(),
            0
        );
    }

    #[test]
    fn the_desc_benefit_effect_split_is_the_real_one() {
        let both = feat_tables()
            .iter()
            .filter(|e| e.description.is_some() && e.benefit.is_some())
            .count();
        let desc_only = feat_tables()
            .iter()
            .filter(|e| e.description.is_some() && e.benefit.is_none())
            .count();
        let benefit_only = feat_tables()
            .iter()
            .filter(|e| e.benefit.is_some() && e.description.is_none())
            .count();
        let effect_only = feat_tables()
            .iter()
            .enumerate()
            .filter(|(i, e)| carries_effect(*i, e)
                && e.description.is_none()
                && e.benefit.is_none())
            .count();
        assert_eq!(both, 123, "records with both DESC: and BENEFIT:");
        assert_eq!(desc_only, 15, "the 15 Masterpiece feats, DESC:-complete by design");
        assert_eq!(benefit_only, 2, "Greater Wild Empathy, Versatile Channeler");
        assert_eq!(effect_only, 4, "Extra Cantrips or Orisons, Extra Evolution, Extra Summons, Transfer Feat to Familiar");
        assert_eq!(both + desc_only + benefit_only + effect_only, 144);
    }

    /// The presence flag these tests used to read off `UmFeatEntry.effect`,
    /// now read from the relocated table — SD-35 `AT-35-E6-003-SWEEP`
    /// cycle 6. The assertions are unchanged; only where the flag comes
    /// from moved.
    fn carries_effect(index: usize, entry: &UmFeatEntry) -> bool {
        um_feat_carries_effect(RuleSetId::Um, index, entry.key)
    }


}
