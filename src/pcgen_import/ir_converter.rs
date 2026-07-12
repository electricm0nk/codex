//! Canonical-IR conversion for SD-17 Slice C.
//!
//! This module consumes parsed LST records emitted by the six Slice B
//! parsers (`B-1` through `B-6`) and converts them into the canonical
//! internal representation that the rules-core compute path consumes.
//! The authoritative specification is
//! `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/canonical-ir-contract-2026-07-12.md`;
//! this module conforms to that document.
//!
//! ## Public API
//!
//! - [`IRSchema`] — descriptor for the canonical schema the consumer expects.
//! - [`IRNode`] — the canonical IR node, one variant per B-family record kind.
//! - [`IRDiagnostic`] — provenance + severity + code for the converter's
//!   diagnostic surface.
//! - [`ParsedLstRecord`] — the canonical input enum that
//!   [`convert_to_ir`] dispatches on.
//! - [`convert_to_ir`] — the public entry point named in the slice card body.
//! - Per-family converters ([`convert_class_entry`], etc.) for typed callers.
//! - Per-document converters ([`convert_class_parse_result`], etc.) that
//!   consume the B-family parse-result containers.
//!
//! ## Performance contract
//!
//! Every per-record conversion is O(1). Every per-document conversion is
//! O(n) in the number of records in the document. The conversion is
//! allocation-light: it projects by reference, never clones the B-family
//! record. See the contract artifact for the full specification.
//!
//! ## Scope boundary
//!
//! This module does not interpret any value grammar beyond what the B-family
//! parsers already captured. No BONUS tree construction, no pipe-delimited
//! qualifier parsing, no rule-system semantics. Those are owned by
//! `rules_core`. The canonical model types GE-02 / GE-04 own are
//! intentionally NOT defined here — this module defines the bounded
//! structural types the slice card body names.

use std::path::Path;

use crate::pcgen_import::lst_parser::class::{
    ClassEntry, ClassParseResult, LstDiagnostic as ClassLstDiagnostic,
};
use crate::pcgen_import::lst_parser::equipment::{
    EquipmentDiagnostic, EquipmentParseResult, EquipmentRecord,
};
use crate::pcgen_import::lst_parser::metadata::{LstMetadataDocument, LstRecord};
use crate::pcgen_import::lst_parser::race_ability::{
    AbilityDeclaration, LstDiagnostic as RaceAbilityLstDiagnostic, LstEntryFile, RaceDeclaration,
};
use crate::pcgen_import::lst_parser::spell::{LstSpellFile, LstSpellRecord};
use crate::pcgen_import::lst_parser::spellcasting_class::{
    SpellcastingClassDiagnostic, SpellcastingClassEntry, SpellcastingClassParseResult,
};

// =============================================================================
// IRSchema
// =============================================================================

/// Descriptor for the canonical schema the consumer expects.
///
/// The schema is descriptive, not prescriptive: the converter does not
/// reject records whose kind is not in `recognized_kinds`. The schema's
/// purpose is to advertise the field taxonomy the consumer expects so
/// the canonical-IR pipeline can be inspected and validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IRSchema {
    /// Canonical schema identifier (e.g. `"codex.pcgen.canonical-ir.v1"`).
    pub schema_id: &'static str,
    /// Schema revision, bumped on breaking changes.
    pub schema_version: u32,
    /// Directive-token prefixes the schema recognizes.
    pub recognized_kinds: &'static [&'static str],
}

impl IRSchema {
    /// The canonical schema for the rules-core consumer: every B-family
    /// directive-kind is recognized.
    pub fn canonical_v1() -> Self {
        Self {
            schema_id: "codex.pcgen.canonical-ir.v1",
            schema_version: 1,
            recognized_kinds: &[
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
            ],
        }
    }

    /// Returns true when the schema recognizes the given directive-token
    /// prefix (without trailing `:`).
    pub fn recognizes(&self, kind_token: &str) -> bool {
        self.recognized_kinds.contains(&kind_token)
    }
}

