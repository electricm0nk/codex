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
use codex::rules_core::equipment_effects::skill_set_rows;
use codex::rules_core::equipment_record::{ContainerCapacity, CorpusEquipmentRecord, StatedBonus};
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
        armor_class_bonus_type: armor_class_bonus_type_of(record),
        stated_bonuses: stated_bonuses_of(record),
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
        container: container_of(record),
        container_capacity: if container_of(record) { container_capacity_of(record) } else { ContainerCapacity::Unstated },
        uses_ammunition: uses_ammunition_of(record),
        ammunition_family: if uses_ammunition_of(record) { ammunition_family_of(record) } else { None },
        ammunition_kinds: ammunition_kinds_of(record),
        base_quantity: base_quantity_of(record),
    }
}

/// The record's `TYPE` words.
fn type_words(record: &EquipmentRecord) -> Vec<&str> {
    equipment_token_value(record, "TYPE").map_or(Vec::new(), |value| value.split('.').collect())
}

/// Whether the item fires ammunition: `TYPE` holds `Projectile` (SD-41 `decisions.md §23`).
fn uses_ammunition_of(record: &EquipmentRecord) -> bool {
    type_words(record).contains(&"Projectile")
}

/// The ammunition family `CONTAINS` names after its capacity field, up to any `=count`, spaces
/// removed: Longbow `UNLIM|Arrow` is `Arrow`, Pistol `1|Firearm Bullet=1` is `FirearmBullet`. Only
/// the first family of a several-family weapon is kept (`FirearmBullet=1|FirearmPellet=1`).
fn ammunition_family_of(record: &EquipmentRecord) -> Option<String> {
    let family = equipment_token_value(record, "CONTAINS")?.split('|').nth(1)?.split('=').next()?.replace(' ', "");
    (!family.is_empty() && family != "Any").then_some(family)
}

/// An `Ammunition`-typed item's `TYPE` words, spaces removed; empty for anything else.
fn ammunition_kinds_of(record: &EquipmentRecord) -> Vec<String> {
    let words = type_words(record);
    if !words.contains(&"Ammunition") {
        return Vec::new();
    }
    words.iter().map(|word| word.replace(' ', "")).collect()
}

/// Pieces per copy as the book sells it: `BASEQTY`, when it is a positive whole number.
fn base_quantity_of(record: &EquipmentRecord) -> Option<u32> {
    equipment_token_value(record, "BASEQTY").and_then(|value| value.parse::<u32>().ok()).filter(|n| *n > 0)
}

/// Whether the item is a container (SD-41 `decisions.md §31` R2, "Container tag, but not
/// weapons"): `TYPE` holds `Container`, and the record is not a weapon by either signal the corpus
/// gives -- no `DAMAGE` and no `Weapon` type word. A weapon's `CONTAINS` names its loaded
/// ammunition (Pistol `1|FirearmBullet=1`, Wrist Launcher `UNLIM|Dart=1`), not a carried bag.
fn container_of(record: &EquipmentRecord) -> bool {
    let words: Vec<&str> = equipment_token_value(record, "TYPE").map_or(Vec::new(), |value| value.split('.').collect());
    words.contains(&"Container") && !words.contains(&"Weapon") && !states_base_damage_of(record)
}

/// One row of the cited container-capacity override list (SD-41 `decisions.md §32`).
#[derive(Debug, serde::Deserialize)]
struct CapacityOverride {
    key: String,
    capacity_lbs: f64,
    #[allow(dead_code)] // parsed so an uncited row fails; read by the tests and by people.
    source: CapacitySource,
}

/// Where an override's figure comes from; a row without one does not parse.
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)] // read by the tests and by people, not by the converter.
struct CapacitySource {
    kind: String,
    #[serde(rename = "ref")]
    reference: String,
    date: String,
}

/// `data/capacity_overrides/container-capacities.json`, compiled in so every conversion sees it.
fn capacity_overrides() -> &'static [CapacityOverride] {
    static ROWS: std::sync::OnceLock<Vec<CapacityOverride>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        serde_json::from_str(include_str!("../../../../data/capacity_overrides/container-capacities.json"))
            .expect("data/capacity_overrides/container-capacities.json parses")
    })
}

/// The override keys that name no record among `identities`.
#[cfg(test)]
fn unmatched_capacity_override_keys<'a>(
    rows: &'a [CapacityOverride],
    identities: &std::collections::BTreeSet<String>,
) -> Vec<&'a str> {
    rows.iter().map(|row| row.key.as_str()).filter(|key| !identities.contains(*key)).collect()
}

