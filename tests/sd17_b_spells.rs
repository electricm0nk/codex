//! SD-17 Slice B-4 acceptance tests for LST spell-row parsing.
//!
//! These tests pin the parse-shape contract for the `SPELL:` corpus surface.
//! They exercise a hand-built TSV fixture, malformed-row diagnostics, header-row
//! detection, full-corpus streaming on the canonical `cr_spells.lst` (the
//! biggest single PF1 spell file), and an O(n) performance assertion.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use codex::pcgen_import::lst_parser::spell::{
    LstParseDiagnosticKind, LstSpellFile, LstSpellRecord, parse_lst_spell_file, parse_lst_spell_row,
};

// A tight TSV row (the ce_spells.lst shape): exactly 16 columns, no
// alignment padding. Aligned TSV rows (apg, cr_spells) carry the same
// fields at scattered column indices with empty padding; the parser
// detects fields by tag, so both shapes share the same parse path.
const HAND_BUILT_TSV: &str = "Acid Splash\tTYPE:Arcane\tCLASSES:Sorcerer=0|Wizard=0\tSCHOOL:Conjuration\tDESCRIPTOR:Acid\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\tITEM:Scroll\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.241\tSOURCELINK:http://example/acid-splash.html\tDESC:1d3 acid damage.\n";

const HAND_BUILT_TSV_NO_TABS_AT_ALL: &str = "Acid Splash; TYPE:Arcane; SCHOOL:Conjuration";

const HAND_BUILT_TSV_EMPTY_NAME: &str = "\tTYPE:Arcane\tCLASSES:Wizard=0\tSCHOOL:Conjuration\t\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\t\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.1\tSOURCELINK:http://example/x.html\tDESC:no name\n";

// Continuation row: column 0 carries data, but starts with a known tag
// after trimming. Real PCGen continuation rows look like
// `Resounding Blow.MOD\t\tCLASSES:...`; the column-0 cell is the spell
// name with a `.MOD` suffix that we treat as the continuation marker.
// For this test fixture we use a synthetic name whose first non-whitespace
// token begins with a tag prefix; the parser's `starts_with_known_tag`
// check fires on the trimmed value, not on the raw cell.
const HAND_BUILT_CONTINUATION: &str = "CLASSES:Paladin=4|Inquisitor=5\tCLASSES:Paladin=4\n";

const HAND_BUILT_HEADER: &str = "# CVS $Revision: 1 $\nSOURCELONG:Core\tSOURCESHORT:CR\n###Block: Acid\n# Spell Name\tType\tClasses of caster\tSchool\tDescriptor\tComponents\tCasting Time\tRange\tItem\tTarget Area or Effect\tDuration\tSave Info\tSpell Resistance\tSource Page\tSource Pub Link\tDescription\nAcid Splash\tTYPE:Arcane\tCLASSES:Wizard=0\tSCHOOL:Conjuration\tDESCRIPTOR:Acid\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\tITEM:Scroll\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.241\tSOURCELINK:http://example/acid-splash.html\tDESC:1d3 acid damage.\n";

// Description carrying the canonical PCGen pipe-delimited meta-annotation.
const HAND_BUILT_TSV_WITH_PRERULE: &str = "Acid Splash\tTYPE:Arcane\tCLASSES:Wizard=0\tSCHOOL:Conjuration\tDESCRIPTOR:Acid\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\tITEM:Scroll\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.241\tSOURCELINK:http://example/acid-splash.html\tDESC:1d3 acid damage.|!PRERULE:1,DisplayFullSpell\n";

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn write_temp_lst(contents: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "codex_sd17_b4_spells_{}_{}.lst",
        std::process::id(),
        nonce
    ));
    fs::write(&path, contents).expect("write temp lst fixture");
    path
}

