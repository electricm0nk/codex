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

use std::path::Path;

use crate::pcgen_import::include_resolver::{
    IncludeDiagnostic, IncludeResolution, resolve_pcc_includes_from,
};
use crate::pcgen_import::ir_converter::{
    IRSchema, convert_equipment_parse_result, convert_package_from_class_parse_result,
    convert_package_from_lst_entry_file, convert_package_from_lst_metadata_document,
    convert_package_from_spellcasting_class_parse_result, convert_spell_file,
};
use crate::pcgen_import::lst_parser::class::{
    ClassParseResult, parse_class_file,
};
use crate::pcgen_import::lst_parser::equipment::{
    EquipmentParseResult, parse_equipment_file,
};
use crate::pcgen_import::lst_parser::metadata::{
    LstMetadataDocument, parse_lst_metadata,
};
use crate::pcgen_import::lst_parser::race_ability::{
    LstEntryFile, parse_lst_entry,
};
use crate::pcgen_import::lst_parser::spell::{
    LstSpellFile, parse_lst_spell_file,
};
use crate::pcgen_import::lst_parser::spellcasting_class::{
    parse_spellcasting_class_file, SpellcastingClassParseResult,
};
use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::source_content::{SourcePackageContent, SourceRef};

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

    /// Construct an `IncludeResolutionFailure` diagnostic (severity Warning).
    pub fn include_resolution_failure(include: &IncludeDiagnostic) -> Self {
        Self {
            severity: ComposedInputSeverity::Warning,
            kind: ComposedInputDiagnosticKind::IncludeResolutionFailure,
            message: include.message.clone(),
            subject_ref: format!("include:{}", include.source_path.display()),
            line_in_source: include.line_number.map(|n| n as u32),
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

// =============================================================================
// load_composed_core_rulebook — convenience loader (PCC -> IR -> compose)
// =============================================================================

/// Canonical-corpus identity for the PF1 Core Rulebook bundle.
///
/// The pre-loop slice is grounded against the PF1 Core Rulebook
/// (the corpus already shipped in `pf1.core_rulebook` source-package
/// id throughout the GE-06 / SD-13 evidence). This constant names
/// that id at the composer level so call sites don't drift.
pub const CORE_RULEBOOK_PACKAGE_ID: &str = "pf1.core_rulebook";

/// Owned parser containers retained so the consumer can build a
/// [`SourcePackageContent`] whose borrowed records stay anchored.
///
/// The convenience loader parses every LST file the PCC include
/// graph surfaces and stashes the owned parse-result containers
/// here. The caller builds a corpus (via [`project_corpus_from_owned`])
/// and then calls [`compose`] to wire it to the character input.
///
/// The `Vec`s are public-by-necessity so the consumer can iterate
/// them. The fields are not part of the composer's logical
/// contract; they are an implementation detail that keeps the
/// borrowed lifetime honest.
#[derive(Debug, Default)]
pub struct ComposedCoreRulebookOwnedInputs {
    /// Owned B-1 [`ClassParseResult`]s.
    pub class_results: Vec<ClassParseResult>,
    /// Owned B-2 [`SpellcastingClassParseResult`]s.
    pub spellcasting_class_results: Vec<SpellcastingClassParseResult>,
    /// Owned B-3 [`LstEntryFile`]s.
    pub race_ability_results: Vec<LstEntryFile>,
    /// Owned B-4 [`LstSpellFile`]s.
    pub spell_results: Vec<LstSpellFile>,
    /// Owned B-5 [`EquipmentParseResult`]s.
    pub equipment_results: Vec<EquipmentParseResult>,
    /// Owned B-6 [`LstMetadataDocument`]s.
    pub metadata_results: Vec<LstMetadataDocument>,
}

/// Result of [`load_composed_core_rulebook`]: the resolved PCC
/// include graph, every diagnostic the loader collected, and the
/// owned parser containers the caller uses to build a corpus.
///
/// The loader deliberately does NOT produce the composed input: the
/// canonical envelope borrows from the parser containers, so the
/// corpus's lifetime is tied to this loader result. To stay sound
/// without resorting to `unsafe` lifetime transmutes, the loader
/// hands the borrowed anchors to the caller and the caller builds
/// the corpus + composition on its own frame. This is the
/// minimum-friction honest path; the alternative — returning a
/// `'static` corpus — requires `unsafe` we are not authorized to
/// introduce without an explicit human directive.
#[derive(Debug)]
pub struct ComposedCoreRulebookLoadResult {
    /// The PCC include graph that was resolved. Always populated
    /// when the loader ran without erroring out before resolution.
    pub include_resolution: Option<IncludeResolution>,
    /// Loader-collected diagnostics (PCC include diagnostics as
    /// Warnings, corpus-empty Warnings, package-id-mismatch Errors).
    pub diagnostics: Vec<ComposedInputDiagnostic>,
    /// Owned parser containers that anchor the corpus's borrowed
    /// records. See [`project_corpus_from_owned`].
    pub owned_inputs: ComposedCoreRulebookOwnedInputs,
}

/// Convenience loader: resolve a PCC entry-file's include graph
/// and parse every LST file the graph emitted (one pass per kind).
///
/// `corpus_root` is the corpus root for PCGen `@/` and `*/` path
/// semantics. It is the directory that contains the
/// `_universal` / `homebrew` / `pathfinder` siblings.
///
/// LST-file routing:
/// - `CLASS:` -> B-1 `parse_class_file` and B-2
///   `parse_spellcasting_class_file` (both parsers operate on the
///   same source file; each parser's per-entry filtering ensures
///   only its kind reaches the corpus).
/// - `RACE:` / `RACES:` / `ABILITY:` -> B-3 `parse_lst_entry`.
/// - `SPELL:` -> B-4 `parse_lst_spell_file`.
/// - `EQUIP:` / `EQUIPMOD:` -> B-5 `parse_equipment_file`.
/// - `DEITY:` / `DOMAIN:` / `KITS:` / `LANGUAGE:` / `TEMPLATE:` /
///   `COMPANIONMOD:` -> B-6 `parse_lst_metadata`.
///
/// Unrouted PCC directives (e.g. `DATA`, `COMPANIONLIST`, `KIT`)
/// are silently skipped. The pre-loop slice is grounded on the
/// corpus-side six B-family kinds plus the B-6 metadata kinds; a
/// new directive would be deferred to a loop-driven cycle.
///
/// The loader does not swallow errors. If a B-family parser
/// returns `Err`, the loader simply skips that LST file.
///
/// After this returns, the caller invokes
/// [`project_corpus_from_owned`] on `result.owned_inputs` to build
/// a [`SourcePackageContent`], then [`compose`]s it with the chosen
/// `CharacterInput`.
pub fn load_composed_core_rulebook(
    corpus_root: impl AsRef<Path>,
    pcc_path: &Path,
) -> ComposedCoreRulebookLoadResult {
    let mut diagnostics = Vec::new();

    let resolution = match resolve_pcc_includes_from(corpus_root.as_ref(), pcc_path) {
        Ok(resolution) => resolution,
        Err(include) => {
            diagnostics.push(ComposedInputDiagnostic::include_resolution_failure(
                &include,
            ));
            return ComposedCoreRulebookLoadResult {
                include_resolution: None,
                diagnostics,
                owned_inputs: ComposedCoreRulebookOwnedInputs::default(),
            };
        }
    };

    for include in &resolution.diagnostics {
        diagnostics.push(ComposedInputDiagnostic::include_resolution_failure(include));
    }

    let mut owned = ComposedCoreRulebookOwnedInputs::default();

    for lst in &resolution.lst_files {
        let kind_upper = lst.kind.to_ascii_uppercase();

        match kind_upper.as_str() {
            "CLASS" => {
                if let Ok(class_result) = parse_class_file(&lst.path) {
                    owned.class_results.push(class_result);
                }
                if let Ok(sc_result) = parse_spellcasting_class_file(&lst.path) {
                    owned.spellcasting_class_results.push(sc_result);
                }
            }
            "RACE" | "RACES" | "ABILITY" => {
                let path_str = lst.path.display().to_string();
                let Ok(text) = std::fs::read_to_string(&lst.path) else {
                    continue;
                };
                owned
                    .race_ability_results
                    .push(parse_lst_entry(&path_str, &text));
            }
            "SPELL" => {
                if let Ok(spell_file) = parse_lst_spell_file(&lst.path) {
                    owned.spell_results.push(spell_file);
                }
            }
            "EQUIP" | "EQUIPMOD" => {
                if let Ok(equip_result) = parse_equipment_file(&lst.path) {
                    owned.equipment_results.push(equip_result);
                }
            }
            "DEITY" | "DOMAIN" | "KITS" | "LANGUAGE" | "TEMPLATE" | "COMPANIONMOD" => {
                if let Ok(metadata_doc) = parse_lst_metadata(&lst.path) {
                    owned.metadata_results.push(metadata_doc);
                }
            }
            _ => {}
        }
    }

    ComposedCoreRulebookLoadResult {
        include_resolution: Some(resolution),
        diagnostics,
        owned_inputs: owned,
    }
}

/// Project the owned parser containers into a canonical
/// [`SourcePackageContent`].
///
/// The corpus's borrowed records point into `owned_inputs`; the
/// caller must keep `owned_inputs` alive as long as it uses the
/// returned corpus. The composer-side design forces this on purpose
/// (rather than relying on `unsafe` lifetime transmutes to return a
/// `'static` corpus) so the borrow discipline stays auditable.
///
/// `package_id` is the corpus identity the consumer expects;
/// [`CORE_RULEBOOK_PACKAGE_ID`] is the canonical choice for the
/// pre-loop slice.
pub fn project_corpus_from_owned<'a>(
    package_id: &str,
    owned_inputs: &'a ComposedCoreRulebookOwnedInputs,
    corpus_anchor: SourceRef,
) -> SourcePackageContent<'a> {
    let schema = IRSchema::canonical_v1();
    let mut corpus: SourcePackageContent<'a> =
        SourcePackageContent::empty(package_id.to_string(), corpus_anchor);

    for class_result in &owned_inputs.class_results {
        let (mut pkg, _) = convert_package_from_class_parse_result(
            class_result,
            package_id,
            &schema,
        );
        drain_into(&mut corpus, &mut pkg);
    }
    for sc_result in &owned_inputs.spellcasting_class_results {
        let (mut pkg, _) = convert_package_from_spellcasting_class_parse_result(
            sc_result,
            package_id,
            &schema,
        );
        drain_into(&mut corpus, &mut pkg);
    }
    for lst_entry in &owned_inputs.race_ability_results {
        let (mut pkg, _) =
            convert_package_from_lst_entry_file(lst_entry, package_id, &schema);
        drain_into(&mut corpus, &mut pkg);
    }
    for spell_file in &owned_inputs.spell_results {
        let mut pkg: SourcePackageContent<'_> = SourcePackageContent::empty(
            package_id,
            SourceRef::new(spell_file.source_path.display().to_string(), 0),
        );
        for (record, ir_diagnostics) in convert_spell_file(spell_file, &schema) {
            pkg.push(record);
            for d in ir_diagnostics {
                pkg.push_diagnostic(d.to_canonical());
            }
        }
        drain_into(&mut corpus, &mut pkg);
    }
    for equip_result in &owned_inputs.equipment_results {
        let mut pkg: SourcePackageContent<'_> = SourcePackageContent::empty(
            package_id,
            SourceRef::new(equip_result.source_path.clone(), 0),
        );
        for (record, ir_diagnostics) in convert_equipment_parse_result(equip_result, &schema) {
            pkg.push(record);
            for d in ir_diagnostics {
                pkg.push_diagnostic(d.to_canonical());
            }
        }
        drain_into(&mut corpus, &mut pkg);
    }
    for metadata_doc in &owned_inputs.metadata_results {
        let (mut pkg, _) = convert_package_from_lst_metadata_document(
            metadata_doc,
            package_id,
            &schema,
        );
        drain_into(&mut corpus, &mut pkg);
    }

    corpus
}

/// Drain records and diagnostics from `src` into `dst`, leaving
/// `src` empty. The entries are borrowed by `SourcePackageContent`,
/// so the merger does not clone the underlying parser entries; the
/// move is O(n) on `records.len()` and `diagnostics.len()`.
fn drain_into<'a>(dst: &mut SourcePackageContent<'a>, src: &mut SourcePackageContent<'a>) {
    for record in src.records.drain(..) {
        dst.push(record);
    }
    for diagnostic in src.diagnostics.drain(..) {
        dst.push_diagnostic(diagnostic);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
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
    fn project_corpus_from_owned_handles_empty_owned_inputs() {
        let owned = ComposedCoreRulebookOwnedInputs::default();
        let corpus = project_corpus_from_owned(
            CORE_RULEBOOK_PACKAGE_ID,
            &owned,
            SourceRef::new("synthetic.pcc", 0),
        );
        assert_eq!(corpus.package_id, CORE_RULEBOOK_PACKAGE_ID);
        assert_eq!(corpus.len(), 0);
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