/// A container's capacity: a cited override row for the record's identity wins (`decisions.md
/// §32`); else `CONTAINS`'s first `|` field: `UNLIM…` is unlimited, a number is pounds, and a
/// `*…` weight-reduction kind or an absent token states none (DEF-1).
fn container_capacity_of(record: &EquipmentRecord) -> ContainerCapacity {
    let identity = equipment_token_value(record, "KEY").unwrap_or(record.name.as_str());
    if let Some(row) = capacity_overrides().iter().find(|row| row.key == identity) {
        return ContainerCapacity::Pounds(row.capacity_lbs);
    }
    let Some(first) = equipment_token_value(record, "CONTAINS").and_then(|value| value.split('|').next()) else {
        return ContainerCapacity::Unstated;
    };
    if first.starts_with("UNLIM") {
        ContainerCapacity::Unlimited
    } else {
        first.parse::<f64>().map_or(ContainerCapacity::Unstated, ContainerCapacity::Pounds)
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
///
/// A record cut from a template and given its own key names that template as its name, with no
/// `BASEITEM:` (`Mithral Shirt` is `Chain Shirt (Base)`, `Flame Tongue` is `Longsword (Base)`).
fn base_item_of(record: &EquipmentRecord) -> Option<String> {
    equipment_token_value(record, "BASEITEM").map(str::to_string).or_else(|| {
        let identity = equipment_token_value(record, "KEY").unwrap_or(record.name.as_str());
        (record.name != identity && record.name.ends_with(" (Base)")).then(|| record.name.clone())
    })
}

/// The engine's word for a `TYPE=` qualifier. The corpus spells some in capitals (`TYPE=DEFLECTION`).
/// `.STACK` says the bonus stacks with itself, which is what no type already means.
fn bonus_type_word(raw: &str) -> Option<String> {
    const WORDS: [&str; 21] = [
        "Alchemical", "Armor", "ArmorEnhancement", "Circumstance", "Competence", "Deflection", "Dodge",
        "Enhancement", "Insight", "Luck", "Morale", "NaturalArmor", "NaturalArmorEnhancement", "Profane",
        "Racial", "Resistance", "Sacred", "Shield", "ShieldEnhancement", "Size", "Trait",
    ];
    if raw.to_ascii_uppercase().ends_with(".STACK") {
        return None;
    }
    let word = WORDS.iter().find(|word| word.eq_ignore_ascii_case(raw)).map_or(raw, |word| word);
    (!word.is_empty()).then(|| word.to_string())
}

/// A chain's bonus type: its `TYPE=` qualifier, or a bare word where that qualifier belongs
/// (`BONUS:COMBAT|AC|2|NaturalArmor`).
fn chain_bonus_type(qualifiers: &[String]) -> Option<String> {
    let rest = qualifiers.get(3..)?;
    match rest.iter().find_map(|qualifier| qualifier.strip_prefix("TYPE=")) {
        Some(raw) => bonus_type_word(raw),
        None => rest.first().filter(|word| word.chars().all(|c| c.is_ascii_alphabetic())).and_then(|word| bonus_type_word(word)),
    }
}

/// The type of the chain `armor_class_chain_bonus_of` reads, or of the consumable sibling it falls
/// back to, by the same order `arms_armor_stat_effect_of` takes them.
fn armor_class_bonus_type_of(record: &EquipmentRecord) -> Option<String> {
    let chain = record.bonus_chains.iter().find(|bonus| {
        let qualifiers = &bonus.qualifiers;
        qualifiers.len() >= 3
            && qualifiers[0] == "COMBAT"
            && names_armor_class(&qualifiers[1])
            && !crate::pcgen_import::equipment_bonus_reader::declares_circumstance_bonus_type(bonus)
            && qualifiers[2].parse::<i16>().is_ok()
    });
    if let Some(chain) = chain {
        return chain_bonus_type(&chain.qualifiers);
    }
    record.tokens.iter().find_map(|token| {
        if token.key != "TEMPBONUS" {
            return None;
        }
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 4 || (parts[0] != "PC" && parts[0] != "ANYPC") || parts[1] != "COMBAT" || parts[2] != "AC" {
            return None;
        }
        parts[4..].iter().find_map(|part| part.strip_prefix("TYPE=")).and_then(bonus_type_word)
    })
}

/// Whether a chain holds with nothing asked of the wearer. `PREVAREQ:Disable...,0` asks only that
/// no rule has switched the armor's bonus off, which no sheet rule does.
fn chain_is_standing(qualifiers: &[String]) -> bool {
    qualifiers.iter().skip(3).all(|qualifier| {
        let is_condition = qualifier.starts_with("PRE") || qualifier.starts_with("!PRE");
        !is_condition || (qualifier.starts_with("PREVAREQ:Disable") && qualifier.ends_with(",0"))
    })
}

/// A chain's flat magnitude. `Global_LuckBonus` is a rules variable nothing on this sheet sets, so a
/// luck item's `1+Global_LuckBonus` is 1. Any other formula, and `%CHOICE`, is not a number.
fn chain_flat_value(raw: &str) -> Option<i16> {
    raw.strip_suffix("+Global_LuckBonus").unwrap_or(raw).parse().ok()
}

fn stat_target(ability: &str) -> Option<&'static str> {
    Some(match ability.to_ascii_uppercase().as_str() {
        "STR" => "ability:strength",
        "DEX" => "ability:dexterity",
        "CON" => "ability:constitution",
        "INT" => "ability:intelligence",
        "WIS" => "ability:wisdom",
        "CHA" => "ability:charisma",
        _ => return None,
    })
}

fn save_target(save: &str) -> Option<&'static str> {
    Some(match save.to_ascii_uppercase().as_str() {
        "FORTITUDE" => "save:fortitude",
        "REFLEX" => "save:reflex",
        "WILL" => "save:will",
        _ => return None,
    })
}

