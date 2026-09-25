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
//! - `IrPackageContent<'a>` — corpus-rooted aggregate.
//! - `IrContentRecord<'a>`   — per-record envelope.
//! - `IrContentPayload<'a>`  — kind-tagged enum of borrowed
//!   B-family entries (defined in
//!   `src/pcgen_import/ir_content_payload.rs` to keep the
//!   `rules_core <-> pcgen_import` import graph acyclic; re-exported
//!   from `rules_core::source_content`).
//! - `SourceRef` / `SourceContentKind` / `SourceContentDiagnostic` —
//!   provenance + diagnostic surface.
//!
//! **This module (`ir_converter.rs`) is the canonical projection
//! path.** It takes a `ParsedLstRecord<'a>` and produces a
//! `IrContentRecord<'a>` per record, with full provenance
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
//!   [`IrContentRecord`] (the canonical envelope).
//! - Per-family converters ([`convert_class_entry`], etc.) for typed callers.
//! - Per-document converters ([`convert_class_parse_result`], etc.) that
//!   consume the B-family parse-result containers and emit a
//!   [`IrPackageContent`] aggregate plus forwarded canonical diagnostics.
//! - [`convert_package_from_class_parse_result`] etc. — corpus-rooted
//!   entry points that accumulate a complete [`IrPackageContent`].
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
    EquipmentDiagnostic, EquipmentParseResult, EquipmentRecord, EquipmentRecordKind,
};
use crate::pcgen_import::lst_parser::metadata::{LstMetadataDocument, LstRecord};
use crate::pcgen_import::lst_parser::race_ability::{
    AbilityDeclaration, LstDiagnostic as RaceAbilityLstDiagnostic, LstEntryFile, RaceDeclaration,
};
use crate::pcgen_import::lst_parser::spell::{LstSpellFile, LstSpellRecord};
use crate::pcgen_import::lst_parser::spellcasting_class::{
    SpellcastingClassDiagnostic, SpellcastingClassEntry, SpellcastingClassParseResult,
};
use crate::pcgen_import::ir_content_payload::b6_metadata_kind_to_canonical;
use codex::rules_core::damage_total::{DiceExpression, WieldCategory};
use codex::rules_core::equipment_effects::intelligent_item::{
    IntelligentItemContribution, ItemAlignment,
};
use codex::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use codex::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};
use codex::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use codex::rules_core::equipment_effects::EquipmentStatEffect;
use codex::rules_core::equipment_record::CorpusEquipmentRecord;
use codex::rules_core::spell_record::CorpusSpellRecord;
use crate::pcgen_import::ir_content_payload::{
    record_to_live, IrContentPayload, IrContentRecord, IrPackageContent,
};
use codex::rules_core::source_content::{
    SOURCE_IR_VERSION, SourceContentDiagnostic, SourceContentDiagnosticKind, SourceContentKind,
    SourceContentRecord, SourceContentSeverity, SourceRef,
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
    /// Provenance: `source_ref.source_path <- source_path`,
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
// Per-family record converters — return IrContentRecord<'a>
// =============================================================================

/// Build a canonical [`IrContentRecord`] from a B-1 [`ClassEntry`].
///
/// Projection is zero-copy. The borrowed `entry` carries every
/// `tokens` and `feature_blocks` entry verbatim.
pub fn convert_class_entry(entry: &ClassEntry) -> IrContentRecord<'_> {
    let line = entry.header_line_number;
    let source_ref = make_source_ref(entry.record_source_path(), line);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Class,
        IrContentPayload::Class(entry),
    )
}

/// Build a canonical [`IrContentRecord`] from a B-2
/// [`SpellcastingClassEntry`].
pub fn convert_spellcasting_class_entry(entry: &SpellcastingClassEntry) -> IrContentRecord<'_> {
    let line = entry.header_line_number;
    let source_ref = make_source_ref(entry.record_source_path(), line);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::SpellcastingClass,
        IrContentPayload::SpellcastingClass(entry),
    )
}

/// Build a canonical [`IrContentRecord`] from a B-3
/// [`RaceDeclaration`].
pub fn convert_race_declaration(decl: &RaceDeclaration) -> IrContentRecord<'_> {
    let source_ref = make_source_ref(&decl.source_path, decl.line_number);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Race,
        IrContentPayload::Race(decl),
    )
}

