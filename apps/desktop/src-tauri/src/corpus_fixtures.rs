//! SD-19 UI-surfacing: bundled corpus-fixture loader.
//!
//! The desktop app has no mechanism to ship or point at the full PCGen
//! corpus, so this loads a small, bounded set of real corpus records —
//! copied verbatim, the same fixture-authoring convention as
//! `tests/fixtures/rules_core/sd19_seam_crb_*.txt` — bundled as a Tauri
//! resource (`resources/corpus_fixtures/`, see `tauri.conf.json`).
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

use codex::rules_core::corpus_loader::{load_book_corpus, BookCorpusRoot};
use codex::rules_core::source_content::SourcePackageContent;

use crate::authoring_workbench::resolve_package_path;

const FIXTURE_RESOURCE_ROOT: &str = "resources/corpus_fixtures";
/// The converter INPUTS that produced the shipped records. They are bundled
/// beside the converted package so the provenance of every shipped record is
/// one file away, and so `gen_desktop_fixture_corpus` can be re-run against
/// exactly what shipped. Nothing in this crate parses them.
const FIXTURE_SOURCES: &[&str] = &[
    "spell_abjuration.txt",
    "spell_illusion.txt",
    "equip_longsword.txt",
    "equip_chain_shirt.txt",
];
/// The CONVERTED records this crate actually reads, in the
/// `<root>/spell/*.json` + `<root>/equipment/*.json` layout
/// `rules_core::corpus_loader` reads the real corpus in.
const CONVERTED_RECORDS: &[&str] = &[
    "spell/spell_abjuration.json",
    "spell/spell_illusion.json",
    "equipment/equip_longsword.json",
    "equipment/equip_chain_shirt.json",
];

fn fixture_dir() -> Result<PathBuf, String> {
    resolve_package_path(FIXTURE_RESOURCE_ROOT)
}

fn build_corpus_fixture_bundle() -> SourcePackageContent<'static> {
    let dir = fixture_dir().expect("corpus_fixtures resource directory must resolve");

    // Reading the bundled resource is this crate's concern. Parsing and converting it is not,
    // and since SD-35 `AT-35-E6-003-RULED` cycle 9 NOTHING does either at run time: the
    // records ship already converted, produced at authoring time by
    // `src/bin/gen_desktop_fixture_corpus.rs`, and this is the same
    // `rules_core::corpus_loader` call the real on-disk corpus goes through
    // (`decisions.md` §19, ruling B16 -- no live path reads `pcgen_import` to build this).
    let roots = [BookCorpusRoot { book_id: "desktop_ui_fixtures", dir: dir.as_path() }];
    let package = load_book_corpus(&roots);

    // Loud, never partial: this is a bounded, committed package. An empty or short load means
    // the resource did not ship or the generator was not re-run, which is a broken build --
    // not a diagnostic to be collected and rendered as a character with no equipment.
    assert_eq!(
        package.len(),
        CONVERTED_RECORDS.len(),
        "bundled corpus fixture package must carry exactly {} converted records, got {} (diagnostics: {:?})",
        CONVERTED_RECORDS.len(),
        package.len(),
        package.diagnostics,
    );
    package
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
        for name in FIXTURE_SOURCES.iter().chain(CONVERTED_RECORDS.iter()) {
            assert!(
                dir.join(name).is_file(),
                "expected bundled fixture '{name}' at {}",
                dir.join(name).display()
            );
        }
    }

    /// SD-35 `AT-35-E6-003-RULED` cycle 9: what ships is CONVERTED data, and
    /// the shipping binary never parses a PCGen row to get it. The converted
    /// records resolve to the same real values the raw-`.lst` path produced --
    /// this asserts them on the resolved package, not on the file text, so it
    /// fails if the generator ever writes a document the live loader cannot
    /// read back.
    #[test]
    fn bundled_records_carry_their_real_converted_values() {
        use codex::rules_core::source_content::SourceContentPayload;

        let corpus = corpus_fixture_bundle();
        let spells = corpus.records_by_kind(SourceContentKind::Spell);
        let schools: Vec<(String, Option<String>)> = spells
            .iter()
            .filter_map(|record| match record.payload {
                SourceContentPayload::Spell(spell) => {
                    Some((spell.name.clone(), spell.school.clone()))
                }
                _ => None,
            })
            .collect();
        assert!(
            schools.contains(&("Alarm".to_string(), Some("Abjuration".to_string()))),
            "Alarm must load as an Abjuration spell, got {schools:?}"
        );
        assert!(
            schools.contains(&("Blur".to_string(), Some("Illusion".to_string()))),
            "Blur must load as an Illusion spell, got {schools:?}"
        );

        let equipment = corpus.records_by_kind(SourceContentKind::Equipment);
        let longsword = equipment
            .iter()
            .filter_map(|record| match record.payload {
                SourceContentPayload::Equipment(equip) => Some(equip),
                _ => None,
            })
            .find(|equip| equip.name == "Longsword")
            .expect("the bundled Longsword record must load");
        let damage = longsword
            .tokens
            .iter()
            .find(|token| token.key == "DAMAGE")
            .expect("Longsword's real damage token must survive the build-time conversion");
        assert_eq!(damage.value, "1d8", "Longsword's real DAMAGE value");

        let chain_shirt = equipment
            .iter()
            .filter_map(|record| match record.payload {
                SourceContentPayload::Equipment(equip) => Some(equip),
                _ => None,
            })
            .find(|equip| equip.name == "Chain Shirt")
            .expect("the bundled Chain Shirt record must load");
        let ac_chain = chain_shirt
            .bonus_chains
            .iter()
            .find(|chain| chain.qualifiers.first().map(String::as_str) == Some("COMBAT"))
            .expect("Chain Shirt's real armour bonus chain must survive the conversion");
        assert_eq!(
            ac_chain.qualifiers,
            vec!["COMBAT", "AC", "4", "TYPE=Armor", "PREVAREQ:DisableArmorBonus,0"],
            "Chain Shirt's first BONUS chain, qualifier for qualifier"
        );
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
