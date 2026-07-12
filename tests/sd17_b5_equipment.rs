//! SD-17 Slice B-5 acceptance tests for LST equipment and
//! equipment-modifier parsing.
//!
//! Covers the slice card verification criteria:
//!   - Every recognized `EQUIP:` / `EQUIPMOD:` directive-prefix row
//!     produces a record carrying its source line number and raw line
//!     text.
//!   - The corpus-typical row form (name in column 0, tab-delimited
//!     `KEY:VAL` tags in subsequent columns) produces a record under the
//!     kind inferred from the source path (`*_equipmods.lst` → EquipMod,
//!     anything else → Equip).
//!   - Malformed rows (no value after the directive) emit a
//!     `MalformedSD17B5` diagnostic.
//!   - Orphan continuation rows (no preceding record) emit an
//!     `OrphanContinuationRow` diagnostic.
//!   - Empty `###Block:` markers emit a `MalformedBlockMarker`
//!     diagnostic.
//!   - The bonus-chain parser flattens deeply nested pipe-delimited
//!     qualifiers into a list (no recursion, no stack overflow).
//!   - Parse runs in linear time on a synthetic large file.

use codex::pcgen_import::lst_parser::{
    EquipmentDiagnosticKind, EquipmentRecordKind, parse_equipment_entries, parse_equipment_file,
};

#[test]
fn recognizes_every_equipment_kind_declared_in_the_slice() {
    assert_eq!(
        EquipmentRecordKind::from_token("EQUIP"),
        Some(EquipmentRecordKind::Equip)
    );
    assert_eq!(
        EquipmentRecordKind::from_token("EQUIPMOD"),
        Some(EquipmentRecordKind::EquipMod)
    );
    assert_eq!(EquipmentRecordKind::Equip.token(), "EQUIP");
    assert_eq!(EquipmentRecordKind::EquipMod.token(), "EQUIPMOD");
}

#[test]
fn rejects_unknown_equipment_kinds() {
    assert!(EquipmentRecordKind::from_token("BOGUS").is_none());
    // lower-cased form is not the same as the canonical token
    assert!(EquipmentRecordKind::from_token("equip").is_none());
    assert!(EquipmentRecordKind::from_token("Equipmod").is_none());
}

#[test]
fn parses_directive_prefix_equip_rows_with_line_numbers_preserved() {
    let text = "\
EQUIP:Padded Armor\tKEY:Padded Armor (Base)\tTYPE:Armor.Light.Suit\tCOST:5\tWT:10
EQUIP:Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light.Suit\tCOST:10\tWT:15
";
    let result = parse_equipment_entries("test.lst", text);

    assert_eq!(result.entries.len(), 2);
    assert!(
        result
            .entries
            .iter()
            .all(|r| r.kind == EquipmentRecordKind::Equip)
    );
    assert!(result.entries.iter().all(|r| r.is_record_start));

    let padded = &result.entries[0];
    assert_eq!(padded.name, "Padded Armor");
    assert_eq!(padded.header_line_number, 1);
    assert_eq!(
        padded.header_raw_line,
        "EQUIP:Padded Armor\tKEY:Padded Armor (Base)\tTYPE:Armor.Light.Suit\tCOST:5\tWT:10"
    );
    assert!(padded.diagnostics.is_empty());
    // Token count: KEY, TYPE, COST, WT (4 tokens; the directive-prefix and name
    // are not counted as tokens).
    assert_eq!(padded.tokens.len(), 4);
    let key_token = &padded.tokens[0];
    assert_eq!(key_token.key, "KEY");
    assert_eq!(key_token.value, "Padded Armor (Base)");
    assert_eq!(key_token.line_number, 1);

    let leather = &result.entries[1];
    assert_eq!(leather.name, "Leather Armor");
    assert_eq!(leather.header_line_number, 2);
}

#[test]
fn parses_directive_prefix_equipmod_rows_with_line_numbers_preserved() {
    let text = "\
EQUIPMOD:Material ~ Steel\tKEY:Material ~ Steel\tNAMEOPT:NOTHING\tTYPE:BaseMaterial\tCOST:0\tBONUS:ITEMCOST|TYPE.WEAPON|50
EQUIPMOD:Masterwork\tKEY:Special Quality ~ Masterwork ~ Weapon\tTYPE:MasterworkQuality.Weapon\tBONUS:WEAPON|TOHIT|1|TYPE=Enhancement
";
    let result = parse_equipment_entries("test.lst", text);

    assert_eq!(result.entries.len(), 2);
    assert!(
        result
            .entries
            .iter()
            .all(|r| r.kind == EquipmentRecordKind::EquipMod)
    );

    let steel = &result.entries[0];
    assert_eq!(steel.name, "Material ~ Steel");
    assert_eq!(steel.header_line_number, 1);
    assert_eq!(steel.bonus_chains.len(), 1);
    assert_eq!(
        steel.bonus_chains[0].qualifiers,
        vec!["ITEMCOST", "TYPE.WEAPON", "50"]
    );
    assert_eq!(steel.bonus_chains[0].line_number, 1);

    let masterwork = &result.entries[1];
    assert_eq!(masterwork.bonus_chains.len(), 1);
    assert_eq!(
        masterwork.bonus_chains[0].qualifiers,
        vec!["WEAPON", "TOHIT", "1", "TYPE=Enhancement"]
    );
}

