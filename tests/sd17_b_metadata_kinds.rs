//! SD-17 Slice B-6 acceptance tests for LST metadata-kind parsing.
//!
//! Covers the six metadata object kinds the operator directive names:
//! `DEITY:`, `DOMAIN:`, `KITS:`, `LANGUAGE:`, `TEMPLATE:`, `COMPANIONMOD:`.
//! Each kind is parsed from a representative fixture deterministically with
//! source line numbers preserved on every record.
//!
//! Parser semantics (SD-17 Slice B-6):
//!   - A metadata-kind directive at line start is its own record.
//!   - Records are not grouped by blank lines for these six kinds; every
//!     recognized directive produces a record-start.
//!   - The parser is case-sensitive on the directive prefix.
//!   - A directive with no value emits a `MalformedDirective` diagnostic.

use codex::pcgen_import::lst_parser::{
    LstDiagnosticKind, MetadataKind, parse_lst_metadata, parse_lst_metadata_text,
};

const SIX_METADATA_KIND_NAMES: &[&str] = &[
    "DEITY",
    "DOMAIN",
    "KITS",
    "LANGUAGE",
    "TEMPLATE",
    "COMPANIONMOD",
];

/// The PCGen corpus root this test reads. `PCGEN_CORPUS_ROOT` wins when set;
/// otherwise `$HOME/workspace/repos/pcgen/data` — HOME-relative because the
/// operator keeps `workspace/` in the home directory and syncs it between
/// machines, so the default is correct on any box. Rust does not expand `~`,
/// so `$HOME` is read via `std::env::var`. `None` when `HOME` itself is
/// unset, which leaves the caller's existing skip path intact.
fn corpus_root() -> Option<std::path::PathBuf> {
    if let Some(configured) = std::env::var_os("PCGEN_CORPUS_ROOT") {
        return Some(std::path::PathBuf::from(configured));
    }
    let home = std::env::var_os("HOME").filter(|home| !home.is_empty())?;
    Some(std::path::PathBuf::from(home).join("workspace/repos/pcgen/data"))
}

#[test]
fn recognizes_every_metadata_kind_declared_in_the_slice() {
    for kind_name in SIX_METADATA_KIND_NAMES {
        let kind = MetadataKind::from_token(kind_name)
            .unwrap_or_else(|| panic!("metadata kind {kind_name} must be recognized"));
        assert_eq!(kind.token(), *kind_name);
    }
}

#[test]
fn rejects_unknown_metadata_kinds() {
    assert!(MetadataKind::from_token("BOGUS").is_none());
    // lower-cased form is not the same as the canonical token
    assert!(MetadataKind::from_token("deity").is_none());
}

#[test]
fn parses_deity_record_start_with_line_number_preserved() {
    let text = "\
DEITY:Lamashtu\tDOMAIN:Evil|Strength
TEMPLATE:Lycanthrope
";
    let document = parse_lst_metadata_text("sample_deities.lst", text);

    // Two records: one DEITY + one TEMPLATE, each is its own record-start.
    assert_eq!(document.records.len(), 2);
    assert!(document.records.iter().all(|r| r.is_record_start));

    let deity = &document.records[0];
    assert_eq!(deity.kind, MetadataKind::Deity);
    assert_eq!(deity.name, "Lamashtu\tDOMAIN:Evil");
    assert_eq!(deity.line_number, 1);
    assert_eq!(deity.raw_line, "DEITY:Lamashtu\tDOMAIN:Evil|Strength");
    assert!(deity.diagnostics.is_empty());

    let template = &document.records[1];
    assert_eq!(template.kind, MetadataKind::Template);
    assert_eq!(template.name, "Lycanthrope");
    assert_eq!(template.line_number, 2);
}

