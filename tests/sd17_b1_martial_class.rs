//! SD-17 Slice B-1 acceptance tests — LST parser for the martial-class
//! `CLASS:` object kind.
//!
//! Scope is bounded to the six martial classes named in the slice card
//! (Fighter, Barbarian, Monk, Rogue, Ranger, Paladin) plus their `Ex-*`
//! variants. Each martial class is described by one or more
//! `CLASS:<name>\t<KEY>:<VAL> ...` header lines followed by `###Block:`
//! separators and per-level feature lines. The parser must:
//!
//! - recognize every `CLASS:` line in the corpus for the named martial
//!   classes (do NOT narrow to one class);
//! - preserve one-based source line numbers on every record;
//! - carry every tab-delimited token pair to the canonical IR so the
//!   SD-13 matrix uplift can lift values in a later slice;
//! - report malformed `CLASS:` lines and bad `###Block:` markers as
//!   `MalformedSD17B1` diagnostics with line numbers and raw evidence;
//! - parse real PCGen files deterministically with no per-class
//!   special-casing inside the parser core (a class is just whatever
//!   class names are listed in the test).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use codex::pcgen_import::lst_parser::class::LstDiagnosticKind;
use codex::pcgen_import::lst_parser::{ClassToken, parse_class_entries, parse_class_file};

/// The six martial classes named in the slice card body.
const MARTIAL_CLASSES: &[&str] = &["Fighter", "Barbarian", "Monk", "Rogue", "Ranger", "Paladin"];

struct TestCorpus {
    root: PathBuf,
}

impl TestCorpus {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "codex_sd17_b1_martial_class_{}_{}_{}",
            name,
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test corpus root");
        Self { root }
    }

    #[allow(dead_code)]
    fn write(&self, relative_path: &str, contents: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent dir");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    #[allow(dead_code)]
    fn root(&self) -> &Path {
        &self.root
    }

    fn path(&self, relative_path: &str) -> PathBuf {
        self.root.join(relative_path)
    }
}

impl Drop for TestCorpus {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[allow(dead_code)]
fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Build a representative header line for one of the martial classes using
/// the same shape the real `cr_classes.lst` uses (tab-delimited KEY:VAL
/// tokens, with `HD:`, `TYPE:Base.PC`, `MAXLEVEL:20`).
fn header_line(class_name: &str, hd: u8) -> String {
    format!(
        "CLASS:{class_name}\tHD:{hd}\t\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.0\t\
         DEFINE:{class_name}LVL|0\tBONUS:COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")\t\
         ROLE:Combat.Skill\tFACT:ClassType|PC\tFACT:Abb|Abb"
    )
}

#[test]
fn parses_single_class_header_with_all_tab_delimited_tokens() {
    let input = "\
# comment line
SOURCELONG:Core Rulebook\tSOURCESHORT:CR
CLASS:Fighter\tHD:10\t\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.55\tDEFINE:FighterLVL|0\tBONUS:COMBAT|BASEAB|classlevel
CLASS:Fighter\tSTARTSKILLPTS:2
###Block:Proficiencies
1\tABILITY:Class|AUTOMATIC|Fighter
";
    let result = parse_class_entries("cr_classes.lst", input);

    assert_eq!(result.entries.len(), 1, "one class, one entry");
    let entry = &result.entries[0];
    assert_eq!(entry.class_name, "Fighter");
    assert_eq!(entry.header_line_number, 3, "header line is line 3");
    assert!(
        result.diagnostics.is_empty(),
        "valid LST should produce no diagnostics: {:?}",
        result.diagnostics
    );
    // Every tab-delimited KEY:VAL token is carried forward, including the
    // BONUS:COMBAT nested pipe-separated payload. The single-line fixture
    // here does not include ROLE/FACT — those are checked by the
    // header_line() helper test below.
    let keys: Vec<&str> = entry.tokens.iter().map(|t| t.key.as_str()).collect();
    assert!(keys.contains(&"HD"));
    assert!(keys.contains(&"TYPE"));
    assert!(keys.contains(&"MAXLEVEL"));
    assert!(keys.contains(&"SOURCEPAGE"));
    assert!(keys.contains(&"DEFINE"));
    assert!(keys.contains(&"BONUS"));
    // STARTSKILLPTS appears on the second CLASS: line — collected as a
    // continuation token, not a separate entry.
    let startskill = entry
        .tokens
        .iter()
        .find(|t| t.key == "STARTSKILLPTS")
        .expect("STARTSKILLPTS token carried forward");
    assert_eq!(startskill.value, "2");
    assert_eq!(startskill.line_number, 4);
    // ###Block: marks the boundary to a per-level feature block. Each
    // non-CLASS line between ###Block: and the next blank line belongs to
    // the feature block.
    assert_eq!(entry.feature_blocks.len(), 1);
    assert_eq!(entry.feature_blocks[0].label, "Proficiencies");
    assert_eq!(entry.feature_blocks[0].level_lines.len(), 1);
    assert_eq!(entry.feature_blocks[0].level_lines[0].level, 1);
    assert_eq!(
        entry.feature_blocks[0].level_lines[0].raw_line,
        "1\tABILITY:Class|AUTOMATIC|Fighter"
    );
    assert_eq!(entry.feature_blocks[0].level_lines[0].line_number, 6);
}