#[test]
fn parses_corpus_typical_equip_rows_with_inferred_kind() {
    // The source path contains `equipmods` → records must be EquipMod.
    // No directive prefix → corpus-typical row form.
    let text = "\
Cloth\tKEY:Material ~ Cloth\tNAMEOPT:NOTHING\tTYPE:BaseMaterial\tCOST:0\tBONUS:ITEMCOST|TYPE.CLOTH|0
Steel\tKEY:Material ~ Steel\tNAMEOPT:NOTHING\tTYPE:BaseMaterial\tCOST:0\tITYPE:Metal
";
    let result = parse_equipment_entries("/data/.../equipmods_test.lst", text);

    assert_eq!(result.entries.len(), 2);
    assert!(
        result
            .entries
            .iter()
            .all(|r| r.kind == EquipmentRecordKind::EquipMod)
    );

    let cloth = &result.entries[0];
    assert_eq!(cloth.name, "Cloth");
    assert_eq!(cloth.header_line_number, 1);
    assert_eq!(cloth.tokens.len(), 4); // KEY, NAMEOPT, TYPE, COST
    assert_eq!(cloth.bonus_chains.len(), 1);

    let steel = &result.entries[1];
    assert_eq!(steel.name, "Steel");
    assert_eq!(steel.header_line_number, 2);
    // KEY, NAMEOPT, TYPE, COST, ITYPE = 5 tokens; Steel carries no BONUS:
    // clause so bonus_chains is empty.
    assert_eq!(steel.tokens.len(), 5);
    assert_eq!(steel.bonus_chains.len(), 0);
}

#[test]
fn parses_corpus_typical_equip_rows_without_equipmods_substring() {
    let text = "\
Arrow\tKEY:Arrow (Base)\tTYPE:Ammunition.Resizable.Arrow\tCOST:0.05\tWT:0.15\tSOURCEPAGE:p.145\tVISIBLE:NO
Bolt\tKEY:Bolt (Base)\tTYPE:Ammunition.Resizable.Bolt\tCOST:0.1\tWT:0.1\tSOURCEPAGE:p.145\tVISIBLE:NO
";
    let result = parse_equipment_entries("/data/.../cr_equip_arms_armor.lst", text);

    assert_eq!(result.entries.len(), 2);
    assert!(
        result
            .entries
            .iter()
            .all(|r| r.kind == EquipmentRecordKind::Equip)
    );
    assert!(result.entries.iter().all(|r| r.is_record_start));
}

#[test]
fn strips_copy_suffix_from_equipment_record_name() {
    let text = "Bolt (Base).COPY=Bolt, Crossbow\tKEY:Bolt (Crossbow)\tTYPE:Ammunition\n";
    let result = parse_equipment_entries("/data/.../cr_equip_arms_armor.lst", text);
    assert_eq!(result.entries.len(), 1);
    assert_eq!(result.entries[0].name, "Bolt (Base)");
}

#[test]
fn emits_malformed_diagnostic_when_directive_has_no_value() {
    let text = "EQUIP:\tKEY:MalformedTest\n";
    let result = parse_equipment_entries("test.lst", text);
    assert!(result.entries.is_empty());
    assert_eq!(result.diagnostics.len(), 1);
    let diag = &result.diagnostics[0];
    assert_eq!(diag.kind, EquipmentDiagnosticKind::MalformedSD17B5);
    assert_eq!(diag.line_number, Some(1));
    assert!(diag.message.contains("EQUIP"));
}

#[test]
fn emits_malformed_diagnostic_for_empty_equipmod_directive() {
    let text = "EQUIPMOD:\tKEY:MalformedEquipmodTest\n";
    let result = parse_equipment_entries("test.lst", text);
    assert!(result.entries.is_empty());
    assert_eq!(result.diagnostics.len(), 1);
    let diag = &result.diagnostics[0];
    assert_eq!(diag.kind, EquipmentDiagnosticKind::MalformedSD17B5);
    assert!(diag.message.contains("EQUIPMOD"));
}

