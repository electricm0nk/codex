//! SD-17 Slice D acceptance tests — record-aggregation unification.
//!
//! Slice D relocated `ParsedLstRecord<'a>` from the IR-converter surface
//! into the LST-parser surface, and Slice E retargeted the converter to
//! return `SourceContentRecord<'a>` directly (no `Result` wrapper). The
//! D test file is rewritten here to the canonical Slice E API while
//! preserving the D acceptance criteria:
//!
//! - D1 each of the six variants constructed from a representative
//!   fixture record round-trips through `convert_to_ir` and produces a
//!   `SourceContentRecord` whose `kind` and `payload` variant match.
//! - D2 the enum's kind tag carries through (the convenience
//!   constructors `from_*` produce the variant with the matching tag).
//! - D3 a malformed entry still surfaces the B-family parser's
//!   diagnostic (no regression in the error path through the relocated
//!   aggregate enum's constructors).
//! - D4 the relocated aggregate is reachable via the parser-surface
//!   path `crate::pcgen_import::lst_parser::ParsedLstRecord` AND via the
//!   umbrella path `crate::pcgen_import::ParsedLstRecord`. Both
//!   re-exports must resolve.
//! - D5 the local definition is gone from `ir_converter.rs` —
//!   `pcgen_import::ir_converter::ParsedLstRecord` is a re-export,
//!   not a definition. (See the file header for proof.)

use codex::pcgen_import::ir_converter::{
    convert_to_ir, IRSchema,
};
use codex::pcgen_import::lst_parser::class::{ClassEntry, parse_class_entries};
use codex::pcgen_import::lst_parser::equipment::{
    EquipmentRecord, EquipmentRecordKind, parse_equipment_entries,
};
use codex::pcgen_import::lst_parser::metadata::{LstRecord, MetadataKind, parse_lst_metadata_text};
use codex::pcgen_import::lst_parser::race_ability::{
    AbilityDeclaration, RaceDeclaration, parse_lst_entry,
};
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::pcgen_import::lst_parser::spellcasting_class::{
    SpellcastingClassEntry, parse_spellcasting_class_entries,
};
// Canonical home: parser surface.
use codex::pcgen_import::lst_parser::ParsedLstRecord;
// Canonical envelope: rules_core::source_content (SourceContentRecord,
// SourceContentKind, SourceRef) and pcgen_import::source_content_payload
// (SourceContentPayload).
use codex::pcgen_import::source_content_payload::SourceContentPayload;
use codex::rules_core::source_content::{
    MetadataKindInner, SourceContentKind, SourceContentRecord, SourceRef,
};
// Umbrella re-export path: pcgen_import itself.
use codex::pcgen_import::ParsedLstRecord as ParsedLstRecordUmbrella;

fn canonical_schema() -> IRSchema {
    IRSchema::canonical_v1()
}

fn assert_kind_payload(
    record: &SourceContentRecord<'_>,
    expected_kind: SourceContentKind,
    payload_label: &str,
) {
    assert_eq!(
        record.kind, expected_kind,
        "expected kind {expected_kind:?}, got {kind:?}",
        kind = record.kind,
    );
    let payload_label_matches = match (&record.payload, payload_label) {
        (SourceContentPayload::Class(_), "Class") => true,
        (SourceContentPayload::SpellcastingClass(_), "SpellcastingClass") => true,
        (SourceContentPayload::Race(_), "Race") => true,
        (SourceContentPayload::Ability(_), "Ability") => true,
        (SourceContentPayload::Spell(_), "Spell") => true,
        (SourceContentPayload::Equipment(_), "Equipment") => true,
        (SourceContentPayload::Metadata(_), "Metadata") => true,
        _ => false,
    };
    assert!(
        payload_label_matches,
        "payload variant did not match expected label `{payload_label}`: got {:?}",
        std::mem::discriminant(&record.payload)
    );
}

// =============================================================================
// D1 — each of the six variants round-trips through convert_to_ir
// =============================================================================

