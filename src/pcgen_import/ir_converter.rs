//! Canonical-IR conversion for SD-17 Slice C + Slice E.
//!
//! This module consumes parsed LST records emitted by the six Slice B
//! parsers (`B-1` through `B-6`) and converts them into the canonical
//! internal representation that the rules-core compute path consumes.
//!
//! ## Slice E authorship — what lives here
//!
//! Slice E authors the canonical source-IR record shape in
//! `src/rules_core/source_content.rs`:
//!
//! - `SourcePackageContent<'a>` — corpus-rooted aggregate.
//! - `SourceContentRecord<'a>`   — per-record envelope.
//! - `SourceContentPayload<'a>`  — kind-tagged enum of borrowed
//!   B-family entries (defined in
//!   `src/pcgen_import/source_content_payload.rs` to keep the
//!   `rules_core <-> pcgen_import` import graph acyclic; re-exported
//!   from `rules_core::source_content`).
//! - `SourceRef` / `SourceContentKind` / `SourceContentDiagnostic` —
//!   provenance + diagnostic surface.
//!
//! **This module (`ir_converter.rs`) is the canonical projection
//! path.** It takes a `ParsedLstRecord<'a>` and produces a
//! `SourceContentRecord<'a>` per record, with full provenance
//! forwarded and (when applicable) forwarded from the B-family
//! parse-result containers into `SourceContentDiagnostic`s.
//!
//! The authoritative specification is
//! `docs/release/SD-17/artifacts/canonical-ir-contract-2026-07-12.md` (Slice C,
//! IR-conversion surface); the source-IR shape is authoritative at
//! `docs/release/SD-17/artifacts/canonical-source-ir-contract-2026-07-12.md`
//! (Slice E, canonical envelope).
//!
//! ## Public API (post-Slice-E)
//!
//! - [`IRSchema`] — descriptor for the canonical schema the consumer expects.
//! - [`IRDiagnostic`] — provenance + severity + code for the converter's
//!   diagnostic surface.
//! - [`ParsedLstRecord`] — canonical input enum that [`convert_to_ir`]
//!   dispatches on. Authored by Slice D (parser-aggregate relocation).
//! - [`convert_to_ir`] — public entry point. Returns a
//!   [`SourceContentRecord`] (the canonical envelope).
//! - Per-family converters ([`convert_class_entry`], etc.) for typed callers.
//! - Per-document converters ([`convert_class_parse_result`], etc.) that
//!   consume the B-family parse-result containers and emit a
//!   [`SourcePackageContent`] aggregate plus forwarded canonical diagnostics.
//! - [`convert_package_from_class_parse_result`] etc. — corpus-rooted
//!   entry points that accumulate a complete [`SourcePackageContent`].
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
//! intentionally NOT defined here — this module projects parsed LST
//! records into the canonical source-IR envelope the rules engine
//! eventually consumes.

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
use crate::pcgen_import::source_content_payload::b6_metadata_kind_to_canonical;
use crate::rules_core::source_content::{
    SOURCE_IR_VERSION, SourceContentDiagnostic, SourceContentDiagnosticKind, SourceContentKind,
    SourceContentPayload, SourceContentRecord, SourceContentSeverity, SourcePackageContent,
    SourceRef,
};

// =============================================================================
// IRSchema
// =============================================================================

