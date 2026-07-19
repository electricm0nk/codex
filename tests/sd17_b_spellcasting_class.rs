//! SD-17 Slice B-2 acceptance tests — LST parser for the spellcasting
//! `CLASS:` object kind.
//!
//! Scope is bounded to the five spellcasting classes named in the slice
//! card body (Cleric, Druid, Wizard, Sorcerer, Bard) plus their `Ex-*`
//! mirror variants. The parser must:
//!
//! - recognize every `CLASS:` line in the corpus for the named
//!   spellcasting set (do NOT narrow to one class);
//! - preserve one-based source line numbers on every record;
//! - carry every tab-delimited `KEY:VAL` token pair to the canonical IR;
//! - recognize the spellcasting sub-shapes the real PCGen corpus uses:
//!     - `MEMORIZE:NO` on the SPELLSTAT line → spontaneous posture
//!     - `SPELLBOOK:YES` on the SPELLSTAT line → spellbook-prepared posture
//!     - absent signals → standard-prepared posture
//!     - `KNOWNSPELLS:LEVEL=N|...|LEVEL=9` → list of automatically-known
//!       spell levels carried on the SPELLSTAT line
//!     - `###Block: Level progression` and `###Block: Spell Level Progression`
//!       rows of `<level>\tCAST:0,1\t...\tKNOWN:4,2` → the per-level
//!       spell-progression curve
//!     - `###Block: Domain Selections` rows of `0\tDOMAIN:<name>|...` →
//!       domain selection list (Cleric, Druid)
//!     - `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>` lines that
//!       follow a Wizard's HEADER block → arcane-school specialization
//!       (Wizard)
//! - treat any `CLASS:` line for an out-of-scope class name (e.g. a
//!   martial class) as out of scope and skip it without a diagnostic;
//! - report malformed `CLASS:` lines and non-numeric `CAST:` rows as
//!   `MalformedSD17B2` diagnostics with line numbers and raw evidence;
//! - parse deterministically with no per-class special-casing inside the
//!   parser core (each of the five classes is just whatever name the test
//!   scopes).
//!
//! The verification list on the slice card mandates that every record
//! carries a source line number and that parsing does not exceed O(n)
//! time on a representative corpus file; both are covered below.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use codex::pcgen_import::lst_parser::spellcasting_class::{
    CastingPosture, ClassBlockKind, SpellLevelRow, SpellcastingClassDiagnostic,
    SpellcastingClassDiagnosticKind, SpellcastingClassEntry, SpellcastingClassParseResult,
    parse_spellcasting_class_entries, parse_spellcasting_class_file,
};

/// The five spellcasting classes named in the slice card body.
const SPELLCASTING_CLASSES: &[&str] = &["Cleric", "Druid", "Wizard", "Sorcerer", "Bard"];

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
            "codex_sd17_b2_spellcasting_class_{}_{}_{}",
            name,
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test corpus root");
        Self { root }
    }

    fn write(&self, relative_path: &str, contents: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent dir");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    fn path(&self, relative_path: &str) -> PathBuf {
        self.root.join(relative_path)
    }
}

/// Build a representative header line for one of the five spellcasting
/// classes using the same shape the real `cr_classes.lst` uses
/// (tab-delimited KEY:VAL tokens).
fn header_line(class_name: &str, hd: u8, spell_stat: &str, spell_posture_marker: &str) -> String {
    let posture = if spell_posture_marker.is_empty() {
        String::new()
    } else {
        format!("\t{spell_posture_marker}")
    };
    format!(
        "CLASS:{class_name}\tHD:{hd}\t\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.0\t\
         DEFINE:{class_name}LVL|0\tBONUS:COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")\t\
         ROLE:{class_name}\t\tFACT:ClassType|PC\tFACT:Abb|{short}\tFACT:SpellType|Divine",
        short = &class_name[..3]
    ) + "\n"
        + &format!(
            "CLASS:{class_name}\tSTARTSKILLPTS:2\n\
             CLASS:{class_name}\tSPELLSTAT:{spell_stat}{posture}\tBONUS:CASTERLEVEL|{class_name}|CasterLevel",
        )
}

