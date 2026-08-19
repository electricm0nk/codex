//! SD-27 Epic 2 dual-audit gate: proves Shape B v1
//! (`src/rules_core/shape_b_v1.rs`) is additive over v0
//! (`src/rules_core/rules_tables/crb/json_cache.rs`) and that the
//! license/pi_field/pi_marker fields serialize with the exact names and
//! literal string values `decisions.md §17` specifies.
//!
//! **Updated post-retrofit (cycles E2.0.6-2.0.9).** Originally authored in
//! cycle E2.0.5, before any in-scope book had been retro-fitted, Audit 1
//! asserted every on-disk record carried `license: None` -- the honest
//! pre-retrofit state at the time. Now that all 4 in-scope books
//! (core_rulebook, advanced_players_guide, advanced_class_guide,
//! beastiary -- 4,435 records total) have been retro-fitted for real,
//! that assertion is stale by construction: it would fail on every single
//! record. Audit 1 below asserts the new, correct post-retrofit invariant
//! instead -- every on-disk record has a *populated* license and passes
//! `validate_license()` cleanly. The pre-retrofit "a v0-shaped record
//! deserializes as v1 with fields defaulting to `None`" proof still
//! exists and still matters (it's what a *future* book's pre-retrofit
//! state must satisfy) -- that proof now lives as a synthetic-fixture
//! unit test in `shape_b_v1.rs` itself
//! (`v0_shaped_json_deserializes_as_v1_record_with_license_fields_defaulting_to_none`),
//! since no real on-disk file is in that state anymore.
//!
//! The file walk also now excludes `LICENSE.json` -- each per-book retrofit
//! cycle writes one under `data/corpus/<book>/`, and its shape (book /
//! license_declaration / redaction_policy / ...) is deliberately not a
//! `CorpusRecordV1`; it is book-level metadata, not a per-record cache
//! entry, per `docs/governance/ogl-pi-blacklist.md §5`'s template.
//!
//! **Updated in cycle E3.1 (advanced_race_guide parity baseline).** The
//! walk also now excludes any path under a `_parity/` directory. No book
//! had a `_parity/` directory when this test was authored in E2.0.5 (the
//! per-book parity-baseline cycles, `loop-instruction.md §3.4`, run after
//! E2.0.5-2.0.10 and E2.1-2.2). `_parity/` holds `scripts/pcgen-normalize-output.py`'s
//! oracle-comparison output shape (`case_id`/`source_package_id`/
//! `legacy_route`/`claim_tier_floor`/`dimensions`/`diagnostics`) -- a
//! different, non-`CorpusRecordV1` schema by design, the same way
//! `LICENSE.json` is deliberately excluded above. E3.1 is the first cycle
//! to populate a `_parity/` directory for real
//! (`data/corpus/advanced_race_guide/_parity/pf_advanced_race_guide_human_fighter_level1.json`),
//! which is what surfaced this pre-existing gap in the walk.

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
            if path.file_name().and_then(|n| n.to_str()) == Some("_parity") {
                // `_parity/` holds PCGen-normalized oracle-comparison output
                // (loop-instruction.md §3.4's `pf_<book>_human_<class>_level1.json`),
                // not Shape B v1 corpus records -- deliberately excluded, same
                // rationale as the `LICENSE.json` exclusion below.
                continue;
            }
            out.extend(json_files_under(&path));
        } else if path.extension().and_then(|e| e.to_str()) == Some("json")
            && path.file_name().and_then(|n| n.to_str()) != Some("LICENSE.json")
        {
            // LICENSE.json is book-level metadata (docs/governance/ogl-pi-blacklist.md
            // §5's template), not a per-record Shape B cache entry -- it does not
            // conform to CorpusRecordV1 and is deliberately excluded from this walk.
            out.push(path);
        }
    }
    out
}

/// Audit 1: every real, on-disk Shape B record across all 4 in-scope
/// books deserializes as a Shape B v1 record (`serde_json::Value` for
/// `data`, since this gate is schema-generic, not book-specific) *and*
/// carries a populated, `validate_license()`-clean license annotation --
/// the post-retrofit state every in-scope book's data must be in once its
/// own E2.0.6-2.0.9 cycle has landed.
#[test]
fn every_on_disk_record_is_a_valid_v1_record_with_populated_license() {
    let root = corpus_root();
    let files = json_files_under(&root);
    assert!(
        !files.is_empty(),
        "expected real, non-empty Shape B v1 data under {root:?} to audit"
    );

    let mut audited = 0u32;
    let mut ogl = 0u32;
    let mut redacted = 0u32;
    for path in &files {
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
        let record: CorpusRecordV1<serde_json::Value> = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("{path:?} is a real Shape B record but failed to deserialize as v1: {e}"));

        // Every in-scope book has been through its license-stripping
        // retro-fit (E2.0.6-2.0.9) -- an on-disk record with no license
        // annotation at this point is a real gap, not an expected state.
        validate_license(&record).unwrap_or_else(|e| {
            panic!("{path:?}: retro-fitted record must pass validate_license(), got: {e}")
        });

        match record.license {
            Some(License::Ogl) => ogl += 1,
            Some(License::Pi) | Some(License::PiRedacted) => redacted += 1,
            None => panic!("{path:?}: retro-fitted record must carry a populated license"),
        }

        // Sanity: the v0-inherited fields themselves came through intact.
        assert!(!record.ingested_at.is_empty(), "{path:?}: ingested_at must survive the v1 parse");
        assert!(record.data.is_object(), "{path:?}: data payload must survive the v1 parse");

        audited += 1;
    }

    // Regression guard: this must actually audit the real corpus, not an
    // empty glob silently passing.
    assert!(
        audited >= 4000,
        "expected to audit the full 4,435-record in-scope corpus, found only {audited}"
    );
    assert_eq!(
        audited,
        ogl + redacted,
        "every audited record must be classified as either OGL or PI/PI-REDACTED"
    );
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
    wiring_class: String::new(),
    wiring_class_signals: Vec::new(),
    description_source: None,
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
    wiring_class: String::new(),
    wiring_class_signals: Vec::new(),
    description_source: None,
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
    wiring_class: String::new(),
    wiring_class_signals: Vec::new(),
    description_source: None,
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
