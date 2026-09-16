//! Consumer-side composition for SD-18 pre-loop (SD18-PRELOOP).
//!
//! This module bridges the chosen character input
//! ([`crate::rules_core::character_input::CharacterInput`]) and the
//! corpus-side records
//! ([`crate::rules_core::source_content::SourcePackageContent`])
//! into the single input that
//! [`crate::rules_core::pilot_compute::compute_pilot_base_chassis`]
//! (and its headless wrapper
//! [`crate::rules_core::pilot_compute::build_pilot_headless_receipt`])
//! evaluates.
//!
//! ## Why this lives here (not in `pcgen_import`)
//!
//! The composer is a consumer-side artifact: it takes two already-projected
//! inputs (a chosen-state and a source-IR aggregate) and produces a
//! composition the rules engine can evaluate. It does no parsing, no
//! include resolution, no IR conversion of its own. Putting it in
//! `rules_core` (rather than `pcgen_import`) keeps the import graph
//! acyclic — `rules_core` already depends on the source-IR shapes defined
//! in its own `source_content` module, and the consumer-side composition
//! adds no dependency in the other direction.
//!
//! That sentence was **not** true of this file until SD-35
//! `AT-35-E6-003-RULED` cycle 3. The PCC convenience loader
//! (`load_composed_core_rulebook` / `project_corpus_from_owned`) shared the
//! module and did all three of the things the paragraph above disclaims —
//! include resolution, LST parsing and IR conversion — from a live root.
//! It now lives where it belongs, at
//! [`crate::pcgen_import::pcc_package_loader`], unchanged in behavior.
//!
//! ## Scope (SD-18 pre-loop slice, no loop-driven cycle content)
//!
//! - The composer accepts any `CharacterInput` + any `SourcePackageContent`
//!   and produces a [`ComposedCharacterInput`] on the happy path.
//! - Empty `SourcePackageContent` is **not** an error: the slice body
//!   states the composer must accept any corpus shape. An empty corpus
//!   emits a [`ComposedInputDiagnostic`] (severity Warning) so the caller
//!   can surface the gap without the engine refusing to evaluate.
//! - The composer performs no derivation, no rule evaluation, and no
//!   value-grammar interpretation. It is a transport-and-shape carrier.
//!   Its single behavior beyond the type-level union is the cross-input
//!   `source_package_id` consistency check and the empty-corpus warning.
//!
//! ## Authoritative references
//!
//! - `programs/codex/requirements/SD-18-core-rules-breadth/decisions.md`
//!   §6 (pre-loop vs. loop lane split; this slice is pre-loop bridge-only).
//! - `programs/codex/requirements/SD-18-core-rules-breadth/technical-design.md`
//!   §1.1 (pre-loop surface; concrete function pointers).
//! - `programs/codex/doctrine/support-state-vocabulary.md` (severity
//!   taxonomy the diagnostic conforms to).

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::source_content::SourcePackageContent;

// =============================================================================
// ComposedCharacterInput — the consumer-side input union
// =============================================================================

/// Consumer-side composed input: a chosen character input plus the
/// corpus-side records the rules engine will evaluate against.
///
/// The struct owns both inputs directly. The borrowed lifetime on
/// [`SourcePackageContent`] (`'a`) follows the canonical source-IR
/// envelope's lifetime parameter; consumers that need to detach the
/// corpus from the loader frame should clone the records they need
/// (see [`crate::rules_core::source_content::SourcePackageContent::records_by_kind`]).
#[derive(Debug, Clone)]
pub struct ComposedCharacterInput<'a> {
    /// The chosen character input (what the user picked).
    pub character_input: CharacterInput,
    /// The corpus-side aggregate (what the PCGen corpus says is available).
    pub corpus: SourcePackageContent<'a>,
}

// =============================================================================
// ComposedInputDiagnostic — the composer's diagnostic surface
// =============================================================================

