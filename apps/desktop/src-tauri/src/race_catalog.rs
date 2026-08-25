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
//! ARG's 153 *alternate* racial traits are deliberately **not** rows (156 is
//! its corpus record count; 153 of those classify as `Alternate`, as
//! `arg_contributes_no_rows_but_its_alternates_are_loaded_and_counted`
//! derives). An
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
//! per `decisions.md §25.2` is its true source book.
//!
//! **`core_essentials` is now one of those directories, and the reason is a
//! narrow one.** PCGen uses `core_essentials/races/<race>/` as physical
//! storage for the shared racial-trait files of races that *belong* to other
//! books, and `ingest_races` correctly files those under `core_rulebook` and
//! `beastiary` — so a Dwarf standard trait is still never attributed here to
//! `core_essentials`, exactly as this note used to say of the whole
//! directory. What changed (SD-29 race-trait lane, round 4) is that two files
//! in that tree carry content belonging to no other book: Aasimar's and
//! Tiefling's *heritage* traits, which no Paizo book outside this data set
//! declares. Those 64 records are attributed to `core_essentials` because
//! that is genuinely where they come from, and they contribute **zero**
//! catalog rows here regardless — none is a racial default.
//!
//! ARG contributes **zero** rows because it declares zero races
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
use codex::rules_core::shape_b_v1::RawBonusChain;

use crate::authoring_workbench::codex_repo_root;

/// The corpus books that carry race content today. A book with no `race/`
/// or `race_trait/` directory contributes nothing and is not an error, so
/// this list is safe to extend as further books are ingested.
/// `pub(crate)` so `corpus_ingest_diagnostic` reports a per-book race count
/// over exactly the books this catalog actually searches — including a book
/// that is searched and contributes nothing (ARG), which is a measured zero
/// rather than an omission.
pub(crate) const RACE_CORPUS_BOOKS: &[&str] = &[
    "core_rulebook",
    "beastiary",
    "advanced_race_guide",
    "advanced_players_guide",
    "monster_codex",
    "inner_sea_races",
    "horror_adventures",
    // Bestiary 2, SD-31 Epic 1-F2 (2026-08-15): the chassis + standard
    // traits for 6 newly-modelled races (`ingest_races::IN_SCOPE_RACES`),
    // filed under `bestiary_2` per `advanced_race_guide.pcc`'s own
    // `# B2 races` section, exactly the way core_rulebook/beastiary already
    // are.
    "bestiary_2",
    // Bestiary 5, SD-31 Epic 1 follow-on batch (2026-08-15): Skinwalker's
    // chassis + 9 standard-tier traits (`ingest_races::IN_SCOPE_RACES`).
    // Skinwalker's HERITAGE rows are not ingested by this batch (see
    // `ingest_races.rs`'s `skinwalker` entry doc comment) -- this book
    // loads the chassis + default-tier trait set only, same shape as the
    // other flat, non-heritage race books above.
    "bestiary_5",
    // Bestiary 6, SD-31 wave-24 integration cycle (2026-08-20): Rougarou's
    // chassis + 8 standard-tier traits (`ingest_races::IN_SCOPE_RACES`).
    // Confirmed a flat, non-heritage shape (no `*_subrace.lst` file, no
    // `Rougarou_Replace*` flag ever set to `True` anywhere in the pinned
    // oracle) -- same shape as Bestiary 2/5 above, not the Dhampir-style
    // heritage gap an earlier note wrongly grouped it with.
    "bestiary_6",
    // Bestiary 3, SD-32 `decisions.md §25` cycle 2: 5 Adopted-Race selector
    // records (`ingest_race_traits.rs`'s new `selector_only` `BookSource`).
    // Loaded, unlike Bestiary 2/5/6, purely for its 5 selector rows -- this
    // book contributes zero chassis and zero standard-tier traits of its own
    // (its 5 target races' chassis are ARG-native, already loaded under
    // `advanced_race_guide`), so it never appears in `RACE_CATALOG_BOOKS`
    // below, the same way ARG/APG/MC/ISR/HA are loaded here without
    // contributing catalog rows.
    "bestiary_3",
];

