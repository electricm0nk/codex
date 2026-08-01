//! SD-27 — the one Bestiary 1 race-trait record no player can reach, and the
//! executable proof that it is blocked upstream rather than unwired.
//!
//! # Why this file exists
//!
//! `reach_gate`'s `OPEN_FINDINGS` entry for `beastiary1`/`race_traits` says
//! `Duergar ~ Spell-Like Ability ~ Invisibility` is unreachable until Monster
//! Codex is ingested. That is a strong claim — "not our bug" is exactly the
//! kind of claim that should not be taken on a comment's word — so it is
//! re-derived here from the on-disk corpus every run.
//!
//! # The protocol, read off the corpus
//!
//! Duergar's two spell-like-ability rows are mutually exclusive alternatives.
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
//! first half: `Duergar ~ Blood Enmity` sets `Duergar_ReplaceSLAInvisibility`,
//! so choosing it drops Invisibility and grants Enlarge Person — and that
//! selection really works, which
//! [`the_symmetric_half_of_the_same_protocol_does_reach_a_player`] asserts so
//! that "blocked" cannot quietly become "broken".
//!
//! The second half has no setter in any ingested book. Across the whole PCGen
//! checkout, `FACT:Duergar_ReplaceSLAEnlargePerson|True` appears on exactly one
//! row: `Duergar ~ Ironskinned`, `monster_codex/mc_abilities_race.lst:16`.
//! Monster Codex is Tier-1 but deferred — `decisions.md §9` lists it among the
//! books SD-27 does not take, and `epic-breakdown.md:150` assigns it to
//! SD-29's Bestiary bundle. So there is nothing to wire, and hiding the record
//! to make a gate green would destroy the only signal that says so.
//!
//! # What fails when the block lifts
//!
//! [`no_ingested_book_sets_the_flag_that_grants_duergar_invisibility`] fails
//! the moment a corpus record sets that flag — i.e. the day Monster Codex is
//! ingested. That is the intent: the finding closes by a test going red and
//! being read, not by anyone remembering.

use std::collections::BTreeSet;
use std::path::Path;

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus};

const FLAG: &str = "Duergar_ReplaceSLAEnlargePerson";
const BLOCKED_RECORD: &str = "Duergar ~ Spell-Like Ability ~ Invisibility";

fn corpus() -> RaceCorpus {
    let roots = [
        BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") },
        BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") },
        BookCorpusRoot { book_id: "advanced_race_guide", dir: Path::new("data/corpus/advanced_race_guide") },
    ];
    load_race_corpus(&roots)
}

/// The record is really ingested — this is a reach gap, not a missing file.
#[test]
fn the_blocked_record_is_present_on_disk_with_its_real_prose() {
    let corpus = corpus();
    let record = corpus
        .traits_for("Duergar")
        .into_iter()
        .find(|r| r.data.key == BLOCKED_RECORD)
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

/// The blocking fact itself, derived: nothing in any loaded book sets the flag
/// this record's gate names.
#[test]
fn no_ingested_book_sets_the_flag_that_grants_duergar_invisibility() {
    let corpus = corpus();
    let setters: BTreeSet<&str> = corpus
        .race_keys()
        .into_iter()
        .flat_map(|race| corpus.traits_for(race))
        .filter(|record| record.data.sets_replace_flags.iter().any(|f| f == FLAG))
        .map(|record| record.data.key.as_str())
        .collect();

    assert!(
        setters.is_empty(),
        "{FLAG} now has a setter in an ingested book ({setters:?}). The Bestiary 1 race-trait \
         finding in reach_gate::OPEN_FINDINGS is closeable: {BLOCKED_RECORD} should now arrive \
         from that selection. Delete the finding and the UNREACHED_RECORD_FINDINGS entry."
    );
}

/// And, exhaustively: no selection a player can make brings the row in.
///
/// Every Duergar alternate, one at a time — the same probe `reach_gate`'s
/// racial-trait claim runs, restated here so the reason is readable next to
/// the evidence rather than only inside the gate.
#[test]
fn no_single_alternate_selection_brings_the_blocked_record_in() {
    let corpus = corpus();
    let alternates: Vec<String> =
        corpus.alternate_traits("Duergar").into_iter().map(|r| r.data.key.clone()).collect();
    assert!(!alternates.is_empty(), "Duergar has selectable alternates");

    for alternate in &alternates {
        let resolved = corpus.resolve("Duergar", &[alternate.as_str()]).expect("resolves");
        assert!(
            !resolved.traits.iter().any(|t| t.key == BLOCKED_RECORD),
            "{alternate} unexpectedly grants {BLOCKED_RECORD}"
        );
    }

    // Selecting all of them at once does not reach it either.
    let all: Vec<&str> = alternates.iter().map(String::as_str).collect();
    let resolved = corpus.resolve("Duergar", &all).expect("resolves");
    assert!(!resolved.traits.iter().any(|t| t.key == BLOCKED_RECORD));
}

/// The mirror-image row *is* reachable, which is what makes "blocked" the
/// right word instead of "broken": the protocol works, one of its two setters
/// lives in a book this project has not ingested.
#[test]
fn the_symmetric_half_of_the_same_protocol_does_reach_a_player() {
    let corpus = corpus();
    let resolved = corpus.resolve("Duergar", &["Duergar ~ Blood Enmity"]).expect("resolves");
    let keys: BTreeSet<&str> = resolved.traits.iter().map(|t| t.key.as_str()).collect();
    assert!(
        keys.contains("Duergar ~ Spell-Like Ability ~ Enlarge Person"),
        "Blood Enmity sets Duergar_ReplaceSLAInvisibility, which grants the Enlarge Person row: {keys:?}"
    );
    assert!(!keys.contains(BLOCKED_RECORD));
}
