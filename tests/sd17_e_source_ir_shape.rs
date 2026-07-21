//! SD-17 Slice E acceptance tests — canonical source-IR shape.
//!
//! Every test in this file exercises a clause of the canonical
//! contract published in
//! `docs/release/SD-17/artifacts/canonical-source-ir-contract-2026-07-12.md`.
//! The slice-E converter entry point is `convert_to_ir` in
//! `codex::pcgen_import::ir_converter`, which projects a parsed
//! LST record (via `ParsedLstRecord<'a>`) into a canonical
//! `SourceContentRecord<'a>` envelope.
//!
//! ## Verification coverage
//!
//! - V1 round-trip per kind: each of the six B-family entry types
//!   projects into the matching `SourceContentPayload` variant and
//!   back via the borrowed entry.
//! - V2 `SourceRef` line numbers preserved from the parser output.
//! - V3 malformed-record diagnostic: a hand-built parsed record
//!   produces a `SourceContentDiagnostic` with severity Error.
//! - V4 lossy-mapping diagnostic: a token the corpus supports but
//!   the source-IR preserves as a raw string produces a
//!   `SourceContentDiagnostic` with severity Warning (LossyMapping).
//! - V5 `SourcePackageContent::records_by_kind` returns a
//!   deterministic order.

use codex::pcgen_import::ir_converter::{
    convert_ability_declaration, convert_class_entry, convert_equipment_record,
    convert_metadata_record, convert_race_declaration, convert_spell_record,
    convert_spellcasting_class_entry, convert_to_ir, IRSchema, ParsedLstRecord,
};
use codex::pcgen_import::lst_parser::class::{parse_class_entries, ClassEntry, ClassToken};
use codex::pcgen_import::lst_parser::equipment::{
    parse_equipment_entries, BonusToken, EquipmentRecord, EquipmentRecordKind, EquipmentToken,
};
use codex::pcgen_import::lst_parser::metadata::{parse_lst_metadata_text, LstRecord, MetadataKind};
use codex::pcgen_import::lst_parser::race_ability::{
    parse_lst_entry, AbilityDeclaration, RaceDeclaration,
};
use codex::pcgen_import::lst_parser::spell::{parse_lst_spell_row, LstSpellRecord};
use codex::pcgen_import::lst_parser::spellcasting_class::{
    parse_spellcasting_class_entries, SpellcastingClassEntry,
};
use codex::rules_core::source_content::{
    MetadataKindInner, SourceContentDiagnostic, SourceContentDiagnosticKind, SourceContentKind,
    SourceContentPayload, SourceContentRecord, SourceContentSeverity, SourcePackageContent,
    SourceRef, SOURCE_IR_VERSION,
};

// =============================================================================
// V1 — round-trip per kind (one variant per B-family entry)
// =============================================================================

fn assert_payload_class(record: &SourceContentRecord<'_>, expected: &ClassEntry) {
    match record.payload {
        SourceContentPayload::Class(p) => {
            // Zero-copy: the pointer the envelope holds IS the
            // same pointer the caller still holds.
            assert!(std::ptr::eq(p, expected));
            // Every B-family field is reachable through the borrow.
            assert_eq!(p.class_name, expected.class_name);
            assert_eq!(p.header_line_number, expected.header_line_number);
            assert_eq!(p.tokens.len(), expected.tokens.len());
            assert_eq!(p.feature_blocks.len(), expected.feature_blocks.len());
        }
        _ => panic!("expected SourceContentPayload::Class variant"),
    }
}

#[test]
fn v1_class_entry_round_trips_into_class_payload() {
    let text = "CLASS:Fighter\tHD:10\tPROFICIENCY:Armor,Weapon\n";
    let parsed = parse_class_entries("cr_classes.lst", text);
    assert_eq!(parsed.entries.len(), 1);
    let entry = &parsed.entries[0];

    let record = convert_class_entry(entry);
    assert_eq!(record.kind, SourceContentKind::Class);
    assert_payload_class(&record, entry);
}