#[test]
fn parse_hand_built_tsv_extracts_all_tagged_fields() {
    let parsed = parse_lst_spell_row("test_input.lst", 7, HAND_BUILT_TSV);
    assert!(
        parsed.diagnostics.is_empty(),
        "hand-built TSV row must produce no diagnostics, got {:?}",
        parsed.diagnostics
    );
    let row = parsed
        .into_record()
        .expect("hand-built TSV row must be well-formed");

    assert_eq!(row.line_number, 7);
    assert_eq!(row.source_path, "test_input.lst");
    assert_eq!(row.name, "Acid Splash");
    assert_eq!(row.spell_type.as_deref(), Some("Arcane"));
    assert_eq!(row.classes.as_deref(), Some("Sorcerer=0|Wizard=0"));
    assert_eq!(row.school.as_deref(), Some("Conjuration"));
    assert_eq!(row.descriptor.as_deref(), Some("Acid"));
    assert_eq!(row.components.as_deref(), Some("V, S"));
    assert_eq!(row.casting_time.as_deref(), Some("1 standard action"));
    assert_eq!(row.range.as_deref(), Some("Close"));
    assert_eq!(row.item.as_deref(), Some("Scroll"));
    assert_eq!(row.target_area.as_deref(), Some("One target"));
    assert_eq!(row.duration.as_deref(), Some("Instantaneous"));
    assert_eq!(row.save_info.as_deref(), Some("None"));
    assert_eq!(row.spell_resistance.as_deref(), Some("No"));
    assert_eq!(row.source_page.as_deref(), Some("p.241"));
    assert_eq!(
        row.source_link.as_deref(),
        Some("http://example/acid-splash.html")
    );
    assert_eq!(row.description.as_deref(), Some("1d3 acid damage."));
    assert_eq!(row.output_name, None);
    assert_eq!(row.sub_school, None);
}

#[test]
fn parse_hand_built_tsv_returns_malformed_diagnostic_when_no_tab_separator() {
    let parsed = parse_lst_spell_row("test_input.lst", 1, HAND_BUILT_TSV_NO_TABS_AT_ALL);

    assert!(
        parsed.record.is_none(),
        "no-tab row must not produce a record"
    );

    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics
            .iter()
            .any(|diag| matches!(diag.kind, LstParseDiagnosticKind::MalformedRow)),
        "expected at least one MalformedRow diagnostic, got {:?}",
        diagnostics
    );
}

#[test]
fn parse_hand_built_tsv_flags_empty_name_as_missing_spell_name() {
    let parsed = parse_lst_spell_row("test_input.lst", 4, HAND_BUILT_TSV_EMPTY_NAME);

    assert!(parsed.record.is_none());
    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics
            .iter()
            .any(|diag| matches!(diag.kind, LstParseDiagnosticKind::MissingSpellName)),
        "expected MissingSpellName diagnostic, got {:?}",
        diagnostics
    );
}

#[test]
fn parse_hand_built_tsv_recognizes_continuation_rows() {
    let parsed = parse_lst_spell_row("test_input.lst", 11, HAND_BUILT_CONTINUATION);

    assert!(
        parsed.record.is_none(),
        "continuation rows must not produce a new spell record"
    );
    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics
            .iter()
            .any(|diag| matches!(diag.kind, LstParseDiagnosticKind::ContinuationRow)),
        "expected ContinuationRow diagnostic, got {:?}",
        diagnostics
    );
    assert_eq!(
        diagnostics
            .iter()
            .find(|d| matches!(d.kind, LstParseDiagnosticKind::ContinuationRow))
            .and_then(|d| d.line_number),
        Some(11),
        "continuation-row diagnostic must carry the source line number"
    );
}

#[test]
fn parse_hand_built_tsv_strips_prerule_annotation_from_description() {
    let parsed = parse_lst_spell_row("test_input.lst", 1, HAND_BUILT_TSV_WITH_PRERULE);
    assert!(parsed.diagnostics.is_empty());
    let row = parsed.into_record().expect("row must parse");

    assert_eq!(
        row.description.as_deref(),
        Some("1d3 acid damage."),
        "visible description must drop the pipe-delimited PRERULE suffix"
    );
    assert_eq!(
        row.description_raw.as_deref(),
        Some("1d3 acid damage.|!PRERULE:1,DisplayFullSpell"),
        "raw description must preserve the full payload including the annotation"
    );
}

