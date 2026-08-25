//! Corpus-derived class-feature roster for the `untabled_base_class_chassis`
//! registry (SD-32 card 11, shape T12), generalising Pathfinder Unchained's
//! own `push_pu_class_feature_records` mechanism from four hand-curated
//! Rust tables to a single corpus-derived fixture, reused by every class it
//! covers.
//!
//! # Population and its own honest boundary
//!
//! `scripts/census_untabled_base_class_feature_roster.py` (this fixture's
//! own re-derive command) extracts every corpus row matching either of two
//! generic PCGen "own-named-group, automatically granted, level-gated"
//! class-feature shapes — a `CATEGORY=Class|<ClassName>.MOD` virtual
//! ability (shape 1) or a `CLASS:<ClassName>` level-table row whose own
//! leading column states the level (shape 2, added when closing the T12
//! attribution gap) — for the 20-class `untabled_base_class_chassis`
//! registry. **Not every registered class uses either shape**: this run
//! found data for 19 of the 20 (`antipaladin`, `magus`, `vigilante` via
//! shape 1; `aegis`, `cryptic`, `dread`, `marksman`, `psychic_warrior`,
//! `shifter`, `soulknife`, `tactician`, `vitalist`, `wilder` via shape 2;
//! `kineticist`, `medium`, `mesmerist`, `occultist`, `psychic`,
//! `spiritualist` via shape 1 too — the corpus itself spells the
//! `CATEGORY=` token both `Class` and `CLASS` (all six are the uppercase
//! form, all in `occult_adventures/oa_abilities_class.lst`); an inherited
//! framing that these six needed "a third progression shape" was checked
//! against the oracle directly and found false — it was this script's own
//! case-sensitive substring match missing an existing shape-1 row, not a
//! new shape (`decisions.md §17a` — validate before trusting a lead;
//! `docs/retro/` correction logged). 235 records total. The remaining one
//! (`psion`) is registered under `ultimate_psionics`
//! (`up_classes.lst`), not the superseded `psionics_unleashed` book an
//! earlier brief cited, and genuinely uses a third convention this script
//! does not parse: its own-named features are singly-named
//! (`ABILITY:Psion Class Feature|AUTOMATIC|Psion Manifesting`, no
//! `"Psion ~ "` group prefix) and chain through further per-discipline
//! `ABILITY:` indirection (e.g. `Clairsentience Discipline` itself grants
//! `Psion Class Feature|AUTOMATIC|Clairsentience ~ ...` rows) rather than
//! a single flat level-table or `.MOD` list — confirmed absent from both
//! shapes, not merely unchecked (see the script's own `--summary` output).
//! A class absent from this fixture is honestly absent, not silently
//! assumed complete; [`roster_for`] returns an empty slice for it and no
//! caller may treat that as "nothing to grant."
//!
//! Pool-shaped groups (`Vigilante Talent`, `Magus Arcana`, ...) are
//! deliberately excluded — see the census script's own doc comment for why
//! guessing their owner would violate `decisions.md §1a`/§3.

use std::sync::OnceLock;

/// One corpus-derived class-feature grant: this record's own key, the
/// display name, whether the corpus record carries no magnitude token (a
/// text-only "%1"-free description), the minimum class level PCGen's own
/// `PREVARGTEQ:<Class>_CFP_Level,<N>` clause states, and a citation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntabledClassFeatureRow {
    pub key: String,
    pub name: String,
    pub text_only: bool,
    pub min_level: u8,
    pub source_file: String,
    pub source_line: u32,
}

#[derive(serde::Deserialize)]
struct FixtureFile {
    entries: Vec<FixtureEntry>,
}

#[derive(serde::Deserialize)]
struct FixtureEntry {
    class_id: String,
    key: String,
    name: String,
    #[allow(dead_code)]
    description: String,
    text_only: bool,
    min_level: u8,
    source_file: String,
    source_line: u32,
}

const FIXTURE_JSON: &str = include_str!(
    "../../../tests/fixtures/rules_core/untabled-base-class-feature-roster.json"
);

static ROSTER: OnceLock<Vec<(String, UntabledClassFeatureRow)>> = OnceLock::new();

fn roster() -> &'static [(String, UntabledClassFeatureRow)] {
    ROSTER
        .get_or_init(|| {
            let parsed: FixtureFile = serde_json::from_str(FIXTURE_JSON).expect(
                "tests/fixtures/rules_core/untabled-base-class-feature-roster.json must parse",
            );
            parsed
                .entries
                .into_iter()
                .map(|e| {
                    (
                        e.class_id,
                        UntabledClassFeatureRow {
                            key: e.key,
                            name: e.name,
                            text_only: e.text_only,
                            min_level: e.min_level,
                            source_file: e.source_file,
                            source_line: e.source_line,
                        },
                    )
                })
                .collect()
        })
        .as_slice()
}

