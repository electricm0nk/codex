//! The **settled** canonical shape of one `data/corpus/<book>/race/…` chassis
//! record and one `data/corpus/<book>/race_trait/…` racial-trait record.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 14, under `decisions.md` §11 ("no PCGen in
//! live code"), `decisions.md` §19 (ruling B16) and `technical-design.md` §0
//! (the boundary is **by path**).
//!
//! # What this is, and what it is not
//!
//! This is the race-side sibling of
//! [`CorpusEquipmentRecord`](crate::rules_core::equipment_record::CorpusEquipmentRecord),
//! built by cycle 13 for exactly the same reason.
//! [`crate::rules_core::race_resolver`] used to hold
//! `pcgen_import::ingest_payload::{RaceCacheData, RaceTraitCacheData}` — cache
//! payloads that still carry the `.lst` row's `raw_tokens` and
//! `raw_bonus_chains` arrays — and then read eleven separate facts back out of
//! those arrays at run time through `pcgen_import::race_trait_tokens` and
//! `pcgen_import::bonus_chain_reader`. Three converter imports, in the live
//! race resolver, on every load.
//!
//! Every one of those readings still happens, in exactly the same function,
//! with exactly the same body, on the converter side: see
//! [`crate::pcgen_import::corpus_race_json`], which is the only thing that
//! builds the two structs below. **Nothing is re-derived here and no value is
//! re-parsed here.** What changed is *when* the reading happens — once, at the
//! ingest boundary, instead of once per accessor call — and *where the answer
//! is declared*, which is now the live side, so the live resolver can hold the
//! answer without naming the grammar that produced it.
//!
//! # Why there is no token array on either struct
//!
//! Deliberately. A settled record that still carried `raw_tokens` would let any
//! future live module re-open the ingest format behind the gate's back, which
//! is the precise defect ruling B16 exists to make visible. If a live module
//! needs a fact this struct does not state, the fix is a new **settled field**
//! filled by the converter-side reader — never a token array handed back under
//! a new name.

use crate::rules_core::declared_bonuses::DeclaredBonuses;
use crate::rules_core::size::SizeCategory;

/// One `data/corpus/<book>/race/<slug>.json` record's `data` object, settled.
///
/// Every field is a transcription of what the ingested row states. `None` /
/// empty means the row states nothing, never a default this module chose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusRaceRecord {
    /// The race's PCGen key (e.g. `"Dwarf"`).
    pub key: String,
    pub name: String,
    /// The chassis' declared base size code (e.g. `"M"`), verbatim.
    ///
    /// By its own upstream definition this is "the SMALLEST allowed" size, not
    /// the default one — see
    /// [`SizeSource`](crate::rules_core::race_resolver::SizeSource) for why a
    /// resolved trait may legitimately override it.
    pub base_size: Option<String>,
    /// The chassis' declared walking speed in feet.
    pub base_move_walk: Option<i32>,
    /// The creature's declared race type (e.g. `"Humanoid"`).
    pub race_type: Option<String>,
    /// The row's declared type segments, in source order.
    pub type_tokens: Vec<String>,
    pub legs: Option<i32>,
    pub hands: Option<i32>,
}

