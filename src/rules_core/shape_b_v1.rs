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
}
