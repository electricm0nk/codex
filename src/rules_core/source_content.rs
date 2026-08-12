//! Canonical source-IR record shape (SD-17 Slice E).
//!
//! This module defines the corpus-rooted source-IR envelope that
//! the rules-core engine eventually consumes. The chosen character
//! input shape (what a user has picked at character creation)
//! lives in [`crate::rules_core::character_input`]; this module is
//! its sibling and represents **what the PCGen corpus says is
//! available**, with full provenance back to the LST file and line.
//!
//! # Authoritative contract
//!
//! The authoritative specification is
//! `docs/release/SD-17/artifacts/canonical-source-ir-contract-2026-07-12.md`.
//! Every field, lossy mapping, and intentional non-translation
//! listed in that document is reflected here. If this
//! implementation and the contract disagree, the contract wins;
//! the implementation is repaired.
//!
//! # Scope boundary
//!
//! - This module does NOT modify the chosen-state shape
//!   ([`crate::rules_core::character_input`]). That integration is
//!   a tranche-3 deliverable.
//! - This module does NOT compute any derived value, evaluate any
//!   rule, or interpret any value grammar beyond what the Slice B
//!   parsers already captured.
//! - This module does NOT consume the source-IR into the rules
//!   engine. It defines the canonical shape and the diagnostic
//!   surface; the rules engine composition is owned by the active
//!   tranche that takes it up.
//!
//! # Module-partition rationale
//!
//! The typed payload enum [`SourceContentPayload`] lives in
//! [`crate::pcgen_import::source_content_payload`] because its
//! variants borrow Slice B parser entry types
//! (e.g. `&'a ClassEntry`). Defining the enum directly in this
//! module would force `rules_core::source_content` to pull
//! `pcgen_import::lst_parser::*` into its surface; combined with
//! the canonical projection path (which lives inside
//! `pcgen_import`), that would either form an import cycle at
//! the cargo module level or require an unsafe lifetime
//! extension. The split avoids both: the canonical envelope (this
//! module's surface) depends only on `&str`-based
//! identifiers, integers, owned strings, and the
//! [`SourceContentPayload`] re-export — none of which
//! transitively depend on parser entry types living in the
//! canonical envelope.

pub use crate::pcgen_import::source_content_payload::SourceContentPayload;

// =============================================================================
// Versioning
// =============================================================================

/// Canonical source-IR schema version.
///
/// Bumped on breaking changes to any of the public types in this
/// module. The contract artifact names `source_ir_version: u32 = 1`
/// as the initial version. Converters from older parser outputs
/// are expected to declare compatibility with a specific version.
pub const SOURCE_IR_VERSION: u32 = 1;

// =============================================================================
// SourceRef — canonical provenance anchor
// =============================================================================

/// Canonical provenance anchor for one record in the corpus.
///
/// Every record emitted by the source-IR projection carries a
/// `SourceRef` so the rules engine, the UI, and the diagnostics
/// surface can trace any derived value back to the LST file and
/// the exact line that declared it. The fields are deliberately
/// minimal — the parser records only what it knows; richer
/// provenance (raw line text, container context) is reachable via
/// the underlying parser entry carried in the
/// [`SourceContentPayload`] variant.
///
/// `lst_file` is the path the parser recorded on the source
/// document (string form; `PathBuf::to_string_lossy()` for
/// path-typed parsers, verbatim for string-typed ones). `line` is
/// the one-based line number the parser captured for the
/// record's first directive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceRef {
    /// Identity of the LST file the record originated from.
    pub lst_file: String,
    /// One-based line number in `lst_file` where the record's
    /// first directive appeared.
    pub line: u32,
}

impl SourceRef {
    /// Construct a `SourceRef` from the canonical pieces.
    pub fn new(lst_file: impl Into<String>, line: u32) -> Self {
        Self {
            lst_file: lst_file.into(),
            line,
        }
    }
}

