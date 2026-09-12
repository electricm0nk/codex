//! The two library-side `class_feature` catalogs serve the converted package's words, not a
//! run-time render of the ingest format's own description string.
//!
//! SD-35 Epic 6, `AT-35-E6-003-SWEEP` cycle 17. `decisions.md §11` forbids the live side from
//! reading a PCGen token or formula string; `epic-breakdown.md`'s `AT-35-E6-003` states the
//! consequence for this surface exactly — *"`render_pcgen_desc` is deleted from the live side;
//! its `%N` substitution already happened in the converter."*
//!
//! Until cycle 17 both `class_feature_pool_catalog::load_pool_catalog` and
//! `pilot_compute::class_feature_grant_consumer`'s own corpus walk took
//! `data.description` — the ingest format's `DESC:` row, `%N` slots and `|`-argument tail
//! included — and substituted it at run time, on the way to a character sheet.
//!
//! # What this gate asserts, and why it is a corpus-wide walk and not a fixture
//!
//! `decisions.md §4`: the proof reads the live corpus directory and the live converted package.
//! For **every** record either catalog serves, the converted package must state prose for the
//! same record through `converted_prose`'s join. A record the package cannot answer for would be
//! a description a player can read today and could not read after the rewire, which is a
//! regression in `docs/work-inventory.json` and not a PCGen exit.
//!
//! It refuses to pass on an empty walk: a catalog that returned nothing would otherwise agree
//! with this test about nothing at all.

use codex::rules_core::class_feature_pool_catalog::{
    load_pool_catalog, load_standalone_class_feature_catalog,
};
use codex::rules_core::converted_prose;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn every_record_the_class_feature_catalogs_serve_has_converted_prose() {
    let root = repo_root();
    let mut served = load_pool_catalog(&root);
    served.extend(load_standalone_class_feature_catalog(&root));

    assert!(
        served.len() > 500,
        "the class_feature catalogs served only {} records -- the walk found nothing, so \
         agreement below would be agreement about nothing",
        served.len()
    );

    let mut missing: Vec<String> = Vec::new();
    for entry in &served {
        if converted_prose::description_for(&entry.book, "class_feature", &entry.key).is_none() {
            missing.push(format!("{}:{}", entry.book, entry.key));
        }
    }

    println!(
        "class_feature catalog records served={} without converted prose={}",
        served.len(),
        missing.len()
    );
    missing.sort();
    assert!(
        missing.is_empty(),
        "{} of {} records the class_feature catalogs serve have no converted prose, e.g. {:?}",
        missing.len(),
        served.len(),
        missing.iter().take(12).collect::<Vec<_>>()
    );
}

/// No record either catalog serves may print an ingest-side stub marker on a character sheet.
///
/// `AGENTS.md` rule 6 and `docs/governance/no-stub-mvp-doctrine.md`: a shipping code path must
/// not present "would have done" text as content. `[NOT IMPLEMENTED]` and `[NOT ENFORCED]` are
/// the transcription's own markers for a row whose mechanics were never modelled; printing one
/// on a paper sheet tells a player the book says something it does not.
///
/// `class_feature_pool_catalog::carries_unimplemented_marker` refused exactly the **lowercase**
/// spellings until cycle 17, and the corpus states both cases. Measured at the tree that
/// introduced this test: 85 `class_feature` records carry a marker, of which 68 carry one the
/// lowercase-only guard could not see (`grep -ric '\[not implemented\]\|\[not enforced\]'` over
/// `data/corpus/*/class_feature/`).
#[test]
fn no_record_the_class_feature_catalogs_serve_prints_a_stub_marker() {
    let root = repo_root();
    let mut served = load_pool_catalog(&root);
    served.extend(load_standalone_class_feature_catalog(&root));

    assert!(
        served.len() > 500,
        "the class_feature catalogs served only {} records -- agreement about nothing",
        served.len()
    );

    let mut marked: Vec<String> = Vec::new();
    for entry in &served {
        let lower = entry.description.to_ascii_lowercase();
        if lower.contains("[not implemented]") || lower.contains("[not enforced]") {
            marked.push(format!("{}:{}", entry.book, entry.key));
        }
    }

    println!(
        "class_feature catalog records served={} printing a stub marker={}",
        served.len(),
        marked.len()
    );
    marked.sort();
    assert!(
        marked.is_empty(),
        "{} of {} records the class_feature catalogs serve print a stub marker on the sheet, \
         e.g. {:?}",
        marked.len(),
        served.len(),
        marked.iter().take(12).collect::<Vec<_>>()
    );
}
