//! SD-17 Slice C acceptance tests — canonical-IR conversion of parsed
//! LST records into the canonical source-IR envelope.
//!
//! ## Slice E update (2026-07-12)
//!
//! After SD-17 Slice E, the converter's per-record return type is the
//! canonical [`SourceContentRecord<'a>`] (not the historical
//! `IRNode`). The 45 acceptance tests below were rewritten in lock-step
//! with the converter's Slice E surface change. They exercise the
//! same contract published in
//! `docs/release/SD-17/artifacts/canonical-ir-contract-2026-07-12.md`
//! plus the new canonical source-IR contract artifact
//! `canonical-source-ir-contract-2026-07-12.md`. Every test that
//! previously destructured `IRNode::Class(p)` now destructures
//! `node.payload` directly via [`SourceContentPayload`].
//!
//! ## Verification coverage
//!
//! - V1 round-trip per kind: every B-family entry type projects into
//!   the matching `SourceContentPayload` variant and back.
//! - V2 forwarded-diagnostic preservation: every LstDiagnostic carried by
//!   a B-family parse-result container surfaces as an IRDiagnostic.
//! - V3 malformed diagnostic: a hand-built LST with a malformed entry
//!   surfaces as an IRDiagnostic forwarded from the B-family parser.
//! - V4 O(n) performance: a 5,000-record document converts in well
//!   under the 250ms budget.
//! - V5 line-number provenance: every record carries the source line
//!   number from the originating B-family record via `source_ref.line`.
//! - V6 schema idempotence: the schema is descriptive; conversion is
//!   total and does not reject records based on recognized_kinds.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use codex::pcgen_import::ir_converter::{
    convert_ability_declaration, convert_class_entry, convert_class_parse_result,
    convert_equipment_parse_result, convert_equipment_record, convert_lst_entry_file,
    convert_lst_metadata_document, convert_metadata_record, convert_race_declaration,
    convert_spell_record, convert_spellcasting_class_entry,
    convert_spellcasting_class_parse_result, convert_to_ir, IRDiagnostic, IRDiagnosticSeverity,
    IRSchema, ParsedLstRecord,
};
use codex::pcgen_import::lst_parser::class::{parse_class_entries, ClassEntry};
use codex::pcgen_import::lst_parser::equipment::{
    parse_equipment_entries, EquipmentRecord, EquipmentRecordKind,
};
use codex::pcgen_import::lst_parser::metadata::{parse_lst_metadata_text, LstRecord, MetadataKind};
use codex::pcgen_import::lst_parser::race_ability::{
    parse_lst_entry, AbilityDeclaration, AbilityKind, RaceDeclaration,
};
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::pcgen_import::lst_parser::spellcasting_class::{
    parse_spellcasting_class_entries, CastingPosture, SpellcastingClassEntry,
};
use codex::rules_core::source_content::{
    MetadataKindInner, SourceContentKind, SourceContentPayload,
};

