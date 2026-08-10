//! Shared archetype-swap record shape. SD28-E30 (`epic-32-archetype-swap`)
//! -- one struct definition, reused by every book's own per-book
//! archetype table (`ultimate_psionics::archetype_tables`,
//! `advanced_class_guide::archetype_tables`, ...), the same
//! shared-struct-per-book-table pattern `feats_all.rs`'s own per-book
//! modules already follow for feats.
//!
//! **Record shape is two-tier, confirmed against real corpus rows, not
//! assumed from one example.** Each archetype is:
//! - one **master/selection** row (`CATEGORY:Archetype`, `KEY:<Class>
//!   Archetype ~ <ArchetypeName>`), carrying the archetype's own flavour
//!   `DESC:`, a `TYPE:Archetype.<Class>Archetype.<SlotId>...` facet
//!   naming the base-class feature slots it *replaces*, and an
//!   `ABILITY:<Class> Class Feature|AUTOMATIC|<ArchetypeName> ~
//!   <FeatureName>|PRECLASS:1,<Class>=<Level>` token per feature it
//!   *grants*, each gated to a real class level; plus
//! - one **named sub-feature** row per grant (`CATEGORY:Special
//!   Ability`, `KEY:<ArchetypeName> ~ <FeatureName>`), carrying the real
//!   mechanical `DESC:`/`BONUS:` text for that specific swapped-in
//!   feature.
//!
//! **The `TYPE:` slot list and the `ABILITY:` grant list are NOT two
//! views of one list -- confirmed by counting both across two books, 102
//! records total, not assumed from the first one.** `TYPE:` names what
//! the archetype *takes away*; `ABILITY:` names what it *gives*. UPsi:
//! equal counts in only 4 of 15 (27%). ACG: equal counts in only 28 of
//! 87 (32%) -- the same shape on a much larger sample, not a UPsi
//! artifact. `replaces` and `grants` are kept as two separate lists for
//! exactly this reason -- pairing them positionally would fabricate a
//! correspondence the corpus does not state.
//!
//! **Subject-generic by design.** `subject` names whichever base thing
//! an archetype replaces features of -- a class name (`"Barbarian"`,
//! `"Alchemist"`) in every table landed so far, but also `"Companion"`/
//! `"Familiar"` in other books per this mechanism's own corpus survey
//! (`decisions.md §51`) -- so a future companion-subject table reuses
//! this same struct without a shape change.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchetypeGrant {
    /// The named sub-feature's own corpus `KEY:` (e.g. `"Raging Beast ~
    /// Raging Beast Manifesting"`), verbatim.
    pub grants_feature_key: &'static str,
    /// The class level this grant's own `PRECLASS:1,<Class>=<Level>`
    /// token names, verbatim.
    pub at_level: u8,
    /// The named sub-feature's own corpus `DESC:` token, resolved from
    /// its separate row. `None` when the row has no `DESC:`, or when
    /// this extraction could not find the row at all -- the two cases
    /// are not distinguished here; both are honestly `None`, never
    /// fabricated. Each book's own table doc comment names its own
    /// unresolved grants individually.
    pub description: Option<&'static str>,
    /// The named sub-feature's own corpus `BENEFIT:` token, resolved the
    /// same way. `None` when absent or unresolved.
    pub benefit: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchetypeSwapEntry {
    /// The master/selection row's own corpus `KEY:`, verbatim (e.g.
    /// `"Barbarian Archetype ~ Raging Beast"`).
    pub key: &'static str,
    /// The base class (or other subject -- see this module's own doc
    /// comment) this archetype replaces features of.
    pub subject: &'static str,
    pub archetype_name: &'static str,
    /// The master row's own corpus `DESC:` token, verbatim -- the
    /// archetype's flavour text, not any specific feature's mechanic.
    pub description: Option<&'static str>,
    pub source_page: Option<&'static str>,
    /// Every top-level `PRE`-family token the master row carries,
    /// verbatim and unparsed, in source order.
    pub prerequisites: Option<&'static [&'static str]>,
    /// The base-class feature-slot IDs this archetype's own `TYPE:`
    /// facet names as replaced, verbatim, in source order. **Not
    /// paired 1:1 with `grants`** -- see this module's own doc comment.
    pub replaces: Option<&'static [&'static str]>,
    /// Every feature this archetype's own `ABILITY:...AUTOMATIC` tokens
    /// grant, each with its real level gate and (where resolved) its
    /// own real mechanical text.
    pub grants: &'static [ArchetypeGrant],
}
