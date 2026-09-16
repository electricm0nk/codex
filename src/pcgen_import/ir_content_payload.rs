//! The converter's own kind-tagged IR payload (SD-17 Slice E; SD-35
//! `AT-35-E6-003-RULED` cycle 18).
//!
//! This module is the home of [`IrContentPayload`], the typed per-kind variant
//! enum the **converter** puts behind [`SourceContentRecord`]. Four of its
//! seven variants hold a borrowed reference to a B-family entry type from the
//! Slice B parsers — zero-copy projection over raw PCGen ingest vocabulary.
//!
//! ## Why this enum is not the live envelope's payload
//!
//! Until cycle 18 this enum *was* the live payload: `rules_core::source_content`
//! re-exported it, and every live consumer of the canonical envelope therefore
//! named `pcgen_import` transitively. Under `decisions.md` §11 and ruling B16
//! (§19) that re-export is a residue hit — naming the converter in shipping code
//! is reading the converter — and it was the criterion's largest remaining one,
//! refused for eleven cycles because the enum could not simply move: the
//! `Class`, `SpellcastingClass`, `Race`/`Ability` and `Metadata` variants borrow
//! parser entry types, and moving *them* to the live side would have moved raw
//! `CLASS:`/`RACE:` token vectors with them.
//!
//! Cycle 18 split the type rather than moving it. The canonical envelope
//! ([`SourceContentRecord`], [`SourcePackageContent`]) is generic over its
//! payload and defaults to the live
//! [`SourceContentPayload`](crate::rules_core::source_content::SourceContentPayload),
//! which names only rules-core types. The converter instantiates the same
//! envelope with this enum instead, via the [`IrContentRecord`] and
//! [`IrPackageContent`] aliases, and [`record_to_live`] / [`package_to_live`]
//! project one to the other at the boundary — the same place the corpus stops
//! being ingest vocabulary and starts being sheet data.
//!
//! Nothing was deleted: every variant, every borrow and every proof the
//! slice-E integration suites make about them is still here, on the
//! converter side that `decisions.md` §11 explicitly KEEPS.

use crate::rules_core::source_content::{
    SourceContentKind, SourceContentPayload, SourceContentRecord, SourcePackageContent,
};

use crate::pcgen_import::lst_parser::class::ClassEntry;
use crate::pcgen_import::lst_parser::metadata::{LstRecord, MetadataKind};
use crate::pcgen_import::lst_parser::race_ability::{AbilityDeclaration, RaceDeclaration};
use crate::pcgen_import::lst_parser::spellcasting_class::SpellcastingClassEntry;
use crate::rules_core::equipment_record::CorpusEquipmentRecord;
use crate::rules_core::spell_record::CorpusSpellRecord;

// =============================================================================
// B-6 MetadataKind <-> MetadataKindInner mapping
// =============================================================================
//
// The B-6 [`crate::pcgen_import::lst_parser::metadata::MetadataKind`] is the
// parser's enum; the canonical [`crate::rules_core::source_content::MetadataKindInner`]
// is the rules-core mirror. They have the same shape (six variants, same names,
// same directive-token spellings). The mapping is total and mechanical. We
// expose it here in `pcgen_import` because the conversion involves the
// parser-side enum, and any cycle-avoidance gain from the module split would
// be undone if we placed the mapping in `rules_core::source_content` (which
// cannot import from `pcgen_import::lst_parser::metadata`).

/// Map a B-6 [`MetadataKind`] to the canonical [`crate::rules_core::source_content::MetadataKindInner`].
///
/// The mapping is total (one-to-one, every variant accounted for). Inverse
/// of [`canonical_metadata_kind_inner_to_b6`].
pub fn b6_metadata_kind_to_canonical(
    b6_kind: MetadataKind,
) -> crate::rules_core::source_content::MetadataKindInner {
    use crate::rules_core::source_content::MetadataKindInner;
    match b6_kind {
        MetadataKind::Deity => MetadataKindInner::Deity,
        MetadataKind::Domain => MetadataKindInner::Domain,
        MetadataKind::Kits => MetadataKindInner::Kits,
        MetadataKind::Language => MetadataKindInner::Language,
        MetadataKind::Template => MetadataKindInner::Template,
        MetadataKind::CompanionMod => MetadataKindInner::CompanionMod,
    }
}