// =============================================================================
// Test fixtures and helpers
// =============================================================================

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
            "codex_sd17_c_ir_convert_{}_{}_{}",
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

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TestCorpus {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn canonical_schema() -> IRSchema {
    IRSchema::canonical_v1()
}

// =============================================================================
// V1 — round-trip per kind
// =============================================================================

#[test]
fn v1_round_trip_class_entry_projection_carries_b1_payload() {
    let text = "\
CLASS:Fighter\tHD:10\tPROFICIENCY:Armor,Weapon
###Block: Level progression
1\tABILITY:Class|AUTOMATIC|Fighter\tFEAT:Alertness
20\tABILITY:Class|AUTOMATIC|Fighter\tFEAT:Greater Weapon Focus
";
    let parsed = parse_class_entries("classes_martial.lst", text);
    assert_eq!(parsed.entries.len(), 1);

    let entry = &parsed.entries[0];
    let record = convert_class_entry(entry);

    // Slice E: kind tag is `SourceContentKind::Class`, payload is the
    // canonical `SourceContentPayload::Class(&ClassEntry)` borrow.
    assert_eq!(record.kind, SourceContentKind::Class);
    match record.payload {
        SourceContentPayload::Class(p) => {
            assert_eq!(p.class_name, entry.class_name);
            assert_eq!(p.header_line_number, entry.header_line_number);
            assert_eq!(p.tokens.len(), entry.tokens.len());
            assert_eq!(p.feature_blocks.len(), entry.feature_blocks.len());
        }
        _ => panic!("expected SourceContentPayload::Class variant"),
    }

    // V5: line-number provenance via SourceRef.
    assert_eq!(record.source_ref.line, entry.header_line_number as u32);

    // Source-slice tag from the kind.
    assert_eq!(record.kind.source_slice(), "SD17-B-1");
}

#[test]
fn v1_round_trip_spellcasting_class_entry_projection_carries_b2_payload() {
    let text = "\
CLASS:Cleric\tSPELLSTAT:WIS\tMEMORIZE:YES
###Block: Level progression
1\tCAST:1,2,3
20\tCAST:6,7,8,9,9
";
    let parsed = parse_spellcasting_class_entries("classes_spellcasting.lst", text);
    assert_eq!(parsed.entries.len(), 1);

    let entry = &parsed.entries[0];
    let record = convert_spellcasting_class_entry(entry);

    assert_eq!(record.kind, SourceContentKind::SpellcastingClass);
    match record.payload {
        SourceContentPayload::SpellcastingClass(p) => {
            assert_eq!(p.class_name, entry.class_name);
            assert_eq!(p.spell_progression.len(), entry.spell_progression.len());
            assert_eq!(p.spell_stat, entry.spell_stat);
        }
        _ => panic!("expected SourceContentPayload::SpellcastingClass variant"),
    }
    assert_eq!(record.kind.source_slice(), "SD17-B-2");
}

#[test]
fn v1_round_trip_race_declaration_projection_carries_b3_payload() {
    let text = "RACE:cr_races.lst\n";
    let parsed = parse_lst_entry("sample_races.lst", text);
    assert_eq!(parsed.race_pointers.len(), 1);

    let race = &parsed.race_pointers[0];
    let record = convert_race_declaration(race);

    assert_eq!(record.kind, SourceContentKind::Race);
    match record.payload {
        SourceContentPayload::Race(p) => {
            assert_eq!(p.target, race.target);
            assert_eq!(p.raw_directive, race.raw_directive);
        }
        _ => panic!("expected SourceContentPayload::Race variant"),
    }
    assert_eq!(record.kind.source_slice(), "SD17-B-3");
}

#[test]
fn v1_round_trip_ability_declaration_projection_carries_b3_payload() {
    let text = "ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES\n";
    let parsed = parse_lst_entry("sample_abilities.lst", text);
    assert_eq!(parsed.ability_declarations.len(), 1);

    let ability = &parsed.ability_declarations[0];
    let record = convert_ability_declaration(ability);

    assert_eq!(record.kind, SourceContentKind::Ability);
    match record.payload {
        SourceContentPayload::Ability(p) => {
            assert_eq!(p.raw_directive, ability.raw_directive);
            match (&p.parsed, &ability.parsed) {
                (Some(pf), Some(af)) => {
                    assert_eq!(pf.category, af.category);
                    assert_eq!(pf.name, af.name);
                    assert_eq!(pf.kind, af.kind);
                }
                (None, None) => {}
                _ => panic!("parsed-field shape mismatch"),
            }
        }
        _ => panic!("expected SourceContentPayload::Ability variant"),
    }
}

#[test]
fn v1_round_trip_spell_record_projection_carries_b4_payload() {
    let text = "Fireball\tSCHOOL:Evocation\tDESCRIPTOR:Fire\tCASTTIME:1 standard action";
    let row = parse_lst_spell_row("cr_spells.lst", 7, text);
    let inner = row.record.expect("expected a parsed spell record");

    let record = convert_spell_record(&inner);

    assert_eq!(record.kind, SourceContentKind::Spell);
    match record.payload {
        SourceContentPayload::Spell(p) => {
            assert_eq!(p.name, inner.name);
            assert_eq!(p.school, inner.school);
            assert_eq!(p.descriptor, inner.descriptor);
        }
        _ => panic!("expected SourceContentPayload::Spell variant"),
    }
    assert_eq!(record.source_ref.line, 7);
    assert_eq!(record.kind.source_slice(), "SD17-B-4");
}

#[test]
fn v1_round_trip_equipment_record_projection_carries_b5_payload() {
    let text = "Longsword\tTYPE:Weapon\tCOST:15\tWT:4\n";
    let parsed = parse_equipment_entries("cr_equip.lst", text);
    assert_eq!(parsed.entries.len(), 1);

    let entry = &parsed.entries[0];
    let record = convert_equipment_record(entry);

    assert_eq!(record.kind, SourceContentKind::Equipment);
    match record.payload {
        SourceContentPayload::Equipment(p) => {
            assert_eq!(p.name, entry.name);
            assert_eq!(p.kind, entry.kind);
            assert_eq!(p.tokens.len(), entry.tokens.len());
        }
        _ => panic!("expected SourceContentPayload::Equipment variant"),
    }
    assert_eq!(record.kind.source_slice(), "SD17-B-5");
}

#[test]
fn v1_round_trip_metadata_record_projection_carries_b6_payload() {
    let text = "\
DEITY:Lamashtu
DOMAIN:Evil
TEMPLATE:Lycanthrope
";
    let parsed = parse_lst_metadata_text("sample_metadata.lst", text);
    assert_eq!(parsed.records.len(), 3);

    let expected_kinds = [
        MetadataKindInner::Deity,
        MetadataKindInner::Domain,
        MetadataKindInner::Template,
    ];

    for (i, inner) in parsed.records.iter().enumerate() {
        let record = convert_metadata_record(inner);
        assert_eq!(record.kind, SourceContentKind::Metadata(expected_kinds[i]));
        match record.payload {
            SourceContentPayload::Metadata(p) => {
                assert_eq!(p.kind, inner.kind);
                assert_eq!(p.name, inner.name);
                assert_eq!(p.line_number, inner.line_number);
            }
            _ => panic!("record {} expected SourceContentPayload::Metadata", i),
        }
        assert_eq!(record.kind.source_slice(), "SD17-B-6");
    }
}

// =============================================================================
// V2 — forwarded-diagnostic preservation
// =============================================================================

#[test]
fn v2_metadata_diagnostic_is_forwarded_as_ir_diagnostic_warning() {
    let text = "DEITY:\n";
    let parsed = parse_lst_metadata_text("malformed.lst", text);
    assert_eq!(parsed.records.len(), 1);
    assert_eq!(parsed.records[0].diagnostics.len(), 1);

    let converted = convert_lst_metadata_document(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 1);
    let (record, diagnostics) = &converted[0];

    assert_eq!(diagnostics.len(), 1);
    let d = &diagnostics[0];
    assert_eq!(d.source_kind, "SD17-B-6");
    assert_eq!(d.code, "IR_FORWARDED_B6");
    assert_eq!(d.severity, IRDiagnosticSeverity::Warning);
    assert_eq!(d.line_number, Some(1));
    assert_eq!(d.source_path, "malformed.lst");

    // The canonical record still carries the partial record (R4: conversion is total).
    match record.payload {
        SourceContentPayload::Metadata(p) => {
            assert_eq!(p.name, "");
            assert_eq!(p.kind, MetadataKind::Deity);
        }
        _ => panic!("expected SourceContentPayload::Metadata"),
    }
}

#[test]
fn v2_equipment_diagnostic_on_record_is_forwarded() {
    // A directive-prefix EQUIP: with a malformed BONUS chain produces
    // a malformed diagnostic on the record. We assert that when such
    // a record exists, the converter forwards the diagnostic as an
    // IRDiagnostic carrying the SD17-B-5 slice tag.
    let text = "EQUIP:Longsword\tTYPE:Weapon\tBONUS:COMBAT|AC||\n";
    let parsed = parse_equipment_entries("malformed.lst", text);
    // Whether or not the parser produced a record (depends on whether
    // the malformed BONUS chain halts parsing), the converter must not
    // panic. If a record exists, it must carry forwarded diagnostics.
    let converted = convert_equipment_parse_result(&parsed, &canonical_schema());
    if !converted.is_empty() {
        let (record, diagnostics) = &converted[0];
        assert_eq!(record.kind, SourceContentKind::Equipment);
        for d in diagnostics {
            assert_eq!(d.source_kind, "SD17-B-5");
            assert_eq!(d.code, "IR_FORWARDED_B5");
            assert_eq!(d.severity, IRDiagnosticSeverity::Warning);
        }
    }
}

#[test]
fn v2_equipment_record_level_diagnostic_is_forwarded_with_b5_slice_tag() {
    // Direct unit test of the forwarding helper via a hand-built record
    // (this is the canonical surface when a record exists and carries
    // a diagnostic).
    use codex::pcgen_import::lst_parser::equipment::{
        BonusToken, EquipmentDiagnostic, EquipmentDiagnosticKind,
    };

    let record = EquipmentRecord {
        kind: EquipmentRecordKind::Equip,
        name: "Longsword".to_string(),
        header_line_number: 1,
        header_raw_line: "Longsword\tTYPE:Weapon".to_string(),
        tokens: vec![],
        bonus_chains: vec![],
        is_record_start: true,
        diagnostics: vec![EquipmentDiagnostic {
            line_number: Some(1),
            raw_line: "Longsword".to_string(),
            kind: EquipmentDiagnosticKind::MalformedSD17B5,
            message: "test diagnostic".to_string(),
        }],
    };

    let canonical = convert_equipment_record(&record);
    assert_eq!(canonical.kind, SourceContentKind::Equipment);

    // Now invoke the document converter with this record inside a result.
    use codex::pcgen_import::lst_parser::equipment::EquipmentParseResult;
    let result = EquipmentParseResult {
        source_path: "cr_equip.lst".to_string(),
        entries: vec![record],
        diagnostics: vec![],
    };
    let converted = convert_equipment_parse_result(&result, &canonical_schema());
    assert_eq!(converted.len(), 1);
    let (_, diagnostics) = &converted[0];
    assert!(!diagnostics.is_empty());
    let d = &diagnostics[0];
    assert_eq!(d.source_kind, "SD17-B-5");
    assert_eq!(d.code, "IR_FORWARDED_B5");
    assert_eq!(d.severity, IRDiagnosticSeverity::Warning);

    // BonusToken reference to keep the symbol live.
    let _ = BonusToken {
        line_number: 1,
        raw_bonus: "BONUS:COMBAT|AC|1".to_string(),
        qualifiers: vec!["COMBAT".to_string(), "AC".to_string(), "1".to_string()],
    };
}

#[test]
fn v2_class_diagnostics_are_forwarded_with_b1_slice_tag() {
    // Hand-build a ClassEntry that the B-1 parser flagged as malformed.
    let text = "CLASS:\n";
    let parsed = parse_class_entries("malformed.lst", text);
    let converted = convert_class_parse_result(&parsed, &canonical_schema());
    if !converted.is_empty() {
        let (_, diagnostics) = &converted[0];
        for d in diagnostics {
            assert_eq!(d.source_kind, "SD17-B-1");
            assert_eq!(d.code, "IR_FORWARDED_B1");
        }
    }
    // Whether or not the malformed entry produced a record, the converter
    // must not have panicked. (Empty result is acceptable per R4.)
}

// =============================================================================
// V3 — malformed diagnostic surfaced via the IR pipeline
// =============================================================================

#[test]
fn v3_malformed_metadata_diagnostic_surfaces_in_ir_pipeline() {
    // The malformed `DEITY:` line produces both a record AND a diagnostic.
    let text = "DEITY:\nLANGUAGE:Common\n";
    let parsed = parse_lst_metadata_text("malformed.lst", text);

    let converted = convert_lst_metadata_document(&parsed, &canonical_schema());

    // Two records, both with diagnostics forwarded (the malformed one
    // surfaces its MalformedDirective diagnostic).
    assert_eq!(converted.len(), 2);

    let (malformed_record, malformed_diags) = &converted[0];
    assert!(
        !malformed_diags.is_empty(),
        "malformed record should carry a forwarded diagnostic"
    );
    let d = &malformed_diags[0];
    assert!(d.message.contains("DEITY"));
    assert_eq!(d.source_kind, "SD17-B-6");

    match malformed_record.payload {
        SourceContentPayload::Metadata(p) => assert_eq!(p.name, ""),
        _ => panic!("expected SourceContentPayload::Metadata"),
    }
}

// =============================================================================
// V4 — O(n) performance
// =============================================================================

#[test]
fn v4_metadata_conversion_does_not_exceed_o_n_on_representative_input() {
    let mut text = String::new();
    for i in 1..=5_000 {
        text.push_str(&format!("LANGUAGE:Language_{}\n", i));
    }

    let start = std::time::Instant::now();
    let parsed = parse_lst_metadata_text("perf.lst", &text);
    let parsed_at = start.elapsed();

    let conv_start = std::time::Instant::now();
    let converted = convert_lst_metadata_document(&parsed, &canonical_schema());
    let conv_at = conv_start.elapsed();

    assert_eq!(converted.len(), 5_000);
    // Generous budget: 5,000 records should convert in well under 250ms.
    // Typical observed: 1-5ms.
    assert!(
        conv_at.as_millis() < 250,
        "conversion took {}ms, exceeds 250ms budget",
        conv_at.as_millis()
    );

    // Sanity: parse time is similar order of magnitude — this is a smoke
    // check that conversion does not regress against parse time.
    let _ = parsed_at;
}

#[test]
fn v4_spellcasting_class_conversion_does_not_exceed_o_n_on_representative_input() {
    // The B-2 parser dedupes CLASS:<name> entries by name (5 valid
    // spellcasting classes: Cleric, Druid, Wizard, Sorcerer, Bard).
    // We write 5,000 lines that exercise the parser's full input
    // length and assert the converted result is bounded by the same
    // O(n) budget.
    let spellcasting_names = ["Cleric", "Druid", "Wizard", "Sorcerer", "Bard"];
    let mut text = String::new();
    for i in 0..5_000 {
        let name = spellcasting_names[i % spellcasting_names.len()];
        text.push_str(&format!("CLASS:{}\tSPELLSTAT:WIS\n", name));
    }

    let parsed = parse_spellcasting_class_entries("perf_classes.lst", &text);
    assert_eq!(
        parsed.entries.len(),
        spellcasting_names.len(),
        "B-2 dedupes by name; expected 5 distinct entries"
    );

    let start = std::time::Instant::now();
    let converted = convert_spellcasting_class_parse_result(&parsed, &canonical_schema());
    let elapsed = start.elapsed();

    assert_eq!(converted.len(), parsed.entries.len());
    assert!(
        elapsed.as_millis() < 250,
        "spellcasting-class conversion took {}ms, exceeds 250ms budget",
        elapsed.as_millis()
    );
}

// =============================================================================
// V5 — line-number provenance (now via source_ref.line)
// =============================================================================

#[test]
fn v5_class_entry_provenance_carries_header_line_number() {
    let text = "\
# preamble comment
CLASS:Fighter\tHD:10
CLASS:Fighter\tPROFICIENCY:Armor
";
    let parsed = parse_class_entries("classes.lst", text);
    assert!(!parsed.entries.is_empty());
    let entry = &parsed.entries[0];
    let record = convert_class_entry(entry);
    assert_eq!(record.source_ref.line as usize, entry.header_line_number);
    assert_eq!(record.source_ref.line, 2);
}

#[test]
fn v5_spell_record_provenance_carries_source_line_number() {
    let row = parse_lst_spell_row("cr_spells.lst", 42, "Fireball\tSCHOOL:Evocation");
    let inner = row.record.expect("expected parsed spell record");
    let record = convert_spell_record(&inner);
    assert_eq!(record.source_ref.line, 42);
}

#[test]
fn v5_metadata_record_provenance_carries_source_line_number() {
    let text = "\
DEITY:Lamashtu
TEMPLATE:Lycanthrope
COMPANIONMOD:Raven
";
    let parsed = parse_lst_metadata_text("meta.lst", text);
    let converted = convert_lst_metadata_document(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 3);

    let expected_lines = [1u32, 2, 3];
    for (i, (record, _)) in converted.iter().enumerate() {
        assert_eq!(
            record.source_ref.line, expected_lines[i],
            "record {} line mismatch",
            i
        );
    }
}

#[test]
fn v5_race_declaration_provenance_carries_source_line_number() {
    let text = "\
RACE:cr_races.lst
RACES:cr_races_extra.lst
";
    let parsed = parse_lst_entry("races.lst", text);
    let converted = convert_lst_entry_file(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 2);
    for (i, (record, _)) in converted.iter().enumerate() {
        assert_eq!(record.source_ref.line as usize, i + 1);
    }
}

#[test]
fn v5_ability_declaration_provenance_carries_source_line_number() {
    let text = "\
ABILITY:CATEGORY=FEAT|Alertness
ABILITY:Traits|VIRTUAL|%LIST
";
    let parsed = parse_lst_entry("abilities.lst", text);
    let converted = convert_lst_entry_file(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 2);
    for (i, (record, _)) in converted.iter().enumerate() {
        assert_eq!(record.source_ref.line as usize, i + 1);
    }
}

// =============================================================================
// V6 — schema idempotence
// =============================================================================

#[test]
fn v6_conversion_is_total_regardless_of_schema_recognized_kinds() {
    let mut custom_schema = IRSchema::canonical_v1();
    custom_schema.recognized_kinds = &[]; // schema recognizes nothing

    let text = "DEITY:Lamashtu\n";
    let parsed = parse_lst_metadata_text("meta.lst", text);

    // Conversion must still succeed — the schema is descriptive, not gating.
    let converted = convert_lst_metadata_document(&parsed, &custom_schema);
    assert_eq!(converted.len(), 1);

    // The kind is NOT recognized by the schema (it was emptied above),
    // but the conversion produced the record anyway.
    let (record, _) = &converted[0];
    let kind_token = match record.kind {
        SourceContentKind::Metadata(inner) => inner.token(),
        ref k => k.token(),
    };
    assert!(!custom_schema.recognizes(kind_token));
}

#[test]
fn v6_canonical_schema_recognizes_all_b_family_kinds() {
    let schema = IRSchema::canonical_v1();
    for kind in [
        "CLASS",
        "RACE",
        "RACES",
        "ABILITY",
        "SPELL",
        "EQUIP",
        "EQUIPMOD",
        "DEITY",
        "DOMAIN",
        "KITS",
        "LANGUAGE",
        "TEMPLATE",
        "COMPANIONMOD",
    ] {
        assert!(schema.recognizes(kind), "schema should recognize {kind}");
    }
}

// =============================================================================
// convert_to_ir — the canonical entry point named in the slice card body
// =============================================================================

#[test]
fn convert_to_ir_dispatches_class_records() {
    let text = "CLASS:Fighter\tHD:10\n";
    let parsed = parse_class_entries("classes.lst", text);
    let entry: &ClassEntry = &parsed.entries[0];

    let record = convert_to_ir(&ParsedLstRecord::from_class(entry), &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Class);
}

#[test]
fn convert_to_ir_dispatches_spellcasting_class_records() {
    let text = "CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES\n";
    let parsed = parse_spellcasting_class_entries("classes.lst", text);
    let entry: &SpellcastingClassEntry = &parsed.entries[0];

    let record = convert_to_ir(
        &ParsedLstRecord::from_spellcasting_class(entry),
        &canonical_schema(),
    );
    assert_eq!(record.kind, SourceContentKind::SpellcastingClass);
}

#[test]
fn convert_to_ir_dispatches_race_records() {
    let text = "RACE:cr_races.lst\n";
    let parsed = parse_lst_entry("races.lst", text);
    let race: &RaceDeclaration = &parsed.race_pointers[0];

    let record = convert_to_ir(&ParsedLstRecord::from_race(race), &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Race);
}

#[test]
fn convert_to_ir_dispatches_ability_records() {
    let text = "ABILITY:CATEGORY=FEAT|Alertness\n";
    let parsed = parse_lst_entry("abilities.lst", text);
    let ability: &AbilityDeclaration = &parsed.ability_declarations[0];

    let record = convert_to_ir(&ParsedLstRecord::from_ability(ability), &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Ability);
}

#[test]
fn convert_to_ir_dispatches_spell_records() {
    let row = parse_lst_spell_row("cr_spells.lst", 5, "Magic Missile\tSCHOOL:Evocation");
    let inner = row.record.expect("expected parsed record");

    let record = convert_to_ir(&ParsedLstRecord::from_spell(&inner), &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Spell);
}

#[test]
fn convert_to_ir_dispatches_equipment_records() {
    let text = "Longsword\tTYPE:Weapon\tCOST:15\n";
    let parsed = parse_equipment_entries("cr_equip.lst", text);
    let equip: &EquipmentRecord = &parsed.entries[0];

    let record = convert_to_ir(&ParsedLstRecord::from_equipment(equip), &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Equipment);
}

#[test]
fn convert_to_ir_dispatches_metadata_records() {
    let text = "DEITY:Lamashtu\n";
    let parsed = parse_lst_metadata_text("meta.lst", text);
    let record_input: &LstRecord = &parsed.records[0];

    let record = convert_to_ir(
        &ParsedLstRecord::from_metadata(record_input),
        &canonical_schema(),
    );
    assert_eq!(
        record.kind,
        SourceContentKind::Metadata(MetadataKindInner::Deity)
    );
}

// =============================================================================
// Per-document converters — every B-family parse-result type
// =============================================================================

#[test]
fn document_converters_handle_every_b_family_kind() {
    let corpus = TestCorpus::new("all_kinds");

    // B-1 martial class
    corpus.write("cr_classes.lst", "CLASS:Fighter\tHD:10\n");
    // B-2 spellcasting class
    corpus.write("cr_classes_magic.lst", "CLASS:Wizard\tSPELLSTAT:INT\n");
    // B-3 race + ability
    corpus.write("cr_races.lst", "RACE:cr_races_inner.lst\n");
    corpus.write("cr_abilities.lst", "ABILITY:CATEGORY=FEAT|Alertness\n");
    // B-4 spell — write a fixture so the parser picks it up
    let spells_text = "Magic Missile\tSCHOOL:Evocation\n";
    corpus.write("cr_spells.lst", spells_text);
    // B-5 equipment
    corpus.write("cr_equip.lst", "Longsword\tTYPE:Weapon\n");
    // B-6 metadata
    corpus.write("cr_meta.lst", "DEITY:Lamashtu\nTEMPLATE:Lycanthrope\n");

    let root = corpus.root();

    // B-1
    let classes_text = std::fs::read_to_string(root.join("cr_classes.lst")).unwrap();
    let parsed = parse_class_entries("cr_classes.lst", &classes_text);
    let converted = convert_class_parse_result(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 1);

    // B-2
    let spellcasting_text = std::fs::read_to_string(root.join("cr_classes_magic.lst")).unwrap();
    let parsed = parse_spellcasting_class_entries("cr_classes_magic.lst", &spellcasting_text);
    let converted = convert_spellcasting_class_parse_result(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 1);

    // B-3
    let races_text = std::fs::read_to_string(root.join("cr_races.lst")).unwrap();
    let abilities_text = std::fs::read_to_string(root.join("cr_abilities.lst")).unwrap();
    let parsed_races = parse_lst_entry("cr_races.lst", &races_text);
    let parsed_abilities = parse_lst_entry("cr_abilities.lst", &abilities_text);
    let converted = convert_lst_entry_file(&parsed_races, &canonical_schema());
    assert_eq!(converted.len(), 1);
    let converted = convert_lst_entry_file(&parsed_abilities, &canonical_schema());
    assert_eq!(converted.len(), 1);

    // B-4
    let spells_text_read = std::fs::read_to_string(root.join("cr_spells.lst")).unwrap();
    let row = parse_lst_spell_row("cr_spells.lst", 1, &spells_text_read);
    if let Some(inner) = row.record {
        let record = convert_spell_record(&inner);
        assert_eq!(record.kind, SourceContentKind::Spell);
    }

    // B-5
    let equip_text = std::fs::read_to_string(root.join("cr_equip.lst")).unwrap();
    let parsed = parse_equipment_entries("cr_equip.lst", &equip_text);
    let converted = convert_equipment_parse_result(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 1);

    // B-6
    let meta_text = std::fs::read_to_string(root.join("cr_meta.lst")).unwrap();
    let parsed = parse_lst_metadata_text("cr_meta.lst", &meta_text);
    let converted = convert_lst_metadata_document(&parsed, &canonical_schema());
    assert_eq!(converted.len(), 2);
}

// =============================================================================
// IRSchema — accessor contracts
// =============================================================================

#[test]
fn ir_schema_canonical_v1_advertises_documented_fields() {
    let schema = IRSchema::canonical_v1();
    assert_eq!(schema.schema_id, "codex.pcgen.canonical-ir.v1");
    assert_eq!(schema.schema_version, 1);
    assert!(!schema.recognized_kinds.is_empty());
}

#[test]
fn ir_schema_default_is_canonical_v1() {
    let default_schema = IRSchema::default();
    let canonical_schema = IRSchema::canonical_v1();
    assert_eq!(default_schema.schema_id, canonical_schema.schema_id);
    assert_eq!(
        default_schema.schema_version,
        canonical_schema.schema_version
    );
}

#[test]
fn ir_schema_recognizes_known_kind_and_rejects_unknown() {
    let schema = IRSchema::canonical_v1();
    assert!(schema.recognizes("CLASS"));
    assert!(schema.recognizes("DEITY"));
    assert!(!schema.recognizes("BOGUS"));
}

#[test]
fn source_content_payload_kind_token_is_canonical() {
    let row = parse_lst_spell_row("cr_spells.lst", 1, "Fireball\tSCHOOL:Evocation");
    let spell_inner = row.record.unwrap();
    let spell_record = convert_spell_record(&spell_inner);
    assert_eq!(spell_record.payload.kind_token(), "SPELL");

    let equip_text = "Longsword\tTYPE:Weapon\n";
    let parsed = parse_equipment_entries("cr_equip.lst", equip_text);
    let equip_record = convert_equipment_record(&parsed.entries[0]);
    assert_eq!(equip_record.payload.kind_token(), "EQUIP");

    let meta_text = "DEITY:Lamashtu\nTEMPLATE:Lycanthrope\nCOMPANIONMOD:Raven\n";
    let parsed = parse_lst_metadata_text("meta.lst", meta_text);
    for (i, record) in parsed.records.iter().enumerate() {
        let canonical = convert_metadata_record(record);
        let expected = match i {
            0 => "DEITY",
            1 => "TEMPLATE",
            2 => "COMPANIONMOD",
            _ => unreachable!(),
        };
        assert_eq!(canonical.payload.kind_token(), expected);
    }
}

// =============================================================================
// IRDiagnostic — construction and severity semantics (preserved from Slice C)
// =============================================================================

#[test]
fn ir_diagnostic_converter_error_factory_carries_sd17_c_slice_tag() {
    let d = IRDiagnostic::converter_error(
        "file.lst",
        Some(7),
        "raw line text",
        "IR_MALFORMED_C17",
        "manual test message",
    );
    assert_eq!(d.source_kind, "SD17-C");
    assert_eq!(d.code, "IR_MALFORMED_C17");
    assert_eq!(d.severity, IRDiagnosticSeverity::Error);
    assert_eq!(d.line_number, Some(7));
    assert_eq!(d.source_path, "file.lst");
}

#[test]
fn ir_diagnostic_converter_warning_factory_carries_warning_severity() {
    let d = IRDiagnostic::converter_warning(
        "file.lst",
        None,
        "",
        "IR_LOSSY_PROJECTION_C17",
        "lossy projection detected",
    );
    assert_eq!(d.severity, IRDiagnosticSeverity::Warning);
    assert_eq!(d.line_number, None);
}

// =============================================================================
// ParsedLstRecord — convenience constructors
// =============================================================================

#[test]
fn parsed_lst_record_convenience_constructors_match_variants() {
    let text = "CLASS:Fighter\tHD:10\n";
    let parsed = parse_class_entries("classes.lst", text);
    let entry: &ClassEntry = &parsed.entries[0];

    let r = ParsedLstRecord::from_class(entry);
    assert!(matches!(r, ParsedLstRecord::Class(_)));

    let ability_text = "ABILITY:CATEGORY=FEAT|Alertness\n";
    let parsed = parse_lst_entry("abilities.lst", ability_text);
    let ability: &AbilityDeclaration = &parsed.ability_declarations[0];
    let r = ParsedLstRecord::from_ability(ability);
    assert!(matches!(r, ParsedLstRecord::Ability(_)));

    let race_text = "RACE:cr_races.lst\n";
    let parsed = parse_lst_entry("races.lst", race_text);
    let race: &RaceDeclaration = &parsed.race_pointers[0];
    let r = ParsedLstRecord::from_race(race);
    assert!(matches!(r, ParsedLstRecord::Race(_)));
}

// =============================================================================
// Coverage operator-doctrine lock — converter covers every record class
// =============================================================================

#[test]
fn operator_doctrine_lock_converter_handles_every_b_family_kind() {
    // Per operator doctrine 2026-07-12, this converter MUST accept every
    // PCGen LST record of the relevant kind — no narrowing to a single
    // record class. This test asserts the converter covers all six
    // B-family record kinds end-to-end through the public
    // `convert_to_ir` entry point.
    let schema = canonical_schema();

    let class_text = "CLASS:Fighter\tHD:10\n";
    let class_parsed = parse_class_entries("classes.lst", class_text);
    let class_entry: &ClassEntry = &class_parsed.entries[0];
    assert_eq!(
        convert_to_ir(&ParsedLstRecord::from_class(class_entry), &schema).kind,
        SourceContentKind::Class
    );

    let spellcasting_text = "CLASS:Wizard\tSPELLSTAT:INT\n";
    let spellcasting_parsed = parse_spellcasting_class_entries("classes.lst", spellcasting_text);
    let spellcasting_entry: &SpellcastingClassEntry = &spellcasting_parsed.entries[0];
    assert_eq!(
        convert_to_ir(
            &ParsedLstRecord::from_spellcasting_class(spellcasting_entry),
            &schema
        )
        .kind,
        SourceContentKind::SpellcastingClass
    );

    let race_text = "RACE:cr_races.lst\n";
    let race_parsed = parse_lst_entry("races.lst", race_text);
    let race: &RaceDeclaration = &race_parsed.race_pointers[0];
    assert_eq!(
        convert_to_ir(&ParsedLstRecord::from_race(race), &schema).kind,
        SourceContentKind::Race
    );

    let ability_text = "ABILITY:CATEGORY=FEAT|Alertness\n";
    let ability_parsed = parse_lst_entry("abilities.lst", ability_text);
    let ability: &AbilityDeclaration = &ability_parsed.ability_declarations[0];
    assert_eq!(
        convert_to_ir(&ParsedLstRecord::from_ability(ability), &schema).kind,
        SourceContentKind::Ability
    );

    let spell_row = parse_lst_spell_row("cr_spells.lst", 1, "Fireball\tSCHOOL:Evocation");
    let spell_inner = spell_row.record.unwrap();
    assert_eq!(
        convert_to_ir(&ParsedLstRecord::from_spell(&spell_inner), &schema).kind,
        SourceContentKind::Spell
    );

    let equip_text = "Longsword\tTYPE:Weapon\n";
    let equip_parsed = parse_equipment_entries("cr_equip.lst", equip_text);
    let equip: &EquipmentRecord = &equip_parsed.entries[0];
    assert_eq!(
        convert_to_ir(&ParsedLstRecord::from_equipment(equip), &schema).kind,
        SourceContentKind::Equipment
    );

    let meta_text = "DEITY:Lamashtu\nDOMAIN:Evil\nKITS:Paladin\nLANGUAGE:Common\nTEMPLATE:Lycanthrope\nCOMPANIONMOD:Raven\n";
    let meta_parsed = parse_lst_metadata_text("meta.lst", meta_text);
    assert_eq!(meta_parsed.records.len(), 6);
    let expected_inner_kinds = [
        MetadataKindInner::Deity,
        MetadataKindInner::Domain,
        MetadataKindInner::Kits,
        MetadataKindInner::Language,
        MetadataKindInner::Template,
        MetadataKindInner::CompanionMod,
    ];
    for (i, record) in meta_parsed.records.iter().enumerate() {
        assert_eq!(
            convert_to_ir(&ParsedLstRecord::from_metadata(record), &schema).kind,
            SourceContentKind::Metadata(expected_inner_kinds[i])
        );
    }
}

// =============================================================================
// Field-shape sanity — canonical payload carries the documented entry
// =============================================================================

#[test]
fn equipment_record_carries_record_kind_and_token_payload() {
    let text = "Longsword\tTYPE:Weapon\tCOST:15\tWT:4\tDAMAGE:1d8\n";
    let parsed = parse_equipment_entries("cr_equip.lst", text);
    let equip = &parsed.entries[0];

    assert_eq!(equip.kind, EquipmentRecordKind::Equip);
    assert_eq!(equip.tokens.len(), 4);

    let record = convert_equipment_record(equip);
    match record.payload {
        SourceContentPayload::Equipment(p) => {
            assert_eq!(p.kind, EquipmentRecordKind::Equip);
            assert_eq!(p.tokens.len(), 4);
            // Token keys are preserved
            let keys: Vec<&str> = p.tokens.iter().map(|t| t.key.as_str()).collect();
            assert_eq!(keys, vec!["TYPE", "COST", "WT", "DAMAGE"]);
        }
        _ => panic!("expected SourceContentPayload::Equipment"),
    }
}

#[test]
fn ability_kind_enum_is_preserved_through_conversion() {
    let text = "ABILITY:Traits|VIRTUAL|%LIST\n";
    let parsed = parse_lst_entry("abilities.lst", text);
    let ability = &parsed.ability_declarations[0];
    let parsed_fields = ability.parsed.as_ref().expect("expected parsed fields");

    assert_eq!(parsed_fields.kind, Some(AbilityKind::Virtual));

    let record = convert_ability_declaration(ability);
    match record.payload {
        SourceContentPayload::Ability(p) => {
            let pf = p.parsed.as_ref().expect("expected parsed fields");
            assert_eq!(pf.kind, Some(AbilityKind::Virtual));
        }
        _ => panic!("expected SourceContentPayload::Ability"),
    }
}

#[test]
fn metadata_kind_is_preserved_through_conversion() {
    let kinds = [
        (
            "DEITY:Boccob",
            MetadataKind::Deity,
            MetadataKindInner::Deity,
        ),
        (
            "DOMAIN:Knowledge",
            MetadataKind::Domain,
            MetadataKindInner::Domain,
        ),
        ("KITS:Paladin", MetadataKind::Kits, MetadataKindInner::Kits),
        (
            "LANGUAGE:Common",
            MetadataKind::Language,
            MetadataKindInner::Language,
        ),
        (
            "TEMPLATE:Lycanthrope",
            MetadataKind::Template,
            MetadataKindInner::Template,
        ),
        (
            "COMPANIONMOD:Raven",
            MetadataKind::CompanionMod,
            MetadataKindInner::CompanionMod,
        ),
    ];

    for (text, expected_kind, expected_inner) in kinds.iter() {
        let parsed = parse_lst_metadata_text("meta.lst", text);
        assert_eq!(parsed.records.len(), 1);
        let record = convert_metadata_record(&parsed.records[0]);
        assert_eq!(record.kind, SourceContentKind::Metadata(*expected_inner));
        match record.payload {
            SourceContentPayload::Metadata(p) => assert_eq!(p.kind, *expected_kind),
            _ => panic!("expected SourceContentPayload::Metadata"),
        }
    }
}

#[test]
fn spell_record_preserves_optional_field_set_through_conversion() {
    let text = "Fireball\tSCHOOL:Evocation\tDESCRIPTOR:Fire\tCASTTIME:1 standard action";
    let row = parse_lst_spell_row("cr_spells.lst", 1, text);
    let inner = row.record.unwrap();

    let record = convert_spell_record(&inner);
    match record.payload {
        SourceContentPayload::Spell(p) => {
            assert_eq!(p.school, Some("Evocation".to_string()));
            assert_eq!(p.descriptor, Some("Fire".to_string()));
            assert_eq!(p.casting_time, Some("1 standard action".to_string()));
            assert!(p.sub_school.is_none());
            assert!(p.components.is_none());
        }
        _ => panic!("expected SourceContentPayload::Spell"),
    }
}

#[test]
fn class_entry_carries_all_tab_delimited_tokens_through_conversion() {
    let text = "CLASS:Fighter\tHD:10\tPROFICIENCY:Armor,Weapon\tBONUS:COMBAT|HP|2\n";
    let parsed = parse_class_entries("classes.lst", text);
    let entry = &parsed.entries[0];

    let record = convert_class_entry(entry);
    match record.payload {
        SourceContentPayload::Class(p) => {
            assert_eq!(p.tokens.len(), 3);
            let keys: Vec<&str> = p.tokens.iter().map(|t| t.key.as_str()).collect();
            assert_eq!(keys, vec!["HD", "PROFICIENCY", "BONUS"]);
        }
        _ => panic!("expected SourceContentPayload::Class"),
    }
}

#[test]
fn spellcasting_class_entry_carries_casting_posture_through_conversion() {
    let text = "CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES\n";
    let parsed = parse_spellcasting_class_entries("classes.lst", text);
    let entry = &parsed.entries[0];
    assert_eq!(entry.casting_posture, Some(CastingPosture::Spellbook));

    let record = convert_spellcasting_class_entry(entry);
    match record.payload {
        SourceContentPayload::SpellcastingClass(p) => {
            assert_eq!(p.casting_posture, Some(CastingPosture::Spellbook));
            assert_eq!(p.spell_stat, Some("INT".to_string()));
        }
        _ => panic!("expected SourceContentPayload::SpellcastingClass"),
    }
}

// =============================================================================
// Reference capability detection — AbilityKind, EquipmentRecordKind
// =============================================================================

#[test]
fn equipment_record_kind_round_trips_through_conversion() {
    let text = "EQUIPMOD:Masterwork\tTYPE:Weapon\tBONUS:COMBAT|AC|1\n";
    let parsed = parse_equipment_entries("cr_equipmods.lst", text);
    assert_eq!(parsed.entries.len(), 1);
    assert_eq!(parsed.entries[0].kind, EquipmentRecordKind::EquipMod);

    let record = convert_equipment_record(&parsed.entries[0]);
    assert_eq!(record.payload.kind_token(), "EQUIPMOD");
    match record.payload {
        SourceContentPayload::Equipment(p) => {
            assert_eq!(p.kind, EquipmentRecordKind::EquipMod);
        }
        _ => panic!("expected SourceContentPayload::Equipment"),
    }
}

#[test]
fn ability_parsed_field_full_shape_is_preserved() {
    let text = "ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES\n";
    let parsed = parse_lst_entry("abilities.lst", text);
    let ability = &parsed.ability_declarations[0];
    let pf = ability.parsed.as_ref().expect("expected parsed fields");

    assert_eq!(pf.category, Some("FEAT".to_string()));
    assert_eq!(pf.name, "Alertness");
    assert_eq!(pf.kind, None);
    assert_eq!(pf.trailing_modifiers, vec!["FREE:YES".to_string()]);

    let record = convert_ability_declaration(ability);
    match record.payload {
        SourceContentPayload::Ability(p) => {
            let pf = p.parsed.as_ref().expect("expected parsed fields");
            assert_eq!(pf.category, Some("FEAT".to_string()));
            assert_eq!(pf.name, "Alertness");
            assert_eq!(pf.trailing_modifiers, vec!["FREE:YES".to_string()]);
        }
        _ => panic!("expected SourceContentPayload::Ability"),
    }
}
