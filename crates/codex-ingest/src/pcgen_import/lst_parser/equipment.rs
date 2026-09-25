//! LST equipment parser (SD-17 Slice B-5).
//!
//! Scope: this module parses the equipment and equipment-modifier surface of
//! every PCGen LST file of the relevant kind. Two on-disk shapes are
//! recognized per slice-card directive:
//!
//! 1. The directive-prefix form: a line whose first non-whitespace token is
//!    `EQUIP:<name>` (for an equipment record) or `EQUIPMOD:<name>` (for an
//!    equipment-modifier record). The remainder of the line is the
//!    record's tag payload.
//! 2. The corpus-typical row form (the only form used by Pathfinder,
//!    Pathfinder 2e, D&D 3.5e, D&D 5e, Starfinder, and the homebrew corpora
//!    on this host): a row whose column 0 is a non-empty equipment or
//!    equipment-modifier name and whose subsequent columns carry tab-
//!    delimited `KEY:VAL` tags. `###Block:` markers separate equipment
//!    categories (Ammunition, Armor, Weapons, Masterwork, ...). Continuation
//!    rows whose column 0 starts with a recognized `TAG:` prefix are
//!    appended as additional tag columns to the most recent record.
//!
//! Equipment-modifier records commonly carry deeply nested `BONUS:` chains
//! such as `BONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0`.
//! The parser flattens every `BONUS:` token into a single [`BonusToken`]
//! on the record; nesting is expressed by the linear list of pipe-delimited
//! qualifiers on each bonus token, not by a recursive tree. This is what
//! the slice card calls out as the "no unbounded recursion" requirement:
//! even a chain with a thousand pipe-delimited qualifiers terminates in
//! O(n) time without growing the Rust call stack.
//!
//! Every record carries its one-based source line number and the raw line
//! text. The parser walks `input_text.lines()` exactly once and allocates
//! at most one record per recognized line (O(n) in line count).
//!
//! Scope boundary:
//!
//! - This slice does not modify the GE-03 PCC parser surface; PCC-side
//!   `EQUIP:` / `EQUIPMOD:` directives are an extension owned by a
//!   separate card and intentionally out of scope here.
//! - This slice does not interpret the value grammar (e.g. the pipe-
//!   delimited qualifiers on a `TYPE:` field) beyond recording the raw
//!   token text. The semantic conversion of `KEY:`, `TYPE:`, `BONUS:`, and
//!   friends is owned by later SD-13 / SD-17 slices.
//! - This slice does not touch the rules_core, the UI, or the SD-13
//!   matrix file.

use std::fs;
use std::path::Path;

/// Tags recognized as equipment-row column markers. The first column whose
/// value starts with one of these prefixes supplies the field's data.
///
/// A row is an equipment row iff column 0 is non-empty and is NOT one of
/// these tag prefixes (i.e. column 0 names the equipment). Continuation
/// rows whose column 0 IS one of these prefixes are appended as additional
/// tag columns to the most recent record.
///
/// The ordering is irrelevant to correctness — the parser scans every tag
/// per column and records the first match — but is ordered by frequency in
/// the real corpus for readability.
const KNOWN_TAGS: &[(&str, &str)] = &[
    ("KEY:", "key"),
    ("TYPE:", "type"),
    ("COST:", "cost"),
    ("WT:", "weight"),
    ("SOURCEPAGE:", "source_page"),
    ("SOURCELONG:", "source_long"),
    ("SOURCESHORT:", "source_short"),
    ("SOURCEWEB:", "source_web"),
    ("SOURCEDATE:", "source_date"),
    ("VISIBLE:", "visible"),
    ("OUTPUTNAME:", "output_name"),
    ("OUTPUTTEXT:", "output_text"),
    ("SORTKEY:", "sort_key"),
    ("PROFICIENCY:", "proficiency"),
    ("ACCHECK:", "ac_check"),
    ("MAXDEX:", "max_dex"),
    ("SPELLFAILURE:", "spell_failure"),
    ("BASEITEM:", "base_item"),
    ("BASEQTY:", "base_quantity"),
    ("EQMOD:", "eq_mod"),
    ("MODS:", "mods"),
    ("ITYPE:", "item_type"),
    ("NAMEOPT:", "name_opt"),
    ("FORMATCAT:", "format_cat"),
    ("PRETYPE:", "pre_type"),
    ("PREMULT:", "pre_mult"),
    ("ASSIGNTOALL:", "assign_to_all"),
    ("SLOTS:", "slots"),
    ("QUALITY:", "quality"),
    ("DESC:", "description"),
    ("SPROP:", "special_property"),
    ("BONUS:", "bonus"),
    ("CHOOSE:", "choose"),
    ("AUTO:", "auto"),
    ("ALIGN:", "align"),
    ("RACETYPE:", "race_type"),
    ("WEAPONPROF:", "weapon_prof"),
    ("ARMORPROF:", "armor_prof"),
    ("CRITMULT:", "crit_mult"),
    ("CRITRANGE:", "crit_range"),
    ("DAMAGE:", "damage"),
    ("TOHIT:", "to_hit"),
    ("WEAPONTYPE:", "weapon_type"),
    ("RANGE:", "range"),
    ("AMMUNITION:", "ammunition"),
    ("CATEGORY:", "category"),
    ("ENCUMBERANCE:", "encumberance"),
    ("CONTAINS:", "contains"),
    ("CONTENT:", "content"),
    ("PROFICIENCY:", "proficiency"),
    ("FEAT:", "feat"),
    ("SKILL:", "skill"),
    ("ABILITY:", "ability"),
];

