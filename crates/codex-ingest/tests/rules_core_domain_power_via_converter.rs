// -- split from `fixture_check_tests` in src/rules_core/pilot_compute/domain_power.rs (pcgen-touching items only) --
mod fixture_check_tests {
    use codex::rules_core::pilot_compute::domain_power::*;
    use codex::rules_core::pilot_compute::{GOOD_DOMAIN_SELECTION, WAR_DOMAIN_SELECTION, STRENGTH_DOMAIN_SELECTION, DESTRUCTION_DOMAIN_SELECTION, GLORY_DOMAIN_SELECTION, UNDEAD_SUBDOMAIN_SELECTION, CONSTRUCT_SUBDOMAIN_SELECTION};
    use codex_ingest::pcgen_import::ingest_record;

    /// Guarantee 1/2's structural half: confirms `domain_power_env`'s core
    /// assumption -- "any `LVL`-suffixed variable resolves to the granting
    /// class's own level, with no per-domain offset" -- is what the corpus
    /// ACTUALLY states for every domain this module grounds, rather than an
    /// assumption carried over from Good alone. Also confirms the
    /// uses-per-day chain (`Domain<X>Times|DomainPowerTimes|TYPE=Domain`)
    /// for all five. SD-31 wave 26 widened this from three (Good/War/
    /// Strength) to five (+Destruction/Glory) -- same assertion, more
    /// domains, per this lane's "prove before you extend" requirement.
    #[test]
    fn catalog_domain_headers_share_the_domainlvl_and_domainpowertimes_chain() {
        for (json, domain) in [
            (GOOD_HEADER_JSON, "Good"),
            (WAR_HEADER_JSON, "War"),
            (STRENGTH_HEADER_JSON, "Strength"),
            (DESTRUCTION_HEADER_JSON, "Destruction"),
            (GLORY_HEADER_JSON, "Glory"),
        ] {
            let doc = parse(json);
            let bonuses = bonus_values(&doc);
            assert!(
                bonuses.iter().any(|b| {
                    b.starts_with("VAR|Domain") && b.ends_with("LVL|DomainLVL|TYPE=Domain")
                }),
                "{domain} domain header must chain its own LVL var to the shared DomainLVL: {bonuses:?}"
            );
            assert!(
                bonuses.iter().any(|b| {
                    b.starts_with("VAR|Domain")
                        && b.ends_with("Times|DomainPowerTimes|TYPE=Domain")
                }),
                "{domain} domain header must chain its own Times var to the shared \
                 DomainPowerTimes: {bonuses:?}"
            );
        }
    }

    /// The shared uses-per-day formula itself: [`DOMAIN_POWER_TIMES_FORMULA`]
    /// must be byte-for-byte what `domains.json`'s own `BONUS:VAR|DomainPowerTimes|…`
    /// token states, not a hand-recalled `"3+WIS"`.
    #[test]
    fn domain_power_times_formula_constant_is_byte_identical_to_the_corpus() {
        let doc = parse(DOMAINS_HEADER_JSON);
        let bonuses = bonus_values(&doc);
        let corpus_formula = bonuses
            .iter()
            .find_map(|b| b.strip_prefix("VAR|DomainPowerTimes|"))
            .expect("domains.json must carry a BONUS:VAR|DomainPowerTimes| token");
        assert_eq!(
            corpus_formula, DOMAIN_POWER_TIMES_FORMULA,
            "DOMAIN_POWER_TIMES_FORMULA must match the corpus's own DomainPowerTimes formula \
             byte-for-byte"
        );
    }

    /// Each catalog entry's own `magnitude_formula` must be byte-for-byte
    /// what the granted-power record's `DESC` token embeds as its FIRST
    /// formula segment (PCGen's own `%1` substitution slot) -- not a
    /// hand-recalled or hand-simplified rewrite.
    #[test]
    fn granted_power_magnitude_formulas_are_byte_identical_to_the_corpus() {
        for (json, selection_id) in [
            (TOUCH_OF_GOOD_JSON, GOOD_DOMAIN_SELECTION),
            (BATTLE_RAGE_JSON, WAR_DOMAIN_SELECTION),
            (STRENGTH_SURGE_JSON, STRENGTH_DOMAIN_SELECTION),
            (DESTRUCTIVE_SMITE_JSON, DESTRUCTION_DOMAIN_SELECTION),
            (TOUCH_OF_GLORY_JSON, GLORY_DOMAIN_SELECTION),
            (DEATH_S_KISS_JSON, UNDEAD_SUBDOMAIN_SELECTION),
            (ANIMATE_SERVANT_JSON, CONSTRUCT_SUBDOMAIN_SELECTION),
        ] {
            let doc = parse(json);
            let desc = ingest_record::first_token_value(&doc, "DESC")
                .expect("a DESC token")
                .to_owned();
            let first_formula_segment = desc
                .split('|')
                .nth(1)
                .expect("DESC must carry at least one %N formula segment after the description text");
            let spec = resolve_domain_power(selection_id).expect("must be catalogued");
            assert_eq!(
                first_formula_segment, spec.magnitude_formula,
                "{}'s magnitude_formula must match the corpus DESC's own %1 formula segment \
                 byte-for-byte",
                spec.domain_display_name
            );
        }
    }

    const WAR_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/war/war.json");

    const DOMAINS_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/domains/domains.json");

    const STRENGTH_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/strength/strength.json");

    const DESTRUCTION_HEADER_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/destruction/destruction.json"
    );

    const GOOD_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/good/good.json");

    const GLORY_HEADER_JSON: &str =
        include_str!("../../../data/corpus/core_rulebook/class_feature/glory/glory.json");

    /// Every `BONUS` token value on a corpus record, in file order.
    fn bonus_values(doc: &serde_json::Value) -> Vec<String> {
        ingest_record::token_values(doc, "BONUS").into_iter().map(str::to_owned).collect()
    }

    const ANIMATE_SERVANT_JSON: &str = include_str!(
        "../../../data/corpus/advanced_players_guide/class_feature/construct_subdomain/animate_servant.json"
    );

    const TOUCH_OF_GOOD_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/touch_of_good.json"
    );

    const BATTLE_RAGE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/battle_rage.json"
    );

    fn parse(json: &str) -> serde_json::Value {
        serde_json::from_str(json).expect("committed corpus JSON must parse")
    }

    const DESTRUCTIVE_SMITE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/destructive_smite.json"
    );

    const TOUCH_OF_GLORY_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/touch_of_glory.json"
    );

    const DEATH_S_KISS_JSON: &str = include_str!(
        "../../../data/corpus/advanced_players_guide/class_feature/undead_subdomain/death_s_kiss.json"
    );

    const STRENGTH_SURGE_JSON: &str = include_str!(
        "../../../data/corpus/core_rulebook/class_feature/domain_power/strength_surge.json"
    );


}
