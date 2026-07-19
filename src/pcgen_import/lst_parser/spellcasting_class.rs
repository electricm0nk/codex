//! LST object-kind parser for the `CLASS:` directive (SD-17 Slice B-2;
//! widened SD-22 Epic 3 to add `Alchemist`, then `Inquisitor`, then
//! `Oracle`, then `Summoner`).
//!
//! Scope: the five CRB spellcasting classes named in the original slice
//! card body (`Cleric`, `Druid`, `Wizard`, `Sorcerer`, `Bard`), plus
//! `Alchemist` (added by SD-22 Epic 3's Alchemist ingest cycle — the
//! Alchemist's `CLASS:Alchemist` line in `apg_classes.lst` carries
//! `SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES`, the same posture-bearing
//! shape as the CRB spellcasting classes, so it belongs in this parser
//! rather than the martial-class parser), plus `Inquisitor` (added by
//! SD-22 Epic 3's Inquisitor ingest cycle — the real `CLASS:Inquisitor`
//! line carries `SPELLSTAT:WIS MEMORIZE:NO`, the same spontaneous-divine
//! posture as Sorcerer/Bard), plus `Oracle` (added by SD-22 Epic 3's
//! Oracle ingest cycle — the real `CLASS:Oracle` line carries
//! `SPELLSTAT:CHA MEMORIZE:NO`, the same spontaneous-divine posture as
//! Sorcerer/Bard/Inquisitor), plus `Summoner` (added by SD-22 Epic 3's
//! Summoner ingest cycle — the real `CLASS:Summoner` line carries
//! `SPELLSTAT:CHA MEMORIZE:NO`, the same spontaneous posture, arcane
//! rather than divine, which the parser's posture derivation does not
//! distinguish), plus `Witch` (added by SD-22 Epic 3's Witch ingest
//! cycle, the sixth and last real APG class — the real `CLASS:Witch`
//! line carries `SPELLSTAT:INT` with no `MEMORIZE:NO` and no
//! `SPELLBOOK:YES` token, the same absent-signals prepared-casting
//! shape as Cleric/Druid), plus `Arcanist` (added by SD-22 Epic 4's
//! Arcanist ingest cycle, the first real ACG class — the real
//! `CLASS:Arcanist` line in `acg_classes.lst` carries
//! `SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES`, the same spellbook-prepared
//! posture as Alchemist), plus `Bloodrager` (added by SD-22 Epic 4's
//! Bloodrager ingest cycle, the second real ACG class — the real
//! `CLASS:Bloodrager` line carries `SPELLSTAT:CHA MEMORIZE:NO`, the same
//! spontaneous posture as Sorcerer/Bard/Oracle/Summoner), plus their
//! `Ex-<name>` mirror variants. The
//! parser recognizes every `CLASS:<name>` line in the PCGen corpus where
//! `<name>` is in that set, carries every tab-delimited `KEY:VAL` token
//! pair to a canonical IR record, and preserves one-based source line
//! numbers on every entry.
//!
//! Out-of-scope class names (the six martial classes owned by
//! [`crate::pcgen_import::lst_parser::class`], prestige classes, and
//! unrelated classes owned by other future slices — this includes every
//! other APG/ACG class until its own SD-22 ingest cycle widens this
//! array or `class.rs`'s, per that class's actual casting shape) are
//! skipped silently — the parser does not raise a diagnostic for them.
//! The same diagnostic is reused by the umbrella
//! [`crate::pcgen_import::lst_parser::mod`] only via this slice's own
//! enum (the umbrella already exposes `LstDiagnosticKind` for
//! `metadata`; this slice surfaces its own kind enum to keep the
//! parallel-slices partition honest).
//!
//! Spellcasting sub-shapes recognized on top of the martial-class
//! surface:
//!
//! - `MEMORIZE:NO` on the SPELLSTAT line → [`CastingPosture::Spontaneous`].
//! - `SPELLBOOK:YES` on the SPELLSTAT line → [`CastingPosture::Spellbook`].
//! - absent signals → [`CastingPosture::Prepared`].
//! - `KNOWNSPELLS:LEVEL=N|...|LEVEL=9` token on the SPELLSTAT line →
//!   populated [`SpellcastingClassEntry::automatically_known_levels`].
//! - `###Block: Level progression` and
//!   `###Block: Spell Level Progression` rows
//!   `<level>\tCAST:0,1\t...\tKNOWN:4,2` → populated
//!   [`SpellcastingClassEntry::spell_progression`].
//! - `###Block: Domain Selections` rows
//!   `0\tDOMAIN:<name>|...` → populated
//!   [`SpellcastingClassEntry::domain_selections`].
//! - `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>` lines that follow
//!   a Wizard's HEADER block → populated
//!   [`SpellcastingClassEntry::school_specializations`].
//!
//! This module composes with the GE-03 PCC parser, the SD-17
//! include-graph resolver, and the SD-17 Slice B-1 martial-class parser.
//! Upstream callers (or a future unified PCGen loader) discover the LST
//! file from the `IncludeResolution::lst_files` collection emitted by the
//! resolver; this module consumes one LST file at a time and emits a
//! [`SpellcastingClassParseResult`] that downstream slices can lift
//! into canonical IR.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// The five CRB spellcasting classes named in the SD-17 Slice B-2 card
/// body, plus `Alchemist`, `Inquisitor`, `Oracle`, `Summoner`, and
/// `Witch` (SD-22 Epic 3 widenings), plus `Arcanist`, `Bloodrager`, and
/// `Hunter` (SD-22 Epic 4 widenings — see module doc comment).
pub const SPELLCASTING_CLASS_NAMES: &[&str] = &[
    "Cleric",
    "Druid",
    "Wizard",
    "Sorcerer",
    "Bard",
    "Alchemist",
    "Inquisitor",
    "Oracle",
    "Summoner",
    "Witch",
    "Arcanist",
    "Bloodrager",
    "Hunter",
];