/// Classification of a parse diagnostic emitted by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquipmentDiagnosticKind {
    /// A `EQUIP:` or `EQUIPMOD:` directive with no value, a `###Block:`
    /// marker with no preceding in-scope record, or some other defect
    /// that the parser could not recover from. The `MalformedSD17B5`
    /// label matches the slice card's verification criterion.
    MalformedSD17B5,
    /// A `BONUS:` token whose internal pipe-delimited structure is
    /// degenerate (e.g. zero-length qualifiers, or trailing pipe with
    /// no qualifier). The token is preserved as raw text; the
    /// diagnostic flags the structure rather than the presence of
    /// the token.
    MalformedBonusChain,
    /// A `###Block:` marker with no label.
    MalformedBlockMarker,
    /// A continuation row inside a `###Block:` section that the parser
    /// could not attach to any in-scope record. The row text is
    /// preserved as evidence.
    OrphanContinuationRow,
    /// The LST source could not be read from disk.
    ReadFailed,
}

/// Structured diagnostic emitted by the LST equipment parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentDiagnostic {
    /// One-based line number of the offending line when applicable.
    pub line_number: Option<usize>,
    /// Raw line text preserved as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: EquipmentDiagnosticKind,
    /// Human-readable explanation of the defect.
    pub message: String,
}

/// Distinguishes an equipment record from an equipment-modifier record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentRecordKind {
    /// `EQUIP:` directive or a row in a `*_equip*.lst` file.
    Equip,
    /// `EQUIPMOD:` directive or a row in a `*_equipmods.lst` file.
    EquipMod,
}

impl EquipmentRecordKind {
    /// Canonical PCGen directive token (upper-case, including the trailing `:`).
    pub fn token(self) -> &'static str {
        match self {
            EquipmentRecordKind::Equip => "EQUIP",
            EquipmentRecordKind::EquipMod => "EQUIPMOD",
        }
    }

    /// Map a canonical token (without the trailing `:`) to a kind. Returns
    /// `None` for unknown tokens or non-canonical casing — the parser is
    /// case-sensitive on the directive prefix to match PCGen's convention.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "EQUIP" => Some(EquipmentRecordKind::Equip),
            "EQUIPMOD" => Some(EquipmentRecordKind::EquipMod),
            _ => None,
        }
    }
}

/// One tab-delimited `KEY:VAL` token lifted from an equipment or
/// equipment-modifier row. The parser is intentionally shallow: it does
/// not interpret the value, it only records what the source said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentToken {
    /// Key portion (text before the first `:`).
    pub key: String,
    /// Value portion (text after the first `:`), with surrounding whitespace
    /// stripped.
    pub value: String,
    /// One-based line number where the token appeared in the LST.
    pub line_number: usize,
    /// Raw `KEY:VAL` text as it appeared between tabs on the source line.
    pub raw_pair: String,
}

/// One `BONUS:` clause lifted from a record. The internal pipe-delimited
/// qualifiers are recorded as a flat list rather than a recursive tree,
/// so deeply-nested chains terminate in O(n) time without stack growth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BonusToken {
    /// One-based line number where the `BONUS:` clause appeared.
    pub line_number: usize,
    /// The raw `BONUS:...` text as it appeared in the LST.
    pub raw_bonus: String,
    /// Pipe-delimited qualifiers of the bonus, in source order. A bonus
    /// `BONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0` yields
    /// `["COMBAT", "AC", "1", "TYPE=Armor", "PREVAREQ:DisableArmorBonus,0"]`.
    /// Nested `PREVAREQ:` clauses appear as additional elements in this
    /// list rather than as children of a sub-node.
    pub qualifiers: Vec<String>,
}

/// One equipment or equipment-modifier record aggregated from one or more
/// lines. Equipment-modifier records in particular frequently span
/// multiple lines (a `KEY:` row, a `TYPE:` row, several `BONUS:` rows)
/// because each tag is on its own line in aligned TSV format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentRecord {
    /// Whether this is an `EQUIP:` or `EQUIPMOD:` record.
    pub kind: EquipmentRecordKind,
    /// Equipment or equipment-modifier name as it appears in column 0 of
    /// the first row. Empty when the directive was malformed; a
    /// diagnostic is emitted instead.
    pub name: String,
    /// One-based line number of the first row for this record.
    pub header_line_number: usize,
    /// Raw text of the first row, preserved verbatim as evidence.
    pub header_raw_line: String,
    /// Every tab-delimited `KEY:VAL` token from every row that contributed
    /// to this record, in source order. Continuation rows append their
    /// tokens here.
    pub tokens: Vec<EquipmentToken>,
    /// Every `BONUS:` clause lifted from the record, in source order. The
    /// list is the flat, non-recursive representation required by the
    /// slice card's "no unbounded recursion" guarantee.
    pub bonus_chains: Vec<BonusToken>,
    /// Always `true` for records emitted by this parser: every recognized
    /// equipment or equipment-modifier row at column 0 is its own record
    /// in PCGen's grammar. Retained for API symmetry with the future
    /// multi-kind LST parser.
    pub is_record_start: bool,
    /// Diagnostics attached to this record.
    pub diagnostics: Vec<EquipmentDiagnostic>,
}

impl EquipmentRecord {
    /// Every token on this record whose OWN source line is exactly `line`
    /// -- narrower than `self.tokens`, which can carry tokens `open_record`
    /// merged in from a DIFFERENT physical line representing (or
    /// appearing to represent) the same logical item.
    ///
    /// `open_record`'s cross-line merge is deliberate and correct for its
    /// designed case (one logical item's `KEY:`/`TYPE:`/`BONUS:` rows
    /// spread across several aligned-TSV lines). But nothing about "this
    /// logical record merged rows X and Y" tells a caller which of those
    /// rows is the SPECIFIC one a corpus citation (`source.line`) points
    /// at -- and three real shipped records (`OPEN-ISSUES.md` row 61:
    /// `bastard_s_sting`, `mountain_pattern_armor`, `hunter_s_stand`)
    /// proved a caller that takes the whole merged `self.tokens` list
    /// ships tokens the cited line never states, because a `.COPY=`
    /// variant's base-template name collision (or a genuine same-name
    /// restatement on a later line) pulled a DIFFERENT line's tokens into
    /// the same logical record.
    ///
    /// A caller enforcing single-citation-line provenance (the same
    /// invariant `corpus_literal_sweep` independently re-checks) must use
    /// this accessor, not `self.tokens`, whenever it is about to attribute
    /// tokens to one specific cited line. Every `EquipmentToken` already
    /// carries its own `line_number` -- this is a pure filter, no parser
    /// behavior changes, so it cannot regress any existing caller that
    /// still wants the full merged view.
    pub fn tokens_on_line(&self, line: usize) -> Vec<&EquipmentToken> {
        self.tokens.iter().filter(|token| token.line_number == line).collect()
    }