/// Build a canonical [`IrContentRecord`] from a B-3
/// [`AbilityDeclaration`].
pub fn convert_ability_declaration(decl: &AbilityDeclaration) -> IrContentRecord<'_> {
    let source_ref = make_source_ref(&decl.source_path, decl.line_number);
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Ability,
        IrContentPayload::Ability(decl),
    )
}

/// Convert a B-4 [`LstSpellRecord`] -- the ingest-format parser row -- into
/// the live side's own converted shape,
/// [`CorpusSpellRecord`](codex::rules_core::spell_record::CorpusSpellRecord).
///
/// SD-35 `AT-35-E6-003-RULED` cycle 8. This function is the **only** reader
/// of `LstSpellRecord` on the spell path: everything downstream of the
/// canonical envelope sees `CorpusSpellRecord` and never names
/// `pcgen_import` (`decisions.md` §11, §19). The mapping is total and
/// field-for-field -- no field is dropped, none is invented, and nothing
/// about the ingest format's own vocabulary survives it, because the parser
/// already stripped the `SCHOOL:`/`CASTTIME:`/... column tags.
pub fn spell_record_to_corpus(record: &LstSpellRecord) -> CorpusSpellRecord {
    CorpusSpellRecord {
        line_number: record.line_number,
        source_path: record.source_path.clone(),
        name: record.name.clone(),
        output_name: record.output_name.clone(),
        spell_type: record.spell_type.clone(),
        classes: record.classes.clone(),
        school: record.school.clone(),
        descriptor: record.descriptor.clone(),
        sub_school: record.sub_school.clone(),
        components: record.components.clone(),
        casting_time: record.casting_time.clone(),
        range: record.range.clone(),
        item: record.item.clone(),
        target_area: record.target_area.clone(),
        duration: record.duration.clone(),
        save_info: record.save_info.clone(),
        spell_resistance: record.spell_resistance.clone(),
        source_page: record.source_page.clone(),
        source_link: record.source_link.clone(),
        description: record.description.clone(),
        description_raw: record.description_raw.clone(),
    }
}

/// Build a canonical [`IrContentRecord`] from a B-4 [`LstSpellRecord`].
///
/// The envelope's payload is the **converted** record
/// ([`CorpusSpellRecord`](codex::rules_core::spell_record::CorpusSpellRecord)),
/// not a borrow of the parser row, so this is the one place on the spell
/// path where the projection stops being zero-copy. The converted record is
/// interned for the process lifetime (`Box::leak`) to satisfy the envelope's
/// borrow -- the same thing every caller of this function already did with
/// the parser row itself, one allocation earlier. A live caller that holds
/// already-converted corpus data does not come through here at all: it
/// builds the envelope directly with
/// [`codex::rules_core::source_content::SourceContentRecord::spell`].
pub fn convert_spell_record(record: &LstSpellRecord) -> SourceContentRecord<'static> {
    record_to_live(&convert_spell_record_ir(record))
}

/// The same conversion as [`convert_spell_record`], stopping at the
/// converter's own envelope instead of the live one.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 18: the per-document converters accumulate
/// an [`IrPackageContent`], so they need the IR-side record; every other caller
/// wants the live envelope, whose `Spell` payload is the same settled record.
pub fn convert_spell_record_ir(record: &LstSpellRecord) -> IrContentRecord<'static> {
    let source_ref = make_source_ref(&record.source_path, record.line_number);
    let converted: &'static CorpusSpellRecord =
        Box::leak(Box::new(spell_record_to_corpus(record)));
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Spell,
        IrContentPayload::Spell(converted),
    )
}