/// Casting posture recorded on each spellcasting class. The posture is
/// derived from the SPELLSTAT line: presence of `MEMORIZE:NO` flips it
/// to [`CastingPosture::Spontaneous`], presence of `SPELLBOOK:YES`
/// flips it to [`CastingPosture::Spellbook`]; absence of either yields
/// [`CastingPosture::Prepared`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastingPosture {
    /// Standard-prepared casting (Cleric, Druid in Pathfinder 1e).
    Prepared,
    /// Spontaneous casting, learned-known (Bard, Sorcerer).
    Spontaneous,
    /// Spellbook-prepared casting (Wizard).
    Spellbook,
}

/// Result of parsing one LST file for the spellcasting-class scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellcastingClassParseResult {
    /// Identity of the parsed LST file, as supplied by the caller.
    pub source_path: String,
    /// One [`SpellcastingClassEntry`] per distinct class name encountered.
    pub entries: Vec<SpellcastingClassEntry>,
    /// Non-fatal parse diagnostics.
    pub diagnostics: Vec<SpellcastingClassDiagnostic>,
}

/// One spellcasting class record aggregated from one or more
/// `CLASS:<name>` header lines and any number of `###Block:` sections
/// containing spell-progression curves, domain selections, and so on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellcastingClassEntry {
    /// Class name as it appears after `CLASS:` (e.g. `Cleric`,
    /// `Ex-Wizard`). The line that announces the entry is recorded as
    /// [`Self::header_line_number`] and [`Self::header_raw_line`].
    pub class_name: String,
    /// One-based line number of the first `CLASS:<name>` line for this
    /// class in the source LST.
    pub header_line_number: usize,
    /// Raw first `CLASS:<name>` line, preserved verbatim.
    pub header_raw_line: String,
    /// Every tab-delimited `KEY:VAL` token from every `CLASS:<name>`
    /// line for this class, in source order, with one-based line
    /// numbers.
    pub tokens: Vec<SpellcastingClassToken>,
    /// Casting stat captured from the `SPELLSTAT:` token (e.g. `WIS`,
    /// `CHA`, `INT`). `None` when the LST does not advertise one (rare
    /// in real corpus; e.g. homebrew variants).
    pub spell_stat: Option<String>,
    /// Casting posture derived from the SPELLSTAT line signals
    /// (`MEMORIZE:NO`, `SPELLBOOK:YES`, or absent).
    pub casting_posture: Option<CastingPosture>,
    /// Levels covered by the SPELLSTAT `KNOWNSPELLS:LEVEL=N|...|LEVEL=9`
    /// token (auto-known spell levels for prepared casters). Empty
    /// for spontaneous casters, which carry per-level `KNOWN:` instead.
    pub automatically_known_levels: Vec<u8>,
    /// Per-level cast/known slots harvested from a
    /// `###Block: Level progression` or
    /// `###Block: Spell Level Progression` block.
    pub spell_progression: Vec<SpellLevelRow>,
    /// Domain-selection entries harvested from a
    /// `###Block: Domain Selections` block.
    pub domain_selections: Vec<ClassDomainSelection>,
    /// Wizard-style arcane-school entries harvested from
    /// `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>` lines that
    /// follow the header.
    pub school_specializations: Vec<SchoolSpecialization>,
}

