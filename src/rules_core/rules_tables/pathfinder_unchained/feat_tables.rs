//! Pathfinder Unchained (PU) feat catalog. SD-27 Cycle E2.2 per-book
//! pre-build (`docs/release/SD-27-future-state-book-content-ingestion/
//! loop-instruction.md §3.3.3`).
//!
//! **Full corpus coverage, honestly bounded.** `pu_feats.lst` has 18
//! non-comment, non-blank rows. 17 of them are real, distinct feat
//! definitions -- every one is represented below. The 18th
//! (`CATEGORY=FEAT|Extra Rogue Talent.MOD`) is a `.MOD` record that
//! modifies an existing Advanced Player's Guide feat ("Extra Rogue
//! Talent")'s prerequisite rather than defining a new feat of its own --
//! no `KEY:`/`DESC:`/`BENEFIT:` of its own, nothing to cache as a
//! record. Deliberately excluded, mirroring
//! `rules_tables::crb::feats`'s own documented precedent of excluding
//! `CATEGORY=...MOD` lines ("modify another feat's formula rather than
//! defining a new one").
//!
//! **Category is corpus-block-derived, not `TYPE:`-derived.** Unlike
//! CRB's `cr_feats.lst` (which carries a consistent `TYPE:General|Combat|
//! ItemCreation|Metamagic` facet on every record CRB's own catalog
//! includes), `pu_feats.lst`'s real `TYPE:` tokens are sparse and
//! inconsistent -- the 9 "Champion of ..." alignment feats carry NO
//! `TYPE:` token at all on their own record (their only relationship to
//! "Alignment" is a same-named facet buried inside a `PREABILITY:`
//! prerequisite clause, not a first-class `TYPE:` on the feat itself).
//! The corpus's own `###Block:` section markers (`Alignment Feats`,
//! `Stamina Feats`, `Wound Threshold Feats`) are the real, honest
//! grouping signal this book's corpus provides, so `FeatCategory` mirrors
//! those blocks directly rather than inventing a `TYPE:`-derived scheme
//! the corpus doesn't consistently support. The final 2 records
//! (`Extra Unchained Rogue Talent`, `Signature Skill`) sit outside any
//! named `###Block:` and DO carry a real `TYPE:General` token, so they
//! land in `General`.
//!
//! Every field below (`name`, `description`, `source_page`) is copied
//! verbatim from the real corpus row. `key == name` for every record here
//! -- no record in this catalog's 17 rows carries its own explicit
//! `KEY:` token (only the excluded `.MOD` record references a `KEY:`-less
//! `CATEGORY=FEAT|<name>.MOD` identity), the same fallback
//! `rules_tables::crb::feats::FeatTableEntry.key`'s own doc comment
//! documents.
//!
//! Real per-record LST path/sha256/line citations are computed by
//! `src/bin/sd27_gen_book_cache.rs` at generation time by reading the
//! live corpus file directly (never hand-transcribed here) -- this
//! module supplies only the compiled data values, per the same
//! generation discipline `src/bin/sd26_gen_core_rulebook_cache.rs`
//! already establishes ("never re-derives a data field's value from raw
//! LST at generation time; every value comes from the compiled Rust
//! module's own accessors").

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatCategory {
    /// The 9 "Champion of <alignment>" feats (`###Block: Alignment
    /// Feats`).
    Alignment,
    /// Combat Stamina, Extra Stamina, Push the Limits (`###Block:
    /// Stamina Feats`).
    CombatStamina,
    /// Critical Cure, Endurance, Twist the Knife (`###Block: Wound
    /// Threshold Feats`).
    WoundThreshold,
    /// Extra Unchained Rogue Talent, Signature Skill -- both carry a
    /// real `TYPE:General` token and sit outside any named `###Block:`.
    General,
}

impl FeatCategory {
    pub const ALL: &'static [FeatCategory] = &[
        FeatCategory::Alignment,
        FeatCategory::CombatStamina,
        FeatCategory::WoundThreshold,
        FeatCategory::General,
    ];

    /// This catalog's single corpus source file.
    pub fn corpus_file_name(self) -> &'static str {
        "pu_feats.lst"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatTableEntry {
    /// The corpus record identity (first column). No record in this
    /// catalog carries a distinct `KEY:` token, so `key == name` for
    /// every entry (see this module's own doc comment).
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim. Every one of the 17 real
    /// records in this catalog carries a `DESC:` token (unlike CRB's own
    /// catalog, which has 8 real gaps) -- `None` never actually occurs
    /// here today, but the type stays `Option` to match the established
    /// `rules_tables::crb::feats::FeatTableEntry.description` convention
    /// ("`None` when the record has no `DESC:` token") rather than
    /// asserting a non-optional field this book's corpus happens not to
    /// need yet.
    pub description: Option<&'static str>,
    /// The corpus `SOURCEPAGE:` token, verbatim (e.g. `"p.98"`,
    /// `"APG p.160"`). `None` for the one real record with no
    /// `SOURCEPAGE:` token at all (`Signature Skill`) -- an honest gap,
    /// not a fabricated page number.
    pub source_page: Option<&'static str>,
}

