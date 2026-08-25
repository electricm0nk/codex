//! LST spell-row parse shape (SD-17 Slice B-4).
//!
//! This module parses rows in PCGen LST spell files such as
//! `cr_spells.lst`, `apg_spells.lst`, `um_spells.lst`, and `ce_spells.lst`.
//!
//! Scope boundary:
//!
//! - This slice does not parse or semantically interpret the PCC inclusion
//!   directives that REFERENCE spell LST files; that surface is owned by
//!   the SD-17 Slice A [`crate::pcgen_import::include_resolver`].
//! - This slice does not validate spell math, school opposition, or spell
//!   slot tables; those are owned by later spell-engine slices.
//! - This slice does not consume the wider canonical-IR shape; records
//!   stay attached to their LST source rows.
//!
//! Parse-shape contract:
//!
//! - Every record carries its one-based source line number and source path.
//! - The parser is a single-pass tokenizer: one read of the file, one
//!   iteration over `lines()`, one allocation per row's column strings.
//! - Header rows (`#` comments, `SOURCELONG:` / `SOURCESHORT:` /
//!   `SOURCEWEB:` / `SOURCEDATE:` metadata, `###Block:` markers, blank
//!   lines) are skipped silently. They do not produce diagnostics.
//! - Malformed rows produce structured diagnostics with the source line
//!   number and the raw line preserved as evidence.
//!
//! A row is a spell row iff column 0 is non-empty and does not start with
//! a `TAG:` prefix. Subsequent columns are scanned for known tags
//! (`TYPE:`, `CLASSES:`, `SCHOOL:`, ...). The first column that starts
//! with each known tag is recorded under that tag's field. This is robust
//! to the two real corpus shapes:
//!
//! - **Tight TSV** (e.g. `ce_spells.lst`): every column carries a tag or
//!   the spell name, no empty alignment padding.
//! - **Aligned TSV** (e.g. `apg_spells.lst`, `cr_spells.lst`): ~60 columns,
//!   named fields at specific indices, intervening columns empty for
//!   editor alignment.
//!
//! Continuation rows whose column 0 starts with a `TAG:` prefix (e.g.
//! `Resounding Blow.MOD\t\tCLASSES:Paladin=4|Inquisitor=5`) are skipped:
//! they describe variants of the previous spell definition rather than
//! declaring a new spell.
//!
//! Description values may carry a pipe-delimited PCGen meta-annotation
//! suffix (e.g. `DESC:Foo.|!PRERULE:1,DisplayFullSpell`). The
//! annotation is stripped from the visible description field; it remains
//! available in [`LstSpellRecord::description_raw`] for callers that
//! need to inspect the full payload.

use std::fs;
use std::path::{Path, PathBuf};

/// Tags recognized as spell-row column markers. The first column whose
/// value starts with one of these prefixes supplies the field's data.
///
/// The `_name` slot is unused at runtime; it documents the semantic
/// purpose of each tag for code search and is also where the structural
/// record keeps a field of the same name.
///
/// The ordering of this list is irrelevant to correctness: the parser
/// scans every tag per column and records the first match. It IS relevant
/// to log readability.
const KNOWN_TAGS: &[(&str, &str)] = &[
    ("TYPE:", "spell_type"),
    ("CLASSES:", "classes"),
    ("SCHOOL:", "school"),
    ("DESCRIPTOR:", "descriptor"),
    ("SUBSCHOOL:", "sub_school"),
    ("COMPS:", "components"),
    ("CASTTIME:", "casting_time"),
    ("RANGE:", "range"),
    ("ITEM:", "item"),
    ("TARGETAREA:", "target_area"),
    ("DURATION:", "duration"),
    ("SAVEINFO:", "save_info"),
    ("SPELLRES:", "spell_resistance"),
    ("SOURCEPAGE:", "source_page"),
    ("SOURCELINK:", "source_link"),
    ("OUTPUTNAME:", "output_name"),
    ("DESC:", "description"),
];