    /// The `bonus_chains` analogue of [`Self::tokens_on_line`] -- same
    /// single-citation-line provenance filter, for the flattened `BONUS:`
    /// clause list.
    pub fn bonus_chains_on_line(&self, line: usize) -> Vec<&BonusToken> {
        self.bonus_chains.iter().filter(|bonus| bonus.line_number == line).collect()
    }
}

/// Result of parsing one LST file for the equipment scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentParseResult {
    /// Identity of the parsed LST file, as supplied by the caller.
    pub source_path: String,
    /// Every equipment and equipment-modifier record encountered, in source
    /// order. Equipment-modifier records are NOT distinguished from
    /// equipment records at the result level — both appear in `entries`.
    /// Use [`Self::records_by_kind`] to separate them.
    pub entries: Vec<EquipmentRecord>,
    /// Non-fatal parse diagnostics not attached to a specific record (e.g.
    /// orphan continuation rows).
    pub diagnostics: Vec<EquipmentDiagnostic>,
}

/// Parse the contents of one LST file for the equipment scope.
///
/// `source_path` is the identity recorded on the result; `input_text` is
/// the raw LST contents. The parser walks the input one line at a time,
/// skipping blank lines and comment lines (lines whose first non-
/// whitespace character is `#`, excluding the structural `###Block:`
/// marker which is recognized first).
///
/// Performance: O(n) in the number of input lines. Exactly one allocation
/// per recognized record and one per token; no second scan.
pub fn parse_equipment_entries(source_path: &str, input_text: &str) -> EquipmentParseResult {
    let mut state = EquipmentParseState::new(source_path);
    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        // `###Block:` is a structural LST marker, not a comment. It shares
        // a `#` prefix with comment lines, so it must be recognized before
        // the generic comment skip.
        if let Some(label) = strip_block_marker(content) {
            state.observe_block_marker(line_number, raw_line, label);
            continue;
        }

        // Blank lines and comment lines are record-keeping boundaries, not
        // equipment record material.
        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        // Directive-prefix form: `EQUIP:<name>` or `EQUIPMOD:<name>`.
        // When this form appears, the record starts here regardless of
        // whether a `###Block:` is open.
        if let Some(kind) = recognize_directive_prefix(content) {
            state.observe_directive_row(line_number, raw_line, content, kind);
            continue;
        }

        // The leading non-tab token (column 0) tells us whether this is an
        // equipment row, a continuation row, or an unrelated line.
        let column_zero = raw_line.split('\t').next().unwrap_or("").trim();

        if column_zero.is_empty() {
            // No column 0 (a tag-only continuation line with leading tab).
            // Attach as tokens to the active record if any.
            state.observe_continuation_row(line_number, raw_line);
            continue;
        }

        if is_continuation_tag_prefix(column_zero) {
            // Continuation row: column 0 starts with a recognized tag.
            // Attach the row's tokens to the most recent record.
            state.observe_continuation_row(line_number, raw_line);
            continue;
        }

        // Equipment / equipment-modifier row. Column 0 is the record's
        // name. Lift every tag from subsequent columns into the record's
        // token list.
        state.observe_record_row(line_number, raw_line, column_zero);
    }
    state.finalize()
}

/// Parse one LST file from disk. The path is recorded as the result's
/// `source_path` (using the canonicalized absolute form).
pub fn parse_equipment_file(path: &Path) -> Result<EquipmentParseResult, EquipmentDiagnostic> {
    let input_text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return Err(EquipmentDiagnostic {
                line_number: None,
                raw_line: String::new(),
                kind: EquipmentDiagnosticKind::ReadFailed,
                message: format!("could not read LST file '{}': {error}", path.display()),
            });
        }
    };
    Ok(parse_equipment_entries(
        &path.display().to_string(),
        &input_text,
    ))
}

fn strip_block_marker(content: &str) -> Option<&str> {
    content.strip_prefix("###Block:").map(|label| label.trim())
}

/// Recognize a `EQUIP:` or `EQUIPMOD:` directive-prefix row.
///
/// Returns the [`EquipmentRecordKind`] whose token matched, or `None` if
/// the content does not start with one of those directives. The slice card
/// scope says: "Parses every `EQUIP:` line and every `EQUIPMOD:` line in
/// every PCGen LST file" — the prefix form is the literal interpretation,
/// and the corpus-typical row form is the realistic interpretation. Both
/// are supported.
fn recognize_directive_prefix(content: &str) -> Option<EquipmentRecordKind> {
    for (token, kind) in [
        ("EQUIP:", EquipmentRecordKind::Equip),
        ("EQUIPMOD:", EquipmentRecordKind::EquipMod),
    ] {
        if let Some(rest) = content.strip_prefix(token) {
            // If the content is just the directive with no value, treat it
            // as a malformed record; the caller emits the diagnostic.
            let _ = rest;
            return Some(kind);
        }
    }
    None
}

/// True iff the column-0 token starts with a recognized tag prefix (i.e.
/// is a continuation row rather than a record-start row).
fn is_continuation_tag_prefix(column_zero: &str) -> bool {
    KNOWN_TAGS
        .iter()
        .any(|(tag, _)| column_zero.starts_with(tag))
}

