//! SD-27 Epic 2, cycle E2.0.5 dual-audit gate: proves Shape B v1
//! (`src/rules_core/shape_b_v1.rs`) is additive over v0
//! (`src/rules_core/rules_tables/crb/json_cache.rs`) and that the
//! license/pi_field/pi_marker fields serialize with the exact names and
//! literal string values `decisions.md §17` specifies.
//!
//! This cycle does NOT modify any in-scope book's on-disk records (see
//! `loop-instruction.md §3.2.5` Notes) -- this gate audits the *schema*,
//! not book data. Per-book data retro-fits (E2.0.6+) get their own
//! per-book dual-audit gates against this same shared schema.

use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::shape_b_v1::{
    validate_license, CorpusRecordV1, CorpusSource, License, LicenseValidationError,
    PI_MARKER_REDACTED, REDACTED_PI_MARKER,
};

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

fn json_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(json_files_under(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out
}

/// Audit 1: every real, on-disk Shape B record across all 4 in-scope
/// books deserializes as a Shape B v1 record (`serde_json::Value` for
/// `data`, since this gate is schema-generic, not book-specific) -- the
/// additive proof that a v0 record is also a valid v1 record.
#[test]
fn every_on_disk_v0_record_is_also_a_valid_v1_record() {
    let root = corpus_root();
    let files = json_files_under(&root);
    assert!(
        !files.is_empty(),
        "expected real, non-empty Shape B v0 data under {root:?} to audit"
    );

    let mut audited = 0u32;
    for path in &files {
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
        let record: CorpusRecordV1<serde_json::Value> = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("{path:?} is a real Shape B v0 record but failed to deserialize as v1: {e}"));

        // Every v0 record on disk today predates the license-stripping
        // retro-fit, so license classification is legitimately absent --
        // additive means "still parses," not "already classified."
        assert_eq!(
            record.license, None,
            "{path:?}: an un-retro-fitted v0 record should carry no license annotation yet"
        );
        assert_eq!(record.pi_field, None);
        assert_eq!(record.pi_marker, None);

        // Sanity: the v0 fields themselves came through intact.
        assert!(!record.ingested_at.is_empty(), "{path:?}: ingested_at must survive the v1 parse");
        assert!(record.data.is_object(), "{path:?}: data payload must survive the v1 parse");

        audited += 1;
    }

    // Regression guard: this must actually audit the real corpus, not an
    // empty glob silently passing.
    assert!(audited >= 3, "expected to audit at least 3 real on-disk records, found {audited}");
}

/// Audit 2: the v1 type round-trips through serde (construct -> serialize
/// -> deserialize -> compare), independent of any file on disk.
#[test]
fn v1_type_round_trips_through_serde() {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload {
        name: String,
    }

    let record = CorpusRecordV1 {
        population: codex::rules_core::shape_b_v1::Population::InScope,
        completeness: codex::rules_core::shape_b_v1::Completeness::Full,
        ingested_at: "2026-07-27T00:00:00Z".to_string(),
        data: Payload { name: "Test Deity".to_string() },
        source: CorpusSource::LstToken {
            path: "pathfinder/paizo/roleplaying_game/core_rulebook/cr_deities.lst".to_string(),
            sha256: "abc123".to_string(),
            line: 10,
            record_key: "DEITY:Test".to_string(),
        },
        license: Some(License::PiRedacted),
        pi_field: Some("deity_name".to_string()),
        pi_marker: Some(PI_MARKER_REDACTED.to_string()),
    };

    let json_text = serde_json::to_string(&record).expect("v1 record must serialize to a JSON string");
    let round_tripped: CorpusRecordV1<Payload> =
        serde_json::from_str(&json_text).expect("serialized v1 record must deserialize back");

    assert_eq!(round_tripped.population, record.population);
    assert_eq!(round_tripped.completeness, record.completeness);
    assert_eq!(round_tripped.ingested_at, record.ingested_at);
    assert_eq!(round_tripped.data, record.data);
    assert_eq!(round_tripped.source, record.source);
    assert_eq!(round_tripped.license, record.license);
    assert_eq!(round_tripped.pi_field, record.pi_field);
    assert_eq!(round_tripped.pi_marker, record.pi_marker);
}