/// Convert a B-5 [`EquipmentRecord`] into the live side's own settled
/// equipment record.
///
/// SD-35 `AT-35-E6-003-RULED` cycle 10. Every read below used to happen on the
/// live side, once per query, against the ingest format's own token and
/// `BONUS:` chain arrays -- `encumbrance::weight_and_cost_from_record`,
/// `equipment_effects::magic_items::compute_magic_items_effect` and its
/// `TEMPBONUS:` fallback, and
/// `equipment_effects::intelligent_item::compute_intelligent_item_effect`.
/// They are the same reads, moved to the side of the boundary that owns the
/// ingest vocabulary (`decisions.md` §11): the live consumers now read a
/// settled value off [`CorpusEquipmentRecord`].
///
/// Behaviour is preserved field for field, including each read's own honest
/// absence: a record that states no weight, no ability-score chain, or none
/// of the intelligent-item family yields `None` for that field rather than a
/// zero.
pub fn equipment_record_to_corpus(record: &EquipmentRecord) -> CorpusEquipmentRecord {
    let token_value = |key: &str| {
        record.tokens.iter().find(|token| token.key == key).map(|token| token.value.as_str())
    };
    let identity = token_value("KEY").unwrap_or(record.name.as_str()).to_string();
    let weight_lbs = token_value("WT").and_then(|value| value.parse::<f64>().ok());
    let cost_gp = token_value("COST").and_then(|value| value.parse::<f64>().ok());
    CorpusEquipmentRecord {
        identity,
        name: record.name.clone(),
        weight_lbs,
        cost_gp,
        ability_score_bonus: ability_score_bonus_of(record),
        intelligent_item: intelligent_item_contribution_of(record),
        stat_effect: arms_armor_stat_effect_of(record),
        armor_class_chain_bonus: armor_class_chain_bonus_of(record),
        skill_check_bonus: skill_check_bonus_of(record),
        var_bonuses: var_bonuses_of(record),
        weapon_enhancement: weapon_enhancement_of(record),
        spell_resistance_bonus: spell_resistance_bonus_of(record),
        eqmod_references: eqmod_references_of(record),
        base_damage_dice: base_damage_dice_of(record),
        states_base_damage: states_base_damage_of(record),
        base_item: base_item_of(record),
        wield_category: wield_category_of(record),
        critical_threat_range: critical_threat_range_of(record),
        critical_multiplier: critical_multiplier_of(record),
        damage_size_steps: damage_size_steps_of(record),
        weight_divisor: weight_divisor_of(record),
        is_natural_attack: is_natural_attack_of(record),
        is_shield: is_shield_of(record),
        is_modifier: matches!(record.kind, EquipmentRecordKind::EquipMod),
    }
}

/// The item's settled base damage die. Moved here verbatim from
/// `damage_total::damage_dice_token` (SD-35 `AT-35-E6-003-RULED` cycle 12);
/// the parse rule -- PF1's canonical `<count>d<size>` only, with the
/// degenerate `0d<n>` / `<n>d0` cases refused rather than defaulted -- is
/// `DiceExpression::parse`'s own and stayed with it on the live side.
fn base_damage_dice_of(record: &EquipmentRecord) -> Option<DiceExpression> {
    equipment_token_value(record, "DAMAGE").and_then(DiceExpression::parse)
}

/// Whether the record states a base damage value at all. Separate from
/// [`base_damage_dice_of`] because `equipment_effects::is_weapon_record` has
/// always tested PRESENCE, not parseability: an item stating damage this
/// engine does not spell as dice is still a wielded weapon.
fn states_base_damage_of(record: &EquipmentRecord) -> bool {
    equipment_token_value(record, "DAMAGE").is_some()
}

/// The identity of the item whose own stats stand in for this one's. Moved
/// here from `damage_total::base_item_damage_dice_token`, which held the token
/// spelling only to learn a name; the one-hop chase itself is a corpus
/// resolution and stayed live, because the converter has no corpus.
fn base_item_of(record: &EquipmentRecord) -> Option<String> {
    equipment_token_value(record, "BASEITEM").map(str::to_string)
}

/// The item's settled wield category. Moved here verbatim from
/// `damage_total::wield_category_token`: a value outside PF1's three
/// categories yields `None` rather than a guessed default.
fn wield_category_of(record: &EquipmentRecord) -> Option<WieldCategory> {
    match equipment_token_value(record, "WIELD")? {
        "Light" => Some(WieldCategory::Light),
        "OneHanded" => Some(WieldCategory::OneHanded),
        "TwoHanded" => Some(WieldCategory::TwoHanded),
        _ => None,
    }
}

/// The item's settled critical threat range as inclusive natural-roll bounds.
/// Moved here verbatim from `damage_total::critical_threat_range_token`: the
/// corpus states a threat *width* (the count of consecutive top natural rolls
/// that threaten) and the sheet prints bounds, so a width of `2` settles to
/// `(19, 20)`. A width outside `1..=20` is refused.
fn critical_threat_range_of(record: &EquipmentRecord) -> Option<(u8, u8)> {
    equipment_token_value(record, "CRITRANGE")
        .and_then(|value| value.parse::<u8>().ok())
        .filter(|width| (1..=20).contains(width))
        .map(|width| (20 - width + 1, 20))
}