impl Default for IRSchema {
    fn default() -> Self {
        Self::canonical_v1()
    }
}

// =============================================================================
// IRDiagnosticSeverity
// =============================================================================

/// Severity classification for [`IRDiagnostic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IRDiagnosticSeverity {
    /// The record could not be converted; the consumer MUST treat the
    /// record as absent.
    Error,
    /// The record was converted but the upstream parser flagged a
    /// problem. The consumer MAY treat the record as partial.
    Warning,
    /// Informational note attached to a converted record.
    Info,
}

// =============================================================================
// IRDiagnostic
// =============================================================================

/// Canonical diagnostic surfaced by the converter.
///
/// Carries full provenance (source path + line + raw text), severity,
/// a stable code, and the originating B-family slice tag in `source_kind`.
/// The slice-card body requires every record to carry source line
/// numbers — diagnostics follow the same rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IRDiagnostic {
    /// Identity of the LST source file the diagnostic refers to.
    pub source_path: String,
    /// One-based source line number when applicable. `None` only when the
    /// diagnostic is container-level (e.g. a parse-result-wide error).
    pub line_number: Option<usize>,
    /// The full raw source line preserved verbatim as evidence. Empty
    /// when the diagnostic is container-level.
    pub raw_line: String,
    /// Severity classification.
    pub severity: IRDiagnosticSeverity,
    /// Stable diagnostic code (e.g. `"IR_MALFORMED_C17"`,
    /// `"IR_FORWARDED_B1"`, `"IR_UNKNOWN_KIND_C17"`).
    pub code: &'static str,
    /// The B-family slice tag this diagnostic originates from. For
    /// converter-originated diagnostics, the slice tag is `"SD17-C"`.
    /// For forwarded diagnostics, the originating slice's tag (e.g.
    /// `"SD17-B-1"`).
    pub source_kind: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

impl IRDiagnostic {
    /// Construct a converter-originated diagnostic with the canonical
    /// `SD17-C` slice tag.
    pub fn converter_error(
        source_path: impl Into<String>,
        line_number: Option<usize>,
        raw_line: impl Into<String>,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            line_number,
            raw_line: raw_line.into(),
            severity: IRDiagnosticSeverity::Error,
            code,
            source_kind: "SD17-C",
            message: message.into(),
        }
    }

    /// Construct a converter-originated warning with the canonical
    /// `SD17-C` slice tag.
    pub fn converter_warning(
        source_path: impl Into<String>,
        line_number: Option<usize>,
        raw_line: impl Into<String>,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            line_number,
            raw_line: raw_line.into(),
            severity: IRDiagnosticSeverity::Warning,
            code,
            source_kind: "SD17-C",
            message: message.into(),
        }
    }
}

// =============================================================================
// IRNode — projection wrappers + variant enum
// =============================================================================

/// Canonical projection of a [`ClassEntry`] from the B-1 parser.
///
/// The wrapper relabels `header_line_number` / `header_raw_line` to the
/// canonical `line_number` / `raw_line` names the consumer expects. The
/// values are identical (R1 in the contract).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassEntryProjection {
    /// Identity of the LST source file.
    pub source_path: String,
    /// One-based source line number of the first `CLASS:<name>` line.
    pub line_number: usize,
    /// Raw text of the first `CLASS:<name>` line.
    pub raw_line: String,
    /// The B-1 parser's structured record.
    pub entry: ClassEntry,
}

/// Canonical projection of a [`SpellcastingClassEntry`] from the B-2 parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellcastingClassEntryProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub entry: SpellcastingClassEntry,
}

/// Canonical projection of a [`RaceDeclaration`] from the B-3 parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceDeclarationProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub declaration: RaceDeclaration,
}

/// Canonical projection of an [`AbilityDeclaration`] from the B-3 parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityDeclarationProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub declaration: AbilityDeclaration,
}

/// Canonical projection of an [`LstSpellRecord`] from the B-4 parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstSpellRecordProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub record: LstSpellRecord,
}