/// Audit 3: `license`/`pi_field`/`pi_marker` serialize under exactly the
/// field names and literal string values `decisions.md §17` specifies --
/// `"license": "OGL" | "PI" | "PI-REDACTED"`, `"pi_field": <name>`,
/// `"pi_marker": "redacted"`.
#[test]
fn license_fields_serialize_with_the_exact_names_and_literals_decisions_md_17_specifies() {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload {
        n: u8,
    }

    let make = |license: License, pi_field: Option<&str>, pi_marker: Option<&str>| CorpusRecordV1 {
        population: codex::rules_core::shape_b_v1::Population::InScope,
        completeness: codex::rules_core::shape_b_v1::Completeness::ChassisOnly,
        ingested_at: "2026-07-27T00:00:00Z".to_string(),
        data: Payload { n: 1 },
        source: CorpusSource::SameBookFallback { fallback_basis: "audit fixture".to_string() },
        license: Some(license),
        pi_field: pi_field.map(str::to_string),
        pi_marker: pi_marker.map(str::to_string),
    };

    let ogl = serde_json::to_value(make(License::Ogl, None, None)).unwrap();
    assert_eq!(ogl["license"], "OGL");
    assert_eq!(ogl["pi_field"], serde_json::Value::Null);
    assert_eq!(ogl["pi_marker"], serde_json::Value::Null);

    let pi = serde_json::to_value(make(License::Pi, Some("deity_name"), None)).unwrap();
    assert_eq!(pi["license"], "PI");
    assert_eq!(pi["pi_field"], "deity_name");

    let redacted = serde_json::to_value(make(
        License::PiRedacted,
        Some("deity_name"),
        Some(PI_MARKER_REDACTED),
    ))
    .unwrap();
    assert_eq!(redacted["license"], "PI-REDACTED");
    assert_eq!(redacted["pi_field"], "deity_name");
    assert_eq!(redacted["pi_marker"], "redacted");

    // The literal redaction marker constant matches decisions.md §17's
    // pinned value exactly.
    assert_eq!(REDACTED_PI_MARKER, "[redacted PI]");
    assert_eq!(PI_MARKER_REDACTED, "redacted");
}

/// `decisions.md §17`'s validation requirement: "every record has a
/// license field." A record missing `license` fails validation (this is
/// the honest state of every un-retro-fitted v0 record today); a clean
/// OGL record and a properly-redacted PI record both pass.
#[test]
fn validate_license_enforces_the_decisions_md_17_validation_requirement() {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload {
        n: u8,
    }
    let base = |license, pi_field: Option<&str>, pi_marker: Option<&str>| CorpusRecordV1 {
        population: codex::rules_core::shape_b_v1::Population::InScope,
        completeness: codex::rules_core::shape_b_v1::Completeness::ChassisOnly,
        ingested_at: "2026-07-27T00:00:00Z".to_string(),
        data: Payload { n: 1 },
        source: CorpusSource::SameBookFallback { fallback_basis: "audit fixture".to_string() },
        license,
        pi_field: pi_field.map(str::to_string),
        pi_marker: pi_marker.map(str::to_string),
    };

    assert_eq!(
        validate_license(&base(None, None, None)),
        Err(LicenseValidationError::MissingLicense)
    );
    assert_eq!(validate_license(&base(Some(License::Ogl), None, None)), Ok(()));
    assert_eq!(
        validate_license(&base(Some(License::Pi), None, None)),
        Err(LicenseValidationError::MissingPiField)
    );
    assert_eq!(
        validate_license(&base(Some(License::PiRedacted), Some("deity_name"), Some(PI_MARKER_REDACTED))),
        Ok(())
    );
}