/// Severity classification for [`ComposedInputDiagnostic`].
///
/// Mirrors the source-IR severity taxonomy
/// ([`crate::rules_core::source_content::SourceContentSeverity`]) so
/// downstream tooling can present both surfaces through one renderer.
/// The composer only ever emits `Error` or `Warning`; `Info` is not
/// produced because every composer-originated signal is either a
/// consumer-side block or a corpus-shape gap (a gap is a Warning, not
/// an informational note).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComposedInputSeverity {
    /// The composition could not be produced; the consumer MUST treat
    /// the input as absent.
    Error,
    /// The composition was produced but with a gap the consumer should
    /// surface (e.g. an empty corpus).
    Warning,
}

/// Diagnostic kind tag for [`ComposedInputDiagnostic`].
///
/// The composer emits three diagnostic kinds:
///
/// - [`ComposedInputDiagnosticKind::PackageIdMismatch`] — Error. The
///   `CharacterInput::source_package_id` does not equal the
///   `SourcePackageContent::package_id`; refusing to compose across
///   different corpora is the only safe behavior.
/// - [`ComposedInputDiagnosticKind::EmptyCorpus`] — Warning. The corpus
///   has zero records; the consumer may still evaluate (it produces
///   the deterministic-pilot path the rules engine already supports),
///   but should surface the gap.
/// - [`ComposedInputDiagnosticKind::IncludeResolutionFailure`] —
///   Warning. The PCC include resolver returned one or more diagnostics
///   during the load convenience path; the consumer may continue but
///   should surface them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComposedInputDiagnosticKind {
    /// `character_input.source_package_id` ≠ `corpus.package_id`.
    PackageIdMismatch,
    /// The corpus is empty (no records projected).
    EmptyCorpus,
    /// The PCC include resolver reported diagnostics during the
    /// convenience load path.
    IncludeResolutionFailure,
}

/// One diagnostic emitted by the composer.
///
/// The shape mirrors
/// [`crate::rules_core::character_input::CharacterInputDiagnostic`]
/// (severity / kind / message / subject) but adds a structured
/// `line_in_source: Option<u32>` field so diagnostics that originated
/// inside the corpus loader can carry their provenance. For diagnostics
/// that don't have a source line (e.g. container-wide composer
/// signals), `line_in_source` is `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposedInputDiagnostic {
    /// Severity classification.
    pub severity: ComposedInputSeverity,
    /// Diagnostic kind tag.
    pub kind: ComposedInputDiagnosticKind,
    /// Human-readable explanation.
    pub message: String,
    /// Logical subject of the diagnostic (e.g. the package id, a kind
    /// tag, an include path). Empty when the diagnostic is
    /// composer-global.
    pub subject_ref: String,
    /// Provenance line in the originating source when applicable.
    pub line_in_source: Option<u32>,
}

impl ComposedInputDiagnostic {
    /// Construct a `PackageIdMismatch` diagnostic (severity Error).
    pub fn package_id_mismatch(character_input_package_id: &str, corpus_package_id: &str) -> Self {
        Self {
            severity: ComposedInputSeverity::Error,
            kind: ComposedInputDiagnosticKind::PackageIdMismatch,
            message: format!(
                "character_input.source_package_id '{character_input_package_id}' does not match \
                 corpus.package_id '{corpus_package_id}'"
            ),
            subject_ref: format!("character_input.source_package_id:{character_input_package_id}"),
            line_in_source: None,
        }
    }

    /// Construct an `EmptyCorpus` diagnostic (severity Warning).
    pub fn empty_corpus(package_id: &str) -> Self {
        Self {
            severity: ComposedInputSeverity::Warning,
            kind: ComposedInputDiagnosticKind::EmptyCorpus,
            message: format!(
                "corpus for package_id '{package_id}' is empty; \
                 consumer should surface this gap"
            ),
            subject_ref: format!("corpus.package_id:{package_id}"),
            line_in_source: None,
        }
    }
}

// =============================================================================
// compose — the bounded composer entry point
// =============================================================================

/// Result of [`compose`]: a [`ComposedCharacterInput`] on the happy path,
/// plus any non-fatal diagnostics the composer collected.
///
/// When `composed` is `Some`, the consumer may evaluate. When
/// `composed` is `None`, at least one `Error`-severity diagnostic is
/// present in `diagnostics` and the consumer MUST treat the input as
/// absent. Warning-severity diagnostics are advisory; the consumer may
/// surface them but evaluation is permitted.
#[derive(Debug, Clone)]
pub struct ComposedInputLoadResult<'a> {
    /// The composed input, when composition succeeded.
    pub composed: Option<ComposedCharacterInput<'a>>,
    /// Composer-collected diagnostics (Error + Warning).
    pub diagnostics: Vec<ComposedInputDiagnostic>,
}