/// Which ingested book a catalog entry came from. Short codes are the wire
/// form, identical to the ones `equipment_catalog.rs` and `spell_catalog.rs`
/// already emit for the same books.
const BOOK_CRB: &str = "CRB";
const BOOK_B1: &str = "B1";
const BOOK_ARG: &str = "ARG";
const BOOK_APG: &str = "APG";
/// Monster Codex. Loadable like ARG and APG, and like them it contributes no
/// *catalog* rows — the catalog serves racial-default traits and this book
/// declares none. Its five records are all alternates, which reach a player
/// through `race_trait_picker` instead. See SD-29 `decisions.md §43`.
const BOOK_MC: &str = "MC";
/// Inner Sea Races. Loadable like ARG, APG and MC, and like them it
/// contributes no *catalog* rows — it declares no racial-default trait. Its 72
/// records are 68 alternates plus 4 rows the resolver classifies from their
/// gates. SD-29 race-trait lane, round 2.
const BOOK_ISR: &str = "ISR";
/// Horror Adventures. Loadable like ARG, APG, MC and ISR, and like them it
/// contributes no *catalog* rows — it declares no racial-default trait. Its 43
/// records are 41 alternates plus the two `Deep Jungle Halfling ~ …` rows the
/// resolver classifies from their gates. SD-29 race-trait lane, round 3.
const BOOK_HA: &str = "HA";
/// Bestiary 2. SD-31 Epic 1-F2 (2026-08-15): the first race-chassis batch
/// this project has added since the original 18 (`decisions.md §25.3`).
/// Loaded exactly like `core_rulebook`/`beastiary` — `ingest_races` files
/// its 6 races' chassis and standard traits here, so unlike ARG/APG/MC/
/// ISR/HA/CE it DOES contribute catalog rows (see `RACE_CATALOG_BOOKS`
/// below).
const BOOK_B2: &str = "B2";
/// Bestiary 5. SD-31 Epic 1 follow-on batch (2026-08-15): Skinwalker's
/// chassis + standard traits. Loaded like `core_rulebook`/`beastiary`/
/// `bestiary_2` -- `ingest_races` files Skinwalker's chassis and 9
/// standard-tier traits here, so it DOES contribute catalog rows (see
/// `RACE_CATALOG_BOOKS` below).
const BOOK_B5: &str = "B5";
/// Bestiary 6. SD-31 wave-24 integration cycle (2026-08-20): Rougarou's
/// chassis + 8 standard traits. Loaded like `bestiary_2`/`bestiary_5` --
/// `ingest_races` files Rougarou's chassis and standard-tier traits here,
/// so it DOES contribute catalog rows (see `RACE_CATALOG_BOOKS` below).
const BOOK_B6: &str = "B6";
/// Bestiary 3. SD-32 `decisions.md §25` cycle 2: 5 Adopted-Race selector
/// records only (see `RACE_CORPUS_BOOKS`'s own doc comment above) -- it
/// contributes no catalog rows, so unlike `BOOK_B2`/`BOOK_B5`/`BOOK_B6` it is
/// never added to `RACE_CATALOG_BOOKS` below.
const BOOK_B3: &str = "B3";

/// Every book code this catalog can emit. As of SD-31-E6-F4-002
/// (2026-08-16) ARG DOES contribute catalog rows: `ingest_races` filed 6
/// new races' (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang) chassis and
/// standard traits under `advanced_race_guide`, the first race-catalog
/// content this book has ever declared. See `BOOK_ARG`'s call site below
/// and this module's doc comment for why ARG previously contributed none.
pub const RACE_CATALOG_BOOKS: &[&str] = &[BOOK_CRB, BOOK_B1, BOOK_B2, BOOK_B5, BOOK_B6, BOOK_ARG];