// Slot indices into the parallel `field_values` accumulator. These MUST
// stay in lockstep with the order of KNOWN_TAGS; the compiler cannot check
// it because both arrays are runtime slice literals.
const SLOT_SPELL_TYPE: usize = 0;
const SLOT_CLASSES: usize = 1;
const SLOT_SCHOOL: usize = 2;
const SLOT_DESCRIPTOR: usize = 3;
const SLOT_SUB_SCHOOL: usize = 4;
const SLOT_COMPONENTS: usize = 5;
const SLOT_CASTING_TIME: usize = 6;
const SLOT_RANGE: usize = 7;
const SLOT_ITEM: usize = 8;
const SLOT_TARGET_AREA: usize = 9;
const SLOT_DURATION: usize = 10;
const SLOT_SAVE_INFO: usize = 11;
const SLOT_SPELL_RESISTANCE: usize = 12;
const SLOT_SOURCE_PAGE: usize = 13;
const SLOT_SOURCE_LINK: usize = 14;
const SLOT_OUTPUT_NAME: usize = 15;
const SLOT_DESCRIPTION_RAW: usize = 16;
const SLOT_COUNT: usize = 17;

/// One parsed spell row from an LST file.
///
/// `line_number` and `source_path` are provenance; the [`LstSpellRecord::payload`]
/// field exposes the structural-equality projection of the row so callers that
/// only care about content can compare two records without their file positions
/// getting in the way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstSpellRecord {
    /// One-based source line number where this row was parsed.
    pub line_number: usize,
    /// Path to the LST file the row was parsed from (stringified for
    /// diagnostic and assertion convenience).
    pub source_path: String,
    /// Spell name, column 0.
    pub name: String,
    /// Optional `OUTPUTNAME:` value: PCGen's display-name override.
    pub output_name: Option<String>,
    /// Column tagged `TYPE:` (e.g. `Arcane`, `Divine`, `Arcane.Divine`).
    pub spell_type: Option<String>,
    /// Column tagged `CLASSES:` (e.g. `Wizard=0|Sorcerer=0`).
    pub classes: Option<String>,
    /// Column tagged `SCHOOL:` (e.g. `Conjuration`).
    pub school: Option<String>,
    /// Column tagged `DESCRIPTOR:` (e.g. `Acid`, `Fire|Mind-Affecting`).
    pub descriptor: Option<String>,
    /// Column tagged `SUBSCHOOL:` (e.g. `Teleportation`).
    pub sub_school: Option<String>,
    /// Column tagged `COMPS:` (e.g. `V, S, M`).
    pub components: Option<String>,
    /// Column tagged `CASTTIME:` (e.g. `1 standard action`).
    pub casting_time: Option<String>,
    /// Column tagged `RANGE:` (e.g. `Personal`, `Close`).
    pub range: Option<String>,
    /// Column tagged `ITEM:` (e.g. `Potion`, `Scroll`).
    pub item: Option<String>,
    /// Column tagged `TARGETAREA:` (e.g. `One creature`).
    pub target_area: Option<String>,
    /// Column tagged `DURATION:` (e.g. `Instantaneous`).
    pub duration: Option<String>,
    /// Column tagged `SAVEINFO:` (e.g. `Fortitude negates`).
    pub save_info: Option<String>,
    /// Column tagged `SPELLRES:` (e.g. `Yes (harmless)`).
    pub spell_resistance: Option<String>,
    /// Column tagged `SOURCEPAGE:` (e.g. `p.241`).
    pub source_page: Option<String>,
    /// Column tagged `SOURCELINK:`.
    pub source_link: Option<String>,
    /// Visible description text with the trailing `|!PRERULE:...`
    /// annotation stripped.
    pub description: Option<String>,
    /// Raw description text including the trailing `|!PRERULE:...`
    /// annotation, preserved for callers that need the full payload.
    pub description_raw: Option<String>,
    /// Structural-equality projection: every field except `line_number` and
    /// `source_path`. Two records parsed from the same row text compare equal
    /// on `payload` regardless of where they were found.
    pub payload: LstSpellRecordPayload,
}

/// Structural-equality projection of an [`LstSpellRecord`].
///
/// Two records whose `payload` compares equal describe the same spell row
/// even if they were found on different source lines of different LST files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstSpellRecordPayload {
    pub name: String,
    pub output_name: Option<String>,
    pub spell_type: Option<String>,
    pub classes: Option<String>,
    pub school: Option<String>,
    pub descriptor: Option<String>,
    pub sub_school: Option<String>,
    pub components: Option<String>,
    pub casting_time: Option<String>,
    pub range: Option<String>,
    pub item: Option<String>,
    pub target_area: Option<String>,
    pub duration: Option<String>,
    pub save_info: Option<String>,
    pub spell_resistance: Option<String>,
    pub source_page: Option<String>,
    pub source_link: Option<String>,
    pub description: Option<String>,
    pub description_raw: Option<String>,
}

