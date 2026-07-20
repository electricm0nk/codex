//! LST object-kind parser for the `CLASS:` directive (SD-17 Slice B-1;
//! widened SD-22 Epic 3 to add `Cavalier`; widened SD-22 Epic 4 to add
//! `Brawler`; widened SD-22 Epic 4 again to add `Slayer`).
//!
//! Scope: the six martial classes named in the slice card body
//! (Fighter, Barbarian, Monk, Rogue, Ranger, Paladin), plus `Cavalier`
//! (added by SD-22 Epic 3's Cavalier ingest cycle — the real
//! `CLASS:Cavalier` line in `apg_classes.lst` carries
//! `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")` with no
//! `SPELLSTAT:` line, the same non-caster posture as the six original
//! martial classes, so it belongs in this parser rather than the
//! spellcasting-class parser), plus `Brawler` (added by SD-22 Epic 4's
//! Brawler ingest cycle — the real `CLASS:Brawler` line in
//! `acg_classes.lst` carries the same full-BAB, no-`SPELLSTAT:` posture),
//! plus `Slayer` (added by SD-22 Epic 4's Slayer ingest cycle — the real
//! `CLASS:Slayer` line in `acg_classes.lst` carries the identical
//! full-BAB, no-`SPELLSTAT:` posture).
//! The parser recognizes every `CLASS:<name>` line in the corpus where
//! `<name>` is one of those nine (or one of their `Ex-<name>` mirror
//! variants), carries every tab-delimited token pair to a canonical IR
//! record, and preserves one-based source line numbers on every record.
//!
//! Non-martial classes (Bard, Cleric, Druid, Sorcerer, Wizard, ...) and
//! every other APG/ACG class until its own SD-22 ingest cycle widens
//! this array or `spellcasting_class.rs`'s, per that class's actual
//! casting shape, are out of scope for this slice and are deliberately
//! skipped without raising diagnostics.
//!
//! This module composes with the GE-03 PCC parser and the SD-17
//! include-graph resolver rather than shadowing them. The LST file
//! itself is consumed by `parse_class_file` or `parse_class_entries`;
//! upstream callers discover the LST file from the
//! `IncludeResolution::lst_files` collection emitted by the resolver.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// The six martial classes named in the SD-17 Slice B-1 card body, plus
/// `Cavalier` (SD-22 Epic 3 widening), `Brawler` (SD-22 Epic 4 widening),
/// and `Slayer` (SD-22 Epic 4 widening) — see module doc comment.
pub const MARTIAL_CLASS_NAMES: &[&str] = &[
    "Fighter", "Barbarian", "Monk", "Rogue", "Ranger", "Paladin", "Cavalier", "Brawler", "Slayer",
];

/// Result of parsing one LST file for the martial-class scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassParseResult {
    /// Identity of the parsed LST file, as supplied by the caller.
    pub source_path: String,
    /// One [`ClassEntry`] per distinct class name encountered.
    pub entries: Vec<ClassEntry>,
    /// Non-fatal parse diagnostics.
    pub diagnostics: Vec<LstDiagnostic>,
}

/// One martial class record aggregated from one or more `CLASS:`
/// header lines and zero or more `###Block:` per-level feature blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassEntry {
    /// Class name as it appears after `CLASS:` (e.g. `Fighter`,
    /// `Ex-Barbarian`). Excluded from the record if the line was
    /// malformed; a diagnostic is emitted instead.
    pub class_name: String,
    /// One-based line number of the first `CLASS:<name>` line for
    /// this class in the source LST.
    pub header_line_number: usize,
    /// Raw first `CLASS:<name>` line, preserved verbatim.
    pub header_raw_line: String,
    /// Every tab-delimited `KEY:VAL` token from every `CLASS:<name>`
    /// line for this class, in source order, with one-based line numbers.
    pub tokens: Vec<ClassToken>,
    /// `###Block:` separators followed by per-level feature lines.
    /// Feature blocks are emitted in source order.
    pub feature_blocks: Vec<ClassFeatureBlock>,
}

