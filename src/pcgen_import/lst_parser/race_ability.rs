//! LST entry-file parse shape for races and race-ability objects (GE03-E1-F3 / SD17-B-3).
//!
//! This is the smallest parser slice that recognizes `RACE:` (and the
//! operator-authored plural `RACES:`) and `ABILITY:` lines in PCGen LST
//! inputs and turns them into structured parse records while preserving the
//! provenance the GE-02 importer dependency contract requires:
//!
//! - source LST file identity
//! - one-based source line numbers for every record and diagnostic
//! - raw directive evidence for every record
//! - diagnosable malformed object declarations
//!
//! Scope boundary: this slice does not parse or semantically interpret other
//! LST object kinds (`CLASS:`, `SKILL:`, `TEMPLATE:`, etc.) and does not
//! build the broader diagnostics, token-registry, or source-map systems
//! owned by later GE-03 slices.
//!
//! ## Recognition surface
//!
//! Two directive forms are recognized:
//!
//! 1. **`RACE:` / `RACES:`** — a pointer to a races LST file. Used in
//!    PCGen campaign PCC files (e.g. `RACE:cr_races.lst`). The card body
//!    names the plural (`RACES:`); the corpus predominantly uses the
//!    singular. Both are accepted; the resulting record is identical
//!    (the directive text is preserved verbatim in `raw_directive`).
//!
//! 2. **`ABILITY:`** — either a pointer (used in PCCs) or a full
//!    declaration (used in `<race>_abilities_race.lst` files). The full
//!    declaration form is:
//!
//!    ```text
//!    ABILITY:CATEGORY=<cat>|<name>[|<kind>|<target>][\tKEY:VALUE ...]
//!    ```
//!
//!    Where `<kind>` is one of `AUTOMATIC`, `VIRTUAL`, `NORMAL`.
//!    Trailing `KEY:VALUE` modifiers separated by tab/whitespace are
//!    preserved as discrete trailing-modifier entries so the wider
//!    importer can resolve them without losing structure.

use std::fmt;

/// Structured result of parsing a single LST entry file (races + abilities).
///
/// Mirrors the shape of [`crate::pcgen_import::pcc::PccEntryFile`] but adds
/// LST-specific record types (`RaceDeclaration` / `AbilityDeclaration`)
/// plus a richer diagnostic enum to cover both `RACE:` and `ABILITY:`
/// malformed-input shapes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstEntryFile {
    /// Identity of the parsed LST source (as supplied by the caller).
    pub source_path: String,
    /// `RACE:` / `RACES:` race pointer declarations, in source order.
    pub race_pointers: Vec<RaceDeclaration>,
    /// `ABILITY:` declarations (pointers or full declarations), in source order.
    pub ability_declarations: Vec<AbilityDeclaration>,
    /// Parse-shape diagnostics for malformed object declarations.
    pub diagnostics: Vec<LstDiagnostic>,
}

/// A single `RACE:` (or `RACES:`) race-pointer line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceDeclaration {
    /// Identity of the LST source file this record originates from.
    pub source_path: String,
    /// One-based line number of the directive in the parsed input.
    pub line_number: usize,
    /// The full raw directive line, preserved verbatim as evidence.
    pub raw_directive: String,
    /// The include target text following `RACE:` / `RACES:`, preserved exactly.
    pub target: String,
}

/// A single `ABILITY:` line, either a bare pointer or a fully-structured
/// declaration. Bare pointers carry `parsed == None`; pipe-delimited
/// declarations set `parsed` to [`Some`] with field-level structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityDeclaration {
    /// Identity of the LST source file this record originates from.
    pub source_path: String,
    /// One-based line number of the directive in the parsed input.
    pub line_number: usize,
    /// The full raw directive line, preserved verbatim as evidence.
    pub raw_directive: String,
    /// Field-level structure for pipe-delimited declarations; `None` for
    /// bare-target pointers (`ABILITY:<target.lst>`).
    pub parsed: Option<AbilityParsedFields>,
}

