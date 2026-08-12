//! ARG feat catalog.
//!
//! Full in-scope corpus coverage: every real corpus record from
//! `arg_feats.lst` carrying `CATEGORY:FEAT` and a classifiable `TYPE:`
//! facet -- 187 total (132 General,
//! 52 Combat, 3 Teamwork). Generated
//! programmatically from the live corpus -- see `feat_data/`'s own doc
//! comment for the generation method (not hand-authored).
//!
//! **Real, re-verified count differs from the scoping brief's rough
//! estimate of 239.** `arg_feats.lst` carries 239 real non-comment,
//! non-block-header, non-`SOURCELONG:`-header lines total, but only 187 of
//! those are actual player-selectable feats (`CATEGORY:FEAT`). The other
//! 52 are excluded, for two distinct, verified reasons, never silently
//! folded in:
//!
//! - 2 rows (`CATEGORY=FEAT|Flyby Attack.MOD`, `CATEGORY=FEAT|Hover.MOD`) are
//!   cross-book `.MOD` patches onto feats defined in a different sourcebook
//!   (Core Rulebook), not new ARG feat definitions.
//! - 50 rows carry `CATEGORY:Special Ability` (or, for the 5 Kobold Scale
//!   Color rows, `CATEGORY:Kobold Scale Color`) rather than `CATEGORY:FEAT`
//!   -- these are sub-selectable-list entries granted BY a real feat (e.g.
//!   `Multitalented Mastery`'s `Bonus Hit Point`/`Bonus Skill Rank` choices,
//!   `Draconic Aspect`'s energy-line/-cone sub-options, a kobold's scale-color
//!   pick), not themselves player-selectable feats -- the same "floor, not
//!   ceiling" sub-list exclusion `rules_tables::acg::mod`'s own
//!   `named_features_expected` doc comment already establishes for e.g.
//!   Investigator's Discoveries.
//!
//! `TYPE:General`/`TYPE:Combat`/`TYPE:Teamwork` facet -- unlike CRB, ARG's
//! real feat corpus has no `ItemCreation`/`Metamagic` feats, but does carry
//! 3 real `Teamwork`-type feats (a genuine, distinct PF1 feat category CRB's
//! own 4-category scope does not exercise). One record's own `TYPE:Genaral`
//! is a corpus typo for `General` (confirmed by its own `DESC:`/`BENEFIT:`
//! content, which is ordinary General-feat prose) -- classified into
//! `FeatCategory::General` here rather than fabricating a one-off 4th real
//! category for a single misspelled facet.
//!
//! Every record here has a real `DESC:` token (100% coverage, unlike CRB's
//! partial description coverage) -- `arg_feats.lst` never omits `DESC:` on a
//! real `CATEGORY:FEAT` row.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatCategory {
    General,
    Combat,
    Teamwork,
}

impl FeatCategory {
    pub const ALL: &'static [FeatCategory] = &[FeatCategory::General, FeatCategory::Combat, FeatCategory::Teamwork];

    /// This catalog's single corpus source file -- all 3 categories are
    /// drawn from the same `arg_feats.lst` (mirrors
    /// `rules_tables::crb::feats::FeatCategory::corpus_file_name`).
    pub fn corpus_file_name(self) -> &'static str {
        "arg_feats.lst"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatTableEntry {
    /// The corpus `KEY:` token, falling back to the record's `name` when no
    /// `KEY:` token is present (no in-scope `arg_feats.lst` record carries
    /// one, so `key == name` for every entry here today).
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim. Always `Some` in this book's real
    /// in-scope corpus (see this module's own doc comment).
    pub description: Option<&'static str>,
    /// Every `BONUS:` token the corpus record carries, verbatim, in source
    /// order. `None` when the record has no `BONUS:` token at all -- mirrors
    /// `rules_tables::crb::feats::FeatTableEntry.effect`'s own convention
    /// exactly, including never using `Some(&[])` for "no data gathered yet".
    pub effect: Option<&'static [FeatEffectBonus]>,
}

/// One `BONUS:` token lifted from a feat's corpus record, captured as a
/// flat pipe-delimited qualifier list. Mirrors
/// `rules_tables::crb::feats::FeatEffectBonus` exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatEffectBonus {
    pub qualifiers: &'static [&'static str],
}

/// Full ARG feat catalog: every real corpus record across all 3 real
/// categories. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [FeatTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<FeatTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::feat_data::general::GENERAL_TABLE.len()
                + super::feat_data::combat::COMBAT_TABLE.len()
                + super::feat_data::teamwork::TEAMWORK_TABLE.len(),
        );
        all.extend_from_slice(super::feat_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::feat_data::combat::COMBAT_TABLE);
        all.extend_from_slice(super::feat_data::teamwork::TEAMWORK_TABLE);
        all
    })
}

/// Resolves an ARG feat by key. Unlike
/// `rules_tables::acg::equipment_tables::equipment_resolve`, this is not
/// `RuleSetId`-scoped — see `spell_list::spell_resolve`'s own doc comment
/// for why (this module is not wired into the shared `RuleSetId` enum,
/// per SD-27's per-cycle file-touch partition).
pub fn feat_resolve(key: &str) -> Option<&'static FeatTableEntry> {
    feat_tables().iter().find(|entry| entry.key == key)
}
