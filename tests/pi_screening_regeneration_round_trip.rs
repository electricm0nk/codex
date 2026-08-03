//! Proves a regeneration of CRB/ACG/APG's spell and equipment records
//! reproduces the SAME `license`/`pi_field`/`pi_marker` classification
//! already on disk today (the retrofit-script output, `bb497db0`), now
//! that the generators screen inline via `rules_core::pi_screening`
//! instead of relying on a post-hoc pass they knew nothing about.
//!
//! **Deliberately does not run any generator binary or write a scratch
//! directory.** Every generator's `data` construction pulls a record's
//! free-text field straight from the SAME compiled `rules_tables::<book>`
//! accessor this test calls directly (`SPELL_LIST`, `equipment_tables()`)
//! -- so recomputing `pi_screening::classify_field` against that same
//! source text and comparing to what is on disk today is the round-trip
//! proof, without the added surface of spinning up a generator run.
//!
//! **Scope, per the operator's explicit design (regeneration is a
//! separate, later step; this is proving the SCREENING is safe first):**
//! this covers `license`/`pi_field`/`pi_marker` only.
//! `raw_tokens`/`raw_bonus_chains` are OUT of scope here on purpose --
//! those fields are owned by `enrich_equipment_raw_tokens.rs`'s separate,
//! deliberate post-hoc pass (see that binary's own module doc comment for
//! why a typed-struct convergence already failed once on exactly that
//! field pair), not something a bare generator run was ever meant to
//! produce. Do not "fix" the generator to emit them; run the enrichment
//! pass after regenerating instead.
//!
//! **Scoped by default; exhaustive behind `PI_ROUND_TRIP_FULL_SWEEP=1`.**
//! Running the full corpus is the more expensive default this suite would
//! rather avoid always paying (`cargo test`'s own bottleneck, per
//! `scripts/verify.sh`'s doc comments) -- the default run checks every
//! record whose on-disk `license` is anything other than plain `"OGL"`
//! (every place a redaction actually lives) plus a representative
//! per-book/per-kind sample, which still catches a classification
//! regression on any already-redacted record. Set the env var for the
//! exhaustive sweep a regeneration cycle should run before committing.
//! (In practice the full sweep over all 4,049 CRB/ACG/APG spell +
//! equipment records runs in well under a second, so this default exists
//! as a deliberate safety margin against the corpus growing, not because
//! the exhaustive check is currently expensive.)
//!
//! **The boundary of what this harness proves, stated explicitly so the
//! next reader does not have to infer it.** It covers CRB, ACG, and APG's
//! spell/equipment records ONLY -- every writer whose free-text source is
//! a static compiled `rules_tables` table. `ingest_race_traits_arg.rs`'s
//! 156 ARG racial-trait records are NOT covered here: that binary's
//! `description` text comes from live PCGen `.lst` parsing at ingest
//! time, not a static table this test could call and recompute against
//! independently. Those 156 records are protected only by that binary's
//! own inline `pi_screening::classify_optional_field` call (added this
//! same cycle, closing a previously-documented gap where the binary
//! classified every record `OGL` without running the term scan at all) --
//! not by an independent round-trip proof the way CRB/ACG/APG are here.
//! That is an honest, accepted limitation (156 records, externally
//! verified at 0 PI hits on 2026-07-31), not a silent one. Extending this
//! harness to cover them would require either re-parsing the same raw
//! `.lst` file this test does not otherwise touch, or refactoring that
//! binary's `main()` into a callable library function purely for testing
//! -- neither was in scope for this cycle.

use std::collections::BTreeMap;
use std::path::PathBuf;