/// The item's settled critical-hit damage multiplier. Moved here verbatim from
/// `damage_total::critical_multiplier_token`, including the `x` prefix the
/// corpus states it with and the refusal of any multiplier below `2`.
fn critical_multiplier_of(record: &EquipmentRecord) -> Option<u8> {
    equipment_token_value(record, "CRITMULT")
        .and_then(|value| value.strip_prefix('x'))
        .and_then(|digits| digits.parse::<u8>().ok())
        .filter(|multiplier| *multiplier >= 2)
}

/// How many steps this item, as a referenced modifier, moves its host weapon's
/// single-die damage progression. Moved here verbatim from
/// `damage_total::eqmweapon_damagesize_chain_value`, including its summation
/// across every such chain the record carries.
fn damage_size_steps_of(record: &EquipmentRecord) -> i32 {
    record
        .bonus_chains
        .iter()
        .filter_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            if qualifiers.len() >= 3 && qualifiers[0] == "EQMWEAPON" && qualifiers[1] == "DAMAGESIZE"
            {
                qualifiers[2].parse::<i32>().ok()
            } else {
                None
            }
        })
        .sum()
}

/// The divisor this item, as a referenced modifier, applies to its host item's
/// weight. Moved here verbatim from the inner scan of
/// `equipment_effects::resolve_eqm_weightdiv_effect`, including its `find_map`
/// shape: a record carrying more than one such chain contributes the first,
/// exactly as before.
fn weight_divisor_of(record: &EquipmentRecord) -> Option<f32> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() >= 3 && qualifiers[0] == "EQM" && qualifiers[1] == "WEIGHTDIV" {
            qualifiers[2].parse::<f32>().ok()
        } else {
            None
        }
    })
}

/// Whether the item is one of PF1's natural attacks. Moved here verbatim from
/// `equipment_effects::is_natural_attack_weapon`, including the exact-segment
/// match that keeps `Weapon Group Natural` from firing it.
fn is_natural_attack_of(record: &EquipmentRecord) -> bool {
    equipment_token_value(record, "TYPE")
        .is_some_and(|value| value.split('.').any(|segment| segment == "Natural"))
}

/// Whether the item is a shield. Moved here verbatim from the shield half of
/// `equipment_effects::is_weapon_record`, including its first-segment rule.
fn is_shield_of(record: &EquipmentRecord) -> bool {
    equipment_token_value(record, "TYPE")
        .is_some_and(|value| value.split('.').next() == Some("Shield"))
}

/// The item's settled armour/shield stat contribution. Moved here verbatim
/// from `equipment_effects::arms_armor` (SD-35 `AT-35-E6-003-RULED` cycle 11);
/// every rule each field encodes, and every real-corpus witness behind it, is
/// stated in that module's own doc comments, which stayed with the numbers.
fn arms_armor_stat_effect_of(record: &EquipmentRecord) -> EquipmentStatEffect {
    EquipmentStatEffect {
        armor_class_bonus: armor_class_chain_bonus_of(record)
            .or_else(|| tempbonus_combat_ac_of(record)),
        max_dex: equipment_token_i16(record, "MAXDEX")
            .or_else(|| eqmarmor_chain_value_of(record, "MAXDEX")),
        spell_failure: equipment_token_value(record, "SPELLFAILURE")
            .and_then(|value| value.parse().ok())
            .or_else(|| eqmarmor_chain_value_of(record, "SPELLFAILURE").map(f32::from)),
        armor_check_penalty: equipment_token_i16(record, "ACCHECK")
            .or_else(|| eqmarmor_chain_value_of(record, "ACCHECK")),
    }
}

fn equipment_token_value<'a>(record: &'a EquipmentRecord, key: &str) -> Option<&'a str> {
    record.tokens.iter().find(|token| token.key == key).map(|token| token.value.as_str())
}

fn equipment_token_i16(record: &EquipmentRecord, key: &str) -> Option<i16> {
    equipment_token_value(record, key).and_then(|value| value.parse().ok())
}