// =============================================================================
// SourceContentKind — kind tag for SourceContentRecord
// =============================================================================

/// Canonical kind tag for [`SourceContentRecord`].
///
/// Mirrors the six B-family parser output kinds plus the six
/// [`MetadataKindInner`] variants from the B-6 parser. The mapping
/// is stable: future B-family kinds will add variants here and on
/// [`SourceContentPayload`] together (one per kind, in the same
/// order).
///
/// The split into [`SourceContentKind::Metadata`] (carrying a
/// [`MetadataKindInner`] inner tag) versus the rest is
/// intentional: the six metadata kinds are surfaced through a
/// single B-6 parser and share a single
/// [`SourceContentPayload::Metadata`] variant, while the six
/// B-family kinds each have their own payload variant and a
/// top-level kind tag here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceContentKind {
    /// A martial class record from B-1 (`CLASS:` for Fighter /
    /// Barbarian / Monk / Rogue / Ranger / Paladin and their
    /// `Ex-<name>` mirrors).
    Class,
    /// A spellcasting class record from B-2 (`CLASS:` for Cleric
    /// / Druid / Wizard / Sorcerer / Bard and their `Ex-<name>`
    /// mirrors).
    SpellcastingClass,
    /// A race pointer declaration from B-3 (`RACE:` /
    /// `RACES:`).
    Race,
    /// An ability declaration from B-3 (`ABILITY:`).
    Ability,
    /// A spell row record from B-4 (`SPELL:` rows).
    Spell,
    /// An equipment or equipment-modifier record from B-5
    /// (`EQUIP:` / `EQUIPMOD:` rows).
    Equipment,
    /// A metadata-kind record from B-6 (Deity / Domain / Kits /
    /// Language / Template / CompanionMod). The inner
    /// [`MetadataKindInner`] tag distinguishes which.
    Metadata(MetadataKindInner),
}

/// Inner kind tag for metadata records. Mirrors the B-6 parser's
/// [`crate::pcgen_import::lst_parser::metadata::MetadataKind`] so
/// the canonical envelope can carry it without dragging the
/// parser's enum into the rules-core surface. The mapping is
/// mechanical and total (every parser-side variant maps to
/// exactly one `MetadataKindInner` and back).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetadataKindInner {
    /// `DEITY:` directive.
    Deity,
    /// `DOMAIN:` directive.
    Domain,
    /// `KITS:` directive.
    Kits,
    /// `LANGUAGE:` directive.
    Language,
    /// `TEMPLATE:` directive.
    Template,
    /// `COMPANIONMOD:` directive.
    CompanionMod,
}

impl SourceContentKind {
    /// Canonical directive-token prefix for this kind (the
    /// capitalised token preceding the colon in the LST source,
    /// e.g. `CLASS`, `RACE`, `SPELL`).
    ///
    /// For `Equipment`, the canonical token is `EQUIP` (an alias
    /// for both `EQUIP:` and `EQUIPMOD:` source tokens); the
    /// equipment-vs-equipment-modifier distinction lives on the
    /// borrowed `EquipmentRecord::kind`.
    pub fn token(&self) -> &'static str {
        match self {
            SourceContentKind::Class => "CLASS",
            SourceContentKind::SpellcastingClass => "CLASS",
            SourceContentKind::Race => "RACE",
            SourceContentKind::Ability => "ABILITY",
            SourceContentKind::Spell => "SPELL",
            SourceContentKind::Equipment => "EQUIP",
            SourceContentKind::Metadata(inner) => inner.token(),
        }
    }

    /// The B-family slice tag this kind originated from. Returns
    /// the slice-id string for the SD17-B-* slice that authored
    /// the parser entry type this kind projects.
    pub fn source_slice(&self) -> &'static str {
        match self {
            SourceContentKind::Class => "SD17-B-1",
            SourceContentKind::SpellcastingClass => "SD17-B-2",
            SourceContentKind::Race | SourceContentKind::Ability => "SD17-B-3",
            SourceContentKind::Spell => "SD17-B-4",
            SourceContentKind::Equipment => "SD17-B-5",
            SourceContentKind::Metadata(_) => "SD17-B-6",
        }
    }
}