/// A `SKILL` chain's name, where it names a set of the Skills panel's rows: `ALL`, `TYPE.Base`,
/// `TYPE=Knowledge`. `LIST` is the item's own choice, so it is no set.
fn skill_set_target(name: &str) -> Option<String> {
    let set = name.strip_prefix("TYPE.").or_else(|| name.strip_prefix("TYPE=")).unwrap_or(name).to_ascii_lowercase();
    let is_a_set = (name.starts_with("TYPE") || set == "all") && !skill_set_rows(&set).is_empty();
    is_a_set.then(|| format!("skill_set:{set}"))
}

/// `Knowledge (Arcana)` is `skill:knowledge_arcana`, as the sheet lists it. A wildcard names no one
/// skill, so it is not a target.
fn skill_target(skill: &str) -> Option<String> {
    let wildcard = skill.is_empty()
        || skill.starts_with("TYPE")
        || skill.contains('%')
        || skill.contains('=')
        || skill.eq_ignore_ascii_case("ALL")
        || skill.eq_ignore_ascii_case("LIST");
    if wildcard {
        return None;
    }
    let words: Vec<String> = skill
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    Some(format!("skill:{}", words.join("_")))
}

/// A `COMBAT` chain's name, where it is a number on this sheet. `TOHIT.Ranged`, `.Thrown` and
/// `.Unarmed` are not: the sheet totals one attack, Melee, so the item's words print them.
fn combat_target(name: &str) -> Option<&'static str> {
    match name {
        "TOHIT" | "ATTACK" => Some("attack"),
        "INITIATIVE" => Some("initiative"),
        _ => None,
    }
}

/// A `MOVEADD` or `POSTMOVEADD` chain's movement mode, where it moves the land speed the sheet prints.
fn speed_target(mode: &str) -> Option<&'static str> {
    matches!(mode, "TYPE.Walk" | "TYPE=Walk" | "TYPE.All" | "TYPE=All").then_some("speed")
}

/// The word a `MOVEMULT` chain's whole-percent row counts as: it folds only with its own kind, and
/// one counts.
const SPEED_MULTIPLIER_WORD: &str = "Percent of speed";