fn cleric_block() -> &'static str {
    "\
CLASS:Cleric\tHD:8\t\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.0\tDEFINE:ClericLVL|0\tBONUS:COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")\tFACT:SpellType|Divine
CLASS:Cleric\tSTARTSKILLPTS:2
CLASS:Cleric\tSPELLSTAT:WIS\tKNOWNSPELLS:LEVEL=0|LEVEL=1|LEVEL=2|LEVEL=3|LEVEL=4|LEVEL=5|LEVEL=6|LEVEL=7|LEVEL=8|LEVEL=9\tBONUS:CASTERLEVEL|Cleric|CasterLevel
###Block: Domain Selections
0\tDOMAIN:Air|PREABILITY:1,Air Domain
0\tDOMAIN:Animal|PREABILITY:1,Animal Domain
0\tDOMAIN:Death|PREABILITY:1,Death Domain
0\tDOMAIN:Destruction|PREABILITY:1,Destruction Domain
###Block: Level progression
1\tCAST:3,1
2\tCAST:3,2
20\tCAST:4,4,4,4,4,4,4,4,4,4
"
}

fn sorcerer_block() -> &'static str {
    "\
CLASS:Sorcerer\tHD:6\t\tTYPE:Base.PC.SpontaneousArcane.Spontaneous\tMAXLEVEL:20\tSOURCEPAGE:p.0\tDEFINE:SorcererLVL|0\tBONUS:COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")\tFACT:SpellType|Arcane
CLASS:Sorcerer\tSTARTSKILLPTS:2
CLASS:Sorcerer\tSPELLSTAT:CHA\tMEMORIZE:NO\tBONUS:CASTERLEVEL|Sorcerer|CasterLevel
###Block:Proficiencies
1\tABILITY:Class|AUTOMATIC|Sorcerer
###Block: Spell Level Progression
1\tCAST:0,3\t\t\t\tKNOWN:4,2
2\tCAST:0,4\t\t\t\tKNOWN:5,2
20\tCAST:0,6,6,6,6,6,6,6,6,6\tKNOWN:9,5,5,4,4,4,3,3,2
"
}

fn wizard_block() -> &'static str {
    "\
CLASS:Wizard\tHD:6\t\tTYPE:Base.PC\tMAXLEVEL:20\tALLOWBASECLASS:NO\tSOURCEPAGE:p.0\tVISIBLE:YES\tDEFINE:WizardLVL|0\tFACT:SpellType|Arcane
CLASS:Wizard\tSTARTSKILLPTS:2
CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES\tBONUS:CASTERLEVEL|Wizard|CasterLevel
SUBCLASS:Abjurer\t\tCOST:0\tCHOICE:SCHOOL|Abjuration
SUBCLASS:Conjurer\t\tCOST:0\tCHOICE:SCHOOL|Conjuration
SUBCLASS:Diviner\t\tCOST:0\tCHOICE:SCHOOL|Divination
SUBCLASS:Enchanter\tCOST:0\tCHOICE:SCHOOL|Enchantment
SUBCLASS:Evoker\t\tCOST:0\tCHOICE:SCHOOL|Evocation
SUBCLASS:Illusionist\tCOST:0\tCHOICE:SCHOOL|Illusion
SUBCLASS:Necromancer\tCOST:0\tCHOICE:SCHOOL|Necromancy
SUBCLASS:Transmuter\tCOST:0\tCHOICE:SCHOOL|Transmutation
SUBCLASS:Universalist\tCOST:0
###Block: Level progression
1\tCAST:3,1
20\tCAST:4,4,4,4,4,4,4,4,4,4
"
}

