//! PF1 ACG (Advanced Class Guide) feat catalog.
//!
//! Full corpus coverage of `advanced_class_guide/acg_feats.lst`,
//! mirroring `crb::feats` exactly -- same `FeatTableEntry` type, same
//! `TYPE:`-facet category-derivation rule, same
//! generated-from-the-live-corpus method (see `feat_data/`'s own doc
//! comments). Before this catalog existed the engine's only feats were
//! CRB's 185, so a player building an Arcanist, Bloodrager, Brawler,
//! Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler or
//! Warpriest could not take a single feat from that class's own book --
//! including the Panache deed feats the Swashbuckler is built around.
//!
//! **The arithmetic, from the raw file.** `acg_feats.lst` holds 173
//! non-comment lines. 39 are `NAME.MOD` records, which modify an
//! already-declared record rather than declaring one, and are excluded.
//! That leaves 134 real declarations, of which **129** carry a `TYPE:`
//! facet this catalog can honestly classify: General 62, Combat 59,
//! Teamwork 4, Panache 4. `#`-prefixed lines (including `###Block:`
//! markers and commented-out disabled records) are skipped first.
//!
//! **The 5 excluded records, and why** -- none is fabricated into a
//! category it does not claim:
//!
//! - `Witch Hex` and `Shaman Hex` (`TYPE:Hex Selection`) -- both are
//!   declared `CATEGORY:Internal`, not `CATEGORY:FEAT`, under
//!   `KEY:Hex Selection ~ Witch Hex` / `~ Shaman Hex`, and carry no
//!   `DESC:` at all. They are hex-pool selection plumbing, not feats.
//! - `Animal Companion of Nature Bond Class Feature`,
//!   `Animal Companion of Divine Bond Class Feature` and
//!   `Animal Companion of Mount Class Feature`
//!   (`TYPE:Evolved Companion`) -- these name where a companion comes
//!   from ("Typical Nature Bond Companion is from Druid or Ranger"),
//!   they are not feats a player takes.
//!
//! Every one of the 129 included records is declared `CATEGORY:FEAT` in
//! the corpus -- checked directly, as an independent cross-check on the
//! `TYPE:`-facet rule, so no `CATEGORY:Internal` plumbing record reaches
//! the picker.
//!
//! **`KEY:` vs. display name.** No included ACG record carries a `KEY:`
//! token that differs from its display name, so `key` equals `name` for
//! all 129 -- the same fallback `crb::feats` documents. There are no
//! duplicate display names within this book, and no ACG feat key or
//! display name collides with any CRB or APG feat (verified across all
//! three ingested books and pinned by
//! `tests/v06_apg_acg_feat_catalog.rs::feat_keys_never_collide_across_books`).
//!
//! **Panache and Teamwork categories.** 4 records carry `TYPE:Panache`
//! and 4 carry `TYPE:Teamwork` with no `Combat`/`General` facet
//! alongside. Under CRB's four-category rule those 8 real, player-facing
//! feats would have been dropped; `FeatCategory` widened instead -- see
//! its own doc comment. Records carrying both facets (5 ×
//! `TYPE:Combat.Teamwork`, 1 × `TYPE:Combat.Panache`, 3 ×
//! `TYPE:Combat.Style`) resolve to `Combat`, exactly as CRB resolves
//! `TYPE:Combat.AttackOption.ModifyAC`.
//!
//! **What this catalog does not do.** It carries each record's real
//! `BONUS:` tokens (`effect`) and real `PRE`-family tokens
//! (`prerequisites`) verbatim, but nothing here *computes* a feat's
//! mechanical effect against a character. `rules_core::feat_effects`
//! grounds computed effects for a small subset of CRB feats only; no APG
//! or ACG feat has a grounded computed effect yet, and that is
//! deliberately out of this ingest's scope.

use super::super::crb::feats::FeatTableEntry;

/// Full ACG feat catalog: every classifiable record from
/// `acg_feats.lst`, generated from the live corpus. Built once and
/// cached for the process lifetime -- mirrors `crb::feats::feat_tables`.
pub fn feat_tables() -> &'static [FeatTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<FeatTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::feat_data::general::GENERAL_TABLE.len()
                + super::feat_data::combat::COMBAT_TABLE.len()
                + super::feat_data::teamwork::TEAMWORK_TABLE.len()
                + super::feat_data::panache::PANACHE_TABLE.len(),
        );
        all.extend_from_slice(super::feat_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::feat_data::combat::COMBAT_TABLE);
        all.extend_from_slice(super::feat_data::teamwork::TEAMWORK_TABLE);
        all.extend_from_slice(super::feat_data::panache::PANACHE_TABLE);
        all
    })
}