/// Full PU feat catalog: all 17 real, distinct corpus records, in source
/// order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [FeatTableEntry] {
    static TABLE: std::sync::OnceLock<Vec<FeatTableEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            FeatTableEntry {
                key: "Champion of Anarchy",
                category: FeatCategory::Alignment,
                name: "Champion of Anarchy",
                description: Some("You spread chaos wherever you go."),
                source_page: Some("p.98"),
            },
            FeatTableEntry {
                key: "Champion of Balance",
                category: FeatCategory::Alignment,
                name: "Champion of Balance",
                description: Some("You are dedicated to perfect balance in the multiverse."),
                source_page: Some("p.98"),
            },
            FeatTableEntry {
                key: "Champion of Destruction",
                category: FeatCategory::Alignment,
                name: "Champion of Destruction",
                description: Some("You would destroy the world if it were within your power."),
                source_page: Some("p.98"),
            },
            FeatTableEntry {
                key: "Champion of Freedom",
                category: FeatCategory::Alignment,
                name: "Champion of Freedom",
                description: Some("You believe that beings can thrive only when free."),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Champion of Grace",
                category: FeatCategory::Alignment,
                name: "Champion of Grace",
                description: Some("It is your mission to do as much good as possible."),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Champion of Malevolence",
                category: FeatCategory::Alignment,
                name: "Champion of Malevolence",
                description: Some("Things would be better if everyone just did as you wished."),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Champion of Righteousness",
                category: FeatCategory::Alignment,
                name: "Champion of Righteousness",
                description: Some(
                    "You know that good must be tempered with order if it's going to prevail in the long term.",
                ),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Champion of Tranquility",
                category: FeatCategory::Alignment,
                name: "Champion of Tranquility",
                description: Some("The harmony of law is your highest ideal."),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Champion of Tyranny",
                category: FeatCategory::Alignment,
                name: "Champion of Tyranny",
                description: Some("You must beat down the masses to have true order."),
                source_page: Some("p.99"),
            },
            FeatTableEntry {
                key: "Combat Stamina",
                category: FeatCategory::CombatStamina,
                name: "Combat Stamina",
                description: Some("You stop at nothing to drive your attack home."),
                source_page: Some("p.112"),
            },
            FeatTableEntry {
                key: "Extra Stamina",
                category: FeatCategory::CombatStamina,
                name: "Extra Stamina",
                description: Some("You can push yourself to higher limits."),
                source_page: Some("p.113"),
            },
            FeatTableEntry {
                key: "Push the Limits",
                category: FeatCategory::CombatStamina,
                name: "Push the Limits",
                description: Some(
                    "Even when suffering from fatigue, you can fight through and overcome incredible opposition.",
                ),
                source_page: Some("p.113"),
            },
            FeatTableEntry {
                key: "Critical Cure",
                category: FeatCategory::WoundThreshold,
                name: "Critical Cure",
                description: Some("Your healing is more effective if your patient is badly injured."),
                source_page: Some("P.137"),
            },
            FeatTableEntry {
                key: "Endurance",
                category: FeatCategory::WoundThreshold,
                name: "Endurance",
                description: Some("Harsh conditions or long exertions do not easily tire you."),
                source_page: Some("P.137"),
            },
            FeatTableEntry {
                key: "Twist the Knife",
                category: FeatCategory::WoundThreshold,
                name: "Twist the Knife",
                description: Some("You're especially dangerous against enemies who are suffering from injuries."),
                source_page: Some("P.137"),
            },
            FeatTableEntry {
                key: "Extra Unchained Rogue Talent",
                category: FeatCategory::General,
                name: "Extra Unchained Rogue Talent",
                description: Some("Through constant practice, you have learned how to perform a special trick."),
                source_page: Some("APG p.160"),
            },
            FeatTableEntry {
                key: "Signature Skill",
                category: FeatCategory::General,
                name: "Signature Skill",
                description: Some(
                    "Your ability with a particular skill is the stuff of legends, and you can do things with that skill that others cannot.",
                ),
                source_page: None,
            },
        ]
    })
}
