//! The **ingest cache payload** half of Shape B v1 — the per-content-kind `data`
//! payload types a PCGen `.lst` row is converted into at ingest, and the two
//! verbatim-token carriers (`RawToken`, `RawBonusChain`) those payloads hold.
//!
//! **Why this lives here and not in `rules_core::shape_b_v1`** (SD-35
//! `AT-35-E6-002` cycle 4, `decisions.md` §11, `technical-design.md` §0). These
//! types are the *converter's* output format: every field on them is transcribed
//! off a PCGen token by `src/pcgen_import/cache_gen/**` and `src/bin/ingest_*`,
//! and `RawToken`/`RawBonusChain` are literally the ingest format's verbatim
//! token arrays. They sat in `src/rules_core/` only because that is where the
//! schema module was first authored; the record *envelope* (`CorpusRecordV1`,
//! `License`, `Population`, `Completeness`, `CorpusSource`, the PI markers) is
//! genuinely ours and stays there, and it is generic over `T` precisely so the
//! payload can live on the side that produces it.
//!
//! The move is behaviour-identical: the type definitions, their serde derives,
//! their field order and their doc comments are transcribed unchanged, so every
//! on-disk `data/corpus/**` record serializes and deserializes byte-for-byte as
//! before. Only the import path consumers name has changed. KEPT for Starfinder.

use serde::{Deserialize, Serialize};

/// The on-disk JSON field name of a record payload's verbatim ingest token
/// array — the single definition of that wire name.
///
/// It exists so a test or an audit that must speak the *ingest format's* field
/// name cites it from the converter side rather than hard-coding a PCGen-shaped
/// literal inside `src/rules_core/` (SD-35 `decisions.md` §11). Changing the
/// wire name means changing [`RawToken`]'s field on every payload struct below
/// and regenerating the corpus; this constant is not a knob.
pub const INGEST_TOKENS_FIELD: &str = "raw_tokens";

/// The ingest token array, as the JSON value an on-disk record payload carries
/// under [`INGEST_TOKENS_FIELD`].
///
/// Callers that need to *speak the ingest format* — a test pinning the literal
/// on-disk shape, an audit fixture — build it here instead of hand-writing a
/// PCGen-shaped JSON literal, so the wire name has exactly one definition and
/// it lives on the converter side (SD-35 `decisions.md` §11).
pub fn ingest_tokens_value(tokens: &[(&str, &str)]) -> serde_json::Value {
    serde_json::Value::Array(
        tokens
            .iter()
            .map(|(key, value)| serde_json::json!({ "key": key, "value": value }))
            .collect(),
    )
}

/// The full on-disk JSON text of a record whose payload is just a key plus an
/// ingest token array — the minimal shape a `data/corpus/**` file has, and the
/// shape scratch-corpus fixtures write. See [`ingest_tokens_value`].
pub fn ingest_record_json(key: &str, tokens: &[(&str, &str)]) -> String {
    serde_json::json!({
        "data": { "key": key, INGEST_TOKENS_FIELD: ingest_tokens_value(tokens) }
    })
    .to_string()
}

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
/// json_cache`, `gen_book_cache`'s own local copy -- byte-identical to
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
/// copies and `gen_book_cache.rs`'s local struct should import this
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::shape_b_v1::CorpusRecordV1;

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
