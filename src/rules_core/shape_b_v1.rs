//! Shape B v1 — the license-aware, cross-book corpus-cache record schema
//! authority for SD-27 (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md`
//! §17, cycle E2.0.5).
//!
//! **Consolidation, deliberately not book-local.** SD-26's Shape B v0
//! (`src/rules_core/rules_tables/crb/json_cache.rs`) was deliberately kept
//! local to the CRB module so 3 sibling `isolation: 'worktree'` cycles
//! (APG/ACG/Bestiary-1) building the same shape independently against the
//! same shared branch would not collide on one shared file. That
//! isolation constraint no longer applies here — v1 is authored once,
//! after those cycles landed, as the single shared schema every future
//! book's cache generator targets. See this module's own doc comment
//! contrast with `json_cache.rs`'s module doc comment for the reasoning
//! each cycle operated under.
//!
//! **Additive over v0 (decisions.md §17).** `CorpusRecordV1<T>` carries
//! every field `CorpusRecord<T>` (v0) has, unchanged, plus 3 new
//! `#[serde(default)]` fields: `license`, `pi_field`, `pi_marker`. A v0
//! JSON blob — one of the 4 in-scope books' existing on-disk records,
//! none of which carry a `license` key yet — deserializes cleanly as a
//! v1 record: the 3 new fields come back `None`, which means "not yet
//! license-classified," not "safe to treat as OGL." This cycle does not
//! modify any existing record (loop-instruction.md §3.2.5 Notes); the
//! per-book retro-fit that actually populates `license` on disk is
//! cycles 2.0.6-2.0.9.
//!
//! **Redaction-to-marker policy (decisions.md §17, operator-pinned
//! 2026-07-25).** A Product-Identity-tagged field value is replaced with
//! the literal marker string `"[redacted PI]"` rather than omitted —
//! the record's schema shape is preserved so downstream consumers read
//! one branch per field ("is this a marker? render a generic label")
//! instead of conditional-everywhere code for a field that might not
//! exist. `pi_field` names which field was redacted; `pi_marker` is
//! `"redacted"` when that happened.
//!
//! The PI-blacklist that classifies real field names (`deity`,
//! `npc_name`, `monster_name`, `description`, ...) as PI or OGL-inlinable
//! lives at `docs/governance/ogl-pi-blacklist.md` — an operator-reviewable
//! **draft**, not something this module enforces silently. This module
//! only defines the shape a classification decision is recorded in.

use serde::{Deserialize, Serialize};

/// Shape B's `population` discriminator, unchanged from v0
/// (`json_cache.rs::Population` / SD-26 `decisions.md §7`). Re-declared
/// here (not re-exported from the CRB-local v0 module) because v1 is the
/// shared, book-agnostic authority — a CRB-specific module is not a
/// principled place for every future book's cache generator to depend
/// on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Population {
    InScope,
    FutureState,
    RuleSystemStub,
}

/// Shape B's `completeness` discriminator, unchanged from v0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    ChassisOnly,
    ChassisPlusExtract,
    Full,
}

/// The discriminated `source` union, unchanged from v0
/// (`json_cache.rs::CorpusSource` / SD-26 `decisions.md §11.2`). Variant
/// names, field names, and the `kind` tag all byte-match v0 so that every
/// existing on-disk `source` object deserializes into this type without
/// modification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CorpusSource {
    LstToken {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
    },
    LstInheritedCopy {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        inherited_from_record_key: String,
    },
    LstCorrectedIngest {
        path: String,
        sha256: String,
        line: u32,
        record_key: String,
        original_ingest_defect: String,
    },
    WebSecondSource {
        url: String,
        fetched_at: String,
        identity_match_basis: String,
    },
    SameBookFallback { fallback_basis: String },
}

/// The license classification a record (or one redacted field within it)
/// carries, per `decisions.md §17`. Serializes to exactly the 3 literal
/// strings the decision record pins: `"OGL"`, `"PI"`, `"PI-REDACTED"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum License {
    /// Open Game Content — game mechanics/procedures/formulae (BAB
    /// progression, save formulas, spell level, weight, cost, ...).
    /// Safe to inline verbatim.
    #[serde(rename = "OGL")]
    Ogl,
    /// Product Identity, present and NOT redacted. This state exists for
    /// completeness of the 3-way enum decisions.md §17 specifies, but
    /// per the redaction-to-marker policy no record should ship in this
    /// state long-term — PI is either not present or already redacted.
    #[serde(rename = "PI")]
    Pi,
    /// Product Identity whose value has been replaced with the
    /// `"[redacted PI]"` marker. This is the terminal, shippable state
    /// for a record that once carried a PI-tagged field value.
    #[serde(rename = "PI-REDACTED")]
    PiRedacted,
}