impl MetadataKindInner {
    /// Canonical directive-token prefix for this metadata kind.
    pub fn token(&self) -> &'static str {
        match self {
            MetadataKindInner::Deity => "DEITY",
            MetadataKindInner::Domain => "DOMAIN",
            MetadataKindInner::Kits => "KITS",
            MetadataKindInner::Language => "LANGUAGE",
            MetadataKindInner::Template => "TEMPLATE",
            MetadataKindInner::CompanionMod => "COMPANIONMOD",
        }
    }

    /// The originating B-family slice tag. Always `SD17-B-6`.
    pub fn source_slice(&self) -> &'static str {
        "SD17-B-6"
    }
}

// =============================================================================
// SourceContentRecord — per-record envelope
// =============================================================================

/// Canonical source-IR record: one record per LST directive the
/// parser recognizes, with full provenance and a kind-specific
/// payload.
///
/// The envelope is the unit the rules engine eventually consumes.
/// The record carries:
/// - a [`SourceRef`] so any value derived from the record can be
///   traced back to its LST source,
/// - a [`SourceContentKind`] tag that drives routing through the
///   rules engine's per-kind consumers, and
/// - a [`SourceContentPayload`] that holds the borrowed B-family
///   entry. The borrow is the projection: cloning the borrowed
///   entry is the consumer's decision, never the envelope's.
///
/// Records are constructed via the canonical projection path
/// (see [`crate::pcgen_import::source_content_payload`]'s
/// `SourceContentRecord`-typed projection helpers, and the
/// slice-E contract artifact). Consumers should treat the type as
/// the output of that projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContentRecord<'a> {
    /// Provenance anchor. Every record carries one.
    pub source_ref: SourceRef,
    /// Kind tag. Drives per-kind routing in the rules engine.
    pub kind: SourceContentKind,
    /// Kind-specific payload. Borrowed from the B-family parser's
    /// entry; the borrow is the canonical projection.
    pub payload: SourceContentPayload<'a>,
}

impl<'a> SourceContentRecord<'a> {
    /// Construct an arbitrary canonical envelope from its three
    /// constituent parts. Most callers should use the
    /// kind-specific constructors
    /// ([`SourceContentRecord::class`],
    /// [`SourceContentRecord::spellcasting_class`],
    /// [`SourceContentRecord::race`],
    /// [`SourceContentRecord::ability`],
    /// [`SourceContentRecord::spell`],
    /// [`SourceContentRecord::equipment`],
    /// [`SourceContentRecord::metadata`]) or the projection
    /// path in
    /// [`crate::pcgen_import::source_content_payload`].
    pub fn new(
        source_ref: SourceRef,
        kind: SourceContentKind,
        payload: SourceContentPayload<'a>,
    ) -> Self {
        Self {
            source_ref,
            kind,
            payload,
        }
    }
}

// =============================================================================
// SourceContentDiagnostic — diagnostic surface for source-IR projection
// =============================================================================

/// Severity classification for [`SourceContentDiagnostic`].
///
/// Mirrors the chosen-state
/// [`crate::rules_core::character_input::DiagnosticSeverity`]
/// shape and adds an explicit `Warning` variant for lossy/partial
/// mappings the projection surfaces without rejecting the record.
/// The rules engine uses severity to decide whether the
/// diagnostic blocks evaluation or merely annotates the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceContentSeverity {
    /// The record could not be projected; the consumer MUST treat
    /// the record as absent.
    Error,
    /// The record was projected but a portion of the source
    /// content was preserved lossily (e.g. a raw token string
    /// rather than a structured form). The consumer MAY treat
    /// the record as partial.
    Warning,
    /// Informational note attached to a projected record.
    Info,
}

