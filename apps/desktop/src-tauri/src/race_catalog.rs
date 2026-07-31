//! SD-19 race trait catalog browser — Tauri command adapter over the real
//! on-disk race corpus (`data/corpus/<book>/race/` +
//! `data/corpus/<book>/race_trait/`), read through
//! `codex::rules_core::race_resolver`.
//!
//! **This adapter served the 7 hardcoded CRB races alone until now.** It
//! imported `rules_tables::crb::race_tables::race_traits()` — a 49-row
//! hand-transcribed table — so the 11 Bestiary 1 races that SD-27 ingested
//! (`aasimar drow duergar goblin hobgoblin kobold merfolk orc svirfneblin
//! tengu tiefling`) reached no user-facing surface at all. This module now
//! serves all **18** in-scope races from the corpus itself, and tags each
//! DTO with the book its race came from — exactly the widening
//! `equipment_catalog.rs` already performed across its six books.
//!
//! # What a catalog row is, and what it deliberately is not
//!
//! One row per **racial default trait**: the traits a plain member of the
//! race has, obtained from `RaceCorpus::resolve(race_key, &[])` — the
//! resolver's own no-alternates-selected resolution, not a re-implementation
//! of its protocol here.
//!
//! ARG's 156 *alternate* racial traits are deliberately **not** rows. An
//! alternate is a selection that replaces named standard traits
//! (`decisions.md §26`); rendered as a flat catalog row next to the trait it
//! replaces it would read as a trait the race additionally has — e.g. Dwarf
//! would appear to have `Greed` twice, its CRB one and Saltbeard's
//! replacement. They need a picker that shows the swap, which is real
//! follow-on work in the frontend (not owned by this cycle), not a flat list.
//! The corpus data for them is already loaded here and reachable through
//! `RaceCorpus::alternate_traits`; the count is pinned by a test below so the
//! gap stays visible rather than becoming invisible.
//!
//! # Book attribution
//!
//! A row's `book` is the corpus directory its record was loaded from, which
//! per `decisions.md §25.2` is its true source book. `core_essentials/` —
//! PCGen's physical storage for shared race files — is never a book and never
//! appears here. ARG contributes **zero** rows because it declares zero races
//! and zero racial defaults (`decisions.md §25`); that is asserted below
//! rather than assumed.
//!
//! Distinct from the Character Sheet: this is a standalone catalog view of
//! every real racial default the engine knows about, not what one character
//! has selected.

use std::path::PathBuf;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus, ResolvedTrait};

use crate::ge08_workbench::codex_repo_root;

/// The corpus books that carry race content today. A book with no `race/`
/// or `race_trait/` directory contributes nothing and is not an error, so
/// this list is safe to extend as further books are ingested.
const RACE_CORPUS_BOOKS: &[&str] = &["core_rulebook", "beastiary", "advanced_race_guide"];

/// Which ingested book a catalog entry came from. Short codes are the wire
/// form, identical to the ones `equipment_catalog.rs` and `spell_catalog.rs`
/// already emit for the same books.
const BOOK_CRB: &str = "CRB";
const BOOK_B1: &str = "B1";
const BOOK_ARG: &str = "ARG";

/// Every book code this catalog can emit. ARG is a *loadable* book here but
/// contributes no rows — see this module's doc comment.
pub const RACE_CATALOG_BOOKS: &[&str] = &[BOOK_CRB, BOOK_B1];

