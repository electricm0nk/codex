//! The closure of the last standing Bestiary 1 race-trait reach gap, and the
//! executable proof that it is closed by *data* rather than by deletion.
//!
//! # What this file replaces
//!
//! Its predecessor
//! (`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`) asserted the
//! opposite: that `Duergar ~ Spell-Like Ability ~ Invisibility` could not be
//! reached by any selection, because the only row in the whole PCGen checkout
//! that sets its gate flag lives in a book this project had not ingested. That
//! file's own doc comment named the condition for its retirement —
//!
//! > [`no_ingested_book_sets_the_flag_that_grants_duergar_invisibility`] fails
//! > the moment a corpus record sets that flag — i.e. the day Monster Codex is
//! > ingested. That is the intent: the finding closes by a test going red and
//! > being read, not by anyone remembering.
//!
//! — and this is that day. SD-29's race-trait lane pilot ingests
//! `monster_codex/mc_abilities_race.lst`, whose `Duergar ~ Ironskinned` row
//! carries the sole `FACT:Duergar_ReplaceSLAEnlargePerson|True` token in the
//! upstream corpus.
//!
//! **The predecessor's probe would NOT have caught this on its own.** Its
//! `corpus()` helper hardcoded three book roots (`core_rulebook`, `beastiary`,
//! `advanced_race_guide`), so ingesting a *fourth* book leaves it green while
//! the fact it asserts has become false. Re-deriving the loaded book list from
//! the same constant the running application reads
//! ([`race_catalog::RACE_CORPUS_BOOKS`], via
//! [`race_trait_picker::build_alternate_racial_traits`] in the desktop crate)
//! is what makes this file's claim about the shipped product rather than about
//! a list written here — see [`the_loaded_books_are_the_ones_the_app_loads`].
//!
//! # The protocol, read off the corpus, not asserted
//!
//! Duergar's two spell-like-ability rows are mutually exclusive alternatives,
//! `core_essentials/races/duergar/duergar_abilities_race.lst:27`-`28`:
//!
//! ```text
//! Spell-Like Ability  KEY:Duergar ~ Spell-Like Ability ~ Enlarge Person
//!     PREFACT:1,ABILITIES,Duergar_ReplaceSLAInvisibility=True
//! Spell-Like Ability  KEY:Duergar ~ Spell-Like Ability ~ Invisibility
//!     PREFACT:1,ABILITIES,Duergar_ReplaceSLAEnlargePerson=True
//! ```
//!
//! Each is granted by the flag that removes *the other one*. ARG supplies the
//! first half (`Duergar ~ Blood Enmity` sets `Duergar_ReplaceSLAInvisibility`);
//! Monster Codex supplies the second (`Duergar ~ Ironskinned` sets
//! `Duergar_ReplaceSLAEnlargePerson`). Both halves are asserted here, so
//! "closed" cannot quietly become "closed in one direction only".

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus};

const FLAG: &str = "Duergar_ReplaceSLAEnlargePerson";
const GRANTED_RECORD: &str = "Duergar ~ Spell-Like Ability ~ Invisibility";
const SETTER_RECORD: &str = "Duergar ~ Ironskinned";
const SETTER_BOOK: &str = "monster_codex";

