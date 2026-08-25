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
//! a static compiled `rules_tables` table. `ingest_race_traits.rs`'s
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
/// each generator's own (collision-numbered) filename slug. Also carries
/// `completeness` and the record's `raw_tokens` key set, needed only by
/// `is_known_mod_access_residue` below (see its own doc comment) -- every
/// OTHER caller in this file only ever reads the first four fields, same
/// as before that predicate was added.
struct OnDiskFields {
    license: Option<String>,
    pi_field: Option<String>,
    pi_marker: Option<String>,
    description: Option<String>,
    completeness: Option<String>,
    raw_token_keys: Vec<String>,
}

fn load_on_disk(book: &str, kind: &str) -> BTreeMap<String, OnDiskFields> {
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
        let raw_token_keys = json["data"]["raw_tokens"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|t| t["key"].as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        out.insert(
            key.to_string(),
            OnDiskFields {
                license: json["license"].as_str().map(str::to_string),
                pi_field: json["pi_field"].as_str().map(str::to_string),
                pi_marker: json["pi_marker"].as_str().map(str::to_string),
                description: json["data"]["description"].as_str().map(str::to_string),
                completeness: json["completeness"].as_str().map(str::to_string),
                raw_token_keys,
            },
        );
    }
    out
}

/// `cache_gen::spell_mod_access` (commit `22212f87ee`, SD-32 `decisions.md
/// §20`) is a SECOND, real, oracle-cited generator writing into this SAME
/// `<book>/spell/` directory across exactly its 3 `BOOK_SPECS` books
/// (`occult_adventures`, `ultimate_magic`, `advanced_players_guide`) --
/// the identical shape `sd26_cache_core_rulebook.rs`'s own class/equipment
/// tests document for `ingest_class.py`/`equipment_gap_tables`. Its rows
/// dump a `.MOD` line that widens an EXISTING spell's class access
/// (`Occultist Spell ~ Accelerate Poison.MOD ... CLASSES:Occultist=2`) --
/// never a NEW spell declaration, so these correctly have no entry in
/// `apg::spell_list::SPELL_LIST`/etc, which only ever modelled real base
/// declarations. Distinguished from a genuine base declaration by a real
/// PCGen domain invariant, not by name-listing 491 individual keys: a
/// `.MOD` access-widening row never carries its own `SCHOOL:` token (only
/// a base spell declaration does; `parse_mod_row`'s own `has_classes`
/// gate requires `CLASSES:` and never inspects `SCHOOL:`), and
/// `spell_mod_access::generate` never populates `description` unless the
/// row itself carries a real `DESC:` token distinct from `.CLEAR`
/// (`completeness` stays `chassis_only` when it does not). Verified this
/// cycle to hold with ZERO exceptions across the current on-disk state:
/// `python3 -c "import json,glob; [print(f) for f in \
/// glob.glob('data/corpus/advanced_players_guide/spell/*.json') if \
/// (d:=json.load(open(f)))['completeness']=='chassis_only' and \
/// d['data']['description'] is None and 'SCHOOL' in \
/// [t['key'] for t in d['data']['raw_tokens']]]"` -> no output (0 rows
/// match the residue shape while also carrying a `SCHOOL:` token).
fn is_known_mod_access_residue(book: &str, kind: &str, fields: &OnDiskFields) -> bool {
    if kind != "spell" || !matches!(book, "occult_adventures" | "ultimate_magic" | "advanced_players_guide") {
        return false;
    }
    fields.completeness.as_deref() == Some("chassis_only")
        && fields.description.is_none()
        && fields.raw_token_keys.iter().any(|k| k == "CLASSES")
        && !fields.raw_token_keys.iter().any(|k| k == "SCHOOL")
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
    stale_on_disk: &mut Vec<String>,
    mod_access_residue_count: &mut usize,
) {
    let on_disk = load_on_disk(book, kind);
    if on_disk.is_empty() {
        // Nothing generated for this book/kind yet in this checkout --
        // not this test's problem; the sd26/sd27 shape tests own record
        // *counts*. Round-trip has nothing to prove against an empty set.
        return;
    }

    // The OTHER direction of the set difference: a record on disk with NO
    // corresponding real-table entry. This is the dangerous direction --
    // "a record silently stops being generated" -- and was NOT checked
    // here until 2026-08-03, when 9 stale advanced_players_guide
    // `summon_monster_{i..ix}.json` records (last written by bb497db0,
    // citing content-free `.MOD` bookkeeping rows -- `apg_spells.lst:695`
    // `Summon Monster I.MOD  CLASSES:Witch=1`, no DESC:) survived
    // undetected through a full regeneration because the forward-only
    // loop below only ever asks "is every REAL entry present on disk",
    // never "is every ON-DISK entry real". APG does not declare
    // `Summon Monster I` at all -- it only `.MOD`s CRB's own copy to add
    // Witch to its class list -- so these were never real APG records;
    // the real one lives at
    // `data/corpus/core_rulebook/spell/level_1/summon_monster_i.json`.
    //
    // SD-32 widening: `cache_gen::spell_mod_access` (see
    // `is_known_mod_access_residue`'s own doc comment) writes MANY more
    // such real `.MOD`-widening rows into these same 3 books' `spell/`
    // directories. Those are counted separately, into
    // `mod_access_residue_count`, rather than flagged as `stale` --
    // real, oracle-cited content the narrow `real_entries` base-
    // declaration list was never meant to model, not a "second
    // Summon Monster" defect.
    let real_keys: std::collections::BTreeSet<&str> = real_entries.iter().map(|(k, _)| *k).collect();
    for (on_disk_key, fields) in on_disk.iter() {
        if real_keys.contains(on_disk_key.as_str()) {
            continue;
        }
        if is_known_mod_access_residue(book, kind, fields) {
            *mod_access_residue_count += 1;
            continue;
        }
        stale_on_disk.push(format!(
            "{book}/{kind}: on-disk record {on_disk_key:?} has no corresponding real table entry \
             (the generator would not write this record today -- a stale leftover, not a gap)"
        ));
    }

    let sweep = full_sweep();
    let sample_n = 15usize;
    let mut checked_sample = 0usize;

    for (key, real_desc) in real_entries {
        let Some(OnDiskFields { license: stored_license, pi_field: stored_pi_field, pi_marker: stored_pi_marker, description: stored_desc, .. }) = on_disk.get(*key) else {
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
    let mut stale = Vec::new();
    let mut mod_access_residue_count = 0usize;

    let crb_spells: Vec<(&str, Option<&str>)> =
        crb_spell_list::SPELL_LIST.iter().map(|e| (e.key, Some(e.description))).collect();
    check_book_kind(
        "core_rulebook",
        "spell",
        &crb_spells,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    let crb_equipment: Vec<(&str, Option<&str>)> =
        crb_equipment_tables::equipment_tables().iter().map(|e| (e.key, e.description)).collect();
    check_book_kind(
        "core_rulebook",
        "equipment",
        &crb_equipment,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    let acg_spells: Vec<(&str, Option<&str>)> =
        acg::spell_list::SPELL_LIST.iter().map(|e| (e.key, Some(e.description))).collect();
    check_book_kind(
        "advanced_class_guide",
        "spell",
        &acg_spells,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    let acg_equipment: Vec<(&str, Option<&str>)> =
        acg::equipment_tables::equipment_tables().iter().map(|e| (e.key, e.description)).collect();
    check_book_kind(
        "advanced_class_guide",
        "equipment",
        &acg_equipment,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    let apg_spells: Vec<(&str, Option<&str>)> =
        apg::spell_list::SPELL_LIST.iter().map(|e| (e.key, e.description)).collect();
    check_book_kind(
        "advanced_players_guide",
        "spell",
        &apg_spells,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    let apg_equipment: Vec<(&str, Option<&str>)> =
        apg::equipment_tables::EQUIPMENT_TABLE.iter().map(|e| (e.key, e.description)).collect();
    check_book_kind(
        "advanced_players_guide",
        "equipment",
        &apg_equipment,
        &mut mismatches,
        &mut missing,
        &mut stale,
        &mut mod_access_residue_count,
    );

    assert!(
        missing.is_empty(),
        "records present in the compiled source but missing from disk (a record that silently \
         stopped being generated is the dangerous direction):\n{}",
        missing.join("\n")
    );
    assert!(
        stale.is_empty(),
        "{} on-disk records have no corresponding real table entry (a stale leftover the \
         generator would not produce today -- the OTHER dangerous direction, see \
         KNOWN_MISSING_FROM_DISK's sibling investigation):\n{}",
        stale.len(),
        stale.join("\n")
    );
    // `cache_gen::spell_mod_access`'s real, oracle-cited `.MOD` class-
    // access-widening residue (see `is_known_mod_access_residue`'s doc
    // comment) -- re-derived fresh, not repinned to whatever the detector
    // happens to find. `load_on_disk` keys its map by `data.key`, so this
    // count is UNIQUE KEYS, not file instances (some keys, e.g.
    // "True Seeing", have one file per widened class -- `true_seeing.json`
    // /`-3`/`-4` -- 648 file instances collapse to 491 unique keys here):
    // `python3 -c "import json,glob; \
    // print(len({json.load(open(f))['data']['key'] for f in \
    // glob.glob('data/corpus/advanced_players_guide/spell/*.json') if \
    // json.load(open(f))['completeness']=='chassis_only' and \
    // json.load(open(f))['data']['description'] is None}))"` -> 491.
    // Of this test's 6 books/kinds, only `advanced_players_guide/spell`
    // carries this residue -- `spell_mod_access::BOOK_SPECS` never
    // targets `core_rulebook`/`advanced_class_guide`'s directories. All
    // 491 confirmed written by the single `22212f87ee` generator run (one
    // shared `ingested_at` timestamp; `git log -- \
    // data/corpus/advanced_players_guide/spell/` shows exactly one commit
    // after the original 297-record cache). This assertion is the gate
    // that a change WOULD move: an unrelated future regression should
    // change this count, and a real reader should notice and re-derive it
    // rather than the assertion silently widening on its own.
    assert_eq!(
        mod_access_residue_count, 491,
        "spell_mod_access residue count drifted from the re-derived 491 unique keys -- re-run \
         the python3 one-liner in this assertion's own comment against current disk state \
         before changing this pin"
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