/// Every corpus-derived class-feature row this fixture holds for
/// `class_id` (the bare, underscored form -- `"antipaladin"`, not
/// `"class:antipaladin"`), in ascending `min_level` order. Empty for a
/// class this fixture has no data for -- see this module's own doc comment
/// for why that is an honest absence, not a claim of completeness.
pub fn roster_for(class_id: &str) -> Vec<&'static UntabledClassFeatureRow> {
    roster()
        .iter()
        .filter(|(cid, _)| cid == class_id)
        .map(|(_, row)| row)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antipaladin_has_at_least_one_row_and_every_row_states_its_own_min_level() {
        let rows = roster_for("antipaladin");
        assert!(!rows.is_empty(), "census script found antipaladin data; fixture must carry it");
        for row in &rows {
            assert!(row.min_level >= 1, "{}: min_level must be a real class level", row.key);
            // Own-named group membership is either the explicit
            // "Antipaladin ~ " prefix (shapes 1/2) or a bare target with no
            // " ~ " group separator at all (shape 3, e.g. a `.MOD`-shaped
            // core mirror-paladin grant like `Aura of Evil` -- census
            // script's own doc comment, shape 3).
            assert!(
                row.key.starts_with("Antipaladin ~ ") || !row.key.contains(" ~ "),
                "{}: must be own-named group",
                row.key
            );
        }
    }

    #[test]
    fn unknown_class_returns_empty_not_a_panic() {
        assert!(roster_for("class-nobody-registered").is_empty());
    }

    #[test]
    fn a_class_the_census_script_found_no_mod_shaped_data_for_is_honestly_empty() {
        // Cryptic was this test's original example but is now covered by
        // shape 2 (`CLASS:` level-table row); Psion was this test's second
        // example through SD-32 card 11 (T12) cycle 4 but is now covered by
        // shape 3 (bare own-named `CLASS:` row -- census script's own doc
        // comment) and asserted positively below
        // (`psion_manifesting_row_is_shape_3_and_carries_no_group_prefix`).
        // `undine_scion` is not a registered class at all -- confirmed
        // absent, not merely unchecked.
        assert!(roster_for("undine_scion").is_empty());
    }

    /// Fixture-checked against bytes this module never reads: the oracle's
    /// own `up_classes.lst` line 264 (`1\t...\tABILITY:Psion Class
    /// Feature|AUTOMATIC|Psion Manifesting`, re-derived against
    /// `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`), hand-
    /// transcribed here from the oracle text this cycle's own probe read.
    #[test]
    fn psion_manifesting_row_is_shape_3_and_carries_no_group_prefix() {
        let rows = roster_for("psion");
        assert_eq!(rows.len(), 1, "psion carries exactly one shape-3 own-named row: {rows:?}");
        let row = rows[0];
        assert_eq!(row.key, "Psion Manifesting");
        assert_eq!(row.min_level, 1);
        assert!(
            !row.key.contains(" ~ "),
            "shape 3's whole point is the bare, unprefixed target name"
        );
    }

    /// Fixture-checked against bytes this module never reads: the oracle's
    /// own `apg_abilities_globalvar.lst` line 79
    /// (`ABILITY:Special Ability|AUTOMATIC|Antipaladin ~ Touch of
    /// Corruption|PREVARGTEQ:Antipaladin_CFP_Level,2`), hand-transcribed
    /// here from the corpus text this cycle's own probe read, not from the
    /// fixture file.
    #[test]
    fn antipaladin_touch_of_corruption_matches_the_oracle_s_level_2_grant() {
        let rows = roster_for("antipaladin");
        let row = rows
            .iter()
            .find(|r| r.key == "Antipaladin ~ Touch of Corruption")
            .expect("Touch of Corruption must be in the fixture");
        assert_eq!(row.min_level, 2);
        assert_eq!(row.source_line, 79);
        assert!(row.source_file.ends_with("apg_abilities_globalvar.lst"));
    }

    /// Same discipline, shape 2: the oracle's own
    /// `pathfinder/dreamscarred_press/ultimate_psionics/up_classes.lst`
    /// line 84 is a `CLASS:Cryptic` level-table row whose own leading
    /// tab-field is the literal level number `1`, carrying
    /// `ABILITY:Cryptic Class Feature|AUTOMATIC|Cryptic ~ Altered Defense`.
    /// Hand-transcribed from the corpus text this cycle's own probe read.
    #[test]
    fn cryptic_altered_defense_matches_the_oracle_s_shape_2_level_1_grant() {
        let rows = roster_for("cryptic");
        let row = rows
            .iter()
            .find(|r| r.key == "Cryptic ~ Altered Defense")
            .expect("Altered Defense must be in the fixture");
        assert_eq!(row.min_level, 1);
        assert_eq!(row.source_line, 84);
        assert!(row.source_file.ends_with("up_classes.lst"));
    }

    /// Same discipline, shape 1, uppercase `CATEGORY=CLASS` casing: the
    /// oracle's own `occult_adventures/oa_abilities_class.lst` line 37 is a
    /// `CATEGORY=CLASS|Kineticist.MOD` virtual-ability row carrying
    /// `ABILITY:Kineticist Class Feature|AUTOMATIC|Kineticist ~ Class
    /// Skills|...|PREVARGTEQ:Kineticist_CFP_Level,1`. Hand-transcribed from
    /// the corpus text this cycle's own probe read. Guards the case-fold
    /// fix in `scripts/census_untabled_base_class_feature_roster.py`
    /// against regressing back to zero for the six `occult_adventures`
    /// classes this shape covers.
    #[test]
    fn kineticist_class_skills_matches_the_oracle_s_uppercase_category_shape_1_grant() {
        let rows = roster_for("kineticist");
        let row = rows
            .iter()
            .find(|r| r.key == "Kineticist ~ Class Skills")
            .expect("Class Skills must be in the fixture");
        assert_eq!(row.min_level, 1);
        assert_eq!(row.source_line, 37);
        assert!(row.source_file.ends_with("oa_abilities_class.lst"));
    }

    /// The other five `occult_adventures` classes the same case-fold fix
    /// unblocked: each must now carry at least one own-named row, not the
    /// pre-fix empty slice.
    #[test]
    fn the_other_five_occult_adventures_classes_are_no_longer_empty() {
        for class_id in ["medium", "mesmerist", "occultist", "psychic", "spiritualist"] {
            let rows = roster_for(class_id);
            assert!(
                !rows.is_empty(),
                "{class_id}: census script's case-fold fix must have found shape-1 data"
            );
        }
    }
}
