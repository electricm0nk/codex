//! Corpus ingest diagnostic (Criterion 5.1, Epic 5 sketch scope).
//!
//! Reports the REAL ingested state of every book landed in `rules_tables` —
//! `RuleSetId::{Crb,Apg,Acg,Bestiary1,Arg,Pu}` — by counting the actual data
//! structures compiled into this binary (`ClassId::ALL`, `SPELL_LIST`,
//! `equipment_tables()`, ...). This is the same "book-partitioned table
//! store" `docs/architecture/rules-data-tables.md` describes; nothing here
//! is fixture/hand-guessed data — every count is read from the real
//! landed tables, so a future book that ships zero content, or a book that
//! loses a class roster to a regression, changes this diagnostic's output
//! without anyone touching this file.
//!
//! # SD-27 (2026-07-31): the panel was claiming two ingested books did not exist
//!
//! This module reported four books — `crb`, `apg`, `acg`, `beastiary1` —
//! long after `advanced_race_guide` and `pathfinder_unchained` landed as
//! real `rules_tables` books with real records (ARG: 200 equipment, 187
//! feats, 93 spells, 156 racial traits; PU: 4 classes, 64 class features, 42
//! equipment modifiers, 17 feats). Because the panel's own caption reads
//! "every rule book landed in `rules_tables`", a tester reading that screen
//! would correctly conclude, from a truthful-sounding caption, that two
//! ingested books were missing.
//!
//! Both are now reported, and
//! `tests::every_book_landed_in_rules_tables_is_reported` derives the
//! expected book set by **reading `src/rules_core/rules_tables/`**, so book
//! seven fails this module's own test suite until it is added here. That
//! guard, not the one-time correction, is the fix.
//!
//! # SD-27 (2026-07-31): the panel could not tell you which book a race came from
//!
//! `races` was a Core Rulebook row only, read from `RaceId::ALL`, because that
//! is the only race roster with a `rules_tables` representation. Bestiary 1
//! showed `monsters: 41` and no race count whatsoever — while 18 races were
//! creatable in the app and **11 of them were Bestiary 1's**. Read literally,
//! the panel said Bestiary 1 shipped no playable race.
//!
//! `races` is now reported per book, derived from the same on-disk race corpus
//! the race catalog and the character-creation roster read, so the panel and
//! the creation screen cannot disagree about what exists. See
//! [`race_counts_by_diagnostic_book`] for why that one row is corpus-derived
//! rather than table-derived, and what it does when the corpus is unreachable.
//!
//! That does widen the caption's "landed in `rules_tables`" boundary by
//! exactly one content kind, deliberately and only where the alternative was a
//! false statement: a `rules_tables`-only race count cannot be honest, because
//! for eleven of the eighteen races there is no `rules_tables` row to count.
//!
//! ## What this panel is still not counting, stated rather than left implicit
//!
//! ARG's **156 alternate/default racial-trait records are not a row here**,
//! and that is a boundary, not an oversight. (Distinct from the `races` row
//! added above: ARG contributes 0 *races*, which is reported, and 156 racial
//! *traits*, which are not.) They have no `rules_tables`
//! module: they were ingested straight to
//! `data/corpus/advanced_race_guide/race_trait/` and are read at runtime by
//! `race_resolver::load_race_corpus` (the path `race_catalog.rs` and
//! `race_trait_picker.rs` use to put them on screen). This panel's own
//! caption — "every rule book landed in `rules_tables`" — is what bounds it,
//! and folding a corpus-JSON-only content kind into a `rules_tables` count
//! would make that caption false in the other direction.
//!
//! They are accounted for elsewhere, so nothing is hidden: their compliance
//! count is in `data/corpus/advanced_race_guide/LICENSE.json` (649 records as
//! of SD-29 Epic 7 round 9, which added the book's 14 companion records to the
//! 635 it carried before; the artifact states the number and
//! `tests/sd27_book_license_record_counts.rs` derives it from the files on
//! disk, so neither this comment nor that field is the source of truth), and
//! every one of
//! ARG's 153 *alternate* traits reaches the player through
//! `list_alternate_racial_traits`.
//!
//! **That open finding is closed.** It read: of those 156 records, 3 reach no
//! player surface at all — `Feral ~ Languages` and
//! `Scion of Humanity ~ Languages` (`TraitRole::Unclassified`: no gate the
//! resolver can read) and `Saltbeard ~ Dwarf ~ Greed`
//! (`TraitRole::FlagGranted`), all three dropped by
//! `race_trait_picker::build_menu`, which filters to `Default | Alternate`.
//! All three are now `TraitRole::FlagGranted` and reach the player through
//! `resolve_race_alternate_selection`'s applied rows, which
//! AlternateTraitPicker.tsx renders — the menu is still `Default | Alternate`,
//! and it was never the only surface. See `reach_gate::race_traits_reach` and
//! `tests/sd27_ability_automatic_granted_race_traits.rs`.
//!
//! **Sketch-only boundary (per `cycles/5_1.md`):** exactly four fields —
//! `book_id`, `status`, `last_ingested_at`, `content_kind_counts`. SD-26
//! fans out the full status table + flags + ETA once the JSON ingest cache
//! lands; anything beyond these four fields belongs there, not here.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use codex::rules_core::rules_tables::acg::{self, AcgClassId};
use codex::rules_core::rules_tables::adventurers_guide as ag;
use codex::rules_core::rules_tables::advanced_race_guide as arg;
use codex::rules_core::rules_tables::apg::{self, ApgClassId};
use codex::rules_core::rules_tables::beastiary1::MonsterId;
use codex::rules_core::rules_tables::inner_sea_faiths as isf;
use codex::rules_core::rules_tables::inner_sea_magic as ism;
use codex::rules_core::rules_tables::inner_sea_races as isr;
use codex::rules_core::rules_tables::inner_sea_temples as istem;
use codex::rules_core::rules_tables::mythic_adventures as ma;
use codex::rules_core::rules_tables::ultimate_magic_wordsofpower as umwop;
use codex::rules_core::rules_tables::crb::{
    class_tables::ClassId, equipment_tables as crb_equipment_tables, feats as crb_feats,
    race_tables::RaceId, spell_list as crb_spell_list,
};
use codex::rules_core::rules_tables::occult_adventures as oa;
use codex::rules_core::rules_tables::pathfinder_unchained as pu;
use codex::rules_core::rules_tables::ultimate_campaign as uca;
use codex::rules_core::rules_tables::ultimate_equipment as ue;
use codex::rules_core::rules_tables::ultimate_combat as uc;
use codex::rules_core::rules_tables::ultimate_magic as um;
use codex::rules_core::rules_tables::ultimate_psionics as upsi;
use codex::rules_core::rules_tables::ultimate_wilderness as uw;
use codex::rules_core::rules_tables::ultimate_intrigue as ui;

use crate::race_catalog::{book_code, build_race_catalog, RACE_CORPUS_BOOKS};

/// One book's real ingested-corpus status. Field set is deliberately
/// bounded to the sketch scope named above.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookIngestStatus {
    /// Lowercase book identifier, matching the `rules_tables` directory
    /// name (`"crb"`, `"apg"`, `"acg"`, `"beastiary1"`,
    /// `"advanced_race_guide"`, `"pathfinder_unchained"`). That the two sets
    /// are identical is asserted, not assumed — see
    /// `tests::every_book_landed_in_rules_tables_is_reported`.
    pub book_id: String,
    /// `"populated"` when this book's real content-kind counts sum to more
    /// than zero, `"empty"` otherwise — computed, never hand-set per book.
    pub status: String,
    /// RFC 3339 timestamp of the most recent git commit that touched this
    /// book's `rules_tables` directory, or `None` when git history isn't
    /// reachable (e.g. a packaged build with no `.git` checkout alongside
    /// it — the same graceful-degradation shape `build.rs`'s
    /// `git_short_sha` already uses for `CODEX_GIT_SHA`).
    pub last_ingested_at: Option<String>,
    /// Real per-kind record counts (e.g. `"classes"`, `"spells"`), read
    /// directly from the book's own landed tables.
    pub content_kind_counts: BTreeMap<String, u32>,
}

/// Every Bestiary 1 monster the real `MonsterId` enum declares today.
///
/// This used to be a hand-maintained duplicate list (`beastiary1::mod.rs`
/// had no public `ALL`/count constant, unlike
/// `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL`). SD-26 Epic 3
/// Criterion 3.4 (`decisions.md §11.6`) added the real
/// `MonsterId::ALL` constant so this diagnostic and the JSON-cache
/// generator (`codex::rules_core::cache_gen::beastiary1`) both read the
/// same single source of truth instead of each maintaining their own
/// copy of this list a second/third time.
const ALL_BESTIARY1_MONSTERS: &[MonsterId] = MonsterId::ALL;