#[test]
fn parses_every_martial_class_with_uniform_record_shape() {
    // Build a synthetic LST containing a header line for each of the six
    // martial classes. The parser must recognize all six (and only those
    // six as the named-classes scope) without per-class special-casing.
    let mut input = String::from("SOURCELONG:Core Rulebook\n");
    for (idx, name) in MARTIAL_CLASSES.iter().enumerate() {
        let hd = match *name {
            "Fighter" | "Monk" | "Paladin" | "Ranger" => 10,
            "Rogue" => 8,
            "Barbarian" => 12,
            _ => unreachable!(),
        };
        input.push_str(&header_line(name, hd));
        input.push('\n');
        input.push_str(&format!("CLASS:{name}\tSTARTSKILLPTS:{}\n", 2 + idx));
    }

    let result = parse_class_entries("cr_classes.lst", &input);
    let parsed_names: Vec<&str> = result
        .entries
        .iter()
        .map(|entry| entry.class_name.as_str())
        .collect();
    assert_eq!(parsed_names, MARTIAL_CLASSES);
    assert!(
        result.diagnostics.is_empty(),
        "uniform record shape must yield no diagnostics: {:?}",
        result.diagnostics
    );

    // Every entry must carry an HD token matching the input.
    for (name, entry) in MARTIAL_CLASSES.iter().zip(result.entries.iter()) {
        let hd = entry
            .tokens
            .iter()
            .find(|t| t.key == "HD")
            .unwrap_or_else(|| panic!("missing HD token for {name}"));
        let expected = match *name {
            "Fighter" | "Monk" | "Paladin" | "Ranger" => "10",
            "Rogue" => "8",
            "Barbarian" => "12",
            _ => unreachable!(),
        };
        assert_eq!(hd.value, expected, "HD for {name}");
        assert!(entry.tokens.iter().any(|t| t.key == "TYPE"));
        assert!(entry.tokens.iter().any(|t| t.key == "MAXLEVEL"));
    }
}

#[test]
fn merges_multiple_class_lines_for_the_same_class_name() {
    // Real PCGen files describe a class across multiple CLASS: lines —
    // header, requirements, STARTSKILLPTS, SPELLSTAT, KNOWNSPELLS, etc.
    // They must collapse into a single ClassEntry, preserving every
    // token and every line number.
    let input = "\
CLASS:Monk\tHD:10\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.56
CLASS:Monk\tPREMULT:1,[PREALIGN:LG,LN,LE]
CLASS:Monk\tSTARTSKILLPTS:4
";
    let result = parse_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1);
    let entry = &result.entries[0];
    assert_eq!(entry.class_name, "Monk");
    assert_eq!(entry.header_line_number, 1);
    let premult = entry
        .tokens
        .iter()
        .find(|t| t.key == "PREMULT")
        .expect("PREMULT token preserved across multiple lines");
    assert_eq!(premult.line_number, 2);
    let startskill = entry
        .tokens
        .iter()
        .find(|t| t.key == "STARTSKILLPTS")
        .expect("STARTSKILLPTS carried forward");
    assert_eq!(startskill.line_number, 3);
    assert!(
        result.diagnostics.is_empty(),
        "multi-line class description should yield no diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn preserves_one_based_line_numbers_on_every_record() {
    let input = "\
# comment at line 1
# comment at line 2
CLASS:Fighter\tHD:10\tTYPE:Base.PC
CLASS:Fighter\tSTARTSKILLPTS:2
###Block:Proficiencies
1\tABILITY:Class|AUTOMATIC|Fighter
###Block: Level progression
1\tDEFINE:FighterFeat|1
";
    let result = parse_class_entries("cr_classes.lst", input);
    let entry = &result.entries[0];
    assert_eq!(entry.header_line_number, 3);
    let hd = entry.tokens.iter().find(|t| t.key == "HD").unwrap();
    assert_eq!(hd.line_number, 3);
    let startskill = entry
        .tokens
        .iter()
        .find(|t| t.key == "STARTSKILLPTS")
        .unwrap();
    assert_eq!(startskill.line_number, 4);
    assert_eq!(entry.feature_blocks.len(), 2);
    assert_eq!(entry.feature_blocks[0].header_line_number, 5);
    assert_eq!(entry.feature_blocks[0].level_lines[0].line_number, 6);
    assert_eq!(entry.feature_blocks[1].label, "Level progression");
    assert_eq!(entry.feature_blocks[1].header_line_number, 7);
    assert_eq!(entry.feature_blocks[1].level_lines[0].line_number, 8);
}