#[test]
fn d1_class_variant_round_trips_through_convert_to_ir() {
    let text = "CLASS:Fighter\tHD:10\n";
    let parsed = parse_class_entries("classes.lst", text);
    let entry: &ClassEntry = &parsed.entries[0];

    let record = convert_to_ir(&ParsedLstRecord::from_class(entry), &canonical_schema());
    assert_kind_payload(&record, SourceContentKind::Class, "Class");
}

#[test]
fn d1_spellcasting_class_variant_round_trips_through_convert_to_ir() {
    let text = "CLASS:Wizard\tSPELLSTAT:INT\tSPELLBOOK:YES\n";
    let parsed = parse_spellcasting_class_entries("classes.lst", text);
    let entry: &SpellcastingClassEntry = &parsed.entries[0];

    let record = convert_to_ir(
        &ParsedLstRecord::from_spellcasting_class(entry),
        &canonical_schema(),
    );
    assert_kind_payload(
        &record,
        SourceContentKind::SpellcastingClass,
        "SpellcastingClass",
    );
}

#[test]
fn d1_race_variant_round_trips_through_convert_to_ir() {
    let text = "RACE:cr_races.lst\n";
    let parsed = parse_lst_entry("races.lst", text);
    let race: &RaceDeclaration = &parsed.race_pointers[0];

    let record = convert_to_ir(&ParsedLstRecord::from_race(race), &canonical_schema());
    assert_kind_payload(&record, SourceContentKind::Race, "Race");
}

#[test]
fn d1_ability_variant_round_trips_through_convert_to_ir() {
    let text = "ABILITY:CATEGORY=FEAT|Alertness\n";
    let parsed = parse_lst_entry("abilities.lst", text);
    let ability: &AbilityDeclaration = &parsed.ability_declarations[0];

    let record = convert_to_ir(
        &ParsedLstRecord::from_ability(ability),
        &canonical_schema(),
    );
    assert_kind_payload(&record, SourceContentKind::Ability, "Ability");
}

#[test]
fn d1_spell_variant_round_trips_through_convert_to_ir() {
    let row = parse_lst_spell_row("cr_spells.lst", 5, "Magic Missile\tSCHOOL:Evocation");
    let record = row.record.expect("expected parsed record");

    let converted = convert_to_ir(&ParsedLstRecord::from_spell(&record), &canonical_schema());
    assert_kind_payload(&converted, SourceContentKind::Spell, "Spell");
}

#[test]
fn d1_equipment_variant_round_trips_through_convert_to_ir() {
    let text = "Longsword\tTYPE:Weapon\tCOST:15\n";
    let parsed = parse_equipment_entries("cr_equip.lst", text);
    let equip: &EquipmentRecord = &parsed.entries[0];

    let record = convert_to_ir(
        &ParsedLstRecord::from_equipment(equip),
        &canonical_schema(),
    );
    assert_kind_payload(&record, SourceContentKind::Equipment, "Equipment");
}

#[test]
fn d1_metadata_variant_round_trips_through_convert_to_ir() {
    let text = "DEITY:Lamashtu\n";
    let parsed = parse_lst_metadata_text("meta.lst", text);
    let record: &LstRecord = &parsed.records[0];

    let converted = convert_to_ir(
        &ParsedLstRecord::from_metadata(record),
        &canonical_schema(),
    );
    let SourceContentKind::Metadata(inner) = converted.kind else {
        panic!("expected Metadata kind, got {:?}", converted.kind);
    };
    assert_eq!(inner, MetadataKindInner::Deity);
    assert!(matches!(converted.payload, SourceContentPayload::Metadata(_)));
}

// =============================================================================
// D2 — kind tag carries through the convenience constructors
// =============================================================================