/// Classification of the diagnostic kind for source-IR projection.
///
/// Mirrors the converter-side diagnostic vocabulary but operates
/// on the source-IR side: every variant names a defect class the
/// projection itself introduces (lossy mapping, unsupported
/// token, partial translation) or inherits from the upstream
/// parser (malformed record forwarded from a B-family parse
/// result).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceContentDiagnosticKind {
    /// The parser forwarded a malformed-record diagnostic. The
    /// record's underlying parser entry carries the originating
    /// line and kind; the projected record MAY still be usable.
    MalformedRecord,
    /// A portion of the source content was preserved as a raw
    /// token string rather than a structured form (per the
    /// contract artifact's lossy-mapping inventory). The token's
    /// [`SourceRef`] is on the diagnostic.
    LossyMapping,
    /// A directive or value token the corpus supports but the
    /// source-IR does not currently recognize. Carries the
    /// token's [`SourceRef`] for downstream inspection.
    UnsupportedToken,
    /// A source record projected partially: known fields are
    /// populated, unknown fields are preserved as raw evidence on
    /// the underlying entry but the canonical shape does not
    /// surface them.
    PartialTranslation,
}

/// Canonical diagnostic surfaced by source-IR projection.
///
/// Mirrors
/// [`crate::rules_core::character_input::CharacterInputDiagnostic`]
/// but for source-side issues. The struct deliberately uses the
/// same field names (severity / kind / message / source_ref) so
/// future tooling can share presentation logic between
/// chosen-state and source-side diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContentDiagnostic {
    /// Severity classification.
    pub severity: SourceContentSeverity,
    /// Diagnostic kind tag.
    pub kind: SourceContentDiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
    /// Provenance anchor for the offending record or token.
    pub source_ref: SourceRef,
}

impl SourceContentDiagnostic {
    /// Construct a diagnostic with the canonical envelope shape.
    pub fn new(
        severity: SourceContentSeverity,
        kind: SourceContentDiagnosticKind,
        message: impl Into<String>,
        source_ref: SourceRef,
    ) -> Self {
        Self {
            severity,
            kind,
            message: message.into(),
            source_ref,
        }
    }

    /// Convenience: malformed-record diagnostic (severity Error).
    pub fn malformed(message: impl Into<String>, source_ref: SourceRef) -> Self {
        Self::new(
            SourceContentSeverity::Error,
            SourceContentDiagnosticKind::MalformedRecord,
            message,
            source_ref,
        )
    }

    /// Convenience: lossy-mapping diagnostic (severity Warning).
    pub fn lossy_mapping(message: impl Into<String>, source_ref: SourceRef) -> Self {
        Self::new(
            SourceContentSeverity::Warning,
            SourceContentDiagnosticKind::LossyMapping,
            message,
            source_ref,
        )
    }

    /// Convenience: unsupported-token diagnostic (severity Warning).
    pub fn unsupported_token(message: impl Into<String>, source_ref: SourceRef) -> Self {
        Self::new(
            SourceContentSeverity::Warning,
            SourceContentDiagnosticKind::UnsupportedToken,
            message,
            source_ref,
        )
    }

    /// Convenience: partial-translation diagnostic (severity Info).
    pub fn partial_translation(message: impl Into<String>, source_ref: SourceRef) -> Self {
        Self::new(
            SourceContentSeverity::Info,
            SourceContentDiagnosticKind::PartialTranslation,
            message,
            source_ref,
        )
    }
}

// =============================================================================
// SourcePackageContent — corpus-rooted aggregate
// =============================================================================