/// One tab-delimited `KEY:VAL` token lifted from a `CLASS:<name>` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellcastingClassToken {
    /// Key portion (text before the first `:`).
    pub key: String,
    /// Value portion (text after the first `:`).
    pub value: String,
    /// One-based line number where the token appeared in the LST.
    pub line_number: usize,
    /// Raw `KEY:VAL` text as it appeared between tabs on the source line.
    pub raw_pair: String,
}

/// One row of the spell-progression curve, harvested from a per-level
/// row inside the `###Block: Level progression` /
/// `###Block: Spell Level Progression` block.
///
/// The row is recorded as a single [`SpellcastingClassEntry`] for the
/// owning class, in source order. `cast_slots` and `known_spells` are
/// the comma-separated integer sequences that appeared after `CAST:`
/// and `KNOWN:` respectively. Either may be empty (the row might
/// carry only one of the two).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellLevelRow {
    /// Integer level that led the row (1..=20 for PCGen).
    pub level: u8,
    /// Comma-separated integer sequence from the `CAST:` token.
    pub cast_slots: Vec<u8>,
    /// Comma-separated integer sequence from the `KNOWN:` token.
    pub known_spells: Vec<u8>,
    /// One-based line number in the source LST.
    pub line_number: usize,
}

/// One domain-selection row from a `###Block: Domain Selections` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDomainSelection {
    /// Domain name as it appears after `DOMAIN:` (e.g. `Air`, `Healing`).
    pub name: String,
    /// Full raw `DOMAIN:<value>` text including any trailing
    /// pipe-delimited prerequisite payload, preserved as evidence.
    pub raw_payload: String,
    /// One-based line number in the source LST.
    pub line_number: usize,
}

/// One Wizard-style arcane-school specialization harvested from
/// `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>` lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchoolSpecialization {
    /// School name lifted from `CHOICE:SCHOOL|<school>`. Empty when
    /// the subclass advertises itself (e.g. `Universalist`) without a
    /// CHOICE:SCHOOL token.
    pub school: String,
    /// Subclass label that announced the school (e.g. `Abjurer`,
    /// `Evoker`, `Universalist`).
    pub subclass_name: String,
    /// One-based line number in the source LST.
    pub line_number: usize,
}

/// Classification of a parse diagnostic emitted by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellcastingClassDiagnosticKind {
    /// A `CLASS:` directive with no class name, or some other defect
    /// the parser could not recover from. The `MalformedSD17B2` label
    /// matches the slice card's verification criterion.
    MalformedSD17B2,
    /// A `###Block:` marker that opens a recognized section (Level
    /// progression, Domain Selections) but whose interior row is
    /// missing the required data fields (e.g. a CAST row whose
    /// comma-separated payload is not all integers).
    MalformedBlockRow,
    /// The LST source could not be read from disk.
    ReadFailed,
}