#[test]
fn every_metadata_directive_at_line_start_is_a_record_start() {
    let text = "\
DEITY:Lamashtu\tDOMAIN:Evil|Strength
TEMPLATE:Lycanthrope

DEITY:Urgathoa\tDOMAIN:Death|War
TEMPLATE:Priest of Urgathoa's Magic Buffs
";
    let document = parse_lst_metadata_text("multi_deities.lst", text);

    assert_eq!(document.records.len(), 4);
    assert!(document.records.iter().all(|r| r.is_record_start));
    assert_eq!(document.records[0].kind, MetadataKind::Deity);
    assert_eq!(document.records[0].name, "Lamashtu\tDOMAIN:Evil");
    assert_eq!(document.records[0].line_number, 1);
    assert_eq!(document.records[1].kind, MetadataKind::Template);
    assert_eq!(document.records[1].line_number, 2);
    assert_eq!(document.records[2].kind, MetadataKind::Deity);
    assert_eq!(document.records[2].name, "Urgathoa\tDOMAIN:Death");
    assert_eq!(document.records[2].line_number, 4);
    assert_eq!(document.records[3].kind, MetadataKind::Template);
    assert_eq!(document.records[3].line_number, 5);
    assert!(document.records.iter().all(|r| r.diagnostics.is_empty()));
}

#[test]
fn parses_a_representative_fixture_per_kind_with_kind_specific_records() {
    let text = "\
DEITY:Lamashtu\tDOMAIN:Evil|Strength
DOMAIN:Evil
KITS:Starting Gold ~ CRB
LANGUAGE:Abyssal\tTYPE:Spoken.Written.Read.Planar
TEMPLATE:Lycanthrope
COMPANIONMOD:FOLLOWER:AnimalCompanionLVL=1
";

    let document = parse_lst_metadata_text("kinds.lst", text);

    // Every metadata-kind directive present is recognized as its own record.
    let by_kind = document.records_by_kind();
    assert_eq!(by_kind.get(&MetadataKind::Deity).map(Vec::len), Some(1));
    assert_eq!(
        by_kind.get(&MetadataKind::Domain).map(Vec::len),
        Some(1),
        "DOMAIN appears once as a record-start; the DEITY record's tab-separated value is part of the Deity record name, not a separate DOMAIN record"
    );
    assert_eq!(by_kind.get(&MetadataKind::Kits).map(Vec::len), Some(1));
    assert_eq!(by_kind.get(&MetadataKind::Language).map(Vec::len), Some(1));
    assert_eq!(by_kind.get(&MetadataKind::Template).map(Vec::len), Some(1));
    assert_eq!(
        by_kind.get(&MetadataKind::CompanionMod).map(Vec::len),
        Some(1)
    );
}

#[test]
fn emits_malformed_diagnostic_when_metadata_directive_has_no_value() {
    let text = "\
DEITY:
TEMPLATE:Lycanthrope
";
    let document = parse_lst_metadata_text("malformed.lst", text);

    assert_eq!(document.records.len(), 2);
    let malformed = document
        .records
        .iter()
        .find(|r| {
            r.diagnostics
                .iter()
                .any(|d| d.kind == LstDiagnosticKind::MalformedDirective)
        })
        .expect("malformed record must carry a diagnostic");
    assert_eq!(malformed.kind, MetadataKind::Deity);
    assert_eq!(malformed.line_number, 1);
    assert!(malformed.name.is_empty());
}