/// One tab-delimited `KEY:VAL` token lifted from a `CLASS:<name>` line.
///
/// The parser is intentionally shallow: it does not interpret the value
/// of `BONUS:`, `DEFINE:`, `FACT:`, `SPELLSTAT:`, or any other key —
/// it only records what the source said. The SD-13 matrix uplift is
/// a later slice and consumes the raw tokens verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassToken {
    /// Key portion (text before the first `:`).
    pub key: String,
    /// Value portion (text after the first `:`).
    pub value: String,
    /// One-based line number where the token appeared in the LST.
    pub line_number: usize,
    /// Raw `KEY:VAL` text as it appeared between tabs on the source line.
    pub raw_pair: String,
}

/// One `###Block:` separator and the per-level feature lines that
/// follow it (until the next `###Block:` marker or end of file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFeatureBlock {
    /// Label text after `###Block:`. Empty if the marker was malformed
    /// (e.g. `###Block:` with no label); a diagnostic is emitted.
    pub label: String,
    /// One-based line number of the `###Block:` marker itself.
    pub header_line_number: usize,
    /// Per-level feature lines in source order. A line like
    /// `1\tABILITY:Class|AUTOMATIC|Fighter` produces a level of `1`
    /// and a raw line preserving the entire tab-delimited payload.
    pub level_lines: Vec<ClassLevelLine>,
}

/// One feature line within a `###Block:` section, tagged with the
/// level integer that led the line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassLevelLine {
    /// Integer level that led the line, parsed as base-10.
    pub level: u8,
    /// Full raw text of the line, including any `KEY:VAL` payload.
    pub raw_line: String,
    /// One-based line number in the source LST.
    pub line_number: usize,
}

/// Classification of a parse diagnostic emitted by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LstDiagnosticKind {
    /// A `CLASS:` directive with no class name, or some other defect
    /// the parser could not recover from. The `MalformedSD17B1` label
    /// matches the slice card's verification criterion.
    MalformedSD17B1,
    /// A `###Block:` marker with no label.
    MalformedBlockMarker,
    /// A feature line inside a `###Block:` section that did not begin
    /// with a level integer. The line is preserved as raw text.
    UnleveledFeatureLine,
    /// The LST source could not be read from disk.
    ReadFailed,
}

/// Structured diagnostic emitted by the LST parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstDiagnostic {
    /// One-based line number of the offending line when applicable.
    pub line_number: Option<usize>,
    /// Raw line text preserved as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: LstDiagnosticKind,
    /// Human-readable explanation of the defect.
    pub message: String,
}

/// Parse the contents of one LST file for the martial-class scope.
///
/// `source_path` is the identity recorded on the result; `input_text`
/// is the raw LST contents.
pub fn parse_class_entries(source_path: &str, input_text: &str) -> ClassParseResult {
    let mut state = ClassParseState::new(source_path);
    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        // `###Block:` is a structural LST marker, not a comment. It
        // shares a `#` prefix with comment lines, so it must be
        // recognized before the generic comment skip.
        if let Some(label) = strip_block_marker(content) {
            state.observe_block_marker(line_number, raw_line, label);
            continue;
        }

        // Blank lines and comment lines (lines that start with a
        // single `#`, not the `###Block:` structural marker handled
        // above) are record-keeping boundaries, not class record
        // material.
        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        if let Some(rest) = content.strip_prefix("CLASS:") {
            state.observe_class_line(line_number, raw_line, rest);
        } else if state.active_block.is_some() {
            state.observe_feature_line(line_number, raw_line);
        }
        // Lines outside any active `###Block:` that are not `CLASS:`
        // directives are ignored (e.g. `SOURCELONG:`, `RACE:`, `SPELL:`,
        // `ABILITY:`, etc.). Out-of-scope non-class directives are not
        // a diagnostic per the slice card scope.
    }
    state.finalize()
}