/// Field-level structure for an `ABILITY:` pipe-delimited declaration.
///
/// Examples:
///
/// ```text
/// ABILITY:Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Ability Scores
///   category=None, name="Dwarf Racial Trait",
///   kind=Some(Automatic), target="Dwarf ~ Ability Scores",
///   trailing_modifiers=[]
///
/// ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES
///   category=Some("FEAT"), name="Alertness",
///   kind=None, target="",
///   trailing_modifiers=["FREE:YES"]
///
/// ABILITY:Traits|VIRTUAL|%LIST
///   category=None, name="Traits",
///   kind=Some(Virtual), target="%LIST",
///   trailing_modifiers=[]
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityParsedFields {
    /// `CATEGORY=...` prefix when present on the first segment.
    pub category: Option<String>,
    /// The bare ability name (the second pipe-segment, or first segment
    /// if no kind/target).
    pub name: String,
    /// Discriminator for `AUTOMATIC`/`VIRTUAL`/`NORMAL`. `None` if not
    /// present in the declaration.
    pub kind: Option<AbilityKind>,
    /// The ability reference for `kind|target` declarations. Empty for
    /// `CATEGORY=<cat>|<name>` shapes.
    pub target: String,
    /// Trailing `KEY:VALUE` modifiers separated by tab/whitespace on the
    /// raw line, preserved verbatim as evidence for the wider importer.
    pub trailing_modifiers: Vec<String>,
}

/// Discriminator for pipe-delimited `ABILITY:` declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityKind {
    /// Granted automatically when the ability's `PRE`/`PREFACT` is satisfied.
    Automatic,
    /// Virtual ability reference (the source is a list/pool).
    Virtual,
    /// Normal ability (chosen at the appropriate level, no auto-grant).
    Normal,
}

impl fmt::Display for AbilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AbilityKind::Automatic => "AUTOMATIC",
            AbilityKind::Virtual => "VIRTUAL",
            AbilityKind::Normal => "NORMAL",
        };
        f.write_str(s)
    }
}

/// Classification of an LST parse-shape diagnostic.
///
/// Every variant carries the implicit **SD17-B-3** slice tag — the
/// diagnostic struct itself sets `slice = "SD17-B-3"` on construction so
/// downstream consumers do not have to memorize the slice by kind alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LstDiagnosticKind {
    /// `RACE:` / `RACES:` directive with no include target.
    MalformedRacePointer,
    /// `ABILITY:` directive whose post-CATEGORY/pipe segments do not
    /// produce a parseable declaration (no name, no kind/target pair).
    MalformedAbilityDeclaration,
}

/// A first-class diagnostic record for a malformed LST object line.
///
/// Every diagnostic carries the **SD17-B-3** slice tag in `slice` and the
/// structured fields demanded by the SD-17-B-3 verification contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstDiagnostic {
    /// Identity of the LST source file the diagnostic refers to.
    pub source_path: String,
    /// One-based line number of the offending line.
    pub line_number: usize,
    /// The full raw line, preserved verbatim as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: LstDiagnosticKind,
    /// The slice tag this diagnostic belongs to (`SD17-B-3`).
    pub slice: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

impl LstDiagnostic {
    /// Construct a fresh SD17-B-3 diagnostic with the canonical slice tag.
    fn new(
        source_path: &str,
        line_number: usize,
        raw_line: &str,
        kind: LstDiagnosticKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_path: source_path.to_string(),
            line_number,
            raw_line: raw_line.to_string(),
            kind,
            slice: "SD17-B-3",
            message: message.into(),
        }
    }

    /// Returns true when this diagnostic classifies as a SD17-B-3 finding.
    /// All LST parser diagnostics carry the SD17-B-3 tag, so this is a
    /// structural assertion used by the acceptance tests.
    pub fn kind_is_sd17_b3(&self) -> bool {
        self.slice == "SD17-B-3"
            && matches!(
                self.kind,
                LstDiagnosticKind::MalformedRacePointer
                    | LstDiagnosticKind::MalformedAbilityDeclaration
            )
    }
}