/// The books the desktop app's race corpus actually loads.
///
/// Duplicated from `apps/desktop/src-tauri/src/race_catalog.rs`'s
/// `RACE_CORPUS_BOOKS` because the root crate's integration tests cannot depend
/// on the Tauri crate. [`the_loaded_books_are_the_ones_the_app_loads`] pins the
/// two lists equal by reading that file, so the copy cannot drift silently —
/// which is precisely the failure mode the predecessor test had.
const LOADED_BOOKS: &[&str] = &[
    "core_rulebook",
    "beastiary",
    "advanced_race_guide",
    "advanced_players_guide",
    "monster_codex",
    // SD-29 race-trait lane round 2. Round 2 added the book to the app's
    // `RACE_CORPUS_BOOKS` without adding it here, so
    // `the_loaded_books_are_the_ones_the_app_loads` -- the test whose whole job is
    // stopping this copy from drifting -- was RED on the branch until round 3
    // (`decisions.md §47.3`). It did its job; nobody read it.
    "inner_sea_races",
    // SD-29 race-trait lane round 3.
    "horror_adventures",
    // `core_essentials` was here from SD-29 race-trait lane round 4 until
    // `SD31-CE-COMPANION-001` (2026-08-18). `decisions.md §9` retired the book
    // id: Aasimar's and Tiefling's 64 heritage records now live under
    // `advanced_race_guide` (already in this list, three entries up), so the
    // app's `RACE_CORPUS_BOOKS` dropped the entry and this copy follows.
    //
    // The two comments this replaces recorded that this same test caught this
    // same lane adding a book here twice, in rounds 2 and 4. It has now caught
    // a REMOVAL too, on the same day, from the same list -- which is the point:
    // the check is on set equality, not on growth.
    // SD-31 Epic 1-F2 (2026-08-15): Bestiary 2's 6-race chassis batch. THIRD
    // time this exact test has caught this exact omission -- this copy is
    // edited in the same commit that adds a book to `RACE_CORPUS_BOOKS` from
    // now on, not after the fact.
    "bestiary_2",
    // SD-31 Epic 1 follow-on batch (2026-08-15): Skinwalker's chassis. FOURTH
    // time this exact test has caught this exact omission.
    "bestiary_5",
    // SD-31 wave-24 integration cycle (2026-08-20): Rougarou's chassis
    // (Bestiary 6). FIFTH time this exact test has caught this exact
    // omission -- edited in the same commit that adds the book to
    // `RACE_CORPUS_BOOKS`, per this file's own standing rule above.
    "bestiary_6",
];

fn corpus_dirs() -> Vec<(&'static str, PathBuf)> {
    LOADED_BOOKS.iter().map(|book| (*book, PathBuf::from("data/corpus").join(book))).collect()
}

fn corpus() -> RaceCorpus {
    let dirs = corpus_dirs();
    let roots: Vec<BookCorpusRoot<'_>> = dirs
        .iter()
        .map(|(book, dir)| BookCorpusRoot { book_id: book, dir: dir.as_path() })
        .collect();
    load_race_corpus(&roots)
}

/// The list above is the shipped one, re-derived from the desktop crate's own
/// source rather than trusted.
#[test]
fn the_loaded_books_are_the_ones_the_app_loads() {
    let src = std::fs::read_to_string("apps/desktop/src-tauri/src/race_catalog.rs")
        .expect("the desktop race catalog source is readable from the repo root");
    let decl = src
        .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
        .nth(1)
        .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
    let list = decl.split(';').next().expect("the declaration terminates");
    let shipped: Vec<String> =
        list.split('"').skip(1).step_by(2).map(str::to_owned).collect();

    assert_eq!(
        shipped,
        LOADED_BOOKS.iter().map(|b| (*b).to_owned()).collect::<Vec<String>>(),
        "this test's book list has drifted from the one the app loads; a claim about a list \
         written here is not a claim about the product"
    );
}

/// The setter is really on disk, in Monster Codex, carrying the flag.
#[test]
fn monster_codex_ships_the_row_that_sets_the_flag() {
    let corpus = corpus();
    let setters: BTreeSet<(&str, &str)> = corpus
        .race_keys()
        .into_iter()
        .flat_map(|race| corpus.traits_for(race))
        .filter(|record| record.data.sets_replace_flags.iter().any(|f| f == FLAG))
        .map(|record| (record.book_id.as_str(), record.data.key.as_str()))
        .collect();

    assert_eq!(
        setters,
        BTreeSet::from([(SETTER_BOOK, SETTER_RECORD)]),
        "exactly one ingested row may set {FLAG}, and it is {SETTER_BOOK}'s {SETTER_RECORD}"
    );
}

/// The record that was blocked is still the Bestiary 1 record — the gap closed
/// by a new setter arriving, not by the gated record being moved or rewritten.
#[test]
fn the_granted_record_is_still_bestiary_ones_own_with_its_real_prose() {
    let corpus = corpus();
    let record = corpus
        .traits_for("Duergar")
        .into_iter()
        .find(|r| r.data.key == GRANTED_RECORD)
        .expect("the record is ingested");
    assert_eq!(record.book_id, "beastiary");
    assert_eq!(record.requires_flag.as_deref(), Some(FLAG));
    assert_eq!(
        record.data.description.as_deref(),
        Some(
            "A duergar can use invisibility once per day, using its character level as its caster \
             level and affecting itself only."
        )
    );
}