/// Classification of an [`LstParseDiagnostic`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LstParseDiagnosticKind {
    /// A row that does not have at least one tab separator is malformed.
    MalformedRow,
    /// A row that has a non-empty column 0 but column 0 starts with a
    /// recognized `TAG:` prefix is a continuation row, not a spell
    /// declaration. PCGen spell declarations name the spell in column 0.
    ContinuationRow,
    /// A row with one or more tab separators but no extractable name in
    /// column 0 is malformed.
    MissingSpellName,
    /// The LST file itself could not be read.
    IoError,
}

/// Structured diagnostic emitted by the LST parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstParseDiagnostic {
    /// Path to the LST source, when available.
    pub source_path: PathBuf,
    /// One-based source line number, when available.
    pub line_number: Option<usize>,
    /// Raw source line preserved as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: LstParseDiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
}

/// Result of parsing a single LST row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstRowParse {
    /// Parsed record, present iff the row was a well-formed spell row.
    pub record: Option<LstSpellRecord>,
    /// Diagnostics emitted for this row. Always empty for a well-formed row.
    pub diagnostics: Vec<LstParseDiagnostic>,
}

impl LstRowParse {
    /// Convenience: take the parsed record, dropping diagnostics.
    pub fn into_record(self) -> Option<LstSpellRecord> {
        self.record
    }

    /// Borrow the diagnostics without consuming the record.
    pub fn diagnostics(&self) -> &[LstParseDiagnostic] {
        &self.diagnostics
    }
}

/// Result of parsing an entire LST file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LstSpellFile {
    /// Absolute or caller-supplied path to the LST source.
    pub source_path: PathBuf,
    /// Every well-formed spell row in source order.
    pub records: Vec<LstSpellRecord>,
    /// Non-fatal diagnostics emitted during the single-pass parse.
    pub diagnostics: Vec<LstParseDiagnostic>,
}