/// Inverse of [`b6_metadata_kind_to_canonical`]: map a canonical
/// [`crate::rules_core::source_content::MetadataKindInner`] back to the B-6
/// [`MetadataKind`].
pub fn canonical_metadata_kind_inner_to_b6(
    canonical: crate::rules_core::source_content::MetadataKindInner,
) -> MetadataKind {
    use crate::rules_core::source_content::MetadataKindInner;
    match canonical {
        MetadataKindInner::Deity => MetadataKind::Deity,
        MetadataKindInner::Domain => MetadataKind::Domain,
        MetadataKindInner::Kits => MetadataKind::Kits,
        MetadataKindInner::Language => MetadataKind::Language,
        MetadataKindInner::Template => MetadataKind::Template,
        MetadataKindInner::CompanionMod => MetadataKind::CompanionMod,
    }
}

/// Canonical source-content payload: one variant per B-family record
/// kind plus metadata, each holding a borrowed reference to the parser
/// entry.
///
/// The enum is total over the B-family kinds (the contract requires it
/// to cover every Slice B output, per the 2026-07-12 operator directive
/// that this slice accepts the full PF1 corpus without narrowing). The
/// metadata variants follow [`crate::pcgen_import::lst_parser::metadata::MetadataKind`].
///
/// Every variant is a borrow (`&'a ParserEntry`) — projection is
/// zero-copy. Callers that need to own the projected record clone the
/// underlying entry explicitly; the canonical-IR surface never clones.
// `Eq` is deliberately absent: SD-35 `AT-35-E6-003-RULED` cycle 10 added the
// converted equipment record to this enum, and a settled weight in pounds is an
// `f64`. `PartialEq` is what every caller uses.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrContentPayload<'a> {
    /// A martial class record from the B-1 parser.
    Class(&'a ClassEntry),
    /// A spellcasting class record from the B-2 parser.
    SpellcastingClass(&'a SpellcastingClassEntry),
    /// A race pointer declaration from the B-3 parser.
    Race(&'a RaceDeclaration),
    /// An ability declaration from the B-3 parser.
    Ability(&'a AbilityDeclaration),
    /// A spell record in the live side's own converted shape.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 8: this variant is deliberately
    /// **not** a borrow of the B-4 parser row. The spell kind is the first
    /// one whose converted shape the live side owns
    /// ([`CorpusSpellRecord`](crate::rules_core::spell_record::CorpusSpellRecord)),
    /// so `rules_core::spell_resolver` and everything downstream of it no
    /// longer name `pcgen_import` at all (`decisions.md` §11, §19). The
    /// conversion happens once, in
    /// [`crate::pcgen_import::ir_converter::spell_record_to_corpus`], for a
    /// record that came from a raw `.lst` row; a record read from
    /// already-converted `data/corpus/` JSON never touches the converter.
    Spell(&'a CorpusSpellRecord),
    /// An equipment or equipment-modifier record in the live side's own
    /// converted shape.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 13: **the variant has collapsed.** It
    /// carried the B-5 parser row alongside the settled
    /// [`CorpusEquipmentRecord`](crate::rules_core::equipment_record::CorpusEquipmentRecord)
    /// from cycle 10, as a stated transition shape, "until the last consumer
    /// that has not moved does". Cycle 12 moved the last two
    /// (`damage_total`, `equipment_effects`) and cycle 13 moved the resolver
    /// and the loader, so the row has no live reader left and is gone from the
    /// envelope. Equipment is now the second kind, after cycle 8's spell, whose
    /// canonical payload names no `pcgen_import` type at all
    /// (`decisions.md` §11, §19).
    Equipment(&'a CorpusEquipmentRecord),
    /// A metadata-kind record from the B-6 parser. The
    /// `MetadataKind` is reachable on the borrowed entry itself
    /// (`record.kind`); the canonical envelope's
    /// [`crate::rules_core::source_content::SourceContentKind`]
    /// mirrors the same tag for routing convenience.
    Metadata(&'a LstRecord),
}

impl<'a> IrContentPayload<'a> {
    /// The canonical directive-token prefix for this payload variant.
    /// Mirrors [`crate::rules_core::source_content::SourceContentKind::token`]
    /// so callers that hold either an envelope or a payload can ask the
    /// same question.
    pub fn kind_token(&self) -> &'static str {
        match self {
            IrContentPayload::Class(_) => "CLASS",
            IrContentPayload::SpellcastingClass(_) => "CLASS",
            IrContentPayload::Race(_) => "RACE",
            IrContentPayload::Ability(_) => "ABILITY",
            IrContentPayload::Spell(_) => "SPELL",
            IrContentPayload::Equipment(e) => {
                if e.is_modifier {
                    "EQUIPMOD"
                } else {
                    "EQUIP"
                }
            }
            IrContentPayload::Metadata(m) => m.kind.token(),
        }
    }

    /// The originating B-family slice tag for this payload variant.
    /// Same answer as the matching
    /// [`crate::rules_core::source_content::SourceContentKind::source_slice`].
    pub fn source_slice(&self) -> &'static str {
        match self {
            IrContentPayload::Class(_) => "SD17-B-1",
            IrContentPayload::SpellcastingClass(_) => "SD17-B-2",
            IrContentPayload::Race(_) | IrContentPayload::Ability(_) => "SD17-B-3",
            IrContentPayload::Spell(_) => "SD17-B-4",
            IrContentPayload::Equipment(..) => "SD17-B-5",
            IrContentPayload::Metadata(_) => "SD17-B-6",
        }
    }
}

// =============================================================================
// The converter's envelope instantiation, and the boundary projection
// =============================================================================

/// The canonical envelope carrying the converter's own payload.
///
/// Same struct as the live [`SourceContentRecord<'a>`], instantiated with
/// [`IrContentPayload`] instead of the live default.
pub type IrContentRecord<'a> = SourceContentRecord<'a, IrContentPayload<'a>>;

/// The canonical aggregate carrying the converter's own payload.
pub type IrPackageContent<'a> = SourcePackageContent<'a, IrContentPayload<'a>>;

impl<'a> IrContentPayload<'a> {
    /// The record's canonical name, as the live side is allowed to know it.
    ///
    /// This is identity — the string a sheet would print to say *which* record
    /// this is — never ingest vocabulary. No raw directive line, no token
    /// vector and no `KEY:VAL` text crosses through here.
    pub fn canonical_name(&self) -> &'a str {
        match self {
            IrContentPayload::Class(e) => e.class_name.as_str(),
            IrContentPayload::SpellcastingClass(e) => e.class_name.as_str(),
            IrContentPayload::Race(d) => d.target.as_str(),
            IrContentPayload::Ability(d) => match &d.parsed {
                Some(p) => p.name.as_str(),
                None => "",
            },
            IrContentPayload::Spell(s) => s.name.as_str(),
            IrContentPayload::Equipment(e) => e.name.as_str(),
            IrContentPayload::Metadata(m) => m.name.as_str(),
        }
    }

    /// Project this converter payload onto the live envelope's payload.
    ///
    /// The two settled kinds carry their live-owned record straight through —
    /// they already *are* the live shape. Every parser-borrowing kind becomes
    /// [`SourceContentPayload::Unsettled`], carrying the envelope's kind tag and
    /// [`canonical_name`](IrContentPayload::canonical_name) and nothing else,
    /// which is the whole point of the split: the parser entry stops at the
    /// boundary (`decisions.md` §11, §19).
    pub fn to_live(&self, kind: SourceContentKind) -> SourceContentPayload<'a> {
        match self {
            IrContentPayload::Spell(s) => SourceContentPayload::Spell(s),
            IrContentPayload::Equipment(e) => SourceContentPayload::Equipment(e),
            _ => SourceContentPayload::Unsettled {
                kind,
                name: self.canonical_name(),
            },
        }
    }
}

/// Project one converter record onto the live envelope, preserving its
/// provenance anchor and kind tag exactly.
pub fn record_to_live<'a>(record: &IrContentRecord<'a>) -> SourceContentRecord<'a> {
    SourceContentRecord::new(
        record.source_ref.clone(),
        record.kind,
        record.payload.to_live(record.kind),
    )
}