/// The literal redaction marker decisions.md §17 pins: the value a
/// PI-tagged field is replaced with when redacted (schema-preserving —
/// the field keeps its place and its type, only the content changes).
pub const REDACTED_PI_MARKER: &str = "[redacted PI]";

/// The literal `pi_marker` value a redacted field's record carries.
pub const PI_MARKER_REDACTED: &str = "redacted";

/// One Shape B v1 JSON-cache record, generic over the book-specific
/// `data` payload — the same shape as v0's `CorpusRecord<T>` plus 3
/// additive, `#[serde(default)]` license fields (decisions.md §17).
///
/// Additive proof: every field on `CorpusRecord<T>` (v0) is present here
/// unchanged (`population`, `completeness`, `ingested_at`, `data`,
/// `source`); nothing was removed or renamed. A v0 JSON object — lacking
/// `license`/`pi_field`/`pi_marker` entirely — deserializes into this
/// type with those 3 fields defaulting to `None`. See this file's tests
/// for a runnable proof against a real v0-shaped payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusRecordV1<T> {
    pub population: Population,
    pub completeness: Completeness,
    pub ingested_at: String,
    pub data: T,
    pub source: CorpusSource,
    /// `"OGL" | "PI" | "PI-REDACTED"`. `None` on a record that has not
    /// yet been through a license-stripping retro-fit cycle (every
    /// existing v0 record, as of this cycle) — deliberately NOT defaulted
    /// to `Ogl`, which would silently assert an unreviewed record is
    /// safe to redistribute.
    #[serde(default)]
    pub license: Option<License>,
    /// Which field name was redacted, when `license` is
    /// `Pi`/`PiRedacted`. `None` for an `Ogl` record or a not-yet-
    /// classified (`license: None`) record.
    #[serde(default)]
    pub pi_field: Option<String>,
    /// `Some("redacted")` exactly when the field named by `pi_field` had
    /// its value replaced with [`REDACTED_PI_MARKER`].
    #[serde(default)]
    pub pi_marker: Option<String>,
}

/// A v1-record validation defect (`decisions.md §17`'s "Validation: every
/// record has a license field" requirement plus the 5th-audit's
/// PI/marker consistency rule).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseValidationError {
    /// `license` is `None` — the record has not been through a
    /// license-stripping retro-fit yet.
    MissingLicense,
    /// `license` is `Pi`/`PiRedacted` but `pi_field` is not populated,
    /// so nothing identifies which field was tagged.
    MissingPiField,
    /// `license` is `PiRedacted` but `pi_marker` is not exactly
    /// `Some("redacted")` (the 5th-audit's PI-blacklist grep rule,
    /// decisions.md §17).
    MissingRedactionMarker,
}

impl std::fmt::Display for LicenseValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingLicense => write!(f, "record is missing a license field (not yet retro-fitted to Shape B v1)"),
            Self::MissingPiField => write!(f, "PI/PI-REDACTED record is missing pi_field (which field was tagged)"),
            Self::MissingRedactionMarker => {
                write!(f, "PI-REDACTED record must carry pi_marker: \"redacted\"")
            }
        }
    }
}

impl std::error::Error for LicenseValidationError {}