/// Maps a corpus book directory name to its wire code. An unrecognized book
/// id passes through verbatim rather than being silently relabelled, so a
/// newly ingested book shows up as itself (and trips
/// `every_book_code_is_a_declared_one_and_every_declared_code_is_present`)
/// instead of being mis-attributed to one of the books above.
fn book_code(book_id: &str) -> String {
    match book_id {
        "core_rulebook" => BOOK_CRB.to_string(),
        "beastiary" => BOOK_B1.to_string(),
        "advanced_race_guide" => BOOK_ARG.to_string(),
        other => other.to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCatalogEntryDto {
    /// The race's stable identity: its corpus race key with separators
    /// removed. For the seven CRB races this is exactly the `RaceId` variant
    /// name the previous hardcoded catalog emitted (`Half-Elf` → `HalfElf`),
    /// so existing consumers — including `reach_gate`'s `races_reach` — are
    /// unaffected. For the eleven Bestiary 1 races the key is a single word,
    /// so it is the key verbatim (`Tengu`, `Svirfneblin`).
    pub race_id: String,
    /// The trait record's own `name`, verbatim from the corpus (e.g.
    /// `Stonecunning`, `Darkvision`, `+2 Dexterity, +2 Wisdom, -2
    /// Constitution`).
    pub trait_name: String,
    /// A display reading, not a resolved mechanical effect: the single
    /// distinct numeric qualifier this trait's own `BONUS:` chains declare
    /// when there is exactly one (`Stonecunning` → 2), else its declared
    /// `MOVE:Walk` in feet (`Slow and Steady` → 20), else 0. `decisions.md
    /// §24` rules out interpreting `BONUS:` formulas, and this does not:
    /// nothing is summed, resolved or attributed to a game effect. A trait
    /// whose chains declare several magnitudes (`+2 Con, +2 Wis, -2 Cha`)
    /// carries 0 and states its numbers in `detail`, exactly as the previous
    /// hardcoded table did.
    pub value: i16,
    /// The trait's real corpus `DESC:` text. Every one of the served rows
    /// carries one today (pinned by a test below), so this is never a
    /// fabricated placeholder.
    pub detail: String,
    /// Which ingested book this race's record came from: one of
    /// [`RACE_CATALOG_BOOKS`]. Additive field — a consumer that does not read
    /// it is unaffected, and one that does can label or filter by book the
    /// way the Equipment and Spell Catalog screens already do.
    pub book: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaceCatalogResponse {
    pub entries: Vec<RaceCatalogEntryDto>,
    /// Corpus files that could not be read, plus any failure to locate the
    /// corpus at all. Empty in a healthy checkout. Carried on the wire so a
    /// shrunken catalog reports *why* it shrank instead of silently serving
    /// less than it claims to.
    pub diagnostics: Vec<String>,
}

fn corpus_root_dir() -> Result<PathBuf, String> {
    codex_repo_root().map(|root| root.join("data/corpus"))
}

/// Loads the real race corpus once per process, mirroring
/// `corpus_full::full_corpus_bundle`'s own caching shape.
fn race_corpus() -> &'static Result<RaceCorpus, String> {
    static CORPUS: OnceLock<Result<RaceCorpus, String>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let corpus_root = corpus_root_dir()?;
        let book_dirs: Vec<PathBuf> =
            RACE_CORPUS_BOOKS.iter().map(|book| corpus_root.join(book)).collect();
        let roots: Vec<BookCorpusRoot<'_>> = RACE_CORPUS_BOOKS
            .iter()
            .zip(book_dirs.iter())
            .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
            .collect();
        Ok(load_race_corpus(&roots))
    })
}

/// See [`RaceCatalogEntryDto::value`] for what this number is and is not.
fn display_value(resolved: &ResolvedTrait) -> i16 {
    if let [only] = resolved.declared_bonus_magnitudes().as_slice() {
        if let Ok(value) = i16::try_from(*only) {
            return value;
        }
    }
    if let Some(feet) = resolved.declared_walk_speed_ft() {
        if let Ok(value) = i16::try_from(feet) {
            return value;
        }
    }
    0
}

/// `Half-Elf` → `HalfElf`. See [`RaceCatalogEntryDto::race_id`].
fn race_identity(race_name: &str) -> String {
    race_name.chars().filter(char::is_ascii_alphanumeric).collect()
}

fn build_catalog() -> RaceCatalogResponse {
    let corpus = match race_corpus() {
        Ok(corpus) => corpus,
        Err(err) => {
            return RaceCatalogResponse {
                entries: Vec::new(),
                diagnostics: vec![format!("race corpus unavailable: {err}")],
            };
        }
    };

    let mut entries = Vec::new();
    for race_key in corpus.race_keys() {
        let Some(race) = corpus.resolve(race_key, &[]) else { continue };
        for resolved in &race.traits {
            entries.push(RaceCatalogEntryDto {
                race_id: race_identity(&race.name),
                trait_name: resolved.name.clone(),
                value: display_value(resolved),
                detail: resolved.description.clone().unwrap_or_default(),
                book: book_code(&resolved.book_id),
            });
        }
    }

    let diagnostics = corpus
        .diagnostics()
        .iter()
        .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
        .collect();

    RaceCatalogResponse { entries, diagnostics }
}

/// Build the full catalog response across every in-scope race. A thin,
/// testable wrapper behind the Tauri command below (mirroring
/// `equipment_catalog`'s own command/pure-fn split).
pub fn build_race_catalog() -> RaceCatalogResponse {
    static CATALOG: OnceLock<RaceCatalogResponse> = OnceLock::new();
    CATALOG.get_or_init(build_catalog).clone()
}