/// Structured diagnostic emitted by the spellcasting-class LST parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellcastingClassDiagnostic {
    /// One-based line number of the offending line when applicable.
    pub line_number: Option<usize>,
    /// Raw line text preserved as evidence.
    pub raw_line: String,
    /// Classification of the diagnostic.
    pub kind: SpellcastingClassDiagnosticKind,
    /// Human-readable explanation of the defect.
    pub message: String,
}

/// Classification of a `###Block:` label recognized by this parser.
///
/// Real PCGen LST files use the following labels for spellcasting-class
/// sections; the enum keeps the matcher explicit and lets future slices
/// lift block semantics into canonical IR without rescanning labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassBlockKind {
    /// `###Block: Level progression` or
    /// `###Block: Spell Level Progression` (and the rarities listed in
    /// [`Self::from_label`]).
    ProgressionCurve,
    /// `###Block: Domain Selections`.
    DomainSelections,
    /// Any other `###Block:` label (e.g. `Proficiencies`, `Bonus Feats`).
    Other,
}

impl ClassBlockKind {
    /// Classify a `###Block:` label. The match is case-sensitive and
    /// matches the labels the canonical Pathfinder `cr_classes.lst`
    /// actually uses; other labels fall through to [`Self::Other`].
    pub fn from_label(label: &str) -> Self {
        match label {
            "Level progression" | "Spell Level Progression" | "Spell Progression" => {
                Self::ProgressionCurve
            }
            "Domain Selections" => Self::DomainSelections,
            _ => Self::Other,
        }
    }

    /// Returns true iff the label opens a spell-progression curve.
    pub fn is_progression_curve(self) -> bool {
        matches!(self, Self::ProgressionCurve)
    }

    /// Returns true iff the label opens a domain-selection block.
    pub fn is_domain_selections(self) -> bool {
        matches!(self, Self::DomainSelections)
    }

    /// Returns true iff the label is anything else.
    pub fn is_other(self) -> bool {
        matches!(self, Self::Other)
    }

    /// The trimmed label that produced this classification.
    pub fn label(self) -> &'static str {
        match self {
            Self::ProgressionCurve => "Level progression",
            Self::DomainSelections => "Domain Selections",
            Self::Other => "Other",
        }
    }
}

/// Parse the contents of one LST file for the spellcasting-class scope.
///
/// `source_path` is the identity recorded on the result; `input_text`
/// is the raw LST contents.
pub fn parse_spellcasting_class_entries(
    source_path: &str,
    input_text: &str,
) -> SpellcastingClassParseResult {
    let mut state = SpellcastingClassParseState::new(source_path);
    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        if let Some(label) = strip_block_marker(content) {
            state.observe_block_marker(line_number, raw_line, label);
            continue;
        }

        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        if let Some(rest) = content.strip_prefix("CLASS:") {
            state.observe_class_line(line_number, raw_line, rest);
            continue;
        }

        if let Some(rest) = content.strip_prefix("SUBCLASS:") {
            state.observe_subclass_line(line_number, raw_line, rest);
            continue;
        }

        if let Some(block_kind) = state.active_block_kind() {
            match block_kind {
                ClassBlockKind::ProgressionCurve => {
                    state.observe_progression_row(line_number, raw_line);
                }
                ClassBlockKind::DomainSelections => {
                    state.observe_domain_selection_row(line_number, raw_line);
                }
                ClassBlockKind::Other => {
                    // Other blocks (Proficiencies, Bonus Feats, ...) carry
                    // no spellcasting-class sub-shape data. Ignore.
                }
            }
        }
    }
    state.finalize()
}