/// One raw `KEY:VALUE` token from a record's source LST line(s), preserved
/// verbatim. Mirrors `pcgen_import::lst_parser::equipment::EquipmentToken`'s
/// `key`/`value` fields exactly (that struct's `line_number`/raw-text fields
/// are provenance detail already covered by `CorpusSource`, not duplicated
/// here).
///
/// **Why this exists (2026-07-30, desktop-runtime-reachability finding):**
/// Shape B v1's original per-content-kind `data` payloads (`EquipmentCacheData`
/// et al.) are a deliberately thin "bootstrap coverage" projection --
/// `key`/`category`/`name`/`cost_gp`/`weight_lbs`/`description` for
/// equipment, no `ACCHECK:`/`MAXDEX:`/`SPELLFAILURE:`/`BONUS:` data at all.
/// That's sufficient for the compiled `rules_tables::crb::equipment_tables()`
/// static table this schema originally fed, but the real engine's
/// book-agnostic resolvers (`encumbrance.rs`, `equipment_effects.rs`, and
/// every future book-agnostic resolver) read a record's raw tokens/bonus
/// chains directly -- which only ever existed in the *raw LST text* the
/// codegen tools parse at generation time, never persisted to the on-disk
/// JSON cache itself. The desktop app's live corpus loader
/// (`apps/desktop/src-tauri/src/corpus_fixtures.rs`) needs a **reviewable,
/// PI-screened, license-annotated** artifact to load from -- raw LST text
/// is neither reviewed nor screened. This type (plus [`RawBonusChain`]) is
/// the additive fix: carry the *generic* raw token/bonus-chain shape every
/// resolver already expects, inside the same Shape B v1 record the
/// PI-blacklist/license machinery already governs, rather than re-deriving
/// bespoke named fields (`accheck`, `maxdex`, ...) per content kind that
/// would need updating every time a resolver needs one more token. Pure
/// game-mechanic tokens (`WT`, `COST`, `ACCHECK`, `BONUS:COMBAT|AC`, ...)
/// are OGL open content, not Product Identity -- PI risk lives in
/// prose/name fields (`description`, `name` in rare cases), which the
/// existing `license`/`pi_field`/`pi_marker` machinery already governs
/// unchanged by this addition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawToken {
    pub key: String,
    pub value: String,
}

/// One raw `BONUS:...` clause's pipe-delimited qualifiers, in source order.
/// Mirrors `pcgen_import::lst_parser::equipment::BonusToken::qualifiers`
/// exactly. See [`RawToken`]'s doc comment for why this exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawBonusChain {
    pub qualifiers: Vec<String>,
}

/// `data/corpus/<book>/equipment/<category>/<slug>.json` payload, v1.
/// Additive over the pre-existing `EquipmentCacheData` copies
/// (`rules_tables::crb::json_cache`, `rules_tables::advanced_race_guide::
/// json_cache`, `sd27_gen_book_cache`'s own local copy -- byte-identical to
/// each other before this addition, confirmed via direct diff): every field
/// those carry (`key`/`category`/`name`/`cost_gp`/`weight_lbs`/
/// `description`) is present here unchanged, plus 2 new
/// `#[serde(default)]` fields (`raw_tokens`, `raw_bonus_chains` -- see
/// [`RawToken`]'s doc comment). An on-disk record written before this
/// addition deserializes cleanly with both new fields defaulting to empty
/// `Vec`s, which is honestly indistinguishable from "record was
/// regenerated but genuinely has no tokens" -- the real distinguishing
/// signal is `completeness`, unchanged by this addition, so a caller that
/// needs to tell "not yet regenerated" apart from "regenerated, no extra
/// tokens" should gate on that instead.
///
/// This is now the single shared definition; per-book `json_cache.rs`
/// copies and `sd27_gen_book_cache.rs`'s local struct should import this
/// type rather than maintain their own, per the same consolidation
/// principle `CorpusRecordV1<T>` itself already established over v0's
/// per-book duplication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentCacheData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    pub weight_lbs: Option<f64>,
    pub description: Option<String>,
    #[serde(default)]
    pub raw_tokens: Vec<RawToken>,
    #[serde(default)]
    pub raw_bonus_chains: Vec<RawBonusChain>,
}

/// `data/corpus/<book>/race/<slug>.json` payload, v1 — the race *chassis*
/// row from a `*_races.lst` file (`decisions.md §25`).
///
/// **Provenance warning, and why `source_page` is absent here.** The chassis
/// rows in `core_essentials/races/*/*_races.lst` carry a placeholder
/// `SOURCEPAGE:p.xx`, not a real page (verified 2026-07-31, `decisions.md
/// §26`). Transcribing that as though it were a citation would manufacture
/// false provenance, so this payload deliberately has no page field: a race's
/// real citation comes off its *trait* rows ([`RaceTraitCacheData::source_page`],
/// e.g. Dwarf's `p.21`), which do carry genuine ones.
///
/// **Book attribution.** `core_essentials/` is PCGen's physical storage for
/// race files shared across books, not a book in its own right, and it is out
/// of project scope (`decisions.md §1`, §25.2). The `book` a record is filed
/// under is therefore its *true* source per `advanced_race_guide.pcc`'s own
/// section comments — Core Rulebook for the 7 core races, Bestiary 1 for its
/// 11 — never `core_essentials`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceCacheData {
    /// The race's PCGen key (e.g. `"Dwarf"`).
    pub key: String,
    pub name: String,
    /// `FACT:BaseSize|M` → `"M"`.
    pub base_size: Option<String>,
    /// `MOVE:Walk,20` → `20`.
    pub base_move_walk: Option<i32>,
    /// `RACETYPE:Humanoid`.
    pub race_type: Option<String>,
    /// `TYPE:Humanoid.Base.PC` split on `.`.
    pub type_tokens: Vec<String>,
    /// `LEGS:2` / `HANDS:2`.
    pub legs: Option<i32>,
    pub hands: Option<i32>,
    #[serde(default)]
    pub raw_tokens: Vec<RawToken>,
}