#[tauri::command]
pub fn list_race_catalog() -> RaceCatalogResponse {
    build_race_catalog()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::rules_tables::crb::race_tables::RaceId;
    use std::collections::{BTreeMap, BTreeSet};

    /// Every count in this module's tests was derived by running the catalog
    /// itself, never quoted from a doc:
    ///
    /// ```text
    /// export CARGO_TARGET_DIR=$HOME/.cache/codex-race-desktop
    /// cd apps/desktop/src-tauri && cargo test --bin codex-desktop race_catalog
    /// ```
    fn count_for(response: &RaceCatalogResponse, race_id: &str) -> usize {
        response.entries.iter().filter(|e| e.race_id == race_id).count()
    }

    #[test]
    fn catalog_serves_every_in_scope_race_with_its_real_default_trait_count() {
        let response = build_race_catalog();

        // The 7 Core Rulebook races. Each count is the race's real number of
        // `<Race> Racial Default` corpus rows — richer than the 49-row
        // hardcoded table this replaced, which carried Human 6 / Dwarf 9 /
        // Elf 7 / Gnome 8 / Half-Elf 6 / Half-Orc 5 / Halfling 8.
        assert_eq!(count_for(&response, "Human"), 6);
        assert_eq!(count_for(&response, "Dwarf"), 12);
        assert_eq!(count_for(&response, "Elf"), 9);
        assert_eq!(count_for(&response, "Gnome"), 12);
        assert_eq!(count_for(&response, "HalfElf"), 10);
        assert_eq!(count_for(&response, "HalfOrc"), 9);
        assert_eq!(count_for(&response, "Halfling"), 9);

        // The 11 Bestiary 1 races, which reached no user-facing surface at
        // all before this widening.
        assert_eq!(count_for(&response, "Aasimar"), 9);
        assert_eq!(count_for(&response, "Drow"), 13);
        assert_eq!(count_for(&response, "Duergar"), 10);
        assert_eq!(count_for(&response, "Goblin"), 7);
        assert_eq!(count_for(&response, "Hobgoblin"), 7);
        assert_eq!(count_for(&response, "Kobold"), 9);
        assert_eq!(count_for(&response, "Merfolk"), 9);
        assert_eq!(count_for(&response, "Orc"), 9);
        assert_eq!(count_for(&response, "Svirfneblin"), 13);
        assert_eq!(count_for(&response, "Tengu"), 10);
        assert_eq!(count_for(&response, "Tiefling"), 10);

        // Pinned as a total as well as per race so a race silently dropping
        // out cannot be masked by another race growing.
        assert_eq!(response.entries.len(), 173);

        let races: BTreeSet<&str> = response.entries.iter().map(|e| e.race_id.as_str()).collect();
        assert_eq!(races.len(), 18, "18 in-scope races: {races:?}");
    }

    /// The regression guard for the identity change: `reach_gate::races_reach`
    /// matches catalog rows against `RaceId::ALL`'s variant names, so the
    /// corpus race keys must normalize back to exactly those spellings.
    #[test]
    fn every_hardcoded_race_id_variant_still_has_rows() {
        let response = build_race_catalog();
        for race in RaceId::ALL {
            let id = format!("{race:?}");
            assert!(count_for(&response, &id) > 0, "RaceId::{id} must still reach the catalog");
        }
    }

    #[test]
    fn catalog_serves_a_bestiary_1_race() {
        let response = build_race_catalog();
        let tengu: Vec<_> = response.entries.iter().filter(|e| e.race_id == "Tengu").collect();
        assert!(!tengu.is_empty(), "Bestiary 1's Tengu must reach the catalog");

        for entry in &tengu {
            assert_eq!(entry.book, BOOK_B1, "Tengu is a Bestiary 1 race");
        }

        let by_name: BTreeMap<&str, &&RaceCatalogEntryDto> =
            tengu.iter().map(|e| (e.trait_name.as_str(), e)).collect();

        // Real Tengu corpus rows, verified against
        // data/corpus/beastiary/race_trait/tengu/.
        // The ability-score row states its modifiers in its corpus *name*;
        // its `DESC:` is flavour text, so the numbers are read from the name
        // rather than expected in the detail.
        let abilities = by_name["+2 Dexterity, +2 Wisdom, -2 Constitution"];
        assert_eq!(abilities.value, 0, "two distinct declared magnitudes, so no single number");
        assert_eq!(abilities.detail, "Tengus are fast and observant, but relatively fragile.");

        assert_eq!(by_name["Normal Speed"].value, 30);
        assert_eq!(by_name["Medium"].value, 0);
        assert!(by_name.contains_key("Swordtrained"));
        assert!(by_name.contains_key("Gifted Linguist"));
    }

    #[test]
    fn every_book_code_is_a_declared_one_and_every_declared_code_is_present() {
        let response = build_race_catalog();
        let declared: BTreeSet<&str> = RACE_CATALOG_BOOKS.iter().copied().collect();
        let seen: BTreeSet<&str> = response.entries.iter().map(|e| e.book.as_str()).collect();
        assert_eq!(
            seen, declared,
            "every emitted book code must be declared, and every declared code must actually \
             reach the response — an unreachable code is the exact defect this widening fixes"
        );

        // Derived, not assumed: 67 CRB rows + 106 Bestiary 1 rows = 173.
        let crb = response.entries.iter().filter(|e| e.book == BOOK_CRB).count();
        let b1 = response.entries.iter().filter(|e| e.book == BOOK_B1).count();
        assert_eq!(crb, 67);
        assert_eq!(b1, 106);
        assert_eq!(crb + b1, response.entries.len());
    }

    /// ARG declares zero races and zero racial defaults (`decisions.md §25`),
    /// so it contributes zero rows even though its corpus directory is loaded.
    /// The alternate traits it *does* declare are not catalog rows — see this
    /// module's doc comment — and are counted here so that gap stays measured
    /// rather than forgotten. 153, derived: ARG's 156 corpus records are 153
    /// `Alternate` plus 3 the resolver classifies otherwise (its
    /// `FlagGranted`/`Unclassified` rows).
    #[test]
    fn arg_contributes_no_rows_but_its_alternates_are_loaded_and_counted() {
        let response = build_race_catalog();
        assert_eq!(response.entries.iter().filter(|e| e.book == BOOK_ARG).count(), 0);

        let corpus = race_corpus().as_ref().expect("race corpus loads in a source checkout");
        let alternates: usize =
            corpus.race_keys().iter().map(|key| corpus.alternate_traits(key).len()).sum();
        assert_eq!(alternates, 153, "alternate racial traits, loaded but not yet surfaced");
    }

    /// The widening's most consequential user-visible correction: the
    /// hardcoded table said Dwarf was "+2 Constitution / -2 Charisma" and Elf
    /// "+2 Dexterity / -2 Constitution", each dropping the second stat PCGen
    /// grants in the same token. The corpus row states all three, so a player
    /// reading this catalog now sees the real PF1 modifier line.
    #[test]
    fn corpus_ability_rows_carry_the_stat_the_hardcoded_table_dropped() {
        let response = build_race_catalog();
        let names = |race_id: &str| -> Vec<&str> {
            response
                .entries
                .iter()
                .filter(|e| e.race_id == race_id)
                .map(|e| e.trait_name.as_str())
                .collect()
        };
        assert!(names("Dwarf").contains(&"+2 Constitution, +2 Wisdom, -2 Charisma"));
        assert!(names("Elf").contains(&"+2 Dexterity, +2 Intelligence, -2 Constitution"));
    }

    #[test]
    fn every_entry_has_a_non_empty_race_id_trait_name_detail_and_book() {
        let response = build_race_catalog();
        assert!(!response.entries.is_empty());
        for entry in &response.entries {
            assert!(!entry.race_id.is_empty());
            assert!(!entry.trait_name.is_empty());
            assert!(
                !entry.detail.is_empty(),
                "{} / {} has no corpus DESC: text",
                entry.race_id,
                entry.trait_name
            );
            assert!(!entry.book.is_empty());
        }
    }

    /// A healthy source checkout must load every corpus file. A diagnostic
    /// here means the catalog is quietly serving less than the corpus holds.
    #[test]
    fn the_real_corpus_loads_with_no_diagnostics() {
        let response = build_race_catalog();
        assert!(response.diagnostics.is_empty(), "{:?}", response.diagnostics);
    }

    /// The frontend keys its rows on `raceId:traitName`, so duplicates would
    /// collide. Serving racial defaults only, they do not.
    #[test]
    fn race_id_and_trait_name_together_are_unique() {
        let response = build_race_catalog();
        let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
        for entry in &response.entries {
            assert!(
                seen.insert((entry.race_id.as_str(), entry.trait_name.as_str())),
                "duplicate row {} / {}",
                entry.race_id,
                entry.trait_name
            );
        }
    }

    /// The `book` field must actually serialize onto the wire under the
    /// camelCase name the TypeScript boundary reads — a Rust-side field that
    /// never crosses the IPC boundary would surface nothing, which is the
    /// exact defect class this change closes.
    #[test]
    fn book_is_serialized_onto_the_wire() {
        let entry = build_race_catalog()
            .entries
            .into_iter()
            .find(|entry| entry.book == BOOK_B1)
            .expect("Bestiary 1 entries are in the catalog");
        let json = serde_json::to_value(&entry).expect("entry serializes");
        assert_eq!(json.get("book").and_then(|v| v.as_str()), Some(BOOK_B1));
        assert!(json.get("raceId").is_some(), "existing camelCase fields are unchanged");
        assert!(json.get("traitName").is_some());
    }
}