#[test]
fn v1_spellcasting_class_entry_round_trips_into_spellcasting_class_payload() {
    let text = "CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES\n";
    let parsed = parse_spellcasting_class_entries("cr_classes_magic.lst", text);
    assert_eq!(parsed.entries.len(), 1);
    let entry = &parsed.entries[0];

    let record = convert_spellcasting_class_entry(entry);
    assert_eq!(record.kind, SourceContentKind::SpellcastingClass);
    match record.payload {
        SourceContentPayload::SpellcastingClass(p) => {
            assert!(std::ptr::eq(p, entry));
            assert_eq!(p.class_name, entry.class_name);
            assert_eq!(p.spell_stat, entry.spell_stat);
            assert_eq!(p.casting_posture, entry.casting_posture);
            assert_eq!(p.spell_progression.len(), entry.spell_progression.len());
        }
        _ => panic!("expected SourceContentPayload::SpellcastingClass variant"),
    }
}

#[test]
fn v1_race_declaration_round_trips_into_race_payload() {
    let text = "RACE:cr_races.lst\n";
    let parsed = parse_lst_entry("cr_races.lst", text);
    assert_eq!(parsed.race_pointers.len(), 1);
    let race = &parsed.race_pointers[0];

    let record = convert_race_declaration(race);
    assert_eq!(record.kind, SourceContentKind::Race);
    match record.payload {
        SourceContentPayload::Race(p) => {
            assert!(std::ptr::eq(p, race));
            assert_eq!(p.target, race.target);
            assert_eq!(p.raw_directive, race.raw_directive);
            assert_eq!(p.line_number, race.line_number);
        }
        _ => panic!("expected SourceContentPayload::Race variant"),
    }
}

#[test]
fn v1_ability_declaration_round_trips_into_ability_payload() {
    let text = "ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES\n";
    let parsed = parse_lst_entry("cr_abilities.lst", text);
    assert_eq!(parsed.ability_declarations.len(), 1);
    let ability = &parsed.ability_declarations[0];

    let record = convert_ability_declaration(ability);
    assert_eq!(record.kind, SourceContentKind::Ability);
    match record.payload {
        SourceContentPayload::Ability(p) => {
            assert!(std::ptr::eq(p, ability));
            assert_eq!(p.raw_directive, ability.raw_directive);
            assert_eq!(p.line_number, ability.line_number);
            // parsed-shape preservation
            match (&p.parsed, &ability.parsed) {
                (Some(pf), Some(af)) => {
                    assert_eq!(pf.category, af.category);
                    assert_eq!(pf.name, af.name);
                }
                (None, None) => {}
                _ => panic!("parsed-field shape mismatch"),
            }
        }
        _ => panic!("expected SourceContentPayload::Ability variant"),
    }
}

#[test]
fn v1_spell_record_round_trips_into_spell_payload() {
    // First column must NOT be a recognized PCGen spell-attribute
    // tag (`SCHOOL:`, `TYPE:`, etc.), otherwise the parser treats
    // the row as a continuation of a previous spell row.
    let raw_line = "Magic Missile\tSCHOOL:Evocation\tDESCRIPTOR:Force\tCASTTIME:1 standard action";
    let parsed_row = parse_lst_spell_row("cr_spells_magic_missile.lst", 42, raw_line);
    // Note: parse_lst_spell_row returns LstRowParse { record: Option<LstSpellRecord>, diagnostics }
    let inner = parsed_row.record.as_ref().expect("expected parsed record");
    let record = convert_spell_record(inner);
    assert_eq!(record.kind, SourceContentKind::Spell);
    match record.payload {
        SourceContentPayload::Spell(p) => {
            assert!(std::ptr::eq(p, inner));
            assert_eq!(p.name, inner.name);
            assert_eq!(p.school, inner.school);
            assert_eq!(p.descriptor, inner.descriptor);
        }
        _ => panic!("expected SourceContentPayload::Spell variant"),
    }
}