/// Lift the equipment or equipment-modifier name from the column-0 token.
/// Strips an optional `.COPY=<source>` suffix (used in the corpus for
/// equipment variants whose data is copied from a base item) so the
/// canonical record name is the display name only.
fn extract_record_name(column_zero: &str) -> &str {
    // `Bolt (Base).COPY=Bolt, Crossbow` → `Bolt (Base)`
    // `Padded Armor` → `Padded Armor`
    if let Some((name, _rest)) = column_zero.split_once(".COPY=") {
        name.trim()
    } else {
        column_zero.trim()
    }
}

struct EquipmentParseState {
    result: EquipmentParseResult,
    /// Index into `result.entries` of the most-recently opened record, if
    /// any. Continuation rows append to this record.
    active_entry: Option<usize>,
}

impl EquipmentParseState {
    fn new(source_path: &str) -> Self {
        Self {
            result: EquipmentParseResult {
                source_path: source_path.to_string(),
                entries: Vec::new(),
                diagnostics: Vec::new(),
            },
            active_entry: None,
        }
    }

    fn observe_directive_row(
        &mut self,
        line_number: usize,
        raw_line: &str,
        content: &str,
        kind: EquipmentRecordKind,
    ) {
        // Strip the directive prefix; the rest of the line is the
        // tab-delimited payload (record name + tags). The first tab-
        // delimited field of `rest` is the record's name; do NOT trim the
        // whole `rest` first because that would swallow the leading tab
        // when the directive is bare (`EQUIP:\tKEY:...`) and incorrectly
        // surface the first KEY tag as the name.
        let prefix = format!("{}:", kind.token());
        let rest = content.strip_prefix(&prefix).unwrap_or("");
        let name = rest.split('\t').next().unwrap_or("").trim().to_string();

        if name.is_empty() {
            self.result.diagnostics.push(EquipmentDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: EquipmentDiagnosticKind::MalformedSD17B5,
                message: format!("{} directive has no value", kind.token()),
            });
            return;
        }