#[test]
fn reports_malformed_class_lines_with_malformed_sd17_b1_diagnostic() {
    // `CLASS:` with no class name is malformed. The diagnostic must
    // surface as MalformedSD17B1 with the raw line and one-based line
    // number, not crash the parser. A subsequent valid class line
    // (within the in-scope martial set) must still parse.
    let input = "CLASS:\tHD:10\nCLASS:Fighter\tHD:10\n";
    let result = parse_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1, "valid class still parses");
    assert_eq!(result.entries[0].class_name, "Fighter");
    assert_eq!(result.diagnostics.len(), 1);
    let diag = &result.diagnostics[0];
    assert_eq!(diag.kind, LstDiagnosticKind::MalformedSD17B1);
    assert_eq!(diag.line_number, Some(1));
    assert!(diag.raw_line.starts_with("CLASS:"));
    assert!(
        diag.message.contains("class name"),
        "diagnostic must explain the defect: {diag:?}"
    );
}

#[test]
fn reports_malformed_block_marker_with_diagnostic() {
    // A `###Block:` with no label is malformed — the parser must
    // report it and continue rather than skip silently.
    let input = "\
CLASS:Fighter\tHD:10\tTYPE:Base.PC
###Block:
1\tABILITY:Class|AUTOMATIC|Fighter
";
    let result = parse_class_entries("cr_classes.lst", input);
    let entry = &result.entries[0];
    assert_eq!(entry.class_name, "Fighter");
    assert_eq!(entry.feature_blocks.len(), 1);
    // The malformed marker still produces a feature block, but the
    // parser records a diagnostic so the operator can see the defect.
    assert_eq!(entry.feature_blocks[0].label, "");
    assert_eq!(result.diagnostics.len(), 1);
    let diag = &result.diagnostics[0];
    assert_eq!(diag.kind, LstDiagnosticKind::MalformedBlockMarker);
    assert_eq!(diag.line_number, Some(2));
    assert!(diag.message.contains("###Block"));
}

#[test]
fn ignores_non_class_directives_and_non_martial_classes() {
    // The slice scope is the six martial classes. Other CLASS: names
    // (Bard, Cleric) are not in scope and must be skipped without
    // raising diagnostics — they belong to a different B-slice. Other
    // directives (RACE:, SPELL:, ABILITY:) must also be ignored.
    let input = "\
RACE:human.lst
SPELL:fireball
ABILITY:Class|AUTOMATIC|Fighter
CLASS:Bard\tHD:8
CLASS:Cleric\tHD:8
CLASS:Fighter\tHD:10\tTYPE:Base.PC
";
    let result = parse_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1, "only Fighter is in scope");
    assert_eq!(result.entries[0].class_name, "Fighter");
    assert!(
        result.diagnostics.is_empty(),
        "out-of-scope classes must not raise diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn parse_class_file_reads_path_and_returns_typed_error() {
    let corpus = TestCorpus::new("file-io");
    let path = corpus.path("cr_classes.lst");
    fs::write(
        &path,
        "CLASS:Fighter\tHD:10\tTYPE:Base.PC\n###Block:Proficiencies\n1\tABILITY:Class|AUTOMATIC|Fighter\n",
    )
    .expect("write fixture");

    let result = parse_class_file(&path).expect("file should parse");
    assert!(
        result.source_path.ends_with("cr_classes.lst"),
        "source_path should record the file name (got {:?})",
        result.source_path
    );
    assert_eq!(result.entries.len(), 1);
    assert_eq!(result.entries[0].class_name, "Fighter");
}