impl<'a> ComposedInputLoadResult<'a> {
    /// Returns true when no Error-severity diagnostics are present.
    pub fn is_ok(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|d| d.severity == ComposedInputSeverity::Error)
    }
}

/// Compose a chosen [`CharacterInput`] with a corpus-side
/// [`SourcePackageContent`] into the input the rules engine evaluates.
///
/// The composer performs three checks:
///
/// 1. `character_input.source_package_id` must equal
///    `corpus.package_id`; otherwise it returns an Error-severity
///    [`ComposedInputDiagnostic`] and no composition.
/// 2. An empty corpus produces a Warning-severity
///    [`ComposedInputDiagnostic`] but the composition is still
///    returned (the rules engine can still evaluate the
///    deterministic-pilot path with seeded defaults; the gap is
///    surfaced, not fatal).
/// 3. Otherwise the composition succeeds with no diagnostics.
///
/// The composer never panics, never invents data, never re-projects
/// the corpus. It is a transport-and-shape carrier.
pub fn compose(
    character_input: CharacterInput,
    corpus: SourcePackageContent<'_>,
) -> ComposedInputLoadResult<'_> {
    let mut diagnostics = Vec::new();

    if character_input.source_package_id != corpus.package_id {
        diagnostics.push(ComposedInputDiagnostic::package_id_mismatch(
            &character_input.source_package_id,
            &corpus.package_id,
        ));
        return ComposedInputLoadResult {
            composed: None,
            diagnostics,
        };
    }

    if corpus.is_empty() {
        diagnostics.push(ComposedInputDiagnostic::empty_corpus(&corpus.package_id));
    }

    ComposedInputLoadResult {
        composed: Some(ComposedCharacterInput {
            character_input,
            corpus,
        }),
        diagnostics,
    }
}