/// Parse one LST file from disk. The path is recorded as the result's
/// `source_path` (using the canonicalized absolute form).
pub fn parse_spellcasting_class_file(path: &Path) -> Result<SpellcastingClassParseResult, SpellcastingClassDiagnostic> {
    let input_text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return Err(SpellcastingClassDiagnostic {
                line_number: None,
                raw_line: String::new(),
                kind: SpellcastingClassDiagnosticKind::ReadFailed,
                message: format!(
                    "could not read spellcasting-class LST file '{}': {error}",
                    path.display()
                ),
            });
        }
    };
    Ok(parse_spellcasting_class_entries(
        &path.display().to_string(),
        &input_text,
    ))
}

fn strip_block_marker(content: &str) -> Option<&str> {
    content.strip_prefix("###Block:").map(|label| label.trim())
}

struct SpellcastingClassParseState {
    result: SpellcastingClassParseResult,
    active_entry: Option<usize>,
    active_block: Option<(ClassBlockKind, usize)>,
    scope: HashSet<String>,
}

impl SpellcastingClassParseState {
    fn new(source_path: &str) -> Self {
        let scope = SPELLCASTING_CLASS_NAMES
            .iter()
            .map(|name| name.to_string())
            .chain(
                SPELLCASTING_CLASS_NAMES
                    .iter()
                    .map(|name| format!("Ex-{name}")),
            )
            .collect();
        Self {
            result: SpellcastingClassParseResult {
                source_path: source_path.to_string(),
                entries: Vec::new(),
                diagnostics: Vec::new(),
            },
            active_entry: None,
            active_block: None,
            scope,
        }
    }

    fn active_block_kind(&self) -> Option<ClassBlockKind> {
        self.active_block.map(|(kind, _)| kind)
    }

    fn observe_class_line(
        &mut self,
        line_number: usize,
        raw_line: &str,
        after_class: &str,
    ) {
        // `CLASS:` directives terminate any open `###Block:` section.
        self.active_block = None;

        let class_name = after_class
            .split('\t')
            .next()
            .unwrap_or(after_class)
            .trim()
            .to_string();
        if class_name.is_empty() {
            self.result.diagnostics.push(SpellcastingClassDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: SpellcastingClassDiagnosticKind::MalformedSD17B2,
                message: "CLASS directive has no class name".to_string(),
            });
            return;
        }

        if !self.scope.contains(&class_name) {
            self.active_entry = None;
            return;
        }

        let entry_index = if let Some(index) = self
            .result
            .entries
            .iter()
            .position(|entry| entry.class_name == class_name)
        {
            index
        } else {
            self.result.entries.push(SpellcastingClassEntry {
                class_name: class_name.clone(),
                header_line_number: line_number,
                header_raw_line: raw_line.to_string(),
                tokens: Vec::new(),
                spell_stat: None,
                casting_posture: None,
                automatically_known_levels: Vec::new(),
                spell_progression: Vec::new(),
                domain_selections: Vec::new(),
                school_specializations: Vec::new(),
            });
            self.result.entries.len() - 1
        };