/// `data/corpus/<book>/race_trait/<race>/<slug>.json` payload, v1 — one
/// racial trait, standard *or* alternate.
///
/// **This type models PCGen's replace-flag protocol directly** rather than
/// inventing a swap mechanic (`decisions.md §26`). A standard trait declares
/// the flag that suppresses it; an alternate trait declares the flags it
/// sets. Resolution is then: a standard trait applies iff no selected
/// alternate has set its [`suppressed_by_flag`](Self::suppressed_by_flag).
///
/// The two halves live on one struct because they are two ends of one
/// relationship, and keeping them together makes an unmatched flag — an
/// alternate that replaces a standard trait nothing declares, or vice versa —
/// a checkable defect rather than a silent no-op.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceTraitCacheData {
    /// `KEY:Dwarf ~ Greed` → `"Dwarf ~ Greed"`.
    pub key: String,
    pub name: String,
    /// The owning race's key (e.g. `"Dwarf"`), so traits resolve per race
    /// without re-parsing the key string.
    pub race_key: String,
    /// `CATEGORY:Special Ability`.
    pub category: Option<String>,
    /// `TYPE:` split on `.` — carries `"Dwarf Racial Default"` on the
    /// standard set, which is how the default roster is read from the corpus
    /// instead of assumed.
    pub type_tokens: Vec<String>,
    /// True when [`type_tokens`](Self::type_tokens) contains a
    /// `"<Race> Racial Default"` marker.
    pub is_racial_default: bool,
    /// From `!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True` →
    /// `Some("Dwarf_ReplaceGreed")`. Set on standard traits: the flag whose
    /// presence suppresses this trait.
    pub suppressed_by_flag: Option<String>,
    /// The `<Race>_Replace<Trait>` flags this trait *sets*. Populated on
    /// ARG's alternate traits; empty on standard ones.
    #[serde(default)]
    pub sets_replace_flags: Vec<String>,
    pub description: Option<String>,
    /// `SOURCEPAGE:p.21`. Genuine on trait rows, unlike the chassis row —
    /// see [`RaceCacheData`].
    pub source_page: Option<String>,
    #[serde(default)]
    pub raw_tokens: Vec<RawToken>,
    #[serde(default)]
    pub raw_bonus_chains: Vec<RawBonusChain>,
}

/// One class feature a [`ClassVariantCacheData`] grants, with the level it
/// comes online at — transcribed off the grant row, never inferred.
///
/// PCGen states a variant's whole level progression declaratively, one
/// `.MOD` row per feature:
///
/// ```text
/// CATEGORY=Class|Monk ~ Unchained Class.MOD    ABILITY:Unchained Monk Class Feature|AUTOMATIC|Unchained Monk ~ Ki Pool|PREVAREQ:Monk_CF_KiPool,0|PREVARGTEQ:Monk_CFP_Level,3
/// ```
///
/// (The gap after `.MOD` is a single literal TAB in the corpus file --
/// PCGen `.lst` rows are tab-delimited. It is shown as four spaces here
/// because a tab in a doc comment renders unpredictably.)
///
/// Reading `3` off `PREVARGTEQ:Monk_CFP_Level,3` is transcription of a
/// same-row integer literal, so `decisions.md §24`'s ban on a formula
/// interpreter is not engaged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassFeatureGrant {
    /// The granted ability's key, e.g. `"Unchained Monk ~ Ki Pool"`.
    pub feature_key: String,
    /// The ability category the grant names, e.g.
    /// `"Unchained Monk Class Feature"`.
    pub feature_category: String,
    /// `PREVARGTEQ:<Class>_CFP_Level,<n>` → `n`. `None` when the grant row
    /// states no level at all — Unchained Barbarian's Weapon and Armor
    /// Proficiency genuinely carries none upstream, and inventing the
    /// book's "1st level" here would be fabricating corpus data.
    pub min_level: Option<u8>,
    /// The `.MOD` target that carries this grant. Unchained Barbarian
    /// splits its progression across two sub-selections
    /// (`Barbarian ~ Unchained Class Full` and
    /// `Barbarian ~ Unchained Ex-Class`), so which one granted a feature is
    /// content, not bookkeeping.
    pub granted_by_key: String,
    /// `PREVAREQ:<Class>_CF_<Feature>,0` → `"Monk_CF_KiPool"`. The archetype
    /// suppression variable: the grant applies only while the variable is 0,
    /// i.e. while no archetype has replaced the feature.
    pub suppressed_by_var: Option<String>,
}