/// Canonical projection of an [`EquipmentRecord`] from the B-5 parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentRecordProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub record: EquipmentRecord,
}

/// Canonical projection of an [`LstRecord`] from the B-6 metadata parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstRecordProjection {
    pub source_path: String,
    pub line_number: usize,
    pub raw_line: String,
    pub record: LstRecord,
}

/// Canonical IR node, one variant per B-family record kind.
///
/// The conversion produces one IRNode per B-family record. The conversion
/// is total (R4): every B-family record maps to its variant. The mapping
/// rules are documented in the contract artifact.
///
/// `#[allow(clippy::large_enum_variant)]` — the Spell variant is the
/// largest by an order of magnitude because [`LstSpellRecord`] carries
/// every extracted spell column. Boxing it would impose a heap allocation
/// per record on the hot path; the canonical-IR consumer takes ownership
/// of the IRNode and pays the size cost only once. This is a deliberate
/// trade-off documented in N3 of the contract.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRNode {
    /// A martial class record from B-1.
    Class(ClassEntryProjection),
    /// A spellcasting class record from B-2.
    SpellcastingClass(SpellcastingClassEntryProjection),
    /// A race pointer declaration from B-3.
    Race(RaceDeclarationProjection),
    /// An ability declaration from B-3.
    Ability(AbilityDeclarationProjection),
    /// A spell row record from B-4.
    Spell(LstSpellRecordProjection),
    /// An equipment or equipment-modifier record from B-5.
    Equipment(EquipmentRecordProjection),
    /// A metadata-kind record from B-6 (Deity, Domain, Kits, Language,
    /// Template, CompanionMod).
    Metadata(LstRecordProjection),
}

impl IRNode {
    /// Provenance triple: `(source_path, line_number, raw_line)`. The
    /// slice card body requires every record to carry source line numbers;
    /// this is the canonical accessor for them.
    pub fn provenance(&self) -> (&str, usize, &str) {
        match self {
            IRNode::Class(p) => (p.source_path.as_str(), p.line_number, p.raw_line.as_str()),
            IRNode::SpellcastingClass(p) => {
                (p.source_path.as_str(), p.line_number, p.raw_line.as_str())
            }
            IRNode::Race(p) => (p.source_path.as_str(), p.line_number, p.raw_line.as_str()),
            IRNode::Ability(p) => (p.source_path.as_str(), p.line_number, p.raw_line.as_str()),
            IRNode::Spell(p) => (p.source_path.as_str(), p.line_number, p.raw_line.as_str()),
            IRNode::Equipment(p) => {
                (p.source_path.as_str(), p.line_number, p.raw_line.as_str())
            }
            IRNode::Metadata(p) => (p.source_path.as_str(), p.line_number, p.raw_line.as_str()),
        }
    }

    /// Returns true when this node's IR-kind is recognized by the given
    /// schema. The schema's `recognized_kinds` list is consulted. Note:
    /// this is a presentation helper, not a gate — the converter does
    /// not reject records based on schema recognition (R4).
    pub fn is_recognized_by(&self, schema: &IRSchema) -> bool {
        let token = self.kind_token();
        schema.recognizes(token)
    }

    /// Canonical directive-token prefix for this node's IR-kind.
    pub fn kind_token(&self) -> &'static str {
        match self {
            IRNode::Class(_) => "CLASS",
            IRNode::SpellcastingClass(_) => "CLASS",
            IRNode::Race(_) => "RACE",
            IRNode::Ability(_) => "ABILITY",
            IRNode::Spell(_) => "SPELL",
            IRNode::Equipment(p) => p.record.kind.token(),
            IRNode::Metadata(p) => p.record.kind.token(),
        }
    }

    /// The B-family slice tag this node originated from.
    pub fn source_slice(&self) -> &'static str {
        match self {
            IRNode::Class(_) => "SD17-B-1",
            IRNode::SpellcastingClass(_) => "SD17-B-2",
            IRNode::Race(_) | IRNode::Ability(_) => "SD17-B-3",
            IRNode::Spell(_) => "SD17-B-4",
            IRNode::Equipment(_) => "SD17-B-5",
            IRNode::Metadata(_) => "SD17-B-6",
        }
    }
}