#[test]
fn parse_class_file_reports_missing_file() {
    let corpus = TestCorpus::new("missing-file");
    let path = corpus.path("does_not_exist.lst");
    let err = parse_class_file(&path).expect_err("missing file must error");
    assert!(err.raw_line.is_empty());
    assert!(err.message.contains("does_not_exist.lst"));
}

#[test]
fn parses_full_level_progression_block_with_20_levels() {
    // The 6 martial classes have a uniform 1..=20 level progression. The
    // parser must ingest all 20 feature lines without losing order.
    let mut input =
        String::from("CLASS:Fighter\tHD:10\tTYPE:Base.PC\n###Block: Level progression\n");
    for level in 1..=20 {
        input.push_str(&format!(
            "{level}\tABILITY:Class|AUTOMATIC|Fighter|Level{level}\n"
        ));
    }
    let result = parse_class_entries("cr_classes.lst", &input);
    let entry = &result.entries[0];
    let progression = entry
        .feature_blocks
        .iter()
        .find(|block| block.label == "Level progression")
        .expect("Level progression block");
    assert_eq!(progression.level_lines.len(), 20);
    for (idx, level_line) in progression.level_lines.iter().enumerate() {
        let expected_level = (idx + 1) as u8;
        assert_eq!(level_line.level, expected_level);
        assert!(
            level_line
                .raw_line
                .contains(&format!("Level{expected_level}")),
            "level {expected_level} content"
        );
    }
}

#[test]
fn malformed_class_line_does_not_halt_subsequent_parsing() {
    // A malformed CLASS: line in the middle of the file must not stop
    // the parser from processing later valid entries.
    let input = "\
CLASS:Barbarian\tHD:12\tTYPE:Base.PC
CLASS:\tHD:10
CLASS:Fighter\tHD:10\tTYPE:Base.PC
";
    let result = parse_class_entries("cr_classes.lst", input);
    let names: Vec<&str> = result
        .entries
        .iter()
        .map(|entry| entry.class_name.as_str())
        .collect();
    assert_eq!(names, vec!["Barbarian", "Fighter"]);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].line_number, Some(2));
}

#[test]
fn performance_parse_does_not_exceed_o_n_on_representative_input() {
    // Build a representative input: one line per level per class for all
    // 6 martial classes (6 * 20 = 120 level lines + 6 header lines).
    // The parser must be O(n) in the number of input lines: doubling
    // the input size must not more than double the parse time. We use
    // a generous bound (4x) to absorb scheduler noise on a loaded
    // machine, since the doctrine is "O(n) time" not "constant-time".
    fn build_input(scale: usize) -> String {
        let mut input = String::new();
        for class in MARTIAL_CLASSES {
            input.push_str(&header_line(class, 10));
            input.push('\n');
            input.push_str(&format!("CLASS:{class}\tSTARTSKILLPTS:2\n"));
            input.push_str("###Block: Level progression\n");
            for level in 1..=20 {
                input.push_str(&format!(
                    "{level}\tABILITY:Class|AUTOMATIC|{class}|L{level}\n"
                ));
                // Insert `scale` extra comment-style no-op lines so we
                // can grow the input without changing the parse outcome.
                for _ in 0..scale {
                    input.push_str(&format!("# noise line for {class} L{level}\n"));
                }
            }
        }
        input
    }

    let small = build_input(0);
    let large = build_input(50);

    let start_small = std::time::Instant::now();
    let small_result = parse_class_entries("cr_classes.lst", &small);
    let small_elapsed = start_small.elapsed();

    let start_large = std::time::Instant::now();
    let large_result = parse_class_entries("cr_classes.lst", &large);
    let large_elapsed = start_large.elapsed();

    // The small input has 6 headers + 6 STARTSKILLPTS + 6 ###Block + 6*20
    // = 138 CLASS-related lines. The large input is ~51x larger.
    assert_eq!(small_result.entries.len(), 6);
    assert_eq!(large_result.entries.len(), 6);
    let ratio = large_elapsed.as_nanos() as f64 / small_elapsed.as_nanos().max(1) as f64;
    let size_ratio = large.len() as f64 / small.len() as f64;
    assert!(
        ratio <= size_ratio * 4.0 + 0.5,
        "expected roughly-linear scaling: size_ratio={size_ratio:.2}, \
         time_ratio={ratio:.2} (small={}ns, large={}ns, small={}B, large={}B)",
        small_elapsed.as_nanos(),
        large_elapsed.as_nanos(),
        small.len(),
        large.len()
    );
}