/// `data/corpus/<book>/class/<slug>.json` payload, v1 — a **class variant**
/// declared as a `CATEGORY:CLASS` selection ability over a base class
/// declared in another book.
///
/// **Why this is not `rules_tables::crb::json_cache::ClassCacheData`.**
/// Pathfinder Unchained's `.pcc` declares **no `CLASS:` file at all**
/// (verified 2026-07-31). Its four "Unchained classes" are not `CLASS`
/// objects: each is an `ABILITY` in `CATEGORY:CLASS` that plugs into the
/// base class's own selection pool (`ABILITYCATEGORY:<Class> Class
/// Selection`, declared by `core_rulebook/cr_abilitycategories.lst`) and
/// swaps the base class's features out for its own. A record shaped like a
/// full class chassis would therefore have to invent a hit die, a BAB
/// column and a save column the book does not state — the same failure mode
/// `decisions.md §25` caught for ARG's races.
///
/// The chassis fields here are consequently all `Option`, and populated
/// **only** where PU genuinely overrides the base class. In practice that is
/// Unchained Monk alone: it carries a `TEMPLATE:` that raises the hit die and
/// `BONUS:COMBAT|BASEAB|...|TYPE=Base.REPLACE` plus `BONUS:SAVE|BASE...`
/// clauses that replace the BAB and save columns. The other three inherit
/// their base class's chassis unchanged and leave every field `None`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassVariantCacheData {
    /// `KEY:Monk ~ Unchained Class` → `"Monk ~ Unchained Class"`.
    pub key: String,
    /// The unkeyed display name, e.g. `"Unchained Monk"`.
    pub name: String,
    /// The base class this variant replaces the features of, e.g. `"Monk"`.
    /// Read off the `TYPE:<Class> Class Selection` token, not off the key
    /// string.
    pub base_class_key: String,
    /// The corpus book directory the base class is already ingested under —
    /// `"core_rulebook"` for Barbarian/Monk/Rogue, `"advanced_players_guide"`
    /// for Summoner. A variant whose base class is not ingested must not be
    /// written at all (`decisions.md §25.3`'s rule, applied to classes).
    pub base_class_book: String,
    /// `CATEGORY:CLASS`.
    pub category: Option<String>,
    /// `TYPE:Monk Class Selection.AltMonkChoice` split on `.`.
    pub type_tokens: Vec<String>,
    /// From the `TEMPLATE:` this row applies, whose `HITDIE:10|CLASS=Monk`
    /// states the override. `None` where the variant applies no such
    /// template.
    pub hit_die: Option<u32>,
    /// The template name the [`hit_die`](Self::hit_die) came off, so the
    /// override is traceable to the row that states it.
    pub hit_die_template: Option<String>,
    /// BAB progression in the same `level`-relative notation the existing
    /// class records use (`"level*3/4"`), derived by substituting `level`
    /// for a `classlevel("<BaseClass>",...)` call and keeping the arithmetic
    /// tail byte-identical. Populated only from a
    /// `BONUS:COMBAT|BASEAB|...` clause on this row; `None` otherwise.
    pub bab: Option<String>,
    /// True when the BAB clause carries `TYPE=Base.REPLACE`, i.e. it
    /// *replaces* the base class's column rather than stacking on it.
    #[serde(default)]
    pub bab_replaces_base: bool,
    /// Save progressions, same notation and same derivation as
    /// [`bab`](Self::bab), from `BONUS:SAVE|BASE.<Save>|...` clauses.
    pub save_fort: Option<String>,
    pub save_ref: Option<String>,
    pub save_will: Option<String>,
    /// The variant's class-skill list, resolved from the
    /// `ABILITY:Internal|AUTOMATIC|Class Skills ~ <name>` grant to that
    /// internal row's `CSKILL:` token. Empty where the variant states its
    /// class skills on a class *feature* row instead (Unchained Rogue and
    /// Unchained Summoner both do — see
    /// [`ClassFeatureCacheData::class_skills`]).
    #[serde(default)]
    pub class_skills: Vec<String>,
    /// Every feature this variant grants, in source order.
    #[serde(default)]
    pub feature_grants: Vec<ClassFeatureGrant>,
    pub description: Option<String>,
    /// `SOURCEPAGE:p.27`, `None` when absent or when the value is PCGen's
    /// `p.xx` placeholder (`decisions.md §27.2`).
    pub source_page: Option<String>,
    #[serde(default)]
    pub raw_tokens: Vec<RawToken>,
    #[serde(default)]
    pub raw_bonus_chains: Vec<RawBonusChain>,
}

