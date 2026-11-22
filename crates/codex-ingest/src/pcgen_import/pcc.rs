//! PCC entry-file parse shape (GE03-E1-F1).
//!
//! This is the smallest parser slice that turns a PCC entry-file text input
//! into a structured parse result while preserving the provenance the GE-02
//! importer dependency contract requires for include surfaces:
//!
//! - source PCC file identity
//! - `PCC:` include directives as structural include edges
//! - one-based source line numbers for each include edge
//! - raw include-directive evidence
//! - diagnosable malformed PCC include lines
//!
//! Scope boundary: this slice does not parse or semantically interpret LST
//! object declarations (e.g. `CLASS:cr_classes.lst`), and it does not build the
//! broader diagnostics, token-registry, or source-map systems.

/// Structured result of parsing a single PCC entry file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PccEntryFile {
    /// Identity of the parsed PCC source (as supplied by the caller).
    pub source_path: String,
    /// Include edges discovered in source order.
    pub includes: Vec<PccIncludeEdge>,
    /// Parse-shape diagnostics for malformed include directives.
    pub diagnostics: Vec<PccDiagnostic>,
}

/// A single `PCC:` include directive represented as a structural edge from the
/// parsed source file to an included target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PccIncludeEdge {
    /// Identity of the PCC source file this edge originates from.
    pub source_path: String,
    /// One-based line number of the directive in the parsed input.
    pub line_number: usize,
    /// The full raw directive line, preserved verbatim as evidence.
    pub raw_directive: String,
    /// The include target text following `PCC:`, preserved exactly enough to
    /// distinguish forward-slash and backslash path styles.
    pub target: String,
}

/// Classification of a PCC parse-shape diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PccDiagnosticKind {
    /// A `PCC:` directive with no include target.
    MalformedInclude,
}

/// A first-class diagnostic record for a malformed PCC include line.
///
/// Diagnostics are structured records, not prose printed to stdout/stderr, so
/// malformed directives are surfaced rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PccDiagnostic {
    /// Identity of the PCC source file the diagnostic refers to.
    pub source_path: String,
    /// One-based line number of the offending line.
    pub line_number: usize,
    /// The full raw line, preserved verbatim as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: PccDiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
}

/// Parse a PCC entry file into a structured [`PccEntryFile`].
///
/// `source_path` is the identity recorded on every include edge and diagnostic;
/// `input_text` is the raw entry-file contents.
pub fn parse_pcc_entry(source_path: &str, input_text: &str) -> PccEntryFile {
    let mut includes = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        // Ignore blank lines and comment lines beginning with `#`.
        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        // Recognize include directives beginning with `PCC:`; every other
        // construct (e.g. `CLASS:`/`RACE:`/`SKILL:` LST declarations) is left
        // for later GE-03 slices and intentionally ignored here.
        let Some(rest) = content.strip_prefix("PCC:") else {
            continue;
        };

        let target = rest.trim();
        if target.is_empty() {
            // A `PCC:` directive with no target is malformed: surface it as a
            // structured diagnostic rather than dropping it silently.
            diagnostics.push(PccDiagnostic {
                source_path: source_path.to_string(),
                line_number,
                raw_line: raw_line.to_string(),
                kind: PccDiagnosticKind::MalformedInclude,
                message: "PCC include directive has no target".to_string(),
            });
        } else {
            includes.push(PccIncludeEdge {
                source_path: source_path.to_string(),
                line_number,
                raw_directive: raw_line.to_string(),
                target: target.to_string(),
            });
        }
    }

    PccEntryFile {
        source_path: source_path.to_string(),
        includes,
        diagnostics,
    }
}
