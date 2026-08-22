//! Generation entry point for `class_feature` GRANT-fact data (SD-31 wave
//! 22, rebuild of wave 21's rejected `SD31-W21-CF-GRANT-001` --
//! `OPEN-ISSUES.md` row 334). Run via
//! `cargo run --locked --bin gen_class_feature_grants` with
//! `PCGEN_CORPUS_ROOT` pointing at a local PCGen `data/` checkout
//! (defaults to `$HOME/workspace/repos/pcgen/data`).
//!
//! Writes `data/class_feature_grants/<book>/<class-slug>.json` (a sibling
//! tree to `data/corpus/`, deliberately not inside it) for every book
//! `class_feature::BOOK_PRIMARY_FILES` names. See
//! `codex::rules_core::cache_gen::class_feature_grants`'s module doc
//! comment for the grant-token shapes, the resolution rules, and the
//! correctness proof.
//!
//! Prints both halves of the honest ceiling this wave's brief asked for:
//! resolved facts (split further into corpus-record-exists vs not) AND
//! refused grant tokens by reason -- neither is a headline on its own.

use std::path::PathBuf;

use codex::rules_core::cache_gen::class_feature_grants;

fn main() {
    let corpus_root = match std::env::var("PCGEN_CORPUS_ROOT") {
        Ok(configured) => PathBuf::from(configured),
        Err(_) => {
            let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
            PathBuf::from(home).join("workspace/repos/pcgen/data")
        }
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let grants_root = PathBuf::from(manifest_dir).join("data/class_feature_grants");
    let repo_corpus_root = PathBuf::from(manifest_dir).join("data/corpus");

    match class_feature_grants::generate_all(&corpus_root, &repo_corpus_root, &grants_root) {
        Ok(report) => {
            println!(
                "class_feature_grants generated: {} grant facts across {} books ({} with a real corpus_feature record, {} without); {} skipped (PI blacklist)",
                report.written,
                report.books_written.len(),
                report.corpus_record_exists,
                report.corpus_record_missing,
                report.pi_skipped,
            );
            let unresolved_total: usize = report.unresolved.values().sum();
            println!("unresolved grant tokens (refused, not defaulted): {unresolved_total}");
            let mut reasons: Vec<(&String, &usize)> = report.unresolved.iter().collect();
            reasons.sort_by(|a, b| b.1.cmp(a.1));
            for (reason, count) in reasons {
                println!("  {count:6}  {reason}");
            }
        }
        Err(e) => {
            eprintln!("class_feature_grants generation failed: {e:?}");
            std::process::exit(1);
        }
    }
}
