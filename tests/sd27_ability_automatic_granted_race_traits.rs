//! SD-27 — the third racial-trait grant shape: `ABILITY:<cat>|AUTOMATIC|<key>`.
//!
//! # The defect this closes
//!
//! PCGen encodes "selecting alternate A also gives you replacement row B" three
//! different ways, and `race_resolver` read only two of them:
//!
//! | shape | example | read before this file existed |
//! |---|---|---|
//! | negated `!PREFACT` on the *standard* row | `Dwarf ~ Greed` is gated `!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True` | yes — [`RaceCorpus::resolve`]'s suppression pass |
//! | positive `PREFACT` on the *replacement* row | `Saltbeard ~ Dwarf ~ Greed` is gated `PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True` | yes — [`TraitRole::FlagGranted`] |
//! | direct `ABILITY:` grant on the *alternate* row | `Orc ~ Feral` carries `ABILITY:Orc Racial Trait\|AUTOMATIC\|Feral ~ Languages` | **no** |
//!
//! Two real ARG records ride the third shape and nothing else:
//! `Feral ~ Languages` (`arg_abilities_race.lst:606`, granted by `Orc ~ Feral`
//! at `:600`) and `Scion of Humanity ~ Languages` (`:336`, granted by
//! `Aasimar ~ Scion of Humanity` at `:331`). They carry no `PREFACT`, set no
//! replace-flag and are not racial defaults, so `classify` landed both in
//! [`TraitRole::Unclassified`] — never applied, never rendered. A player who
//! took Feral lost Orc's standard Languages row (the alternate sets
//! `Orc_ReplaceLanguages`, which suppresses it) and got **nothing** in its
//! place: the replacement text existed on disk and reached no surface.
//!
//! # What is asserted, and why in both directions
//!
//! Every assertion is a before/after pair on the same race, in the same spirit
//! as `sd27_alternate_racial_trait_reachability.rs`: the granted row must be
//! absent without the selection and present with it. A one-directional
//! assertion would pass just as well if the resolver started applying the row
//! unconditionally, which would be a different bug of the same size.
//!
//! The grant set is **derived from the corpus**, not transcribed:
//! [`the_ability_automatic_grant_shape_is_exactly_two_records_corpus_wide`]
//! re-reads every loaded race-trait record's `ABILITY:` tokens and requires the
//! set of grants that name another loaded trait to equal exactly those two. A
//! third such record appearing upstream fails this test rather than sliding in
//! unnoticed.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus, TraitRole};

fn corpus() -> RaceCorpus {
    let roots = [
        BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") },
        BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") },
        BookCorpusRoot { book_id: "advanced_race_guide", dir: Path::new("data/corpus/advanced_race_guide") },
    ];
    load_race_corpus(&roots)
}

/// Trait keys the resolver actually applies for `race` given `selected`.
fn applied_keys(corpus: &RaceCorpus, race: &str, selected: &[&str]) -> BTreeSet<String> {
    corpus
        .resolve(race, selected)
        .unwrap_or_else(|| panic!("race {race:?} must have a chassis record"))
        .traits
        .iter()
        .map(|t| t.key.clone())
        .collect()
}

#[test]
fn feral_languages_reaches_a_player_only_when_orc_feral_is_selected() {
    let corpus = corpus();

    let without = applied_keys(&corpus, "Orc", &[]);
    assert!(
        !without.contains("Feral ~ Languages"),
        "the replacement Languages row must not apply to a plain Orc"
    );
    assert!(
        without.contains("Orc ~ Languages"),
        "a plain Orc keeps its standard Languages row"
    );

    let with = applied_keys(&corpus, "Orc", &["Orc ~ Feral"]);
    assert!(
        with.contains("Feral ~ Languages"),
        "selecting `Orc ~ Feral` must grant `Feral ~ Languages` \
         (arg_abilities_race.lst:600 `ABILITY:Orc Racial Trait|AUTOMATIC|Feral ~ Languages`); \
         applied: {with:?}"
    );
    assert!(
        !with.contains("Orc ~ Languages"),
        "`Orc ~ Feral` sets `Orc_ReplaceLanguages`, which suppresses the standard row — \
         the replacement is what takes its place"
    );
}