#[test]
fn emits_malformed_block_marker_diagnostic_for_empty_label() {
    let text = "\
EQUIP:Padded Armor\tKEY:Padded\tTYPE:Armor

###Block:
EQUIP:Test\tKEY:Test\tTYPE:Test
";
    let result = parse_equipment_entries("test.lst", text);
    let empty_marker_diag = result
        .diagnostics
        .iter()
        .find(|d| d.kind == EquipmentDiagnosticKind::MalformedBlockMarker)
        .expect("empty block marker must emit a diagnostic");
    assert_eq!(empty_marker_diag.line_number, Some(3));
}

#[test]
fn emits_orphan_continuation_row_diagnostic() {
    let text = "\
BONUS:COMBAT|AC|99|TYPE=OrphanTest
EQUIP:Padded\tKEY:Padded\tTYPE:Armor
";
    let result = parse_equipment_entries("test.lst", text);
    let orphan_diag = result
        .diagnostics
        .iter()
        .find(|d| d.kind == EquipmentDiagnosticKind::OrphanContinuationRow)
        .expect("orphan continuation row must emit a diagnostic");
    assert_eq!(orphan_diag.line_number, Some(1));
    // The orphan's BONUS: token is captured as a diagnostic, not as part of
    // any record. The padded record still exists.
    assert!(result.entries.iter().any(|e| e.name == "Padded"));
}

#[test]
fn parses_deeply_nested_bonus_chain_without_stack_overflow() {
    // 200 pipe-delimited qualifiers — recursive parsing would overflow the
    // stack on this exact input. The flat parser must terminate.
    let mut raw = String::from("EQUIP:Test\tKEY:Test\tTYPE:Test\tBONUS:");
    for i in 0..200 {
        raw.push_str(&format!("QUALIFIER{i}|"));
    }
    raw.push_str("END\n");

    let result = parse_equipment_entries("test.lst", &raw);
    assert_eq!(result.entries.len(), 1);
    let record = &result.entries[0];
    assert_eq!(record.bonus_chains.len(), 1);
    assert_eq!(record.bonus_chains[0].qualifiers.len(), 201);
    assert_eq!(record.bonus_chains[0].qualifiers[0], "QUALIFIER0");
    assert_eq!(record.bonus_chains[0].qualifiers[200], "END");
}

#[test]
fn parses_real_world_bonus_with_prev_qualifier_chain() {
    // The exact chain shape from cr_equip_arms_armor.lst: BONUS:COMBAT|AC|1|
    // TYPE=Armor|PREVAREQ:DisableArmorBonus,0
    let text = "Padded Armor\tKEY:Padded\tTYPE:Armor\tBONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0\n";
    let result = parse_equipment_entries("/data/.../cr_equip_arms_armor.lst", text);
    assert_eq!(result.entries.len(), 1);
    let record = &result.entries[0];
    assert_eq!(record.bonus_chains.len(), 1);
    assert_eq!(
        record.bonus_chains[0].qualifiers,
        vec![
            "COMBAT",
            "AC",
            "1",
            "TYPE=Armor",
            "PREVAREQ:DisableArmorBonus,0"
        ]
    );
    assert_eq!(record.bonus_chains[0].line_number, 1);
}

#[test]
fn multiple_bonus_tokens_on_same_record_are_all_captured() {
    // cr_equip_arms_armor.lst rows frequently carry multiple BONUS: tags.
    let text = "Padded Armor\tKEY:Padded\tTYPE:Armor\tBONUS:COMBAT|AC|1|TYPE=Armor\tBONUS:VAR|ArmorCheckPenalty|0|TYPE=BaseArmor\n";
    let result = parse_equipment_entries("/data/.../cr_equip_arms_armor.lst", text);
    let record = &result.entries[0];
    assert_eq!(record.bonus_chains.len(), 2);
    assert_eq!(
        record.bonus_chains[0].qualifiers,
        vec!["COMBAT", "AC", "1", "TYPE=Armor"]
    );
    assert_eq!(
        record.bonus_chains[1].qualifiers,
        vec!["VAR", "ArmorCheckPenalty", "0", "TYPE=BaseArmor"]
    );
}

#[test]
fn ignores_blank_lines_and_comment_lines() {
    let text = "\
# leading comment

EQUIP:Padded Armor\tKEY:Padded\tTYPE:Armor
   # indented comment

EQUIP:Leather Armor\tKEY:Leather\tTYPE:Armor
";
    let result = parse_equipment_entries("test.lst", text);
    assert_eq!(result.entries.len(), 2);
    assert_eq!(result.entries[0].header_line_number, 3);
    assert_eq!(result.entries[1].header_line_number, 6);
}