fn bard_block() -> &'static str {
    "\
CLASS:Bard\tHD:8\t\tTYPE:Base.PC.SpontaneousArcane.Spontaneous\tMAXLEVEL:20\tSOURCEPAGE:p.0\tFACT:SpellType|Arcane
CLASS:Bard\tSTARTSKILLPTS:6
CLASS:Bard\tSPELLSTAT:CHA\tMEMORIZE:NO\tBONUS:CASTERLEVEL|Bard|CasterLevel
###Block: Level progression
1\tCAST:0,1\t\t\tKNOWN:4,2
20\tCAST:0,5,5,5,5,5,5,5,5,5\tKNOWN:6,6,6,6,6,6,5,5,5
"
}

#[allow(dead_code)]
fn druid_block() -> &'static str {
    "\
CLASS:Druid\tHD:8\t\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.0\tDEFINE:DruidLVL|0\tFACT:SpellType|Divine
CLASS:Druid\tSTARTSKILLPTS:4
CLASS:Druid\tSPELLSTAT:WIS\tKNOWNSPELLS:LEVEL=0|LEVEL=1|LEVEL=2|LEVEL=3|LEVEL=4|LEVEL=5|LEVEL=6|LEVEL=7|LEVEL=8|LEVEL=9\tBONUS:CASTERLEVEL|Druid|CasterLevel
###Block: Domain Selections
0\tDOMAIN:Air|PREABILITY:1,Air Domain
0\tDOMAIN:Animal|PREABILITY:1,Animal Domain
0\tDOMAIN:Earth|PREABILITY:1,Earth Domain
0\tDOMAIN:Fire|PREABILITY:1,Fire Domain
0\tDOMAIN:Plant|PREABILITY:1,Plant Domain
0\tDOMAIN:Water|PREABILITY:1,Water Domain
0\tDOMAIN:Weather|PREABILITY:1,Weather Domain
###Block: Level progression
1\tCAST:3,1
20\tCAST:4,4,4,4,4,4,4,4,4,4
"
}

fn assert_first_class_entry<'a>(
    result: &'a SpellcastingClassParseResult,
    expected_name: &str,
) -> &'a SpellcastingClassEntry {
    result
        .entries
        .iter()
        .find(|entry| entry.class_name == expected_name)
        .unwrap_or_else(|| {
            panic!(
                "expected class entry for {expected_name} in result {:#?}",
                result.entries
            )
        })
}

#[test]
fn parses_cleric_with_spell_progression_and_domain_selections() {
    let input = cleric_block();
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);

    assert_eq!(
        result.diagnostics,
        Vec::<SpellcastingClassDiagnostic>::new(),
        "well-formed cleric input must not produce diagnostics: {:?}",
        result.diagnostics
    );

    let cleric = assert_first_class_entry(&result, "Cleric");
    assert_eq!(cleric.header_line_number, 1);
    assert!(
        cleric.spell_stat.is_some(),
        "cleric SPELLSTAT must be captured"
    );
    assert_eq!(cleric.spell_stat.as_deref(), Some("WIS"));
    // Standard-prepared posture: no MEMORIZE:NO, no SPELLBOOK:YES.
    assert_eq!(cleric.casting_posture, Some(CastingPosture::Prepared));
    // KNOWNSPELLS:LEVEL=0|...|LEVEL=9 → 10 entries.
    assert_eq!(
        cleric.automatically_known_levels,
        (0..=9_u8).collect::<Vec<u8>>(),
        "KNOWNSPELLS list must surface every level present"
    );
    // Domain Selections block: four entries.
    assert_eq!(cleric.domain_selections.len(), 4);
    assert_eq!(cleric.domain_selections[0].name, "Air");
    // Spell progression curve: three rows (1, 2, 20).
    assert_eq!(cleric.spell_progression.len(), 3);
    assert_eq!(cleric.spell_progression[0].level, 1);
    assert_eq!(cleric.spell_progression[0].cast_slots, vec![3, 1]);
    assert_eq!(cleric.spell_progression[2].level, 20);
    assert_eq!(
        cleric.spell_progression[2].cast_slots,
        vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4]
    );
}