/// `data/corpus/<book>/class_feature/<class>/<slug>.json` payload, v1 — one
/// class feature belonging to a [`ClassVariantCacheData`].
///
/// This is **ingestion only**: the row's mechanics are preserved verbatim in
/// [`raw_tokens`](Self::raw_tokens) / [`raw_bonus_chains`](Self::raw_bonus_chains)
/// and the player-facing prose is rendered from `DESC:`. Turning a feature
/// into a working rule is the hand-modelled pure function `decisions.md §24`
/// mandates, and deliberately does not happen here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassFeatureCacheData {
    /// `KEY:Unchained Monk ~ Ki Pool` → `"Unchained Monk ~ Ki Pool"`.
    pub key: String,
    /// The unkeyed display name, e.g. `"Ki Pool"`.
    pub name: String,
    /// The owning variant's key, e.g. `"Monk ~ Unchained Class"`.
    pub class_key: String,
    /// The underlying base class's key, e.g. `"Monk"`.
    pub base_class_key: String,
    /// `CATEGORY:Special Ability`.
    pub category: Option<String>,
    /// `TYPE:Unchained Monk Class Feature.ClassFeatures.SpecialQuality` split
    /// on `.`.
    pub type_tokens: Vec<String>,
    /// The level this feature is granted at, joined from the variant's grant
    /// row. `None` when no grant row references the feature, or when the
    /// grant row states no level — never defaulted to 1.
    pub min_level: Option<u8>,
    /// True when a grant row references this feature. A feature the corpus
    /// declares but never grants is a real corpus fact, not an error, and is
    /// recorded as such rather than dropped.
    #[serde(default)]
    pub is_granted: bool,
    /// `VISIBLE:NO` marks PCGen's internal bookkeeping rows (the two
    /// "Uncanny Dodge Tracker" abilities). Carried so a UI can exclude them
    /// instead of showing a player a tracker.
    pub visible: Option<String>,
    /// `CSKILL:` on the feature row, split on `|`. Unchained Rogue and
    /// Unchained Summoner state their class-skill lists here rather than on
    /// the variant row.
    #[serde(default)]
    pub class_skills: Vec<String>,
    pub description: Option<String>,
    /// `SOURCEPAGE:p.14`, `None` when absent or `p.xx`.
    pub source_page: Option<String>,
    #[serde(default)]
    pub raw_tokens: Vec<RawToken>,
    #[serde(default)]
    pub raw_bonus_chains: Vec<RawBonusChain>,
}