#[test]
fn parses_a_fixture_from_disk_with_line_numbers_preserved() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lst/equipment_minimal.lst");
    let result = parse_equipment_file(&path).expect("disk fixture parses deterministically");

    // Directive-prefix EQUIP rows must appear at their known line numbers.
    // Fixture layout: EQUIP:Padded Armor on line 13, EQUIP:Leather Armor on
    // line 14 (after 10 lines of header comments + the SOURCELONG row +
    // a blank line + the `###Block:` marker on line 12).
    assert!(result.entries.iter().any(|e| e.name == "Padded Armor"
        && e.header_line_number == 13
        && e.kind == EquipmentRecordKind::Equip));
    assert!(result.entries.iter().any(|e| e.name == "Leather Armor"
        && e.header_line_number == 14
        && e.kind == EquipmentRecordKind::Equip));

    // Corpus-typical EQUIP rows must appear at their known line numbers.
    // Arrow is unique, so it must appear at its exact line. Padded Armor
    // is shared with the directive-prefix section above; the parser merges
    // by (kind, name) so only one record exists for it, and its header
    // line number is the FIRST occurrence (line 13, the directive-prefix
    // row). The corpus-typical row at line 19 appends additional tokens
    // and bonus_chains to the merged record.
    assert!(result.entries.iter().any(|e| e.name == "Arrow"
        && e.header_line_number == 17
        && e.kind == EquipmentRecordKind::Equip));
    let merged_padded = result
        .entries
        .iter()
        .find(|e| e.name == "Padded Armor" && e.kind == EquipmentRecordKind::Equip)
        .expect("merged Padded Armor record must exist");
    assert_eq!(merged_padded.header_line_number, 13);
    // The merged record carries BOTH the KEY from the directive-prefix row
    // AND the ACCHECK/MAXDEX/SOURCEPAGE/BONUS tokens from the corpus-typical
    // row, proving the continuation semantics worked.
    assert!(
        merged_padded.bonus_chains.len() >= 2,
        "merged Padded Armor must carry at least 2 BONUS chains from both rows"
    );
    assert!(merged_padded.tokens.iter().any(|t| t.key == "ACCHECK"));
    assert!(merged_padded.tokens.iter().any(|t| t.key == "MAXDEX"));

    // Every recognized record carries a positive line number and raw text.
    for entry in &result.entries {
        assert!(entry.header_line_number >= 1);
        assert!(!entry.header_raw_line.is_empty());
        assert!(entry.is_record_start);
    }
}

#[test]
fn parses_equipmods_fixture_from_disk_with_kind_inferred_from_path() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lst/equipmods_minimal.lst");
    let result = parse_equipment_file(&path).expect("disk fixture parses deterministically");

    // Source path contains `equipmods` → all records must be EquipMod.
    assert!(!result.entries.is_empty());
    assert!(
        result
            .entries
            .iter()
            .all(|e| e.kind == EquipmentRecordKind::EquipMod)
    );

    // Directive-prefix EQUIPMOD rows must appear at their known line numbers.
    // Fixture layout: EQUIPMOD:Material ~ Steel on line 16, EQUIPMOD:Masterwork
    // on line 17 (after the SOURCELONG row on line 13 + a blank line + the
    // `###Block:` marker on line 15).
    assert!(
        result
            .entries
            .iter()
            .any(|e| e.name == "Material ~ Steel" && e.header_line_number == 16)
    );
    assert!(
        result
            .entries
            .iter()
            .any(|e| e.name == "Masterwork" && e.header_line_number == 17)
    );

    // Corpus-typical EquipMod rows must appear at their known line numbers.
    assert!(
        result
            .entries
            .iter()
            .any(|e| e.name == "Cloth" && e.header_line_number == 20)
    );
    assert!(
        result
            .entries
            .iter()
            .any(|e| e.name == "Steel" && e.header_line_number == 21)
    );

    // Both `Material ~ Steel` rows (directive-prefix + corpus-typical) must
    // carry BONUS: chains. The corpus-typical Cloth and Steel rows also
    // carry BONUS: tokens (visible in the fixture source).
    // Both `Material ~ Steel` rows (directive-prefix + corpus-typical) must
    // carry BONUS: chains. The corpus-typical Cloth row carries a BONUS:
    // token; the corpus-typical Steel row does not.
    for entry in &result.entries {
        if entry.name == "Material ~ Steel" || entry.name == "Cloth" {
            assert!(
                !entry.bonus_chains.is_empty(),
                "expected BONUS chain on record '{}', got none",
                entry.name
            );
        }
    }
}