#[test]
fn recognizes_sorcerer_and_bard_as_spontaneous_posture() {
    let mut input = sorcerer_block().to_string();
    input.push('\n');
    input.push_str(bard_block());

    let result = parse_spellcasting_class_entries("cr_classes.lst", &input);
    assert!(
        result.diagnostics.is_empty(),
        "spontaneous-class input must not produce diagnostics: {:?}",
        result.diagnostics
    );

    let sorcerer = assert_first_class_entry(&result, "Sorcerer");
    assert_eq!(sorcerer.casting_posture, Some(CastingPosture::Spontaneous));
    assert_eq!(sorcerer.spell_stat.as_deref(), Some("CHA"));
    // Sorcerer has no KNOWNSPELLS list (spontaneous uses KNOWN per-row).
    assert!(
        sorcerer.automatically_known_levels.is_empty(),
        "spontaneous casters must not advertise an automatically-known list"
    );
    // Sorcerer Spell Level Progression is recognized under a different
    // block label than Cleric's "Level progression" — both are accepted
    // by the parser.
    assert_eq!(sorcerer.spell_progression.len(), 3);

    let bard = assert_first_class_entry(&result, "Bard");
    assert_eq!(bard.casting_posture, Some(CastingPosture::Spontaneous));
    assert_eq!(bard.spell_progression.len(), 2);
}

#[test]
fn recognizes_wizard_spellbook_posture_and_school_specializations() {
    let input = wizard_block();
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);

    assert!(
        result.diagnostics.is_empty(),
        "wizard input must not produce diagnostics: {:?}",
        result.diagnostics
    );

    let wizard = assert_first_class_entry(&result, "Wizard");
    assert_eq!(
        wizard.casting_posture,
        Some(CastingPosture::Spellbook),
        "SPELLBOOK:YES on the SPELLSTAT line must produce Spellbook posture"
    );
    assert_eq!(wizard.spell_stat.as_deref(), Some("INT"));
    // Wizard advertises 9 schools in this corpus (incl. Universalist,
    // which is mapped to the implicit "Universal" school name because
    // it carries no CHOICE:SCHOOL token in the LST).
    assert_eq!(wizard.school_specializations.len(), 9);
    let school_names: Vec<&str> = wizard
        .school_specializations
        .iter()
        .map(|school| school.school.as_str())
        .collect();
    assert!(school_names.contains(&"Abjuration"));
    assert!(school_names.contains(&"Conjuration"));
    assert!(school_names.contains(&"Universal"));
    // Universalist SUBCLASS without CHOICE:SCHOOL surfaces as
    // SchoolSpecialization { school: "Universal", subclass_name:
    // "Universalist" }; the parser records the implicit mapping
    // rather than an empty school name.
    assert!(
        wizard
            .school_specializations
            .iter()
            .any(|school| school.school == "Universal" && school.subclass_name == "Universalist"),
        "Universalist SUBCLASS without CHOICE:SCHOOL must map to the implicit Universal school"
    );
}

#[test]
fn recognises_spell_progression_curve_and_known_spells_per_level() {
    let input = sorcerer_block();
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    let sorcerer = assert_first_class_entry(&result, "Sorcerer");

    // Level 1 → CAST:0,3, KNOWN:4,2.
    let level1 = sorcerer
        .spell_progression
        .iter()
        .find(|row| row.level == 1)
        .expect("level-1 row present");
    assert_eq!(level1.cast_slots, vec![0, 3]);
    assert_eq!(level1.known_spells, vec![4, 2]);
    // Level 20 has 10 cast slots (PF1 Sorcerer level-20 progression
    // has the 0-level cantrip plus 9 spell levels) and 9 known slots.
    let level20 = sorcerer
        .spell_progression
        .iter()
        .find(|row| row.level == 20)
        .expect("level-20 row present");
    assert_eq!(level20.cast_slots.len(), 10);
    assert_eq!(level20.known_spells.len(), 9);
}