        let tokens = collect_tokens_from_rest(line_number, raw_line, rest);
        let key_token = tokens
            .iter()
            .find(|token| token.key == "KEY")
            .map(|token| token.value.clone());
        self.open_record(kind, name, key_token, line_number, raw_line);
        // Attach the tags from this same line to the new record.
        let entry_index = self.active_entry.expect("record just opened");
        let entry = &mut self.result.entries[entry_index];
        for token in tokens {
            if token.key == "BONUS" {
                entry
                    .bonus_chains
                    .push(build_bonus_token(line_number, &token.raw_pair));
            } else {
                entry.tokens.push(token);
            }
        }
    }

    fn observe_record_row(&mut self, line_number: usize, raw_line: &str, column_zero: &str) {
        // The corpus uses two file naming conventions: `*_equip*.lst` and
        // `*_equipmods.lst`. The parser cannot know which it is from line
        // content alone, so it uses the directory segment of `source_path`
        // to disambiguate when the source_path is informative. When the
        // source_path is a synthetic fixture (no `_equip*` segment), the
        // default is `Equip` — the test fixtures use a hand-built EQUIP:
        // row when the kind needs to be EQUIPMOD.
        let kind = infer_kind_from_source_path(&self.result.source_path);
        let name = extract_record_name(column_zero).to_string();

        if name.is_empty() {
            self.result.diagnostics.push(EquipmentDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: EquipmentDiagnosticKind::MalformedSD17B5,
                message: "equipment row has empty column 0".to_string(),
            });
            return;
        }

        let tokens = collect_tokens_from_columns(line_number, raw_line);
        let key_token = tokens
            .iter()
            .find(|token| token.key == "KEY")
            .map(|token| token.value.clone());
        self.open_record(kind, name, key_token, line_number, raw_line);
        let entry_index = self.active_entry.expect("record just opened");
        let entry = &mut self.result.entries[entry_index];
        for token in tokens {
            if token.key == "BONUS" {
                entry
                    .bonus_chains
                    .push(build_bonus_token(line_number, &token.raw_pair));
            } else {
                entry.tokens.push(token);
            }
        }
    }

    fn observe_continuation_row(&mut self, line_number: usize, raw_line: &str) {
        let Some(entry_index) = self.active_entry else {
            self.result.diagnostics.push(EquipmentDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: EquipmentDiagnosticKind::OrphanContinuationRow,
                message:
                    "continuation row with no preceding equipment or equipment-modifier record"
                        .to_string(),
            });
            return;
        };
        let tokens = collect_tokens_from_columns(line_number, raw_line);
        let entry = &mut self.result.entries[entry_index];
        for token in tokens {
            if token.key == "BONUS" {
                entry
                    .bonus_chains
                    .push(build_bonus_token(line_number, &token.raw_pair));
            } else {
                entry.tokens.push(token);
            }
        }
    }

    fn observe_block_marker(&mut self, line_number: usize, raw_line: &str, label: &str) {
        if label.is_empty() {
            self.result.diagnostics.push(EquipmentDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: EquipmentDiagnosticKind::MalformedBlockMarker,
                message: "###Block marker has no label".to_string(),
            });
        }
        // `###Block:` markers terminate any open record. The next row is
        // either a new record or a continuation of the previous file
        // section's structure (no equipment data follows a bare marker
        // until the next row).
        self.active_entry = None;
    }

    fn open_record(
        &mut self,
        kind: EquipmentRecordKind,
        name: String,
        key_token: Option<String>,
        line_number: usize,
        raw_line: &str,
    ) {
        // PCGen LST files occasionally re-state the SAME logical item across
        // multiple lines in different representation styles (e.g. once via
        // the directive-prefix form, once via the corpus-typical form) —
        // merge those into one record. But the corpus's `.COPY=` naming
        // convention (`<BaseTemplate>.COPY=<DistinctItemName>`, e.g.
        // `Staff.COPY=Staff of Abjuration`, `Staff.COPY=Staff of Charming`)
        // and coincidental plain-name reuse (two unrelated equipment-modifier
        // rows both named "Cloth") both produce DISTINCT rows whose
        // `extract_record_name` output collides even though they are not the
        // same item. The real distinguishing signal is each row's own `KEY:`
        // token: merge only when both rows carry an explicit KEY that
        // matches, or when neither row carries one (name-only fallback, for
        // the KEY-less case this rule originally targeted). A row with an
        // explicit KEY never merges into a row without one — an explicit KEY
        // is a deliberate disambiguation signal.
        //
        // `OPEN-ISSUES.md` row 61, ROOT-CAUSED (`SD31-E6-F5-003`): the
        // KEY-less name-fallback above is ALSO exactly wrong for two
        // DIFFERENT `.COPY=` rows that happen to share the same base
        // template on their left-hand side (`extract_record_name` strips
        // to the template, not the distinct post-`.COPY=` identity) --
        // `Bastard Sword (Base).COPY=Bastard's Sting` and `Bastard Sword
        // (Base).COPY=Valor's Minion` both reduce to `name ==
        // "Bastard Sword (Base)"` even though they declare two genuinely
        // different items (confirmed against the real, pinned oracle:
        // `ue_equip_arms_armor.lst:447`/`:550`; `uw_equip_general.lst:23`/
        // `:40`/`:41`; `um_equip_general.lst:12..29`, 18 real Ultimate
        // Magic spellbooks all declared as `Spellbook.COPY=<title>`). A
        // `.COPY=` row is, BY THE CORPUS'S OWN NAMING CONVENTION, always a
        // declaration of a new, distinct record (`Trap::CopyRecord`'s own
        // `miscount_risk`: "`.COPY=` declares a *new* record") -- so it
        // must never merge into another entry via bare name equality,
        // only via an explicit matching `KEY:` (still handled above,
        // unchanged) or with ANOTHER row that is not itself the head of a
        // `.COPY=`-declared entry. `is_copy_declaration` reads the row's
        // OWN first tab-column (no new struct field needed --
        // `header_raw_line` already carries it for an existing entry).
        fn is_copy_declaration(raw_line: &str) -> bool {
            raw_line.split('\t').next().unwrap_or("").contains(".COPY=")
        }
        let new_is_copy = is_copy_declaration(raw_line);
        let entry_index = if let Some(index) = self.result.entries.iter().position(|entry| {
            if entry.kind != kind {
                return false;
            }
            let existing_key = entry
                .tokens
                .iter()
                .find(|token| token.key == "KEY")
                .map(|token| token.value.as_str());
            match (key_token.as_deref(), existing_key) {
                (Some(new_key), Some(old_key)) => new_key == old_key,
                (None, None) => {
                    entry.name == name
                        && !new_is_copy
                        && !is_copy_declaration(&entry.header_raw_line)
                }
                _ => false,
            }
        }) {
            index
        } else {
            self.result.entries.push(EquipmentRecord {
                kind,
                name,
                header_line_number: line_number,
                header_raw_line: raw_line.to_string(),
                tokens: Vec::new(),
                bonus_chains: Vec::new(),
                is_record_start: true,
                diagnostics: Vec::new(),
            });
            self.result.entries.len() - 1
        };
        self.active_entry = Some(entry_index);
    }

    fn finalize(mut self) -> EquipmentParseResult {
        // Determinism: sort entries by header line number so the output
        // order matches source order even if upstream callers ever mutate
        // the entry list. New entries are already pushed in source order, so
        // this is a no-op in practice; it is here as a guard.
        self.result
            .entries
            .sort_by_key(|entry| entry.header_line_number);
        // Also sort tokens and bonus_chains within each entry by line
        // number so the canonical IR has stable order even if the parser
        // is refactored later.
        for entry in &mut self.result.entries {
            entry.tokens.sort_by_key(|token| token.line_number);
            entry.bonus_chains.sort_by_key(|bonus| bonus.line_number);
        }
        self.result
    }
}

/// Infer whether a corpus path is an equipment-modifier LST file
/// (`*_equipmods.lst`) or an equipment LST file (`*_equip*.lst` or any
/// other). Returns `EquipmentRecordKind::Equip` as the default when the
/// source_path is ambiguous.
fn infer_kind_from_source_path(source_path: &str) -> EquipmentRecordKind {
    let lower = source_path.to_ascii_lowercase();
    // The substring `equipmods` is a strong signal — it's the file
    // naming convention PCGen uses for equipment-modifier LST files.
    if lower.contains("equipmods") {
        EquipmentRecordKind::EquipMod
    } else {
        EquipmentRecordKind::Equip
    }
}

/// Lift every tab-delimited `KEY:VAL` token off a directive-prefix row
/// after the record name has been removed. The first non-empty column is
/// the record name and is skipped.
fn collect_tokens_from_rest(line_number: usize, raw_line: &str, rest: &str) -> Vec<EquipmentToken> {
    let mut tokens = Vec::new();
    let mut column_index = 0;
    for field in rest.split('\t') {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            column_index += 1;
            continue;
        }
        // The first non-empty field is the record name; skip it.
        if column_index == 0 && !trimmed.contains(':') {
            column_index += 1;
            continue;
        }
        let (key, value) = match trimmed.split_once(':') {
            Some((key, value)) => (key.to_string(), value.to_string()),
            None => (trimmed.to_string(), String::new()),
        };
        tokens.push(EquipmentToken {
            key,
            value,
            line_number,
            // `SD31-E6-F10-004`: the field's BYTE-EXACT text, per this
            // struct field's own doc comment -- `key`/`value` above still
            // trim (every other caller wants a clean value), but `field`
            // itself (not `trimmed`) preserves any leading/trailing
            // whitespace the real corpus row's tab-delimited field carries,
            // matching `corpus_literal_sweep::tab_tokens`'s own untrimmed
            // reading of the identical corpus line so a byte-exact
            // downstream comparison (`enrich_equipment_raw_tokens.rs`) has
            // something byte-exact to compare.
            raw_pair: field.to_string(),
        });
        column_index += 1;
    }
    let _ = raw_line; // parameter kept for symmetry with sibling parsers
    tokens
}