/// Parse one LST row into a [`LstRowParse`].
///
/// `source_path` is the identity recorded on every record and diagnostic;
/// `line_number` is the one-based source line; `raw_line` is the unparsed
/// row text. Allocation is exactly one `String` per non-empty column plus
/// a small fixed overhead.
pub fn parse_lst_spell_row(
    source_path: impl Into<String>,
    line_number: usize,
    raw_line: &str,
) -> LstRowParse {
    let source_path = source_path.into();
    let trimmed_start = raw_line.trim_start();

    // Header / metadata rows: skip silently.
    if trimmed_start.is_empty()
        || trimmed_start.starts_with('#')
        || trimmed_start.starts_with("SOURCELONG:")
        || trimmed_start.starts_with("SOURCESHORT:")
        || trimmed_start.starts_with("SOURCEWEB:")
        || trimmed_start.starts_with("SOURCEDATE:")
        || trimmed_start.starts_with("###")
    {
        return LstRowParse {
            record: None,
            diagnostics: Vec::new(),
        };
    }

    // Malformed: no tab separator at all -- EXCEPT a bare `.COPY=` row,
    // which is valid, minimal PCGen syntax: a `.COPY=` declaration states
    // nothing beyond its own name (it inherits every other field from the
    // base record it names), so a real corpus row can legitimately consist
    // of the name column alone with no trailing tab at all (real example:
    // `book_of_the_damned_volume_1/botd1_spells.lst:16`,
    // `"Teleport (Greater).COPY=Greater Teleport (Self Plus 50 Lbs. Of
    // Objects Only)"`, no tab anywhere on the line). Treating this as
    // malformed silently dropped the row before any caller ever saw it,
    // masking it as a corpus gap rather than a real, resolvable `.COPY=`
    // variant. Every other tab-less line remains malformed, unchanged.
    if !raw_line.contains('\t') {
        let bare_name = trimmed_start.trim_end();
        if bare_name.contains(".COPY=") {
            let payload = LstSpellRecordPayload {
                name: bare_name.to_string(),
                output_name: None,
                spell_type: None,
                classes: None,
                school: None,
                descriptor: None,
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
            };
            return LstRowParse {
                record: Some(LstSpellRecord {
                    line_number,
                    source_path,
                    name: bare_name.to_string(),
                    output_name: None,
                    spell_type: None,
                    classes: None,
                    school: None,
                    descriptor: None,
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
                    payload,
                }),
                diagnostics: Vec::new(),
            };
        }
        return LstRowParse {
            record: None,
            diagnostics: vec![LstParseDiagnostic {
                source_path: PathBuf::from(&source_path),
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstParseDiagnosticKind::MalformedRow,
                message: format!(
                    "row on line {line_number} has no tab separator; cannot parse spell columns"
                ),
            }],
        };
    }

    // Single-pass column split. PCGen LST rows are written with explicit tab
    // separators and we do not need to scan the line more than once to find
    // them.
    let columns: Vec<&str> = raw_line.split('\t').collect();
    let name_column = columns[0].trim();
    if name_column.is_empty() {
        return LstRowParse {
            record: None,
            diagnostics: vec![LstParseDiagnostic {
                source_path: PathBuf::from(&source_path),
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstParseDiagnosticKind::MissingSpellName,
                message: format!(
                    "row on line {line_number} has empty spell name column; PCGen spell rows must name the spell"
                ),
            }],
        };
    }

    // Continuation rows: column 0 carries data, but starts with a recognized
    // tag prefix. These are sub-tables that describe a variant of the
    // previous spell definition (e.g. `Resounding Blow.MOD\t\tCLASSES:...`).
    // They are not new spell declarations.
    if starts_with_known_tag(name_column) {
        return LstRowParse {
            record: None,
            diagnostics: vec![LstParseDiagnostic {
                source_path: PathBuf::from(&source_path),
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: LstParseDiagnosticKind::ContinuationRow,
                message: format!(
                    "row on line {line_number} starts with a tag prefix; treated as a spell continuation, not a new spell"
                ),
            }],
        };
    }

    // Walk each column after column 0 and capture the FIRST occurrence of
    // each known tag. Single-pass: O(n) over columns, no second scan.
    let mut field_values: Vec<Option<String>> = vec![None; SLOT_COUNT];
    for column in columns.iter().skip(1) {
        let value = column.trim();
        if value.is_empty() {
            continue;
        }
        for (slot, (tag, _name)) in KNOWN_TAGS.iter().enumerate() {
            if let (true, Some(stripped)) = (field_values[slot].is_none(), value.strip_prefix(tag))
            {
                let cleaned = stripped.trim();
                if !cleaned.is_empty() {
                    field_values[slot] = Some(cleaned.to_string());
                }
                break;
            }
        }
    }

    // Description carries an optional `|!PRERULE:...` annotation; capture
    // both the raw and stripped forms.
    let description_raw = field_values[SLOT_DESCRIPTION_RAW].clone();
    let description = description_raw
        .as_ref()
        .and_then(|raw| raw.split('|').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let name = name_column.to_string();
    let spell_type = field_values[SLOT_SPELL_TYPE].clone();
    let classes = field_values[SLOT_CLASSES].clone();
    let school = field_values[SLOT_SCHOOL].clone();
    let descriptor = field_values[SLOT_DESCRIPTOR].clone();
    let sub_school = field_values[SLOT_SUB_SCHOOL].clone();
    let components = field_values[SLOT_COMPONENTS].clone();
    let casting_time = field_values[SLOT_CASTING_TIME].clone();
    let range = field_values[SLOT_RANGE].clone();
    let item = field_values[SLOT_ITEM].clone();
    let target_area = field_values[SLOT_TARGET_AREA].clone();
    let duration = field_values[SLOT_DURATION].clone();
    let save_info = field_values[SLOT_SAVE_INFO].clone();
    let spell_resistance = field_values[SLOT_SPELL_RESISTANCE].clone();
    let source_page = field_values[SLOT_SOURCE_PAGE].clone();
    let source_link = field_values[SLOT_SOURCE_LINK].clone();
    let output_name = field_values[SLOT_OUTPUT_NAME].clone();

    let payload = LstSpellRecordPayload {
        name: name.clone(),
        output_name: output_name.clone(),
        spell_type: spell_type.clone(),
        classes: classes.clone(),
        school: school.clone(),
        descriptor: descriptor.clone(),
        sub_school: sub_school.clone(),
        components: components.clone(),
        casting_time: casting_time.clone(),
        range: range.clone(),
        item: item.clone(),
        target_area: target_area.clone(),
        duration: duration.clone(),
        save_info: save_info.clone(),
        spell_resistance: spell_resistance.clone(),
        source_page: source_page.clone(),
        source_link: source_link.clone(),
        description: description.clone(),
        description_raw: description_raw.clone(),
    };

    let record = LstSpellRecord {
        line_number,
        source_path: source_path.clone(),
        name,
        output_name,
        spell_type,
        classes,
        school,
        descriptor,
        sub_school,
        components,
        casting_time,
        range,
        item,
        target_area,
        duration,
        save_info,
        spell_resistance,
        source_page,
        source_link,
        description,
        description_raw,
        payload,
    };

    LstRowParse {
        record: Some(record),
        diagnostics: Vec::new(),
    }
}

/// Parse an entire LST file in a single streaming pass.
///
/// Reads the file once with [`std::fs::read_to_string`], walks lines with
/// [`str::lines`], and forwards each non-header row to
/// [`parse_lst_spell_row`]. There is no second pass; the parser never
/// re-scans a row once it has been split.
///
/// Returns an [`LstParseDiagnostic`] with [`LstParseDiagnosticKind::IoError`]
/// when the file cannot be read.
pub fn parse_lst_spell_file(path: impl AsRef<Path>) -> Result<LstSpellFile, LstParseDiagnostic> {
    let path = path.as_ref();
    let source_path = path.to_path_buf();
    let input_text = match fs::read_to_string(path) {
        Ok(input_text) => input_text,
        Err(error) => {
            return Err(LstParseDiagnostic {
                source_path: source_path.clone(),
                line_number: None,
                raw_line: String::new(),
                kind: LstParseDiagnosticKind::IoError,
                message: format!(
                    "could not read LST file '{}': {error}",
                    source_path.display()
                ),
            });
        }
    };

    let source_path_string = source_path.to_string_lossy().into_owned();
    let mut records = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let parsed = parse_lst_spell_row(&source_path_string, line_number, raw_line);
        if let Some(record) = parsed.record {
            records.push(record);
        }
        diagnostics.extend(parsed.diagnostics);
    }

    Ok(LstSpellFile {
        source_path,
        records,
        diagnostics,
    })
}