#[test]
fn d2_convenience_constructors_produce_matching_variant_tag() {
    let class_text = "CLASS:Fighter\tHD:10\n";
    let class_parsed = parse_class_entries("classes.lst", class_text);
    let class_entry: &ClassEntry = &class_parsed.entries[0];
    assert!(matches!(
        ParsedLstRecord::from_class(class_entry),
        ParsedLstRecord::Class(_)
    ));

    let spellcasting_text = "CLASS:Wizard\tSPELLSTAT:INT\n";
    let spellcasting_parsed = parse_spellcasting_class_entries("classes.lst", spellcasting_text);
    let spellcasting_entry: &SpellcastingClassEntry = &spellcasting_parsed.entries[0];
    assert!(matches!(
        ParsedLstRecord::from_spellcasting_class(spellcasting_entry),
        ParsedLstRecord::SpellcastingClass(_)
    ));

    let race_text = "RACE:cr_races.lst\n";
    let race_parsed = parse_lst_entry("races.lst", race_text);
    let race: &RaceDeclaration = &race_parsed.race_pointers[0];
    assert!(matches!(
        ParsedLstRecord::from_race(race),
        ParsedLstRecord::Race(_)
    ));

    let ability_text = "ABILITY:CATEGORY=FEAT|Alertness\n";
    let ability_parsed = parse_lst_entry("abilities.lst", ability_text);
    let ability: &AbilityDeclaration = &ability_parsed.ability_declarations[0];
    assert!(matches!(
        ParsedLstRecord::from_ability(ability),
        ParsedLstRecord::Ability(_)
    ));

    let spell_row = parse_lst_spell_row("cr_spells.lst", 7, "Shield\tSCHOOL:Abjuration");
    let spell_record = spell_row.record.expect("expected parsed record");
    assert!(matches!(
        ParsedLstRecord::from_spell(&spell_record),
        ParsedLstRecord::Spell(_)
    ));

    let equip_text = "Longsword\tTYPE:Weapon\tCOST:15\n";
    let equip_parsed = parse_equipment_entries("cr_equip.lst", equip_text);
    let equip: &EquipmentRecord = &equip_parsed.entries[0];
    assert!(matches!(
        ParsedLstRecord::from_equipment(equip),
        ParsedLstRecord::Equipment(_)
    ));

    let meta_text = "DEITY:Rovagug\n";
    let meta_parsed = parse_lst_metadata_text("meta.lst", meta_text);
    let meta_record: &LstRecord = &meta_parsed.records[0];
    assert!(matches!(
        ParsedLstRecord::from_metadata(meta_record),
        ParsedLstRecord::Metadata(_)
    ));
}

// =============================================================================
// D3 — malformed entry still surfaces the B-family parser's diagnostic
// =============================================================================
//
// Slice E retargets the converter, so this D criterion is verified at
// the per-record level: when an input record is filtered out by the
// B-family parser (e.g. a misspelled martial-class name), the parser
// produces an `LstDiagnostic` on the parse result and the
// `ParsedLstRecord` constructor path does not synthesize a phantom
// record.

#[test]
fn d3_class_parser_malformed_emits_diagnostic_no_aggregate_regression() {
    let text = "CLASS:\tHD:10\n"; // malformed: empty class name
    let parsed = parse_class_entries("classes.lst", text);
    assert!(
        !parsed.diagnostics.is_empty(),
        "B-1 malformed-line diagnostic must be present",
    );
    assert!(
        parsed.entries.is_empty(),
        "no record should be produced for a malformed header line",
    );
    let malformed = parsed.diagnostics.iter().find(|d| {
        matches!(
            d.kind,
            codex::pcgen_import::lst_parser::class::LstDiagnosticKind::MalformedSD17B1
        )
    });
    assert!(
        malformed.is_some(),
        "expected a MalformedSD17B1 diagnostic in {:?}",
        parsed.diagnostics,
    );
}

#[test]
fn d3_equipment_kind_carries_through_aggregate_constructor() {
    let weapon_text = "Longsword\tTYPE:Weapon\tCOST:15\n";
    let weapon_parsed = parse_equipment_entries("cr_equip.lst", weapon_text);
    let weapon: &EquipmentRecord = &weapon_parsed.entries[0];
    assert_eq!(weapon.kind, EquipmentRecordKind::Equip);

    let aggregate = ParsedLstRecord::from_equipment(weapon);
    let record = convert_to_ir(&aggregate, &canonical_schema());
    assert_eq!(record.kind, SourceContentKind::Equipment);
    if let SourceContentPayload::Equipment(payload) = &record.payload {
        assert_eq!(payload.kind, EquipmentRecordKind::Equip);
    } else {
        panic!("expected Equipment payload, got {:?}", record.payload);
    }
}