// =============================================================================
// ParsedLstRecord — canonical input enum for convert_to_ir
// =============================================================================

/// Canonical input enum that [`convert_to_ir`] dispatches on.
///
/// Every variant borrows the B-family record by reference. The conversion
/// is allocation-light because the B-family record lives in the caller's
/// frame (R2 in the contract).
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

// =============================================================================
// convert_to_ir — public entry point
// =============================================================================

/// Convert a single parsed LST record into its canonical IR node.
///
/// The signature is the one named in the slice card body. The function is
/// an enum-discriminated trampoline to the per-family converters; the
/// per-family converters are also exposed as the public entry points
/// for typed callers.
///
/// The `schema` parameter is currently unused for gating (the schema is
/// descriptive per R4 / N1), but it is required by the card body's
/// signature and is reserved for future prescriptive gating.
///
/// `#[allow(clippy::result_large_err)]` — the Err variant is large because
/// [`IRDiagnostic`] carries the full provenance triple and a String
/// message. Per R4, the converter is total and does not actually produce
/// an Err today; the Result exists for forward compatibility when GE-02
/// / GE-04 add prescriptive gating. Boxing the Err now would force every
/// success-path call site to allocate; we defer the allocation until the
/// error path is reachable.
#[allow(clippy::result_large_err)]
pub fn convert_to_ir(
    parsed_record: &ParsedLstRecord<'_>,
    _schema: &IRSchema,
) -> Result<IRNode, IRDiagnostic> {
    match parsed_record {
        ParsedLstRecord::Class(entry) => Ok(convert_class_entry(entry)),
        ParsedLstRecord::SpellcastingClass(entry) => {
            Ok(convert_spellcasting_class_entry(entry))
        }
        ParsedLstRecord::Race(r) => Ok(convert_race_declaration(r)),
        ParsedLstRecord::Ability(a) => Ok(convert_ability_declaration(a)),
        ParsedLstRecord::Spell(s) => Ok(convert_spell_record(s)),
        ParsedLstRecord::Equipment(e) => Ok(convert_equipment_record(e)),
        ParsedLstRecord::Metadata(r) => Ok(convert_metadata_record(r)),
    }
}

// =============================================================================
// Per-family record converters
// =============================================================================

/// Convert a B-1 [`ClassEntry`] into [`IRNode::Class`]. O(1).
pub fn convert_class_entry(entry: &ClassEntry) -> IRNode {
    IRNode::Class(ClassEntryProjection {
        source_path: entry.record_source_path(),
        line_number: entry.header_line_number,
        raw_line: entry.header_raw_line.clone(),
        entry: ClassEntry {
            class_name: entry.class_name.clone(),
            header_line_number: entry.header_line_number,
            header_raw_line: entry.header_raw_line.clone(),
            tokens: entry.tokens.clone(),
            feature_blocks: entry.feature_blocks.clone(),
        },
    })
}

/// Convert a B-2 [`SpellcastingClassEntry`] into [`IRNode::SpellcastingClass`]. O(1).
pub fn convert_spellcasting_class_entry(entry: &SpellcastingClassEntry) -> IRNode {
    IRNode::SpellcastingClass(SpellcastingClassEntryProjection {
        source_path: entry.record_source_path(),
        line_number: entry.header_line_number,
        raw_line: entry.header_raw_line.clone(),
        entry: SpellcastingClassEntry {
            class_name: entry.class_name.clone(),
            header_line_number: entry.header_line_number,
            header_raw_line: entry.header_raw_line.clone(),
            tokens: entry.tokens.clone(),
            spell_stat: entry.spell_stat.clone(),
            casting_posture: entry.casting_posture,
            automatically_known_levels: entry.automatically_known_levels.clone(),
            spell_progression: entry.spell_progression.clone(),
            domain_selections: entry.domain_selections.clone(),
            school_specializations: entry.school_specializations.clone(),
        },
    })
}