/// Real creatable-race counts, per book, derived from the same on-disk race
/// corpus the race catalog and character creation both read.
///
/// # Why this is not a `rules_tables` count like every other row here
///
/// It cannot be. Only the Core Rulebook's seven races have a `rules_tables`
/// representation (`RaceId::ALL`); Bestiary 1's eleven were ingested straight
/// to `data/corpus/beastiary/race/` and are read at runtime by
/// `race_resolver::load_race_corpus`, which is the path `race_catalog.rs` and
/// `character_hub`'s creation roster use to put them on screen. So this panel
/// reported CRB `races: 7` and, for Bestiary 1, `monsters: 41` and no race
/// count at all — while 18 races were creatable in the app and 11 of them were
/// Bestiary 1's. A tester reading the panel would conclude Bestiary 1 shipped
/// no playable race.
///
/// Reading the live corpus rather than `RaceId::ALL` also makes the two agree
/// by construction rather than by coincidence;
/// `crb_race_count_agrees_with_the_compiled_race_id_table` pins that.
///
/// Every book in `RACE_CORPUS_BOOKS` gets an entry here, including a genuine
/// zero for ARG (`decisions.md §25.2` — all 37 rows in `arg_races.lst` are
/// `.MOD` reprints of chassis owned by other books). `book_status` is what
/// decides not to *render* a zero, so this map stays a straight measurement
/// and the presentation rule lives in one place.
///
/// Returns an empty map when the corpus cannot be loaded at all (a packaged
/// build with no `data/corpus/` alongside it). Callers then omit the `races`
/// row rather than report a fabricated zero — the same graceful-degradation
/// shape `last_ingested_at` already uses for an unreachable git history.
fn race_counts_by_diagnostic_book() -> BTreeMap<String, u32> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let catalog = build_race_catalog();
    if catalog.entries.is_empty() {
        // No on-disk corpus (a packaged build). The Core Rulebook's seven
        // races are additionally compiled into this binary as `RaceId::ALL`,
        // so that one row is still answerable and is answered; the books whose
        // races exist only as corpus JSON honestly report nothing rather than
        // a fabricated zero.
        counts.insert("crb".to_string(), RaceId::ALL.len() as u32);
        return counts;
    }
    for book in RACE_CORPUS_BOOKS {
        counts.insert(diagnostic_book_id(&book_code(book)), 0);
    }
    let mut seen: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for entry in &catalog.entries {
        seen.entry(diagnostic_book_id(&entry.book))
            .or_default()
            .insert(entry.race_id.clone());
    }
    for (book_id, races) in seen {
        counts.insert(book_id, races.len() as u32);
    }
    counts
}

/// Maps a race-catalog wire book code (`"CRB"`, `"B1"`, `"ARG"`) onto this
/// panel's own book identifier. An unrecognized code passes through verbatim
/// rather than being silently re-attributed, so a newly ingested book shows up
/// as itself and trips
/// `tests::every_book_landed_in_rules_tables_is_reported` instead of quietly
/// landing in the wrong row.
fn diagnostic_book_id(race_catalog_book_code: &str) -> String {
    match race_catalog_book_code {
        "CRB" => "crb".to_string(),
        "B1" => "beastiary1".to_string(),
        "ARG" => "advanced_race_guide".to_string(),
        // SD-31 Epic 1-F2 (2026-08-15). Without this arm the race count for
        // Bestiary 2's 6 new races is computed under key "B2" but
        // `book_status("bestiary_2", ...)` looks it up by "bestiary_2" --
        // same mismatch class `diagnostic_book_id`'s own doc comment names
        // for CRB/B1/ARG, just not yet hit for this book.
        "B2" => "bestiary_2".to_string(),
        // Skinwalker follow-on batch (2026-08-15), same mismatch class.
        "B5" => "bestiary_5".to_string(),
        // SD-31 wave-24 integration cycle (2026-08-20), same mismatch class.
        "B6" => "bestiary_6".to_string(),
        other => other.to_string(),
    }
}

fn crb_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), ClassId::ALL.len() as u32);
    counts.insert(
        "feats".to_string(),
        crb_feats::feat_tables().len() as u32,
    );
    counts.insert(
        "spells".to_string(),
        crb_spell_list::SPELL_LIST.len() as u32,
    );
    counts.insert(
        "equipment".to_string(),
        crb_equipment_tables::equipment_tables().len() as u32,
    );
    // SD-29 Epic 7 round 8. Merged in rather than inserted by hand, so this
    // book's companion count is derived from the SAME registry every other
    // companion book's row reads. Round 7 found Ultimate Wilderness present in
    // this panel with a number that under-stated it threefold, because its row
    // inserted `feats` and stopped; a book whose row is hand-built drifts the
    // moment it gains a family. `crb` had five families before this round and
    // is the most likely book in the corpus to gain a sixth again.
    counts.extend(companion_book_counts("core_rulebook"));
    counts
}

/// SD-29 Epic 7 round 9 adds this book's `companion` family. The lookup key is
/// the CORPUS book `advanced_players_guide`, not the engine module `apg` this
/// function is named for — `companion_chassis::COMPANION_BOOKS` keys on the
/// `data/corpus/` spelling, the same split `beastiary1_counts` documents.
fn apg_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), ApgClassId::ALL.len() as u32);
    counts.insert("feats".to_string(), apg::feats::feat_tables().len() as u32);
    counts.insert("spells".to_string(), apg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        apg::equipment_tables::EQUIPMENT_TABLE.len() as u32,
    );
    counts.extend(companion_book_counts("advanced_players_guide"));
    counts
}

fn acg_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("classes".to_string(), AcgClassId::ALL.len() as u32);
    counts.insert("feats".to_string(), acg::feats::feat_tables().len() as u32);
    counts.insert("spells".to_string(), acg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        acg::equipment_tables::equipment_tables().len() as u32,
    );
    counts
}

/// One chassis book's two record families, read from its own live tables
/// (SD-29 Epic 5). `monster_abilities` is a kind this panel first reported
/// with the Bonus Bestiary pilot -- Bestiary 1 ingests monsters only.
///
/// Read through `monster_chassis::monster_book` rather than from a named
/// module, so a book registered in the chassis but forgotten here fails on
/// its own row instead of reporting silently absent. The panel's own
/// fail-closed test already treats an unreported book as an un-ingested one.
fn chassis_book_counts(corpus_book: &str) -> BTreeMap<String, u32> {
    use codex::rules_core::rules_tables::monster_chassis;
    let table = monster_chassis::monster_book(corpus_book).unwrap_or_else(|| {
        panic!("{corpus_book} is not registered in monster_chassis::MONSTER_BOOKS")
    });
    let mut counts = BTreeMap::new();
    counts.insert("monsters".to_string(), table.monsters.len() as u32);
    counts.insert(
        "monster_abilities".to_string(),
        table.monster_abilities.len() as u32,
    );
    counts
}

/// The `companion` counts a book contributes, read off the live companion
/// chassis exactly as [`chassis_book_counts`] reads the monster one.
///
/// **Written independently and identically by two lanes, and kept once.** The
/// companion lane added it here with its tables (`decisions.md §48`); the
/// race-trait lane's round 4 added the same function for the same reason
/// (`§49`) because the three books `bac2f569` landed left
/// `every_book_landed_in_rules_tables_is_reported` — the drift guard written
/// for exactly this defect — RED on `origin/tranche/9` until whichever lane got
/// there first. Both reasons are kept because both are true, and the merge
/// unioned them rather than picking a side (`§46.6` rule 1).
///
/// Kept separate from [`chassis_book_counts`] rather than merged into it: a
/// book can be in either registry, both, or neither, and a single helper would
/// have to guess which. `monster_codex` is in both, which is exactly the case
/// that would have broken a merged one.
fn companion_book_counts(corpus_book: &str) -> BTreeMap<String, u32> {
    use codex::rules_core::rules_tables::companion_chassis;
    let table = companion_chassis::companion_book(corpus_book).unwrap_or_else(|| {
        panic!("{corpus_book} is not registered in companion_chassis::COMPANION_BOOKS")
    });
    // **One kind, not two**, matching `reach_gate::CORPUS_KIND_NAMES`'s
    // deliberate single `companion -> companions` entry and the corpus's own
    // single `data/corpus/<book>/companion/` directory, which holds creature
    // records and ability records side by side. Splitting them here would
    // invent a `companion_abilities` family the reach gate has no claim for,
    // and `every_ingested_family_is_accounted_for` would correctly demand one
    // -- an invented family is exactly the drift this diagnostic exists to
    // report, not to create. The sum is the on-disk record count, verified per
    // book: `find data/corpus/inner_sea_combat/companion -name '*.json' | wc -l`
    // -> 10 = 4 creatures + 6 abilities; inner_sea_intrigue 11 = 2 + 9;
    // horror_adventures 2 = 1 + 1; monster_codex 15 = 8 + 7.
    let mut counts = BTreeMap::new();
    counts.insert(
        "companions".to_string(),
        (table.companions.len() + table.companion_abilities.len()) as u32,
    );
    counts
}

/// A book carrying BOTH chassis registries' tables, merged into one row set.
///
/// `monster_codex` is the only such book today and the reason this exists: its
/// panel row has to state its monsters, its monster abilities AND its 15
/// companion units, and reporting only one registry would under-state a book
/// the tester is looking at.
fn monster_and_companion_book_counts(corpus_book: &str) -> BTreeMap<String, u32> {
    let mut counts = chassis_book_counts(corpus_book);
    counts.extend(companion_book_counts(corpus_book));
    counts
}

/// Bestiary 1's compiled families: its SD-22 monsters, plus — since SD-29
/// Epic 7 round 3 — its companion rows.
///
/// The companion half is looked up under `"beastiary"`, the `data/corpus/`
/// spelling `companion_chassis::COMPANION_BOOKS` keys on, while this book's
/// diagnostic id is `beastiary1`. Reporting only the monster half would
/// under-state the book by 59 records, the same defect
/// `monster_and_companion_book_counts` exists for.
/// Since SD-29 Epic 5 round 8 the monster half is TWO tables, not one
/// (`decisions.md §58.3`): `ALL_BESTIARY1_MONSTERS`' 46 hand-modelled blocks and
/// the chassis's 284. The panel states the book, so it states the sum — reporting
/// either alone under-states a book the tester is looking at by the size of the
/// other, the same defect `monster_and_companion_book_counts` exists for. The
/// chassis also contributes this book's first `monster_abilities` family.
fn beastiary1_counts() -> BTreeMap<String, u32> {
    let mut counts = chassis_book_counts("beastiary");
    let chassis_monsters = counts.get("monsters").copied().unwrap_or_default();
    counts.insert(
        "monsters".to_string(),
        ALL_BESTIARY1_MONSTERS.len() as u32 + chassis_monsters,
    );
    counts.extend(companion_book_counts("beastiary"));
    counts
}