#[test]
fn scion_of_humanity_languages_reaches_a_player_only_when_the_alternate_is_selected() {
    let corpus = corpus();

    let without = applied_keys(&corpus, "Aasimar", &[]);
    assert!(
        !without.contains("Scion of Humanity ~ Languages"),
        "the replacement Languages row must not apply to a plain Aasimar"
    );

    let with = applied_keys(&corpus, "Aasimar", &["Aasimar ~ Scion of Humanity"]);
    assert!(
        with.contains("Scion of Humanity ~ Languages"),
        "selecting `Aasimar ~ Scion of Humanity` must grant \
         `Scion of Humanity ~ Languages` (arg_abilities_race.lst:331 \
         `ABILITY:Aasimar Racial Trait|AUTOMATIC|Scion of Humanity ~ Languages`); \
         applied: {with:?}"
    );
}

/// The granted rows carry their real corpus prose, not just a key — the whole
/// point is that a player sees what replaced their languages.
#[test]
fn a_granted_row_arrives_with_its_corpus_name_and_description() {
    let corpus = corpus();
    let resolved = corpus.resolve("Orc", &["Orc ~ Feral"]).expect("Orc chassis");
    let row = resolved
        .traits
        .iter()
        .find(|t| t.key == "Feral ~ Languages")
        .expect("granted row present");

    assert_eq!(row.name, "Languages");
    assert_eq!(row.role, TraitRole::FlagGranted);
    assert_eq!(row.book_id, "advanced_race_guide");
    assert_eq!(
        row.description.as_deref(),
        Some(
            "Feral orcs begin play speaking no languages. Orcs with high Intelligence scores can \
             choose from among the following bonus languages: Dwarven, Giant, Gnoll, Goblin, \
             Undercommon."
        )
    );
}

/// A grant is *not* a selectable menu item. These rows arrive because another
/// trait named them; offering them directly would let a player take the
/// replacement without the replacement's cost.
#[test]
fn granted_rows_are_not_offered_as_selectable_alternates() {
    let corpus = corpus();
    for (race, key) in [("Orc", "Feral ~ Languages"), ("Aasimar", "Scion of Humanity ~ Languages")] {
        let offered: BTreeSet<&str> =
            corpus.alternate_traits(race).into_iter().map(|t| t.data.key.as_str()).collect();
        assert!(
            !offered.contains(key),
            "{key:?} must not appear in {race}'s selectable alternate menu"
        );
    }
}

/// Derived, not transcribed: scan every loaded race-trait record's `ABILITY:`
/// tokens for `AUTOMATIC` grants that name another *loaded race-trait record*,
/// and require the result to be exactly the two known edges.
///
/// Most `ABILITY:...|AUTOMATIC|...` tokens in `arg_abilities_race.lst` name
/// things that are not race-trait records at all — `FEAT`, `Class Skill`,
/// `Spell-Like Ability`, `Internal` trackers — and are correctly ignored here
/// because nothing in `data/corpus/*/race_trait/` answers to those keys.
#[test]
fn the_ability_automatic_grant_shape_is_exactly_two_records_corpus_wide() {
    let corpus = corpus();

    let mut edges: BTreeMap<String, String> = BTreeMap::new();
    for race in corpus.race_keys() {
        let records = corpus.traits_for(race);
        let known: BTreeSet<&str> = records.iter().map(|r| r.data.key.as_str()).collect();
        for record in &records {
            for granted in record.automatic_trait_grants() {
                if known.contains(granted.as_str()) && granted != record.data.key {
                    edges.insert(granted.clone(), record.data.key.clone());
                }
            }
        }
    }

    let expected: BTreeMap<String, String> = [
        ("Feral ~ Languages", "Orc ~ Feral"),
        ("Saltbeard ~ Dwarf ~ Greed", "Dwarf ~ Saltbeard"),
        ("Scion of Humanity ~ Languages", "Aasimar ~ Scion of Humanity"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b.to_owned()))
    .collect();

    assert_eq!(
        edges, expected,
        "the ABILITY|AUTOMATIC grant edges between loaded race-trait records changed; \
         `Saltbeard ~ Dwarf ~ Greed` is listed because `Dwarf ~ Saltbeard` names it BOTH ways \
         (it also carries the positive PREFACT gate), so it was already reachable"
    );
}

/// The residue: after this shape is read, no loaded race-trait record is left
/// without a gate the resolver understands.
#[test]
fn no_loaded_race_trait_record_is_unclassified_any_more() {
    let corpus = corpus();
    let stranded: Vec<&str> =
        corpus.unclassified_traits().into_iter().map(|t| t.data.key.as_str()).collect();
    assert!(
        stranded.is_empty(),
        "records with no readable gate remain: {stranded:?}"
    );
}