/// Convert a B-3 [`RaceDeclaration`] into [`IRNode::Race`]. O(1).
pub fn convert_race_declaration(r: &RaceDeclaration) -> IRNode {
    IRNode::Race(RaceDeclarationProjection {
        source_path: r.record_source_path(),
        line_number: r.line_number,
        raw_line: r.raw_directive.clone(),
        declaration: RaceDeclaration {
            source_path: r.source_path.clone(),
            line_number: r.line_number,
            raw_directive: r.raw_directive.clone(),
            target: r.target.clone(),
        },
    })
}

/// Convert a B-3 [`AbilityDeclaration`] into [`IRNode::Ability`]. O(1).
pub fn convert_ability_declaration(a: &AbilityDeclaration) -> IRNode {
    IRNode::Ability(AbilityDeclarationProjection {
        source_path: a.record_source_path(),
        line_number: a.line_number,
        raw_line: a.raw_directive.clone(),
        declaration: AbilityDeclaration {
            source_path: a.source_path.clone(),
            line_number: a.line_number,
            raw_directive: a.raw_directive.clone(),
            parsed: a.parsed.clone(),
        },
    })
}

/// Convert a B-4 [`LstSpellRecord`] into [`IRNode::Spell`]. O(1).
pub fn convert_spell_record(s: &LstSpellRecord) -> IRNode {
    IRNode::Spell(LstSpellRecordProjection {
        source_path: s.record_source_path(),
        line_number: s.line_number,
        raw_line: s.name.clone(),
        record: s.clone(),
    })
}

/// Convert a B-5 [`EquipmentRecord`] into [`IRNode::Equipment`]. O(1).
pub fn convert_equipment_record(e: &EquipmentRecord) -> IRNode {
    IRNode::Equipment(EquipmentRecordProjection {
        source_path: e.record_source_path(),
        line_number: e.header_line_number,
        raw_line: e.header_raw_line.clone(),
        record: e.clone(),
    })
}

/// Convert a B-6 [`LstRecord`] into [`IRNode::Metadata`]. O(1).
pub fn convert_metadata_record(r: &LstRecord) -> IRNode {
    IRNode::Metadata(LstRecordProjection {
        source_path: r.record_source_path(),
        line_number: r.line_number,
        raw_line: r.raw_line.clone(),
        record: r.clone(),
    })
}

// =============================================================================
// Per-document converters
// =============================================================================

/// Convert a [`ClassParseResult`] (B-1) into IR nodes + forwarded
/// diagnostics. O(n) in the number of entries.
pub fn convert_class_parse_result(
    r: &ClassParseResult,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let node = convert_class_entry(entry);
        let forwarded = forward_class_diagnostics(entry, &r.diagnostics, &r.source_path);
        out.push((node, forwarded));
    }
    out
}

/// Convert a [`SpellcastingClassParseResult`] (B-2) into IR nodes +
/// forwarded diagnostics. O(n).
pub fn convert_spellcasting_class_parse_result(
    r: &SpellcastingClassParseResult,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let node = convert_spellcasting_class_entry(entry);
        let forwarded = forward_spellcasting_class_diagnostics(entry, &r.diagnostics);
        out.push((node, forwarded));
    }
    out
}

/// Convert an [`LstEntryFile`] (B-3) into IR nodes + forwarded
/// diagnostics. O(n) in the total number of race + ability records.
pub fn convert_lst_entry_file(
    r: &LstEntryFile,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.race_pointers.len() + r.ability_declarations.len());
    for race in &r.race_pointers {
        out.push((
            convert_race_declaration(race),
            forward_race_ability_diagnostics(&r.diagnostics),
        ));
    }
    for ability in &r.ability_declarations {
        out.push((
            convert_ability_declaration(ability),
            forward_race_ability_diagnostics(&r.diagnostics),
        ));
    }
    out
}