#[test]
fn parse_runs_in_linear_time_on_a_synthetic_large_file() {
    let n = 5_000usize;
    let mut text = String::with_capacity(n * 30);
    for i in 0..n {
        text.push_str(&format!(
            "EQUIP:Synthetic Item {i}\tKEY:Synthetic {i}\tTYPE:Test\tCOST:1\n"
        ));
    }

    let start = std::time::Instant::now();
    let result = parse_equipment_entries("synthetic.lst", &text);
    let elapsed = start.elapsed();

    assert_eq!(result.entries.len(), n);
    // Loose bound: 5k records should parse in well under 2 seconds on any
    // reasonable host. The tighter intent is O(n) (linear, not quadratic).
    assert!(
        elapsed.as_secs() < 2,
        "5k equipment records should parse in well under 2s, took {elapsed:?}"
    );
}

#[test]
fn records_by_kind_groups_records_correctly() {
    // The source path contains `equipmods`, so corpus-typical rows default
    // to EquipMod. Directive-prefix rows keep their explicit kind. The
    // first EQUIP: row in this fixture therefore lands in Equip; the EQUIPMOD:
    // row lands in EquipMod; the second EQUIP: row is also in Equip.
    let text = "\
EQUIP:Padded\tKEY:Padded\tTYPE:Armor
EQUIPMOD:Steel\tKEY:Steel\tTYPE:Material
EQUIP:Leather\tKEY:Leather\tTYPE:Armor
";
    let result = parse_equipment_entries("equipmods_test.lst", text);
    let grouped = result.records_by_kind();
    assert_eq!(result.entries.len(), 3);
    assert_eq!(
        grouped.get(&EquipmentRecordKind::Equip).map(|v| v.len()),
        Some(2),
        "the two EQUIP:-prefixed rows must land in the Equip bucket"
    );
    assert_eq!(
        grouped.get(&EquipmentRecordKind::EquipMod).map(|v| v.len()),
        Some(1),
        "the EQUIPMOD:-prefixed row must land in the EquipMod bucket"
    );
}

#[test]
fn real_corpus_cr_equip_arms_armor_parses_with_line_numbers_preserved() {
    // The core_rulebook cr_equip_arms_armor.lst is the canonical
    // Pathfinder equipment corpus file. The parser must round-trip line
    // numbers and produce records for both Ammunition and Armor blocks.
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    let path = std::path::PathBuf::from(corpus_root)
        .join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst");
    if !path.is_file() {
        eprintln!(
            "skipping: real cr_equip_arms_armor.lst not at {}",
            path.display()
        );
        return;
    }
    let result = parse_equipment_file(&path).expect("real corpus file parses");
    assert!(
        result.entries.len() >= 30,
        "expected at least 30 equipment records in cr_equip_arms_armor.lst, got {}",
        result.entries.len()
    );
    assert!(
        result
            .entries
            .iter()
            .all(|e| e.kind == EquipmentRecordKind::Equip)
    );
    // Spot-check: Arrow (Base) must appear with a positive line number and
    // a non-empty raw line.
    let arrow = result
        .entries
        .iter()
        .find(|e| e.name == "Arrow (Base)")
        .expect("Arrow (Base) must be present in the corpus");
    assert!(arrow.header_line_number >= 1);
    assert!(!arrow.header_raw_line.is_empty());
}

#[test]
fn real_corpus_cr_equipmods_parses_with_kind_inferred_from_path() {
    let corpus_root = std::env::var("PCGEN_CORPUS_ROOT")
        .unwrap_or_else(|_| "/home/ubuntu/workspace/repos/pcgen/data".to_string());
    let path = std::path::PathBuf::from(corpus_root)
        .join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst");
    if !path.is_file() {
        eprintln!("skipping: real cr_equipmods.lst not at {}", path.display());
        return;
    }
    let result = parse_equipment_file(&path).expect("real corpus file parses");
    assert!(
        result.entries.len() >= 20,
        "expected at least 20 equipment-modifier records in cr_equipmods.lst, got {}",
        result.entries.len()
    );
    assert!(
        result
            .entries
            .iter()
            .all(|e| e.kind == EquipmentRecordKind::EquipMod)
    );
    // Spot-check: at least one Cloth / Leather / Steel / Wood material modifier
    // must be present.
    let material_names = ["Cloth", "Leather", "Steel", "Wood"];
    for material in &material_names {
        assert!(
            result.entries.iter().any(|e| e.name == *material),
            "expected material '{material}' in cr_equipmods.lst parse"
        );
    }
}