/// The first standing `BONUS:COMBAT|AC|<n>` chain's magnitude. A
/// `TYPE=Circumstance` chain is excluded: by PF1's own definition it applies
/// only while its holder is in a named situation, so it is not a standing
/// armour contribution.
fn armor_class_chain_bonus_of(record: &EquipmentRecord) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        let is_ac_bonus = qualifiers.len() >= 3
            && qualifiers[0] == "COMBAT"
            && qualifiers[1] == "AC"
            && !crate::pcgen_import::equipment_bonus_reader::declares_circumstance_bonus_type(bonus);
        if is_ac_bonus {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

/// The consumable-triggered sibling of the standing AC chain, read only for
/// the item's own total and never for a referenced modifier's contribution.
fn tempbonus_combat_ac_of(record: &EquipmentRecord) -> Option<i16> {
    record.tokens.iter().find_map(|token| {
        if token.key != "TEMPBONUS" {
            return None;
        }
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 4
            || (parts[0] != "PC" && parts[0] != "ANYPC")
            || parts[1] != "COMBAT"
            || parts[2] != "AC"
        {
            return None;
        }
        parts[3].parse::<i16>().ok()
    })
}

/// A modifier record's own `BONUS:EQMARMOR|<field>|<n>` magnitude -- the
/// family a material/masterwork/enhancement modifier states its armour-stat
/// contribution in, consulted only when the bare token is absent.
fn eqmarmor_chain_value_of(record: &EquipmentRecord, field: &str) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() >= 3 && qualifiers[0] == "EQMARMOR" && qualifiers[1] == field {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

/// The item's settled circumstance bonus to one named skill, including the
/// automatic swim-speed racial bonus PF1 grants on top of an explicit Swim
/// bonus. Moved here verbatim from `equipment_effects::general`.
fn skill_check_bonus_of(record: &EquipmentRecord) -> Option<SkillCheckBonus> {
    let explicit = record
        .bonus_chains
        .iter()
        .find_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            if qualifiers.len() < 3 || qualifiers[0] != "SKILL" {
                return None;
            }
            qualifiers[2].parse::<i16>().ok().map(|bonus_value| SkillCheckBonus {
                skill: qualifiers[1].clone(),
                bonus: bonus_value,
            })
        })
        .or_else(|| tempbonus_skill_of(record))?;
    Some(SkillCheckBonus {
        bonus: explicit.bonus + swim_speed_racial_bonus_of(record, &explicit.skill),
        ..explicit
    })
}

/// The consumable-triggered single-skill sibling of the explicit skill chain.
/// A comma-joined list, a `TYPE.<Group>` wildcard and the literal `ALL`
/// wildcard are all deliberately unread: each is a wider shape this settled
/// single-skill value has no way to state, so the honest answer is absence.
fn tempbonus_skill_of(record: &EquipmentRecord) -> Option<SkillCheckBonus> {
    record.tokens.iter().find_map(|token| {
        if token.key != "TEMPBONUS" {
            return None;
        }
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 4 || (parts[0] != "PC" && parts[0] != "ANYPC") || parts[1] != "SKILL" {
            return None;
        }
        let skill = parts[2];
        if skill.is_empty()
            || skill.contains(',')
            || skill.starts_with("TYPE.")
            || skill.eq_ignore_ascii_case("ALL")
        {
            return None;
        }
        parts[3].parse::<i16>().ok().map(|bonus_value| SkillCheckBonus {
            skill: skill.to_string(),
            bonus: bonus_value,
        })
    })
}

/// PF1's Swim skill rule: a swim speed of at least 5 feet is a +8 racial
/// bonus on Swim checks, additive with any explicit Swim bonus the same item
/// grants.
fn swim_speed_racial_bonus_of(record: &EquipmentRecord, skill: &str) -> i16 {
    if skill != "Swim" {
        return 0;
    }
    let grants_swim_speed = record.tokens.iter().any(|token| {
        token.key == "MOVE"
            && token.value.split(',').any(|part| part.trim().eq_ignore_ascii_case("Swim"))
    });
    if grants_swim_speed {
        8
    } else {
        0
    }
}

/// The item's settled flat bonuses to named rules variables, one row per
/// name. A chain naming several variables at once contributes the same
/// magnitude to each. Moved here verbatim from `equipment_effects::general`.
fn var_bonuses_of(record: &EquipmentRecord) -> Vec<VarBonus> {
    record
        .bonus_chains
        .iter()
        .filter_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            if qualifiers.len() < 3 || qualifiers[0] != "VAR" {
                return None;
            }
            let value = qualifiers[2].parse::<i16>().ok()?;
            Some((qualifiers[1].as_str(), value))
        })
        .flat_map(|(names, value)| {
            names.split(',').map(move |name| VarBonus { name: name.to_string(), bonus: value })
        })
        .collect()
}

/// One record's own named-variable magnitude, used only to substitute a
/// sibling roll chain's non-literal magnitude segment. Never looks outside
/// this one record.
fn var_reference_of(record: &EquipmentRecord, name: &str) -> Option<i16> {
    record.bonus_chains.iter().find_map(|bonus| {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() >= 3 && qualifiers[0] == "VAR" && qualifiers[1] == name {
            qualifiers[2].parse::<i16>().ok()
        } else {
            None
        }
    })
}