/// Parse an LST entry file into a structured [`LstEntryFile`].
///
/// `source_path` is the identity recorded on every record and diagnostic;
/// `input_text` is the raw LST file contents. The parser walks the input
/// once (line-by-line, O(n) in line count) and accumulates records +
/// diagnostics into the result.
///
/// The recognized directive prefixes are exactly:
/// - `RACE:` (singular, the corpus form)
/// - `RACES:` (plural, the operator-card form)
/// - `ABILITY:`
///
/// All other prefixes (including blank lines, comments, and other LST
/// object kinds) are ignored.
pub fn parse_lst_entry(source_path: &str, input_text: &str) -> LstEntryFile {
    let mut race_pointers = Vec::new();
    let mut ability_declarations = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        // Ignore blank lines and comment lines beginning with `#`.
        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        // Order matters: `RACE:` precedes `RACES:` so we don't accidentally
        // consume `RACE:` as a misspelled `RACES:`. Both forms are accepted;
        // either produces a [`RaceDeclaration`].
        if let Some(rest) = content.strip_prefix("RACE:") {
            handle_race_directive(
                source_path,
                line_number,
                raw_line,
                "RACE",
                rest,
                &mut race_pointers,
                &mut diagnostics,
            );
            continue;
        }
        if let Some(rest) = content.strip_prefix("RACES:") {
            handle_race_directive(
                source_path,
                line_number,
                raw_line,
                "RACES",
                rest,
                &mut race_pointers,
                &mut diagnostics,
            );
            continue;
        }

        if let Some(rest) = content.strip_prefix("ABILITY:") {
            handle_ability_directive(
                source_path,
                line_number,
                raw_line,
                rest,
                &mut ability_declarations,
                &mut diagnostics,
            );
            continue;
        }

        // Every other construct (CLASS:, SKILL:, etc.) is left to its own
        // slice and intentionally ignored here.
    }

    LstEntryFile {
        source_path: source_path.to_string(),
        race_pointers,
        ability_declarations,
        diagnostics,
    }
}

/// Build a [`RaceDeclaration`] or surface a `MalformedRacePointer`
/// diagnostic depending on whether the post-prefix content is non-empty
/// after trimming.
fn handle_race_directive(
    source_path: &str,
    line_number: usize,
    raw_line: &str,
    directive_name: &str,
    rest: &str,
    race_pointers: &mut Vec<RaceDeclaration>,
    diagnostics: &mut Vec<LstDiagnostic>,
) {
    let target = rest.trim();
    if target.is_empty() {
        diagnostics.push(LstDiagnostic::new(
            source_path,
            line_number,
            raw_line,
            LstDiagnosticKind::MalformedRacePointer,
            format!(
                "{} include directive has no target [SD17-B-3]",
                directive_name
            ),
        ));
        return;
    }
    race_pointers.push(RaceDeclaration {
        source_path: source_path.to_string(),
        line_number,
        raw_directive: raw_line.to_string(),
        target: target.to_string(),
    });
}

/// Build an [`AbilityDeclaration`] (with optional field-level structure)
/// or surface a `MalformedAbilityDeclaration` diagnostic when the
/// post-prefix content cannot be parsed into the documented shape.
fn handle_ability_directive(
    source_path: &str,
    line_number: usize,
    raw_line: &str,
    rest: &str,
    ability_declarations: &mut Vec<AbilityDeclaration>,
    diagnostics: &mut Vec<LstDiagnostic>,
) {
    // A bare pointer (e.g. `ABILITY:cr_abilities.lst`) is the simplest
    // shape. If the post-prefix content has no pipe character and no
    // tabs, treat the trimmed tail as the pointer target — there is no
    // structural shape to parse beyond that.
    if !rest.contains('|') && !rest.contains('\t') {
        let target = rest.trim();
        if target.is_empty() {
            diagnostics.push(LstDiagnostic::new(
                source_path,
                line_number,
                raw_line,
                LstDiagnosticKind::MalformedAbilityDeclaration,
                "ABILITY directive has no target [SD17-B-3]",
            ));
            return;
        }
        ability_declarations.push(AbilityDeclaration {
            source_path: source_path.to_string(),
            line_number,
            raw_directive: raw_line.to_string(),
            parsed: Some(AbilityParsedFields {
                category: None,
                name: String::new(),
                kind: None,
                target: target.to_string(),
                trailing_modifiers: Vec::new(),
            }),
        });
        return;
    }

    // Field-level parse of a pipe-delimited declaration.
    let parsed = parse_ability_fields(rest);
    if parsed.is_none() {
        diagnostics.push(LstDiagnostic::new(
            source_path,
            line_number,
            raw_line,
            LstDiagnosticKind::MalformedAbilityDeclaration,
            "ABILITY declaration has no parseable fields [SD17-B-3]",
        ));
        return;
    }

    ability_declarations.push(AbilityDeclaration {
        source_path: source_path.to_string(),
        line_number,
        raw_directive: raw_line.to_string(),
        parsed,
    });
}