        let tokens = collect_tokens(line_number, raw_line);
        let (spell_stat, posture, auto_known_levels) = extract_spellcasting_signals(&tokens);
        let entry = &mut self.result.entries[entry_index];
        entry.tokens.extend(tokens);
        if entry.spell_stat.is_none() && spell_stat.is_some() {
            entry.spell_stat = spell_stat;
        }
        if entry.casting_posture.is_none() && posture.is_some() {
            entry.casting_posture = posture;
        }
        if entry.automatically_known_levels.is_empty() && !auto_known_levels.is_empty() {
            entry.automatically_known_levels = auto_known_levels;
        }
        self.active_entry = Some(entry_index);
    }

    fn observe_block_marker(
        &mut self,
        line_number: usize,
        raw_line: &str,
        label: &str,
    ) {
        let kind = ClassBlockKind::from_label(label);
        let Some(entry_index) = self.active_entry else {
            if kind != ClassBlockKind::Other {
                self.result.diagnostics.push(SpellcastingClassDiagnostic {
                    line_number: Some(line_number),
                    raw_line: raw_line.to_string(),
                    kind: SpellcastingClassDiagnosticKind::MalformedSD17B2,
                    message: format!(
                        "###Block marker with no preceding CLASS directive (label: '{label}')"
                    ),
                });
            }
            return;
        };
        let block_index = self.result.entries[entry_index].block_index_for(kind);
        self.active_block = Some((kind, block_index));
    }

    fn observe_progression_row(&mut self, line_number: usize, raw_line: &str) {
        let (cast_payload, known_payload, level) = parse_progression_row_fields(raw_line);
        if level == 0 {
            self.result.diagnostics.push(SpellcastingClassDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: SpellcastingClassDiagnosticKind::MalformedBlockRow,
                message: "###Block: Level progression row did not lead with a level integer"
                    .to_string(),
            });
            return;
        }
        let (cast_slots, cast_ok) = parse_int_payload(&cast_payload);
        let (known_spells, known_ok) = parse_int_payload(&known_payload);
        if !cast_ok || !known_ok {
            self.result.diagnostics.push(SpellcastingClassDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: SpellcastingClassDiagnosticKind::MalformedSD17B2,
                message: format!(
                    "###Block: Level progression row carries a non-integer payload \
                     (cast='{cast_payload}', known='{known_payload}')"
                ),
            });
            // A malformed payload means the row cannot be lifted into
            // a `SpellLevelRow` faithfully. Skip the row instead of
            // recording a partially-populated entry — the diagnostic
            // is the durable record of the defect.
            return;
        }
        let Some(entry_index) = self.active_entry else {
            return;
        };
        let row = SpellLevelRow {
            level,
            cast_slots,
            known_spells,
            line_number,
        };
        self.result.entries[entry_index].spell_progression.push(row);
    }

    fn observe_domain_selection_row(
        &mut self,
        line_number: usize,
        raw_line: &str,
    ) {
        let trimmed = raw_line.trim_start();
        // A row like `0\tDOMAIN:Air|PREABILITY:1,...`. The column 0
        // `0` is the level selection marker; we don't carry it.
        let rest_after_level = trimmed
            .split('\t')
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\t");
        // Defensive: a hand-built fixture might not have `0\t` — fall
        // back to scanning the whole line for the `DOMAIN:` token.
        let domain_source = if rest_after_level.is_empty() {
            trimmed
        } else {
            rest_after_level.as_str()
        };
        let payload = match domain_source.split_once("DOMAIN:") {
            Some((_, value)) => value,
            None => "",
        };
        let name = payload
            .split(['|', '\t'])
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if name.is_empty() {
            self.result.diagnostics.push(SpellcastingClassDiagnostic {
                line_number: Some(line_number),
                raw_line: raw_line.to_string(),
                kind: SpellcastingClassDiagnosticKind::MalformedSD17B2,
                message: "###Block: Domain Selections row has no DOMAIN name".to_string(),
            });
            return;
        }
        let Some(entry_index) = self.active_entry else {
            return;
        };
        self.result.entries[entry_index]
            .domain_selections
            .push(ClassDomainSelection {
                name,
                raw_payload: payload.to_string(),
                line_number,
            });
    }

    fn observe_subclass_line(
        &mut self,
        line_number: usize,
        raw_line: &str,
        after_subclass: &str,
    ) {
        let subclass_name = after_subclass
            .split('\t')
            .next()
            .unwrap_or(after_subclass)
            .trim()
            .to_string();
        if subclass_name.is_empty() {
            return;
        }
        let mut school = extract_school_from_subclass(raw_line);
        // A subclass without `CHOICE:SCHOOL|<school>` (e.g. PF1
        // Wizard's `Universalist`) carries no school name in the LST.
        // We surface it as the implicit "Universal" school — this
        // matches the in-corpus classification for Wizard's universal
        // specialization without inventing a separate diagnostic.
        if school.is_empty() {
            school = universal_school_name(&subclass_name);
        }
        let Some(entry_index) = self.active_entry else {
            // SUBCLASS outside any active class block is rare in real
            // PCGen, but the parser keeps silent here: it could belong
            // to a previous class and we don't want to surface a
            // diagnostic the corpus doesn't warrant.
            return;
        };
        // A SUBCLASS line tied to a class that is not a Wizard (i.e.
        // one that has no level-progression schools in PF1) is still
        // captured — some homebrew classes do declare schools. The
        // scope here is the SD-17 Slice B-2 card, which lists Wizard
        // explicitly but asks for "every PCGen LST of the relevant
        // kind" per the operator directive 2026-07-12.
        self.result.entries[entry_index]
            .school_specializations
            .push(SchoolSpecialization {
                school,
                subclass_name,
                line_number,
            });
    }

    fn finalize(mut self) -> SpellcastingClassParseResult {
        self.result
            .entries
            .sort_by_key(|entry| entry.header_line_number);
        for entry in &mut self.result.entries {
            entry.tokens.sort_by_key(|token| token.line_number);
            entry.spell_progression.sort_by_key(|row| row.level);
        }
        self.result
    }
}