/// Real, known, individually-justified exception: the 9 APG Summoner
/// `Summon Monster I`-`IX` spells were never generated to disk in this
/// checkout at all (a real gap in APG's own generation, not something
/// this test or the license work introduced or should paper over) --
/// named explicitly per the operator's instruction that a genuine
/// difference is a finding to report, not something the test absorbs
/// silently. `crb_apg_acg_license_classification_round_trips_...` skips
/// these by name; it does not weaken the "missing from disk" check for
/// anything else.
const KNOWN_MISSING_FROM_DISK: &[(&str, &str, &str)] = &[
    ("advanced_players_guide", "spell", "Summoner Summon Monster I"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster II"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster III"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster IV"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster V"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster VI"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster VII"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster VIII"),
    ("advanced_players_guide", "spell", "Summoner Summon Monster IX"),
];

use codex::rules_core::pi_screening;
use codex::rules_core::rules_tables::acg;
use codex::rules_core::rules_tables::apg;
use codex::rules_core::rules_tables::crb::{equipment_tables as crb_equipment_tables, spell_list as crb_spell_list};
use codex::rules_core::shape_b_v1::License;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

fn full_sweep() -> bool {
    std::env::var("PI_ROUND_TRIP_FULL_SWEEP").as_deref() == Ok("1")
}

/// One real, on-disk record's `license`/`pi_field`/`pi_marker`, keyed by
/// the record's own `data.key` -- every generator in scope here stores the
/// real corpus identity there, so this is a stable join key independent of
/// each generator's own (collision-numbered) filename slug.
fn load_on_disk(book: &str, kind: &str) -> BTreeMap<String, (Option<String>, Option<String>, Option<String>, Option<String>)> {
    let dir = corpus_dir().join(book).join(kind);
    let mut out = BTreeMap::new();
    let Ok(entries) = std::fs::read_dir(&dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(key) = json["data"]["key"].as_str() else { continue };
        out.insert(
            key.to_string(),
            (
                json["license"].as_str().map(str::to_string),
                json["pi_field"].as_str().map(str::to_string),
                json["pi_marker"].as_str().map(str::to_string),
                json["data"]["description"].as_str().map(str::to_string),
            ),
        );
    }
    out
}

fn license_str(l: License) -> &'static str {
    match l {
        License::Ogl => "OGL",
        License::Pi => "PI",
        License::PiRedacted => "PI-REDACTED",
    }
}

/// Checks one (key, real source description) pair against on-disk state,
/// pushing a keyed mismatch string (never a bulk count) on disagreement.
/// `restrict_to_non_ogl_and_sample`: when true (the default, non-full-sweep
/// run), only checks a key if its on-disk record is already non-OGL, OR it
/// is one of the first `sample_n` keys encountered for this book/kind --
/// the narrow-by-default, still-covers-every-redaction scope the operator
/// asked for.
#[allow(clippy::too_many_arguments)]
fn check_book_kind(
    book: &str,
    kind: &str,
    real_entries: &[(&str, Option<&str>)], // (key, real description text)
    mismatches: &mut Vec<String>,
    missing_from_disk: &mut Vec<String>,
) {
    let on_disk = load_on_disk(book, kind);
    if on_disk.is_empty() {
        // Nothing generated for this book/kind yet in this checkout --
        // not this test's problem; the sd26/sd27 shape tests own record
        // *counts*. Round-trip has nothing to prove against an empty set.
        return;
    }
    let sweep = full_sweep();
    let sample_n = 15usize;
    let mut checked_sample = 0usize;

    for (key, real_desc) in real_entries {
        let Some((stored_license, stored_pi_field, stored_pi_marker, stored_desc)) = on_disk.get(*key) else {
            if KNOWN_MISSING_FROM_DISK.contains(&(book, kind, *key)) {
                continue;
            }
            missing_from_disk.push(format!("{book}/{kind}: real record {key:?} has no on-disk file"));
            continue;
        };
        let is_non_ogl = stored_license.as_deref().is_some_and(|l| l != "OGL");
        let in_sample = checked_sample < sample_n;
        if !sweep && !is_non_ogl && !in_sample {
            continue;
        }
        if in_sample {
            checked_sample += 1;
        }

        let (want_license, want_pi_field, want_pi_marker, want_desc) =
            pi_screening::classify_optional_field("description", *real_desc);
        let want_license_str = license_str(want_license);

        if stored_license.as_deref() != Some(want_license_str) {
            mismatches.push(format!(
                "{book}/{kind}/{key}: license on disk = {stored_license:?}, recomputed = {want_license_str:?}"
            ));
        }
        if stored_pi_field.as_deref() != want_pi_field.as_deref() {
            mismatches.push(format!(
                "{book}/{kind}/{key}: pi_field on disk = {stored_pi_field:?}, recomputed = {want_pi_field:?}"
            ));
        }
        if stored_pi_marker.as_deref() != want_pi_marker.as_deref() {
            mismatches.push(format!(
                "{book}/{kind}/{key}: pi_marker on disk = {stored_pi_marker:?}, recomputed = {want_pi_marker:?}"
            ));
        }
        if stored_desc.as_deref() != want_desc.as_deref() {
            mismatches.push(format!(
                "{book}/{kind}/{key}: description on disk = {stored_desc:?}, recomputed = {want_desc:?}"
            ));
        }
    }
}