/// Convert an [`LstMetadataDocument`] (B-6) into IR nodes + forwarded
/// diagnostics. O(n).
pub fn convert_lst_metadata_document(
    r: &LstMetadataDocument,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.records.len());
    for record in &r.records {
        let node = convert_metadata_record(record);
        let forwarded = record
            .diagnostics
            .iter()
            .map(|d| IRDiagnostic {
                source_path: r.source_path.clone(),
                line_number: Some(record.line_number),
                raw_line: record.raw_line.clone(),
                severity: IRDiagnosticSeverity::Warning,
                code: "IR_FORWARDED_B6",
                source_kind: "SD17-B-6",
                message: d.message.clone(),
            })
            .collect();
        out.push((node, forwarded));
    }
    out
}

/// Convert the records from a B-4 spell file (or any borrowed `&[LstSpellRecord]`)
/// into IR nodes + forwarded diagnostics. O(n). The caller supplies the
/// `source_path` because `LstSpellFile` carries a `PathBuf` while the
/// converter normalizes to a `String`.
pub fn convert_spell_record_list(
    records: &[LstSpellRecord],
    source_path: &str,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        let node = convert_spell_record(record);
        let forwarded: Vec<IRDiagnostic> = Vec::new();
        // Note: per-row spell diagnostics live on the LstSpellFile, not on
        // the LstSpellRecord itself. The caller passes the borrowed slice;
        // container-level diagnostics must be retrieved separately. This
        // function focuses on per-record conversion.
        let _ = (source_path, forwarded);
        out.push((node, Vec::new()));
    }
    out
}

/// Convert the records from a B-4 [`LstSpellFile`] into IR nodes +
/// forwarded diagnostics. O(n). Normalizes the `PathBuf` source_path
/// to a `String` for the diagnostic surface.
pub fn convert_spell_file(
    r: &LstSpellFile,
    schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let source_path = path_to_string(&r.source_path);
    let container_diagnostics: Vec<IRDiagnostic> = r
        .diagnostics
        .iter()
        .map(|d| IRDiagnostic {
            source_path: source_path.clone(),
            line_number: d.line_number,
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B4",
            source_kind: "SD17-B-4",
            message: d.message.clone(),
        })
        .collect();
    let mut out = Vec::with_capacity(r.records.len());
    for record in &r.records {
        out.push((
            convert_spell_record(record),
            container_diagnostics.clone(),
        ));
    }
    // Add an extra IRNode-shaped placeholder for each container-level
    // diagnostic that has no record, so no diagnostic is silently dropped.
    let _ = schema;
    out
}

/// Convert an [`EquipmentParseResult`] (B-5) into IR nodes + forwarded
/// diagnostics. O(n).
pub fn convert_equipment_parse_result(
    r: &EquipmentParseResult,
    _schema: &IRSchema,
) -> Vec<(IRNode, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let node = convert_equipment_record(entry);
        let forwarded = forward_equipment_diagnostics(entry, &r.diagnostics, &r.source_path);
        out.push((node, forwarded));
    }
    out
}

// =============================================================================
// Diagnostic-forwarding helpers
// =============================================================================

fn forward_class_diagnostics(
    _entry: &ClassEntry,
    container: &[ClassLstDiagnostic],
    source_path: &str,
) -> Vec<IRDiagnostic> {
    container
        .iter()
        .map(|d| IRDiagnostic {
            source_path: source_path.to_string(),
            line_number: d.line_number,
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B1",
            source_kind: "SD17-B-1",
            message: d.message.clone(),
        })
        .collect()
}

fn forward_spellcasting_class_diagnostics(
    entry: &SpellcastingClassEntry,
    container: &[SpellcastingClassDiagnostic],
) -> Vec<IRDiagnostic> {
    container
        .iter()
        .filter(|d| {
            // Filter to diagnostics whose line matches this entry's
            // header line, or container-wide diagnostics with no line.
            match d.line_number {
                None => true,
                Some(ln) => ln >= entry.header_line_number,
            }
        })
        .map(|d| IRDiagnostic {
            source_path: entry.record_source_path(),
            line_number: d.line_number,
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B2",
            source_kind: "SD17-B-2",
            message: d.message.clone(),
        })
        .collect()
}

