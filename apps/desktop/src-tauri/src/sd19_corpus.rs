//! SD-19 UI-surfacing: bundled corpus-fixture loader.
//!
//! The desktop app has no mechanism to ship or point at the full PCGen
//! corpus, so this loads a small, bounded set of real corpus records —
//! copied verbatim, the same fixture-authoring convention as
//! `tests/fixtures/rules_core/sd19_seam_crb_*.txt` — bundled as a Tauri
//! resource (`resources/sd19_corpus_fixtures/`, see `tauri.conf.json`).
//! This is enough to prove `compute_pilot_with_corpus` resolves real
//! corpus data end-to-end in the live UI; it is not a general corpus
//! provider. Exhaustive corpus coverage is out of scope here (see
//! `~/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md`
//! §2.4/§2.5 — that coverage is already proven by the loop's own
//! `CORPUS_ROOT`-gated tests against the real corpus, not by this bundle).
//!
//! The `SourcePackageContent` is built once, lazily, and cached for the
//! process lifetime — the underlying parsed records are leaked to get a
//! `'static` borrow, which is fine for a fixed, ~4-record bundle built at
//! most once per process.

use std::path::PathBuf;
use std::sync::OnceLock;

use codex::pcgen_import::ir_converter::{convert_equipment_record, convert_spell_record};
use codex::pcgen_import::lst_parser::equipment::parse_equipment_entries;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

use crate::ge08_workbench::resolve_package_path;

const FIXTURE_RESOURCE_ROOT: &str = "resources/sd19_corpus_fixtures";
const SPELL_FIXTURES: &[&str] = &["spell_abjuration.txt", "spell_illusion.txt"];
const EQUIPMENT_FIXTURES: &[&str] = &["equip_longsword.txt", "equip_chain_shirt.txt"];

/// Strips the fixture's leading `# Fixture for ...` comment line, matching
/// the convention `tests/sd19_seam_shapes_correctness.rs` established.
fn record_line(fixture_text: &str) -> Option<&str> {
    fixture_text
        .lines()
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
}

fn fixture_dir() -> Result<PathBuf, String> {
    resolve_package_path(FIXTURE_RESOURCE_ROOT)
}

fn read_fixture(dir: &std::path::Path, name: &str) -> Result<String, String> {
    std::fs::read_to_string(dir.join(name))
        .map_err(|err| format!("could not read bundled corpus fixture '{name}': {err}"))
}

fn build_corpus_fixture_bundle() -> SourcePackageContent<'static> {
    let dir = fixture_dir().expect("sd19_corpus_fixtures resource directory must resolve");

    let source_ref = SourceRef {
        lst_file: "sd19_corpus_fixtures".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_desktop_ui_fixtures", source_ref);

    for name in SPELL_FIXTURES {
        let text = read_fixture(&dir, name).expect("bundled spell fixture must be readable");
        let line = record_line(&text)
            .unwrap_or_else(|| panic!("spell fixture '{name}' has no record line"));
        let parsed = parse_lst_spell_row("sd19_corpus_fixtures", 1, line);
        let record = parsed
            .record
            .unwrap_or_else(|| panic!("spell fixture '{name}' failed to parse: {line}"));
        let record: &'static codex::pcgen_import::lst_parser::spell::LstSpellRecord =
            Box::leak(Box::new(record));
        corpus.push(convert_spell_record(record));
    }

    for name in EQUIPMENT_FIXTURES {
        let text = read_fixture(&dir, name).expect("bundled equipment fixture must be readable");
        let line = record_line(&text)
            .unwrap_or_else(|| panic!("equipment fixture '{name}' has no record line"));
        let result = parse_equipment_entries("sd19_corpus_fixtures", line);
        for entry in result.entries {
            let entry: &'static codex::pcgen_import::lst_parser::equipment::EquipmentRecord =
                Box::leak(Box::new(entry));
            corpus.push(convert_equipment_record(entry));
        }
    }

    corpus
}

/// The bundled corpus fixture set, built once and cached for the process
/// lifetime.
pub fn corpus_fixture_bundle() -> &'static SourcePackageContent<'static> {
    static BUNDLE: OnceLock<SourcePackageContent<'static>> = OnceLock::new();
    BUNDLE.get_or_init(build_corpus_fixture_bundle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::source_content::SourceContentKind;

    #[test]
    fn bundled_fixture_directory_resolves_and_contains_expected_files() {
        let dir = fixture_dir().expect("fixture dir must resolve in a source checkout");
        for name in SPELL_FIXTURES.iter().chain(EQUIPMENT_FIXTURES.iter()) {
            assert!(
                dir.join(name).is_file(),
                "expected bundled fixture '{name}' at {}",
                dir.join(name).display()
            );
        }
    }

    #[test]
    fn corpus_fixture_bundle_has_two_spells_and_two_equipment_records() {
        let corpus = corpus_fixture_bundle();
        assert_eq!(
            corpus.records_by_kind(SourceContentKind::Spell).len(),
            2,
            "expected 2 bundled spell records"
        );
        assert_eq!(
            corpus.records_by_kind(SourceContentKind::Equipment).len(),
            2,
            "expected 2 bundled equipment records"
        );
    }
}