#[test]
fn parse_lst_spell_file_skips_header_comments_and_source_metadata() {
    let path = write_temp_lst(HAND_BUILT_HEADER);
    let result = parse_lst_spell_file(&path).expect("file must parse without IO error");

    assert_eq!(
        result.records.len(),
        1,
        "header lines, source-metadata lines, and block markers must be skipped"
    );
    assert_eq!(result.records[0].name, "Acid Splash");
    assert_eq!(result.records[0].line_number, 5);
    assert!(
        result.diagnostics.is_empty(),
        "no diagnostics expected for clean fixture, got {:?}",
        result.diagnostics
    );
}

#[test]
fn parse_lst_spell_file_carries_source_line_numbers_for_every_record() {
    let mut fixture = String::new();
    fixture.push_str("# header\n");
    fixture.push_str("SOURCELONG:Test\tSOURCESHORT:T\n");
    fixture.push('\n');
    for i in 1..=50 {
        fixture.push_str(&format!(
            "Test Spell {i}\tTYPE:Arcane\tCLASSES:Wizard=0\tSCHOOL:Evocation\t\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\t\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.{i}\tSOURCELINK:http://example/{i}.html\tDESC:row {i}.\n"
        ));
    }
    let path = write_temp_lst(&fixture);
    let result = parse_lst_spell_file(&path).expect("file must parse without IO error");

    assert_eq!(result.records.len(), 50);
    for (i, record) in result.records.iter().enumerate() {
        let expected_line = 4 + i; // header(1) + metadata(2) + blank(3) + index_offset
        assert_eq!(
            record.line_number, expected_line,
            "row {i} must carry its source line number"
        );
    }
}

#[test]
fn parse_lst_spell_file_emits_malformed_and_continuation_diagnostics_with_line_numbers() {
    let mut fixture = String::new();
    fixture.push_str("Acid Splash\tTYPE:Arcane\tSCHOOL:Conjuration\t\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\t\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.1\tSOURCELINK:http://example/1.html\tDESC:1\n");
    fixture.push_str("this line has no tab separators at all\n");
    fixture.push_str("Burning Hands\tTYPE:Arcane\tSCHOOL:Evocation\t\tCOMPS:V, S\tCASTTIME:1 standard action\tRANGE:Close\t\tTARGETAREA:One target\tDURATION:Instantaneous\tSAVEINFO:None\tSPELLRES:No\tSOURCEPAGE:p.2\tSOURCELINK:http://example/2.html\tDESC:2\n");
    fixture.push_str("CLASSES:Paladin=4|Inquisitor=5\tCLASSES:Paladin=4\n");
    let path = write_temp_lst(&fixture);
    let result = parse_lst_spell_file(&path).expect("file must parse without IO error");

    assert_eq!(result.records.len(), 2, "two well-formed rows expected");
    assert_eq!(result.records[0].name, "Acid Splash");
    assert_eq!(result.records[0].line_number, 1);
    assert_eq!(result.records[1].name, "Burning Hands");
    assert_eq!(result.records[1].line_number, 3);

    let malformed: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|diag| matches!(diag.kind, LstParseDiagnosticKind::MalformedRow))
        .collect();
    assert_eq!(malformed.len(), 1, "exactly one MalformedRow expected");
    assert_eq!(
        malformed[0].line_number,
        Some(2),
        "malformed-row diagnostic must carry the source line number"
    );

    let continuations: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|diag| matches!(diag.kind, LstParseDiagnosticKind::ContinuationRow))
        .collect();
    assert_eq!(
        continuations.len(),
        1,
        "exactly one ContinuationRow expected"
    );
    assert_eq!(
        continuations[0].line_number,
        Some(4),
        "continuation-row diagnostic must carry the source line number"
    );
}