/// Every standing bonus the record states on a save, an ability score, a skill or a set of skills,
/// attack, initiative, land speed, CMB or CMD, a multiplier on land speed, and the load limits.
fn stated_bonuses_of(record: &EquipmentRecord) -> Vec<StatedBonus> {
    let mut stated = Vec::new();
    for bonus in &record.bonus_chains {
        let qualifiers = &bonus.qualifiers;
        if qualifiers.len() < 3 || !chain_is_standing(qualifiers) {
            continue;
        }
        if qualifiers[0] == "MOVEMULT" {
            if let (Some(_), Ok(multiplier)) = (speed_target(&qualifiers[1]), qualifiers[2].parse::<f64>()) {
                stated.push(StatedBonus {
                    target: "speed_percent".to_owned(),
                    bonus_type: Some(SPEED_MULTIPLIER_WORD.to_owned()),
                    value: (multiplier * 100.0).round() as i16,
                });
            }
            continue;
        }
        let Some(value) = chain_flat_value(&qualifiers[2]) else { continue };
        let names = qualifiers[1].split(',');
        let targets: Vec<String> = match qualifiers[0].as_str() {
            "STAT" => names.filter_map(stat_target).map(str::to_string).collect(),
            "SKILL" => names.filter_map(|name| skill_set_target(name).or_else(|| skill_target(name))).collect(),
            "SAVE" if qualifiers[1].eq_ignore_ascii_case("ALL") => {
                ["save:fortitude", "save:reflex", "save:will"].map(str::to_string).to_vec()
            }
            "SAVE" => names.filter_map(save_target).map(str::to_string).collect(),
            // `FortitudeSave_ResistanceBonus`: a save's variable, named for the save.
            "VAR" => names
                .filter_map(|name| match name {
                    "CMB" => Some("cmb"),
                    "CMD" => Some("cmd"),
                    "LOADSCORE" => Some("load_score"),
                    _ => name.split_once("Save_").and_then(|(save, _)| save_target(save)),
                })
                .map(str::to_string)
                .collect(),
            "COMBAT" => names.filter_map(combat_target).map(str::to_string).collect(),
            "LOADMULT" if qualifiers[1] == "TYPE=SIZE" => vec!["load_multiplier".to_owned()],
            "MOVEADD" | "POSTMOVEADD" => names.filter_map(speed_target).map(str::to_string).collect(),
            _ => continue,
        };
        let bonus_type = chain_bonus_type(qualifiers);
        stated.extend(targets.into_iter().map(|target| StatedBonus { target, bonus_type: bonus_type.clone(), value }));
    }
    stated
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

/// Whether a `COMBAT` chain's names hold Armor Class: `AC`, or `ATTACK,AC`.
fn names_armor_class(names: &str) -> bool {
    names.split(',').any(|name| name == "AC")
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
            && names_armor_class(&qualifiers[1])
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

// ---------------------------------------------------------------------------------------------
// Equipment price (SD-41 E1.9, `decisions.md §26`): base + Σ eqmod cost, as PCGen's data states it
// ---------------------------------------------------------------------------------------------

/// What one equipment modifier adds to an item's price: its `COST` formulas, its `PLUS` (the
/// enhancement the `+N` rate squares), whether it is a masterwork quality, and its
/// `BONUS:ITEMCOST|<target>|<formula>[|<prereq>]` chains (the qualifiers after `ITEMCOST`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EqmodPrice {
    pub cost: Vec<String>,
    pub plus: i64,
    pub masterwork_quality: bool,
    pub item_costs: Vec<Vec<String>>,
}

impl EqmodPrice {
    /// From an eqmod record's `KEY:VALUE` tokens and its bonus chains' qualifiers.
    pub fn from_tokens(tokens: &[(String, String)], chains: &[Vec<String>]) -> Self {
        fn values<'a>(tokens: &'a [(String, String)], key: &'a str) -> impl Iterator<Item = &'a str> {
            tokens.iter().filter(move |(k, _)| k == key).map(|(_, v)| v.as_str())
        }
        EqmodPrice {
            cost: values(tokens, "COST").map(str::to_string).collect(),
            plus: values(tokens, "PLUS").next().and_then(|v| v.trim().parse().ok()).unwrap_or(0),
            masterwork_quality: values(tokens, "TYPE").any(|v| v.split('.').any(|w| w.eq_ignore_ascii_case("MasterworkQuality"))),
            item_costs: chains
                .iter()
                .filter(|q| q.first().is_some_and(|tag| tag == "ITEMCOST"))
                .map(|q| q[1..].to_vec())
                .collect(),
        }
    }
}

/// Why an item has no price the converter can state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unpriced {
    /// The item's own `COST` is a formula, not a number.
    ItemCostFormula(String),
    /// An `EQMOD` names a modifier no record holds.
    EqmodMissing(String),
    /// A cost formula uses a variable or operator the converter does not evaluate (`SIZE`, `^`).
    Formula(String),
    /// An `ITEMCOST` chain carries a prerequisite other than the masterwork-quality one.
    Prerequisite(String),
}

/// The prerequisite every `+N` and masterwork chain carries: "unless another eqmod on the item is a
/// masterwork quality" (adamantine, mithral and darkwood already include masterwork).
const NOT_ALREADY_MASTERWORK: &str = "!PRETYPE:1,EQMODTYPE=MASTERWORKQUALITY";