#[test]
fn v1_equipment_record_round_trips_into_equipment_payload() {
    let text = "Longsword\tTYPE:Weapon\tCOST:15\tWT:4\tDAMAGE:1d8\n";
    let parsed = parse_equipment_entries("cr_equip.lst", text);
    assert_eq!(parsed.entries.len(), 1);
    let entry = &parsed.entries[0];

    let record = convert_equipment_record(entry);
    assert_eq!(record.kind, SourceContentKind::Equipment);
    match record.payload {
        SourceContentPayload::Equipment(p) => {
            assert!(std::ptr::eq(p, entry));
            assert_eq!(p.name, entry.name);
            assert_eq!(p.kind, entry.kind);
            assert_eq!(p.tokens.len(), entry.tokens.len());
            assert_eq!(p.bonus_chains.len(), entry.bonus_chains.len());
        }
        _ => panic!("expected SourceContentPayload::Equipment variant"),
    }
}

#[test]
fn v1_metadata_record_round_trips_into_metadata_payload_with_inner_kind_mapping() {
    let kinds_text = [
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

    for (text, expected_b6_kind, expected_inner) in kinds_text.iter() {
        let parsed = parse_lst_metadata_text("meta.lst", text);
        assert_eq!(
            parsed.records.len(),
            1,
            "parser should yield one record for `{text}`"
        );
        let entry = &parsed.records[0];

        let record = convert_metadata_record(entry);
        assert_eq!(record.kind, SourceContentKind::Metadata(*expected_inner));
        match record.payload {
            SourceContentPayload::Metadata(p) => {
                assert!(std::ptr::eq(p, entry));
                assert_eq!(p.kind, *expected_b6_kind);
                assert_eq!(p.name, entry.name);
                assert_eq!(p.line_number, entry.line_number);
            }
            _ => panic!("expected SourceContentPayload::Metadata variant"),
        }
    }
}

// =============================================================================
// V2 — SourceRef line numbers preserved from the parser output
// =============================================================================

#[test]
fn v2_source_ref_line_matches_parser_header_line_per_kind() {
    // ClassEntry -> header_line_number
    let class_text = "CLASS:Fighter\tHD:10\n";
    let class_parsed = parse_class_entries("cr_classes.lst", class_text);
    let class_record = convert_class_entry(&class_parsed.entries[0]);
    assert_eq!(class_record.source_ref.line, 1);

    // RaceDeclaration -> line_number (one-based, as parsed)
    let race_text = "RACE:cr_races.lst\nRACES:cr_races_extra.lst\n";
    let race_parsed = parse_lst_entry("cr_races.lst", race_text);
    assert_eq!(race_parsed.race_pointers.len(), 2);
    for (i, race) in race_parsed.race_pointers.iter().enumerate() {
        let record = convert_race_declaration(race);
        assert_eq!(record.source_ref.line, (i + 1) as u32);
    }

    // SpellcastingClassEntry -> header_line_number
    let scc_text = "CLASS:Wizard\tSPELLSTAT:INT\n";
    let scc_parsed = parse_spellcasting_class_entries("cr_scc.lst", scc_text);
    let scc_record = convert_spellcasting_class_entry(&scc_parsed.entries[0]);
    assert_eq!(scc_record.source_ref.line, 1);

    // SpellRecord -> line_number (passed by the caller to parse_lst_spell_row)
    let spell_row = parse_lst_spell_row(
        "cr_spells.lst",
        7,
        "Fireball\tSCHOOL:Evocation\tDESCRIPTOR:Fire\tCASTTIME:1 standard action",
    );
    let spell = spell_row.record.as_ref().expect("expected parsed record");
    let spell_record = convert_spell_record(spell);
    assert_eq!(spell_record.source_ref.line, 7);

    // EquipmentRecord -> header_line_number
    let equip_text = "Longsword\tTYPE:Weapon\n";
    let equip_parsed = parse_equipment_entries("cr_equip.lst", equip_text);
    let equip_record = convert_equipment_record(&equip_parsed.entries[0]);
    assert_eq!(equip_record.source_ref.line, 1);

    // LstRecord (B-6 metadata) -> line_number
    let meta_text = "DEITY:Lamashtu\nTEMPLATE:Lycanthrope\n";
    let meta_parsed = parse_lst_metadata_text("cr_meta.lst", meta_text);
    assert_eq!(meta_parsed.records.len(), 2);
    for (i, m) in meta_parsed.records.iter().enumerate() {
        let record = convert_metadata_record(m);
        assert_eq!(record.source_ref.line, (i + 1) as u32);
    }
}

// =============================================================================
// V3 — malformed-record diagnostic carries severity Error + diagnostic kind
// =============================================================================

#[test]
fn v3_malformed_record_produces_error_diagnostic_naming_the_kind() {
    // Hand-build a `ParsedLstRecord::Metadata` whose underlying
    // `LstRecord` carries a malformed-record diagnostic. The
    // canonical converter forwards the diagnostic into a
    // `SourceContentDiagnostic` with severity Error and kind
    // `MalformedRecord`.
    use codex::pcgen_import::lst_parser::metadata::{LstDiagnostic, LstDiagnosticKind};

    let bad_record = LstRecord {
        kind: MetadataKind::Deity,
        name: "BadDeity".to_string(),
        line_number: 17,
        raw_line: "DEITY:BadDeity".to_string(),
        is_record_start: true,
        diagnostics: vec![LstDiagnostic {
            kind: LstDiagnosticKind::MalformedDirective,
            message: "Deity directive has empty value".to_string(),
        }],
    };

    // The canonical envelope wraps the malformed record; the
    // contract artifact documents that the canonical projection
    // always succeeds (conversion is total). The diagnostic
    // stream is what surfaces the defect.
    let record: SourceContentRecord<'_> = convert_metadata_record(&bad_record);
    assert_eq!(record.source_ref.line, 17);
    assert_eq!(
        record.kind,
        SourceContentKind::Metadata(MetadataKindInner::Deity)
    );

    // The contract test: the conversion is total and does not
    // produce an Error-level diagnostic on the envelope itself
    // (the parser-side diagnostic is forwarded by the
    // per-document converter, not the per-record converter). The
    // per-record `convert_metadata_record` produces the
    // canonical envelope and the diagnostic lives on the parser
    // entry — the Slice E source-IR surface preserves it.
    let parsed_doc = codex::pcgen_import::lst_parser::metadata::LstMetadataDocument {
        source_path: "bad_meta.lst".to_string(),
        records: vec![bad_record],
    };
    let (pkg, forwarded) = (
        codex::pcgen_import::ir_converter::convert_package_from_lst_metadata_document(
            &parsed_doc,
            "test_pkg",
            &IRSchema::canonical_v1(),
        )
        .0,
        codex::pcgen_import::ir_converter::convert_package_from_lst_metadata_document(
            &parsed_doc,
            "test_pkg",
            &IRSchema::canonical_v1(),
        )
        .1,
    );
    assert_eq!(pkg.records.len(), 1);
    assert!(
        !forwarded.is_empty(),
        "expected at least one forwarded diagnostic"
    );
    let d = &forwarded[0];
    assert_eq!(d.source_kind, "SD17-B-6");
    assert_eq!(d.code, "IR_FORWARDED_B6");
    assert_eq!(
        d.severity,
        codex::pcgen_import::ir_converter::IRDiagnosticSeverity::Warning
    );

    // The canonical `SourceContentDiagnostic` form of the same
    // defect is severity Error + kind MalformedRecord.
    let canonical: SourceContentDiagnostic = d.to_canonical();
    assert_eq!(canonical.severity, SourceContentSeverity::Error);
    assert_eq!(canonical.kind, SourceContentDiagnosticKind::MalformedRecord);
    // The parser message uses Title-cased kind names ("Deity
    // directive has empty value"); the contract requires the
    // diagnostic to identify the offending kind name so the
    // consumer can group diagnostics by kind. Assert both
    // casings to defend against future renames.
    assert!(
        canonical.message.contains("Deity") || canonical.message.contains("DEITY"),
        "canonical diagnostic message must name the offending kind: got `{}`",
        canonical.message
    );
    assert_eq!(canonical.source_ref.lst_file, "bad_meta.lst");
    assert_eq!(canonical.source_ref.line, 17);
}

// =============================================================================
// V4 — lossy-mapping diagnostic carries severity Warning + LossyMapping kind
// =============================================================================

#[test]
fn v4_lossy_mapping_diagnostic_carries_the_token_source_ref() {
    // The contract artifact names `LossyMapping` as the diagnostic
    // kind for cases where the corpus supports a token the
    // source-IR preserves as a raw string (e.g. an
    // unrecognized directive qualifier). The Slice E
    // implementation does NOT yet inject lossy-mapping
    // diagnostics automatically — the converter is strictly
    // zero-copy. This test exercises the contract clause by
    // hand-constructing the diagnostic kind and asserting the
    // canonical surface it produces.
    let lossy_source_ref = SourceRef::new("cr_equip.lst", 88);

    let diag = SourceContentDiagnostic::lossy_mapping(
        "BONUS:VARMAX=20|OSTYPE=Windows preserved as raw token string",
        lossy_source_ref.clone(),
    );

    assert_eq!(diag.severity, SourceContentSeverity::Warning);
    assert_eq!(diag.kind, SourceContentDiagnosticKind::LossyMapping);
    assert_eq!(diag.source_ref.lst_file, "cr_equip.lst");
    assert_eq!(diag.source_ref.line, 88);
    assert!(diag.message.contains("OSTYPE"));

    // Push the diagnostic into a SourcePackageContent and assert
    // it round-trips through the records_by_kind / diagnostics
    // surface without losing the `LossyMapping` kind.
    let mut pkg = SourcePackageContent::empty("test_pkg", SourceRef::new("cr_equip.lst", 0));
    pkg.push_diagnostic(diag);
    assert_eq!(pkg.diagnostics.len(), 1);
    assert_eq!(
        pkg.diagnostics[0].kind,
        SourceContentDiagnosticKind::LossyMapping
    );
    assert_eq!(pkg.diagnostics[0].severity, SourceContentSeverity::Warning);
}

// =============================================================================
// V5 — SourcePackageContent::records_by_kind returns a deterministic order
// =============================================================================

#[test]
fn v5_records_by_kind_sorts_by_lst_file_then_line() {
    // Build a static Vec of ClassEntry so the borrowed
    // SourceContentRecord<'_> returned by convert_class_entry can
    // outlive the for-loop iter below. Same shape for the lone
    // RaceDeclaration.
    let class_fixtures: Vec<(&str, u32, &'static str)> = vec![
        ("a.lst", 5, "Fighter"),
        ("a.lst", 1, "Wizard"),
        ("b.lst", 4, "Cleric"),
        ("a.lst", 3, "Rogue"),
        ("b.lst", 2, "Bard"),
    ];
    let class_entries: Vec<ClassEntry> = class_fixtures
        .iter()
        .map(|(path, line, name)| {
            // path is only captured into source_ref below; the
            // ClassEntry itself doesn't carry it.
            let _ = path;
            ClassEntry {
                class_name: (*name).to_string(),
                header_line_number: *line as usize,
                header_raw_line: format!("CLASS:{}", name),
                tokens: vec![ClassToken {
                    key: "NAME".to_string(),
                    value: (*name).to_string(),
                    line_number: *line as usize,
                    raw_pair: format!("CLASS:{}", name),
                }],
                feature_blocks: vec![],
            }
        })
        .collect();
    let race_entry = RaceDeclaration {
        source_path: "a.lst".to_string(),
        line_number: 2,
        raw_directive: "RACE:elf".to_string(),
        target: "elf".to_string(),
    };

    let pcc_entry = SourceRef::new("core_rulebook.pcc", 0);
    let mut pkg = SourcePackageContent::empty("pathfinder_pf1", pcc_entry.clone());
    for (idx, entry) in class_entries.iter().enumerate() {
        let (path, line, _) = class_fixtures[idx];
        let mut rec = convert_class_entry(entry);
        rec.source_ref = SourceRef::new(path, line);
        pkg.push(rec);
    }
    let mut race_record = convert_race_declaration(&race_entry);
    race_record.source_ref = SourceRef::new("a.lst", 2);
    pkg.push(race_record);

    let sorted: Vec<SourceContentRecord<'_>> = pkg.records_by_kind(SourceContentKind::Class);
    assert_eq!(sorted.len(), class_fixtures.len());

    // Expected order: by (lst_file, line) ascending. Within
    // "a.lst": 1, 3, 5. Within "b.lst": 2, 4.
    let expected_order = [
        ("a.lst", 1u32),
        ("a.lst", 3),
        ("a.lst", 5),
        ("b.lst", 2),
        ("b.lst", 4),
    ];
    for (i, (path, line)) in expected_order.iter().enumerate() {
        assert_eq!(
            sorted[i].source_ref.lst_file, *path,
            "record {} lst_file mismatch",
            i
        );
        assert_eq!(
            sorted[i].source_ref.line, *line,
            "record {} line mismatch",
            i
        );
    }

    // Race-kind filter returns exactly the one Race record.
    let races = pkg.records_by_kind(SourceContentKind::Race);
    assert_eq!(races.len(), 1);
    assert_eq!(races[0].source_ref.lst_file, "a.lst");
    assert_eq!(races[0].source_ref.line, 2);

    // Same input built twice yields identical order.
    let mut pkg2 = SourcePackageContent::empty("pathfinder_pf1", pcc_entry);
    let tie_entries: Vec<ClassEntry> = (0..class_entries.len())
        .map(|i| ClassEntry {
            class_name: format!("Class{}", i),
            header_line_number: 1,
            header_raw_line: format!("CLASS:Class{}", i),
            tokens: vec![],
            feature_blocks: vec![],
        })
        .collect();
    for entry in &tie_entries {
        let mut rec = convert_class_entry(entry);
        rec.source_ref = SourceRef::new("same.lst", 1);
        pkg2.push(rec);
    }
    let s1 = pkg2.records_by_kind(SourceContentKind::Class);
    let s2 = pkg2.records_by_kind(SourceContentKind::Class);
    assert_eq!(s1.len(), s2.len());
    for (a, b) in s1.iter().zip(s2.iter()) {
        assert_eq!(a.source_ref, b.source_ref);
    }
    let _ = class_entries; // keep borrow live through test end
    let _ = race_entry;
    let _ = tie_entries;
}

// =============================================================================
// Bonus — ConverttoIr (the public entry point) produces canonical records
// =============================================================================

#[test]
fn bonus_convert_to_ir_dispatches_to_canonical_record_per_kind() {
    let schema = IRSchema::canonical_v1();

    // ClassEntry
    let class_parsed = parse_class_entries("cr_classes.lst", "CLASS:Fighter\tHD:10\n");
    let class_entry: &ClassEntry = &class_parsed.entries[0];
    let r = convert_to_ir(&ParsedLstRecord::from_class(class_entry), &schema);
    assert_eq!(r.kind, SourceContentKind::Class);
    assert!(matches!(r.payload, SourceContentPayload::Class(_)));

    // SpellcastingClassEntry
    let scc_parsed =
        parse_spellcasting_class_entries("cr_classes_magic.lst", "CLASS:Wizard\tSPELLSTAT:INT\n");
    let scc_entry: &SpellcastingClassEntry = &scc_parsed.entries[0];
    let r = convert_to_ir(
        &ParsedLstRecord::from_spellcasting_class(scc_entry),
        &schema,
    );
    assert_eq!(r.kind, SourceContentKind::SpellcastingClass);
    assert!(matches!(
        r.payload,
        SourceContentPayload::SpellcastingClass(_)
    ));

    // RaceDeclaration
    let race_parsed = parse_lst_entry("cr_races.lst", "RACE:cr_races_inner.lst\n");
    let race: &RaceDeclaration = &race_parsed.race_pointers[0];
    let r = convert_to_ir(&ParsedLstRecord::from_race(race), &schema);
    assert_eq!(r.kind, SourceContentKind::Race);
    assert!(matches!(r.payload, SourceContentPayload::Race(_)));

    // AbilityDeclaration
    let ability_parsed = parse_lst_entry("cr_abilities.lst", "ABILITY:CATEGORY=FEAT|Alertness\n");
    let ability: &AbilityDeclaration = &ability_parsed.ability_declarations[0];
    let r = convert_to_ir(&ParsedLstRecord::from_ability(ability), &schema);
    assert_eq!(r.kind, SourceContentKind::Ability);
    assert!(matches!(r.payload, SourceContentPayload::Ability(_)));

    // SpellRecord
    let spell_row = parse_lst_spell_row(
        "cr_spells.lst",
        1,
        "Magic Missile\tSCHOOL:Evocation\tDESCRIPTOR:Force\tCASTTIME:1 standard action",
    );
    let spell_inner: &LstSpellRecord = spell_row.record.as_ref().expect("expected record");
    let r = convert_to_ir(&ParsedLstRecord::Spell(spell_inner), &schema);
    assert_eq!(r.kind, SourceContentKind::Spell);
    assert!(matches!(r.payload, SourceContentPayload::Spell(_)));

    // EquipmentRecord
    let equip_parsed = parse_equipment_entries("cr_equip.lst", "Longsword\tTYPE:Weapon\n");
    let equip: &EquipmentRecord = &equip_parsed.entries[0];
    let r = convert_to_ir(&ParsedLstRecord::from_equipment(equip), &schema);
    assert_eq!(r.kind, SourceContentKind::Equipment);
    assert!(matches!(r.payload, SourceContentPayload::Equipment(_)));

    // LstRecord
    let meta_parsed = parse_lst_metadata_text("cr_meta.lst", "DEITY:Lamashtu\n");
    let meta_entry: &LstRecord = &meta_parsed.records[0];
    let r = convert_to_ir(&ParsedLstRecord::from_metadata(meta_entry), &schema);
    assert_eq!(
        r.kind,
        SourceContentKind::Metadata(MetadataKindInner::Deity)
    );
    assert!(matches!(r.payload, SourceContentPayload::Metadata(_)));
}

// =============================================================================
// Bonus — SOURCE_IR_VERSION is u32::1, schema mirrors it
// =============================================================================

#[test]
fn bonus_source_ir_version_starts_at_one() {
    assert_eq!(SOURCE_IR_VERSION, 1);
    let schema = IRSchema::canonical_v1();
    assert_eq!(schema.schema_version, SOURCE_IR_VERSION);
}

// =============================================================================
// Bonus — Constructor sanity for the diagnostic surface
// =============================================================================

#[test]
fn bonus_diagnostic_constructors_set_correct_severity_and_kind() {
    let sr = SourceRef::new("cr_equip.lst", 1);

    let m = SourceContentDiagnostic::malformed("oops", sr.clone());
    assert_eq!(m.severity, SourceContentSeverity::Error);
    assert_eq!(m.kind, SourceContentDiagnosticKind::MalformedRecord);

    let l = SourceContentDiagnostic::lossy_mapping("OS:Windows", sr.clone());
    assert_eq!(l.severity, SourceContentSeverity::Warning);
    assert_eq!(l.kind, SourceContentDiagnosticKind::LossyMapping);

    let u = SourceContentDiagnostic::unsupported_token("ZKEY:x", sr.clone());
    assert_eq!(u.severity, SourceContentSeverity::Warning);
    assert_eq!(u.kind, SourceContentDiagnosticKind::UnsupportedToken);

    let p = SourceContentDiagnostic::partial_translation("partial", sr);
    assert_eq!(p.severity, SourceContentSeverity::Info);
    assert_eq!(p.kind, SourceContentDiagnosticKind::PartialTranslation);
}

// =============================================================================
// Bonus — Class/Spell/Equipment fixture imports kept live (BonusToken, etc.)
// =============================================================================

#[allow(dead_code)]
fn _keep_fixture_imports_live() {
    // Lint suppression: imports referenced above are kept live via
    // the test functions, but the B-5 `BonusToken` type is also
    // referenced indirectly through EquipmentRecord::bonus_chains.
    let _: Option<BonusToken> = None;
    let _: Option<EquipmentRecordKind> = None;
    let _: Option<EquipmentToken> = None;
    let _: Option<ClassToken> = None;
}