fn starts_with_known_tag(value: &str) -> bool {
    KNOWN_TAGS.iter().any(|(tag, _name)| value.starts_with(tag))
}

#[cfg(test)]
mod copy_only_row_tests {
    use super::*;

    /// RED before this cycle's fix: a bare `.COPY=` row (no tab anywhere on
    /// the line -- the real corpus shape at
    /// `book_of_the_damned_volume_1/botd1_spells.lst:16`) was rejected as
    /// `MalformedRow` and produced no record at all, silently dropping a
    /// real, resolvable `.COPY=` variant before `ingest_spells.rs` ever saw
    /// it. GREEN after: the row parses, `name` carries the raw
    /// `base.COPY=variant` string unchanged (unpacked downstream by
    /// `ingest_spells.rs::copy_variant_split`), every other field is `None`
    /// (never fabricated), and zero diagnostics fire.
    #[test]
    fn bare_copy_row_with_no_tab_parses_instead_of_malformed() {
        let raw = "Teleport (Greater).COPY=Greater Teleport (Self Plus 50 Lbs. Of Objects Only)";
        let parsed = parse_lst_spell_row("test.lst", 16, raw);
        assert!(
            parsed.diagnostics.is_empty(),
            "expected no diagnostics, got {:?}",
            parsed.diagnostics
        );
        let record = parsed.record.expect("expected a record, got None");
        assert_eq!(record.name, raw);
        assert_eq!(record.classes, None);
        assert_eq!(record.school, None);
        assert_eq!(record.description, None);
    }

    /// A genuinely malformed tab-less row that is NOT a `.COPY=` shape
    /// still reports `MalformedRow` exactly as before -- this fix is
    /// narrowly scoped to the one real shape it targets, not a blanket
    /// relaxation of the tab requirement.
    #[test]
    fn bare_non_copy_row_with_no_tab_is_still_malformed() {
        let parsed = parse_lst_spell_row("test.lst", 1, "Fireball");
        assert!(parsed.record.is_none());
        assert_eq!(parsed.diagnostics.len(), 1);
        assert_eq!(parsed.diagnostics[0].kind, LstParseDiagnosticKind::MalformedRow);
    }
}