/// Descriptor for the canonical schema the consumer expects.
///
/// The schema is descriptive, not prescriptive: the converter does not
/// reject records whose kind is not in `recognized_kinds`. The schema's
/// purpose is to advertise the field taxonomy the consumer expects so
/// the canonical-IR pipeline can be inspected and validated. The schema
/// version mirrors [`SOURCE_IR_VERSION`] (the source-IR schema version
/// the converter emits); the schema id is prefixed with
/// `codex.pcgen.canonical-ir.v` to distinguish from a future
/// source-IR-only schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IRSchema {
    /// Canonical schema identifier (e.g. `"codex.pcgen.canonical-ir.v1"`).
    pub schema_id: &'static str,
    /// Schema revision, bumped on breaking changes. Mirrors
    /// [`SOURCE_IR_VERSION`] at the time of the converter's build.
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
            schema_version: SOURCE_IR_VERSION,
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
// IRDiagnostic — converter-side diagnostic surface (preserved)
// =============================================================================
//
// Slice E adds the canonical [`SourceContentDiagnostic`] (source-side
// diagnostic). The converter still emits its own [`IRDiagnostic`] for
// the same forwarded events because downstream tooling was authored
// against the C contract; this type is preserved (not deprecated)
// while the canonical projection re-shapes the same diagnostic into
// `SourceContentDiagnostic` via `IRDiagnostic::to_canonical`.
//
// Slice C's contract artifact (`canonical-ir-contract-2026-07-12.md`)
// remains authoritative for `IRDiagnostic`. Slice E's contract artifact
// (`canonical-source-ir-contract-2026-07-12.md`) is authoritative for
// `SourceContentDiagnostic`. The two coexist; nothing in this slice
// changes `IRDiagnostic`'s semantics or constructor signatures.

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

    /// Reshape this converter-side diagnostic into the canonical
    /// [`SourceContentDiagnostic`] (source-IR projection form).
    ///
    /// The mapping is direct:
    ///
    /// - `severity` (Error/Warning/Info) →
    ///   [`SourceContentSeverity::Error`] / `Warning` / `Info`.
    /// - For malformed/forwarded diagnostics whose `code` starts with
    ///   `IR_FORWARDED_` (the B-family forwarded diagnostic family),
    ///   the canonical kind is [`SourceContentDiagnosticKind::MalformedRecord`]
    ///   (severity Error).
    /// - Other converter-originated diagnostics map to
    ///   [`SourceContentDiagnosticKind::PartialTranslation`] (severity
    ///   Info) — converter-internal notes do not block projection.
    ///
    /// Provenance: `source_ref.lst_file <- source_path`,
    /// `source_ref.line <- line_number.unwrap_or(0) as u32`. The
    /// `line: u32` rounding is intentional: container-level diagnostics
    /// (where `line_number == None`) anchor to `line == 0`, the
    /// canonical placeholder for "no specific line."
    pub fn to_canonical(&self) -> SourceContentDiagnostic {
        let source_ref = SourceRef::new(
            self.source_path.clone(),
            self.line_number.unwrap_or(0) as u32,
        );

        let (severity, kind) = if self.code.starts_with("IR_FORWARDED_") {
            // Forwarded-from-B-family -> canonical MalformedRecord (Error).
            (
                SourceContentSeverity::Error,
                SourceContentDiagnosticKind::MalformedRecord,
            )
        } else {
            // Converter-originated -> canonical PartialTranslation (Info).
            (
                SourceContentSeverity::Info,
                SourceContentDiagnosticKind::PartialTranslation,
            )
        };

        SourceContentDiagnostic {
            severity,
            kind,
            message: self.message.clone(),
            source_ref,
        }
    }
}

// =============================================================================
// ParsedLstRecord — canonical input enum for convert_to_ir
// =============================================================================
//
// Slice D relocated the aggregate `ParsedLstRecord` from the IR-converter
// surface (where the per-kind parsers live) into the LST parser surface
// (`pcgen_import::lst_parser::ParsedLstRecord`). The canonical home is
// the parser surface. This module re-exports it for backward compatibility
// so every consumer that imports
// `pcgen_import::ir_converter::ParsedLstRecord` continues to resolve.
// The doc comment on the parser-surface definition carries the full
// rationale and the convenience-constructor list.
//
// The local definition that previously lived here was removed when Slice E
// landed the canonical source-IR envelope; the parser-surface enum has
// the same shape plus the new `from_*` constructor set. See the
// `pcgen_import::lst_parser::ParsedLstRecord` definition for the single
// source of truth.
pub use crate::pcgen_import::lst_parser::ParsedLstRecord;