/// Every Pathfinder Unchained class feature the four real feature tables
/// declare.
///
/// Summed rather than read from one constant because PU has no aggregate: the
/// records live in four per-class modules, two exposing `Feature::ALL` and two
/// a `features()` accessor. Summing the four live tables is what makes this a
/// derived count — a class whose feature table is emptied by a regression
/// changes this number without anyone editing it.
fn pu_class_feature_count() -> u32 {
    (pu::barbarian_features::features().len()
        + pu::monk_features::features().len()
        + pu::rogue_features::UnchainedRogueFeature::ALL.len()
        + pu::summoner_features::UnchainedSummonerFeature::ALL.len()) as u32
}

/// SD-29 Epic 7 round 9 adds this book's `companion` family, merged in rather
/// than hand-inserted for the reason `ultimate_wilderness_counts` records.
fn advanced_race_guide_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), arg::feats::feat_tables().len() as u32);
    counts.insert("spells".to_string(), arg::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        arg::equipment_tables::equipment_tables().len() as u32,
    );
    counts.extend(companion_book_counts("advanced_race_guide"));
    counts
}

fn pathfinder_unchained_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert(
        "classes".to_string(),
        pu::class_chassis::PuClassId::ALL.len() as u32,
    );
    counts.insert("class_features".to_string(), pu_class_feature_count());
    counts.insert("feats".to_string(), pu::feat_tables::feat_tables().len() as u32);
    counts.insert(
        "equipment".to_string(),
        pu::equipment_tables::equipment_tables().len() as u32,
    );
    counts
}

/// Ultimate Campaign: SD-28 Epic 13 (`epic-13-calibration`) cost
/// calibration book. 23 real corpus records, all `feats` -- see
/// `ultimate_campaign::feat_tables`'s own doc comment for the catalog and
/// its 3 `deferred-with-reason` records (still counted here: they are real
/// ingested rows, not stubs -- see that module's own doc comment).
fn ultimate_campaign_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uca::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Intrigue: SD-28 Epic 24 (`epic-24-ui-complete`) from-scratch
/// book ingest, slices 1-2. 104 feats, 101 spells, 98 equipment (91 + 7
/// equipmods) -- see `ultimate_intrigue::{feat_tables,spell_list,
/// equipment_tables}`'s own doc comments for each catalog. Remaining
/// record families (`class_feature`, races, etc.) are not yet ingested
/// and are honestly absent from this map rather than reported as a
/// fabricated zero. (Caught while wiring UE: this map was never updated
/// when slice 2 landed spell/equipment -- fixed here.)
fn ultimate_intrigue_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), ui::feat_tables::feat_tables().len() as u32);
    counts.insert("spells".to_string(), ui::spell_list::SPELL_LIST.len() as u32);
    counts.insert(
        "equipment".to_string(),
        (ui::equipment_tables::equipment_tables().len() + ui::equipment_tables::equipmod_tables().len()) as u32,
    );
    counts
}

/// Ultimate Equipment: SD-28 Epic 25 (`epic-25-ue-complete`) from-scratch
/// book ingest, first slice. 1,369 equipment + 180 equipment-modifier
/// records -- see `ultimate_equipment::equipment_tables`'s own doc
/// comment for the catalog and its collision-exclusion ruling.
fn ultimate_equipment_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert(
        "equipment".to_string(),
        (ue::equipment_tables::equipment_tables().len() + ue::equipment_tables::equipmod_tables().len()) as u32,
    );
    counts
}

/// Ultimate Wilderness: SD-28 Epic 26 (`epic-26-uw-complete`) from-scratch
/// book ingest, first slice. 135 feat records -- see
/// `ultimate_wilderness::feat_tables`'s own doc comment for the catalog.
/// SD-29 Epic 7 round 6 added this book's `companion` family and did NOT add it
/// here, so the panel reported Ultimate Wilderness's 135 feats and none of its
/// 327 companion records (`decisions.md §63.4`). Corrected in round 7, which
/// found it only because registering ITS book turned
/// `every_book_landed_in_rules_tables_is_reported` red — that test asks whether
/// a book appears at all, and Ultimate Wilderness already did.
fn ultimate_wilderness_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uw::feat_tables::feat_tables().len() as u32);
    counts.extend(companion_book_counts("ultimate_wilderness"));
    counts
}

/// Ultimate Combat: SD-28 Epic 27 (`epic-27-uc-complete`) from-scratch
/// book ingest, first slice. 263 feat records -- see
/// `ultimate_combat::feat_tables`'s own doc comment for the catalog.
fn ultimate_combat_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), uc::feat_tables::feat_tables().len() as u32);
    counts
}

/// Ultimate Magic: SD-28 Epic 28 (`epic-28-um-complete`) from-scratch
/// book ingest. 144 feat records -- see `ultimate_magic::feat_tables`'s
/// own doc comment for the catalog. SD-28-E15's second slice adds 26
/// equipment records (24 General + 2 ArmsArmor) -- see
/// `ultimate_magic::equipment_tables`'s own doc comment.
/// SD-29 Epic 7 round 9 adds this book's `companion` family, MERGED in via
/// `companion_book_counts` rather than hand-inserted — the drift round 7 found
/// in `ultimate_wilderness_counts` came from exactly the hand-built shape this
/// function otherwise has, and a book that already reports two families is the
/// most likely to gain a third silently.
fn ultimate_magic_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), um::feat_tables::feat_tables().len() as u32);
    counts.insert("equipment".to_string(), um::equipment_tables::equipment_tables().len() as u32);
    counts.extend(companion_book_counts("ultimate_magic"));
    counts
}

/// Occult Adventures: SD31-E6-F2-003 -- this book's FIRST compiled record
/// family of any kind (no prior lane has ingested any of its content). 144
/// base spell records -- see `occult_adventures::spell_list`'s own doc
/// comment for the catalog and the 328-unit `mod_only` class-widening
/// residue it deliberately does not cover.
///
/// `decisions.md §27b` — EVERYTHING: this book's second compiled family,
/// `monster`/`monster_ability` (`monster_chassis::MONSTER_BOOKS` now lists
/// `"occult_adventures"`, same shape `mythic_adventures_counts` above
/// chains), overturns the repeatedly-reconfirmed "correctly out of scope"
/// disposition for its 5 `monster_ability` units -- a reachability finding
/// about a negated `PRECAMPAIGN` gate, not an ingest exemption. Chained via
/// `chassis_book_counts` rather than a literal insert, matching the same
/// convention every other chassis-registered book on this panel uses.
fn occult_adventures_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), oa::spell_list::SPELL_LIST.len() as u32);
    counts.extend(
        chassis_book_counts("occult_adventures")
            .into_iter()
            .filter(|(_, count)| *count > 0),
    );
    counts
}

/// Adventurer's Guide: SD-31 wave-29 (`lane5-book-onboard` lane) -- this
/// book's FIRST compiled record family of any kind. 45 base spell records
/// -- see `adventurers_guide::spell_list`'s own doc comment.
fn adventurers_guide_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), ag::spell_list::SPELL_LIST.len() as u32);
    counts
}

/// Inner Sea Faiths: SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled record family of any kind. 2 base spell records (of 3
/// raw base declarations; a genuine intra-book reprint dedups to 1) -- see
/// `inner_sea_faiths::spell_list`'s own doc comment.
fn inner_sea_faiths_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), isf::spell_list::SPELL_LIST.len() as u32);
    counts
}

/// Inner Sea Magic: SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled record family of any kind. 34 base spell records -- see
/// `inner_sea_magic::spell_list`'s own doc comment. Its 218 `class_feature`
/// records are corpus-JSON-only (no `rules_tables` module carries
/// class_feature data for this book, the same shape ARG's own
/// corpus-JSON-only class_feature records document above), so they are
/// deliberately not summed into this row -- this panel counts compiled
/// `rules_tables` families only.
fn inner_sea_magic_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), ism::spell_list::SPELL_LIST.len() as u32);
    counts
}

/// Inner Sea Temples: SD-32 Gate 0 book-onboarding precondition
/// (`gate-0-book-onboarding-precondition`, AT-32-G0-003) -- this book's
/// FIRST compiled record family of any kind. 21 base spell records -- see
/// `inner_sea_temples::spell_list`'s own doc comment.
fn inner_sea_temples_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), istem::spell_list::SPELL_LIST.len() as u32);
    counts
}

/// Inner Sea Races: this book's FIRST compiled record family of any kind --
/// 34 base spell records (`inner_sea_races::spell_list::SPELL_LIST`).
/// Registered here per SD-32's stale-assertion fix
/// (`the_two_ingested_books_totals_reconcile_with_their_license_artifacts`'s
/// sibling drift guard, `every_book_landed_in_rules_tables_is_reported`):
/// the module landed in `rules_tables` with a real compiled table and no
/// panel row, which reads to a tester as an un-ingested book.
fn inner_sea_races_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), isr::spell_list::SPELL_LIST.len() as u32);
    counts
}

/// Mythic Adventures: carries BOTH a compiled spell list and the monster
/// chassis registry (`monster_chassis::MONSTER_BOOKS` already lists
/// `"mythic_adventures"`), so this row chains [`chassis_book_counts`] the
/// same way [`monster_and_companion_book_counts`] chains a second registry
/// onto a book's own family, rather than under-reporting by one family.
///
/// **This book is one of the "zero-monster" books**
/// (`monster_chassis.rs`'s own `§17a` re-derive comment): it declares 21
/// `monster_abilities` and genuinely zero `monsters` -- no monster stat
/// block owns them. `chassis_book_counts` would insert a literal
/// `monsters: 0` row, which `every_book_is_populated_with_real_nonzero_
/// counts` correctly refuses (this panel's own contract is that a
/// content-kind row means real records). Dropped the same way `book_status`
/// already drops a genuine zero `races` row, rather than reporting a count
/// that means something different from every other row on the panel.
fn mythic_adventures_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("spells".to_string(), ma::spell_list::SPELL_LIST.len() as u32);
    counts.extend(
        chassis_book_counts("mythic_adventures")
            .into_iter()
            .filter(|(_, count)| *count > 0),
    );
    counts
}