#[test]
fn parse_lst_spell_file_handles_canonical_pf1_corpus_when_available() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping canonical corpus test (set \
             CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data to enable)"
        );
        return;
    };

    let cr_spells = root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst");
    if !cr_spells.is_file() {
        eprintln!(
            "canonical cr_spells.lst not present at {}; skipping",
            cr_spells.display()
        );
        return;
    }

    let result = parse_lst_spell_file(&cr_spells).expect("cr_spells.lst must parse");

    // cr_spells.lst is the largest single PF1 spell file (~920 KB) and
    // contains more than 700 well-formed spell declarations on the aligned
    // TSV shape. Pin a minimum record count to lock the parser to the
    // full corpus surface; a curated subset would not hit this number.
    assert!(
        result.records.len() >= 700,
        "expected >= 700 spell records on cr_spells.lst, got {}",
        result.records.len()
    );

    // Every record must carry a non-zero line number from the source file.
    for record in &result.records {
        assert!(record.line_number >= 1, "line number must be 1-based");
    }
}

#[test]
fn parse_lst_spell_file_runs_in_linear_time_on_canonical_corpus() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping perf test (set \
             CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data to enable)"
        );
        return;
    };

    let cr_spells = root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst");
    if !cr_spells.is_file() {
        eprintln!(
            "canonical cr_spells.lst not present at {}; skipping",
            cr_spells.display()
        );
        return;
    }

    let bytes = fs::metadata(&cr_spells)
        .expect("read cr_spells.lst metadata")
        .len() as usize;
    assert!(bytes > 0, "cr_spells.lst must be non-empty");

    let _first = parse_lst_spell_file(&cr_spells).expect("first parse must succeed");
    let start = std::time::Instant::now();
    let result = parse_lst_spell_file(&cr_spells).expect("second parse must succeed");
    let elapsed = start.elapsed();

    let bytes_per_sec = bytes as f64 / elapsed.as_secs_f64();
    eprintln!(
        "parsed {} bytes ({} records) in {:?} ({:.0} bytes/sec)",
        bytes,
        result.records.len(),
        elapsed,
        bytes_per_sec
    );

    assert!(
        bytes_per_sec > 5_000_000.0,
        "parser throughput {:.0} bytes/sec below 5 MB/sec O(n) floor",
        bytes_per_sec
    );
}

#[test]
fn parse_lst_spell_file_round_trips_known_pf1_spell_name() {
    let Some(root) = corpus_root() else {
        eprintln!("CORPUS_ROOT not set; skipping named-spell pin test");
        return;
    };
    let apg = root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst");
    if !apg.is_file() {
        eprintln!("apg_spells.lst not present at {}; skipping", apg.display());
        return;
    }

    let result = parse_lst_spell_file(&apg).expect("apg_spells.lst must parse");
    let found = result
        .records
        .iter()
        .find(|record| record.name == "Corruption Resistance")
        .expect("Corruption Resistance must appear in apg_spells.lst");

    assert_eq!(found.school.as_deref(), Some("Abjuration"));
    assert_eq!(found.classes.as_deref(), Some("Antipaladin,Paladin=2"));
    assert_eq!(found.components.as_deref(), Some("V, S, DF"));
    assert_eq!(found.casting_time.as_deref(), Some("1 standard action"));
    assert_eq!(found.range.as_deref(), Some("Touch"));
    assert!(found.line_number >= 1);
}

#[test]
fn lst_spell_file_struct_carries_source_path() {
    let path = write_temp_lst(HAND_BUILT_TSV);
    let result: LstSpellFile = parse_lst_spell_file(&path).expect("parse must succeed");
    assert_eq!(result.source_path, path);
}

#[test]
fn lst_spell_record_equality_is_structural() {
    let record_a: LstSpellRecord = parse_lst_spell_row("x", 1, HAND_BUILT_TSV)
        .into_record()
        .expect("row must parse");
    let record_b: LstSpellRecord = parse_lst_spell_row("y", 99, HAND_BUILT_TSV)
        .into_record()
        .expect("row must parse");
    assert_eq!(record_a.payload, record_b.payload);
}