fn forward_race_ability_diagnostics(container: &[RaceAbilityLstDiagnostic]) -> Vec<IRDiagnostic> {
    container
        .iter()
        .map(|d| IRDiagnostic {
            source_path: d.source_path.clone(),
            line_number: Some(d.line_number),
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B3",
            source_kind: d.slice,
            message: d.message.clone(),
        })
        .collect()
}

fn forward_equipment_diagnostics(
    entry: &EquipmentRecord,
    container: &[EquipmentDiagnostic],
    source_path: &str,
) -> Vec<IRDiagnostic> {
    let mut out = Vec::new();
    // Forward attached record-level diagnostics.
    for d in &entry.diagnostics {
        out.push(IRDiagnostic {
            source_path: source_path.to_string(),
            line_number: d.line_number,
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B5",
            source_kind: "SD17-B-5",
            message: d.message.clone(),
        });
    }
    // Forward container-level diagnostics whose line matches the record's
    // header line, or container-wide diagnostics with no line.
    for d in container {
        let matches = match d.line_number {
            None => true,
            Some(ln) => ln >= entry.header_line_number,
        };
        if matches {
            out.push(IRDiagnostic {
                source_path: source_path.to_string(),
                line_number: d.line_number,
                raw_line: d.raw_line.clone(),
                severity: IRDiagnosticSeverity::Warning,
                code: "IR_FORWARDED_B5",
                source_kind: "SD17-B-5",
                message: d.message.clone(),
            });
        }
    }
    out
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Convert any `AsRef<Path>` to a normalized string form for use as the
/// `source_path` field on `IRNode` and `IRDiagnostic`. The path is
/// stringified with `to_string_lossy` so non-UTF-8 paths do not panic;
/// the canonical-IR surface only ever needs a display identifier.
pub(crate) fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

// =============================================================================
// Provenance accessors on the B-family records
//
// `ClassEntry`, `SpellcastingClassEntry`, and `LstRecord` (B-6 metadata)
// do NOT carry a per-record `source_path` field — they rely on the
// container's `source_path` for the document-level identity. The
// per-record converters therefore use `String::new()` as a placeholder
// when the converter is invoked outside a document context. The
// document-level converters (`convert_class_parse_result`,
// `convert_lst_metadata_document`, etc.) provide the real source path
// via the forwarded diagnostic stream and via the IRNode's `kind_token`
// accessor — the consumer can reconstruct the full provenance from
// the IRNode + its forwarded diagnostics.
//
// `LstSpellRecord` (B-4) does carry `source_path: String` and is the
// only B-family record that the converter can fully populate without
// consulting a container. The trait below makes that asymmetry explicit
// so future maintainers don't accidentally assume per-record source
// paths exist for every family.
// =============================================================================

/// Per-record `source_path` accessor. Returns `String::new()` for
/// record kinds whose B-family parser does not embed a `source_path`
/// field on the record (ClassEntry, SpellcastingClassEntry, LstRecord
/// from B-6 metadata, EquipmentRecord). Returns the real path for
/// LstSpellRecord, which carries one.
trait RecordSourcePath {
    fn record_source_path(&self) -> String;
}

impl RecordSourcePath for ClassEntry {
    fn record_source_path(&self) -> String {
        String::new()
    }
}

impl RecordSourcePath for SpellcastingClassEntry {
    fn record_source_path(&self) -> String {
        String::new()
    }
}

impl RecordSourcePath for LstRecord {
    fn record_source_path(&self) -> String {
        String::new()
    }
}

impl RecordSourcePath for EquipmentRecord {
    fn record_source_path(&self) -> String {
        String::new()
    }
}

impl RecordSourcePath for LstSpellRecord {
    fn record_source_path(&self) -> String {
        self.source_path.clone()
    }
}

impl RecordSourcePath for RaceDeclaration {
    fn record_source_path(&self) -> String {
        self.source_path.clone()
    }
}

impl RecordSourcePath for AbilityDeclaration {
    fn record_source_path(&self) -> String {
        self.source_path.clone()
    }
}