#[test]
fn parses_a_fixture_from_disk_with_line_numbers_preserved() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lst/metadata_kinds_minimal.lst");
    let document = parse_lst_metadata(&path).expect("disk fixture parses deterministically");

    // The fixture contains exactly one record-start per recognized metadata
    // kind; line numbers must round-trip exactly.
    let starts: Vec<(MetadataKind, String, usize)> = document
        .records
        .iter()
        .map(|r| (r.kind, r.name.clone(), r.line_number))
        .collect();

    assert!(
        starts.iter().any(|(k, n, ln)| *k == MetadataKind::Deity
            && n == "Lamashtu\tDOMAIN:Evil"
            && *ln == 9),
        "DEITY:Lamashtu (with DOMAIN sub-tag) must preserve line 9; got {:?}",
        starts
    );
    assert!(
        starts.iter().any(|(k, n, ln)| {
            *k == MetadataKind::Kits && n == "Starting Gold ~ CRB" && *ln == 10
        }),
        "KITS:Starting Gold ~ CRB must preserve line 10; got {:?}",
        starts
    );
    assert!(
        starts
            .iter()
            .any(|(k, n, ln)| *k == MetadataKind::Template && n == "Lycanthrope" && *ln == 11),
        "TEMPLATE:Lycanthrope must preserve line 11; got {:?}",
        starts
    );
    assert!(
        starts.iter().any(|(k, n, ln)| {
            *k == MetadataKind::Language
                && n == "Abyssal\tTYPE:Spoken.Written.Read.Planar"
                && *ln == 12
        }),
        "LANGUAGE:Abyssal must preserve line 12; got {:?}",
        starts
    );
}

#[test]
fn parse_runs_in_linear_time_on_a_synthetic_large_file() {
    let n = 5_000usize;
    let mut text = String::with_capacity(n * 30);
    for i in 0..n {
        text.push_str(&format!("TEMPLATE:Synthetic Template {i}\n"));
    }

    let start = std::time::Instant::now();
    let document = parse_lst_metadata_text("synthetic.lst", &text);
    let elapsed = start.elapsed();

    let template_records = document
        .records
        .iter()
        .filter(|r| r.kind == MetadataKind::Template)
        .count();
    assert_eq!(template_records, n);

    // Loose bound: 5k records should parse in well under 2 seconds on any
    // reasonable host. The tighter intent is O(n) (linear, not quadratic).
    assert!(
        elapsed.as_secs() < 2,
        "5k template records should parse in well under 2s, took {elapsed:?}"
    );
}

#[test]
fn ignores_blank_lines_and_comment_lines() {
    let text = "\
# leading comment

DEITY:Lamashtu
   # indented comment

TEMPLATE:Lycanthrope
";
    let document = parse_lst_metadata_text("comments.lst", text);

    assert_eq!(document.records.len(), 2);
    assert_eq!(document.records[0].kind, MetadataKind::Deity);
    assert_eq!(document.records[0].line_number, 3);
    assert_eq!(document.records[1].kind, MetadataKind::Template);
    assert_eq!(document.records[1].line_number, 6);
}

#[test]
fn real_corpus_kits_file_parses_with_line_numbers_preserved() {
    // The real core_rulebook LST files contain `TEMPLATE:` lines as
    // record-starts. This is the most common metadata-kind occurrence in the
    // corpus; the parser must round-trip line numbers and record-start flags.
    let Some(corpus_root) = corpus_root() else {
        eprintln!("skipping: no PCGEN_CORPUS_ROOT and no $HOME to derive one for cr_kits.lst");
        return;
    };
    let path = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_kits.lst");
    if !path.is_file() {
        // Skip gracefully if the corpus is unavailable in this sandbox; the
        // synthetic-fixture tests above are still authoritative.
        eprintln!("skipping: real cr_kits.lst not at {}", path.display());
        return;
    }
    let document = parse_lst_metadata(&path).expect("real corpus file parses");
    let templates: Vec<_> = document
        .records
        .iter()
        .filter(|r| r.kind == MetadataKind::Template)
        .collect();
    assert!(
        templates.len() >= 3,
        "expected at least 3 TEMPLATE record-starts in cr_kits.lst, got {}",
        templates.len()
    );
    // Every TEMPLATE record must carry a non-empty name and a positive line
    // number — the parser cannot drop or rewrite these.
    for template in &templates {
        assert!(!template.name.is_empty(), "TEMPLATE name must be non-empty");
        assert!(
            template.line_number >= 1,
            "TEMPLATE line number must be positive"
        );
        assert!(template.diagnostics.is_empty());
    }
}
