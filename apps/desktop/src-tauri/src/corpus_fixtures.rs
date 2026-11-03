//! SD-19 UI-surfacing: bundled corpus-fixture loader.
//!
//! The desktop app has no mechanism to ship or point at the full PCGen
//! corpus, so this loads a small, bounded set of real corpus records —
//! copied verbatim, the same fixture-authoring convention as
//! `tests/fixtures/rules_core/sd19_seam_crb_*.txt` — bundled as a Tauri
//! resource (`resources/corpus_fixtures/`, see `tauri.conf.json`).
//!
//! SD-35 `AT-35-E6-005-SHIPPED-DATA`, operator ruling B17 (`decisions.md` §20):
//! what ships here is CONVERTED data only. The `.lst` row text those records
//! were produced from, and the ingest-format `data.raw_tokens` /
//! `data.raw_bonus_chains` arrays, are converter INPUTS — kept
//! (`decisions.md` §11), moved to `apps/desktop/src-tauri/fixtures_src/`, and
//! not listed in `bundle.resources`, so no PCGen token text goes into the
//! installer. Regenerate with `cargo run --locked --bin
//! gen_desktop_fixture_corpus` then `--bin gen_settled_corpus`.
//! It holds the arms and armor the Market sells a `+N` of, and those `+N`
//! modifiers, so a worn or wielded `+N` moves the sheet's numbers
//! (`enhancement_rows_*` in `character_hub.rs`). What else a character can hold
//! comes after those, off the whole corpus (`add_what_a_character_can_hold`).
//!
//! The `SourcePackageContent` is built once, lazily, and cached for the
//! process lifetime — the underlying parsed records are leaked to get a
//! `'static` borrow, which is fine for a fixed bundle built at most once
//! per process.

use std::path::PathBuf;
use std::sync::OnceLock;

use codex::rules_core::corpus_loader::{load_book_corpus, BookCorpusRoot};
use codex::rules_core::source_content::{SourceContentKind, SourcePackageContent};

use crate::authoring_workbench::{codex_repo_root, resolve_package_path};

const FIXTURE_RESOURCE_ROOT: &str = "resources/corpus_fixtures";
/// How many records the load must find: the spell files, and the settled bundle's equipment. The
/// loader reads equipment values out of that bundle, so a file under `equipment/` with no entry
/// there is a leftover it skips, not a record that failed to load.
fn converted_record_count(dir: &std::path::Path) -> usize {
    let spells = std::fs::read_dir(dir.join("spell")).map_or(0, |entries| {
        entries.filter(|entry| entry.as_ref().is_ok_and(|e| e.path().extension().is_some_and(|ext| ext == "json"))).count()
    });
    let equipment = codex::rules_core::settled_corpus::read_equipment_bundle(dir).map_or(0, |bundle| bundle.records.len());
    spells + equipment
}

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
    let mut package = load_book_corpus(&roots);

    // Loud, never partial: this is a bounded, committed package. An empty or short load means
    // the resource did not ship or the generator was not re-run, which is a broken build --
    // not a diagnostic to be collected and rendered as a character with no equipment.
    let shipped = converted_record_count(&dir);
    assert!(shipped > 0, "no converted record shipped under {}", dir.display());
    assert_eq!(
        package.len(),
        shipped,
        "bundled corpus fixture package must carry all {shipped} converted records, got {} (diagnostics: {:?})",
        package.len(),
        package.diagnostics,
    );
    add_what_a_character_can_hold(&mut package);
    package
}