/// The item's settled weapon to-hit / damage enhancement, summed across every
/// qualifying roll chain on the record. Moved here verbatim from
/// `equipment_effects::equipmods`, whose own doc comments carry every rule and
/// real-corpus witness behind each arm below.
fn weapon_enhancement_of(record: &EquipmentRecord) -> Option<WeaponEnhancementBonus> {
    let mut tohit_bonus: Option<i16> = None;
    let mut damage_bonus: Option<i16> = None;
    let mut natural_attack_only = false;
    let mut weapon_prof_scope: Option<String> = None;
    let mut matched = false;

    let mut apply = |affects: &str, bonus_value: i16| {
        if affects.contains("TOHIT") {
            tohit_bonus = Some(tohit_bonus.unwrap_or(0) + bonus_value);
        }
        if affects.contains("DAMAGE") {
            damage_bonus = Some(damage_bonus.unwrap_or(0) + bonus_value);
        }
    };

    for bonus in &record.bonus_chains {
        let qualifiers = &bonus.qualifiers;
        let subject = qualifiers.first().map(String::as_str);
        let this_natural_attack_only = subject == Some("WEAPONPROF=TYPE.Natural");
        let is_roll_shape = qualifiers.len() >= 2
            && matches!(
                qualifiers[1].as_str(),
                "TOHIT" | "DAMAGE" | "DAMAGE,TOHIT" | "TOHIT,DAMAGE"
            );

        if (subject == Some("WEAPON") || this_natural_attack_only) && is_roll_shape {
            if crate::pcgen_import::equipment_bonus_reader::roll_bonus_carries_enhancement_type(
                bonus,
            ) {
                let magnitude = qualifiers[2]
                    .parse::<i16>()
                    .ok()
                    .or_else(|| var_reference_of(record, &qualifiers[2]));
                if let Some(bonus_value) = magnitude {
                    matched = true;
                    natural_attack_only = this_natural_attack_only;
                    apply(&qualifiers[1], bonus_value);
                }
            }
            continue;
        }

        if let Some(name) = subject.and_then(|s| s.strip_prefix("WEAPONPROF="))
            && !name.starts_with("TYPE.")
            && is_roll_shape
            && qualifiers.len() >= 3
            && let Ok(bonus_value) = qualifiers[2].parse::<i16>()
        {
            matched = true;
            weapon_prof_scope = Some(name.to_string());
            apply(&qualifiers[1], bonus_value);
        }
    }

    matched.then_some(WeaponEnhancementBonus {
        tohit_bonus,
        damage_bonus,
        natural_attack_only,
        weapon_prof_scope,
    })
}

/// The item's settled flat Spell Resistance grant. A record whose grant is a
/// player choice rather than a literal states no settled number and yields
/// `None`. Moved here verbatim from `equipment_effects::equipmods`.
fn spell_resistance_bonus_of(record: &EquipmentRecord) -> Option<i16> {
    equipment_token_value(record, "SR").and_then(|value| value.parse().ok())
}

/// The corpus identities of the modifier items attached to this one.
///
/// A record can name more than one attachment, and each names its parts in
/// one string; the live side used to hold both of those grammar facts to ask
/// one question ("which other corpus records are attached to this one?").
/// Every candidate segment is emitted in source order, including the ones
/// that resolve to no record -- the caller's resolve-or-skip pass is
/// unchanged, and filtering here would need the corpus the converter does not
/// have.
fn eqmod_references_of(record: &EquipmentRecord) -> Vec<String> {
    let mut references = Vec::new();
    for token in record.tokens.iter().filter(|token| token.key == "EQMOD") {
        for instance in token.value.split('.') {
            for candidate in instance.split('|') {
                let candidate = candidate.trim();
                if !candidate.is_empty() {
                    references.push(candidate.to_string());
                }
            }
        }
    }
    references
}