/// Canonical source-IR aggregate: the corpus-rooted bundle of
/// records produced by resolving a PCC entry file's include graph
/// and projecting every parsed LST directive into the canonical
/// envelope.
///
/// `package_id` is the corpus identity the consumer expects (e.g.
/// `pathfinder_pf1`, `dnd35e_system`). `source_ref` is the PCC
/// entry file the include graph was resolved against — the
/// canonical anchor for the whole bundle. `records` is the flat
/// list of every record the projection produced, in source
/// order. `diagnostics` is the deduplicated set of
/// projection-side diagnostics (lossy mappings,
/// malformed-record forwards, unsupported tokens).
#[derive(Debug, Clone)]
pub struct SourcePackageContent<'a> {
    /// Corpus identity (e.g. `pathfinder_pf1`).
    pub package_id: String,
    /// Provenance anchor for the PCC entry file the include
    /// graph was resolved against.
    pub source_ref: SourceRef,
    /// Projected records, in source order.
    pub records: Vec<SourceContentRecord<'a>>,
    /// Projection-side diagnostics (lossy mappings, malformed
    /// forwards, unsupported tokens).
    pub diagnostics: Vec<SourceContentDiagnostic>,
}

impl<'a> SourcePackageContent<'a> {
    /// Construct an empty `SourcePackageContent` with the
    /// canonical provenance anchors and no records. Useful for
    /// tests and for the projection's preliminary accumulator.
    pub fn empty(package_id: impl Into<String>, source_ref: SourceRef) -> Self {
        Self {
            package_id: package_id.into(),
            source_ref,
            records: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Append a record to the bundle.
    pub fn push(&mut self, record: SourceContentRecord<'a>) {
        self.records.push(record);
    }

    /// Append a diagnostic to the bundle's diagnostics vector.
    pub fn push_diagnostic(&mut self, diagnostic: SourceContentDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns the records filtered to the given kind, in
    /// deterministic order: insertion order first, then a stable
    /// secondary sort by `(lst_file, line)` so two
    /// `SourcePackageContent` values built from the same corpus
    /// produce byte-identical slices.
    ///
    /// The sort operates on a fresh `Vec` and does NOT mutate
    /// `self.records`. The result is a `Vec` (not a slice) so
    /// callers can pass it across the borrow boundary without
    /// lifetime gymnastics.
    pub fn records_by_kind(&self, kind: SourceContentKind) -> Vec<SourceContentRecord<'a>> {
        let mut filtered: Vec<SourceContentRecord<'a>> = self
            .records
            .iter()
            .filter(|r| kinds_match(r.kind, kind))
            .cloned()
            .collect();
        // Secondary sort by provenance. The slice already
        // preserves insertion order; we sort a parallel index
        // vector by (lst_file, line) and permute the result to
        // keep O(n log n) overall.
        let mut indices: Vec<usize> = (0..filtered.len()).collect();
        indices.sort_by(|&a, &b| {
            let ra = &filtered[a].source_ref;
            let rb = &filtered[b].source_ref;
            (ra.lst_file.as_str(), ra.line).cmp(&(rb.lst_file.as_str(), rb.line))
        });
        let permuted: Vec<SourceContentRecord<'a>> =
            indices.into_iter().map(|i| filtered[i].clone()).collect();
        filtered = permuted;
        filtered
    }

    /// Returns the number of records in the bundle.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true when the bundle contains no records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

/// Equality on `SourceContentKind` that treats
/// `Metadata(MetadataKindInner::Deity)` as equal to
/// `Metadata(MetadataKindInner::Deity)` and to no other kind,
/// and distinguishes non-equal metadata inner tags.
fn kinds_match(a: SourceContentKind, b: SourceContentKind) -> bool {
    match (a, b) {
        (SourceContentKind::Class, SourceContentKind::Class) => true,
        (SourceContentKind::SpellcastingClass, SourceContentKind::SpellcastingClass) => true,
        (SourceContentKind::Race, SourceContentKind::Race) => true,
        (SourceContentKind::Ability, SourceContentKind::Ability) => true,
        (SourceContentKind::Spell, SourceContentKind::Spell) => true,
        (SourceContentKind::Equipment, SourceContentKind::Equipment) => true,
        (SourceContentKind::Metadata(x), SourceContentKind::Metadata(y)) => x == y,
        _ => false,
    }
}

// =============================================================================
// SourceContentLoadResult — top-level load result
// =============================================================================

/// Top-level load result: a [`SourcePackageContent`] (when
/// projection succeeded) plus the projection's diagnostics
/// vector.
///
/// `content` is `None` iff the projection produced a
/// source-IR-blocking error. When `content` is `Some`, every
/// record is usable; the diagnostics vector still carries the
/// projection's warnings (lossy mappings, partial translations,
/// malformed-record forwards) so the consumer can surface them.
#[derive(Debug, Clone)]
pub struct SourceContentLoadResult<'a> {
    /// The projected corpus, when projection produced a usable
    /// bundle.
    pub content: Option<SourcePackageContent<'a>>,
    /// All projection-side diagnostics (errors, warnings, infos).
    pub diagnostics: Vec<SourceContentDiagnostic>,
}

impl<'a> SourceContentLoadResult<'a> {
    /// Construct an empty `SourceContentLoadResult` with no
    /// content and no diagnostics.
    pub fn empty() -> Self {
        Self {
            content: None,
            diagnostics: Vec::new(),
        }
    }

    /// Construct a successful load result wrapping the supplied
    /// package content.
    pub fn ok(content: SourcePackageContent<'a>) -> Self {
        Self {
            content: Some(content),
            diagnostics: Vec::new(),
        }
    }

    /// Construct a failed load result carrying the supplied
    /// diagnostics.
    pub fn err(diagnostics: Vec<SourceContentDiagnostic>) -> Self {
        Self {
            content: None,
            diagnostics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn src_ref_round_trip() {
        let sr = SourceRef::new("foo.lst", 42);
        assert_eq!(sr.lst_file, "foo.lst");
        assert_eq!(sr.line, 42);
    }

    #[test]
    fn src_ref_equality() {
        let a = SourceRef::new("foo.lst", 1);
        let b = SourceRef::new("foo.lst", 1);
        let c = SourceRef::new("bar.lst", 1);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn src_ref_hash_distinct() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SourceRef::new("foo.lst", 1));
        set.insert(SourceRef::new("foo.lst", 2));
        set.insert(SourceRef::new("bar.lst", 1));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn kind_token_covers_all_variants() {
        assert_eq!(SourceContentKind::Class.token(), "CLASS");
        assert_eq!(SourceContentKind::SpellcastingClass.token(), "CLASS");
        assert_eq!(SourceContentKind::Race.token(), "RACE");
        assert_eq!(SourceContentKind::Ability.token(), "ABILITY");
        assert_eq!(SourceContentKind::Spell.token(), "SPELL");
        assert_eq!(SourceContentKind::Equipment.token(), "EQUIP");
        assert_eq!(
            SourceContentKind::Metadata(MetadataKindInner::Deity).token(),
            "DEITY"
        );
        assert_eq!(
            SourceContentKind::Metadata(MetadataKindInner::CompanionMod).token(),
            "COMPANIONMOD"
        );
    }

    #[test]
    fn kind_source_slice_covers_all_variants() {
        assert_eq!(SourceContentKind::Class.source_slice(), "SD17-B-1");
        assert_eq!(
            SourceContentKind::SpellcastingClass.source_slice(),
            "SD17-B-2"
        );
        assert_eq!(SourceContentKind::Race.source_slice(), "SD17-B-3");
        assert_eq!(SourceContentKind::Ability.source_slice(), "SD17-B-3");
        assert_eq!(SourceContentKind::Spell.source_slice(), "SD17-B-4");
        assert_eq!(SourceContentKind::Equipment.source_slice(), "SD17-B-5");
        assert_eq!(
            SourceContentKind::Metadata(MetadataKindInner::Domain).source_slice(),
            "SD17-B-6"
        );
    }

    #[test]
    fn metadata_kind_inner_round_trip_tokens() {
        for inner in [
            MetadataKindInner::Deity,
            MetadataKindInner::Domain,
            MetadataKindInner::Kits,
            MetadataKindInner::Language,
            MetadataKindInner::Template,
            MetadataKindInner::CompanionMod,
        ] {
            assert_eq!(inner.source_slice(), "SD17-B-6");
        }
    }

    #[test]
    fn kinds_match_distinguishes_metadata_inner_tags() {
        assert!(kinds_match(
            SourceContentKind::Metadata(MetadataKindInner::Deity),
            SourceContentKind::Metadata(MetadataKindInner::Deity),
        ));
        assert!(!kinds_match(
            SourceContentKind::Metadata(MetadataKindInner::Deity),
            SourceContentKind::Metadata(MetadataKindInner::Domain),
        ));
        assert!(!kinds_match(
            SourceContentKind::Metadata(MetadataKindInner::Deity),
            SourceContentKind::Race,
        ));
        assert!(!kinds_match(
            SourceContentKind::Class,
            SourceContentKind::SpellcastingClass,
        ));
        assert!(kinds_match(
            SourceContentKind::Class,
            SourceContentKind::Class,
        ));
    }

    #[test]
    fn diagnostic_constructors_set_correct_severity_kind() {
        let sr = SourceRef::new("foo.lst", 1);

        let m = SourceContentDiagnostic::malformed("oops", sr.clone());
        assert!(matches!(m.severity, SourceContentSeverity::Error));
        assert!(matches!(
            m.kind,
            SourceContentDiagnosticKind::MalformedRecord
        ));

        let l = SourceContentDiagnostic::lossy_mapping("token x", sr.clone());
        assert!(matches!(l.severity, SourceContentSeverity::Warning));
        assert!(matches!(l.kind, SourceContentDiagnosticKind::LossyMapping));

        let u = SourceContentDiagnostic::unsupported_token("ZKEY:x", sr.clone());
        assert!(matches!(u.severity, SourceContentSeverity::Warning));
        assert!(matches!(
            u.kind,
            SourceContentDiagnosticKind::UnsupportedToken
        ));

        let p = SourceContentDiagnostic::partial_translation("partial", sr);
        assert!(matches!(p.severity, SourceContentSeverity::Info));
        assert!(matches!(
            p.kind,
            SourceContentDiagnosticKind::PartialTranslation
        ));
    }

    #[test]
    fn package_content_filter_by_kind_returns_canonical() {
        let sr = SourceRef::new("foo.lst", 1);
        let pkg = SourcePackageContent::empty("test_pkg", sr.clone());
        // We can't construct SourceContentRecord here without the
        // parser entries, but we can verify the empty package
        // behaves correctly.
        assert!(pkg.is_empty());
        assert_eq!(pkg.len(), 0);

        let class_filter: Vec<SourceContentRecord> = pkg.records_by_kind(SourceContentKind::Class);
        assert!(class_filter.is_empty());
    }

    #[test]
    fn empty_load_result_has_no_content() {
        let r: SourceContentLoadResult = SourceContentLoadResult::empty();
        assert!(r.content.is_none());
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn ok_load_result_carries_content() {
        let sr = SourceRef::new("foo.lst", 1);
        let pkg = SourcePackageContent::empty("test_pkg", sr);
        let r = SourceContentLoadResult::ok(pkg);
        assert!(r.content.is_some());
    }

    #[test]
    fn err_load_result_carries_only_diagnostics() {
        let sr = SourceRef::new("foo.lst", 1);
        let diag = SourceContentDiagnostic::malformed("oops", sr);
        let r: SourceContentLoadResult = SourceContentLoadResult::err(vec![diag]);
        assert!(r.content.is_none());
        assert_eq!(r.diagnostics.len(), 1);
    }
}