#[test]
fn crb_apg_acg_license_classification_round_trips_against_the_compiled_source_text() {
    let mut mismatches = Vec::new();
    let mut missing = Vec::new();

    let crb_spells: Vec<(&str, Option<&str>)> =
        crb_spell_list::SPELL_LIST.iter().map(|e| (e.key, Some(e.description))).collect();
    check_book_kind("core_rulebook", "spell", &crb_spells, &mut mismatches, &mut missing);

    let crb_equipment: Vec<(&str, Option<&str>)> =
        crb_equipment_tables::equipment_tables().iter().map(|e| (e.key, e.description)).collect();
    check_book_kind("core_rulebook", "equipment", &crb_equipment, &mut mismatches, &mut missing);

    let acg_spells: Vec<(&str, Option<&str>)> =
        acg::spell_list::SPELL_LIST.iter().map(|e| (e.key, Some(e.description))).collect();
    check_book_kind("advanced_class_guide", "spell", &acg_spells, &mut mismatches, &mut missing);

    let acg_equipment: Vec<(&str, Option<&str>)> =
        acg::equipment_tables::equipment_tables().iter().map(|e| (e.key, e.description)).collect();
    check_book_kind("advanced_class_guide", "equipment", &acg_equipment, &mut mismatches, &mut missing);

    let apg_spells: Vec<(&str, Option<&str>)> =
        apg::spell_list::SPELL_LIST.iter().map(|e| (e.key, e.description)).collect();
    check_book_kind("advanced_players_guide", "spell", &apg_spells, &mut mismatches, &mut missing);

    let apg_equipment: Vec<(&str, Option<&str>)> =
        apg::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| (e.key, e.description)).collect();
    check_book_kind("advanced_players_guide", "equipment", &apg_equipment, &mut mismatches, &mut missing);

    assert!(
        missing.is_empty(),
        "records present in the compiled source but missing from disk (a record that silently \
         stopped being generated is the dangerous direction):\n{}",
        missing.join("\n")
    );
    assert!(
        mismatches.is_empty(),
        "{} field-level round-trip mismatches (each is a real finding, not a bulk count):\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

/// The dedicated fixture the operator named: a real `PI-REDACTED` record
/// must never silently lose its redaction. Independent of the sweep
/// above so this one specific, high-stakes case is never skipped by the
/// narrow-by-default scoping.
#[test]
fn discern_next_of_kin_pi_redaction_survives_reclassification() {
    let path = corpus_dir().join("advanced_class_guide/spell/discern_next_of_kin.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("fixture record must exist on disk at {path:?}: {e}"));
    let json: serde_json::Value = serde_json::from_str(&text).expect("fixture must be valid JSON");
    assert_eq!(json["license"], "PI-REDACTED", "this record must already be on-disk PI-REDACTED");
    assert_eq!(json["data"]["description"], codex::rules_core::shape_b_v1::REDACTED_PI_MARKER);

    let entry = acg::spell_list::SPELL_LIST
        .iter()
        .find(|e| e.key == "Discern Next of Kin")
        .expect("Discern Next of Kin must exist in the compiled ACG spell table");
    let (license, pi_field, pi_marker, desc) = pi_screening::classify_field("description", entry.description);
    assert_eq!(license_str(license), "PI-REDACTED", "reclassification must still redact");
    assert_eq!(pi_field.as_deref(), Some("description"));
    assert!(pi_marker.is_some());
    assert_eq!(desc, json["data"]["description"].as_str().unwrap());
}
