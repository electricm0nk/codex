//! PF1 CRB feat catalog.
//!
//! Full corpus coverage: every real corpus record from
//! `core_rulebook/cr_feats.lst` that carries an explicit `TYPE:` facet
//! matching one of the four Chapter 5 feat categories the corpus itself
//! encodes (185 total: General 50, Combat 110, Item Creation 8,
//! Metamagic 17). Generated programmatically from the live corpus -- see
//! `feat_data/`'s own doc comment for the generation method (not
//! hand-authored, so there is no fabrication/transcription risk at this
//! scale; regenerate if the corpus changes).
//!
//! Category is derived from the `TYPE:` facet, not the corpus's own
//! `###Block:` markers. Unlike `cr_equip_*.lst` (one corpus *file* per
//! equipment category), `cr_feats.lst` has a single `###Block: General
//! Feats` section holding every feat; the real category signal is the
//! per-record `TYPE:` tag (e.g. `TYPE:Combat.Critical` carries both a
//! `Combat` facet and a `Critical` subtype facet). A record is included
//! here when its `TYPE:` value contains one of `General`, `Combat`,
//! `Metamagic`, or `ItemCreation` as a dot-separated facet.
//!
//! 12 corpus records are deliberately excluded, not fabricated into a
//! category: 8 "Power Attack (...)" and 1 "Leadership Score" internal
//! `VISIBLE:NO` helper records (PCGen export-engine plumbing, not
//! player-facing feats, and lacking any `TYPE:` token at all), 1 "Cleave
//! (Granted by Sylvan Scimitar)" item-granted variant (no `TYPE:` token),
//! and 2 `CATEGORY=...MOD` lines that modify another feat's formula
//! rather than defining a new one. None of these carry a `TYPE:` facet
//! this catalog can honestly classify, matching `Pf1SchoolId::from_corpus_str`'s
//! own discipline of returning nothing rather than guessing.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatCategory {
    General,
    Combat,
    ItemCreation,
    Metamagic,
}

impl FeatCategory {
    pub const ALL: &'static [FeatCategory] = &[
        FeatCategory::General,
        FeatCategory::Combat,
        FeatCategory::ItemCreation,
        FeatCategory::Metamagic,
    ];

    /// This catalog's single corpus source file -- all 4 categories are
    /// drawn from the same `core_rulebook/cr_feats.lst`, unlike
    /// `EquipmentCategory::corpus_file_name` where each category has its
    /// own file. See this module's own doc comment for the `TYPE:`-facet
    /// derivation rule that replaces the file-per-category split.
    pub fn corpus_file_name(self) -> &'static str {
        "cr_feats.lst"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatTableEntry {
    /// The corpus `KEY:` token, falling back to the record's `name` when
    /// no `KEY:` token is present -- the same fallback
    /// `EquipmentTableEntry.key` documents. Almost no `cr_feats.lst`
    /// record in this catalog's 4 categories carries an explicit `KEY:`
    /// token (the one corpus record that does, "Cleave (Granted by
    /// Sylvan Scimitar)", is excluded -- see this file's module doc
    /// comment), so `key` equals `name` for every entry here today.
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim. `None` when the record has no
    /// `DESC:` token (e.g. the 8 "Heighten Spell +N" bonus-tier records,
    /// which carry only `ADDSPELLLEVEL:`/`BONUS:`/`FACT:` tokens) --
    /// mirrors `EquipmentTableEntry.cost_gp`'s `None`-when-absent rule.
    pub description: Option<&'static str>,
}

/// Full CRB feat catalog: every real corpus record across all 4 book
/// feat categories, generated from the live corpus (see `feat_data/`'s
/// own doc comment for the generation method -- not hand-authored, so
/// there is no fabrication/transcription risk at this scale). Built once
/// and cached for the process lifetime.
pub fn feat_tables() -> &'static [FeatTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<FeatTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::feat_data::general::GENERAL_TABLE.len()
                + super::feat_data::combat::COMBAT_TABLE.len()
                + super::feat_data::item_creation::ITEM_CREATION_TABLE.len()
                + super::feat_data::metamagic::METAMAGIC_TABLE.len(),
        );
        all.extend_from_slice(super::feat_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::feat_data::combat::COMBAT_TABLE);
        all.extend_from_slice(super::feat_data::item_creation::ITEM_CREATION_TABLE);
        all.extend_from_slice(super::feat_data::metamagic::METAMAGIC_TABLE);
        all
    })
}