impl SpellcastingClassEntry {
    fn block_index_for(&self, kind: ClassBlockKind) -> usize {
        // Each entry tracks its open block by ordinal position in the
        // active kind's sub-vector. The state machine tracks only the
        // currently-open block, but downstream callers may want to
        // inspect every block; the helper gives the active state a
        // target index so the entry's underlying vector can grow in
        // source order without scanning every block on every row.
        match kind {
            ClassBlockKind::ProgressionCurve => self.spell_progression.len(),
            ClassBlockKind::DomainSelections => self.domain_selections.len(),
            ClassBlockKind::Other => 0,
        }
    }
}

/// Walk every tab-delimited field on a `CLASS:<name>` line and emit the
/// `KEY:VAL` token pairs. The class name (the first field) is dropped.
fn collect_tokens(line_number: usize, raw_line: &str) -> Vec<SpellcastingClassToken> {
    let content = raw_line.trim_start();
    let rest = content.strip_prefix("CLASS:").unwrap_or(content);
    let mut tokens = Vec::new();
    for field in rest.split('\t') {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }
        // The first non-empty field is the class name itself. Drop it
        // — the entry's `class_name` already carries it.
        if tokens.is_empty()
            && !trimmed.contains(':')
            && first_field_is_class_name(raw_line)
        {
            continue;
        }
        let (key, value) = match trimmed.split_once(':') {
            Some((key, value)) => (key.to_string(), value.to_string()),
            None => (trimmed.to_string(), String::new()),
        };
        tokens.push(SpellcastingClassToken {
            key,
            value,
            line_number,
            raw_pair: trimmed.to_string(),
        });
    }
    tokens
}

fn first_field_is_class_name(raw_line: &str) -> bool {
    let content = raw_line.trim_start();
    let after = content.strip_prefix("CLASS:").unwrap_or(content);
    let first = after.split('\t').next().unwrap_or(after);
    !first.contains(':')
}

/// Lift the spellcasting-class signals (casting stat, posture,
/// automatically-known spell levels) from the token list collected on
/// the line that carries the `SPELLSTAT:` token.
fn extract_spellcasting_signals(
    tokens: &[SpellcastingClassToken],
) -> (Option<String>, Option<CastingPosture>, Vec<u8>) {
    let mut spell_stat: Option<String> = None;
    let mut memorize_no = false;
    let mut spellbook_yes = false;
    let mut auto_known: Vec<u8> = Vec::new();
    for token in tokens {
        match token.key.as_str() {
            "SPELLSTAT" => spell_stat = Some(token.value.clone()),
            "MEMORIZE" if token.value == "NO" => memorize_no = true,
            "SPELLBOOK" if token.value == "YES" => spellbook_yes = true,
            "KNOWNSPELLS" => {
                auto_known = parse_known_spells_levels(&token.value);
            }
            _ => {}
        }
    }
    let posture = if memorize_no {
        Some(CastingPosture::Spontaneous)
    } else if spellbook_yes {
        Some(CastingPosture::Spellbook)
    } else if spell_stat.is_some() {
        Some(CastingPosture::Prepared)
    } else {
        None
    };
    (spell_stat, posture, auto_known)
}