#[test]
fn parses_every_spellcasting_class_with_uniform_record_shape() {
    let mut input = String::from("SOURCELONG:Core Rulebook\n");
    for name in SPELLCASTING_CLASSES {
        let (hd, stat, posture_marker) = match *name {
            "Cleric" => (8_u8, "WIS", ""),
            "Druid" => (8_u8, "WIS", ""),
            "Wizard" => (6_u8, "INT", "SPELLBOOK:YES"),
            "Sorcerer" => (6_u8, "CHA", "MEMORIZE:NO"),
            "Bard" => (8_u8, "CHA", "MEMORIZE:NO"),
            _ => unreachable!(),
        };
        input.push_str(&header_line(name, hd, stat, posture_marker));
        input.push('\n');
    }

    let result = parse_spellcasting_class_entries("cr_classes.lst", &input);
    let parsed_names: Vec<&str> = result
        .entries
        .iter()
        .map(|entry| entry.class_name.as_str())
        .collect();
    assert_eq!(parsed_names, SPELLCASTING_CLASSES);
    assert!(
        result.diagnostics.is_empty(),
        "uniform record shape must yield no diagnostics: {:?}",
        result.diagnostics
    );

    for (name, entry) in SPELLCASTING_CLASSES.iter().zip(result.entries.iter()) {
        let hd = entry
            .tokens
            .iter()
            .find(|token| token.key == "HD")
            .unwrap_or_else(|| panic!("HD token missing for {name}"));
        let expected = match *name {
            "Fighter" | "Monk" | "Paladin" | "Ranger" | "Bard" => "8",
            "Cleric" | "Druid" => "8",
            "Wizard" | "Sorcerer" => "6",
            _ => unreachable!(),
        };
        assert_eq!(hd.value, expected, "HD value for {name}");
    }
}

#[test]
fn merges_multiple_class_lines_for_the_same_class_name() {
    // Real PCGen files describe a class across multiple CLASS: lines —
    // header, requirements, STARTSKILLPTS, SPELLSTAT, KNOWNSPELLS, etc.
    // They must collapse into a single record, preserving every token
    // and every line number.
    let input = "\
CLASS:Wizard\tHD:6\tTYPE:Base.PC\tMAXLEVEL:20\tSOURCEPAGE:p.0
CLASS:Wizard\tPREMULT:1,[PREALIGN:Any]
CLASS:Wizard\tSTARTSKILLPTS:2
CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES
";
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1, "all four lines merge into one");
    let wizard = &result.entries[0];
    assert_eq!(wizard.class_name, "Wizard");
    assert_eq!(wizard.header_line_number, 1);
    let premult = wizard
        .tokens
        .iter()
        .find(|token| token.key == "PREMULT")
        .expect("PREMULT carries across multiple lines");
    assert_eq!(premult.line_number, 2);
    assert!(
        result.diagnostics.is_empty(),
        "multi-line class description must yield no diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn preserves_one_based_line_numbers_on_every_record() {
    let input = "\
# comment at line 1
# comment at line 2
CLASS:Sorcerer\tHD:6\tTYPE:Base.PC
CLASS:Sorcerer\tSTARTSKILLPTS:2
CLASS:Sorcerer\tSPELLSTAT:CHA\tMEMORIZE:NO
###Block: Spell Level Progression
1\tCAST:0,3\t\t\t\tKNOWN:4,2
###Block: Domain Selections
0\tDOMAIN:Air|PREABILITY:1,Air
";
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    let sorcerer = &result.entries[0];
    assert_eq!(sorcerer.header_line_number, 3);
    let hd = sorcerer.tokens.iter().find(|t| t.key == "HD").unwrap();
    assert_eq!(hd.line_number, 3);
    let memorize = sorcerer
        .tokens
        .iter()
        .find(|token| token.key == "MEMORIZE")
        .expect("MEMORIZE token carried forward");
    assert_eq!(memorize.line_number, 5);
    assert_eq!(sorcerer.spell_progression[0].line_number, 7);
    assert_eq!(sorcerer.domain_selections[0].line_number, 9);
}