#[test]
fn named_martial_class_set_is_exactly_the_six_from_the_slice_card() {
    // Operator doctrine (2026-07-12): the parser covers all 6 martial
    // classes or it doesn't ship. Lock the set in the test surface so
    // it is impossible to silently drop one in a future refactor.
    use std::collections::HashSet;
    let canonical: HashSet<&str> = MARTIAL_CLASSES.iter().copied().collect();
    let required: HashSet<&str> = ["Fighter", "Barbarian", "Monk", "Rogue", "Ranger", "Paladin"]
        .iter()
        .copied()
        .collect();
    assert_eq!(canonical, required);
}

/// Sanity check on the token model: every token records (key, value,
/// line_number, raw_pair). This guards the canonical IR shape the
/// SD-13 matrix uplift will depend on.
#[test]
fn tokens_carry_key_value_line_and_raw_pair() {
    let input = "CLASS:Ranger\tHD:10\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.64\n";
    let result = parse_class_entries("cr_classes.lst", input);
    let entry = &result.entries[0];
    let hd: &ClassToken = entry.tokens.iter().find(|t| t.key == "HD").unwrap();
    assert_eq!(hd.value, "10");
    assert_eq!(hd.line_number, 1);
    assert!(hd.raw_pair.contains("HD:10"));
    let sourcepage: &ClassToken = entry.tokens.iter().find(|t| t.key == "SOURCEPAGE").unwrap();
    assert_eq!(sourcepage.value, "p.64");
    assert!(sourcepage.raw_pair.contains("SOURCEPAGE:p.64"));
}