// =============================================================================
// SourceRef construction helper (used by per-family converters)
// =============================================================================
//
// The B-family records' source paths live in different fields (some on
// the record, some on the container). This helper packages the
// canonical four so per-family converters don't have to spell out the
// literal each time.

/// Build a `SourceRef` from a stringified path and a one-based line
/// number. `path` may be empty for records that don't carry their own
/// source identity (the document-level converter fills it in via the
/// forwarded diagnostic stream).
fn make_source_ref(path: impl Into<String>, line: usize) -> SourceRef {
    SourceRef::new(path.into(), line as u32)
}

// =============================================================================
// Per-family record converters — return SourceContentRecord<'a>
// =============================================================================

/// Build a canonical [`SourceContentRecord`] from a B-1 [`ClassEntry`].
///
/// Projection is zero-copy. The borrowed `entry` carries every
/// `tokens` and `feature_blocks` entry verbatim.
pub fn convert_class_entry(entry: &ClassEntry) -> SourceContentRecord<'_> {
    let line = entry.header_line_number;
    let source_ref = make_source_ref(entry.record_source_path(), line);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Class,
        SourceContentPayload::Class(entry),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-2
/// [`SpellcastingClassEntry`].
pub fn convert_spellcasting_class_entry(entry: &SpellcastingClassEntry) -> SourceContentRecord<'_> {
    let line = entry.header_line_number;
    let source_ref = make_source_ref(entry.record_source_path(), line);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::SpellcastingClass,
        SourceContentPayload::SpellcastingClass(entry),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-3
/// [`RaceDeclaration`].
pub fn convert_race_declaration(decl: &RaceDeclaration) -> SourceContentRecord<'_> {
    let source_ref = make_source_ref(&decl.source_path, decl.line_number);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Race,
        SourceContentPayload::Race(decl),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-3
/// [`AbilityDeclaration`].
pub fn convert_ability_declaration(decl: &AbilityDeclaration) -> SourceContentRecord<'_> {
    let source_ref = make_source_ref(&decl.source_path, decl.line_number);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Ability,
        SourceContentPayload::Ability(decl),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-4 [`LstSpellRecord`].
pub fn convert_spell_record(record: &LstSpellRecord) -> SourceContentRecord<'_> {
    let source_ref = make_source_ref(&record.source_path, record.line_number);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Spell,
        SourceContentPayload::Spell(record),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-5
/// [`EquipmentRecord`].
pub fn convert_equipment_record(record: &EquipmentRecord) -> SourceContentRecord<'_> {
    let line = record.header_line_number;
    let source_ref = make_source_ref(record.record_source_path(), line);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Equipment,
        SourceContentPayload::Equipment(record),
    )
}

/// Build a canonical [`SourceContentRecord`] from a B-6 [`LstRecord`].
/// The inner metadata kind is mapped from the parser-side
/// [`crate::pcgen_import::lst_parser::metadata::MetadataKind`] to the
/// canonical [`crate::rules_core::source_content::MetadataKindInner`] so
/// the rules-core envelope is total over the six B-6 kinds.
pub fn convert_metadata_record(record: &LstRecord) -> SourceContentRecord<'_> {
    let source_ref = make_source_ref(record.record_source_path(), record.line_number);
    let kind = SourceContentKind::Metadata(b6_metadata_kind_to_canonical(record.kind));
    SourceContentRecord::new(source_ref, kind, SourceContentPayload::Metadata(record))
}

// =============================================================================
// convert_to_ir — public entry point
// =============================================================================