/// The claim itself: a selection a player can really make brings the row in.
///
/// This is the same one-at-a-time probe `reach_gate`'s racial-trait claim runs,
/// restated here so the evidence sits next to the reason.
#[test]
fn selecting_ironskinned_grants_the_previously_unreachable_row() {
    let corpus = corpus();
    let alternates: BTreeSet<String> =
        corpus.alternate_traits("Duergar").into_iter().map(|r| r.data.key.clone()).collect();
    assert!(
        alternates.contains(SETTER_RECORD),
        "{SETTER_RECORD} must be a selectable alternate, not merely present: {alternates:?}"
    );

    let resolved = corpus.resolve("Duergar", &[SETTER_RECORD]).expect("resolves");
    let keys: BTreeSet<&str> = resolved.traits.iter().map(|t| t.key.as_str()).collect();
    assert!(
        keys.contains(GRANTED_RECORD),
        "{SETTER_RECORD} sets {FLAG}, which is {GRANTED_RECORD}'s positive PREFACT gate: {keys:?}"
    );
}

/// The mirror-image half still works. The protocol had one reachable direction
/// before this ingest; it must have two afterwards, not a different one.
#[test]
fn the_symmetric_half_of_the_same_protocol_still_reaches_a_player() {
    let corpus = corpus();
    let resolved = corpus.resolve("Duergar", &["Duergar ~ Blood Enmity"]).expect("resolves");
    let keys: BTreeSet<&str> = resolved.traits.iter().map(|t| t.key.as_str()).collect();
    assert!(
        keys.contains("Duergar ~ Spell-Like Ability ~ Enlarge Person"),
        "Blood Enmity sets Duergar_ReplaceSLAInvisibility, which grants the Enlarge Person row: {keys:?}"
    );
    assert!(!keys.contains(GRANTED_RECORD));
}

/// The two halves stay mutually exclusive: neither selection grants both rows.
/// Without this, "closed" could mean the gate stopped gating.
#[test]
fn neither_selection_grants_both_spell_like_ability_rows() {
    let corpus = corpus();
    for (selection, expected, forbidden) in [
        (SETTER_RECORD, GRANTED_RECORD, "Duergar ~ Spell-Like Ability ~ Enlarge Person"),
        ("Duergar ~ Blood Enmity", "Duergar ~ Spell-Like Ability ~ Enlarge Person", GRANTED_RECORD),
    ] {
        let resolved = corpus.resolve("Duergar", &[selection]).expect("resolves");
        let keys: BTreeSet<&str> = resolved.traits.iter().map(|t| t.key.as_str()).collect();
        assert!(keys.contains(expected), "{selection} must grant {expected}: {keys:?}");
        assert!(!keys.contains(forbidden), "{selection} must not also grant {forbidden}: {keys:?}");
    }
}

/// The pilot's own denominator, derived from disk rather than stated: Monster
/// Codex's race-trait records are exactly the rows whose `TYPE:` names an
/// in-scope race, and every one of them is a real file.
#[test]
fn the_monster_codex_race_trait_records_are_the_in_scope_ones() {
    let dir = Path::new("data/corpus/monster_codex/race_trait");
    assert!(dir.is_dir(), "the pilot ingest writes {dir:?}");

    let corpus = corpus();
    let keys: BTreeSet<&str> = corpus
        .race_keys()
        .into_iter()
        .flat_map(|race| corpus.traits_for(race))
        .filter(|record| record.book_id == SETTER_BOOK)
        .map(|record| record.data.key.as_str())
        .collect();

    assert_eq!(
        keys,
        BTreeSet::from([
            "Duergar ~ Ironskinned",
            "Duergar ~ Twilight-Touched",
            "Oversized Goblin",
            "Oversized Goblin ~ Ability Scores",
            "Oversized Goblin ~ Size",
            "Ratfolk ~ Cheek Pouches",
            "Ratfolk ~ Cleanliness",
            "Ratfolk ~ Lab Rat",
            "Ratfolk ~ Surface Sprinter",
            "Ratfolk ~ Surface Sprinter ~ Speed",
            "Ratfolk ~ Surface Sprinter ~ Vision",
        ]),
        "Monster Codex's in-scope racial traits are Duergar's 2, Goblin's 3, and (SD-32 \
         card-11 T2b lane, 2026-08-23) Ratfolk's 6. Ratfolk gained a real chassis in \
         `ingest_races.rs`'s SD-31-E6-F4-002 batch (2026-08-16); this test's own prior \
         wording -- 'Ratfolk has no ingested race chassis' -- had gone stale against that \
         landing. `Standard Goblin` (mc_abilities_race.lst:30) stays absent: no `DESC:`, \
         `BONUS:` or `ABILITY:` token at all, so there is nothing to transcribe."
    );
}