/// Ultimate Magic (Words of Power): a second `rules_tables` module for the
/// `ultimate_magic` book, covering its Words of Power spell variant. This
/// book directory is distinct from `ultimate_magic` itself (a genuinely
/// separate `src/rules_core/rules_tables/` directory,
/// `every_book_landed_in_rules_tables_is_reported`'s drift guard walks
/// directories, not book titles), so it gets its own row rather than being
/// folded into `ultimate_magic_counts`.
fn ultimate_magic_wordsofpower_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert(
        "spells".to_string(),
        umwop::spell_list::SPELL_LIST.len() as u32,
    );
    counts
}

/// Ultimate Psionics: SD-28 Epic 29 (`epic-29-upsi-complete`) from-scratch
/// book ingest, and the last Ultimate book. 221 feat records -- see
/// `ultimate_psionics::feat_tables`'s own doc comment for the catalog and
/// the license-posture check. SD-28-E15's second slice adds 552 equipment
/// records (326 equipment + 226 equipmods) -- see
/// `ultimate_psionics::equipment_tables`'s own doc comment.
///
/// SD-29 Epic 5 extend, round 10 adds this book's monster families by CHAINING
/// [`chassis_book_counts`] rather than by inserting two more literals. That is
/// `decisions.md §63.4`'s finding applied prospectively: `ultimate_wilderness`
/// had 327 companion records absent from this panel from the day they landed,
/// because its per-book counts function listed only the families that existed
/// when it was written and the row above it was the only place a later lane
/// looked. Extending the map keeps every family this book has, whoever added
/// it.
fn ultimate_psionics_counts() -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    counts.insert("feats".to_string(), upsi::feat_tables::feat_tables().len() as u32);
    counts.insert(
        "equipment".to_string(),
        (upsi::equipment_tables::equipment_tables().len()
            + upsi::equipment_tables::equipmod_tables().len()) as u32,
    );
    counts.extend(chassis_book_counts("ultimate_psionics"));
    counts
}

/// Repo root, derived from the crate's own compile-time manifest
/// directory (`apps/desktop/src-tauri`) rather than the process's current
/// working directory, which Tauri does not guarantee.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Live, read-only count of every real record file under a book's
/// `data/corpus/<book>/` directory: every `*.json` file except
/// `LICENSE.json` itself, skipping any `_`-prefixed directory (the same
/// non-content-storage convention `src/bin/gen_book_cache.rs`'s own
/// `count_on_disk_records` documents -- `_parity/` holds build/test
/// fixtures, not licensed content records).
///
/// # Why this walks the filesystem instead of reading `LICENSE.json`
///
/// `the_two_ingested_books_totals_reconcile_with_their_license_artifacts`
/// used to trust `LICENSE.json`'s own `records_processed` field as its
/// independent ground truth. That field is itself a book-wide on-disk
/// snapshot **taken at whatever moment some lane last ran a generator for
/// that book**, and this cycle proved live that it goes stale the moment a
/// SIBLING lane adds corpus-JSON-only content through a different ingest
/// path afterward: `advanced_race_guide`'s `feat`/`equipment`/`companion`
/// directories gained 67 new records from `1410424cf3` ("close feat+spell
/// no_record via existing corpus-cache generators", `decisions.md §20`)
/// after `LICENSE.json` was last written, so the field read 2157 while the
/// true on-disk count was already 2205.
///
/// Re-running `gen_book_cache --bin advanced_race_guide` to refresh that
/// field is **not safe**: reproduced live this cycle and reverted
/// (`git status --porcelain` before/after confirmed), that generator
/// deletes every file under `feat/`/`equipment`/etc it did not itself just
/// write -- so "refreshing the count" would have destroyed the exact 48
/// `feat` records `1410424cf3` legitimately added. That is
/// `workflow-instruction.md`'s footgun 2 ("a bundled generator staged
/// deletion of files it did not own"), caught here before it was
/// committed. Logged: `scripts/retro.py incident` (recurrence_key
/// `generator-orphans-unowned-files-on-directory-sync`; coordinates only,
/// no PI content in this path).
///
/// A live walk cannot go stale between two lanes' commits the way a
/// generated snapshot field can, and it never writes anything, so it
/// carries none of that risk. `data/corpus` is otherwise off-limits to
/// hand-editing (`AGENTS.md`); this function only ever reads.
fn live_on_disk_record_count(book_corpus_dir: &Path) -> u32 {
    fn walk(dir: &Path, count: &mut u32) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let is_internal = path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .is_some_and(|n| n.starts_with('_'));
                if !is_internal {
                    walk(&path, count);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                *count += 1;
            }
        }
    }
    let mut count = 0;
    walk(book_corpus_dir, &mut count);
    count
}

/// RFC 3339 commit date of the most recent commit touching
/// `repo_relative_dir`, or `None` when git isn't reachable (no `.git`
/// checkout, `git` not on `PATH`, or the path has no history — e.g. a
/// packaged production build, which ships the compiled binary only).
fn last_commit_iso_date(repo_relative_dir: &str) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root())
        .args(["log", "-1", "--format=%cI", "--", repo_relative_dir])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// The race count is passed in rather than looked up per book so the whole
/// panel reads the corpus once, and so this stays a pure function the tests can
/// drive with a known map.
fn book_status(
    book_id: &str,
    repo_relative_dir: &str,
    mut counts: BTreeMap<String, u32>,
    race_counts: &BTreeMap<String, u32>,
) -> BookIngestStatus {
    // A zero is dropped rather than reported. This panel's own contract —
    // pinned by `every_book_is_populated_with_real_nonzero_counts` — is that a
    // content-kind row means real records; ARG genuinely declares zero races
    // (`decisions.md §25.2`), and a `races: 0` row would be the one row on the
    // screen that means something different from all the others.
    if let Some(races) = race_counts.get(book_id).filter(|races| **races > 0) {
        counts.insert("races".to_string(), *races);
    }
    let total: u32 = counts.values().sum();
    BookIngestStatus {
        book_id: book_id.to_string(),
        status: if total > 0 { "populated" } else { "empty" }.to_string(),
        last_ingested_at: last_commit_iso_date(repo_relative_dir),
        content_kind_counts: counts,
    }
}

