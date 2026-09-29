//! LST metadata-kind parser for SD-17 Slice B-6.
//!
//! Scope: this module recognizes the six metadata object kinds the operator
//! directive names — `DEITY:`, `DOMAIN:`, `KITS:`, `LANGUAGE:`, `TEMPLATE:`,
//! and `COMPANIONMOD:` — wherever they appear at the start of a non-comment
//! line in a PCGen LST file. Each occurrence is captured as a structured
//! record with the source line number preserved, the directive kind tagged,
//! the value's leading token recorded as the record name, and a malformed
//! diagnostic emitted when the directive has no value.
//!
//! PCGen semantics: each of these six object kinds is a top-level list
//! object. In real corpus files (e.g. `cr_languages.lst`, `cr_templates.lst`,
//! `cr_companionmods.lst`, `cr_kits.lst`) every `LANGUAGE:`, `TEMPLATE:`,
//! `COMPANIONMOD:`, or `KITS:` directive at the start of a line is a distinct
//! record — records are not grouped by blank lines for these kinds. The
//! parser therefore reports each recognized metadata-kind directive as a
//! record-start; the `is_record_start` field is always `true` for records
//! produced by this module and is retained for API symmetry with the
//! future multi-kind LST parser.
//!
//! Scope boundary: this slice does not parse LST directives other than the
//! six metadata kinds named above. CLASS:, SPELL:, ABILITY:, FEAT:, SKILL:,
//! RACE:, EQUIP:, and other LST record kinds are intentionally left to later
//! SD-17 slice-B epics. Likewise, this slice does not interpret the value
//! grammar (e.g. `DOMAIN:Evil|Strength`) beyond recording the leading token
//! as the record's name.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// One of the six metadata object kinds SD-17 Slice B-6 owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetadataKind {
    Deity,
    Domain,
    Kits,
    Language,
    Template,
    CompanionMod,
}

impl MetadataKind {
    /// Canonical PCGen directive token (upper-case, including the trailing `:`).
    pub fn token(self) -> &'static str {
        match self {
            MetadataKind::Deity => "DEITY",
            MetadataKind::Domain => "DOMAIN",
            MetadataKind::Kits => "KITS",
            MetadataKind::Language => "LANGUAGE",
            MetadataKind::Template => "TEMPLATE",
            MetadataKind::CompanionMod => "COMPANIONMOD",
        }
    }

    /// Map a canonical token (without the trailing `:`) to a [`MetadataKind`].
    /// Returns `None` for unknown tokens or non-canonical casing — the parser
    /// is case-sensitive on the directive prefix to match PCGen's convention.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "DEITY" => Some(MetadataKind::Deity),
            "DOMAIN" => Some(MetadataKind::Domain),
            "KITS" => Some(MetadataKind::Kits),
            "LANGUAGE" => Some(MetadataKind::Language),
            "TEMPLATE" => Some(MetadataKind::Template),
            "COMPANIONMOD" => Some(MetadataKind::CompanionMod),
            _ => None,
        }
    }
}

/// Diagnostic kinds emitted by the metadata parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LstDiagnosticKind {
    /// A recognized metadata directive was found with no value (e.g. `DEITY:`).
    MalformedDirective,
}

/// A structured diagnostic record for an LST metadata parse problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstDiagnostic {
    /// The directive kind that produced the diagnostic.
    pub kind: LstDiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
}

/// A single recognized metadata-kind occurrence in an LST file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstRecord {
    /// Which metadata kind this record represents.
    pub kind: MetadataKind,
    /// The leading token of the directive value, with surrounding whitespace
    /// stripped. Empty when the directive had no value.
    pub name: String,
    /// One-based source line number where the directive appeared.
    pub line_number: usize,
    /// The full raw line text, preserved verbatim as evidence.
    pub raw_line: String,
    /// Always `true` for records emitted by this parser: every recognized
    /// metadata-kind directive at line start is its own record in PCGen's
    /// grammar for these six kinds. Retained for API symmetry with the
    /// future multi-kind LST parser.
    pub is_record_start: bool,
    /// Diagnostics attached to this record (e.g. malformed value).
    pub diagnostics: Vec<LstDiagnostic>,
}

/// Result of parsing an LST file's metadata-kind surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstMetadataDocument {
    /// Identity of the source file as supplied by the caller.
    pub source_path: String,
    /// All recognized metadata-kind records, in source order.
    pub records: Vec<LstRecord>,
}

/// Parse an LST file on disk and return its metadata-kind surface.
pub fn parse_lst_metadata(source_path: &Path) -> Result<LstMetadataDocument, std::io::Error> {
    let input_text = fs::read_to_string(source_path)?;
    Ok(parse_lst_metadata_text(
        &source_path.to_string_lossy(),
        &input_text,
    ))
}

/// Parse the text of an LST file and return its metadata-kind surface.
///
/// `source_path` is recorded as the document's identity. The parser walks the
/// input one line at a time; blank lines and comment lines (lines whose
/// first non-whitespace character is `#`) are skipped. Any line whose first
/// token (followed by `:`) is one of the six recognized metadata kinds
/// becomes an `LstRecord`. Every recognized metadata directive at line start
/// is reported as a record-start.
///
/// Performance: O(n) in the number of input lines. The parser allocates
/// exactly one record per recognized line and never rescans earlier lines.
pub fn parse_lst_metadata_text(source_path: &str, input_text: &str) -> LstMetadataDocument {
    let mut records: Vec<LstRecord> = Vec::new();

    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Pull the leading token up to the first `:`. If the line does not
        // contain a `:`, it's not a directive line and the parser ignores it.
        let Some((directive, value_with_pipe)) = trimmed.split_once(':') else {
            continue;
        };

        let Some(kind) = MetadataKind::from_token(directive) else {
            continue;
        };

        let raw_value = value_with_pipe.split('|').next().unwrap_or("").trim();
        let mut diagnostics = Vec::new();
        if raw_value.is_empty() {
            diagnostics.push(LstDiagnostic {
                kind: LstDiagnosticKind::MalformedDirective,
                message: format!("{} directive has no value", kind.token()),
            });
        }

        records.push(LstRecord {
            kind,
            name: raw_value.to_string(),
            line_number,
            raw_line: raw_line.to_string(),
            is_record_start: true,
            diagnostics,
        });
    }

    LstMetadataDocument {
        source_path: source_path.to_string(),
        records,
    }
}

impl LstMetadataDocument {
    /// Group records by [`MetadataKind`] in source order. Useful for
    /// acceptance tests and downstream code that consumes a single kind at a
    /// time.
    pub fn records_by_kind(&self) -> HashMap<MetadataKind, Vec<&LstRecord>> {
        let mut grouped: HashMap<MetadataKind, Vec<&LstRecord>> = HashMap::new();
        for record in &self.records {
            grouped.entry(record.kind).or_default().push(record);
        }
        grouped
    }
}