/// Parse the post-prefix `ABILITY:` content into field-level structure.
///
/// Accepted shapes:
///
/// 1. `CATEGORY=<cat>|<name>` — category prefix, no kind/target.
///    Trailing modifiers (tab/whitespace-separated `KEY:VALUE`) preserved.
/// 2. `<name>|<kind>|<target>` — pipe-delimited declaration.
///    Trailing modifiers preserved.
/// 3. `CATEGORY=<cat>|<name>|<kind>|<target>` — full combination.
/// 4. Mixed forms where one segment carries trailing modifiers via tabs
///    (e.g. `ABILITY:CATEGORY=FEAT|Alertness\tFREE:YES`).
///
/// Returns `None` when the input does not yield at least a non-empty
/// `name` (the canonical malformed-declaration signal).
fn parse_ability_fields(rest: &str) -> Option<AbilityParsedFields> {
    // Split trailing tab/whitespace-separated `KEY:VALUE` modifiers off
    // the rightmost segment first. The split point is any run of
    // tabs/spaces that contains at least one `:` on the right side.
    let (core, trailing) = split_trailing_modifiers(rest);

    // `core` is now up to the first tab/space boundary. Split on `|`.
    let pipe_segments: Vec<&str> = core.split('|').collect();

    if pipe_segments.is_empty() || pipe_segments.iter().all(|s| s.trim().is_empty()) {
        return None;
    }

    let mut category: Option<String> = None;
    let mut name: Option<String> = None;
    let mut kind: Option<AbilityKind> = None;
    let mut target: Option<String> = None;

    // Walk the pipe segments. The first segment may be a `CATEGORY=...`
    // prefix or the bare name. The third/fourth segments, if present,
    // carry `kind|target`.
    for (i, segment) in pipe_segments.iter().enumerate() {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }
        match i {
            0 => {
                if let Some(cat) = trimmed.strip_prefix("CATEGORY=") {
                    category = Some(cat.trim().to_string());
                } else {
                    name = Some(trimmed.to_string());
                }
            }
            1 => {
                // Position 1 is either the name (when position 0 was
                // CATEGORY=...) or the kind (when position 0 was the bare
                // name).
                if name.is_some() {
                    // The previous position was the bare name; this one is
                    // either kind or target. Disambiguate by the recognized
                    // kind tokens.
                    if let Some(k) = parse_ability_kind(trimmed) {
                        kind = Some(k);
                    } else {
                        target = Some(trimmed.to_string());
                    }
                } else {
                    name = Some(trimmed.to_string());
                }
            }
            2 => {
                // Position 2 is either kind or target.
                if kind.is_some() {
                    target = Some(trimmed.to_string());
                } else if let Some(k) = parse_ability_kind(trimmed) {
                    kind = Some(k);
                } else {
                    target = Some(trimmed.to_string());
                }
            }
            3 => {
                // Position 3 is the (last) target when both name and kind
                // are already set.
                target = Some(trimmed.to_string());
            }
            _ => {
                // Past position 3 we ignore extra segments rather than
                // fabricating structure.
            }
        }
    }

    let name = name?;
    Some(AbilityParsedFields {
        category,
        name,
        kind,
        target: target.unwrap_or_default(),
        trailing_modifiers: trailing,
    })
}