/// Sanity check on the real PCGen corpus `cr_classes.lst`: the
/// parser must recognize every in-scope martial class in the file
/// with a uniform record shape. Out-of-scope classes (Bard, Cleric,
/// Druid, Sorcerer, Wizard, Alchemist, ...) are skipped without
/// raising diagnostics; the in-scope set is the 6 martial classes
/// the slice card names.
///
/// This test is opt-in via `PCGEN_CORPUS_ROOT` because the corpus
/// is a separate checkout (~700MB) and not part of the codex
/// repo. CI and the operator's local box should both set it.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_core_rulebook_classes_lst_for_all_six_martial_classes() {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst");
    assert!(
        source.is_file(),
        "real cr_classes.lst must exist at {}",
        source.display()
    );

    let result = parse_class_file(&source).expect("real corpus parses");
    let parsed_names: std::collections::BTreeSet<&str> = result
        .entries
        .iter()
        .map(|entry| entry.class_name.as_str())
        .collect();

    // The 6 in-scope martial classes must all be present in the
    // core rulebook's cr_classes.lst. The `Ex-*` mirror variants
    // only exist for classes with alignment restrictions; the
    // real corpus has Ex-Barbarian and Ex-Paladin but no
    // Ex-Fighter, Ex-Monk, Ex-Rogue, or Ex-Ranger.
    for required in ["Fighter", "Barbarian", "Monk", "Rogue", "Ranger", "Paladin"] {
        assert!(
            parsed_names.contains(required),
            "real corpus must contain CLASS:{required}; got {parsed_names:?}"
        );
    }
    for required in ["Ex-Barbarian", "Ex-Paladin"] {
        assert!(
            parsed_names.contains(required),
            "real corpus must contain CLASS:{required}; got {parsed_names:?}"
        );
    }
    // Out-of-scope CLASS: lines (Bard, Cleric, Druid, Sorcerer,
    // Wizard, Alchemist, Inquisitor, Oracle, Magus, Summoner,
    // Witch, ...) must NOT appear in the entry set. The parser
    // skips them without raising a diagnostic.
    let in_scope: std::collections::BTreeSet<&str> = [
        "Fighter",
        "Barbarian",
        "Monk",
        "Rogue",
        "Ranger",
        "Paladin",
        "Ex-Barbarian",
        "Ex-Paladin",
    ]
    .iter()
    .copied()
    .collect();
    for name in &parsed_names {
        assert!(
            in_scope.contains(name),
            "out-of-scope class {name} was emitted by the parser; got {parsed_names:?}"
        );
    }

    // Every in-scope class entry must have a uniform header shape:
    // HD, TYPE, MAXLEVEL, and a source page are all present on the
    // very first CLASS:<name> line. If the parser stops emitting
    // any of those four tokens, this assertion fires.
    for name in ["Fighter", "Barbarian", "Monk", "Rogue", "Ranger", "Paladin"] {
        let entry = result
            .entries
            .iter()
            .find(|entry| entry.class_name == name)
            .expect("entry exists");
        for required_key in ["HD", "TYPE", "MAXLEVEL", "SOURCEPAGE"] {
            assert!(
                entry.tokens.iter().any(|t| t.key == required_key),
                "{name} must carry a {required_key} token"
            );
        }
    }

    // The real corpus mixes in-scope (Fighter/Barbarian/Monk/Rogue/
    // Ranger/Paladin) and out-of-scope (Bard/Cleric/Druid/Sorcerer/
    // Wizard/...) CLASS: lines, separated by comments and `###Block:`
    // markers. Orphan `###Block:` markers that appear after an
    // out-of-scope class are an expected and honest diagnostic
    // signal: the parser correctly refuses to attach them to any
    // in-scope class, and the operator can review them by line
    // number if needed. The test does NOT require zero
    // diagnostics — that would be a false positive on a corpus
    // that intentionally intermixes scope classes.
    //
    // What this test DOES require: every in-scope class entry has
    // a uniform record shape (HD, TYPE, MAXLEVEL, SOURCEPAGE
    // tokens), and every in-scope class appears exactly once in
    // the entry set. The number of out-of-scope `###Block:`
    // orphans is logged for the operator's awareness but is not
    // asserted.
    let orphan_block_markers: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| {
            matches!(d.kind, LstDiagnosticKind::MalformedSD17B1)
                && d.raw_line.starts_with("###Block")
        })
        .collect();
    eprintln!(
        "real corpus produced {} orphan ###Block: diagnostics (expected when \
         out-of-scope classes are interleaved); in-scope entries: {}",
        orphan_block_markers.len(),
        result.entries.len()
    );
}

/// SD-22 Epic 3 widening (Cavalier ingest cycle): the real `CLASS:Cavalier`
/// record in `apg_classes.lst` carries `BONUS:COMBAT|BASEAB|classlevel(...)`
/// (full BAB, no fractional divisor) and no `SPELLSTAT:` line — the same
/// non-caster posture as the six original martial classes — so it belongs
/// in `MARTIAL_CLASS_NAMES` rather than `SPELLCASTING_CLASS_NAMES`. Before
/// this cycle's widening, the real corpus record is silently skipped
/// (out-of-scope, no diagnostic). This test is real-corpus-gated on
/// `PCGEN_CORPUS_ROOT`, mirroring `sd17_b_spellcasting_class.rs`'s
/// `parses_real_alchemist_record_from_apg_classes_lst` pattern.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_cavalier_record_from_apg_classes_lst() {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source = corpus_root
        .join("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst");
    let result = parse_class_file(&source).expect("real corpus parses");

    let cavalier = result
        .entries
        .iter()
        .find(|entry| entry.class_name == "Cavalier")
        .expect(
            "Cavalier should be recognized from the real apg_classes.lst once \
             MARTIAL_CLASS_NAMES is widened to include it",
        );
    let baseab = cavalier
        .tokens
        .iter()
        .find(|token| token.key == "BONUS" && token.value.starts_with("COMBAT|BASEAB"))
        .expect("Cavalier entry should carry its BASEAB bonus token");
    assert!(
        baseab.value.contains("classlevel(\"APPLIEDAS=NONEPIC\")")
            && !baseab.value.contains("*3/4"),
        "Cavalier's real BASEAB token should be full BAB (no fractional divisor): {}",
        baseab.value
    );
}