/// The item's price in gp as PCGen's data defines it: its `COST` (its own, else its `.COPY=`
/// base's), plus, for each `EQMOD`, that modifier's `COST` formula and every `ITEMCOST` chain whose
/// `TYPE.<word>` target the item's `TYPE` holds. A formula sees `BASECOST`, `WT`, `PLUSTOTAL` /
/// `HEADPLUSTOTAL` (the item's summed `PLUS`), `%CHOICE` (the eqmod's `|N` argument),
/// `%SPELLLEVEL` / `%CASTERLEVEL` / `%CHARGES` (its `NAME[N]` arguments), `%SPELLCOST` (the named
/// spell's material cost from `spell_cost`) and `%SPELLXPCOST` = 0. `TYPE` words add up across the
/// base and the row (`.CLEAR` resets); `EQMOD` is the row's own, else the base's.
pub fn equipment_price_gp(
    own: &[(String, String)],
    base: Option<&[(String, String)]>,
    eqmod: &dyn Fn(&str) -> Option<EqmodPrice>,
    spell_cost: &dyn Fn(&str) -> f64,
) -> Result<f64, Unpriced> {
    let base = base.unwrap_or(&[]);
    let first = |key: &str| {
        own.iter().chain(base).find(|(k, _)| k == key).map(|(_, v)| v.trim())
    };
    let base_cost = match first("COST") {
        None => 0.0,
        Some(v) => v.parse::<f64>().map_err(|_| Unpriced::ItemCostFormula(v.to_string()))?,
    };
    let weight = first("WT").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
    let mut types: Vec<String> = Vec::new();
    for (_, value) in base.iter().chain(own).filter(|(k, _)| k == "TYPE") {
        for word in value.split('.') {
            if word == "CLEAR" && value.starts_with(".CLEAR") {
                types.clear();
            } else if !word.is_empty() {
                types.push(word.to_ascii_uppercase());
            }
        }
    }
    let eqmod_value = |tokens: &[(String, String)]| {
        let joined: Vec<&str> = tokens.iter().filter(|(k, _)| k == "EQMOD").map(|(_, v)| v.as_str()).collect();
        (!joined.is_empty()).then(|| joined.join("."))
    };
    let eqmods_text = eqmod_value(own).or_else(|| eqmod_value(base)).unwrap_or_default();
    let mut mods = Vec::new();
    for part in eqmods_text.split('.').filter(|p| !p.trim().is_empty()) {
        let (key, args) = part.split_once('|').unwrap_or((part, ""));
        let key = key.trim();
        let price = eqmod(key).ok_or_else(|| Unpriced::EqmodMissing(key.to_string()))?;
        mods.push((key.to_string(), args.to_string(), price));
    }
    let plus_total: i64 = mods.iter().map(|(_, _, m)| m.plus).sum();
    let mut total = base_cost;
    for (index, (_, args, price)) in mods.iter().enumerate() {
        let mut vars = std::collections::BTreeMap::from([
            ("BASECOST".to_string(), base_cost),
            ("WT".to_string(), weight),
            ("PLUSTOTAL".to_string(), plus_total as f64),
            ("HEADPLUSTOTAL".to_string(), plus_total as f64),
            ("SPELLXPCOST".to_string(), 0.0),
        ]);
        if let Ok(choice) = args.trim().parse::<f64>() {
            vars.insert("CHOICE".to_string(), choice);
        }
        let mut rest = args.as_str();
        while let Some((name, tail)) = rest.split_once('[') {
            let Some((value, after)) = tail.split_once(']') else { break };
            let name = name.trim();
            match name {
                "SPELLNAME" => {
                    vars.insert("SPELLCOST".to_string(), spell_cost(value));
                }
                "SPELLLEVEL" | "CASTERLEVEL" | "CHARGES" => {
                    if let Ok(n) = value.trim().parse::<f64>() {
                        vars.insert(name.to_string(), n);
                    }
                }
                _ => {}
            }
            rest = after;
        }
        vars.entry("SPELLCOST".to_string()).or_insert(0.0);
        for formula in &price.cost {
            total += evaluate_price_formula(formula, &vars)?;
        }
        let other_masterwork = mods.iter().enumerate().any(|(i, (_, _, m))| i != index && m.masterwork_quality);
        for chain in &price.item_costs {
            let (Some(target), Some(formula)) = (chain.first(), chain.get(1)) else { continue };
            let applies = target.split(',').any(|t| {
                t.split_once('.').is_some_and(|(kind, word)| {
                    kind.eq_ignore_ascii_case("TYPE") && types.iter().any(|w| w.eq_ignore_ascii_case(word))
                })
            });
            if !applies {
                continue;
            }
            let mut holds = true;
            for prereq in &chain[2..] {
                if prereq == NOT_ALREADY_MASTERWORK {
                    holds &= !other_masterwork;
                } else {
                    return Err(Unpriced::Prerequisite(prereq.clone()));
                }
            }
            if holds {
                total += evaluate_price_formula(formula, &vars)?;
            }
        }
    }
    Ok(total)
}