/// The item's settled ability-score enhancement: its first
/// `BONUS:STAT|<ability>|<n>` chain, else the `TEMPBONUS:<PC|ANYPC>|STAT|...`
/// form the CRB ability-score potions state theirs in (they carry no `BONUS:`
/// chain at all). Moved here from `equipment_effects::magic_items`.
fn ability_score_bonus_of(record: &EquipmentRecord) -> Option<AbilityScoreBonus> {
    record
        .bonus_chains
        .iter()
        .find_map(|bonus| {
            let qualifiers = &bonus.qualifiers;
            if qualifiers.len() < 3 || qualifiers[0] != "STAT" {
                return None;
            }
            qualifiers[2].parse::<i16>().ok().map(|bonus_value| AbilityScoreBonus {
                ability: qualifiers[1].clone(),
                bonus: bonus_value,
            })
        })
        .or_else(|| {
            record.tokens.iter().find_map(|token| {
                if token.key != "TEMPBONUS" {
                    return None;
                }
                let parts: Vec<&str> = token.value.split('|').collect();
                if parts.len() < 4 || (parts[0] != "PC" && parts[0] != "ANYPC") || parts[1] != "STAT"
                {
                    return None;
                }
                let ability = parts[2];
                if ability.is_empty() || ability.contains(',') {
                    return None;
                }
                parts[3].parse::<i16>().ok().map(|bonus_value| AbilityScoreBonus {
                    ability: ability.to_string(),
                    bonus: bonus_value,
                })
            })
        })
}

/// The item's settled contribution to an intelligent item's stat block: every
/// unconditional three-part `BONUS:VAR|<name>|<value>` chain of the
/// intelligent-item family. A chain carrying a trailing condition is not
/// unconditionally true and is excluded rather than asserted. Moved here from
/// `equipment_effects::intelligent_item`.
fn intelligent_item_contribution_of(record: &EquipmentRecord) -> Option<IntelligentItemContribution> {
    let mut result = IntelligentItemContribution::default();
    let mut found = false;
    for bonus in &record.bonus_chains {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() != 3 || qualifiers[0] != "VAR" {
            continue;
        }
        let Ok(value) = qualifiers[2].parse::<i16>() else {
            continue;
        };
        match qualifiers[1].as_str() {
            "IntItemStatINT" => {
                result.intelligence_bonus += value;
                found = true;
            }
            "IntItemStatWIS" => {
                result.wisdom_bonus += value;
                found = true;
            }
            "IntItemStatCHA" => {
                result.charisma_bonus += value;
                found = true;
            }
            "IntelligentItemEgo" => {
                result.ego_bonus += value;
                found = true;
            }
            "IntItemAlignment" => {
                if let Some(alignment) = ItemAlignment::from_code(value) {
                    result.alignment = Some(alignment);
                    found = true;
                }
            }
            _ => {}
        }
    }
    found.then_some(result)
}

/// Build a canonical [`IrContentRecord`] from a B-5
/// [`EquipmentRecord`].
///
/// SD-35 `AT-35-E6-003-RULED` cycle 13: the envelope's payload is the converted
/// [`CorpusEquipmentRecord`] **alone**. Cycle 10 put it there beside the parser
/// row and wrote down that the pair was temporary -- "it goes when the last
/// consumer reads a settled value instead". Cycle 12 moved the last two value
/// readers and cycle 13 moved the resolver and the live loader, so the row is
/// gone from the envelope. It is still this function's input, because the
/// conversion is what this function is.
///
/// The converted record is interned for the process lifetime (`Box::leak`) to
/// satisfy the envelope's borrow, exactly as cycle 8's spell path already does.
pub fn convert_equipment_record(record: &EquipmentRecord) -> SourceContentRecord<'static> {
    record_to_live(&convert_equipment_record_ir(record))
}

/// The same conversion as [`convert_equipment_record`], stopping at the
/// converter's own envelope instead of the live one. See
/// [`convert_spell_record_ir`] for why both exist.
pub fn convert_equipment_record_ir(record: &EquipmentRecord) -> IrContentRecord<'static> {
    let line = record.header_line_number;
    let source_ref = make_source_ref(record.record_source_path(), line);
    let converted: &'static CorpusEquipmentRecord =
        Box::leak(Box::new(equipment_record_to_corpus(record)));
    SourceContentRecord::new(
        source_ref,
        SourceContentKind::Equipment,
        IrContentPayload::Equipment(converted),
    )
}