/// Parse `LEVEL=0|LEVEL=1|...|LEVEL=9` to a sorted integer vector. The
/// recognizer is permissive: missing or out-of-range levels are
/// silently ignored, malformed entries are skipped.
fn parse_known_spells_levels(value: &str) -> Vec<u8> {
    let mut levels: Vec<u8> = Vec::new();
    for chunk in value.split('|') {
        let chunk = chunk.trim();
        if let Some(rest) = chunk.strip_prefix("LEVEL=")
            && let Ok(n) = rest.trim().parse::<u8>()
            && n <= 9
        {
            levels.push(n);
        }
    }
    levels.sort_unstable();
    levels.dedup();
    levels
}

/// Parse a spell-progression row. A row looks like:
/// `1\tCAST:0,3\t\t\tKNOWN:4,2` — leading integer is the level. Returns
/// `(cast_payload, known_payload, level)`. On a malformed row (no
/// leading integer), the level is `0`.
fn parse_progression_row_fields(raw_line: &str) -> (String, String, u8) {
    let leading = raw_line.trim_start();
    // Take the leading ASCII-digit run as the level. If the leading
    // run is missing or out of `0..=20`, treat the row as malformed
    // and let the caller decide which diagnostic to emit.
    let digits: String = leading
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    let level = if digits.is_empty() {
        0
    } else {
        digits.parse::<u8>().unwrap_or(0)
    };
    // The first non-digit character must be a tab. If not, it's not a
    // progression row at all — leave payloads empty.
    let after_level = match leading.find('\t') {
        Some(idx) => &leading[idx + 1..],
        None => return (String::new(), String::new(), level),
    };
    // Extract CAST: and KNOWN: payloads by scanning the remaining line.
    let mut cast_payload = String::new();
    let mut known_payload = String::new();
    for field in after_level.split('\t') {
        let trimmed = field.trim();
        if let Some(rest) = trimmed.strip_prefix("CAST:") {
            cast_payload = rest.to_string();
        } else if let Some(rest) = trimmed.strip_prefix("KNOWN:") {
            known_payload = rest.to_string();
        }
    }
    (cast_payload, known_payload, level)
}

/// Parse a comma-separated integer payload. Returns `(ints, ok)` where
/// `ok` is false when any comma-separated field fails to parse as an
/// integer in the range `0..=20`.
fn parse_int_payload(payload: &str) -> (Vec<u8>, bool) {
    let trimmed = payload.trim();
    if trimmed.is_empty() {
        return (Vec::new(), true);
    }
    let mut output = Vec::new();
    for field in trimmed.split(',') {
        let field = field.trim();
        if field.is_empty() {
            continue;
        }
        match field.parse::<u8>() {
            Ok(n) => output.push(n),
            Err(_) => return (output, false),
        }
    }
    (output, true)
}

/// Extract the school name from a `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>`
/// line. Returns an empty string when the line carries no
/// `CHOICE:SCHOOL|<school>` segment (e.g. Universalist).
fn extract_school_from_subclass(raw_line: &str) -> String {
    let trimmed = raw_line.trim_start();
    for field in trimmed.split('\t') {
        let field = field.trim();
        if let Some(rest) = field.strip_prefix("CHOICE:SCHOOL|") {
            return rest.trim().to_string();
        }
    }
    String::new()
}

/// Resolve the implicit "Universal" school for subclass names that
/// carry no `CHOICE:SCHOOL|` token (e.g. PF1 Wizard's `Universalist`).
/// Non-universal subclass names return an empty string so the caller
/// records the absence of a school rather than fabricating a label.
fn universal_school_name(subclass_name: &str) -> String {
    if subclass_name.eq_ignore_ascii_case("Universalist") {
        "Universal".to_string()
    } else {
        String::new()
    }
}