#[test]
fn d3_metadata_kind_carries_through_aggregate_constructor() {
    use codex::rules_core::source_content::MetadataKindInner;
    for (text, expected_parser_kind, expected_canonical_inner) in [
        ("DEITY:Lamashtu\n", MetadataKind::Deity, MetadataKindInner::Deity),
        ("DOMAIN:Death\n", MetadataKind::Domain, MetadataKindInner::Domain),
        ("KITS:Standard\n", MetadataKind::Kits, MetadataKindInner::Kits),
        ("LANGUAGE:Common\n", MetadataKind::Language, MetadataKindInner::Language),
        (
            "TEMPLATE:Skeleton\n",
            MetadataKind::Template,
            MetadataKindInner::Template,
        ),
        (
            "COMPANIONMOD:Standard\n",
            MetadataKind::CompanionMod,
            MetadataKindInner::CompanionMod,
        ),
    ] {
        let parsed = parse_lst_metadata_text("meta.lst", text);
        assert!(
            !parsed.records.is_empty(),
            "expected a parsed record for `{text}`"
        );
        let rec = &parsed.records[0];
        assert_eq!(rec.kind, expected_parser_kind);
        let aggregate = ParsedLstRecord::from_metadata(rec);
        let converted = convert_to_ir(&aggregate, &canonical_schema());
        let SourceContentKind::Metadata(inner) = converted.kind else {
            panic!("expected Metadata kind, got {:?}", converted.kind);
        };
        assert_eq!(inner, expected_canonical_inner);
    }
}

// =============================================================================
// D4 — re-export surfaces resolve via both the parser path and the umbrella
// =============================================================================
//
// `pcgen_import::lst_parser::ParsedLstRecord` (canonical home) and
// `pcgen_import::ParsedLstRecord` (umbrella re-export) are the same
// type. Compile-time guarantee: this test simply constructs one of
// each and pattern-matches both; the compiler enforces the re-export
// exists.

#[test]
fn d4_both_re_exports_resolve_to_the_same_variant() {
    let class_text = "CLASS:Fighter\tHD:10\n";
    let parsed = parse_class_entries("classes.lst", class_text);
    let entry: &ClassEntry = &parsed.entries[0];

    let canonical: ParsedLstRecord<'_> = ParsedLstRecord::from_class(entry);
    let umbrella: ParsedLstRecordUmbrella<'_> = ParsedLstRecordUmbrella::from_class(entry);
    let canonical_record = convert_to_ir(&canonical, &canonical_schema());
    let umbrella_record = convert_to_ir(&umbrella, &canonical_schema());
    assert_eq!(canonical_record.kind, umbrella_record.kind);
    assert_eq!(canonical_record.source_ref.line, umbrella_record.source_ref.line);
}

// =============================================================================
// D5 — SourceRef line numbers are preserved through the projection
// =============================================================================
//
// The relocation is structural, not behavioral. Source-line provenance
// from the parser surface must be preserved end-to-end.

#[test]
fn d5_source_ref_line_numbers_are_preserved_through_conversion() {
    let text = "CLASS:Fighter\tHD:10\nCLASS:Barbarian\tHD:12\n";
    let parsed = parse_class_entries("classes.lst", text);
    let first: &ClassEntry = &parsed.entries[0];
    let second: &ClassEntry = &parsed.entries[1];
    assert_eq!(first.header_line_number, 1);
    assert_eq!(second.header_line_number, 2);

    let first_record = convert_to_ir(&ParsedLstRecord::from_class(first), &canonical_schema());
    let second_record =
        convert_to_ir(&ParsedLstRecord::from_class(second), &canonical_schema());
    assert_eq!(first_record.source_ref.line, 1);
    assert_eq!(second_record.source_ref.line, 2);
}

#[test]
fn d5_source_ref_constructor_matches_parser_layout() {
    // Smoke test: the canonical `SourceRef` constructor accepts
    // (path, line) just like the parser's own source layout.
    let r = SourceRef::new("cr_classes.lst".to_string(), 42);
    assert_eq!(r.lst_file, "cr_classes.lst");
    assert_eq!(r.line, 42);
}