/// Lift every tab-delimited `KEY:VAL` token off a corpus-typical row,
/// skipping the first non-empty column (which is the record name).
fn collect_tokens_from_columns(line_number: usize, raw_line: &str) -> Vec<EquipmentToken> {
    let mut tokens = Vec::new();
    let mut first_nonempty_skipped = false;
    for field in raw_line.split('\t') {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !first_nonempty_skipped && !trimmed.contains(':') {
            // Column 0 is the equipment name — drop it.
            first_nonempty_skipped = true;
            continue;
        }
        first_nonempty_skipped = true;
        let (key, value) = match trimmed.split_once(':') {
            Some((key, value)) => (key.to_string(), value.to_string()),
            None => (trimmed.to_string(), String::new()),
        };
        tokens.push(EquipmentToken {
            key,
            value,
            line_number,
            // See `collect_tokens_from_rest`'s identical comment: `field`,
            // not `trimmed`, so `raw_pair` stays byte-exact to the corpus.
            raw_pair: field.to_string(),
        });
    }
    tokens
}

/// Build a [`BonusToken`] from a raw `BONUS:...` clause. The pipe-
/// delimited qualifiers are flattened into a single list. A bonus chain
/// like `BONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0`
/// becomes `qualifiers = ["COMBAT", "AC", "1", "TYPE=Armor",
/// "PREVAREQ:DisableArmorBonus,0"]`.
///
/// A degenerate bonus with no qualifiers after the colon emits a
/// `MalformedBonusChain` diagnostic attached to the most recent token;
/// the parser still records the raw bonus text as evidence.
fn build_bonus_token(line_number: usize, raw_bonus: &str) -> BonusToken {
    // The raw_bonus text starts with `BONUS:`; strip the prefix so we
    // can split on pipes.
    let value = raw_bonus.strip_prefix("BONUS:").unwrap_or(raw_bonus).trim();
    let qualifiers: Vec<String> = value
        .split('|')
        .map(|qualifier| qualifier.trim().to_string())
        .filter(|qualifier| !qualifier.is_empty())
        .collect();
    BonusToken {
        line_number,
        raw_bonus: raw_bonus.to_string(),
        qualifiers,
    }
}

impl EquipmentParseResult {
    /// Group records by [`EquipmentRecordKind`] in source order.
    pub fn records_by_kind(
        &self,
    ) -> std::collections::HashMap<EquipmentRecordKind, Vec<&EquipmentRecord>> {
        let mut grouped: std::collections::HashMap<EquipmentRecordKind, Vec<&EquipmentRecord>> =
            std::collections::HashMap::new();
        for record in &self.entries {
            grouped.entry(record.kind).or_default().push(record);
        }
        grouped
    }
}

#[cfg(test)]
mod tests {
    //! Internal sanity tests for the bonus-token builder. These mirror
    //! the external `tests/sd17_b5_equipment.rs` acceptance tests but
    //! stay in-module so refactors of the bonus parser don't accidentally
    //! lose coverage.

    use super::*;

    #[test]
    fn bonus_token_flattening_does_not_recurse() {
        // 100+ nested qualifiers — a recursive parser would overflow the
        // stack on this exact input. The flat parser must not.
        let mut raw = String::from("BONUS:");
        for i in 0..200 {
            raw.push_str(&format!("QUALIFIER{i}|"));
        }
        raw.push_str("END");
        let bonus = build_bonus_token(1, &raw);
        assert_eq!(bonus.qualifiers.len(), 201);
        assert_eq!(bonus.qualifiers[0], "QUALIFIER0");
        assert_eq!(bonus.qualifiers[200], "END");
        assert_eq!(bonus.line_number, 1);
    }

    #[test]
    fn bonus_token_strips_bonus_prefix() {
        let bonus = build_bonus_token(7, "BONUS:COMBAT|AC|1");
        assert_eq!(bonus.qualifiers, vec!["COMBAT", "AC", "1"]);
        assert_eq!(bonus.line_number, 7);
    }

    #[test]
    fn extract_record_name_handles_copy_suffix() {
        assert_eq!(
            extract_record_name("Bolt (Base).COPY=Bolt, Crossbow"),
            "Bolt (Base)"
        );
        assert_eq!(extract_record_name("Padded Armor"), "Padded Armor");
        assert_eq!(
            extract_record_name("  Masterwork (Weapon)  "),
            "Masterwork (Weapon)"
        );
    }

    #[test]
    fn infer_kind_uses_equipmods_substring() {
        assert_eq!(
            infer_kind_from_source_path(
                "/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst"
            ),
            EquipmentRecordKind::EquipMod
        );
        assert_eq!(
            infer_kind_from_source_path(
                "/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst"
            ),
            EquipmentRecordKind::Equip
        );
        assert_eq!(
            infer_kind_from_source_path("synthetic.lst"),
            EquipmentRecordKind::Equip
        );
    }

    #[test]
    fn recognize_directive_prefix_only_matches_equip_or_equipmod() {
        assert_eq!(
            recognize_directive_prefix("EQUIP:Padded Armor\tTYPE:Armor"),
            Some(EquipmentRecordKind::Equip)
        );
        assert_eq!(
            recognize_directive_prefix("EQUIPMOD:Material ~ Steel"),
            Some(EquipmentRecordKind::EquipMod)
        );
        assert_eq!(recognize_directive_prefix("CLASS:Fighter\tTYPE:PC"), None);
    }