/// One cost formula over `vars` (`%NAME` and `NAME` both read `vars["NAME"]`), in `f64`.
fn evaluate_price_formula(formula: &str, vars: &std::collections::BTreeMap<String, f64>) -> Result<f64, Unpriced> {
    use crate::pcgen_import::formula_interpreter::{parse, CmpOp, Expr};
    fn eval(expr: &Expr, vars: &std::collections::BTreeMap<String, f64>) -> Option<f64> {
        Some(match expr {
            Expr::Num(n) => *n,
            Expr::Var(name) => *vars.get(name)?,
            Expr::Neg(a) => -eval(a, vars)?,
            Expr::Add(a, b) => eval(a, vars)? + eval(b, vars)?,
            Expr::Sub(a, b) => eval(a, vars)? - eval(b, vars)?,
            Expr::Mul(a, b) => eval(a, vars)? * eval(b, vars)?,
            Expr::Div(a, b) => eval(a, vars)? / eval(b, vars)?,
            Expr::Call(name, args) => {
                let values = args.iter().map(|a| eval(a, vars)).collect::<Option<Vec<f64>>>()?;
                match name.as_str() {
                    "max" => values.into_iter().fold(f64::NEG_INFINITY, f64::max),
                    "min" => values.into_iter().fold(f64::INFINITY, f64::min),
                    "floor" => values.first()?.floor(),
                    "ceil" => values.first()?.ceil(),
                    _ => return None,
                }
            }
            Expr::Cmp(a, op, b) => {
                let (a, b) = (eval(a, vars)?, eval(b, vars)?);
                let truth = match op {
                    CmpOp::Ge => a >= b,
                    CmpOp::Le => a <= b,
                    CmpOp::Eq => a == b,
                    CmpOp::Ne => a != b,
                    CmpOp::Gt => a > b,
                    CmpOp::Lt => a < b,
                };
                f64::from(u8::from(truth))
            }
            _ => return None,
        })
    }
    let unpriced = || Unpriced::Formula(formula.to_string());
    let expr = parse(&formula.replace('%', "")).map_err(|_| unpriced())?;
    eval(&expr, vars).filter(|v| v.is_finite()).ok_or_else(unpriced)
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

    /// A record carrying exactly these `KEY:VALUE` tokens.
    fn equipment_with(tokens: &[(&str, &str)]) -> EquipmentRecord {
        let mut record = fake_equipment();
        record.tokens = tokens
            .iter()
            .map(|(key, value)| EquipmentToken {
                key: key.to_string(),
                value: value.to_string(),
                line_number: 4,
                raw_pair: format!("{key}:{value}"),
            })
            .collect();
        record
    }

    /// SD-41 E1.1 (`decisions.md §31` R2, "Container tag, but not weapons").
    #[test]
    fn container_of_is_container_typed_and_not_a_weapon() {
        let backpack = equipment_with(&[("TYPE", "Goods.Container.General.Resizable"), ("CONTAINS", "UNLIM|Any")]);
        let pistol = equipment_with(&[
            ("TYPE", "Weapon.Exotic.Ranged.Container.Projectile.Firearm"),
            ("CONTAINS", "1|FirearmBullet=1"),
            ("DAMAGE", "1d8"),
        ]);
        let launcher = equipment_with(&[("TYPE", "Weapon.Exotic.Ranged.Container.Projectile"), ("CONTAINS", "UNLIM|Dart=1")]);
        let rope = equipment_with(&[("TYPE", "Goods.General"), ("CONTAINS", "UNLIM|Any")]);
        assert!(container_of(&backpack));
        assert!(!container_of(&pistol), "states DAMAGE");
        assert!(!container_of(&launcher), "Weapon-typed");
        assert!(!container_of(&rope), "no Container type word");
        let converted = equipment_record_to_corpus(&backpack);
        assert!(converted.is_container());
        let converted = equipment_record_to_corpus(&pistol);
        assert!(!converted.is_container());
        assert_eq!(converted.capacity(), ContainerCapacity::Unstated, "capacity settled only on a container");
    }

    /// SD-41 E1.6 (`decisions.md §23, §31`): ammunition users, their family, ammunition, bundle size.
    #[test]
    fn ammunition_of_settles_projectile_family_kinds_and_bundle() {
        let longbow = equipment_with(&[("TYPE", "Weapon.Ranged.Container.Projectile.Bow"), ("CONTAINS", "UNLIM|Arrow"), ("DAMAGE", "1d8")]);
        let pistol = equipment_with(&[("TYPE", "Weapon.Container.Projectile.Firearm"), ("CONTAINS", "1|Firearm Bullet=1|FirearmPellet=1"), ("DAMAGE", "1d8")]);
        let sling = equipment_with(&[("TYPE", "Weapon.Ranged.Projectile.Sling"), ("DAMAGE", "1d4")]);
        let dagger = equipment_with(&[("TYPE", "Weapon.Melee.Ranged.Thrown.Dagger"), ("DAMAGE", "1d4")]);
        let arrows = equipment_with(&[("TYPE", "Ammunition.Resizable.Standard.Arrow"), ("BASEQTY", "20")]);
        let backpack = equipment_with(&[("TYPE", "Goods.Container.General"), ("CONTAINS", "UNLIM|Any")]);
        let settled = |record: &EquipmentRecord| {
            let r = equipment_record_to_corpus(record);
            (r.uses_ammunition(), r.ammunition_family().map(str::to_owned), r.is_ammunition(), r.base_quantity())
        };
        assert_eq!(settled(&longbow), (true, Some("Arrow".to_owned()), false, 1));
        assert_eq!(settled(&pistol), (true, Some("FirearmBullet".to_owned()), false, 1));
        assert_eq!(settled(&sling), (true, None, false, 1), "no CONTAINS: any ammunition");
        assert_eq!(settled(&dagger), (false, None, false, 1), "Thrown is not Projectile");
        assert_eq!(settled(&arrows), (false, None, true, 20));
        assert_eq!(settled(&backpack), (false, None, false, 1), "a container's CONTAINS is no family");
        assert!(equipment_record_to_corpus(&arrows).is_ammunition_for(Some("Arrow")));
        assert!(!equipment_record_to_corpus(&arrows).is_ammunition_for(Some("Bolt")));
    }

    #[test]
    fn container_of_capacity_reads_the_first_contains_field() {
        let capacity = |contains: Option<&str>| {
            let mut tokens = vec![("TYPE", "Goods.Container")];
            tokens.extend(contains.map(|value| ("CONTAINS", value)));
            container_capacity_of(&equipment_with(&tokens))
        };
        assert_eq!(capacity(Some("UNLIM|Any")), ContainerCapacity::Unlimited);
        assert_eq!(capacity(Some("2000|Any")), ContainerCapacity::Pounds(2000.0));
        assert_eq!(capacity(Some("1.5|Liquid=3")), ContainerCapacity::Pounds(1.5));
        assert_eq!(capacity(Some("*20|Any")), ContainerCapacity::Unstated, "weight-reduction kind (DEF-1)");
        assert_eq!(capacity(None), ContainerCapacity::Unstated);
        assert_eq!(equipment_record_to_corpus(&equipment_with(&[("TYPE", "Goods.Container"), ("CONTAINS", "60|Any")])).capacity(), ContainerCapacity::Pounds(60.0));
    }

    /// SD-41 E1.12 (`decisions.md §32`): a cited override row wins over the `CONTAINS` token, by
    /// the record's identity (`KEY`, else its name); other containers keep the token's capacity.
    #[test]
    fn capacity_override_row_wins_over_the_token() {
        let backpack = equipment_with(&[("KEY", "Backpack"), ("TYPE", "Goods.Container.General"), ("CONTAINS", "UNLIM|Any")]);
        let masterwork = equipment_with(&[("KEY", "Backpack (Masterwork)"), ("TYPE", "Goods.Container"), ("CONTAINS", "UNLIM|Any")]);
        assert_eq!(equipment_record_to_corpus(&backpack).capacity(), ContainerCapacity::Pounds(60.0));
        assert_eq!(equipment_record_to_corpus(&masterwork).capacity(), ContainerCapacity::Unlimited);
        assert!(capacity_overrides().iter().all(|row| !row.source.kind.is_empty() && !row.source.reference.is_empty()), "every row is cited");
    }

    /// SD-41 E1.12: a row whose key names no converted equipment record is an error, and every row
    /// of the shipped list names a record in the settled corpus.
    #[test]
    fn capacity_override_unknown_key_fails() {
        let rows: Vec<CapacityOverride> = serde_json::from_str(
            r#"[{ "key": "Backpack", "capacity_lbs": 60, "source": { "kind": "operator-ruling", "ref": "test", "date": "2026-11-12" } },
                { "key": "No Such Bag", "capacity_lbs": 5, "source": { "kind": "book", "ref": "test", "date": "2026-11-12" } }]"#,
        )
        .unwrap();
        let identities: std::collections::BTreeSet<String> = ["Backpack".to_string()].into();
        assert_eq!(unmatched_capacity_override_keys(&rows, &identities), vec!["No Such Bag"]);

        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut corpus = std::collections::BTreeSet::new();
        for book in codex::rules_core::settled_corpus::corpus_book_dirs(&root.join("data/corpus")) {
            if let Ok(bundle) = codex::rules_core::settled_corpus::read_equipment_bundle(&book) {
                corpus.extend(bundle.records.into_values().map(|entry| entry.record.identity));
            }
        }
        assert!(corpus.len() > 1000, "settled equipment identities read: {}", corpus.len());
        assert_eq!(unmatched_capacity_override_keys(capacity_overrides(), &corpus), Vec::<&str>::new());
    }
}