/// Validates a v1 record's license annotation per `decisions.md §17`'s
/// "Validation: every record has a license field" output requirement,
/// plus the 5th-audit's PI/marker consistency rule (added 2.0.6+, but
/// checkable against the schema from this cycle onward).
pub fn validate_license<T>(record: &CorpusRecordV1<T>) -> Result<(), LicenseValidationError> {
    let Some(license) = record.license else {
        return Err(LicenseValidationError::MissingLicense);
    };
    match license {
        License::Ogl => Ok(()),
        License::Pi | License::PiRedacted => {
            if record.pi_field.is_none() {
                return Err(LicenseValidationError::MissingPiField);
            }
            if license == License::PiRedacted && record.pi_marker.as_deref() != Some(PI_MARKER_REDACTED) {
                return Err(LicenseValidationError::MissingRedactionMarker);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct SamplePayload {
        key: String,
        value: u32,
    }

    fn sample_v0_json() -> &'static str {
        // Byte-for-byte the real on-disk v0 shape (see
        // data/corpus/core_rulebook/class/*.json), with a generic
        // `SamplePayload` in place of a real `ClassCacheData` -- no
        // `license`/`pi_field`/`pi_marker` key anywhere, because no v0
        // record on disk has ever carried one.
        r#"{
            "population": "in_scope",
            "completeness": "chassis_only",
            "ingested_at": "2026-07-22T23:36:36Z",
            "data": { "key": "Bard", "value": 20 },
            "source": {
                "kind": "lst_token",
                "path": "pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst",
                "sha256": "e05eb34d4df7410d9078e35085961da6692978c02de053a8d5bf21b4389dd9b7",
                "line": 24,
                "record_key": "CLASS:Bard"
            }
        }"#
    }

    /// (a) A v0-shaped JSON value deserializes cleanly as a v1 record,
    /// with the 3 new license fields defaulting appropriately (`None`,
    /// not a fabricated `Ogl`) -- the additive proof `decisions.md §17`
    /// requires.
    #[test]
    fn v0_shaped_json_deserializes_as_v1_record_with_license_fields_defaulting_to_none() {
        let record: CorpusRecordV1<SamplePayload> =
            serde_json::from_str(sample_v0_json()).expect("a real v0 record must deserialize as a v1 record");

        assert_eq!(record.population, Population::InScope);
        assert_eq!(record.completeness, Completeness::ChassisOnly);
        assert_eq!(record.ingested_at, "2026-07-22T23:36:36Z");
        assert_eq!(record.data, SamplePayload { key: "Bard".to_string(), value: 20 });
        assert!(matches!(record.source, CorpusSource::LstToken { .. }));

        // The additive fields default to None -- "not yet classified",
        // never a silently-assumed OGL.
        assert_eq!(record.license, None);
        assert_eq!(record.pi_field, None);
        assert_eq!(record.pi_marker, None);

        // And that unclassified state is a real validation failure, not
        // a passing default -- decisions.md §17's validation requirement.
        assert_eq!(validate_license(&record), Err(LicenseValidationError::MissingLicense));
    }

    /// (b) A fully-populated v1 record round-trips through
    /// serialize/deserialize, and the 3 new fields serialize with the
    /// exact field names and string literals decisions.md §17 specifies.
    #[test]
    fn v1_record_round_trips_through_serde_with_exact_license_field_names() {
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: "2026-07-27T00:00:00Z".to_string(),
            data: SamplePayload { key: "Iomedae".to_string(), value: 1 },
            source: CorpusSource::LstToken {
                path: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_deities.lst".to_string(),
                sha256: "deadbeef".to_string(),
                line: 1,
                record_key: "DEITY:Iomedae".to_string(),
            },
            license: Some(License::PiRedacted),
            pi_field: Some("deity_name".to_string()),
            pi_marker: Some(PI_MARKER_REDACTED.to_string()),
        };

        let json = serde_json::to_value(&record).expect("v1 record must serialize");
        assert_eq!(json["license"], "PI-REDACTED");
        assert_eq!(json["pi_field"], "deity_name");
        assert_eq!(json["pi_marker"], "redacted");

        let round_tripped: CorpusRecordV1<SamplePayload> =
            serde_json::from_value(json).expect("v1 record must round-trip");
        assert_eq!(round_tripped.population, record.population);
        assert_eq!(round_tripped.completeness, record.completeness);
        assert_eq!(round_tripped.ingested_at, record.ingested_at);
        assert_eq!(round_tripped.data, record.data);
        assert_eq!(round_tripped.source, record.source);
        assert_eq!(round_tripped.license, record.license);
        assert_eq!(round_tripped.pi_field, record.pi_field);
        assert_eq!(round_tripped.pi_marker, record.pi_marker);

        assert_eq!(validate_license(&round_tripped), Ok(()));
    }

    #[test]
    fn ogl_license_serializes_to_the_literal_ogl_string() {
        assert_eq!(serde_json::to_value(License::Ogl).unwrap(), "OGL");
        assert_eq!(serde_json::to_value(License::Pi).unwrap(), "PI");
        assert_eq!(serde_json::to_value(License::PiRedacted).unwrap(), "PI-REDACTED");
    }

    #[test]
    fn validate_license_rejects_pi_without_pi_field() {
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: "2026-07-27T00:00:00Z".to_string(),
            data: SamplePayload { key: "x".to_string(), value: 0 },
            source: CorpusSource::SameBookFallback { fallback_basis: "test".to_string() },
            license: Some(License::Pi),
            pi_field: None,
            pi_marker: None,
        };
        assert_eq!(validate_license(&record), Err(LicenseValidationError::MissingPiField));
    }

    #[test]
    fn validate_license_rejects_pi_redacted_without_the_exact_redacted_marker() {
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: "2026-07-27T00:00:00Z".to_string(),
            data: SamplePayload { key: "x".to_string(), value: 0 },
            source: CorpusSource::SameBookFallback { fallback_basis: "test".to_string() },
            license: Some(License::PiRedacted),
            pi_field: Some("deity_name".to_string()),
            pi_marker: Some("not-quite-right".to_string()),
        };
        assert_eq!(validate_license(&record), Err(LicenseValidationError::MissingRedactionMarker));
    }

    #[test]
    fn validate_license_accepts_a_clean_ogl_record() {
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::ChassisOnly,
            ingested_at: "2026-07-27T00:00:00Z".to_string(),
            data: SamplePayload { key: "x".to_string(), value: 0 },
            source: CorpusSource::SameBookFallback { fallback_basis: "test".to_string() },
            license: Some(License::Ogl),
            pi_field: None,
            pi_marker: None,
        };
        assert_eq!(validate_license(&record), Ok(()));
    }

    /// Additive proof for `EquipmentCacheData`'s `raw_tokens`/
    /// `raw_bonus_chains` addition (2026-07-30 desktop-runtime-reachability
    /// finding): a real, byte-for-byte on-disk pre-existing equipment
    /// record (`data/corpus/core_rulebook/equipment/arms_armor/
    /// padded_armor_base.json`, verbatim) deserializes cleanly, with both
    /// new fields defaulting to empty `Vec`s rather than failing to parse.
    #[test]
    fn real_pre_existing_equipment_json_deserializes_with_raw_token_fields_defaulting_to_empty() {
        let real_on_disk_json = r#"{
            "population": "in_scope",
            "completeness": "full",
            "ingested_at": "2026-07-22T23:36:36Z",
            "data": {
                "key": "Padded Armor (Base)",
                "category": "arms_armor",
                "name": "Padded Armor",
                "cost_gp": 5.0,
                "weight_lbs": 10.0,
                "description": "Little more than heavy, quilted cloth, this armor provides only the most basic protection."
            },
            "source": {
                "kind": "lst_token",
                "path": "pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst",
                "sha256": "93dbb7ca43793137955851a68d745a069885e059f5eed8d77402380fea934f3a",
                "line": 37,
                "record_key": "Padded Armor (Base)"
            },
            "license": "OGL",
            "pi_field": null,
            "pi_marker": null
        }"#;

        let record: CorpusRecordV1<EquipmentCacheData> =
            serde_json::from_str(real_on_disk_json).expect("a real pre-existing on-disk record must deserialize");

        assert_eq!(record.data.key, "Padded Armor (Base)");
        assert_eq!(record.data.cost_gp, Some(5.0));
        assert_eq!(record.data.weight_lbs, Some(10.0));
        assert!(record.data.raw_tokens.is_empty(), "not-yet-regenerated record: empty, not a parse failure");
        assert!(record.data.raw_bonus_chains.is_empty());

        // And a freshly-regenerated record with real raw tokens round-trips.
        let regenerated = CorpusRecordV1 {
            data: EquipmentCacheData {
                raw_tokens: vec![
                    RawToken { key: "ACCHECK".to_string(), value: "0".to_string() },
                    RawToken { key: "MAXDEX".to_string(), value: "6".to_string() },
                ],
                raw_bonus_chains: vec![RawBonusChain {
                    qualifiers: vec!["COMBAT".to_string(), "AC".to_string(), "2".to_string(), "TYPE=Armor".to_string()],
                }],
                ..record.data.clone()
            },
            ..record
        };
        let json = serde_json::to_string(&regenerated).expect("must serialize");
        let round_tripped: CorpusRecordV1<EquipmentCacheData> =
            serde_json::from_str(&json).expect("must deserialize its own output");
        assert_eq!(round_tripped.data.raw_tokens, regenerated.data.raw_tokens);
        assert_eq!(round_tripped.data.raw_bonus_chains, regenerated.data.raw_bonus_chains);
    }
}
