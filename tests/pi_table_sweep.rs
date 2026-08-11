//! Provenance gate: the Product-Identity blacklist sweep over Pipeline B
//! (`src/rules_core/rules_tables/**/*.rs`).
//!
//! `docs/governance/license-matrix.md` recorded that **zero** files under
//! `rules_tables/` had ever been screened against `pi_screening`'s blacklist,
//! and that a five-minute manual sweep already found real, unredacted hits in
//! committed source. This suite is the executable form of that sweep: it runs
//! over the live tree on every `cargo test`, so a kind lane cannot land a new
//! Product-Identity leak in a generated table.
//!
//! Pre-existing hits owned by other bundles are carried in
//! `docs/governance/pi-sweep-baseline.tsv` with an explicit disposition. The
//! gate fails on anything the baseline does not already account for, and on a
//! baseline row that no longer matches the tree (so the file cannot rot into a
//! blanket suppression).

use codex::rules_core::pi_table_sweep::{parse_baseline, reconcile, sweep_dir, sweep_text};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn sweep_text_reports_a_blacklist_term_with_its_line_and_context() {
    let hits = sweep_text(
        "generated/spell_list.rs",
        "let a = 1;\nlet d = \"granted by Iomedae herself\";\nlet b = 2;\n",
    );
    assert_eq!(hits.len(), 1, "one hit expected, got {hits:?}");
    assert_eq!(hits[0].term, "Iomedae");
    assert_eq!(hits[0].line, 2);
    assert_eq!(hits[0].file, "generated/spell_list.rs");
    assert!(hits[0].context.contains("Iomedae"));
}

#[test]
fn sweep_text_is_clean_on_ordinary_mechanical_prose() {
    let hits = sweep_text("generated/x.rs", "Deals 1d6 points of fire damage.\n");
    assert!(hits.is_empty(), "unexpected hits: {hits:?}");
}

#[test]
fn baseline_parses_disposition_rows_and_ignores_comments() {
    let parsed = parse_baseline(
        "# a comment\n\nsrc/x.rs\tNex\t2\tfalse-positive\tsubstring of \"Next\"\n",
    )
    .expect("baseline should parse");
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].file, "src/x.rs");
    assert_eq!(parsed[0].term, "Nex");
    assert_eq!(parsed[0].count, 2);
    assert_eq!(parsed[0].disposition, "false-positive");
}

#[test]
fn reconcile_flags_a_hit_the_baseline_does_not_account_for() {
    let hits = sweep_text("src/rules_core/rules_tables/x/y.rs", "worships Asmodeus\n");
    let baseline = parse_baseline("").expect("empty baseline parses");
    let verdict = reconcile(&hits, &baseline);
    assert_eq!(verdict.unbaselined.len(), 1);
    assert!(verdict.stale.is_empty());
}

#[test]
fn reconcile_flags_a_baseline_row_the_tree_no_longer_carries() {
    let baseline = parse_baseline("src/gone.rs\tIomedae\t1\treal-leak\towned elsewhere\n")
        .expect("baseline parses");
    let verdict = reconcile(&[], &baseline);
    assert!(verdict.unbaselined.is_empty());
    assert_eq!(verdict.stale.len(), 1, "a stale baseline row must be reported");
}

/// The gate itself, over the live tree.
#[test]
fn rules_tables_carry_no_unbaselined_product_identity_hits() {
    let root = repo_root();
    let hits = sweep_dir(&root.join("src/rules_core/rules_tables")).expect("sweep runs");
    let baseline_text =
        std::fs::read_to_string(root.join("docs/governance/pi-sweep-baseline.tsv")).expect("baseline file exists");
    let baseline = parse_baseline(&baseline_text).expect("baseline parses");
    let verdict = reconcile(&hits, &baseline);

    assert!(
        verdict.unbaselined.is_empty(),
        "unbaselined Product-Identity hits in rules_tables — a hit is a hard stop, not a thing to route around:\n{}",
        verdict
            .unbaselined
            .iter()
            .map(|h| format!("  {}:{} [{}] {}", h.file, h.line, h.term, h.context))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        verdict.stale.is_empty(),
        "stale rows in docs/governance/pi-sweep-baseline.tsv (tree no longer matches): {:?}",
        verdict.stale
    );
}
