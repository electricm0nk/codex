//! GE03-E1-F1 acceptance tests for the PCC entry-file parse shape.

use codex::pcgen_import::pcc::{PccDiagnosticKind, parse_pcc_entry};

const FIXTURE: &str = include_str!("fixtures/pcc/core_rulebook_minimal.pcc");

#[test]
fn parses_includes_with_line_numbers_and_raw_directives() {
    let source_path = "inline_test.pcc";
    let input = concat!(
        "# comment line one\n",
        "PCC:@/homebrew/conversion_support/conversion_support.pcc\n",
        "\n",
        "   PCC:@\\pathfinder\\paizo\\roleplaying_game\\core_essentials\\races\\human\\_race.pcc\n",
        "CLASS:cr_classes.lst\n",
        "PCC:\n",
    );

    let result = parse_pcc_entry(source_path, input);

    assert_eq!(result.source_path, source_path);

    // Comments, blank lines, and LST declarations are ignored; two real
    // include edges remain.
    assert_eq!(
        result.includes.len(),
        2,
        "expected exactly two include edges, got {:?}",
        result.includes
    );

    let forward = &result.includes[0];
    assert_eq!(forward.source_path, source_path);
    assert_eq!(
        forward.line_number, 2,
        "one-based line number for the first include"
    );
    assert_eq!(
        forward.raw_directive,
        "PCC:@/homebrew/conversion_support/conversion_support.pcc"
    );
    assert_eq!(
        forward.target,
        "@/homebrew/conversion_support/conversion_support.pcc"
    );
    assert!(
        forward.target.contains('/'),
        "forward-slash include style preserved"
    );

    let backslash = &result.includes[1];
    assert_eq!(
        backslash.line_number, 4,
        "one-based line number counted across the blank line"
    );
    // The raw directive preserves the original leading whitespace verbatim.
    assert_eq!(
        backslash.raw_directive,
        "   PCC:@\\pathfinder\\paizo\\roleplaying_game\\core_essentials\\races\\human\\_race.pcc"
    );
    assert_eq!(
        backslash.target,
        "@\\pathfinder\\paizo\\roleplaying_game\\core_essentials\\races\\human\\_race.pcc"
    );
    assert!(
        backslash.target.contains('\\'),
        "backslash include style preserved"
    );

    // A malformed `PCC:` directive with no target becomes a diagnostic rather
    // than a silently dropped line.
    assert_eq!(
        result.diagnostics.len(),
        1,
        "expected exactly one diagnostic, got {:?}",
        result.diagnostics
    );
    let diag = &result.diagnostics[0];
    assert_eq!(diag.source_path, source_path);
    assert_eq!(diag.line_number, 6);
    assert_eq!(diag.raw_line, "PCC:");
    assert_eq!(diag.kind, PccDiagnosticKind::MalformedInclude);
}

#[test]
fn parses_fixture_include_graph() {
    let result = parse_pcc_entry("core_rulebook_minimal.pcc", FIXTURE);

    // Nine real include edges: two core includes plus seven race includes.
    assert_eq!(
        result.includes.len(),
        9,
        "fixture include edge count, got {:?}",
        result.includes
    );

    // Both slash styles appear among the fixture include targets.
    assert!(
        result.includes.iter().any(|e| e.target.contains('/')),
        "a forward-slash include target is present"
    );
    assert!(
        result.includes.iter().any(|e| e.target.contains('\\')),
        "a backslash include target is present"
    );

    // Every edge carries the source identity and a one-based line number.
    assert!(
        result
            .includes
            .iter()
            .all(|e| e.source_path == "core_rulebook_minimal.pcc")
    );
    assert!(result.includes.iter().all(|e| e.line_number >= 1));

    // The trailing malformed `PCC:` line surfaces as a diagnostic.
    assert_eq!(
        result.diagnostics.len(),
        1,
        "fixture malformed-include diagnostic count, got {:?}",
        result.diagnostics
    );
    assert_eq!(
        result.diagnostics[0].kind,
        PccDiagnosticKind::MalformedInclude
    );
}
