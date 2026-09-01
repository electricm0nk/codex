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

/// Aasimar's and Tiefling's 48 heritage grant edges: 16 selectable heritages,
/// each naming its own three replacement rows.
///
/// **These edges are not new; this TEST is newly able to see them.** They are
/// the same `ABILITY:<Race> Racial Trait|AUTOMATIC|<key>` shape the eleven
/// literal entries above are, and they have been on disk since the SD-29
/// race-trait lane's round 4. `corpus()` above loads three book directories,
/// and until `SD31-CE-COMPANION-001` (2026-08-18) these records lived in a
/// fourth, `data/corpus/core_essentials/race_trait/`, which this file never
/// loaded -- so a test whose own name says "corpus-wide" was measuring 11 of 59
/// edges. `decisions.md §9` retired that book id and re-filed the records under
/// `advanced_race_guide`, which this file DOES load, and the shortfall surfaced
/// immediately. Recorded rather than absorbed: the count moved 11 -> 59 because
/// the denominator was wrong, not because the corpus grew.
///
/// Built from the 16 heritage names rather than written out as 48 literals, so
/// a heritage gaining or losing a replacement row still fails this assertion --
/// the three row names are fixed by the corpus (`aasimar_abilities_race_subrace.lst`
/// and `tiefling_abilities_race_subrace.lst` each replace exactly ability
/// scores, the skill bonus and the spell-like ability) and a fourth would not
/// be generated here.
fn heritage_grant_edges() -> impl Iterator<Item = (String, String)> {
    const AASIMAR: &[&str] = &[
        "Agathion-Blooded",
        "Angel-Blooded",
        "Archon-Blooded",
        "Azata-Blooded",
        "Garuda-Blooded",
        "Peri-Blooded",
    ];
    const TIEFLING: &[&str] = &[
        "Asura-Spawn",
        "Daemon-Spawn",
        "Demodand-Spawn",
        "Demon-Spawn",
        "Devil-Spawn",
        "Div-Spawn",
        "Kyton-Spawn",
        "Oni-Spawn",
        "Qlippoth-Spawn",
        "Rakshasa-Spawn",
    ];
    const REPLACED: &[&str] = &["Ability Scores", "Skilled", "Spell-Like Ability"];

    [("Aasimar", AASIMAR), ("Tiefling", TIEFLING)].into_iter().flat_map(|(race, heritages)| {
        heritages.iter().flat_map(move |heritage| {
            REPLACED
                .iter()
                .map(move |row| (format!("{heritage} ~ {row}"), format!("{race} ~ {heritage}")))
        })
    })
}

/// The 7 `Human ~ Adoptive Parentage` CHOOSE-pool members ARG ships as bare
/// race-name rows (`Drow`/`Dwarf`/`Elf`/`Gnome`/`Grippli`/`Halfling`/`Orc`,
/// `arg_abilities_race.lst`, e.g. `data/corpus/advanced_race_guide/
/// race_trait/dwarf/dwarf.json`), each granting its adopted race's own
/// `<Race> ~ Weapon Familiarity` and `<Race> ~ Languages` rows via a single
/// `ABILITY:...|AUTOMATIC|<Race> ~ Weapon Familiarity|<Race> ~ Languages`
/// token (`race_resolver.rs`'s own doc comment on `Unclassified`, and its
/// `no_corpus_trait_is_left_without_a_readable_gate` test, name and pin this
/// exact shape — the granting row is *itself* gated by `Human`'s CHOOSE pool,
/// which `classify()` correctly does not treat as a readable default/replace
/// gate, and `adopted_race_choose_selectors`/`trait_pool::
/// resolve_adopted_race_options` are the real readers). Of the 7, only
/// `Grippli`'s target rows live outside this file's 3-book `corpus()` (its
/// race chassis is `bestiary_2`), so only 6 races' edges are visible here —
/// `known.contains(granted)` correctly drops the 7th rather than fabricating
/// an edge to an unloaded record.
fn adoptive_parentage_grant_edges() -> impl Iterator<Item = (String, String)> {
    const RACES: &[&str] = &["Drow", "Dwarf", "Elf", "Gnome", "Halfling", "Orc"];
    RACES.iter().flat_map(|race| {
        ["Weapon Familiarity", "Languages"]
            .into_iter()
            .map(move |row| (format!("{race} ~ {row}"), race.to_string()))
    })
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
        // SD-31-E6-F4-003 (2026-08-16): ARG's own 6-race chassis batch's real
        // alternate-trait rows. Strix's `Wing-Clipped` grants its own
        // weaker-flight replacement row the same way `Dwarf ~ Saltbeard`
        // does above; Suli's `Energy Strike` grants all 4 of its
        // element-flavored follow-on abilities (`CHOOSE:STRING` sub-choices
        // this engine does not separately model, so all 4 arrive together
        // rather than only the chosen one -- a corpus-shape limit, not a
        // resolver bug, named here rather than silently narrowed).
        ("Wing-Clipped ~ Strix ~ Flight", "Strix ~ Wing-Clipped"),
        ("Suli ~ Earthfoot", "Suli ~ Energy Strike"),
        ("Suli ~ Firehand", "Suli ~ Energy Strike"),
        ("Suli ~ Icewalk", "Suli ~ Energy Strike"),
        ("Suli ~ Shockshield", "Suli ~ Energy Strike"),
        // SD31-E6-F4-006 (2026-08-17): ARG's own follow-on 4-race chassis
        // batch's real alternate-trait rows. Gillman's `Throwback` grants
        // both its replacement rows the same way (one `ABILITY:...
        // |AUTOMATIC|` token naming two keys), and Vanara's `Tree Stranger`
        // grants its own speed-replacement row the same way.
        ("Throwback ~ Gillman ~ Type", "Gillman ~ Throwback"),
        ("Throwback ~ Gillman ~ Speed", "Gillman ~ Throwback"),
        ("Tree Stranger ~ Vanara ~ Speed", "Vanara ~ Tree Stranger"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b.to_owned()))
    .chain(heritage_grant_edges())
    .chain(adoptive_parentage_grant_edges())
    .collect();

    assert_eq!(
        edges, expected,
        "the ABILITY|AUTOMATIC grant edges between loaded race-trait records changed; \
         `Saltbeard ~ Dwarf ~ Greed` is listed because `Dwarf ~ Saltbeard` names it BOTH ways \
         (it also carries the positive PREFACT gate), so it was already reachable"
    );
}