/// Parse one LST file from disk. The path is recorded as the result's
/// `source_path` (using the canonicalized absolute form).
pub fn parse_class_file(path: &Path) -> Result<ClassParseResult, LstDiagnostic> {
    let input_text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return Err(LstDiagnostic {
                line_number: None,
                raw_line: String::new(),
                kind: LstDiagnosticKind::ReadFailed,
                message: format!("could not read LST file '{}': {error}", path.display()),
            });
        }
    };
    Ok(parse_class_entries(
        &path.display().to_string(),
        &input_text,
    ))
}

fn strip_block_marker(content: &str) -> Option<&str> {
    content.strip_prefix("###Block:").map(|label| label.trim())
}

struct ClassParseState {
    result: ClassParseResult,
    /// Index into `result.entries` of the currently-open class entry,
    /// if any. Continuation `CLASS:<name>` lines update this.
    active_entry: Option<usize>,
    /// Active `###Block:` section in progress, if any. Feature lines
    /// between block markers append to it.
    active_block: Option<usize>,
    /// Class names accepted by the slice scope.
    scope: HashSet<String>,
}

impl ClassParseState {
    fn new(source_path: &str) -> Self {
        let scope = MARTIAL_CLASS_NAMES
            .iter()
            .map(|name| name.to_string())
            .chain(MARTIAL_CLASS_NAMES.iter().map(|name| format!("Ex-{name}")))
            .collect();
        Self {
            result: ClassParseResult {
                source_path: source_path.to_string(),
                entries: Vec::new(),
                diagnostics: Vec::new(),
            },
            active_entry: None,
            active_block: None,
            scope,
        }
    }

    fn observe_class_line(&mut self, line_number: usize, raw_line: &str, after_class: &str) {
        // `CLASS:` directives terminate any open `###Block:` section.
        // The next class block starts a fresh feature-block scope.
        self.active_block = None;

        // The first tab-delimited field after `CLASS:` is the class
        // name. Empty class name → malformed diagnostic; we still
        // continue parsing subsequent lines.
        let class_name = after_class
            .split('\t')
            .next()
            .unwrap_or(after_class)
            .trim()
            .to_string();
        if class_name.is_empty() {
            self.result.diagnostics.push(LstDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstDiagnosticKind::MalformedSD17B1,
                message: "CLASS directive has no class name".to_string(),
            });
            return;
        }

        if !self.scope.contains(&class_name) {
            // Out of scope: skip the line entirely. No diagnostic, no
            // record — the class belongs to a different B-slice.
            self.active_entry = None;
            return;
        }

        // Find or create the entry for this class name. The card
        // requires the parser to merge multiple `CLASS:<name>` lines
        // for the same class into a single record.
        let entry_index = if let Some(index) = self
            .result
            .entries
            .iter()
            .position(|entry| entry.class_name == class_name)
        {
            index
        } else {
            self.result.entries.push(ClassEntry {
                class_name: class_name.clone(),
                header_line_number: line_number,
                header_raw_line: raw_line.to_string(),
                tokens: Vec::new(),
                feature_blocks: Vec::new(),
            });
            self.result.entries.len() - 1
        };