/// One `data/corpus/<book>/race_trait/<race>/<slug>.json` record's `data`
/// object, settled — the standard *or* the alternate end of PCGen's
/// replace-flag protocol (`decisions.md §26`).
///
/// The first block of fields is stored corpus content. The second block is the
/// settled result of a reading `race_resolver` used to perform at run time, one
/// field per named converter-side function; each field's doc comment names the
/// function that fills it, so the provenance of every value is one grep away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusRaceTraitRecord {
    /// `KEY:Dwarf ~ Greed` → `"Dwarf ~ Greed"`.
    pub key: String,
    pub name: String,
    /// The owning race's key (e.g. `"Dwarf"`), so traits resolve per race
    /// without re-parsing the key string.
    pub race_key: String,
    pub category: Option<String>,
    /// The row's declared type segments — carries `"Dwarf Racial Default"` on
    /// the standard set, which is how the default roster is read from the
    /// corpus instead of assumed.
    pub type_tokens: Vec<String>,
    /// True when [`type_tokens`](Self::type_tokens) contains a
    /// `"<Race> Racial Default"` marker.
    pub is_racial_default: bool,
    /// The flag whose presence suppresses this trait. Set on standard traits.
    pub suppressed_by_flag: Option<String>,
    /// The `<Race>_Replace<Trait>` flags this trait *sets*. Populated on ARG's
    /// alternate traits; empty on standard ones.
    pub sets_replace_flags: Vec<String>,
    pub description: Option<String>,
    pub source_page: Option<String>,

    // ---- settled readings (filled by `pcgen_import::corpus_race_json`) ----
    /// The flags that, once set by some other selection, block this row.
    ///
    /// The corpus states that one relation four different ways; the converter
    /// reads all four (`race_trait_tokens::exclusion_guard_flags`) and this
    /// field is the **relation**, not the grammar.
    pub exclusion_guard_flags: Vec<String>,
    /// Each negated fact gate this row declares, as its group of flag strings.
    ///
    /// A group longer than one entry is a row whose single guard names several
    /// flags — the shape the ARG picker reports as a findings row rather than
    /// absorbing silently. From `race_trait_tokens::negated_fact_gates`.
    pub negated_fact_gates: Vec<Vec<String>>,
    /// Whether this row writes its self-exclusion guard's negated branch as an
    /// ability prerequisite rather than a fact prerequisite — an upstream
    /// corpus slip the picker surfaces rather than absorbs. From
    /// `race_trait_tokens::declares_preability_negated_guard`.
    pub declares_negated_ability_guard: bool,
    /// Every ability key this record grants outright, verbatim and
    /// **unfiltered** — most of these name things that are not racial traits at
    /// all, and deciding which resolve to a loaded record is the caller's job.
    /// From `race_trait_tokens::automatic_ability_grants`.
    pub automatic_trait_grants: Vec<String>,
    /// The Skinwalker kin whose Change Shape pool this row's own automatic
    /// grant names, if this row is a kin master record at all — the kin suffix
    /// (`"Werebear"`, …), never the pool-qualified grant string. From
    /// `race_trait_tokens::skinwalker_change_shape_kin` over
    /// [`automatic_trait_grants`](Self::automatic_trait_grants).
    pub skinwalker_change_shape_kin: Option<String>,
    /// The flag this row requires to have already fired before it is granted.
    /// From `race_trait_tokens::positive_prefact_flag`.
    pub positive_prefact_flag: Option<String>,
    /// This trait's declared walking speed in feet, if it declares one. From
    /// `race_trait_tokens::declared_walk_speed_ft`.
    pub declared_walk_speed_ft: Option<i32>,
    /// The creature size this trait assigns, if it declares one. From
    /// `race_trait_tokens::declared_size`.
    pub declared_size: Option<SizeCategory>,
    /// Every sense this trait declares, one entry per segment, verbatim
    /// (`Darkvision (60)`, `Low-Light Vision`). From
    /// `race_trait_tokens::declared_vision_segments`.
    pub declared_vision: Vec<String>,
    /// Everything this row's declared bonuses state, read once. From
    /// `bonus_chain_reader::declared_bonuses`.
    pub declared_bonuses: DeclaredBonuses,
    /// Every real `Bonus Language ~ <Lang>` row this trait's own `TEMPLATE:`
    /// chain names, transcribed verbatim — including the literal marker
    /// `"Any Spoken"`, which is upstream's "no restriction" template rather
    /// than a real language, so a caller that cares about the difference must
    /// check for it. From `race_trait_tokens::declared_template_bonus_languages`.
    pub template_bonus_languages: Vec<String>,
    /// An Adopted-Race selector row's target pool suffix, e.g.
    /// `"Oread Race Trait"`. `None` for every row that is not one, and for a
    /// malformed selector row this project refuses to guess at. From
    /// `race_trait_tokens::adopted_race_pool_suffix`.
    pub adopted_race_pool_suffix: Option<String>,
}