/// Build the full diagnostic response. A thin, testable wrapper behind the
/// Tauri command below, mirroring `class_catalog::build_class_catalog`'s
/// command/pure-fn split.
pub fn build_corpus_ingest_diagnostic() -> Vec<BookIngestStatus> {
    let races = race_counts_by_diagnostic_book();
    vec![
        book_status("crb", "src/rules_core/rules_tables/crb", crb_counts(), &races),
        book_status("apg", "src/rules_core/rules_tables/apg", apg_counts(), &races),
        book_status("acg", "src/rules_core/rules_tables/acg", acg_counts(), &races),
        book_status(
            "beastiary1",
            "src/rules_core/rules_tables/beastiary1",
            beastiary1_counts(),
            &races,
        ),
        book_status(
            "bonus_bestiary",
            "src/rules_core/rules_tables/bonus_bestiary",
            chassis_book_counts("bonus_bestiary"),
            &races,
        ),
        book_status(
            "monster_codex",
            "src/rules_core/rules_tables/monster_codex",
            monster_and_companion_book_counts("monster_codex"),
            &races,
        ),
        // SD-29 Epic 7 (companion lane). Three books whose only compiled family
        // is `companion`; `inner_sea_combat` is the first book in this repo
        // whose ONLY ingested family is companions at all.
        //
        // **Two lanes added these three rows independently and the merge kept
        // both copies** (`decisions.md §46.6` rule 1: a non-conflicting hunk is
        // exactly what an auto-merge duplicates without saying so). One copy is
        // kept. The race-trait lane's version noted that `monster_codex`'s
        // companion counts were NOT merged into its own row above; that gap is
        // closed here by `monster_and_companion_book_counts`, so the note is
        // corrected rather than dropped.
        book_status(
            "inner_sea_combat",
            "src/rules_core/rules_tables/inner_sea_combat",
            companion_book_counts("inner_sea_combat"),
            &races,
        ),
        book_status(
            "inner_sea_intrigue",
            "src/rules_core/rules_tables/inner_sea_intrigue",
            companion_book_counts("inner_sea_intrigue"),
            &races,
        ),
        // SD-29 Epic 5 extend, FINAL round, changed this row from
        // `companion_book_counts` to the merged helper: this book stopped being
        // companion-ONLY when the monster lane registered its 3 monsters and 6
        // abilities. Reporting only the companion registry would under-state a
        // book the tester is looking at, which is `decisions.md §63.4`'s finding
        // -- and that finding is about THIS function, so leaving the row alone
        // would have repeated it in the file that records it.
        book_status(
            "horror_adventures",
            "src/rules_core/rules_tables/horror_adventures",
            monster_and_companion_book_counts("horror_adventures"),
            &races,
        ),
        // SD-29 Epic 7 round 2 (companion lane, extend). Three more books whose
        // only compiled family is `companion`. This panel's caption says it
        // shows every rule book landed in `rules_tables`, so a book missing here
        // reads to a tester as an un-ingested book — the defect
        // `every_book_landed_in_rules_tables_is_reported` caught for round 1's
        // three, and the reason these rows are written in the same commit that
        // registers the books.
        book_status(
            "bestiary_5",
            "src/rules_core/rules_tables/bestiary_5",
            companion_book_counts("bestiary_5"),
            &races,
        ),
        book_status(
            "bestiary_6",
            "src/rules_core/rules_tables/bestiary_6",
            companion_book_counts("bestiary_6"),
            &races,
        ),
        // SD-29 Epic 5 extend, round 4 turned this row into the SECOND book
        // carrying both registries: B2's 16 familiars were already here, and the
        // same book now compiles 316 monsters + 402 monster abilities. Reporting
        // only the companion half would under-state it by 718 records, which is
        // the exact defect `monster_and_companion_book_counts` was written for
        // when `monster_codex` became the first such book.
        book_status(
            "bestiary_2",
            "src/rules_core/rules_tables/bestiary_2",
            monster_and_companion_book_counts("bestiary_2"),
            &races,
        ),
        // SD-29 Epic 5 extend, round 5. Unlike the row above it, this book
        // carries the monster registry ONLY -- it contributes no companion
        // family -- so it reads through `chassis_book_counts` like the
        // monster-only books below.
        book_status(
            "bestiary_3",
            "src/rules_core/rules_tables/bestiary_3",
            chassis_book_counts("bestiary_3"),
            &races,
        ),
        // SD-29 Epic 5 extend, round 6. Monster registry only, like the row
        // above it.
        book_status(
            "bestiary_4",
            "src/rules_core/rules_tables/bestiary_4",
            chassis_book_counts("bestiary_4"),
            &races,
        ),
        // SD-29 Epic 5 extend, round 7. Monster registry only, like the two
        // rows above it.
        book_status(
            "inner_sea_bestiary",
            "src/rules_core/rules_tables/inner_sea_bestiary",
            chassis_book_counts("inner_sea_bestiary"),
            &races,
        ),
        // SD-29 Epic 5 extend, round 9. Monster registry only, like the rows
        // above it.
        book_status(
            "inner_sea_gods",
            "src/rules_core/rules_tables/inner_sea_gods",
            chassis_book_counts("inner_sea_gods"),
            &races,
        ),
        // SD-29 Epic 7 round 9 gave this book a `companion` family beside its
        // monsters, so its row moves from `chassis_book_counts` to
        // `monster_and_companion_book_counts` — the function that exists
        // precisely because reporting one half of a two-chassis book under-states
        // it by the size of the other.
        book_status(
            "book_of_the_damned_volume_1",
            "src/rules_core/rules_tables/book_of_the_damned_volume_1",
            monster_and_companion_book_counts("book_of_the_damned_volume_1"),
            &races,
        ),
        book_status(
            "book_of_the_damned_volume_2",
            "src/rules_core/rules_tables/book_of_the_damned_volume_2",
            chassis_book_counts("book_of_the_damned_volume_2"),
            &races,
        ),
        book_status(
            "inner_sea_world_guide",
            "src/rules_core/rules_tables/inner_sea_world_guide",
            chassis_book_counts("inner_sea_world_guide"),
            &races,
        ),
        book_status(
            "advanced_race_guide",
            "src/rules_core/rules_tables/advanced_race_guide",
            advanced_race_guide_counts(),
            &races,
        ),
        book_status(
            "pathfinder_unchained",
            "src/rules_core/rules_tables/pathfinder_unchained",
            pathfinder_unchained_counts(),
            &races,
        ),
        book_status(
            "ultimate_campaign",
            "src/rules_core/rules_tables/ultimate_campaign",
            ultimate_campaign_counts(),
            &races,
        ),
        book_status(
            "ultimate_intrigue",
            "src/rules_core/rules_tables/ultimate_intrigue",
            ultimate_intrigue_counts(),
            &races,
        ),
        book_status(
            "ultimate_equipment",
            "src/rules_core/rules_tables/ultimate_equipment",
            ultimate_equipment_counts(),
            &races,
        ),
        book_status(
            "ultimate_wilderness",
            "src/rules_core/rules_tables/ultimate_wilderness",
            ultimate_wilderness_counts(),
            &races,
        ),
        book_status(
            "ultimate_combat",
            "src/rules_core/rules_tables/ultimate_combat",
            ultimate_combat_counts(),
            &races,
        ),
        book_status(
            "ultimate_magic",
            "src/rules_core/rules_tables/ultimate_magic",
            ultimate_magic_counts(),
            &races,
        ),
        book_status(
            "ultimate_psionics",
            "src/rules_core/rules_tables/ultimate_psionics",
            ultimate_psionics_counts(),
            &races,
        ),
        book_status(
            "occult_adventures",
            "src/rules_core/rules_tables/occult_adventures",
            occult_adventures_counts(),
            &races,
        ),
        book_status(
            "adventurers_guide",
            "src/rules_core/rules_tables/adventurers_guide",
            adventurers_guide_counts(),
            &races,
        ),
        book_status(
            "inner_sea_faiths",
            "src/rules_core/rules_tables/inner_sea_faiths",
            inner_sea_faiths_counts(),
            &races,
        ),
        book_status(
            "inner_sea_magic",
            "src/rules_core/rules_tables/inner_sea_magic",
            inner_sea_magic_counts(),
            &races,
        ),
        book_status(
            "inner_sea_temples",
            "src/rules_core/rules_tables/inner_sea_temples",
            inner_sea_temples_counts(),
            &races,
        ),
        // Landed with real compiled tables and no panel row -- caught by
        // `every_book_landed_in_rules_tables_is_reported` (SD-32 stale-
        // assertion fix, `corpus_ingest_diagnostic.rs` RED-branch cycle).
        book_status(
            "inner_sea_races",
            "src/rules_core/rules_tables/inner_sea_races",
            inner_sea_races_counts(),
            &races,
        ),
        book_status(
            "mythic_adventures",
            "src/rules_core/rules_tables/mythic_adventures",
            mythic_adventures_counts(),
            &races,
        ),
        book_status(
            "ultimate_magic_wordsofpower",
            "src/rules_core/rules_tables/ultimate_magic_wordsofpower",
            ultimate_magic_wordsofpower_counts(),
            &races,
        ),
    ]
}