/// Project a whole converter package onto the live envelope, record for record
/// and diagnostic for diagnostic, in order.
pub fn package_to_live<'a>(package: IrPackageContent<'a>) -> SourcePackageContent<'a> {
    let mut live = SourcePackageContent::empty(package.package_id, package.source_ref);
    for record in &package.records {
        live.push(record_to_live(record));
    }
    for diagnostic in package.diagnostics {
        live.push_diagnostic(diagnostic);
    }
    live
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use crate::pcgen_import::lst_parser::class::ClassEntry;
    use crate::rules_core::source_content::{MetadataKindInner, SourceRef};

    fn class_entry(name: &str) -> ClassEntry {
        ClassEntry {
            class_name: name.to_string(),
            header_line_number: 1,
            header_raw_line: format!("CLASS:{name}\tHD:10\tBONUS:COMBAT|BAB|CL"),
            tokens: Vec::new(),
            feature_blocks: Vec::new(),
        }
    }

    #[test]
    fn an_unsettled_kind_crosses_the_boundary_as_identity_only() {
        let entry = class_entry("Fighter");
        let ir = SourceContentRecord::new(
            SourceRef::new("classes.lst", 7),
            SourceContentKind::Class,
            IrContentPayload::Class(&entry),
        );
        let live = record_to_live(&ir);
        assert_eq!(live.source_ref, SourceRef::new("classes.lst", 7));
        assert_eq!(live.kind, SourceContentKind::Class);
        assert_eq!(
            live.payload,
            SourceContentPayload::Unsettled {
                kind: SourceContentKind::Class,
                name: "Fighter",
            }
        );
        // The parser entry itself does not cross: the only thing reachable
        // through the live payload is the name.
        assert_eq!(live.payload.unsettled_name(), Some("Fighter"));
    }

    #[test]
    fn the_kind_token_and_slice_survive_the_projection_for_every_kind() {
        let entry = class_entry("Fighter");
        for kind in [
            SourceContentKind::Class,
            SourceContentKind::SpellcastingClass,
            SourceContentKind::Race,
            SourceContentKind::Ability,
            SourceContentKind::Metadata(MetadataKindInner::Deity),
        ] {
            let payload = IrContentPayload::Class(&entry);
            let live = payload.to_live(kind);
            assert_eq!(live.kind_token(), kind.token(), "token for {kind:?}");
            assert_eq!(live.source_slice(), kind.source_slice(), "slice for {kind:?}");
        }
    }

    #[test]
    fn package_projection_preserves_record_order_and_diagnostics() {
        let a = class_entry("Fighter");
        let b = class_entry("Barbarian");
        let mut pkg: IrPackageContent<'_> =
            SourcePackageContent::empty("pf1.core_rulebook", SourceRef::new("pf1.pcc", 0));
        pkg.push(SourceContentRecord::new(
            SourceRef::new("classes.lst", 1),
            SourceContentKind::Class,
            IrContentPayload::Class(&a),
        ));
        pkg.push(SourceContentRecord::new(
            SourceRef::new("classes.lst", 9),
            SourceContentKind::Class,
            IrContentPayload::Class(&b),
        ));

        let live = package_to_live(pkg);
        assert_eq!(live.package_id, "pf1.core_rulebook");
        assert_eq!(live.len(), 2);
        assert_eq!(live.records[0].payload.unsettled_name(), Some("Fighter"));
        assert_eq!(live.records[1].payload.unsettled_name(), Some("Barbarian"));
        assert_eq!(live.records[1].source_ref.line, 9);
    }
}