/// The whole corpus's equipment, after the fixture, so a Ring of Protection worn adds its +1 and a
/// Backpack carried adds its 2 lb: the Market sells every record, and an item this bundle cannot
/// resolve moves no number and reads as unresolved (`character_hub.rs`, `worn_vs_carried_*`). Every
/// book's settled bundle ships (`resources/corpus_bundle/`), and a checkout reads `data/corpus/`.
///
/// What stays out, so that what the fixture already answers does not move:
/// - a record whose identity is a name the fixture answers to. The resolver tries identity across
///   every record first, so a later `Longsword` would answer before the fixture's `Longsword (Base)`.
///   A record's own display name is no bar: the same pass order lets the fixture answer first.
/// - a bare `Shield (...)`: the resolver folds a name's parenthesis away, so any of these would
///   answer the seeds' Absent `item:shield`.
/// - a modifier: `+N` is what the fixture holds; any other attaches as unresolved (DEF-1).
///
/// A book that fails to read says so on stderr and adds nothing; its items read as unresolved.
fn add_what_a_character_can_hold(package: &mut SourcePackageContent<'static>) {
    use codex::rules_core::game_system::GameSystem;
    use codex::rules_core::source_content::{SourceContentPayload, SourceContentRecord};
    use std::collections::HashSet;

    let mut named: HashSet<String> = HashSet::new();
    for record in package.records_by_kind(SourceContentKind::Equipment) {
        if let SourceContentPayload::Equipment(fixture) = record.payload {
            named.insert(fixture.identity.clone());
            named.insert(fixture.name.clone());
        }
    }
    let corpus_root = match codex_repo_root() {
        Ok(root) => GameSystem::Pathfinder1e.package_roots(&root).corpus,
        Err(err) => {
            eprintln!("Sheet corpus: no corpus root ({err}); the sheet reads the fixture alone");
            return;
        }
    };
    let Ok(entries) = std::fs::read_dir(&corpus_root) else {
        eprintln!("Sheet corpus: {} cannot be read; the sheet reads the fixture alone", corpus_root.display());
        return;
    };
    let mut books: Vec<PathBuf> =
        entries.filter_map(|entry| Some(entry.ok()?.path())).filter(|path| path.join("_settled/equipment.json").is_file()).collect();
    // The first printing a name meets answers it: the Core Rulebook, then by name, so the answer
    // is the same on every disk.
    books.sort_by_key(|book| (!book.ends_with("core_rulebook"), book.clone()));
    for book in books {
        let bundle = match codex::rules_core::settled_corpus::read_equipment_bundle(&book) {
            Ok(bundle) => bundle,
            Err(err) => {
                eprintln!("Sheet corpus: {} skipped: {err}", book.display());
                continue;
            }
        };
        for entry in bundle.records.into_values() {
            let record = entry.record;
            if record.is_modifier || record.name.starts_with("Shield (") || named.contains(&record.identity) {
                continue;
            }
            named.insert(record.identity.clone());
            package.push(SourceContentRecord::equipment(entry.source_ref, Box::leak(Box::new(record))));
        }
    }
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

    /// A record dropped from the bundle can stay on disk: `tauri-build` copies resources and
    /// deletes none, and an install over an older version may leave a file. The loader skips a file
    /// with no settled record, so counting files would stop the app starting over a harmless leftover.
    #[test]
    fn a_leftover_record_file_is_not_a_shipped_record() {
        let dir = std::env::temp_dir().join(format!("codex-fixture-leftover-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("equipment")).unwrap();
        std::fs::create_dir_all(dir.join("_settled")).unwrap();
        std::fs::write(dir.join("equipment/equip_gone.json"), "{}").unwrap();
        std::fs::write(dir.join("_settled/equipment.json"), r#"{"kind":"equipment","records":{}}"#).unwrap();
        let shipped = converted_record_count(&dir);
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(shipped, 0);
    }

    #[test]
    fn bundled_fixture_directory_resolves_and_contains_expected_files() {
        let dir = fixture_dir().expect("fixture dir must resolve in a source checkout");
        for name in [
            "spell/spell_abjuration.json",
            "spell/spell_illusion.json",
            "equipment/equip_longsword.json",
            "equipment/equip_chain_shirt.json",
            "equipment/equip_special_ability_5_shield.json",
        ] {
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
        // SD-35 `AT-35-E6-003-RULED` cycle 13: the equipment payload is the
        // settled record, so the same two claims are asserted on the settled
        // values -- the damage die the `DAMAGE:` token stated, and the armour
        // bonus the `BONUS:COMBAT|AC` chain stated.
        let longsword = equipment
            .iter()
            .filter_map(|record| match record.payload {
                SourceContentPayload::Equipment(equip) => Some(equip),
                _ => None,
            })
            .find(|equip| equip.name == "Longsword")
            .expect("the bundled Longsword record must load");
        let damage = longsword
            .base_damage_dice
            .as_ref()
            .expect("Longsword's real base damage must survive the build-time conversion");
        assert_eq!(damage.count, 1, "Longsword's real 1d8, die count");
        assert_eq!(damage.die_size, 8, "Longsword's real 1d8, die size");

        let chain_shirt = equipment
            .iter()
            .filter_map(|record| match record.payload {
                SourceContentPayload::Equipment(equip) => Some(equip),
                _ => None,
            })
            .find(|equip| equip.name == "Chain Shirt")
            .expect("the bundled Chain Shirt record must load");
        assert_eq!(
            chain_shirt.stat_effect.armor_class_bonus,
            Some(4),
            "Chain Shirt's real +4 armour bonus must survive the conversion"
        );
    }

    #[test]
    fn corpus_fixture_bundle_has_two_spells_and_every_market_arm_and_armor() {
        let corpus = corpus_fixture_bundle();
        assert_eq!(
            corpus.records_by_kind(SourceContentKind::Spell).len(),
            2,
            "expected 2 bundled spell records"
        );
        assert_eq!(
            codex::rules_core::settled_corpus::read_equipment_bundle(&fixture_dir().unwrap()).unwrap().records.len(),
            // The Market's 217 weapon + 51 armor and shield rows, less 2 weapons that are a `.COPY`/
            // `.MOD` pair no one-row fixture can hold, and less 2 `Shield (Light/.../Quickdraw)`
            // twins of `Quickdraw Shield (Light/...)`, whose names would answer the seeds'
            // `item:shield` by normalized name. Then the 15 `+N` modifiers.
            279,
            "expected 279 bundled equipment records"
        );
    }

    /// What the whole corpus adds does not move what the fixture answers: the fixture's own record
    /// still answers a seed's token, and a bare `item:shield` still answers nothing.
    #[test]
    fn what_a_character_can_hold_leaves_the_fixture_answering_its_own_names() {
        use codex::rules_core::equipment_resolver::equipment_converted_resolve;
        let corpus = corpus_fixture_bundle();
        assert!(corpus.records_by_kind(SourceContentKind::Equipment).len() > 279, "the corpus came in");
        let identity = |item: &str| equipment_converted_resolve(item, corpus).map(|record| record.identity.as_str());
        assert_eq!(identity("item:longsword"), Some("Longsword (Base)"));
        assert_eq!(identity("item:chain_shirt"), Some("Chain Shirt (Base)"));
        assert_eq!(identity("item:shield"), None);
        assert_eq!(identity("Ring of Protection +1"), Some("Ring of Protection +1"));
    }
}