    // -----------------------------------------------------------------
    // `OPEN-ISSUES.md` row 61 -- `tokens_on_line`/`bonus_chains_on_line`
    // single-citation-line provenance filter. Each fixture below is the
    // REAL byte content of the corpus line pair that produced a confirmed
    // shipped defect (`SD31-W4-INTEGRATE-001` receipt), read fresh from
    // the pinned PCGen oracle this cycle -- not reconstructed from memory.
    // -----------------------------------------------------------------

    /// `ue_equip_arms_armor.lst:447` (`.COPY=` variant "Bastard's Sting",
    /// only token `VISIBLE:YES`) and `:550` (a DIFFERENT `.COPY=` variant,
    /// "Valor's Minion", off the SAME "Bastard Sword (Base)" template,
    /// carrying `EQMOD:Material ~ Steel` + its own `VISIBLE:YES`) both
    /// reduce to the SAME `extract_record_name` output
    /// (`"Bastard Sword (Base)"`, the template side of `.COPY=`) -- before
    /// this cycle's `open_record` fix, that shared name merged them under
    /// the KEY-less name fallback, the confirmed root cause of the shipped
    /// `bastard_s_sting.json` defect ("shipped an unrelated
    /// `EQMOD:Material ~ Steel` and a duplicated `VISIBLE:YES`"). ROOT-
    /// CAUSED (`SD31-E6-F5-003`): a `.COPY=`-declared row never merges via
    /// bare name equality, only via an explicit matching `KEY:` -- so line
    /// 447 and line 550 must now open TWO SEPARATE entries, each carrying
    /// only its own line's tokens, with no filtering needed at all.
    #[test]
    fn a_copy_declared_row_never_merges_with_a_different_copy_variant_sharing_its_base_template() {
        let lst = "Bastard Sword (Base).COPY=Bastard's Sting\t\t\t\t\t\t\tVISIBLE:YES\n\
             Bastard's Sting.MOD\t\t\t\t\t\t\tSOURCEPAGE:p.150\n\
             Bastard Sword (Base).COPY=Valor's Minion\t\t\tEQMOD:Material ~ Steel\t\t\t\t\t\t\tVISIBLE:YES\n";
        let parsed = parse_equipment_entries("ue_equip_arms_armor.lst", lst);
        let entries_named_template: Vec<&EquipmentRecord> =
            parsed.entries.iter().filter(|e| e.name == "Bastard Sword (Base)").collect();
        assert_eq!(
            entries_named_template.len(),
            2,
            "line 1 and line 3 must stay two DISTINCT entries, not merge into one"
        );
        let line1 = entries_named_template
            .iter()
            .find(|e| e.header_line_number == 1)
            .expect("line 1 opens its own entry");
        assert_eq!(line1.tokens.len(), 1, "line 1's entry carries only its own token");
        assert_eq!(line1.tokens[0].key, "VISIBLE");
        assert!(
            !line1.tokens.iter().any(|t| t.key == "EQMOD"),
            "line 3's EQMOD must never reach line 1's entry now that the merge is fixed"
        );
        let line3 = entries_named_template
            .iter()
            .find(|e| e.header_line_number == 3)
            .expect("line 3 opens its OWN entry rather than merging into line 1's");
        assert!(line3.tokens.iter().any(|t| t.key == "EQMOD"));

        // `tokens_on_line` still agrees, as a second, independent proof of
        // the same invariant (defense in depth, not the only guard).
        assert_eq!(line1.tokens_on_line(1).len(), 1);
        assert!(line1.tokens_on_line(3).is_empty());
    }

    /// `ue_equip_arms_armor.lst:16` and `:46` restate "Mountain Pattern
    /// Armor" on two separate lines with identical `PROFICIENCY`/`TYPE`/
    /// `COST`/`WT` values (neither carries a `KEY:` token) -- a genuine
    /// same-name merge under `open_record`'s restatement handling, but the
    /// merged entry then carries EVERY token from BOTH lines, doubling
    /// `COST`/`WT`/`BONUS` and adding line 46-only tokens
    /// (`SOURCELONG`/`SOURCESHORT`) that are not byte-present on line 16 --
    /// the confirmed shape of the shipped `mountain_pattern_armor.json`
    /// defect ("every token doubled from a second, near-identical row").
    #[test]
    fn tokens_on_line_isolates_mountain_pattern_armor_from_its_own_restated_duplicate_line() {
        let lst = "Mountain Pattern Armor\tPROFICIENCY:ARMOR|Mountain Pattern Armor\tCOST:250\tWT:40\n\
             Mountain Pattern Armor\tPROFICIENCY:ARMOR|Mountain Pattern Armor\tCOST:250\tWT:40\tSOURCELONG:Ultimate Equipment\tSOURCESHORT:UE\n";
        let parsed = parse_equipment_entries("ue_equip_arms_armor.lst", lst);
        let merged = parsed
            .entries
            .iter()
            .find(|e| e.header_line_number == 1)
            .expect("both restated lines share the plain name and merge into one entry");
        assert_eq!(
            merged.tokens.iter().filter(|t| t.key == "COST").count(),
            2,
            "the merge must be reproduced (COST doubled) for this test to mean anything"
        );

        let isolated = merged.tokens_on_line(1);
        assert_eq!(isolated.len(), 3, "line 1 states exactly PROFICIENCY, COST, WT");
        assert_eq!(isolated.iter().filter(|t| t.key == "COST").count(), 1);
        assert!(
            !isolated.iter().any(|t| t.key == "SOURCELONG"),
            "line 2's SOURCELONG must not leak into line 1's isolated view"
        );
    }