#[cfg(test)]
mod price_tests {
    use super::*;

    fn tokens(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn chain(q: &[&str]) -> Vec<String> {
        q.iter().map(|s| s.to_string()).collect()
    }

    /// The CRB eqmods the cases below name, as `cr_equipmods.lst` states them.
    fn crb_eqmod(key: &str) -> Option<EqmodPrice> {
        Some(match key {
            "Material ~ Steel" => EqmodPrice::from_tokens(&tokens(&[("COST", "0"), ("TYPE", "BaseMaterial.Mundane")]), &[]),
            "Material ~ Adamantine ~ Armor / Medium" => EqmodPrice::from_tokens(
                &tokens(&[("TYPE", "BaseMaterial.MasterworkQuality.Armor"), ("COST", "10000")]),
                &[],
            ),
            "Special Quality ~ Masterwork ~ Armor" => EqmodPrice::from_tokens(
                &tokens(&[("TYPE", "MasterworkQuality.Armor.Shield"), ("COST", "0")]),
                &[chain(&["ITEMCOST", "TYPE.ARMOR", "150"]), chain(&["ITEMCOST", "TYPE.SHIELD", "150"])],
            ),
            "Special Ability ~ +3 ~ Armor" => EqmodPrice::from_tokens(
                &tokens(&[("TYPE", "Armor"), ("PLUS", "3")]),
                &[
                    chain(&["ITEMCOST", "TYPE.ARMOR", "1000*PLUSTOTAL*PLUSTOTAL"]),
                    chain(&["ITEMCOST", "TYPE.ARMOR", "150", NOT_ALREADY_MASTERWORK]),
                ],
            ),
            "Special Ability ~ Enhancement Cost" => EqmodPrice::from_tokens(&tokens(&[("COST", "%CHOICE")]), &[]),
            "Special Ability ~ Spell Effect / Spell Trigger" => EqmodPrice::from_tokens(
                &tokens(&[(
                    "COST",
                    "((15*(max(%SPELLLEVEL,(1/2)))*%CASTERLEVEL)+%SPELLCOST+(5*%SPELLXPCOST))*%CHARGES",
                )]),
                &[],
            ),
            "Size" => EqmodPrice::from_tokens(&tokens(&[("COST", "BASECOST*(2^(SIZE-4))")]), &[]),
            "Odd Prerequisite" => EqmodPrice::from_tokens(&[], &[chain(&["ITEMCOST", "TYPE.ARMOR", "5", "!PRETYPE:1,Shield"])]),
            _ => return None,
        })
    }

    fn price(own: &[(&str, &str)], base: Option<&[(&str, &str)]>) -> Result<f64, Unpriced> {
        let base = base.map(tokens);
        equipment_price_gp(&tokens(own), base.as_deref(), &crb_eqmod, &|_| 0.0)
    }

    const BREASTPLATE_BASE: &[(&str, &str)] =
        &[("KEY", "Breastplate (Base)"), ("TYPE", "Armor.Medium.ArmorProfMedium.Suit"), ("COST", "200"), ("WT", "30")];

    #[test]
    fn equipment_price_adamantine_breastplate_is_base_plus_material() {
        // CRB p.464: 10,200 gp. The `.COPY=` row states no COST of its own.
        let own = [("KEY", "Adamantine Breastplate"), ("EQMOD", "Material ~ Adamantine ~ Armor / Medium")];
        assert_eq!(price(&own, Some(BREASTPLATE_BASE)), Ok(10_200.0));
    }

    #[test]
    fn equipment_price_specific_armor_sums_enhancement_masterwork_and_choice() {
        // Banded Mail of Luck, CRB p.465: 250 + 150 masterwork + 1,000 × 3² + 9,500 = 18,900 gp.
        let base = [("KEY", "Banded Mail (Base)"), ("TYPE", "Armor.Heavy.Suit"), ("COST", "250")];
        let own = [(
            "EQMOD",
            "Material ~ Steel.Special Ability ~ +3 ~ Armor.Special Ability ~ Enhancement Cost|9500",
        )];
        assert_eq!(price(&own, Some(&base)), Ok(18_900.0));
    }

    #[test]
    fn equipment_price_masterwork_is_not_charged_twice_on_a_masterwork_material() {
        let own = [("EQMOD", "Material ~ Adamantine ~ Armor / Medium.Special Ability ~ +3 ~ Armor")];
        assert_eq!(price(&own, Some(BREASTPLATE_BASE)), Ok(200.0 + 10_000.0 + 9_000.0));
    }

    #[test]
    fn equipment_price_literal_cost_sums_with_its_eqmods_never_negative() {
        // Astral Armor (Ultimate Psionics): COST:-150 beside the masterwork eqmod's +150.
        let own = [("TYPE", "Armor.Medium.Suit"), ("COST", "-150"), ("EQMOD", "Special Quality ~ Masterwork ~ Armor")];
        assert_eq!(price(&own, None), Ok(0.0));
    }

    #[test]
    fn equipment_price_spell_trigger_reads_the_eqmod_arguments() {
        // Wand of Magic Missile, CRB p.496: 15 × 1 × 1 × 50 charges = 750 gp.
        let base = [("TYPE", "Wand.Combat Gear"), ("COST", "0")];
        let own = [(
            "EQMOD",
            "Special Ability ~ Spell Effect / Spell Trigger|SPELLNAME[Magic Missile]SPELLLEVEL[1]CASTERLEVEL[1]CHARGES[50]",
        )];
        assert_eq!(price(&own, Some(&base)), Ok(750.0));
    }

    #[test]
    fn equipment_price_type_adds_across_the_copy_and_clear_resets() {
        // A `.COPY=` row's TYPE adds to its base's: the armor chains still apply.
        let own = [("TYPE", "Magic"), ("EQMOD", "Special Quality ~ Masterwork ~ Armor")];
        assert_eq!(price(&own, Some(BREASTPLATE_BASE)), Ok(350.0));
        // `.CLEAR` drops the base's words: no ARMOR word, no armor chain.
        let cleared = [("TYPE", ".CLEAR"), ("TYPE", "Goods"), ("EQMOD", "Special Quality ~ Masterwork ~ Armor")];
        assert_eq!(price(&cleared, Some(BREASTPLATE_BASE)), Ok(200.0));
    }

    #[test]
    fn equipment_price_refuses_what_it_cannot_evaluate() {
        assert_eq!(price(&[("COST", "Unknown")], None), Err(Unpriced::ItemCostFormula("Unknown".to_string())));
        assert_eq!(price(&[("EQMOD", "Nope")], None), Err(Unpriced::EqmodMissing("Nope".to_string())));
        assert_eq!(
            price(&[("COST", "10"), ("EQMOD", "Size")], None),
            Err(Unpriced::Formula("BASECOST*(2^(SIZE-4))".to_string()))
        );
        assert_eq!(
            price(&[("EQMOD", "Odd Prerequisite")], Some(BREASTPLATE_BASE)),
            Err(Unpriced::Prerequisite("!PRETYPE:1,Shield".to_string()))
        );
    }
}