/// Maps a corpus book directory name to its wire code. An unrecognized book
/// id passes through verbatim rather than being silently relabelled, so a
/// newly ingested book shows up as itself (and trips
/// `every_book_code_is_a_declared_one_and_every_declared_code_is_present`)
/// instead of being mis-attributed to one of the books above.
///
/// `pub(crate)` so `character_hub`'s race-creation roster labels its races
/// with the *same* book codes this catalog emits rather than carrying a
/// second copy of the mapping that could drift from this one.
pub(crate) fn book_code(book_id: &str) -> String {
    match book_id {
        "core_rulebook" => BOOK_CRB.to_string(),
        "beastiary" => BOOK_B1.to_string(),
        "advanced_race_guide" => BOOK_ARG.to_string(),
        "advanced_players_guide" => BOOK_APG.to_string(),
        "monster_codex" => BOOK_MC.to_string(),
        "inner_sea_races" => BOOK_ISR.to_string(),
        "horror_adventures" => BOOK_HA.to_string(),
        "bestiary_2" => BOOK_B2.to_string(),
        "bestiary_5" => BOOK_B5.to_string(),
        "bestiary_6" => BOOK_B6.to_string(),
        "bestiary_3" => BOOK_B3.to_string(),
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
    /// `VISION:` range in feet (`Darkvision` → 60), else its declared
    /// `MOVE:Walk` in feet (`Slow and Steady` → 20), else 0. `decisions.md
    /// §24` rules out interpreting `BONUS:` formulas, and this does not:
    /// nothing is summed, resolved or attributed to a game effect. A trait
    /// whose chains declare several magnitudes (`+2 Con, +2 Wis, -2 Cha`)
    /// carries 0 and states its numbers in `detail`, exactly as the previous
    /// hardcoded table did.
    ///
    /// **`BONUS:` chains that only write an internal PCGen state flag declare
    /// no quantity and are excluded** — see [`is_internal_flag_chain`]. They
    /// were previously read as magnitudes, which put a meaningless `+1` beside
    /// every vision trait in both books (the flag is
    /// `BONUS:VAR|HasRacialVision|1`; the real quantity was sitting unread in
    /// the row's `VISION:Darkvision (60)` token) and beside three other rows.
    /// 19 rows in total; `the_rows_the_internal_flag_correction_changes_are_pinned`
    /// names every one.
    ///
    /// 0 means "this trait has no single honest number", which is exactly how
    /// the frontend reads it — it renders no badge at 0. It is never a
    /// stand-in for a number the catalog could not work out.
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
///
/// `pub(crate)` so `character_hub`'s race-creation roster reads the *same*
/// cached corpus this catalog browser reads, rather than loading a second
/// copy that could answer differently.
pub(crate) fn race_corpus() -> &'static Result<RaceCorpus, String> {
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

/// PCGen writes internal engine state with the same `BONUS:VAR|<name>|<n>`
/// token it uses for real magnitudes, so the token shape alone cannot tell a
/// game quantity from a boolean flag. Two signals in the corpus can, and this
/// recognizes a name written by either:
///
/// 1. **`Has…` / `Is…` / `…Flag` / `…ExoticUse`** — PCGen's boolean naming
///    conventions. Every vision trait in both ingested books declares exactly
///    one chain, `BONUS:VAR|HasRacialVision|1`; `Tengu ~ Swordtrained`
///    declares `BONUS:VAR|BastardSwordExoticUse,KatanaExoticUse|1` beside an
///    `AUTO:WEAPONPROF` that carries the actual mechanic.
/// 2. See [`is_internal_flag_chain`] for the explicit `TYPE=Boolean`
///    qualifier, which needs no name convention at all.
///
/// The `Has`/`Is` tests require an uppercase letter after the prefix so that
/// magnitude names beginning with those letters (`Hasted_Bonus`,
/// `Island_Bonus`) are not swallowed. Deliberately *not* recognized:
/// `Orc_OrcFerocity_Times` and `Halfling_AdaptableLuck_Times` (uses per day),
/// `AC_Natural_Armor`, and every `…Bonus` — those are real quantities that a
/// looser rule would silently erase, which would be a worse defect than the
/// one this closes.
fn variable_name_is_flag_shaped(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return false;
    }
    if name.ends_with("Flag") || name.ends_with("ExoticUse") {
        return true;
    }
    ["Has", "Is"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|next| next.is_ascii_uppercase())
    })
}

/// True when this `BONUS:` chain only writes an internal PCGen state flag and
/// therefore declares no game quantity at all.
///
/// Authoritative signal first: PCGen tags the chain itself `TYPE=Boolean`.
/// Three served rows carry it — `Drow ~ Light Blindness`
/// (`UMR_LightBlindness_SpecificDesc`), `Merfolk ~ Legless` (`CantBeTripped`)
/// and `Svirfneblin ~ Svirfneblin Magic` (`RacialSLA_Nondetection_Constant`) —
/// as do four ARG alternates that will matter once the alternate-trait picker
/// lands. The vision traits are *not* tagged, hence the name conventions in
/// [`variable_name_is_flag_shaped`] as the second signal.
///
/// A chain naming several variables counts as a flag only when *every* name is
/// flag-shaped, so a mixed chain keeps its magnitude. Non-`VAR` chains are
/// never flags: `Svirfneblin ~ Svirfneblin Magic`'s companion
/// `BONUS:DC|SCHOOL.Illusion|1` is a real +1 and survives.
fn is_internal_flag_chain(chain: &RawBonusChain) -> bool {
    if chain.qualifiers.first().map(String::as_str) != Some("VAR") {
        return false;
    }
    if chain.qualifiers.iter().any(|qualifier| qualifier == "TYPE=Boolean") {
        return true;
    }
    let Some(names) = chain.qualifiers.get(1) else {
        return false;
    };
    let mut named = names.split(',').filter(|name| !name.trim().is_empty()).peekable();
    named.peek().is_some() && named.all(variable_name_is_flag_shaped)
}

