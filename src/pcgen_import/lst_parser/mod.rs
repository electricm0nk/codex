//! PCGen LST parsers, partitioned by object kind.
//!
//! ## Record-aggregation surface (SD-17 Slice D)
//!
//! [`ParsedLstRecord`] is the unified, kind-tagged handle every B-family
//! record references by. It was originally authored inside
//! `pcgen_import::ir_converter` and is relocated here so downstream
//! consumers of the LST parser surface can pattern-match on kind without
//! reaching through the IR converter. The IR converter still consumes
//! the same enum via a re-export of its own for backward compatibility.
//!
//! The relocation is structural, not behavioral: the variants, the
//! convenience constructors, and the lifetime parameter are byte-identical
//! to the in-converter version Slice C landed. See
//! `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/sd17-d-record-aggregate-merge-receipt-2026-07-12.md`
//! for the slice merge receipt.

pub mod class;
pub mod equipment;
pub mod metadata;
pub mod race_ability;
pub mod spell;
pub mod spellcasting_class;

// =============================================================================
// ParsedLstRecord — unified kind-tagged record aggregate
// =============================================================================
//
// Every variant borrows the B-family record by reference. The conversion
// is allocation-light because the B-family record lives in the caller's
// frame (R2 in the SD-17 contract).
//
// The B-family entry types referenced by `ParsedLstRecord` are imported
// below. Of the six, four are already re-exported from their
// submodules via the `pub use class::{...}` etc. blocks at the bottom of
// this file (`ClassEntry`, `SpellcastingClassEntry` is missing from that
// block — see below — but `AbilityDeclaration`, `RaceDeclaration`,
// `EquipmentRecord`, and `LstRecord` are present, as is `LstSpellRecord`
// after the explicit `pub use spell::LstSpellRecord` and
// `pub use spellcasting_class::SpellcastingClassEntry` aliases below).
// Rust resolves the enum's references in declaration order, so the
// `pub use` aliases for the two missing entry types are placed BEFORE
// the `pub enum ParsedLstRecord` definition.

pub use spell::LstSpellRecord;
pub use spellcasting_class::SpellcastingClassEntry;

/// Canonical input enum that [`crate::pcgen_import::ir_converter::convert_to_ir`]
/// dispatches on.
///
/// Every variant borrows the B-family record by reference. The conversion
/// is allocation-light because the B-family record lives in the caller's
/// frame. Slice D relocated this enum from the IR-converter module so
/// downstream consumers of the LST parser surface can pattern-match on
/// kind without depending on the IR converter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedLstRecord<'a> {
    Class(&'a ClassEntry),
    SpellcastingClass(&'a SpellcastingClassEntry),
    Race(&'a RaceDeclaration),
    Ability(&'a AbilityDeclaration),
    Spell(&'a LstSpellRecord),
    Equipment(&'a EquipmentRecord),
    Metadata(&'a LstRecord),
}

impl<'a> ParsedLstRecord<'a> {
    /// Convenience: build a `ParsedLstRecord::Class` from any `&ClassEntry`.
    pub fn from_class(e: &'a ClassEntry) -> Self {
        ParsedLstRecord::Class(e)
    }

    /// Convenience: build a `ParsedLstRecord::SpellcastingClass`.
    pub fn from_spellcasting_class(e: &'a SpellcastingClassEntry) -> Self {
        ParsedLstRecord::SpellcastingClass(e)
    }

    /// Convenience: build a `ParsedLstRecord::Race`.
    pub fn from_race(r: &'a RaceDeclaration) -> Self {
        ParsedLstRecord::Race(r)
    }

    /// Convenience: build a `ParsedLstRecord::Ability`.
    pub fn from_ability(a: &'a AbilityDeclaration) -> Self {
        ParsedLstRecord::Ability(a)
    }

    /// Convenience: build a `ParsedLstRecord::Spell`.
    pub fn from_spell(s: &'a LstSpellRecord) -> Self {
        ParsedLstRecord::Spell(s)
    }

    /// Convenience: build a `ParsedLstRecord::Equipment`.
    pub fn from_equipment(e: &'a EquipmentRecord) -> Self {
        ParsedLstRecord::Equipment(e)
    }

    /// Convenience: build a `ParsedLstRecord::Metadata`.
    pub fn from_metadata(r: &'a LstRecord) -> Self {
        ParsedLstRecord::Metadata(r)
    }
}

// Preserve the slice APIs while avoiding collisions between their diagnostic
// types. The metadata API retains the umbrella names because it landed first;
// the class and race/ability parser diagnostic types remain available through
// their respective submodules. The spellcasting-class slice (SD-17 B-2)
// exposes its surface through its own submodule to keep its diagnostic
// type distinct from B-1's `LstDiagnosticKind::MalformedSD17B1`.
pub use class::{
    ClassEntry, ClassFeatureBlock, ClassLevelLine, ClassParseResult, ClassToken,
    MARTIAL_CLASS_NAMES, parse_class_entries, parse_class_file,
};
pub use equipment::{
    BonusToken, EquipmentDiagnostic, EquipmentDiagnosticKind, EquipmentParseResult,
    EquipmentRecord, EquipmentRecordKind, EquipmentToken, parse_equipment_entries,
    parse_equipment_file,
};
pub use metadata::{
    LstDiagnostic, LstDiagnosticKind, LstMetadataDocument, LstRecord, MetadataKind,
    parse_lst_metadata, parse_lst_metadata_text,
};
pub use race_ability::{
    AbilityDeclaration, AbilityKind, AbilityParsedFields, LstEntryFile, RaceDeclaration,
    parse_lst_entry,
};