        // Pull every tab-delimited KEY:VAL token off the rest of the
        // line. The first field is the class name and is dropped.
        let tokens = collect_class_tokens(line_number, raw_line);
        let entry = &mut self.result.entries[entry_index];
        entry.tokens.extend(tokens);
        self.active_entry = Some(entry_index);
    }

    fn observe_block_marker(&mut self, line_number: usize, raw_line: &str, label: &str) {
        // `###Block:` markers only carry meaning when there is an
        // open class entry. Otherwise the marker is recorded as a
        // diagnostic so the operator can see the orphan.
        let Some(entry_index) = self.active_entry else {
            self.result.diagnostics.push(LstDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstDiagnosticKind::MalformedSD17B1,
                message: format!(
                    "###Block marker with no preceding CLASS directive (label: '{label}')"
                ),
            });
            return;
        };
        if label.is_empty() {
            self.result.diagnostics.push(LstDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstDiagnosticKind::MalformedBlockMarker,
                message: "###Block marker has no label".to_string(),
            });
        }
        let entry = &mut self.result.entries[entry_index];
        entry.feature_blocks.push(ClassFeatureBlock {
            label: label.to_string(),
            header_line_number: line_number,
            level_lines: Vec::new(),
        });
        self.active_block = Some(entry.feature_blocks.len() - 1);
    }

    fn observe_feature_line(&mut self, line_number: usize, raw_line: &str) {
        // The active block lives on the same entry as the active
        // class. The structure mirrors `self.active_block` for
        // clarity rather than reaching through two indirections.
        let Some(entry_index) = self.active_entry else {
            return;
        };
        let Some(block_index) = self.active_block else {
            return;
        };
        let (level, is_leveled) = parse_level_prefix(raw_line);
        if !is_leveled {
            self.result.diagnostics.push(LstDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstDiagnosticKind::UnleveledFeatureLine,
                message: "feature line inside ###Block did not start with a level integer"
                    .to_string(),
            });
        }
        let entry = &mut self.result.entries[entry_index];
        entry.feature_blocks[block_index]
            .level_lines
            .push(ClassLevelLine {
                level,
                raw_line: raw_line.to_string(),
                line_number,
            });
    }

    fn finalize(mut self) -> ClassParseResult {
        // Determinism: sort entries by header line number so the
        // output order matches source order even if upstream callers
        // ever mutate the entry list. New entries are already pushed
        // in source order, so this is a no-op in practice; it is
        // here as a guard.
        self.result
            .entries
            .sort_by_key(|entry| entry.header_line_number);
        // Also sort tokens within each entry by line number so the
        // canonical IR has stable order even if the parser is
        // refactored later.
        for entry in &mut self.result.entries {
            entry.tokens.sort_by_key(|token| token.line_number);
        }
        self.result
    }
}

/// Lift every tab-delimited `KEY:VAL` token off a `CLASS:<name>` line.
///
/// The class name (the first field) is dropped — it is recorded on the
/// enclosing `ClassEntry` as `class_name`. Every other field is
/// expected to be a `KEY:VAL` pair; tokens that lack a `:` separator
/// are kept as raw evidence with an empty value.
fn collect_class_tokens(line_number: usize, raw_line: &str) -> Vec<ClassToken> {
    let content = raw_line.trim_start();
    let rest = content.strip_prefix("CLASS:").unwrap_or(content);
    let mut tokens = Vec::new();
    for field in rest.split('\t') {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }
        // The first non-empty field is the class name. Skip it.
        if tokens.is_empty() && !trimmed.contains(':') && self_first_field_is_class_name(raw_line) {
            continue;
        }
        let (key, value) = match trimmed.split_once(':') {
            Some((key, value)) => (key.to_string(), value.to_string()),
            None => (trimmed.to_string(), String::new()),
        };
        tokens.push(ClassToken {
            key,
            value,
            line_number,
            raw_pair: trimmed.to_string(),
        });
    }
    tokens
}

fn self_first_field_is_class_name(raw_line: &str) -> bool {
    let content = raw_line.trim_start();
    let after = content.strip_prefix("CLASS:").unwrap_or(content);
    let first = after.split('\t').next().unwrap_or(after);
    !first.contains(':')
}

fn parse_level_prefix(raw_line: &str) -> (u8, bool) {
    let leading = raw_line.split('\t').next().unwrap_or(raw_line).trim();
    if leading.is_empty() {
        return (0, false);
    }
    let digits: String = leading
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if digits.is_empty() || digits.len() != leading.len() {
        return (0, false);
    }
    match leading.parse::<u8>() {
        Ok(level) => (level, true),
        Err(_) => (0, false),
    }
}