#[test]
fn reports_malformed_class_lines_with_malformed_sd17_b2_diagnostic() {
    // `CLASS:` with no class name is malformed. The diagnostic must
    // surface as MalformedSD17B2 with the raw line and one-based line
    // number, not crash the parser. A subsequent valid class line
    // (within the in-scope spellcasting set) must still parse.
    let input = "CLASS:\tHD:6\nCLASS:Sorcerer\tHD:6\n";
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1, "valid class still parses");
    assert_eq!(result.entries[0].class_name, "Sorcerer");
    assert_eq!(result.diagnostics.len(), 1);
    let diag = &result.diagnostics[0];
    assert_eq!(diag.kind, SpellcastingClassDiagnosticKind::MalformedSD17B2);
    assert_eq!(diag.line_number, Some(1));
    assert!(diag.raw_line.starts_with("CLASS:"));
    assert!(
        diag.message.contains("class name"),
        "diagnostic should describe the failure: {}",
        diag.message
    );
}

#[test]
fn reports_malformed_cast_row_when_payload_is_not_comma_separated_integers() {
    let input = "\
CLASS:Sorcerer\tHD:6\tTYPE:Base.PC
CLASS:Sorcerer\tSPELLSTAT:CHA\tMEMORIZE:NO
###Block: Spell Level Progression
1\tCAST:not-a-number
2\tCAST:0,4\t\t\t\tKNOWN:5,2
";
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    // Row 2 still parses; row 1 emits a MalformedSD17B2 diagnostic.
    assert_eq!(result.entries.len(), 1);
    let sorcerer = &result.entries[0];
    assert_eq!(sorcerer.spell_progression.len(), 1);
    assert_eq!(
        result
            .diagnostics
            .iter()
            .filter(|d| d.kind == SpellcastingClassDiagnosticKind::MalformedSD17B2)
            .count(),
        1,
        "exactly one diagnostic for the malformed CAST row, got: {:?}",
        result.diagnostics
    );
}

#[test]
fn treats_class_lines_for_non_spellcasting_class_names_as_out_of_scope() {
    // B-1 owns Fighter/Barbarian/Monk/Rogue/Ranger/Paladin. The
    // spellcasting parser must skip these class names WITHOUT producing
    // a diagnostic (skipping out-of-scope names is not a malformed
    // condition here).
    let input = "\
CLASS:Fighter\tHD:10
CLASS:Cleric\tHD:8
CLASS:Rogue\tHD:8
";
    let result = parse_spellcasting_class_entries("cr_classes.lst", input);
    assert_eq!(result.entries.len(), 1, "only Cleric is in scope");
    assert_eq!(result.entries[0].class_name, "Cleric");
    assert!(
        result.diagnostics.is_empty(),
        "out-of-scope class names must skip silently: {:?}",
        result.diagnostics
    );
}