/// Canonical-corpus identity for the PF1 Core Rulebook bundle.
///
/// The pre-loop slice is grounded against the PF1 Core Rulebook
/// (the corpus already shipped in `pf1.core_rulebook` source-package
/// id throughout the GE-06 / SD-13 evidence). This constant names
/// that id at the composer level so call sites don't drift.
pub const CORE_RULEBOOK_PACKAGE_ID: &str = "pf1.core_rulebook";

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::source_content::SourceRef;
    use crate::rules_core::character_input::{
        ActiveState, CharacterClassLevel, ChosenCharacterState, EquipmentSelection,
    };

    fn empty_input(package_id: &str) -> CharacterInput {
        CharacterInput {
            case_id: None,
            source_package_id: package_id.to_string(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_string(),
                class_levels: vec![CharacterClassLevel {
                    class_id: "class:fighter".to_string(),
                    level: 1,
                }],
                ability_scores: Default::default(),
                selected_feats: Vec::new(),
                skill_allocations: Vec::new(),
                equipment_selections: vec![EquipmentSelection {
                    item_id: "item:longsword".to_string(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                    applied_modifiers: Vec::new(),
                }],
                selected_choices: Vec::new(),
                selected_traits: Vec::new(),
                spells_selected: Vec::new(),
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    fn empty_corpus(package_id: &str) -> SourcePackageContent<'static> {
        SourcePackageContent::empty(
            package_id.to_string(),
            SourceRef::new("synthetic.pcc", 0),
        )
    }

    #[test]
    fn compose_happy_path_returns_composed_input() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.core_rulebook");
        let result = compose(input.clone(), corpus);

        assert!(result.is_ok(), "happy path must produce an OK result");
        let composed = result
            .composed
            .expect("composed must be Some on the happy path");
        assert_eq!(
            composed.character_input.source_package_id,
            "pf1.core_rulebook"
        );
        assert_eq!(composed.corpus.package_id, "pf1.core_rulebook");
        // The corpus is empty, so the EmptyCorpus warning is the
        // sole diagnostic; the happy-path assertion is `is_ok()`
        // (no Error-severity diagnostics) rather than
        // `diagnostics.is_empty()`. See
        // `compose_empty_corpus_emits_warning_but_returns_composed`
        // for the explicit empty-corpus-warning assertion.
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.severity == ComposedInputSeverity::Error),
            "happy path must produce no Error-severity diagnostics: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn compose_empty_corpus_emits_warning_but_returns_composed() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.core_rulebook");
        let result = compose(input, corpus);

        assert!(
            result.is_ok(),
            "empty corpus is a Warning, not an Error; the composition must still be returned"
        );
        let composed = result
            .composed
            .expect("composed must be Some even with an empty corpus");
        assert_eq!(composed.corpus.package_id, "pf1.core_rulebook");
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0].kind,
            ComposedInputDiagnosticKind::EmptyCorpus
        );
        assert_eq!(
            result.diagnostics[0].severity,
            ComposedInputSeverity::Warning
        );
    }

    #[test]
    fn compose_package_id_mismatch_returns_error_and_no_composed() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.ultimate_combat");
        let result = compose(input, corpus);

        assert!(!result.is_ok(), "package_id mismatch must be an Error");
        assert!(
            result.composed.is_none(),
            "package_id mismatch must NOT produce a composed input"
        );
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0].kind,
            ComposedInputDiagnosticKind::PackageIdMismatch
        );
        assert_eq!(result.diagnostics[0].severity, ComposedInputSeverity::Error);
        assert!(
            result.diagnostics[0].message.contains("pf1.core_rulebook"),
            "the diagnostic message must name the character_input package id"
        );
        assert!(
            result.diagnostics[0]
                .message
                .contains("pf1.ultimate_combat"),
            "the diagnostic message must name the corpus package id"
        );
    }

    #[test]
    fn compose_corpus_len_zero_on_empty_corpus() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.core_rulebook");
        let result = compose(input, corpus);
        assert!(result.is_ok());
        let composed = result.composed.expect("composed");
        assert_eq!(composed.corpus.len(), 0);
        assert!(composed.corpus.is_empty());
    }

    #[test]
    fn compose_does_not_clobber_or_reorder_diagnostics_on_empty_corpus() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.core_rulebook");
        let result = compose(input, corpus);
        // The empty-corpus warning must be the only diagnostic and
        // must come first (so consumers can short-circuit on it).
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(
            result.diagnostics[0].kind,
            ComposedInputDiagnosticKind::EmptyCorpus
        );
    }

    #[test]
    fn package_id_mismatch_diagnostic_message_is_actionable() {
        let d = ComposedInputDiagnostic::package_id_mismatch("pf1.core_rulebook", "dnd35e");
        assert!(d.message.contains("pf1.core_rulebook"));
        assert!(d.message.contains("dnd35e"));
        assert!(d.subject_ref.contains("pf1.core_rulebook"));
        assert_eq!(d.severity, ComposedInputSeverity::Error);
        assert_eq!(d.line_in_source, None);
    }

    #[test]
    fn empty_corpus_diagnostic_subject_ref_names_package_id() {
        let d = ComposedInputDiagnostic::empty_corpus("pf1.core_rulebook");
        assert!(d.subject_ref.contains("pf1.core_rulebook"));
        assert_eq!(d.severity, ComposedInputSeverity::Warning);
        assert_eq!(d.line_in_source, None);
    }


    #[test]
    fn core_rulebook_package_id_constant_is_correct() {
        assert_eq!(CORE_RULEBOOK_PACKAGE_ID, "pf1.core_rulebook");
    }

    #[test]
    fn composed_character_input_carries_both_inputs() {
        let input = empty_input("pf1.core_rulebook");
        let corpus: SourcePackageContent<'static> = empty_corpus("pf1.core_rulebook");
        let result = compose(input, corpus);
        let composed = result.composed.expect("composed");
        // Both fields must be accessible: chosen-state via
        // `character_input`, corpus via `corpus`.
        assert_eq!(composed.character_input.chosen.race_id, "race:human");
        assert_eq!(composed.corpus.package_id, "pf1.core_rulebook");
    }
}