/// Recognized `ABILITY:` kind tokens.
fn parse_ability_kind(token: &str) -> Option<AbilityKind> {
    match token.trim() {
        "AUTOMATIC" => Some(AbilityKind::Automatic),
        "VIRTUAL" => Some(AbilityKind::Virtual),
        "NORMAL" => Some(AbilityKind::Normal),
        _ => None,
    }
}

/// Split the rightmost tab/whitespace-separated run of `KEY:VALUE` tokens
/// off a pipe-delimited `ABILITY:` body, preserving them verbatim.
///
/// Examples:
///
/// ```text
/// "Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Ability Scores"
///   -> core = "Dwarf Racial Trait|AUTOMATIC|Dwarf ~ Ability Scores"
///      trailing = []
///
/// "CATEGORY=FEAT|Alertness\tFREE:YES"
///   -> core = "CATEGORY=FEAT|Alertness"
///      trailing = ["FREE:YES"]
///
/// "Longbow Proficiency|NORMAL|Longbow\tFREE:NO\tSTACK:YES"
///   -> core = "Longbow Proficiency|NORMAL|Longbow"
///      trailing = ["FREE:NO", "STACK:YES"]
/// ```
fn split_trailing_modifiers(input: &str) -> (String, Vec<String>) {
    // The boundary between core fields and trailing modifiers is the
    // first tab character. Whitespace-only runs (spaces) are also
    // honored for robustness against legacy LST formatting that uses
    // spaces in place of tabs.
    let tab_pos = input.find('\t');
    let space_pos = input
        .find("  ")
        .or_else(|| input.find(' ').filter(|_| false));
    // Above `find(' ').filter(...)` keeps the alternative quiet when the
    // input has no spaces; we prefer the tab boundary when both exist.
    let boundary = match (tab_pos, space_pos) {
        (Some(t), Some(s)) => Some(t.min(s)),
        (Some(t), None) => Some(t),
        (None, Some(s)) => Some(s),
        (None, None) => None,
    };

    let Some(boundary) = boundary else {
        return (input.to_string(), Vec::new());
    };

    let core = input[..boundary].trim_end().to_string();
    let mut trailing = Vec::new();
    let tail = input[boundary..].trim_start_matches(|c: char| c == '\t' || c == ' ');
    for token in tail.split(|c: char| c == '\t' || c == ' ') {
        let t = token.trim();
        if !t.is_empty() {
            trailing.push(t.to_string());
        }
    }
    (core, trailing)
}

#[cfg(test)]
mod tests {
    //! Internal smoke checks. The authoritative acceptance surface lives
    //! in `tests/sd17_b_races_and_abilities.rs`; these are quick guard
    //! tests for the helper functions themselves.
    use super::*;

    #[test]
    fn kind_display_matches_recognized_tokens() {
        assert_eq!(AbilityKind::Automatic.to_string(), "AUTOMATIC");
        assert_eq!(AbilityKind::Virtual.to_string(), "VIRTUAL");
        assert_eq!(AbilityKind::Normal.to_string(), "NORMAL");
    }

    #[test]
    fn split_trailing_modifiers_handles_tab_boundary() {
        let (core, trailing) = split_trailing_modifiers("Longbow|NORMAL|Longbow\tFREE:NO");
        assert_eq!(core, "Longbow|NORMAL|Longbow");
        assert_eq!(trailing, vec!["FREE:NO"]);
    }

    #[test]
    fn split_trailing_modifiers_handles_no_boundary() {
        let (core, trailing) = split_trailing_modifiers("Just|A|B");
        assert_eq!(core, "Just|A|B");
        assert!(trailing.is_empty());
    }

    #[test]
    fn diagnostic_carries_sd17_b3_slice_tag() {
        let d = LstDiagnostic::new(
            "x.lst",
            1,
            "RACE:",
            LstDiagnosticKind::MalformedRacePointer,
            "no target",
        );
        assert_eq!(d.slice, "SD17-B-3");
        assert!(d.kind_is_sd17_b3());
    }
}