/// The magnitudes this trait declares once internal flag writes are discarded.
/// Same reading as [`ResolvedTrait::declared_bonus_magnitudes`] — in source
/// order, deduplicated, nothing summed or interpreted — over the chains that
/// actually state a quantity.
fn declared_magnitudes_excluding_flags(resolved: &ResolvedTrait) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for chain in resolved.raw_bonus_chains.iter().filter(|chain| !is_internal_flag_chain(chain)) {
        for qualifier in &chain.qualifiers {
            if let Ok(value) = qualifier.parse::<i32>() {
                if !out.contains(&value) {
                    out.push(value);
                }
            }
        }
    }
    out
}

/// The range in feet a vision trait declares, read out of its `VISION:` tokens
/// (`VISION:Darkvision (60)` → 60). This is the real quantity the internal
/// `HasRacialVision` flag was standing in for.
///
/// `VISION:Low-Light Vision` states no range and contributes nothing, so a
/// low-light row honestly has no number. A row may carry several `VISION:`
/// tokens (`Svirfneblin ~ Senses` carries `Darkvision (120)` and `Low-Light
/// Vision`); as with [`declared_magnitudes_excluding_flags`], a single distinct
/// range is a reading and several are not.
fn declared_vision_range_ft(resolved: &ResolvedTrait) -> Option<i32> {
    let mut ranges: Vec<i32> = Vec::new();
    for token in resolved.raw_tokens.iter().filter(|token| token.key == "VISION") {
        for segment in token.value.split('(').skip(1) {
            let Some((digits, _)) = segment.split_once(')') else { continue };
            if let Ok(range) = digits.trim().parse::<i32>() {
                if !ranges.contains(&range) {
                    ranges.push(range);
                }
            }
        }
    }
    match ranges.as_slice() {
        [only] => Some(*only),
        _ => None,
    }
}