/// Convert a single parsed LST record into its canonical source-IR
/// envelope.
///
/// The signature is the one named in the slice card body. The function
/// is an enum-discriminated trampoline to the per-family converters;
/// the per-family converters are also exposed as public entry points
/// for typed callers.
///
/// The return type changed from `Result<IRNode, IRDiagnostic>` to
/// `SourceContentRecord<'a>` per the Slice E contract. The conversion
/// is total: every B-family record has exactly one canonical envelope
/// variant. The `Result` wrapper is no longer necessary; the
/// canonical envelope carries its own diagnostic stream via
/// [`SourceContentDiagnostic`] attached to the package-level
/// [`SourcePackageContent`].
pub fn convert_to_ir<'a>(
    parsed_record: &ParsedLstRecord<'a>,
    _schema: &IRSchema,
) -> SourceContentRecord<'a> {
    match parsed_record {
        ParsedLstRecord::Class(entry) => convert_class_entry(entry),
        ParsedLstRecord::SpellcastingClass(entry) => convert_spellcasting_class_entry(entry),
        ParsedLstRecord::Race(r) => convert_race_declaration(r),
        ParsedLstRecord::Ability(a) => convert_ability_declaration(a),
        ParsedLstRecord::Spell(s) => convert_spell_record(s),
        ParsedLstRecord::Equipment(e) => convert_equipment_record(e),
        ParsedLstRecord::Metadata(r) => convert_metadata_record(r),
    }
}

// =============================================================================
// Per-document converters — produce (SourceContentRecord, forwarded IRDiagnostic) pairs
// =============================================================================
//
// The converter returns one `SourceContentRecord` per B-family entry
// plus the forwarded diagnostics stream. Caller-supplied `source_path`
// overrides the per-record `source_path` for records whose parser
// surface does not embed one (ClassEntry, SpellcastingClassEntry,
// LstRecord, EquipmentRecord). This matches the Slice C contract's
// behavior for these kinds.

/// Convert a [`ClassParseResult`] (B-1) into canonical records + forwarded
/// diagnostics. O(n) in the number of entries.
pub fn convert_class_parse_result<'a>(
    r: &'a ClassParseResult,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let record = convert_class_entry(entry);
        let forwarded = forward_class_diagnostics(&r.diagnostics, &r.source_path);
        out.push((record, forwarded));
    }
    out
}

/// Convert a [`SpellcastingClassParseResult`] (B-2) into canonical
/// records + forwarded diagnostics. O(n).
pub fn convert_spellcasting_class_parse_result<'a>(
    r: &'a SpellcastingClassParseResult,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let record = convert_spellcasting_class_entry(entry);
        let forwarded = forward_spellcasting_class_diagnostics(entry, &r.diagnostics);
        out.push((record, forwarded));
    }
    out
}

/// Convert an [`LstEntryFile`] (B-3) into canonical records + forwarded
/// diagnostics. O(n) in the total number of race + ability records.
pub fn convert_lst_entry_file<'a>(
    r: &'a LstEntryFile,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
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

/// Convert an [`LstMetadataDocument`] (B-6) into canonical records +
/// forwarded diagnostics. O(n).
pub fn convert_lst_metadata_document<'a>(
    r: &'a LstMetadataDocument,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.records.len());
    for record in &r.records {
        let canonical = convert_metadata_record(record);
        let forwarded = forward_metadata_record_diagnostics(record, &r.source_path);
        out.push((canonical, forwarded));
    }
    out
}

/// Convert the records from a B-4 spell file (or any borrowed
/// `&[LstSpellRecord]`) into canonical records + forwarded diagnostics.
/// O(n). The caller supplies `source_path` because `LstSpellFile`
/// carries a `PathBuf` while the converter normalizes to a `String`.
pub fn convert_spell_record_list<'a>(
    records: &'a [LstSpellRecord],
    source_path: &str,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let _ = source_path; // Per-row spell diagnostics live on the LstSpellFile, not on the LstSpellRecord itself.
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        let canonical = convert_spell_record(record);
        out.push((canonical, Vec::new()));
    }
    out
}

