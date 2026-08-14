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

use codex::rules_core::pi_table_sweep::{parse_baseline, reconcile, screen_generated_table, sweep_dir, sweep_text};
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

/// `SD30-E3-F1`'s invocation contract, proven against real content, not a
/// fixture: the lane-facing pre-commit entry point
/// (`pi_table_sweep::screen_generated_table`, the exact function
/// `gen_feat_gap_tables`/`gen_equipment_gap_tables` already call before
/// writing) must refuse when the text it is about to write is
/// `class_feature`/archetype content carrying a declared blacklist term.
/// `acg/archetype_tables.rs` line's `Sarenrae` hit
/// (`docs/governance/pi-sweep-baseline.tsv`, disposition `real-leak`,
/// "Ecclesitheurge ~ Domain Mastery description") is already-shipped, real
/// `class_feature` table content — this test reads that exact line back out
/// of the live file and re-plays it through the pre-commit screen a future
/// `class_feature` generator (SD-31's Epic 3 chassis-sweep, per
/// `SD-30-.../decisions.md` this feature seed's invocation-contract entry)
/// would call, so the proof exercises the real pipeline shape, not a
/// hand-written string.
#[test]
fn screen_generated_table_refuses_real_class_feature_content_carrying_a_known_pi_term() {
    let root = repo_root();
    let path = root.join("src/rules_core/rules_tables/acg/archetype_tables.rs");
    let text = std::fs::read_to_string(&path).expect("acg archetype_tables.rs exists");
    let sarenrae_line = text
        .lines()
        .find(|l| l.contains("Sarenrae"))
        .expect("the known Sarenrae PI line (baselined real-leak) is still present in acg::archetype_tables");

    // Simulate a future class_feature lane's generator emitting this exact,
    // real row as newly-generated table text — the shape SD-31's Epic 3
    // chassis-sweep will produce for `adventurers_guide` et al.
    let generated = format!("{sarenrae_line}\n");
    let hits = screen_generated_table(
        "src/rules_core/rules_tables/adventurers_guide/class_feature_gap_tables.rs",
        &generated,
    );

    assert!(
        !hits.is_empty(),
        "the pre-commit screen must refuse real class_feature-shaped content carrying a declared blacklist term; got no hits for: {generated}"
    );
    assert!(
        hits.iter().any(|h| h.term == "Sarenrae"),
        "expected a Sarenrae hit, got {hits:?}"
    );
}

/// The companion true-negative: the same pre-commit entry point, called on
/// real, adjacent `class_feature`/archetype content that carries no
/// blacklist term (`acg/archetype_tables.rs`'s "Weapon and Armor
/// Proficiency" grant, three lines above the `Sarenrae` hit above), must NOT
/// refuse. A gate that flags everything proves as little as one that flags
/// nothing.
#[test]
fn screen_generated_table_is_clean_on_real_class_feature_content_without_a_pi_term() {
    let root = repo_root();
    let path = root.join("src/rules_core/rules_tables/acg/archetype_tables.rs");
    let text = std::fs::read_to_string(&path).expect("acg archetype_tables.rs exists");
    let clean_line = text
        .lines()
        .find(|l| l.contains("Weapon and Armor Proficiency"))
        .expect("the known clean grant line is still present in acg::archetype_tables");
    assert!(
        !clean_line.contains("Sarenrae"),
        "test fixture assumption broken: the chosen clean line now carries the PI term"
    );

    let generated = format!("{clean_line}\n");
    let hits = screen_generated_table(
        "src/rules_core/rules_tables/adventurers_guide/class_feature_gap_tables.rs",
        &generated,
    );

    assert!(hits.is_empty(), "unexpected hit(s) on real, non-PI class_feature content: {hits:?}");
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