/// See [`RaceCatalogEntryDto::value`] for what this number is and is not.
///
/// Precedence: a single declared `BONUS:` magnitude, else a declared `VISION:`
/// range, else a declared `MOVE:Walk`, else no number. Internal flag writes are
/// discarded before the first step rather than being allowed to win it.
fn display_value(resolved: &ResolvedTrait) -> i16 {
    if let [only] = declared_magnitudes_excluding_flags(resolved).as_slice() {
        if let Ok(value) = i16::try_from(*only) {
            return value;
        }
    }
    if let Some(feet) = declared_vision_range_ft(resolved) {
        if let Ok(value) = i16::try_from(feet) {
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

/// The race identities one book actually declares a chassis record for.
///
/// Read from the corpus's own chassis records rather than from the catalog's
/// trait rows, so `reach_gate`'s races claim compares two independent things:
/// what a book ingested (here) against what reaches a player (the catalog
/// rows). Deriving both from the catalog would make the claim vacuously true.
///
/// `book_id` is the corpus directory name (`"core_rulebook"`, `"beastiary"`),
/// not the wire code.
pub(crate) fn ingested_race_ids_for_book(book_id: &str) -> std::collections::BTreeSet<String> {
    let Ok(corpus) = race_corpus() else {
        return std::collections::BTreeSet::new();
    };
    corpus
        .race_keys()
        .into_iter()
        .filter(|race_key| {
            corpus.chassis(race_key).is_some_and(|chassis| chassis.book_id == book_id)
        })
        .filter_map(|race_key| corpus.resolve(race_key, &[]))
        .map(|race| race_identity(&race.name))
        .collect()
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

        // The 6 Bestiary 2 races, SD-31 Epic 1-F2 (2026-08-15) -- the first
        // chassis batch since the original 18. Every one of the 57 corpus
        // rows this batch added is a `<Race> Racial Default` row (re-derived:
        // `find data/corpus/bestiary_2/race_trait -name '*.json'` gives the
        // per-race counts below, and every one of the 57 records'
        // `is_racial_default` is `true`), so `count_for` equals each race's
        // whole standard-trait count with none held back as non-default.
        assert_eq!(count_for(&response, "Fetchling"), 11);
        assert_eq!(count_for(&response, "Grippli"), 10);
        assert_eq!(count_for(&response, "Ifrit"), 9);
        assert_eq!(count_for(&response, "Oread"), 9);
        assert_eq!(count_for(&response, "Sylph"), 9);
        assert_eq!(count_for(&response, "Undine"), 9);
        // Dhampir, SD-32 card-11 T2b lane (2026-08-23): chassis + the 12
        // unconditional `<Race> Racial Default` rows only (its heritage/
        // subrace file stays deferred, same precedent as Skinwalker below).
        assert_eq!(count_for(&response, "Dhampir"), 12);
        // Skinwalker, the follow-on batch (2026-08-15): 9 standard-tier
        // records, chassis + default tier only (heritage rows excluded --
        // see `ingest_races.rs`'s `skinwalker` doc comment).
        assert_eq!(count_for(&response, "Skinwalker"), 9);

        // Advanced Race Guide's own 6-race batch, SD-31-E6-F4-002
        // (2026-08-16): Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang --
        // the same flat chassis+standard-trait shape as Bestiary 2/5 above,
        // no heritage content. Re-derived, not transcribed:
        // `find data/corpus/advanced_race_guide/race_trait/{catfolk,kitsune,
        // ratfolk,strix,suli,wayang} -name '*.json' | wc -l` per race.
        assert_eq!(count_for(&response, "Catfolk"), 9);
        assert_eq!(count_for(&response, "Kitsune"), 10);
        assert_eq!(count_for(&response, "Ratfolk"), 9);
        assert_eq!(count_for(&response, "Strix"), 11);
        assert_eq!(count_for(&response, "Suli"), 9);
        assert_eq!(count_for(&response, "Wayang"), 10);

        // SD31-E6-F4-004 (2026-08-17): 4 more of ARG's own races (Gillman,
        // Nagaji, Vanara, Vishkanya) -- same flat shape, no heritage
        // content. Re-derived the same way as the batch above.
        assert_eq!(count_for(&response, "Gillman"), 9);
        assert_eq!(count_for(&response, "Nagaji"), 9);
        assert_eq!(count_for(&response, "Vanara"), 8);
        assert_eq!(count_for(&response, "Vishkanya"), 12);

        // SD31-E6-F4-007 (2026-08-17): the last 2 of ARG's own races
        // (Changeling, Samsaran), closing `arg_races.lst`'s full 37-row
        // playable-race roster. Same flat chassis+standard-trait shape as
        // the batch above -- Changeling's 3 hag-mother heritage-choice
        // sub-traits are deliberately excluded (`ingest_races.rs`'s
        // `is_heritage_choice_subtrait`), so its count is 9, not 12.
        assert_eq!(count_for(&response, "Changeling"), 9);
        assert_eq!(count_for(&response, "Samsaran"), 9);

        // SD-31 wave-24 integration cycle (2026-08-20): Rougarou (Bestiary
        // 6), the same flat chassis+standard-trait shape, all 8 of its
        // default-trait rows carrying `is_racial_default: true` (re-derived:
        // `find data/corpus/bestiary_6/race_trait/rougarou -name '*.json' |
        // wc -l`).
        assert_eq!(count_for(&response, "Rougarou"), 8);

        // Pinned as a total as well as per race so a race silently dropping
        // out cannot be masked by another race growing.
        // 173 + 57 + 9 + 96 (58 + 9+9+8+12) + 18 (9+9) + 8 + 12 (Dhampir,
        // SD-32 card-11 T2b lane, 2026-08-23) = 373.
        assert_eq!(response.entries.len(), 373);

        let races: BTreeSet<&str> = response.entries.iter().map(|e| e.race_id.as_str()).collect();
        assert_eq!(races.len(), 39, "39 in-scope races: {races:?}");
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

        // Derived, not assumed: 67 CRB rows + 106 Bestiary 1 rows + 57
        // Bestiary 2 rows (SD-31 Epic 1-F2, 2026-08-15, plus Dhampir's 12,
        // SD-32 card-11 T2b lane, 2026-08-23) + 9 Bestiary 5 rows
        // (Skinwalker follow-on batch, 2026-08-15) + 8 Bestiary 6 rows
        // (Rougarou, SD-31 wave-24, 2026-08-20) + ARG's 12-race total
        // (58 from SD-31-E6-F4-002's Catfolk/Kitsune/Ratfolk/Strix/Suli/
        // Wayang batch, 2026-08-16, + 38 from SD31-E6-F4-004's Gillman/
        // Nagaji/Vanara/Vishkanya batch, 2026-08-17, + 18 from
        // SD31-E6-F4-007's Changeling/Samsaran batch, 2026-08-17, closing
        // `arg_races.lst`'s full 37-row playable-race roster).
        let crb = response.entries.iter().filter(|e| e.book == BOOK_CRB).count();
        let b1 = response.entries.iter().filter(|e| e.book == BOOK_B1).count();
        let b2 = response.entries.iter().filter(|e| e.book == BOOK_B2).count();
        let b5 = response.entries.iter().filter(|e| e.book == BOOK_B5).count();
        let b6 = response.entries.iter().filter(|e| e.book == BOOK_B6).count();
        let arg = response.entries.iter().filter(|e| e.book == BOOK_ARG).count();
        assert_eq!(crb, 67);
        assert_eq!(b1, 106);
        assert_eq!(b2, 69);
        assert_eq!(b5, 9);
        assert_eq!(b6, 8);
        assert_eq!(arg, 114);
        assert_eq!(crb + b1 + b2 + b5 + b6 + arg, response.entries.len());
    }

    /// APG, Monster Codex, ISR and HA declare zero races and zero racial
    /// defaults, so each contributes zero rows even though its corpus
    /// directory is loaded. **ARG is no longer in this loop** — as of
    /// SD-31-E6-F4-002 (2026-08-16) it declares 6 races' worth of racial
    /// defaults (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang) and its
    /// row count is asserted at 87 in
    /// `every_book_code_is_a_declared_one_and_every_declared_code_is_present`
    /// instead. The alternate traits these books *do* declare are not
    /// catalog rows — see this module's doc comment — and are counted here
    /// so that gap stays measured rather than forgotten. 349, derived:
    /// ARG's 156 corpus records are 153 `Alternate` plus 3 the resolver
    /// classifies otherwise (its `FlagGranted`/`Unclassified` rows),
    /// Monster Codex's 5 are 4 `Alternate` plus `Oversized Goblin`
    /// (`Unclassified`), and APG's 1 is an `Alternate`. **Widened by
    /// SD-31-E6-F4-003 (2026-08-16): `ingest_race_traits.rs`'s
    /// `IN_SCOPE_RACES` now carries the same 6 new races, and their real
    /// ARG alternate-trait content is ingested — +19 `Alternate` (+24
    /// records total, 5 of them `FlagGranted`).
    ///
    /// **APG's `Half-Orc ~ Plagueborn` is no longer deferred.** SD-27
    /// `decisions.md §39` held it back because `race_resolver.rs`'s
    /// `ALTERNATE_TRAIT_REPLACE_FLAGS` table did not know its key, so
    /// shipping the corpus record would have offered it here and refused it
    /// at character-save time. SD-29's race-trait extend lane landed both
    /// halves together — the record and the table row — so the affordance is
    /// live rather than a stub. APG declares no races/defaults of its own, so
    /// it still contributes zero catalog rows.
    #[test]
    fn alternate_only_books_contribute_no_catalog_rows_but_are_loaded_and_counted() {
        let response = build_race_catalog();
        // ISR and HA belong in this loop for the same reason APG/MC do:
        // each is loaded and each declares no racial DEFAULT trait. Round 2
        // added ISR's records without adding it here, so the loop was one book
        // narrower than the list it is a statement about; round 3 adds both
        // rather than leaving a gap it can see (`decisions.md §44.5`).
        for book in [BOOK_APG, BOOK_MC, BOOK_ISR, BOOK_HA] {
            assert_eq!(
                response.entries.iter().filter(|e| e.book == book).count(),
                0,
                "{book} declares no racial DEFAULT traits, so it contributes no catalog row"
            );
        }

        let corpus = race_corpus().as_ref().expect("race corpus loads in a source checkout");
        let alternates: usize =
            corpus.race_keys().iter().map(|key| corpus.alternate_traits(key).len()).sum();
        assert_eq!(
            alternates, 370,
            "alternate racial traits loaded but contributing no catalog row: ARG's 153 + Monster \
             Codex's 8 (SD-29 decisions.md §43's original 4 + SD-32 card-11 T2b lane's 4 Ratfolk \
             alternates, 2026-08-23) + APG's 1 (`Half-Orc ~ Plagueborn`) + Inner Sea \
             Races' 77 (68, §45, + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, \
             2026-08-22) + Horror Adventures' 41 (§47) + Core Essentials' 16 heritages \
             (§49) + SD-31 Epic 1-F2's 6 Bestiary 2 races' 48 (ARG's 42 + Inner Sea Races' 6 \
             actually-Alternate rows; re-derived by role, not by the raw per-book row counts \
             `ingest_race_traits` prints, which also include that batch's 8 `Unclassified` rows \
             -- ARG 3, Inner Sea Races 5) + SD-31-E6-F4-003's 19 (2026-08-16, ARG's own 6-race \
             chassis batch's real alternate-trait rows, minus Strix's Wing-Clipped-granted \
             Flight and Suli's Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield, \
             which are `FlagGranted` not `Alternate`) + SD31-E6-F4-006's 8 (2026-08-17, ARG's \
             own follow-on 4-race chassis batch's real alternate-trait rows). \
             Two loaded records are not \
             alternates at all and are correctly \
             outside this count: Monster Codex's `Oversized Goblin` and Inner Sea Races' \
             `Human ~ Tribalistic Languages`, both of which set no replace flag, so \
             `race_resolver::classify` leaves them `Unclassified`. Core Essentials' other 48 \
             records are outside it for a different and larger reason: they are \
             `TraitRole::FlagGranted` replacement rows, granted by whichever heritage the \
             player picks rather than chosen, so this book contributes 16 to this count and 64 \
             to the loaded corpus"
        );
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

    /// A vision trait's only `BONUS:` chain is `BONUS:VAR|HasRacialVision|1` —
    /// PCGen's internal "this race has a racial vision mode" flag, not a game
    /// quantity. Reading it as the row's display number rendered a meaningless
    /// `+1` beside "Dwarves can see in the dark up to 60 feet." The real
    /// quantity is the row's `VISION:Darkvision (60)` token.
    #[test]
    fn a_darkvision_row_shows_its_real_range_not_the_internal_flag_in_each_book() {
        let response = build_race_catalog();
        let row = |race_id: &str, trait_name: &str| -> &RaceCatalogEntryDto {
            response
                .entries
                .iter()
                .find(|e| e.race_id == race_id && e.trait_name == trait_name)
                .unwrap_or_else(|| panic!("{race_id} / {trait_name} must be a catalog row"))
        };

        // Core Rulebook. `VISION:Darkvision (60)`, and the DESC: says 60 feet.
        let dwarf = row("Dwarf", "Darkvision");
        assert_eq!(dwarf.book, BOOK_CRB);
        assert_eq!(dwarf.value, 60, "was 1, from BONUS:VAR|HasRacialVision|1");
        assert_eq!(dwarf.detail, "Dwarves can see in the dark up to 60 feet.");

        // Bestiary 1. Same defect, same fix, a different book's corpus files.
        let aasimar = row("Aasimar", "Darkvision");
        assert_eq!(aasimar.book, BOOK_B1);
        assert_eq!(aasimar.value, 60, "was 1, from BONUS:VAR|HasRacialVision|1");

        // Bestiary 1's two longer-ranged cases, so the fix is reading each
        // row's own token rather than hardcoding 60.
        assert_eq!(row("Drow", "Darkvision").value, 120);
        assert_eq!(row("Duergar", "Superior Darkvision").value, 120);
        // `VISION:Darkvision (120)` plus `VISION:Low-Light Vision`: one
        // distinct range across both tokens, so that range is the reading.
        assert_eq!(row("Svirfneblin", "Senses").value, 120);
    }

    /// Low-light vision declares no range at all (`VISION:Low-Light Vision`),
    /// so there is no honest number to show. It must show none — the frontend
    /// renders no badge at `0` — rather than fall back to the internal flag.
    #[test]
    fn a_low_light_vision_row_shows_no_number_in_each_book() {
        let response = build_race_catalog();
        let value = |race_id: &str, trait_name: &str| -> i16 {
            response
                .entries
                .iter()
                .find(|e| e.race_id == race_id && e.trait_name == trait_name)
                .unwrap_or_else(|| panic!("{race_id} / {trait_name} must be a catalog row"))
                .value
        };
        // Core Rulebook.
        assert_eq!(value("Elf", "Low-Light Vision"), 0);
        assert_eq!(value("Gnome", "Low-Light Vision"), 0);
        assert_eq!(value("HalfElf", "Low-Light Vision"), 0);
        // Bestiary 1.
        assert_eq!(value("Merfolk", "Low-Light Vision"), 0);
        assert_eq!(value("Tengu", "Senses"), 0);
    }

    /// The class of defect, not the one instance: **no** served row may take
    /// its display number from a chain that only writes an internal PCGen
    /// state flag. Swept across all 173 rows so a newly ingested book cannot
    /// reintroduce it silently.
    #[test]
    fn no_row_takes_its_display_value_from_an_internal_flag_chain() {
        let corpus = race_corpus().as_ref().expect("race corpus loads in a source checkout");
        let mut offenders: Vec<String> = Vec::new();
        for race_key in corpus.race_keys() {
            let Some(race) = corpus.resolve(race_key, &[]) else { continue };
            for resolved in &race.traits {
                let flag_only = !resolved.raw_bonus_chains.is_empty()
                    && resolved.raw_bonus_chains.iter().all(is_internal_flag_chain);
                if !flag_only {
                    continue;
                }
                // A flag-only row may still carry a real number, but only from
                // a non-`BONUS:` token (`VISION:`, `MOVE:`) — never from the
                // flag itself.
                let honest = declared_vision_range_ft(resolved)
                    .or_else(|| resolved.declared_walk_speed_ft())
                    .unwrap_or(0);
                let shown = i32::from(display_value(resolved));
                if shown != honest {
                    offenders.push(format!("{}/{} shows {shown}, honest {honest}", race.name, resolved.name));
                }
            }
        }
        assert!(offenders.is_empty(), "internal-flag values still reaching the UI: {offenders:?}");
    }

    /// The exact rows this correction moves, and to what. Derived by running
    /// the catalog, not quoted: 19 rows, every one of which previously showed
    /// `+1` off an internal flag. 16 are vision rows (each now showing its
    /// real `VISION:` range, or nothing when it declares none); 3 are the
    /// other internal flags the survey found.
    #[test]
    fn the_rows_the_internal_flag_correction_changes_are_pinned() {
        let response = build_race_catalog();
        let expected: &[(&str, &str, i16)] = &[
            // 16 vision rows, all previously +1 off BONUS:VAR|HasRacialVision|1.
            ("Dwarf", "Darkvision", 60),
            ("Elf", "Low-Light Vision", 0),
            ("Gnome", "Low-Light Vision", 0),
            ("HalfElf", "Low-Light Vision", 0),
            ("HalfOrc", "Darkvision", 60),
            ("Aasimar", "Darkvision", 60),
            ("Drow", "Darkvision", 120),
            ("Duergar", "Superior Darkvision", 120),
            ("Goblin", "Darkvision", 60),
            ("Hobgoblin", "Darkvision", 60),
            ("Kobold", "Darkvision", 60),
            ("Merfolk", "Low-Light Vision", 0),
            ("Orc", "Darkvision", 60),
            ("Svirfneblin", "Senses", 120),
            ("Tengu", "Senses", 0),
            ("Tiefling", "Darkvision", 60),
            // 3 non-vision internal flags found by the same survey.
            // BONUS:VAR|UMR_LightBlindness_SpecificDesc|1|TYPE=Boolean
            ("Drow", "Light Blindness", 0),
            // BONUS:VAR|CantBeTripped|1|TYPE=Boolean
            ("Merfolk", "Legless", 0),
            // BONUS:VAR|BastardSwordExoticUse,KatanaExoticUse|1
            ("Tengu", "Swordtrained", 0),
        ];
        assert_eq!(expected.len(), 19);
        for (race_id, trait_name, value) in expected {
            let entry = response
                .entries
                .iter()
                .find(|e| e.race_id == *race_id && e.trait_name == *trait_name)
                .unwrap_or_else(|| panic!("{race_id} / {trait_name} must be a catalog row"));
            assert_eq!(entry.value, *value, "{race_id} / {trait_name}");
        }
    }

    /// The neighbouring readings the survey deliberately left alone: these are
    /// real game quantities that happen to be written through `BONUS:VAR`, and
    /// a rule that swallowed them would be a worse defect than the one fixed.
    #[test]
    fn genuine_bonus_var_quantities_are_untouched() {
        let response = build_race_catalog();
        let value = |race_id: &str, trait_name: &str| -> i16 {
            response
                .entries
                .iter()
                .find(|e| e.race_id == race_id && e.trait_name == trait_name)
                .unwrap_or_else(|| panic!("{race_id} / {trait_name} must be a catalog row"))
                .value
        };
        assert_eq!(value("Dwarf", "Stonecunning"), 2);
        assert_eq!(value("Dwarf", "Defensive Training"), 4);
        assert_eq!(value("Elf", "Keen Senses"), 2);
        assert_eq!(value("Gnome", "Hatred"), 1);
        assert_eq!(value("HalfOrc", "Orc Ferocity"), 1);
        assert_eq!(value("Halfling", "Halfling Luck"), 1);
        assert_eq!(value("Kobold", "Armor"), 1);
        assert_eq!(value("Tiefling", "Fiendish Resistance"), 5);
        // Its `BONUS:DC|SCHOOL.Illusion|1` survives while the
        // `RacialSLA_Nondetection_Constant` flag beside it is discarded.
        assert_eq!(value("Svirfneblin", "Svirfneblin Magic"), 1);
        // `MOVE:` readings are unaffected by the change.
        assert_eq!(value("Dwarf", "Slow and Steady"), 20);
        assert_eq!(value("Tengu", "Normal Speed"), 30);
        assert_eq!(value("Merfolk", "Slow Speed"), 5);
    }

    #[test]
    fn flag_shaped_variable_names_are_recognized_and_magnitude_names_are_not() {
        // Recognized: PCGen's boolean conventions, each backed by a real
        // corpus row cited in `is_internal_flag_chain`'s doc comment.
        assert!(variable_name_is_flag_shaped("HasRacialVision"));
        assert!(variable_name_is_flag_shaped("IsAquatic"));
        assert!(variable_name_is_flag_shaped("SomeThingFlag"));
        assert!(variable_name_is_flag_shaped("BastardSwordExoticUse"));
        // Not recognized: real magnitudes, including ones that merely start
        // with the same letters.
        assert!(!variable_name_is_flag_shaped("Hasted_Bonus"));
        assert!(!variable_name_is_flag_shaped("Island_Bonus"));
        assert!(!variable_name_is_flag_shaped("KeenSensesBonus"));
        assert!(!variable_name_is_flag_shaped("Orc_OrcFerocity_Times"));
        assert!(!variable_name_is_flag_shaped("AC_Natural_Armor"));
        assert!(!variable_name_is_flag_shaped(""));
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