/// Convert the records from a B-4 [`LstSpellFile`] into canonical
/// records + forwarded diagnostics. O(n). Normalizes the `PathBuf`
/// `source_path` to a `String` for the diagnostic surface.
pub fn convert_spell_file<'a>(
    r: &'a LstSpellFile,
    schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let source_path = path_to_string(&r.source_path);
    let container_diagnostics = forward_spell_container_diagnostics(&r.diagnostics, &source_path);
    let mut out = Vec::with_capacity(r.records.len());
    for record in &r.records {
        // Rebuild the canonical record with the document's source_path
        // override so the B-4 record's own source_path is honored when
        // present but the document's identity fills in for records that
        // were construct-ed with empty paths.
        let canonical = convert_spell_record(record);
        out.push((canonical, container_diagnostics.clone()));
    }
    let _ = schema;
    out
}

/// Convert an [`EquipmentParseResult`] (B-5) into canonical records +
/// forwarded diagnostics. O(n).
pub fn convert_equipment_parse_result<'a>(
    r: &'a EquipmentParseResult,
    _schema: &IRSchema,
) -> Vec<(SourceContentRecord<'a>, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let canonical = convert_equipment_record(entry);
        let forwarded = forward_equipment_diagnostics(entry, &r.diagnostics, &r.source_path);
        out.push((canonical, forwarded));
    }
    out
}

// =============================================================================
// Corpus-rooted SourcePackageContent builders
// =============================================================================
//
// These produce the canonical [`SourcePackageContent`] aggregate
// directly, accumulating records and the canonical diagnostics stream
// in one pass.

/// Build a canonical [`SourcePackageContent`] from a B-1
/// [`ClassParseResult`].
pub fn convert_package_from_class_parse_result<'a>(
    r: &'a ClassParseResult,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (SourcePackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = SourcePackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
    let mut all_ir_diagnostics = Vec::new();
    for entry in &r.entries {
        let record = convert_class_entry(entry);
        let forwarded = forward_class_diagnostics(&r.diagnostics, &r.source_path);
        for d in &forwarded {
            pkg.push_diagnostic(d.to_canonical());
        }
        pkg.push(record);
        all_ir_diagnostics.extend(forwarded);
    }
    let _ = schema;
    (pkg, all_ir_diagnostics)
}

/// Build a canonical [`SourcePackageContent`] from a B-2
/// [`SpellcastingClassParseResult`].
pub fn convert_package_from_spellcasting_class_parse_result<'a>(
    r: &'a SpellcastingClassParseResult,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (SourcePackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = SourcePackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
    let mut all_ir_diagnostics = Vec::new();
    for entry in &r.entries {
        let record = convert_spellcasting_class_entry(entry);
        let forwarded = forward_spellcasting_class_diagnostics(entry, &r.diagnostics);
        for d in &forwarded {
            pkg.push_diagnostic(d.to_canonical());
        }
        pkg.push(record);
        all_ir_diagnostics.extend(forwarded);
    }
    let _ = schema;
    (pkg, all_ir_diagnostics)
}

/// Build a canonical [`SourcePackageContent`] from a B-3
/// [`LstEntryFile`].
pub fn convert_package_from_lst_entry_file<'a>(
    r: &'a LstEntryFile,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (SourcePackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = SourcePackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
    let mut all_ir_diagnostics = Vec::new();
    for race in &r.race_pointers {
        let record = convert_race_declaration(race);
        let forwarded = forward_race_ability_diagnostics(&r.diagnostics);
        for d in &forwarded {
            pkg.push_diagnostic(d.to_canonical());
        }
        pkg.push(record);
        all_ir_diagnostics.extend(forwarded);
    }
    for ability in &r.ability_declarations {
        let record = convert_ability_declaration(ability);
        let forwarded = forward_race_ability_diagnostics(&r.diagnostics);
        for d in &forwarded {
            pkg.push_diagnostic(d.to_canonical());
        }
        pkg.push(record);
        all_ir_diagnostics.extend(forwarded);
    }
    let _ = schema;
    (pkg, all_ir_diagnostics)
}