#[tauri::command]
pub fn corpus_ingest_diagnostic() -> Vec<BookIngestStatus> {
    build_corpus_ingest_diagnostic()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// **The drift guard.** The set of books this diagnostic reports must
    /// equal the set of books actually landed in `rules_tables`, derived by
    /// reading the directory rather than from a list maintained here.
    ///
    /// This is the test that would have caught the defect it was written for:
    /// `advanced_race_guide` and `pathfinder_unchained` landed as real
    /// `rules_tables` books and this diagnostic kept reporting four, so the
    /// panel — whose caption reads "every rule book landed in `rules_tables`"
    /// — told a tester that two ingested books did not exist.
    /// `rules_tables` module directories whose records are reported under
    /// ANOTHER book's panel row, because they are the same book.
    ///
    /// `rules_tables::bestiary` is the chassis half of Bestiary 1 — the 280 rows
    /// `rules_tables::beastiary1` does not hold (`decisions.md §58.3`) — and
    /// `beastiary1_counts` folds its two families into that book's row. A second
    /// row would tell a tester this repo had ingested two Bestiary 1s, which is
    /// the same class of wrong reading this drift guard exists to prevent, in
    /// the opposite direction.
    ///
    /// Each entry states the host row, and the host row's presence is asserted
    /// below: an alias whose host stopped being reported would otherwise turn
    /// this guard into a way to hide a book.
    const MODULES_REPORTED_UNDER_ANOTHER_BOOK: &[(&str, &str)] = &[("bestiary", "beastiary1")];

    #[test]
    fn every_book_landed_in_rules_tables_is_reported() {
        let reported: BTreeSet<String> = build_corpus_ingest_diagnostic()
            .into_iter()
            .map(|book| book.book_id)
            .collect();
        let mut landed = books_on_disk();
        for (module, host) in MODULES_REPORTED_UNDER_ANOTHER_BOOK {
            assert!(
                landed.remove(*module),
                "{module} is recorded as reported under {host} but is not landed in \
                 rules_tables at all -- drop the alias rather than carrying a dead one"
            );
            assert!(
                reported.contains(*host),
                "{module}'s records are reported under {host}, and {host} has no panel row"
            );
        }

        let missing: Vec<&String> = landed.difference(&reported).collect();
        assert!(
            missing.is_empty(),
            "these books are landed in src/rules_core/rules_tables/ and this diagnostic does not \
             report them: {missing:?}. The Corpus Ingest panel states it shows every rule book \
             landed in rules_tables, so an unreported book reads to a tester as an un-ingested \
             book. Add a `book_status(..)` row deriving its counts from its own real tables."
        );

        let phantom: Vec<&String> = reported.difference(&landed).collect();
        assert!(
            phantom.is_empty(),
            "this diagnostic reports books with no src/rules_core/rules_tables/ directory: \
             {phantom:?}"
        );
    }

    /// Every book directory under `src/rules_core/rules_tables/`, which is
    /// where the diagnostic's own `book_id` values come from.
    fn books_on_disk() -> BTreeSet<String> {
        let dir = repo_root().join("src/rules_core/rules_tables");
        std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{} must be readable: {e}", dir.display()))
            .filter_map(|entry| {
                let entry = entry.expect("readable dir entry");
                entry
                    .path()
                    .is_dir()
                    .then(|| entry.file_name().to_string_lossy().into_owned())
            })
            .collect()
    }

    #[test]
    fn reports_every_landed_book_in_a_stable_order() {
        let response = build_corpus_ingest_diagnostic();
        let book_ids: Vec<&str> = response.iter().map(|b| b.book_id.as_str()).collect();
        assert_eq!(
            book_ids,
            vec![
                "crb",
                "apg",
                "acg",
                "beastiary1",
                // SD-29 Epic 5 -- placed next to the other bestiary rather
                // than appended, so the panel reads in book-family order.
                "bonus_bestiary",
                // SD-29 Epic 5 extend round 1, for the same reason.
                "monster_codex",
                // SD-29 Epic 7 (companion lane) -- the three books whose only
                // compiled family is `companion`, kept beside `monster_codex`,
                // which now carries both chassis registries' tables in one row.
                "inner_sea_combat",
                "inner_sea_intrigue",
                "horror_adventures",
                // SD-29 Epic 7 round 2 -- three more companion books, kept in
                // the same block for the same reason. Two are companion-ONLY;
                // `bestiary_2` stopped being one at SD-29 Epic 5 extend round 4,
                // which added its 316 monsters + 402 abilities to the same row.
                "bestiary_5",
                "bestiary_6",
                "bestiary_2",
                // SD-29 Epic 5 extend round 5 -- the monster lane's own, kept
                // in this block so the bestiaries stay adjacent. Unlike
                // `bestiary_2` it carries the monster registry ONLY.
                "bestiary_3",
                // SD-29 Epic 5 extend round 6 -- monster registry only, like
                // `bestiary_3` above.
                "bestiary_4",
                // SD-29 Epic 5 extend round 7 -- monster registry only, like
                // `bestiary_4` above.
                "inner_sea_bestiary",
                // SD-29 Epic 5 extend round 9 -- monster registry only, like
                // `inner_sea_bestiary` above.
                "inner_sea_gods",
                // SD-29 Epic 5 extend round 2 -- the two Book of the Damned
                // volumes, kept next to the other chassis books.
                "book_of_the_damned_volume_1",
                "book_of_the_damned_volume_2",
                // SD-29 Epic 5 extend round 3 -- Inner Sea World Guide, the
                // first chassis book served with only part of its ability rows.
                "inner_sea_world_guide",
                "advanced_race_guide",
                "pathfinder_unchained",
                "ultimate_campaign",
                "ultimate_intrigue",
                "ultimate_equipment",
                "ultimate_wilderness",
                "ultimate_combat",
                "ultimate_magic",
                "ultimate_psionics",
                // SD31-E6-F2-003 -- this book's first compiled record family
                // of any kind, appended at the end of the list rather than
                // inserted into a book-family block, since it shares no
                // chassis with any book above it.
                "occult_adventures",
                // SD-31 wave-29 (`lane5-book-onboard` lane) -- this book's
                // first compiled record family of any kind, same
                // appended-at-the-end placement as `occult_adventures`
                // above, for the same reason.
                "adventurers_guide",
                // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
                // onboarding-precondition`, AT-32-G0-003) -- each of these
                // three books' first compiled record family of any kind,
                // same appended-at-the-end placement as `adventurers_guide`
                // above, for the same reason.
                "inner_sea_faiths",
                "inner_sea_magic",
                "inner_sea_temples",
                // SD-32 stale-assertion fix (`corpus_ingest_diagnostic.rs`
                // RED-branch cycle): these three books landed real compiled
                // `rules_tables` modules with no panel row, tripping
                // `every_book_landed_in_rules_tables_is_reported`. Appended
                // at the end, same placement as the block above.
                "inner_sea_races",
                "mythic_adventures",
                "ultimate_magic_wordsofpower"
            ]
        );
    }

    #[test]
    fn every_book_is_populated_with_real_nonzero_counts() {
        for book in build_corpus_ingest_diagnostic() {
            assert_eq!(book.status, "populated", "book {} should be populated", book.book_id);
            assert!(
                !book.content_kind_counts.is_empty(),
                "book {} must report at least one content kind",
                book.book_id
            );
            for (kind, count) in &book.content_kind_counts {
                assert!(
                    *count > 0,
                    "book {} kind {} must have a real nonzero count",
                    book.book_id,
                    kind
                );
            }
        }
    }

    #[test]
    fn crb_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let crb = response.iter().find(|b| b.book_id == "crb").expect("crb present");
        assert_eq!(crb.content_kind_counts["classes"], ClassId::ALL.len() as u32);
        // `races` is now read off the on-disk race corpus rather than off
        // `RaceId::ALL` (see `race_counts_by_diagnostic_book`). Keeping this
        // assertion turns it into the agreement pin between the two sources:
        // the compiled table and the corpus must answer the same for CRB, or
        // one of them has drifted.
        assert_eq!(crb.content_kind_counts["races"], RaceId::ALL.len() as u32);
        assert_eq!(
            crb.content_kind_counts["feats"],
            crb_feats::feat_tables().len() as u32
        );
        assert_eq!(
            crb.content_kind_counts["spells"],
            crb_spell_list::SPELL_LIST.len() as u32
        );
        assert_eq!(
            crb.content_kind_counts["equipment"],
            crb_equipment_tables::equipment_tables().len() as u32
        );
    }

    /// The APG/ACG feat ingest has to show up here too — this diagnostic
    /// is what reports per-book coverage, and leaving `feats` off the APG
    /// and ACG rows would understate 301 real ingested records.
    #[test]
    fn apg_and_acg_feat_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let apg_book = response.iter().find(|b| b.book_id == "apg").expect("apg present");
        assert_eq!(
            apg_book.content_kind_counts["feats"],
            apg::feats::feat_tables().len() as u32
        );
        assert_eq!(apg_book.content_kind_counts["feats"], 172);

        let acg_book = response.iter().find(|b| b.book_id == "acg").expect("acg present");
        assert_eq!(
            acg_book.content_kind_counts["feats"],
            acg::feats::feat_tables().len() as u32
        );
        assert_eq!(acg_book.content_kind_counts["feats"], 129);
    }

    /// ARG's rows must be the book's own real tables, not a repeat of another
    /// book's. Asserted against the live tables *and* against the literal
    /// counts each module's own doc comment documents, so a silent table
    /// regression fails here even though both sides moved together.
    #[test]
    fn advanced_race_guide_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let arg_book = response
            .iter()
            .find(|b| b.book_id == "advanced_race_guide")
            .expect("advanced_race_guide present");

        assert_eq!(
            arg_book.content_kind_counts["feats"],
            arg::feats::feat_tables().len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["feats"], 187);
        assert_eq!(
            arg_book.content_kind_counts["spells"],
            arg::spell_list::SPELL_LIST.len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["spells"], 93);
        assert_eq!(
            arg_book.content_kind_counts["equipment"],
            arg::equipment_tables::equipment_tables().len() as u32
        );
        assert_eq!(arg_book.content_kind_counts["equipment"], 200);

        // ARG's racial traits are deliberately not a row — they have no
        // `rules_tables` module, which is what this panel counts. Pinned so
        // the boundary is a decision on record rather than something a later
        // reader has to infer from an absence.
        assert!(
            !arg_book.content_kind_counts.contains_key("race_traits"),
            "ARG's 283 racial-trait records (156 -> 201 by SD-31 Epic 1-F2, 2026-08-15; \
             201 -> 259 by SD-31-E6-F4-002; 259 -> 283 by SD-31-E6-F4-003, both 2026-08-16) \
             are corpus-JSON-only; see the module doc for why they are accounted for in \
             LICENSE.json rather than here"
        );
    }

    /// PU's rows, same rule. The class and class-feature counts are the ones
    /// that matter most: they are what the SD-27 Unchained wiring added, and
    /// they are invisible to `reach_gate`'s source scanner (its records live
    /// inside accessor function bodies, not in column-zero `pub const`
    /// slices), so this diagnostic is the only automated inventory that sees
    /// them at all.
    #[test]
    fn pathfinder_unchained_counts_match_the_real_underlying_tables() {
        let response = build_corpus_ingest_diagnostic();
        let pu_book = response
            .iter()
            .find(|b| b.book_id == "pathfinder_unchained")
            .expect("pathfinder_unchained present");

        assert_eq!(
            pu_book.content_kind_counts["classes"],
            pu::class_chassis::PuClassId::ALL.len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["classes"], 4);
        assert_eq!(
            pu_book.content_kind_counts["class_features"],
            pu_class_feature_count()
        );
        assert_eq!(pu_book.content_kind_counts["class_features"], 64);
        assert_eq!(
            pu_book.content_kind_counts["equipment"],
            pu::equipment_tables::equipment_tables().len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["equipment"], 42);
        assert_eq!(
            pu_book.content_kind_counts["feats"],
            pu::feat_tables::feat_tables().len() as u32
        );
        assert_eq!(pu_book.content_kind_counts["feats"], 17);
    }

    /// The two SD-27 books' reported totals must equal the licensed content
    /// records their own `data/corpus/<book>/LICENSE.json` accounts for.
    ///
    /// The two artifacts are derived from completely different sources — this
    /// diagnostic counts compiled tables, `LICENSE.json` counts files on disk
    /// — so agreement between them is real evidence rather than a tautology,
    /// and a table that silently loses records shows up as a mismatch here.
    ///
    /// ARG's corpus-JSON-only racial traits + class_feature records (201 ->
    /// 844 by SD-31 `epic-5-chassis-sweep` F1, `SD31-E5-F1-001`, 2026-08-15;
    /// 844 -> 859 by `SD31-W4-INTEGRATE-001`, 2026-08-16, reconciling
    /// `SD31-E6-F5-002`'s 15 corpus-JSON-only `equipment`/`equipment_modifier`
    /// records -- `equipment_gap_tables` is not among this diagnostic's
    /// tracked `rules_tables` sums for ARG, the same corpus-only shape as
    /// the class_feature records above)
    /// are the one declared difference, stated as a number here rather than
    /// waved at, so the two artifacts reconcile exactly.
    #[test]
    fn the_two_ingested_books_totals_reconcile_with_their_license_artifacts() {
        for (book_id, corpus_dir, corpus_only_records) in [
            // 156 -> 201 by SD-31 Epic 1-F2 (2026-08-15): Bestiary 2's 6-race
            // batch added 45 more ARG race_trait records. 201 -> 844 by
            // SD-31 `SD31-E5-F1-001` (2026-08-15): `cache_gen::class_feature`
            // wrote 643 new class_feature corpus-JSON-only records for this
            // book (no rules_tables module carries class_feature data, so
            // this is not double-counted -- see that cycle's module doc
            // comment and `artifacts/SD31-E5-F1-001-lever-measurement.md`).
            // 859 -> 917 by SD-31-E6-F4-002 (2026-08-16): `ingest_races.rs`'s
            // own 6-race chassis batch adds 58 corpus-JSON-only race_trait
            // records here (no rules_tables module backs race_trait data
            // either); its 6 `race` chassis records are NOT added here --
            // they ARE counted in `reported`, via the SAME `races` map
            // `race_counts_by_diagnostic_book()` already merges into every
            // book's `content_kind_counts` (ARG's `races` row moved
            // `None` -> `Some(6)` this cycle, see the dedicated test above).
            // 917 -> 941 by SD-31-E6-F4-003 (2026-08-16): `ingest_race_traits
            // .rs`'s own 24-record alternate-trait batch for those same 6
            // races (Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang) -- also
            // corpus-JSON-only race_trait content, same shape as above.
            // 941 -> 979 by SD31-E6-F4-004 (2026-08-17): `ingest_races.rs`'s
            // own 4-race follow-on batch adds 38 more corpus-JSON-only
            // race_trait records (Gillman/Nagaji/Vanara/Vishkanya); its 4
            // `race` chassis records are again NOT added here for the same
            // reason the 2026-08-16 batch's weren't -- they ARE counted in
            // `reported`, via the same `races` map merge (ARG's `races` row
            // moved `Some(6)` -> `Some(10)` this cycle).
            // 979 -> 990 by SD31-E6-F4-006 (2026-08-17): `ingest_race_traits
            // .rs`'s own 11-record alternate-trait batch for those same 4
            // races (Gillman/Nagaji/Vanara/Vishkanya) -- also
            // corpus-JSON-only race_trait content, same shape as above.
            // 990 -> 1008 by SD31-E6-F4-007 (2026-08-17): `ingest_races.rs`'s
            // own 2-race follow-on batch adds 18 more corpus-JSON-only
            // race_trait records (Changeling 9, Samsaran 9), closing
            // `arg_races.lst`'s full 37-row playable-race roster; its 2
            // `race` chassis records are again NOT added here for the same
            // reason every prior batch's weren't -- they ARE counted in
            // `reported` (ARG's `races` row moved `Some(10)` -> `Some(12)`
            // this cycle).
            // 1008 -> 1072 by `SD31-CE-COMPANION-001` (2026-08-18): `decisions.md §9`
            // retired the `core_essentials` book id and Aasimar's and Tiefling's 64
            // heritage `race_trait` records moved into this book's own corpus directory.
            // They are corpus-only by construction -- `decisions.md §24` rules out the
            // formula interpreter a compiled race-trait table would need, so they are
            // served off disk and this diagnostic's `rules_tables` half cannot see them.
            // 1072 -> 1073 by `decisions.md §20` no_record-to-zero round 4 (2026-08-23):
            // `gen_advanced_race_guide()` extended to also call `gen_monster_book`, adding
            // this book's 1 owner-less `monster_ability` record. Corpus-only for the same
            // reason as every other family above: no `rules_tables` module counts
            // `monster`/`monster_ability` for this book, only `advanced_race_guide::feats`/
            // `spell_list`/`equipment_tables` (`advanced_race_guide_counts()`).
            // 1073 -> 1699 by the SD-32 stale-assertion fix cycle
            // (`corpus_ingest_diagnostic.rs` RED-branch, 2026-08-23): re-derived fresh
            // against the live filesystem (`live_on_disk_record_count`, same walk
            // `gen_book_cache.rs::count_on_disk_records` documents) rather than trusted
            // from a stale `LICENSE.json` snapshot -- see that function's own doc comment
            // for why the snapshot, and re-running the generator that writes it, are both
            // unsafe here. `2205` (live total) `- 506` (`reported`, unchanged -- this
            // cycle's diff touches no compiled `rules_tables` module) `= 1699`. The 626-unit
            // rise is NOT new corpus-only content this cycle added: `1410424cf3`'s
            // "close feat+spell no_record" landed 67 new `feat`/`equipment`/`companion`
            // corpus-JSON records beyond what `advanced_race_guide_counts()` compiles
            // (verified: `find data/corpus/advanced_race_guide/feat -name '*.json' | wc -l`
            // is 235 against a compiled `feat_tables().len()` of 187, and the equivalent
            // for `equipment`/`companion`), and the remaining ~559 is this constant simply
            // having drifted uncorrected across however many prior sibling-lane commits
            // landed corpus-only content for this book without anyone re-deriving this
            // literal against a live count in the meantime -- `decisions.md §17a`.
            // 1699 -> 1701 by the row-19 desktop reach/catalog reds cycle (SD-32,
            // 2026-08-24): re-derived fresh again against `live_on_disk_record_count`
            // (`2207`) with `reported` unchanged (`506` -- this cycle's diff touches no
            // compiled `rules_tables` module for ARG), `2207 - 506 = 1701`. The +2 is
            // more T12 census/class-feature-lane corpus growth landing between the two
            // re-derivations, not a new defect.
            ("advanced_race_guide", "advanced_race_guide", 1701u32),
            // 0 -> 69 by `decisions.md §20` no_record-to-zero round 4 (2026-08-23):
            // `gen_pathfinder_unchained()` extended to also call `gen_monster_book`, adding
            // this book's 69 owner-less `monster_ability` records (72 orphan candidates, 3
            // refused as an unscreenable multi-DESC: shape). Corpus-only for the same
            // reason: `pathfinder_unchained_counts()` above tracks `classes`/
            // `class_features`/`feats`/`equipment` only, never `monster_abilities`.
            // 69 -> 1137 by the SD-32 stale-assertion fix cycle
            // (`corpus_ingest_diagnostic.rs` RED-branch, 2026-08-23): this branch of the
            // loop had never actually run green -- the `for` loop panics on ARG's failing
            // assertion (iterated first) before ever reaching this one, so this literal's
            // own drift went undetected behind that unrelated failure until this cycle
            // fixed ARG's and the loop reached PU for the first time. Re-derived fresh
            // against `live_on_disk_record_count`: `1264` (live total,
            // `find data/corpus/pathfinder_unchained -name '*.json' | grep -v LICENSE |
            // grep -v /_parity/ | wc -l`) `- 127` (`reported`, unchanged) `= 1137`. The
            // bulk of the rise is `class_feature`: the compiled `pu_class_feature_count()`
            // is 64, but `data/corpus/pathfinder_unchained/class_feature/` holds 604 --
            // `decisions.md §13`'s T12 (2,453 `class_feature`s belonging to classes the
            // engine does not model) plus subsequent `§20` no_record closure landed the
            // rest corpus-only, same shape as `ability`/`skill`/`race_trait_generic`/
            // `template`, none of which this diagnostic's `rules_tables` half compiles.
            // 1137 -> 1144 by the row-19 desktop reach/catalog reds cycle (SD-32,
            // 2026-08-24): re-derived fresh again (this loop only ever reaches this
            // branch once the ARG branch above it stops panicking first -- same shape
            // as the RED-branch note above) against `live_on_disk_record_count`
            // (`1271`) with `reported` unchanged (`127`), `1271 - 127 = 1144`. +7 more
            // T12 census/class-feature-lane corpus growth, not a new defect.
            // 1144 -> 1140 by SD-34 `AT-34-E6-001` gate-lane-B (2026-09-01), settling
            // the wave-27/wave-28 desktop contradiction: `AT-34-E6-001` gate-lane-A's
            // own `e5fd8dddb1` (2026-08-31) fixed the PU equipmods dup-key generator
            // and deleted 4 stale flat `equipment_modifier` records, moving the live
            // on-disk walk from 1271 to 1267 (`find data/corpus/pathfinder_unchained
            // -name '*.json' | grep -v LICENSE.json | grep -v '/_' | wc -l` = 1267,
            // independently re-derived this cycle). `reported` (127, from
            // `pathfinder_unchained_counts()`) is unchanged -- no `rules_tables`
            // module touched by that commit -- so `1267 - 127 = 1140`. This is the
            // SAME defect wave-27 lane C already named and explicitly left for
            // lane B/desktop's own territory to fix (`AT-34-E6-001_gate-lane-c_
            // wave27_cycle_receipt.md:154`), not a new drift.
            ("pathfinder_unchained", "pathfinder_unchained", 1140u32),
        ] {
            let response = build_corpus_ingest_diagnostic();
            let book = response
                .iter()
                .find(|b| b.book_id == book_id)
                .unwrap_or_else(|| panic!("{book_id} present"));
            let reported: u32 = book.content_kind_counts.values().sum();

            let corpus_dir_path = repo_root().join("data/corpus").join(corpus_dir);
            let licensed = live_on_disk_record_count(&corpus_dir_path);

            assert_eq!(
                reported + corpus_only_records,
                licensed,
                "{book_id}: this diagnostic reports {reported} records from rules_tables plus \
                 {corpus_only_records} known corpus-only records, but a live walk of {} \
                 accounts for {licensed} real on-disk records. One of the two is stale -- \
                 re-derive corpus_only_records fresh (decisions.md §17a), never repin without \
                 proof.",
                corpus_dir_path.display()
            );
        }
    }

    #[test]
    fn beastiary1_monster_count_matches_the_documented_real_total() {
        // SD28-E16 subset 09 (2026-08-07) raised this from 41 to 46
        // (Lion, Ogre, Pegasus, Rust Monster, Shadow —
        // `beastiary1::mod.rs`'s own subset roster doc comments carry the
        // current count; `docs/architecture/rules-data-tables.md`'s "41
        // monsters" figure predates this subset and is stale, flagged here
        // rather than edited -- that doc is outside this cycle's write
        // scope).
        //
        // SD-29 Epic 5 round 8 added the chassis half of the same book (280
        // rows, `rules_tables::bestiary`, `decisions.md §58.3`), so the panel's
        // monster count is now the SUM of the two tables serving Bestiary 1 —
        // 46 + 280 — and the book gains its first `monster_abilities` family.
        // Stated as the sum rather than as `326` so a divergence says which
        // table moved.
        let response = build_corpus_ingest_diagnostic();
        let bestiary = response
            .iter()
            .find(|b| b.book_id == "beastiary1")
            .expect("beastiary1 present");
        assert_eq!(bestiary.content_kind_counts["monsters"], 46 + 280);
        // SD31-E6-F9-005 (transcription lane, wave 12): 323 -> 399 (+76),
        // 76 new monster_ability records transcribed for this book.
        // SD31-W21-MONSTER-001 (wave 21): 399 -> 467 (+68), the
        // `CATEGORY:Internal` bundle-row ownership hop
        // (`transcribe_monster_tables.py::find_internal_bundle_ability_refs`)
        // resolved 68 previously-orphaned ability rows.
        // SD31-W23-MONSTER-001 (wave 23): 467 -> 522 (+55), the cross-table-
        // owner remedy `decisions.md §58.3` named and left unbuilt --
        // `transcribe_monster_tables.py`'s cross-table-owner screen now
        // transcribes ability rows whose owner's OWN stat block ships from
        // `rules_tables::beastiary1` (46 legacy monsters) rather than
        // dropping them, keyed to that real owner's name
        // (`MonsterBook::abilities_owned_by_name`).
        // T9 `MonsterAbilityFacet` widening cycle: 522 -> 529 (+7), 7 more
        // owned, reachable ability rows shipped once the widened facet
        // vocabulary (`Weakness`/`Defensive`/`Aura`/`Sense`/`Communicate`)
        // and the multi-`TYPE:`-token parsing fix landed
        // (`rules_tables::bestiary::mod.rs`'s own comment carries the full
        // derivation).
        // `decisions.md §20` (no_record-to-zero wave 2): 529 -> 709 (+180),
        // owner-less rows (no monster row of this book claims them) now
        // ship for shape measurement rather than being dropped as orphans.
        // `decisions.md §22`/round 6: 709 -> 710 (+1), `Spectre ~ Create
        // Spawn` now ingests (a comma-delimiter `TYPE:` row this book's own
        // parser previously refused) -- owned, so it joins the reaching set.
        // `decisions.md §27`/round 8: 710 -> 711 (+1), `Morlock ~ Sneak
        // Attack` (`TYPE:Internal`, no facet/delivery) now ships with a
        // provisional `SpecialQuality` facet default instead of being
        // dropped -- owned (Morlock claims it), so it joins the reaching set.
        // `decisions.md §27b` round 9: 711 -> 733 (+22), the multi-DESC:
        // parse-refusal group closes via `parse_desc`'s new generalised
        // sixth branch -- 21 real `no_record` units plus `Lycanthrope ~
        // Change Shape` (already `text-complete` by inventory evidence
        // alone, same shape as round 8's `Bunyip ~ Blood Rage`).
        assert_eq!(bestiary.content_kind_counts["monster_abilities"], 733);
    }

    #[test]
    fn last_ingested_at_is_a_real_git_derived_timestamp_when_available() {
        // This test runs inside the real repo checkout, so git history for
        // every book directory must be reachable — assert the diagnostic
        // actually queried it rather than silently defaulting to None.
        for book in build_corpus_ingest_diagnostic() {
            let timestamp = book
                .last_ingested_at
                .as_ref()
                .unwrap_or_else(|| panic!("expected a real git-derived timestamp for {}", book.book_id));
            assert!(
                timestamp.contains('T'),
                "timestamp {timestamp} for {} should be RFC 3339-shaped",
                book.book_id
            );
        }
    }

    /// The defect: the panel showed CRB `races: 7` and, for Bestiary 1,
    /// `monsters: 41` with no race count at all — while 18 races were
    /// creatable and 11 of them were Bestiary 1's.
    #[test]
    fn every_book_that_declares_races_reports_how_many_it_declares() {
        let response = build_corpus_ingest_diagnostic();
        let races = |book_id: &str| -> Option<u32> {
            response
                .iter()
                .find(|book| book.book_id == book_id)
                .and_then(|book| book.content_kind_counts.get("races").copied())
        };

        assert_eq!(races("crb"), Some(7), "the Core Rulebook's seven races");
        assert_eq!(
            races("beastiary1"),
            Some(11),
            "Bestiary 1's eleven races reached the creation screen long before they \
             reached this panel"
        );
        assert_eq!(
            races("advanced_race_guide"),
            Some(12),
            "ARG's own 12-race total: the 6-race batch (Catfolk, Kitsune, Ratfolk, Strix, \
             Suli, Wayang; SD-31-E6-F4-002, 2026-08-16) -- ARG no longer declares zero races \
             of its own (superseding `decisions.md §25.2`'s premise) -- plus SD31-E6-F4-004's \
             4-race follow-on (Gillman, Nagaji, Vanara, Vishkanya; 2026-08-17) plus \
             SD31-E6-F4-007's 2-race follow-on (Changeling, Samsaran; 2026-08-17), closing \
             `arg_races.lst`'s full 37-row playable-race roster"
        );
        assert_eq!(
            races("bestiary_2"),
            Some(7),
            "Bestiary 2's seven races: the original six (SD-31 Epic 1-F2, 2026-08-15) -- the \
             first race batch this panel has reported since Bestiary 1's, and proof the \
             `diagnostic_book_id(\"B2\")` mapping this batch added actually attaches the \
             count to the right book row -- plus Dhampir (SD-32 card-11 T2b lane, 2026-08-23)"
        );
    }

    /// The number the panel reports must be the number a player can actually
    /// create, so the two surfaces cannot drift apart.
    #[test]
    fn the_panels_race_total_equals_the_race_catalogs_own_creatable_total() {
        let response = build_corpus_ingest_diagnostic();
        let panel_total: u32 = response
            .iter()
            .filter_map(|book| book.content_kind_counts.get("races").copied())
            .sum();

        let catalog = build_race_catalog();
        let creatable: BTreeSet<&str> =
            catalog.entries.iter().map(|entry| entry.race_id.as_str()).collect();

        assert_eq!(
            panel_total as usize,
            creatable.len(),
            "the panel's per-book race counts must sum to exactly the races the catalog serves"
        );
        assert_eq!(
            panel_total, 39,
            "39 in-scope races today: CRB's 7 plus Bestiary 1's 11 plus Bestiary 2's 7 (the \
             original 6, SD-31 Epic 1-F2, 2026-08-15, plus Dhampir, SD-32 card-11 T2b lane, \
             2026-08-23) plus Bestiary 5's 1 (Skinwalker follow-on batch, \
             2026-08-15) plus Advanced Race Guide's 12 (SD-31-E6-F4-002, 2026-08-16: Catfolk, \
             Kitsune, Ratfolk, Strix, Suli, Wayang; SD31-E6-F4-004, 2026-08-17: Gillman, \
             Nagaji, Vanara, Vishkanya; SD31-E6-F4-007, 2026-08-17: Changeling, Samsaran -- \
             closing `arg_races.lst`'s full 37-row playable-race roster) plus Bestiary 6's 1 \
             (Rougarou, SD-31 wave-24, 2026-08-20)"
        );
    }

    /// A book with no race content must not grow a misleading `races: 0` row —
    /// only the books the race corpus is actually searched for get one.
    #[test]
    fn books_the_race_corpus_does_not_cover_carry_no_race_row_at_all() {
        let response = build_corpus_ingest_diagnostic();
        for book_id in ["apg", "acg", "pathfinder_unchained"] {
            let book = response
                .iter()
                .find(|book| book.book_id == book_id)
                .unwrap_or_else(|| panic!("{book_id} present"));
            assert!(
                !book.content_kind_counts.contains_key("races"),
                "{book_id} is not a race corpus book; a `races` row here would be an \
                 unmeasured zero dressed as a measurement"
            );
        }
    }

    /// `book_status` is pure over the race map, so the "corpus unreachable"
    /// branch is a real, driven case rather than an untested `if`.
    #[test]
    fn a_book_with_no_entry_in_the_race_map_reports_no_race_row() {
        let status = book_status(
            "beastiary1",
            "src/rules_core/rules_tables/beastiary1",
            beastiary1_counts(),
            &BTreeMap::new(),
        );
        assert!(
            !status.content_kind_counts.contains_key("races"),
            "an unreachable corpus must omit the row, never report a fabricated zero"
        );
        assert_eq!(status.content_kind_counts["monsters"], 46 + 280);
    }

    #[test]
    fn the_race_catalog_book_codes_map_onto_this_panels_book_ids() {
        assert_eq!(diagnostic_book_id("CRB"), "crb");
        assert_eq!(diagnostic_book_id("B1"), "beastiary1");
        assert_eq!(diagnostic_book_id("ARG"), "advanced_race_guide");
        assert_eq!(
            diagnostic_book_id("UM"),
            "UM",
            "an unrecognized book passes through verbatim rather than landing in the wrong row"
        );
    }
}