/// Build a canonical [`IrContentRecord`] from a B-6 [`LstRecord`].
/// The inner metadata kind is mapped from the parser-side
/// [`crate::pcgen_import::lst_parser::metadata::MetadataKind`] to the
/// canonical [`codex::rules_core::source_content::MetadataKindInner`] so
/// the rules-core envelope is total over the six B-6 kinds.
pub fn convert_metadata_record(record: &LstRecord) -> IrContentRecord<'_> {
    let source_ref = make_source_ref(record.record_source_path(), record.line_number);
    let kind = SourceContentKind::Metadata(b6_metadata_kind_to_canonical(record.kind));
    SourceContentRecord::new(source_ref, kind, IrContentPayload::Metadata(record))
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
/// `IrContentRecord<'a>` per the Slice E contract. The conversion
/// is total: every B-family record has exactly one canonical envelope
/// variant. The `Result` wrapper is no longer necessary; the
/// canonical envelope carries its own diagnostic stream via
/// [`SourceContentDiagnostic`] attached to the package-level
/// [`IrPackageContent`].
pub fn convert_to_ir<'a>(
    parsed_record: &ParsedLstRecord<'a>,
    _schema: &IRSchema,
) -> IrContentRecord<'a> {
    match parsed_record {
        ParsedLstRecord::Class(entry) => convert_class_entry(entry),
        ParsedLstRecord::SpellcastingClass(entry) => convert_spellcasting_class_entry(entry),
        ParsedLstRecord::Race(r) => convert_race_declaration(r),
        ParsedLstRecord::Ability(a) => convert_ability_declaration(a),
        ParsedLstRecord::Spell(s) => convert_spell_record_ir(s),
        ParsedLstRecord::Equipment(e) => convert_equipment_record_ir(e),
        ParsedLstRecord::Metadata(r) => convert_metadata_record(r),
    }
}

// =============================================================================
// Per-document converters — produce (IrContentRecord, forwarded IRDiagnostic) pairs
// =============================================================================
//
// The converter returns one `IrContentRecord` per B-family entry
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
    let _ = source_path; // Per-row spell diagnostics live on the LstSpellFile, not on the LstSpellRecord itself.
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        let canonical = convert_spell_record_ir(record);
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
    let source_path = path_to_string(&r.source_path);
    let container_diagnostics = forward_spell_container_diagnostics(&r.diagnostics, &source_path);
    let mut out = Vec::with_capacity(r.records.len());
    for record in &r.records {
        // Rebuild the canonical record with the document's source_path
        // override so the B-4 record's own source_path is honored when
        // present but the document's identity fills in for records that
        // were construct-ed with empty paths.
        let canonical = convert_spell_record_ir(record);
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
) -> Vec<(IrContentRecord<'a>, Vec<IRDiagnostic>)> {
    let mut out = Vec::with_capacity(r.entries.len());
    for entry in &r.entries {
        let canonical = convert_equipment_record_ir(entry);
        let forwarded = forward_equipment_diagnostics(entry, &r.diagnostics, &r.source_path);
        out.push((canonical, forwarded));
    }
    out
}

// =============================================================================
// Corpus-rooted IrPackageContent builders
// =============================================================================
//
// These produce the canonical [`IrPackageContent`] aggregate
// directly, accumulating records and the canonical diagnostics stream
// in one pass.

/// Build a canonical [`IrPackageContent`] from a B-1
/// [`ClassParseResult`].
pub fn convert_package_from_class_parse_result<'a>(
    r: &'a ClassParseResult,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (IrPackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = IrPackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
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

/// Build a canonical [`IrPackageContent`] from a B-2
/// [`SpellcastingClassParseResult`].
pub fn convert_package_from_spellcasting_class_parse_result<'a>(
    r: &'a SpellcastingClassParseResult,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (IrPackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = IrPackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
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

/// Build a canonical [`IrPackageContent`] from a B-3
/// [`LstEntryFile`].
pub fn convert_package_from_lst_entry_file<'a>(
    r: &'a LstEntryFile,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (IrPackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = IrPackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
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

/// Build a canonical [`IrPackageContent`] from a B-6
/// [`LstMetadataDocument`].
pub fn convert_package_from_lst_metadata_document<'a>(
    r: &'a LstMetadataDocument,
    package_id: impl Into<String>,
    schema: &IRSchema,
) -> (IrPackageContent<'a>, Vec<IRDiagnostic>) {
    let mut pkg = IrPackageContent::empty(package_id, SourceRef::new(r.source_path.clone(), 0));
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
            IrContentPayload::Class(e) => {
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
        assert_eq!(rec.source_ref.source_path, "spells.lst");
        assert_eq!(rec.source_ref.line, 11);
    }

    #[test]
    fn equipment_record_zero_copy() {
        let record = fake_equipment();
        let rec = convert_equipment_record(&record);
        assert_eq!(rec.kind, SourceContentKind::Equipment);
        match rec.payload {
            codex::rules_core::source_content::SourceContentPayload::Equipment(e) => {
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
        assert_eq!(canonical.source_ref.source_path, "x.lst");
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
