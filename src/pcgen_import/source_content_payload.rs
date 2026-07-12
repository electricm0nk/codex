//! Canonical source-content payload variants (SD-17 Slice E).
//!
//! This module is the home of [`SourceContentPayload`], the typed
//! per-kind variant enum that lives behind [`SourceContentRecord`].
//! The variants hold a borrowed reference to a B-family entry type
//! from the Slice B parsers — zero-copy projection. The rest of the
//! canonical source-IR envelope ([`SourcePackageContent`],
//! [`SourceContentRecord`], [`SourceRef`], [`SourceContentKind`],
//! [`SourceContentDiagnostic`], [`SourceContentLoadResult`]) lives in
//! [`crate::rules_core::source_content`] and re-exports this enum.
//!
//! ## Why the payload lives here and not in `rules_core::source_content`
//!
//! The variants reference parser entry types from
//! `crate::pcgen_import::lst_parser::*`. If the enum were defined in
//! `rules_core::source_content`, the import graph would form a cycle:
//! `pcgen_import::ir_converter` already constructs the canonical
//! `SourceContentRecord` and would have to import from
//! `rules_core::source_content`. Putting the payload variants next
//! to the parser surface (in `pcgen_import`) keeps the cycle
//! acyclic — `rules_core::source_content` re-exports this enum, and
//! the only dependency that crosses the boundary is the canonical
//! envelope constructed by `pcgen_import::ir_converter` and consumed
//! by `rules_core` consumers.

use crate::pcgen_import::lst_parser::class::ClassEntry;
use crate::pcgen_import::lst_parser::equipment::EquipmentRecord;
use crate::pcgen_import::lst_parser::metadata::{LstRecord, MetadataKind};
use crate::pcgen_import::lst_parser::race_ability::{AbilityDeclaration, RaceDeclaration};
use crate::pcgen_import::lst_parser::spell::LstSpellRecord;
use crate::pcgen_import::lst_parser::spellcasting_class::SpellcastingClassEntry;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceContentPayload<'a> {
    /// A martial class record from the B-1 parser.
    Class(&'a ClassEntry),
    /// A spellcasting class record from the B-2 parser.
    SpellcastingClass(&'a SpellcastingClassEntry),
    /// A race pointer declaration from the B-3 parser.
    Race(&'a RaceDeclaration),
    /// An ability declaration from the B-3 parser.
    Ability(&'a AbilityDeclaration),
    /// A spell row record from the B-4 parser.
    Spell(&'a LstSpellRecord),
    /// An equipment or equipment-modifier record from the B-5 parser.
    Equipment(&'a EquipmentRecord),
    /// A metadata-kind record from the B-6 parser. The
    /// `MetadataKind` is reachable on the borrowed entry itself
    /// (`record.kind`); the canonical envelope's
    /// [`crate::rules_core::source_content::SourceContentKind`]
    /// mirrors the same tag for routing convenience.
    Metadata(&'a LstRecord),
}

impl<'a> SourceContentPayload<'a> {
    /// The canonical directive-token prefix for this payload variant.
    /// Mirrors [`crate::rules_core::source_content::SourceContentKind::token`]
    /// so callers that hold either an envelope or a payload can ask the
    /// same question.
    pub fn kind_token(&self) -> &'static str {
        match self {
            SourceContentPayload::Class(_) => "CLASS",
            SourceContentPayload::SpellcastingClass(_) => "CLASS",
            SourceContentPayload::Race(_) => "RACE",
            SourceContentPayload::Ability(_) => "ABILITY",
            SourceContentPayload::Spell(_) => "SPELL",
            SourceContentPayload::Equipment(e) => e.kind.token(),
            SourceContentPayload::Metadata(m) => m.kind.token(),
        }
    }

    /// The originating B-family slice tag for this payload variant.
    /// Same answer as the matching
    /// [`crate::rules_core::source_content::SourceContentKind::source_slice`].
    pub fn source_slice(&self) -> &'static str {
        match self {
            SourceContentPayload::Class(_) => "SD17-B-1",
            SourceContentPayload::SpellcastingClass(_) => "SD17-B-2",
            SourceContentPayload::Race(_) | SourceContentPayload::Ability(_) => "SD17-B-3",
            SourceContentPayload::Spell(_) => "SD17-B-4",
            SourceContentPayload::Equipment(_) => "SD17-B-5",
            SourceContentPayload::Metadata(_) => "SD17-B-6",
        }
    }
}