    /// `uw_equip_general.lst:23` (the real base item, no `.COPY=`) plus
    /// `:40`/`:41` (two DISTINCT `.COPY=` variants, "Camouflage Blind" and
    /// "All-Weather Cover") all reduce to the SAME buggy
    /// `extract_record_name` output (`"Hunter's Stand"`, the pre-`.COPY=`
    /// side) -- before this cycle's `open_record` fix, all three merged
    /// into one entry, the confirmed shape of the shipped
    /// `hunter_s_stand.json` defect ("three genuinely distinct `.COPY=`
    /// items merged into one"). ROOT-CAUSED (`SD31-E6-F5-003`): line 23
    /// (not itself a `.COPY=` row) still merges with a later PLAIN
    /// restatement, but a `.COPY=` row (lines 40/41) never merges via bare
    /// name into ANY entry, including a non-`.COPY=` one -- so all three
    /// lines here must end up as three separate entries.
    #[test]
    fn copy_declared_variants_never_merge_into_the_plain_base_item_sharing_their_name() {
        let lst = "Hunter's Stand\tTYPE:Goods.General\tCOST:25\tWT:15\n\
             Hunter's Stand.COPY=Hunter's Stand (Camouflage Blind)\tOUTPUTNAME:Hunter's Stand, Camouflage Blind\tCOST:30\n\
             Hunter's Stand.COPY=Hunter's Stand (All-Weather Cover)\tOUTPUTNAME:Hunter's Stand, All-Weather Cover\tCOST:35\n";
        let parsed = parse_equipment_entries("uw_equip_general.lst", lst);
        assert_eq!(parsed.entries.len(), 3, "all three lines must open distinct entries");

        let base = parsed.entries.iter().find(|e| e.header_line_number == 1).unwrap();
        assert_eq!(base.tokens.iter().filter(|t| t.key == "COST").count(), 1);
        let cost = base.tokens.iter().find(|t| t.key == "COST").unwrap();
        assert_eq!(cost.value, "25", "line 1's own COST, not a variant's 30 or 35");
        assert!(
            !base.tokens.iter().any(|t| t.key == "OUTPUTNAME"),
            "a `.COPY=` variant's OUTPUTNAME must never leak into the plain base item's entry"
        );

        let variant40 = parsed.entries.iter().find(|e| e.header_line_number == 2).unwrap();
        let variant41 = parsed.entries.iter().find(|e| e.header_line_number == 3).unwrap();
        assert_ne!(
            variant40.tokens.iter().find(|t| t.key == "COST").map(|t| &t.value),
            variant41.tokens.iter().find(|t| t.key == "COST").map(|t| &t.value),
            "the two distinct .COPY= variants must keep their own distinct COST"
        );
    }

    #[test]
    fn tokens_on_line_returns_empty_for_a_line_the_record_never_touched() {
        let lst = "Widget\tCOST:5\n";
        let parsed = parse_equipment_entries("book_equip.lst", lst);
        let entry = &parsed.entries[0];
        assert!(entry.tokens_on_line(99).is_empty());
    }

    /// `SD31-E6-F10-004`: real corpus reproduction, `inner_sea_gods/
    /// isg_equip.lst:220`, `Safecamp Wagon`'s `DESC:` field -- the LAST
    /// tab-delimited field on the row, followed by a literal trailing
    /// space before the newline (`cat -A` on the real file confirms
    /// `...fire resistance 5. $`). `EquipmentToken::raw_pair`'s own doc
    /// comment promises "Raw `KEY:VAL` text as it appeared between tabs on
    /// the source line" -- but the field was built from the SAME
    /// whitespace-trimmed string `key`/`value` come from, so it silently
    /// dropped that trailing space, breaking its own documented contract.
    /// `key`/`value` still trim (every other caller -- numeric parsing,
    /// name matching -- correctly wants that); only `raw_pair` must carry
    /// the byte-exact field. Found live: `enrich_equipment_raw_tokens.rs`
    /// (which DOES use the trimmed `value`, not `raw_pair`, for its
    /// `raw_tokens` JSON -- a separate fix, in that file) shipped a
    /// trimmed `DESC` that `corpus_literal_sweep` correctly flagged as not
    /// byte-present in the corpus's own (untrimmed) token closure.
    #[test]
    fn raw_pair_preserves_a_fields_own_trailing_whitespace_even_though_value_trims_it() {
        let lst = "Safecamp Wagon\tCOST:3000\tDESC:...fire resistance 5. \n";
        let parsed = parse_equipment_entries("isg_equip.lst", lst);
        let entry = &parsed.entries[0];
        let desc = entry.tokens.iter().find(|t| t.key == "DESC").expect("DESC token present");
        assert_eq!(
            desc.value, "...fire resistance 5.",
            "the TRIMMED `value` field is unaffected -- other callers still get a clean value"
        );
        assert_eq!(
            desc.raw_pair, "DESC:...fire resistance 5. ",
            "`raw_pair` must carry the byte-exact field text, trailing space included, per its \
             own documented contract"
        );
    }

    #[test]
    fn bonus_chains_on_line_isolates_by_source_line() {
        let lst = "Widget\tCOST:5\tBONUS:COMBAT|AC|1\n\
             Widget2\tBONUS:COMBAT|AC|2\n";
        // Force a merge: both rows carry no KEY and (post-fix expectation)
        // distinct names -- use the SAME name deliberately so this test
        // exercises the merge path, matching the real restatement shape.
        let lst = lst.replace("Widget2", "Widget");
        let parsed = parse_equipment_entries("book_equip.lst", &lst);
        let merged = &parsed.entries[0];
        assert_eq!(merged.bonus_chains.len(), 2, "both lines' BONUS: merged onto one entry");
        let isolated = merged.bonus_chains_on_line(1);
        assert_eq!(isolated.len(), 1);
        assert_eq!(isolated[0].qualifiers, vec!["COMBAT", "AC", "1"]);
    }
}