// Both real-corpus tests below are opt-in via `PCGEN_CORPUS_ROOT` because
// the corpus is a separate checkout (~700MB) and not part of the codex
// repo. CI and the operator's local box should both set it. Mirrors the
// established pattern in tests/sd17_b1_martial_class.rs,
// tests/sd17_a_include_graph.rs, and tests/sd17_b5_equipment.rs — read the
// real corpus file at runtime via `PCGEN_CORPUS_ROOT`, not `include_str!`
// (a compile-time embed of an absolute local path breaks the build on any
// machine without that exact path, including every CI runner).
fn real_cr_classes_lst() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source = corpus_root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst");
    fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real cr_classes.lst at {}: {err}", source.display()))
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_pathfinder_core_rulebook_spellcasting_classes() {
    // End-to-end: parse the real `cr_classes.lst` and confirm every
    // spellcasting class lands deterministically with its expected
    // posture.
    let corpus = TestCorpus::new("pf1_cr_classes");
    corpus.write(
        "data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst",
        &real_cr_classes_lst(),
    );
    let result = parse_spellcasting_class_file(
        &corpus.path("data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst"),
    )
    .expect("read real cr_classes.lst");

    let names: Vec<&str> = result
        .entries
        .iter()
        .map(|entry| entry.class_name.as_str())
        .collect();
    for class_name in SPELLCASTING_CLASSES {
        assert!(
            names.contains(class_name),
            "expected spellcasting class {class_name} in parsed result; got {names:?}"
        );
    }
    // Wizard must surface nine school specializations.
    let wizard = result
        .entries
        .iter()
        .find(|entry| entry.class_name == "Wizard")
        .expect("Wizard parsed");
    assert_eq!(wizard.school_specializations.len(), 9);
    assert_eq!(wizard.casting_posture, Some(CastingPosture::Spellbook));
    // Bard/Sorcerer must surface as spontaneous; Cleric/Druid as prepared.
    for name in ["Bard", "Sorcerer"] {
        let entry = result
            .entries
            .iter()
            .find(|entry| entry.class_name == *name)
            .unwrap_or_else(|| panic!("{name} parsed"));
        assert_eq!(
            entry.casting_posture,
            Some(CastingPosture::Spontaneous),
            "{name}"
        );
    }
    for name in ["Cleric", "Druid"] {
        let entry = result
            .entries
            .iter()
            .find(|entry| entry.class_name == *name)
            .unwrap_or_else(|| panic!("{name} parsed"));
        assert_eq!(
            entry.casting_posture,
            Some(CastingPosture::Prepared),
            "{name}"
        );
    }
    // Bard must surface exactly 20 progression rows.
    let bard = result
        .entries
        .iter()
        .find(|entry| entry.class_name == "Bard")
        .unwrap();
    assert_eq!(bard.spell_progression.len(), 20);
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_within_linear_time_bound_on_real_corpus() {
    // Performance assertion: the parser is O(n) over input length.
    // A 256 KiB ceiling on the real 5.5 KiB pf1 cr_classes.lst gives
    // ~46x slack without making the test flaky.
    let corpus = TestCorpus::new("pf1_cr_classes_perf");
    corpus.write("cr_classes.lst", &real_cr_classes_lst());
    let path = corpus.path("cr_classes.lst");
    let bytes = fs::metadata(&path).expect("stat fixture").len();
    assert!(bytes > 4_000, "real cr_classes.lst should exceed 4 KiB");
    let started = std::time::Instant::now();
    let _result = parse_spellcasting_class_file(&path).expect("read real cr_classes.lst");
    let elapsed = started.elapsed();
    let ceiling_nanos = (bytes as u128) * 250; // 250 ns / byte headroom
    assert!(
        elapsed.as_nanos() < ceiling_nanos,
        "parser exceeded linear bound: bytes={bytes}, elapsed={elapsed:?}, ceiling_ns={ceiling_nanos}"
    );
}

#[test]
fn class_block_kind_classifier_distinguishes_progression_from_domain() {
    // The block name classifier is the helper that decides whether a
    // `###Block:` label opens a spell-progression curve, a domain
    // selection, or a school specialization shim. Keep this so future
    // slices can rely on the stable label matchers.
    assert!(ClassBlockKind::from_label("Level progression").is_progression_curve());
    assert!(ClassBlockKind::from_label("Spell Level Progression").is_progression_curve());
    assert!(ClassBlockKind::from_label("Domain Selections").is_domain_selections());
    assert!(ClassBlockKind::from_label("Proficiencies").is_other());
    assert_eq!(
        ClassBlockKind::from_label("Level progression").label(),
        "Level progression"
    );
}

#[test]
fn spell_level_row_carries_per_level_cast_and_known_vectors() {
    let row = SpellLevelRow {
        level: 5,
        cast_slots: vec![0, 6, 4],
        known_spells: vec![6, 4, 2],
        line_number: 12,
    };
    assert_eq!(row.level, 5);
    assert_eq!(row.cast_slots.len(), 3);
    assert_eq!(row.known_spells.len(), 3);
    assert_eq!(row.line_number, 12);
}

#[test]
fn gracefully_handles_empty_input() {
    let result = parse_spellcasting_class_entries("empty.lst", "");
    assert_eq!(result.entries.len(), 0);
    assert_eq!(result.diagnostics.len(), 0);
    assert_eq!(result.source_path, "empty.lst");
}

// SD-22 Epic 3 widening: Alchemist added to SPELLCASTING_CLASS_NAMES
// (module doc comment explains why: APG's CLASS:Alchemist line carries
// SPELLSTAT/MEMORIZE/SPELLBOOK tokens, the same posture-bearing shape as
// the five original CRB spellcasting classes).

fn real_apg_classes_lst() -> String {
    let corpus_root = PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    );
    let source =
        corpus_root.join("pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst");
    fs::read_to_string(&source)
        .unwrap_or_else(|err| panic!("failed to read real apg_classes.lst at {}: {err}", source.display()))
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_alchemist_record_from_apg_classes_lst() {
    // End-to-end: parse the real `apg_classes.lst` and confirm Alchemist
    // lands with spellbook-prepared posture (SPELLSTAT:INT MEMORIZE:YES
    // SPELLBOOK:YES on the real corpus line), now that SD-22 Epic 3 has
    // widened SPELLCASTING_CLASS_NAMES to include it.
    let corpus = TestCorpus::new("apg_classes");
    corpus.write(
        "data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst",
        &real_apg_classes_lst(),
    );
    let result = parse_spellcasting_class_file(&corpus.path(
        "data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst",
    ))
    .expect("read real apg_classes.lst");

    let alchemist = result
        .entries
        .iter()
        .find(|entry| entry.class_name == "Alchemist")
        .expect("Alchemist parsed from real apg_classes.lst");
    assert_eq!(alchemist.casting_posture, Some(CastingPosture::Spellbook));
    assert!(
        !alchemist.tokens.is_empty(),
        "Alchemist entry should carry its tab-delimited KEY:VAL tokens"
    );
}

// SD-22 Epic 3 widening (Inquisitor ingest cycle): the real
// `CLASS:Inquisitor` record in `apg_classes.lst` carries
// `SPELLSTAT:WIS MEMORIZE:NO` — the same spontaneous-divine posture as
// Sorcerer/Bard — so it belongs in `SPELLCASTING_CLASS_NAMES` rather than
// `MARTIAL_CLASS_NAMES`. Mirrors `parses_real_alchemist_record_from_apg_classes_lst`.
#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn parses_real_inquisitor_record_from_apg_classes_lst() {
    let corpus = TestCorpus::new("apg_classes_inquisitor");
    corpus.write(
        "data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst",
        &real_apg_classes_lst(),
    );
    let result = parse_spellcasting_class_file(&corpus.path(
        "data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst",
    ))
    .expect("read real apg_classes.lst");

    let inquisitor = result
        .entries
        .iter()
        .find(|entry| entry.class_name == "Inquisitor")
        .expect(
            "Inquisitor should be recognized from the real apg_classes.lst once \
             SPELLCASTING_CLASS_NAMES is widened to include it",
        );
    assert_eq!(inquisitor.casting_posture, Some(CastingPosture::Spontaneous));
    assert_eq!(inquisitor.spell_stat.as_deref(), Some("WIS"));
}