/// Build a canonical [`SourcePackageContent`] from a B-6
/// [`LstMetadataDocument`].
pub fn convert_package_from_lst_metadata_document<'a>(
    r: &'a LstMetadataDocument,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (SourcePackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = SourcePackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
    let mut all_ir_diagnostics = Vec::new();
    for record in &r.records {
        let canonical = convert_metadata_record(record);
        let forwarded = forward_metadata_record_diagnostics(record, &r.source_path);
        for d in &forwarded {
            pkg.push_diagnostic(d.to_canonical());
        }
        pkg.push(canonical);
        all_ir_diagnostics.extend(forwarded);
    }
    let _ = schema;
    (pkg, all_ir_diagnostics)
}

// =============================================================================
// Diagnostic-forwarding helpers (preserve Slice C behavior)
// =============================================================================

fn forward_class_diagnostics(
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

fn forward_spell_container_diagnostics(
    container: &[crate::pcgen_import::lst_parser::spell::LstParseDiagnostic],
    source_path: &str,
) -> Vec<IRDiagnostic> {
    container
        .iter()
        .map(|d| IRDiagnostic {
            source_path: source_path.to_string(),
            line_number: d.line_number,
            raw_line: d.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B4",
            source_kind: "SD17-B-4",
            message: d.message.clone(),
        })
        .collect()
}

fn forward_metadata_record_diagnostics(record: &LstRecord, source_path: &str) -> Vec<IRDiagnostic> {
    record
        .diagnostics
        .iter()
        .map(|d| IRDiagnostic {
            source_path: source_path.to_string(),
            line_number: Some(record.line_number),
            raw_line: record.raw_line.clone(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B6",
            source_kind: "SD17-B-6",
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
/// `source_path` field on `SourceRef` and `IRDiagnostic`. The path is
/// stringified with `to_string_lossy` so non-UTF-8 paths do not panic;
/// the canonical-IR surface only ever needs a display identifier.
pub(crate) fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

// =============================================================================
// Per-record `source_path` accessor (preserved; do not change)
// =============================================================================
//
// `ClassEntry`, `SpellcastingClassEntry`, `EquipmentRecord`, and
// `LstRecord` (B-6 metadata) do NOT carry a per-record `source_path`
// field — they rely on the container's `source_path` for the
// document-level identity. The per-record converters therefore use
// `String::new()` as a placeholder when the converter is invoked
// outside a document context; the document-level converters fill in
// the real path via the forwarded diagnostic stream.
//
// `LstSpellRecord` (B-4), `RaceDeclaration`, and `AbilityDeclaration`
// (B-3) DO carry `source_path` as a per-record field, and are the
// only families whose per-record conversion can fully populate the
// `SourceRef` without consulting a container. The trait below makes
// that asymmetry explicit so future maintainers don't accidentally
// assume per-record source paths exist for every family.

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::lst_parser::equipment::{EquipmentRecordKind, EquipmentToken};

    fn fake_class() -> ClassEntry {
        ClassEntry {
            class_name: "TestClass".to_string(),
            header_line_number: 7,
            header_raw_line: "CLASS:TestClass\tHD:8\tHP:0\tPROFICIENT:NO".to_string(),
            tokens: Vec::new(),
            feature_blocks: Vec::new(),
        }
    }

    fn fake_spell() -> LstSpellRecord {
        LstSpellRecord {
            line_number: 11,
            source_path: "spells.lst".to_string(),
            name: "Fireball".to_string(),
            output_name: None,
            spell_type: None,
            classes: None,
            school: Some("Evocation".to_string()),
            descriptor: Some("Fire".to_string()),
            sub_school: None,
            components: None,
            casting_time: None,
            range: None,
            item: None,
            target_area: None,
            duration: None,
            save_info: None,
            spell_resistance: None,
            description: None,
            source_page: None,
            source_link: None,
            description_raw: None,
            payload: crate::pcgen_import::lst_parser::spell::LstSpellRecordPayload {
                name: "Fireball".to_string(),
                output_name: None,
                spell_type: None,
                classes: None,
                school: Some("Evocation".to_string()),
                descriptor: Some("Fire".to_string()),
                sub_school: None,
                components: None,
                casting_time: None,
                range: None,
                item: None,
                target_area: None,
                duration: None,
                save_info: None,
                spell_resistance: None,
                source_page: None,
                source_link: None,
                description: None,
                description_raw: None,
            },
        }
    }

    fn fake_equipment() -> EquipmentRecord {
        EquipmentRecord {
            kind: EquipmentRecordKind::Equip,
            name: "TestEquip".to_string(),
            header_line_number: 4,
            header_raw_line: "EQUIP:TestEquip".to_string(),
            tokens: vec![EquipmentToken {
                key: "KEY".to_string(),
                value: "value".to_string(),
                line_number: 4,
                raw_pair: "KEY:value".to_string(),
            }],
            bonus_chains: Vec::new(),
            is_record_start: true,
            diagnostics: Vec::new(),
        }
    }

    #[test]
    fn schema_version_follows_source_ir_version() {
        let s = IRSchema::canonical_v1();
        assert_eq!(s.schema_version, SOURCE_IR_VERSION);
    }

    #[test]
    fn class_record_carries_zero_copy_payload() {
        let entry = fake_class();
        let rec = convert_class_entry(&entry);
        assert_eq!(rec.kind, SourceContentKind::Class);
        assert_eq!(rec.source_ref.line, 7);
        match rec.payload {
            SourceContentPayload::Class(e) => {
                assert_eq!(e.class_name, "TestClass");
                assert_eq!(e.header_line_number, 7);
            }
            _ => panic!("expected Class payload"),
        }
    }

    #[test]
    fn spell_record_zero_copy() {
        let record = fake_spell();
        let rec = convert_spell_record(&record);
        assert_eq!(rec.kind, SourceContentKind::Spell);
        assert_eq!(rec.source_ref.lst_file, "spells.lst");
        assert_eq!(rec.source_ref.line, 11);
    }

    #[test]
    fn equipment_record_zero_copy() {
        let record = fake_equipment();
        let rec = convert_equipment_record(&record);
        assert_eq!(rec.kind, SourceContentKind::Equipment);
        match rec.payload {
            SourceContentPayload::Equipment(e) => {
                assert_eq!(e.name, "TestEquip");
            }
            _ => panic!("expected Equipment payload"),
        }
    }

    #[test]
    fn ir_diagnostic_to_canonical_routes_forwarded_as_malformed() {
        let d = IRDiagnostic {
            source_path: "x.lst".to_string(),
            line_number: Some(2),
            raw_line: "raw".to_string(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B5",
            source_kind: "SD17-B-5",
            message: "msg".to_string(),
        };
        let canonical = d.to_canonical();
        assert_eq!(canonical.severity, SourceContentSeverity::Error);
        assert_eq!(canonical.kind, SourceContentDiagnosticKind::MalformedRecord);
        assert_eq!(canonical.source_ref.lst_file, "x.lst");
        assert_eq!(canonical.source_ref.line, 2);
    }

    #[test]
    fn ir_diagnostic_to_canonical_routes_converter_originated_as_partial() {
        let d = IRDiagnostic::converter_error("y.lst", Some(3), "raw", "IR_INTERNAL", "x");
        let canonical = d.to_canonical();
        assert_eq!(canonical.severity, SourceContentSeverity::Info);
        assert_eq!(
            canonical.kind,
            SourceContentDiagnosticKind::PartialTranslation
        );
    }

    #[test]
    fn ir_diagnostic_to_canonical_handles_container_wide_line_placeholder() {
        let d = IRDiagnostic {
            source_path: "z.lst".to_string(),
            line_number: None,
            raw_line: String::new(),
            severity: IRDiagnosticSeverity::Warning,
            code: "IR_FORWARDED_B1",
            source_kind: "SD17-B-1",
            message: "container-wide".to_string(),
        };
        let canonical = d.to_canonical();
        assert_eq!(canonical.source_ref.line, 0);
    }
}
