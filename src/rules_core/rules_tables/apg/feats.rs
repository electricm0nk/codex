//! PF1 APG (Advanced Player's Guide) feat catalog.
//!
//! Full corpus coverage of `advanced_players_guide/apg_feats.lst`,
//! mirroring `crb::feats` exactly -- same `FeatTableEntry` type, same
//! `TYPE:`-facet category-derivation rule, same
//! generated-from-the-live-corpus method (see `feat_data/`'s own doc
//! comments). Before this catalog existed the engine's only feats were
//! CRB's 185, so a player building an Alchemist, Cavalier, Inquisitor,
//! Oracle, Summoner or Witch could not take a single feat from that
//! class's own book.
//!
//! **The arithmetic, from the raw file.** `apg_feats.lst` holds 221
//! non-comment lines. 37 are `NAME.MOD` records, which modify an
//! already-declared record rather than declaring one, and are excluded
//! -- ingesting a `.MOD` line as a feat would invent a feat the book
//! does not have. That leaves 184 real declarations, of which **172**
//! carry a `TYPE:` facet this catalog can honestly classify:
//! General 69, Combat 81, Metamagic 19, Teamwork 3. `#`-prefixed lines
//! (including the file's own `###Block:` markers and its commented-out
//! disabled duplicates) are skipped before any of this.
//!
//! **The 12 excluded records, and why each is excluded** -- none is
//! fabricated into a category it does not claim, matching `crb::feats`'s
//! own discipline with its 10 exclusions:
//!
//! - 4 × `Elemental Focus (Acid|Cold|Electricity|Fire)` (`TYPE:ElementalFocus`)
//!   and 4 × `Greater Elemental Focus (...)` (`TYPE:GreaterElementalFocus`),
//!   both from the file's `###Block: DC Support for Elemental Focus and
//!   Greater Elemental Focus`. These are per-element DC-support records
//!   for a chooser feat, not feats. The two real player-facing feats
//!   they support, `Elemental Focus` and `Greater Elemental Focus`
//!   (`TYPE:General`, carrying the real
//!   `CHOOSE:STRING|Acid|Cold|Electricity|Fire` token), **are** in this
//!   catalog.
//! - 1 × `Elemental Spell` / `KEY:Elemental Spell Output`
//!   (`TYPE:MetamagicOutput`, `VISIBLE:EXPORT`) -- PCGen output plumbing.
//!   The real `Elemental Spell` feat (`TYPE:Metamagic`) is in this
//!   catalog, as are its four `Elemental Spell (element)` variants.
//! - 3 × `Reach Spell +1|+2|+3` -- no `TYPE:` token at all and no
//!   `DESC:`, exactly the shape of CRB's excluded `Power Attack (...)`
//!   helper records. The real `Reach Spell` feat (`TYPE:Metamagic`) is in
//!   this catalog.
//!
//! **`KEY:` vs. display name.** Unlike CRB -- where no included record
//! carries a `KEY:` token and `key` always equals `name` -- 6 APG
//! records carry a `KEY:` that differs from their display name:
//! `Elemental Fist ~ Dragon Ferocity` and `Elemental Fist ~ Full
//! Version` (both display as "Elemental Fist"), and the four
//! `Elemental Spell ~ <element>` keys (displaying as "Elemental Spell
//! (<element>)"). The corpus `KEY:` is kept as identity and the display
//! name kept separately, so the variants stay distinguishable rather
//! than one shadowing another. "Elemental Fist" is consequently the one
//! display name appearing on more than one record in this book (3
//! records, 3 distinct keys) -- the same not-deduplicated posture
//! `crb::feats` takes with its two real `Combat Expertise` records.
//!
//! Every one of the 172 included records is declared `CATEGORY:FEAT` in
//! the corpus -- checked directly, as an independent cross-check on the
//! `TYPE:`-facet rule, so no `CATEGORY:Internal` plumbing record reaches
//! the picker.
//!
//! One APG record genuinely carries no `DESC:` token: the
//! `VISIBLE:DISPLAY` "Elemental Fist" base variant (`key` == `name`).
//! Its two `VISIBLE:EXPORT` siblings do carry the text. `description` is
//! `None` there rather than borrowed from a sibling -- the same honest
//! absence CRB records for its 8 "Heighten Spell +N" records. Every
//! other APG record has real description text.
//!
//! No APG feat key or display name collides with any CRB or ACG feat --
//! verified across all three ingested books and pinned by
//! `tests/v06_apg_acg_feat_catalog.rs::feat_keys_never_collide_across_books`.
//!
//! **What this catalog does not do.** It carries each record's real
//! `BONUS:` tokens (`effect`) and real `PRE`-family tokens
//! (`prerequisites`) verbatim, but nothing here *computes* a feat's
//! mechanical effect against a character. `rules_core::feat_effects`
//! grounds computed effects for a small subset of CRB feats only; no APG
//! or ACG feat has a grounded computed effect yet, and that is
//! deliberately out of this ingest's scope.

use super::super::crb::feats::FeatTableEntry;

/// Full APG feat catalog: every classifiable record from
/// `apg_feats.lst`, generated from the live corpus. Built once and
/// cached for the process lifetime -- mirrors `crb::feats::feat_tables`.
pub fn feat_tables() -> &'static [FeatTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<FeatTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(
            super::feat_data::general::GENERAL_TABLE.len()
                + super::feat_data::combat::COMBAT_TABLE.len()
                + super::feat_data::metamagic::METAMAGIC_TABLE.len()
                + super::feat_data::teamwork::TEAMWORK_TABLE.len(),
        );
        all.extend_from_slice(super::feat_data::general::GENERAL_TABLE);
        all.extend_from_slice(super::feat_data::combat::COMBAT_TABLE);
        all.extend_from_slice(super::feat_data::metamagic::METAMAGIC_TABLE);
        all.extend_from_slice(super::feat_data::teamwork::TEAMWORK_TABLE);
        all
    })
}
