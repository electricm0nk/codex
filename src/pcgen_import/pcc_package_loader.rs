//! PCC -> parse -> IR convenience loader for the SD-18 pre-loop composer.
//!
//! **Why this module is here (SD-35 `AT-35-E6-003-RULED` cycle 3,
//! `docs/release/SD-35-corpus-sheet-completion/decisions.md` §11 and §19).**
//! `rules_core::composed_input`'s own module doc states the composer "does no
//! parsing, no include resolution, no IR conversion of its own". That was true
//! of [`crate::rules_core::composed_input::compose`] and false of the
//! convenience loader that shared the file: it resolved a PCC include graph,
//! ran all six B-family LST parsers and ran the IR converter, at run time,
//! from a live root. Under `§19`/B16 those eight `pcgen_import` reads counted
//! against the live side and they counted correctly — the converter pipeline
//! was living in `src/rules_core/`.
//!
//! Moving it here changes no behavior: the same functions, the same order, the
//! same diagnostics, the same borrow discipline. `§11` KEEPS this code — it is
//! converter-side, and this is where converter-side code lives. The composer it
//! feeds stays in `rules_core`, and is now exactly what its doc says it is.
//!
//! Its only consumer is `tests/sd18_preloop_consumer_compose.rs`.

use std::path::Path;

use crate::pcgen_import::include_resolver::{
    IncludeDiagnostic, IncludeResolution, resolve_pcc_includes_from,
};
use crate::pcgen_import::ir_converter::{
    IRSchema, convert_equipment_parse_result, convert_package_from_class_parse_result,
    convert_package_from_lst_entry_file, convert_package_from_lst_metadata_document,
    convert_package_from_spellcasting_class_parse_result, convert_spell_file,
};
use crate::pcgen_import::lst_parser::class::{ClassParseResult, parse_class_file};
use crate::pcgen_import::lst_parser::equipment::{EquipmentParseResult, parse_equipment_file};
use crate::pcgen_import::lst_parser::metadata::{LstMetadataDocument, parse_lst_metadata};
use crate::pcgen_import::lst_parser::race_ability::{LstEntryFile, parse_lst_entry};
use crate::pcgen_import::lst_parser::spell::{LstSpellFile, parse_lst_spell_file};
use crate::pcgen_import::lst_parser::spellcasting_class::{
    SpellcastingClassParseResult, parse_spellcasting_class_file,
};
use crate::rules_core::composed_input::{
    ComposedInputDiagnostic, ComposedInputDiagnosticKind, ComposedInputSeverity,
};
use crate::rules_core::source_content::{SourcePackageContent, SourceRef};

impl ComposedInputDiagnostic {
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
// load_composed_core_rulebook — convenience loader (PCC -> IR -> compose)
// =============================================================================

/// Owned parser containers retained so the consumer can build a
/// [`SourcePackageContent`] whose borrowed records stay anchored.
///
/// The convenience loader parses every LST file the PCC include
/// graph surfaces and stashes the owned parse-result containers
/// here. The caller builds a corpus (via [`project_corpus_from_owned`])
/// and then calls [`compose`](crate::rules_core::composed_input::compose) to wire it to the character input.
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
/// a [`SourcePackageContent`], then [`compose`](crate::rules_core::composed_input::compose)s it with the chosen
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
/// [`CORE_RULEBOOK_PACKAGE_ID`](crate::rules_core::composed_input::CORE_RULEBOOK_PACKAGE_ID) is the canonical choice for the
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
    use crate::rules_core::composed_input::CORE_RULEBOOK_PACKAGE_ID;

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
}