/// The residue is not empty any more, and — since SD-32 `decisions.md §25`
/// cycle 2 / AT-34-E3-001 — never will be by design. This test's original
/// name (`no_loaded_race_trait_record_is_unclassified_any_more`) asserted
/// `unclassified_traits()` was empty; that was true when this file was
/// written (2026-07-31) and stopped being true once a real, understood
/// residue shape started landing on purpose. `race_resolver.rs`'s own
/// module docs and its `no_corpus_trait_is_left_without_a_readable_gate`
/// test name and pin three CHOOSE-pool-gated shapes that `classify()`
/// **correctly** leaves `Unclassified` rather than inventing a fifth
/// `TraitRole` for — each is gated by a CHOOSE pool on a *different*
/// record, resolved by a dedicated reader
/// (`adopted_race_choose_selectors`/`adoptive_parentage_options`/
/// `trait_pool::resolve_adopted_race_options`), never by
/// `RaceCorpus::resolve`:
///
/// 1. `"Adopted Race ~ <Race>"` selector rows — `core_rulebook`'s own 7
///    (`AT-34-E3-001`, 2026-08-27; a `selector_only` `BookSource`, the same
///    pattern already proven for `bestiary_2`/`_3`/`_5`/`_6`, which this
///    file's 3-book `corpus()` does not load).
/// 2. `Human ~ Adoptive Parentage`'s 7 bare-race-name CHOOSE-pool members
///    (`advanced_race_guide`, SD-32 card-11 T2b, 2026-08-23) — gated by
///    `Human`'s own `CHOOSE:ABILITYSELECTION|Adoptive Parentage|ANY` pool,
///    not by any gate of their own.
/// 3. `core_rulebook`'s 2 `Human Ethnicity ~ {None,Unknown}` flavor
///    placeholders (`AT-34-E3-001`, `is_human_ethnicity_placeholder`) —
///    `CATEGORY:Background`, no `PREFACT`/default gate of any kind.
///
/// This test does NOT assert emptiness (an assertion the corpus's own
/// documented architecture now makes permanently false for this 3-book
/// scope) — it asserts the residue is EXACTLY this named, understood set,
/// derived from the corpus rather than transcribed, so a record landing
/// here for any OTHER reason (a genuine new gap, the kind
/// `Unclassified` exists to surface per the module's own doc comment)
/// still fails loudly, and so does one of these 16 silently disappearing.
#[test]
fn every_unclassified_race_trait_record_is_a_named_choose_pool_residue() {
    let corpus = corpus();
    let stranded: BTreeSet<String> =
        corpus.unclassified_traits().into_iter().map(|t| t.data.key.clone()).collect();

    let adopted_race_selectors = ["Dwarf", "Elf", "Gnome", "Half-Elf", "Half-Orc", "Halfling", "Human"]
        .into_iter()
        .map(|race| format!("Adopted Race ~ {race}"));
    let adoptive_parentage_pool_members =
        ["Drow", "Dwarf", "Elf", "Gnome", "Grippli", "Halfling", "Orc"].into_iter().map(str::to_string);
    let human_ethnicity_placeholders =
        ["Human Ethnicity ~ None", "Human Ethnicity ~ Unknown"].into_iter().map(str::to_string);

    let expected: BTreeSet<String> = adopted_race_selectors
        .chain(adoptive_parentage_pool_members)
        .chain(human_ethnicity_placeholders)
        .collect();

    assert_eq!(
        stranded, expected,
        "the corpus's documented Unclassified residue (CHOOSE-pool-gated rows resolved by a \
         dedicated reader, never by RaceCorpus::resolve -- race_resolver.rs's own module docs \
         name this shape) changed; a NEW entry is a real gap needing investigation, a MISSING \
         one means a record this list names was reclassified or removed and this test must \
         shrink to match"
    );
}
