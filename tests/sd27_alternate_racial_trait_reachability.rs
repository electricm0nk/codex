//! SD-27 — ARG's alternate racial traits reach the compute engine.
//!
//! ARG's 153 alternate racial traits were ingested, classified, and rendered by
//! a live picker that resolved every swap correctly — and no character could
//! take one. `RaceCorpus::resolve` implements `decisions.md §26`'s protocol,
//! `race_trait_picker` serves it, and the answer went nowhere: nothing wrote a
//! selection onto a `CharacterInput`, so `pilot_compute` never saw one and the
//! sheet never changed.
//!
//! This file pins the closure of that gap, and `decisions.md §28`'s standing
//! guard on `pilot_compute.rs` specifically: **every assertion below is a
//! before/after pair on the same race**, so a standard trait that stops being
//! suppressed — or starts being suppressed when it should not — is a caught
//! failure rather than a silent recomputation.
//!
//! # The three things being proven
//!
//! 1. **The hand-modelled table is the corpus.** `race_resolver`'s
//!    `ALTERNATE_TRAIT_REPLACE_FLAGS` exists because `pilot_compute` is a pure
//!    function that may not read the filesystem while `RaceCorpus` is a
//!    disk-backed load — the situation `decisions.md §24` prescribes a
//!    hand-modelled function for, and `RACE_SIZES` already occupies. Every flag
//!    the engine gates on is re-derived here from the real on-disk records.
//! 2. **A suppressed standard trait's effect really stops.** Dwarf is the
//!    worked case: taking `Dwarf ~ Saltbeard` removes the engine's grounded
//!    Greed (+2), Stonecunning (+2) and Defensive Training (+4) records, and
//!    only those.
//! 3. **A chosen alternate's own number really starts.** At both ends of the
//!    honesty spectrum this engine already draws: `Dwarf ~ Minesight` swaps a
//!    grounded *recognition* value (darkvision 60 ft → 90 ft), and eleven
//!    alternates move a *top-level computed total* — `Half-Elf ~ Dual Minded`
//!    the Will save, and ten more the Climb / Intimidate / Swim modifiers.
//!
//! # How the eleven were found, and why not more
//!
//! Not a choice — a measurement, re-run by
//! [`exactly_eleven_alternates_carry_a_bonus_that_lands_on_a_total_this_engine_computes`]
//! over every ingested record's `raw_bonus_chains` against the engine's whole computed-
//! total surface (`total_saves.{fortitude,reflex,will}` and
//! `selected_skill_modifiers.{climb,intimidate,swim}`), and asserted in both
//! directions: a row measured as reachable must also actually move the engine's
//! number by exactly the magnitude the corpus declares.
//!
//! The other 142 are short of a total to land on, not short of an
//! implementation. Their declared numbers are situational
//! (`BONUS:SITUATION|Perception=to notice flying creatures|2`), aimed at a
//! skill or stat this codebase computes nothing for (Perception, Fly,
//! Profession, Linguistics, initiative, caster level, spell DCs), or
//! formula-valued (`TL/2`, `1+Global_LuckBonus`, `5+(TL-HD)`) and therefore out
//! of reach under `decisions.md §24`'s no-interpreter ruling. Widen the
//! engine's totals and the measurement grows; the test will name what became
//! reachable rather than let it pass unnoticed.
//!
//! **The first count published for this was 1, and it was wrong** — this test
//! caught it, which is the entire reason it derives the set instead of
//! asserting it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput, SelectedChoice};
use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::pilot_compute::{
    compute_pilot_base_chassis, race_alternate_trait_selection_id, PilotBaseChassisComputation,
    RACE_ALTERNATE_TRAIT_CHOICE_ID,
};
use codex::rules_core::race_resolver::{
    alternate_traits_fire_flag, load_race_corpus, replace_flags_fired_by, selectable_alternate_trait_keys,
    unknown_alternate_trait_keys, RaceCorpus,
};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// The books the shipped app really loads, read out of its own
/// `RACE_CORPUS_BOOKS` declaration rather than duplicated here.
///
/// **This was a hardcoded `[core_rulebook, beastiary, advanced_race_guide]`,
/// and that made every "for every alternate in the corpus" assertion in this
/// file quietly three-book-scoped** while the app loaded five. The same
/// hardcoding, in `race_resolver.rs`'s own test module, is why four Monster
/// Codex alternates reached a player's picker and were then refused by
/// `pilot_compute` with a claim-blocking `race.alternate_trait.unknown` (SD-29
/// `decisions.md §44.2`). A test that cannot see a new book cannot report that
/// the new book broke something.
fn app_loaded_books() -> Vec<String> {
    let src = std::fs::read_to_string("apps/desktop/src-tauri/src/race_catalog.rs")
        .expect("the desktop race catalog source is readable from the repo root");
    let decl = src
        .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
        .nth(1)
        .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
    let list = decl.split(';').next().expect("the declaration terminates");
    list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
}

fn corpus() -> RaceCorpus {
    let dirs: Vec<(String, PathBuf)> = app_loaded_books()
        .into_iter()
        .map(|book| {
            let dir = PathBuf::from("data/corpus").join(&book);
            (book, dir)
        })
        .collect();
    let roots: Vec<BookCorpusRoot<'_>> = dirs
        .iter()
        .map(|(book, dir)| BookCorpusRoot { book_id: book.as_str(), dir: dir.as_path() })
        .collect();
    let corpus = load_race_corpus(&roots);
    assert!(corpus.diagnostics().is_empty(), "clean corpus load expected: {:?}", corpus.diagnostics());
    corpus
}

/// The shared deterministic Fighter 1 fixture with the race swapped and the
/// given alternate racial traits taken.
///
/// The selections are pushed as `SelectedChoice` entries under
/// [`RACE_ALTERNATE_TRAIT_CHOICE_ID`] — which is exactly what
/// `character_hub::create_character` writes, so this exercises the same channel
/// a real saved character travels through rather than a test-only side door.
fn input_for(race: &str, alternates: &[&str]) -> CharacterInput {
    let slug = race.to_lowercase();
    let text = DETERMINISTIC_FIXTURE.replace("race_id=race:human", &format!("race_id=race:{slug}"));
    let loaded = load_character_input_fixture(&text);
    assert!(loaded.diagnostics.is_empty(), "{race} fixture loads cleanly: {:?}", loaded.diagnostics);
    let mut input = loaded.character_input.expect("valid fixture yields a character input");
    for key in alternates {
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: RACE_ALTERNATE_TRAIT_CHOICE_ID.to_owned(),
            selection_id: race_alternate_trait_selection_id(key),
        });
    }
    input
}

fn computation_for(race: &str, alternates: &[&str]) -> PilotBaseChassisComputation {
    compute_pilot_base_chassis(&input_for(race, alternates))
}

fn explanation<'a>(computation: &'a PilotBaseChassisComputation, id: &str) -> Option<&'a str> {
    computation.explanations.iter().find(|e| e.id == id).map(|e| e.detail.as_str())
}

fn explanation_value(computation: &PilotBaseChassisComputation, id: &str) -> Option<i16> {
    computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

/// Every flag `pilot_compute` gates a hand-modelled standard-trait record on,
/// paired with the corpus record that must declare it.
///
/// Restated here rather than imported from the engine: a test whose expectation
/// is read out of the code under test cannot catch that code changing. This is
/// the same discipline `tests/sd27_size_modifiers_to_touch_cmb_cmd_and_attack.rs`
/// applies to `RACE_SIZES`.
const ENGINE_GATED_FLAGS: &[(&str, &str)] = &[
    ("Dwarf ~ Vision", "Dwarf_ReplaceVision"),
    ("Dwarf ~ Stonecunning", "Dwarf_ReplaceStonecunning"),
    ("Dwarf ~ Greed", "Dwarf_ReplaceGreed"),
    ("Dwarf ~ Hardy", "Dwarf_ReplaceHardy"),
    ("Dwarf ~ Stability", "Dwarf_ReplaceStability"),
    ("Dwarf ~ Defensive Training", "Dwarf_ReplaceDefensiveTraining"),
];

/// Every flag the engine gates on is the flag the corpus row itself declares.
///
/// A gate naming a flag no standard row carries would silently never fire, and
/// the swap would look implemented while doing nothing — the exact failure mode
/// `decisions.md §24` cites as the reason for hand-modelling over
/// interpretation.
#[test]
fn every_flag_the_engine_gates_a_standard_trait_on_is_the_one_the_corpus_row_declares() {
    let corpus = corpus();
    for (trait_key, flag) in ENGINE_GATED_FLAGS {
        let race_key = trait_key.split(" ~ ").next().expect("keys are '<Race> ~ <Trait>'");
        let record = corpus
            .traits_for(race_key)
            .into_iter()
            .find(|record| record.data.key == *trait_key)
            .unwrap_or_else(|| panic!("{trait_key} is a real corpus record"));
        assert!(record.data.is_racial_default, "{trait_key} is a standard racial trait");
        assert_eq!(
            record.data.suppressed_by_flag.as_deref(),
            Some(*flag),
            "{trait_key}: the engine gates on a flag the corpus row does not declare"
        );
    }
}

/// The pure table and the disk-backed resolver must agree on which flags a
/// selection fires — for every selectable alternate the app's own
/// `RACE_CORPUS_BOOKS` loads, not a
/// sample. This is what lets `pilot_compute` answer a corpus question without
/// touching the corpus.
#[test]
fn the_pure_flag_table_agrees_with_the_disk_backed_resolver_for_every_alternate() {
    let corpus = corpus();
    let mut checked = 0usize;
    for race_key in corpus.race_keys() {
        for record in corpus.alternate_traits(race_key) {
            let key = record.data.key.clone();
            let from_resolver = corpus
                .resolve(race_key, &[key.as_str()])
                .unwrap_or_else(|| panic!("{race_key} resolves"))
                .fired_flags;
            let from_table = replace_flags_fired_by(std::slice::from_ref(&key));
            assert_eq!(
                from_table,
                from_resolver.iter().map(String::as_str).collect::<Vec<&str>>(),
                "{key}: table and resolver disagree"
            );
            for flag in &from_resolver {
                assert!(alternate_traits_fire_flag(std::slice::from_ref(&key), flag), "{key} fires {flag}");
            }
            assert!(unknown_alternate_trait_keys(std::slice::from_ref(&key)).is_empty(), "{key} is known");
            checked += 1;
        }
    }
    assert_eq!(checked, 415, "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk alternates, 2026-08-23) + 1 APG + 76 Inner Sea Races (67 pre-existing + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + 41 Horror Adventures + 16 Core Essentials heritages + SD-31-E6-F4-003's 19 (round 5; the book's other 5 new records -- Strix's Wing-Clipped-granted Flight plus Suli's Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield -- are never selectable, same as the prior rounds' dependent rows) + SD31-E6-F4-006's 8 (round 6, 2026-08-17: Gillman 3, Nagaji 1, Vanara 2, Vishkanya 2) + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26: 9 kin selectors + their 36 replacement rows, all `TraitRole::Alternate`; each resolves clean here too -- `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section names the real flags every one of the 45 sets). The 158 this pin held until 2026-08-12 was round 2's miss, not a smaller corpus: ISR's 68 landed on 2026-08-11 and this assertion went RED unnoticed until round 3 reproduced the gate (SD-29 decisions.md 47)");
    assert_eq!(selectable_alternate_trait_keys().len(), 415, "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk alternates, 2026-08-23) + 1 APG + 76 Inner Sea Races (67 pre-existing + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + 41 Horror Adventures + 16 Core Essentials heritages + SD-31-E6-F4-003's 19 (round 5; the book's other 5 new records -- Strix's Wing-Clipped-granted Flight plus Suli's Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield -- are never selectable, same as the prior rounds' dependent rows) + SD31-E6-F4-006's 8 (round 6, 2026-08-17: Gillman 3, Nagaji 1, Vanara 2, Vishkanya 2) + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26: 9 kin selectors + their 36 replacement rows, all `TraitRole::Alternate`; each resolves clean here too -- `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section names the real flags every one of the 45 sets). The 158 this pin held until 2026-08-12 was round 2's miss, not a smaller corpus: ISR's 68 landed on 2026-08-11 and this assertion went RED unnoticed until round 3 reproduced the gate (SD-29 decisions.md 47)");
}

/// The three dependent rows named in this cycle's brief, confirmed by reading
/// the corpus rather than by accepting the claim.
///
/// * `Feral ~ Languages` (Orc) and `Scion of Humanity ~ Languages` (Aasimar)
///   are granted by the `ABILITY:<Race> Racial Trait|AUTOMATIC|<key>` token on
///   `Orc ~ Feral` / `Aasimar ~ Scion of Humanity`. **They used to be
///   `TraitRole::Unclassified` and reach nothing**; since that grant shape is
///   read they are `TraitRole::FlagGranted` and arrive with their granter.
///   Either way they are never auto-applied and never selectable, which is
///   what this test asserts and why its number did not move.
/// * `Saltbeard ~ Dwarf ~ Greed` is [`TraitRole::FlagGranted`]: it carries a
///   *positive* `PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True`, so choosing
///   `Dwarf ~ Saltbeard` brings it in. A player never picks it directly.
///
/// So ARG's 156 ingested records minus these 3 gave the menu its original 153;
/// Monster Codex's 4 and APG's 1 (SD-29 `decisions.md §44`) bring it to 158, and none of the
/// three is forced into it.
#[test]
fn the_three_dependent_rows_are_not_offered_as_choices_and_the_menu_is_exactly_the_alternates() {
    let corpus = corpus();
    let all: usize = corpus.race_keys().iter().map(|race| corpus.traits_for(race).len()).sum();
    let arg: usize = corpus
        .race_keys()
        .iter()
        .flat_map(|race| corpus.traits_for(race))
        .filter(|record| record.book_id == "advanced_race_guide")
        .count();
    assert_eq!(
        all,
        919,
        "175 standard + 156 ARG + 5 Monster Codex + 1 APG + 71 Inner Sea Races \
         + 43 Horror Adventures + 64 Core Essentials heritage rows + SD-31 Epic 1-F2's \
         113 (57 standard + 42 ARG + 6 Inner Sea Races + 8 grant-linked, 2026-08-15) + the \
         Skinwalker follow-on batch's 9 standard rows + SD-31-E6-F4-002's Advanced Race \
         Guide batch of 58 standard rows (2026-08-16: Catfolk, Kitsune, Ratfolk, Strix, \
         Suli, Wayang; 637 -> 695) + SD-31-E6-F4-003's own 24-record alternate-trait batch \
         for those same 6 races (2026-08-16: 695 -> 719) + SD31-E6-F4-004's Advanced Race \
         Guide follow-on batch of 38 standard rows (2026-08-17: Gillman, Nagaji, Vanara, \
         Vishkanya; 719 -> 757) + SD31-E6-F4-006's own 11-record alternate-trait batch for \
         those same 4 races (2026-08-17: 757 -> 768) + SD31-E6-F4-007's Advanced Race Guide \
         follow-on batch of 18 standard rows (2026-08-17: Changeling, Samsaran; 768 -> 786), \
         closing arg_races.lst's full 37-row playable-race roster + SD-31 wave-24's Rougarou \
         (Bestiary 6, 2026-08-20), 8 standard rows (786 -> 794) + SD-32 card-11 T2b lane's 18 \
         (2026-08-23: Dhampir's 12 standard rows + Monster Codex's 4 new Ratfolk alternates \
         + the 2 dependent rows Surface Sprinter grants; 794 -> 812) + a sibling SD-32 card-11 T2b lane's `inner_sea_races` stale-regen fix (2026-08-22): 9 new alternates + their 2 dependent rows + Suli ~ Trusted Mediator (Unclassified) = 12 (812 -> 824) + this cycle's own SD-32 card-11 T2b lane, 2026-08-23 (decisions.md §16 item 2): the 7 `Human ~ Adoptive Parentage` CHOOSE-pool members (824 -> 831) + SD-33 Epic 6's fold of SD31-E6-F4-005's lost wave-11 Skinwalker heritage lane (2026-08-26): 65 new records (9 kin selectors + 36 replacement rows + 20 shared `Change Shape (<Option>)` components) (831 -> 910) + a later ingest cycle (2026-09-01, re-derived): 7 core_rulebook `Adopted Race ~ <Race>` CHOOSE selectors + 2 `Human Ethnicity ~ {None,Unknown}` placeholder rows, neither book ARG (910 -> 919)"
    );
    assert_eq!(
        arg,
        421,
        "ARG's 421 ingested race-trait records (156 -> 201 by SD-31 Epic 1-F2; 201 -> 259 \
         by SD-31-E6-F4-002's own 6-race chassis batch; 259 -> 283 by SD-31-E6-F4-003's own \
         alternate-trait batch for those same 6 races, both 2026-08-16; 283 -> 321 by \
         SD31-E6-F4-004's own 4-race chassis batch, 2026-08-17; 321 -> 332 by \
         SD31-E6-F4-006's own alternate-trait batch for those same 4 races, 2026-08-17; \
         332 -> 350 by SD31-E6-F4-007's own 2-race chassis batch (Changeling, Samsaran), \
         2026-08-17, closing arg_races.lst's full 37-row playable-race roster; 350 -> 414 by \
         the Core Essentials removal, 2026-08-18; 414 -> 421 by SD-32 card-11 T2b lane, \
         2026-08-23, decisions.md §16 item 2: the 7 `Human ~ Adoptive Parentage` \
         CHOOSE-pool members, Drow/Dwarf/Elf/Gnome/Grippli/Halfling/Orc)"
    );

    let selectable: BTreeSet<&str> = selectable_alternate_trait_keys().into_iter().collect();
    assert_eq!(selectable.len(), 415, "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk alternates, 2026-08-23) + 1 APG + 76 Inner Sea Races (67 pre-existing + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + 41 Horror Adventures + 16 Core Essentials heritages + SD-31-E6-F4-003's 19 (round 5; the book's other 5 new records -- Strix's Wing-Clipped-granted Flight plus Suli's Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield -- are never selectable, same as the prior rounds' dependent rows) + SD31-E6-F4-006's 8 (round 6, 2026-08-17: Gillman 3, Nagaji 1, Vanara 2, Vishkanya 2) + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26: 9 kin selectors + their 36 replacement rows, all `TraitRole::Alternate`; each resolves clean here too -- `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section names the real flags every one of the 45 sets). The 158 this pin held until 2026-08-12 was round 2's miss, not a smaller corpus: ISR's 68 landed on 2026-08-11 and this assertion went RED unnoticed until round 3 reproduced the gate (SD-29 decisions.md 47)");
    for dependent in [
        "Feral ~ Languages",
        "Scion of Humanity ~ Languages",
        "Saltbeard ~ Dwarf ~ Greed",
        "Wing-Clipped ~ Strix ~ Flight",
        "Suli ~ Earthfoot",
        "Suli ~ Firehand",
        "Suli ~ Icewalk",
        "Suli ~ Shockshield",
    ] {
        assert!(!selectable.contains(dependent), "{dependent} must not be a menu item");
    }

    // ...and the flag-granted one still arrives, un-selected, with its parent.
    let saltbeard = corpus.resolve("Dwarf", &["Dwarf ~ Saltbeard"]).expect("Dwarf resolves");
    assert!(saltbeard.traits.iter().any(|t| t.key == "Saltbeard ~ Dwarf ~ Greed"));
    // ...and so do the two granted by an `ABILITY:<cat>|AUTOMATIC|<key>`
    // token. **This used to assert they "never auto-apply" even with every
    // alternate selected, and that was the defect, not the guarantee**: the
    // guarantee is that they are never *menu items* (asserted above), which
    // is unchanged. Selecting their granter must bring them in, and not
    // selecting it must not.
    for (race, granter, key) in [
        ("Orc", "Orc ~ Feral", "Feral ~ Languages"),
        ("Aasimar", "Aasimar ~ Scion of Humanity", "Scion of Humanity ~ Languages"),
    ] {
        let plain = corpus.resolve(race, &[]).expect("resolves");
        assert!(!plain.traits.iter().any(|t| t.key == key), "{key} must not auto-apply unchosen");
        let chosen = corpus.resolve(race, &[granter]).expect("resolves");
        assert!(chosen.traits.iter().any(|t| t.key == key), "{granter} must grant {key}");
    }
}

/// The storage namespace is lossless for every ingested record.
///
/// `SavedCharacterStore` rejects a `selection_id` with fewer than two
/// colon-segments and its line grammar splits on colons, so an alternate's
/// corpus key is persisted as `race_trait:<key>`. That only round-trips if no
/// key contains a colon of its own — asserted here over all 156 records rather
/// than assumed from the handful anyone has looked at.
#[test]
fn no_ingested_race_trait_key_contains_a_colon_so_the_storage_namespace_is_lossless() {
    let corpus = corpus();
    let mut checked = 0usize;
    for race_key in corpus.race_keys() {
        for record in corpus.traits_for(race_key) {
            let key = &record.data.key;
            assert!(!key.contains(':'), "{key} would not survive the selection-id grammar");
            let wrapped = race_alternate_trait_selection_id(key);
            assert!(wrapped.split(':').count() >= 2, "{wrapped} must satisfy the store's own check");
            assert_eq!(wrapped.strip_prefix("race_trait:"), Some(key.as_str()));
            assert!(!key.contains('\n'), "{key} must be a single line");
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        919,
        "515 -> 628 by SD-31 Epic 1-F2 (2026-08-15); 628 -> 637 by the Skinwalker follow-on \
         batch; 637 -> 695 by SD-31-E6-F4-002's Advanced Race Guide batch; 695 -> 719 by \
         SD-31-E6-F4-003's own alternate-trait batch for the same 6 races (2026-08-16); \
         719 -> 757 by SD31-E6-F4-004's own 4-race chassis batch (2026-08-17); 757 -> 768 \
         by SD31-E6-F4-006's own alternate-trait batch for the same 4 races (2026-08-17); \
         768 -> 786 by SD31-E6-F4-007's own 2-race chassis batch (Changeling, Samsaran), \
         2026-08-17, closing arg_races.lst's full 37-row playable-race roster; 786 -> 794 \
         by SD-31 wave-24's Rougarou (Bestiary 6, 2026-08-20), 8 standard rows; 794 -> 812 \
         by SD-32 card-11 T2b lane's Dhampir (12) + Monster Codex's 4 new Ratfolk \
         alternates + their 2 grant-linked rows (2026-08-23); 812 -> 824 by a sibling \
         SD-32 card-11 T2b lane's `inner_sea_races` stale-regen fix (2026-08-22): 9 new \
         alternates + their 2 dependent rows + Suli ~ Trusted Mediator (Unclassified); \
         824 -> 831 by this cycle's own SD-32 card-11 T2b lane, 2026-08-23 (decisions.md \
         §16 item 2): the 7 `Human ~ Adoptive Parentage` CHOOSE-pool members; 831 -> 910 by \
         SD-33 Epic 6's fold of SD31-E6-F4-005's lost wave-11 Skinwalker heritage lane \
         (2026-08-26): 65 new records (9 kin selectors + their 36 replacement rows + 20 \
         shared `Change Shape (<Option>)` components), none containing a colon; 910 -> 919 \
         by a later ingest cycle (2026-09-01, re-derived): 7 core_rulebook `Adopted Race ~ \
         <Race>` CHOOSE selectors (`ingest_race_traits.rs`) + 2 `Human Ethnicity ~ \
         {None,Unknown}` placeholder rows, none containing a colon"
    );
}

// ---------------------------------------------------------------------------
// §28's standing guard: before/after on the engine itself.
// ---------------------------------------------------------------------------

/// A Dwarf with no alternate chosen computes byte-identically to the
/// pre-SD-27 engine: all nine grounded Dwarf records, at their existing values.
///
/// This is the load-bearing half of the guard. Every other assertion in this
/// file is a *change*; this one is the proof the change is opt-in.
#[test]
fn a_dwarf_who_chose_no_alternate_keeps_every_grounded_record_at_its_existing_value() {
    let plain = computation_for("Dwarf", &[]);
    let expected: &[(&str, i16)] = &[
        ("race.dwarf.trait_bundle.ability_modifiers", 0),
        ("race.dwarf.trait_bundle.size", 0),
        ("race.dwarf.trait_bundle.speed", 20),
        ("race.dwarf.trait_bundle.senses", 60),
        ("race.dwarf.trait_bundle.stonecunning", 2),
        ("race.dwarf.trait_bundle.greed", 2),
        ("race.dwarf.trait_bundle.hardy", 2),
        ("race.dwarf.trait_bundle.stability", 4),
        ("race.dwarf.trait_bundle.defensive_training", 4),
    ];
    for (id, value) in expected {
        assert_eq!(explanation_value(&plain, id), Some(*value), "{id} for an unmodified Dwarf");
    }
    // No selection means no selection record, and no alternate-trait diagnostic.
    assert!(explanation(&plain, "race.alternate_trait.selected").is_none());
    assert!(plain.diagnostics.iter().all(|d| !d.id.starts_with("race.alternate_trait.")));
}

/// **The Dwarf swap, end to end.** `Dwarf ~ Saltbeard` sets four flags; three
/// of them name a standard trait this engine grounds a number for, and all
/// three of those numbers stop being emitted. The other five Dwarf records are
/// untouched — a suppression that reached too far would be just as wrong as one
/// that did not reach at all.
#[test]
fn taking_saltbeard_removes_exactly_the_three_grounded_dwarf_records_its_flags_name() {
    let before = computation_for("Dwarf", &[]);
    let after = computation_for("Dwarf", &["Dwarf ~ Saltbeard"]);

    // Gone: Greed (+2 Appraise), Stonecunning (+2 Perception), Defensive
    // Training (+4 dodge AC vs giants).
    for id in [
        "race.dwarf.trait_bundle.greed",
        "race.dwarf.trait_bundle.stonecunning",
        "race.dwarf.trait_bundle.defensive_training",
    ] {
        assert!(explanation_value(&before, id).is_some(), "{id} applies before the swap");
        assert_eq!(explanation_value(&after, id), None, "{id} must stop applying after the swap");
    }
    // Untouched: everything Saltbeard's flags do not name.
    for id in [
        "race.dwarf.trait_bundle.ability_modifiers",
        "race.dwarf.trait_bundle.size",
        "race.dwarf.trait_bundle.speed",
        "race.dwarf.trait_bundle.senses",
        "race.dwarf.trait_bundle.hardy",
        "race.dwarf.trait_bundle.stability",
    ] {
        assert_eq!(
            explanation_value(&after, id),
            explanation_value(&before, id),
            "{id} is not one of Saltbeard's flags and must be unchanged"
        );
    }

    // The choice itself is on the sheet, naming the flags it fired.
    let selected = explanation(&after, "race.alternate_trait.selected").expect("selection record");
    assert!(selected.contains("Dwarf ~ Saltbeard"), "{selected}");
    for flag in ["Dwarf_ReplaceDefensiveTraining", "Dwarf_ReplaceGreed", "Dwarf_ReplaceStonecunning"] {
        assert!(selected.contains(flag), "the record must name {flag}: {selected}");
    }
    assert!(after.diagnostics.iter().all(|d| d.id != "race.alternate_trait.unknown"));
}

/// **A number changing rather than merely disappearing.** `Dwarf ~ Minesight`
/// replaces the standard darkvision with a longer one, so the Dwarf's grounded
/// sense range moves 60 ft → 90 ft (ARG p.12, `VISION:Darkvision (90)`).
#[test]
fn taking_minesight_moves_a_dwarfs_grounded_darkvision_from_60_to_90_feet() {
    let before = computation_for("Dwarf", &[]);
    let after = computation_for("Dwarf", &["Dwarf ~ Minesight"]);

    assert_eq!(explanation_value(&before, "race.dwarf.trait_bundle.senses"), Some(60));
    assert_eq!(explanation_value(&before, "race.dwarf.alternate_trait.minesight.senses"), None);

    assert_eq!(
        explanation_value(&after, "race.dwarf.trait_bundle.senses"),
        None,
        "the standard 60 ft darkvision must stop applying"
    );
    assert_eq!(
        explanation_value(&after, "race.dwarf.alternate_trait.minesight.senses"),
        Some(90),
        "Minesight's own 90 ft darkvision must start applying"
    );
    let detail =
        explanation(&after, "race.dwarf.alternate_trait.minesight.senses").expect("Minesight record");
    assert!(detail.contains("arg_abilities_race.lst:39"), "the record cites the corpus line: {detail}");

    // The sibling alternate that replaces the same trait with *no* new sense
    // removes the record and adds nothing — an honest absence, not a 0.
    let survivalist = computation_for("Dwarf", &["Dwarf ~ Surface Survivalist"]);
    assert_eq!(explanation_value(&survivalist, "race.dwarf.trait_bundle.senses"), None);
    assert_eq!(explanation_value(&survivalist, "race.dwarf.alternate_trait.minesight.senses"), None);
}

/// **A top-level computed total moving.** `Half-Elf ~ Dual Minded`'s
/// `BONUS:SAVE|Will|2` is unconditional, so it layers into `total_saves.will`
/// the same way Iron Will already does — the first alternate racial trait in
/// this codebase to change a number the sheet prints at the top of the page.
#[test]
fn taking_dual_minded_raises_a_half_elfs_total_will_save_by_exactly_two() {
    let before = computation_for("Half-Elf", &[]);
    let after = computation_for("Half-Elf", &["Half-Elf ~ Dual Minded"]);

    assert_eq!(before.total_saves.will, 1, "Fighter 1 base Will +0, Wisdom 12 (+1)");
    assert_eq!(after.total_saves.will, 3, "+2 from Dual Minded");
    assert_eq!(explanation_value(&after, "defense.total_save.will"), Some(3));
    assert!(explanation(&after, "defense.total_save.will")
        .expect("will record")
        .contains("Dual Minded"));

    // Fortitude and Reflex are untouched — the corpus chain names Will alone.
    assert_eq!(after.total_saves.fortitude, before.total_saves.fortitude);
    assert_eq!(after.total_saves.reflex, before.total_saves.reflex);
    // ...and so is every other computed cell.
    assert_eq!(after.base_attack_bonus, before.base_attack_bonus);
    assert_eq!(after.baseline_armor_class, before.baseline_armor_class);
    assert_eq!(after.selected_skill_modifiers, before.selected_skill_modifiers);
}

/// The bonus is race-gated by construction: the same selection key on a race
/// that is not Half-Elf contributes nothing, so a copied or hand-edited saved
/// character cannot smuggle another race's trait onto this one.
#[test]
fn dual_minded_contributes_nothing_to_a_race_it_does_not_belong_to() {
    let dwarf_plain = computation_for("Dwarf", &[]);
    let dwarf_with_half_elf_trait = computation_for("Dwarf", &["Half-Elf ~ Dual Minded"]);
    assert_eq!(dwarf_with_half_elf_trait.total_saves.will, dwarf_plain.total_saves.will);
    // It is still reported as a held selection rather than vanishing, and no
    // Dwarf record is suppressed by it.
    assert!(explanation(&dwarf_with_half_elf_trait, "race.alternate_trait.selected").is_some());
    assert_eq!(explanation_value(&dwarf_with_half_elf_trait, "race.dwarf.trait_bundle.greed"), Some(2));
}

/// A selection key the engine cannot place raises a claim-blocking diagnostic
/// rather than being silently ignored. A saved character naming an unknown
/// racial trait has an unproven trait bundle, and every number derived from it
/// is unproven with it.
#[test]
fn an_unknown_alternate_trait_key_claim_blocks_instead_of_being_dropped() {
    let computation = computation_for("Dwarf", &["Dwarf ~ Saltbeerd"]);
    let diagnostic = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "race.alternate_trait.unknown")
        .expect("an unknown key must raise its own diagnostic");
    assert!(diagnostic.claim_blocking);
    assert!(diagnostic.message.contains("Dwarf ~ Saltbeerd"));
    // Nothing was suppressed on the strength of a key nobody recognized.
    assert_eq!(explanation_value(&computation, "race.dwarf.trait_bundle.greed"), Some(2));
}

/// Every selectable alternate, on its own race, through the real engine: no
/// panic, and no unknown-key diagnostic. A menu item the engine refuses would
/// be a dead affordance.
#[test]
fn every_alternate_computes_on_its_own_race_without_an_unknown_key_diagnostic() {
    let corpus = corpus();
    let mut computed = 0usize;
    for race_key in corpus.race_keys() {
        let race_token = race_key.to_lowercase();
        for record in corpus.alternate_traits(race_key) {
            let computation = computation_for(&race_token, &[record.data.key.as_str()]);
            assert!(
                computation.diagnostics.iter().all(|d| d.id != "race.alternate_trait.unknown"),
                "{} raised an unknown-key diagnostic",
                record.data.key
            );
            assert!(
                explanation(&computation, "race.alternate_trait.selected")
                    .is_some_and(|detail| detail.contains(record.data.key.as_str())),
                "{} must appear on the sheet",
                record.data.key
            );
            computed += 1;
        }
    }
    assert_eq!(computed, 415, "153 ARG + 8 Monster Codex (4 original + SD-32 card-11 T2b's 4 Ratfolk alternates, 2026-08-23) + 1 APG + 76 Inner Sea Races (67 pre-existing + 9 from a sibling SD-32 card-11 T2b lane's stale-regen fix, 2026-08-22) + 41 Horror Adventures + 16 Core Essentials heritages + SD-31-E6-F4-003's 19 (round 5; the book's other 5 new records -- Strix's Wing-Clipped-granted Flight plus Suli's Energy-Strike-granted Earthfoot/Firehand/Icewalk/Shockshield -- are never selectable, same as the prior rounds' dependent rows) + SD31-E6-F4-006's 8 (round 6, 2026-08-17: Gillman 3, Nagaji 1, Vanara 2, Vishkanya 2) + SD-33 Epic 6's 45 folded Skinwalker heritage records (2026-08-26: 9 kin selectors + their 36 replacement rows, all `TraitRole::Alternate`; each resolves clean here too -- `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` `Skinwalker` section names the real flags every one of the 45 sets). The 158 this pin held until 2026-08-12 was round 2's miss, not a smaller corpus: ISR's 68 landed on 2026-08-11 and this assertion went RED unnoticed until round 3 reproduced the gate (SD-29 decisions.md 47)");
}

/// **The measurement behind this cycle's honesty claim**, re-derived rather
/// than asserted: across every alternate's `raw_bonus_chains`, exactly one
/// declares a plain integer bonus on a total this engine computes.
///
/// The engine's computed-total surface is small and explicit:
/// `total_saves.{fortitude,reflex,will}` and
/// `selected_skill_modifiers.{climb,intimidate,swim}`. Everything else an ARG
/// alternate targets — Perception, Fly, Profession, Linguistics, initiative,
/// caster level, spell DCs, ability pools — has no total here to land on, so
/// wiring it would mean inventing the total first.
///
/// If a future cycle widens the engine's totals, this test fails and names the
/// alternates that became reachable. That is the intent: the list is a
/// measurement of today's engine, not a permanent verdict on the content.
///
/// **The name no longer carries the count** (`decisions.md §44.5` renamed three
/// tests for the same reason): a new book adds reachable alternates without the
/// property changing, and a test whose name has to be edited alongside its
/// expectation invites editing the expectation. Inner Sea Races took the set
/// from 11 to 15 and Horror Adventures to 16, and the list below is what
/// changed, not the claim.
#[test]
fn every_alternate_whose_bonus_lands_on_a_total_this_engine_computes_is_named_and_really_applies() {
    let corpus = corpus();
    // The names PCGen writes for the six totals this engine actually computes.
    let computed_totals: BTreeMap<&str, &str> = BTreeMap::from([
        ("Fortitude", "total_saves.fortitude"),
        ("Reflex", "total_saves.reflex"),
        ("Will", "total_saves.will"),
        ("Climb", "selected_skill_modifiers.climb"),
        ("Intimidate", "selected_skill_modifiers.intimidate"),
        ("Swim", "selected_skill_modifiers.swim"),
    ]);

    let mut landing: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // The record's own `race_key`, keyed by trait key -- not re-derived by
    // splitting the key string below. Every entry until SD-33 Epic 6's
    // Skinwalker fold (2026-08-26) happened to satisfy "<Race> ~ <Trait>",
    // so the split gave the right answer by coincidence; Skinwalker's own
    // heritage replacement rows are keyed "<Kin> ~ <Trait>" (e.g.
    // `Werebear-Kin ~ Animal-Minded`), where the string's first segment is
    // the HERITAGE name, not the race PCGen resolves against -- the record's
    // real `race_key` is `Skinwalker` regardless of which kin grants it.
    let mut race_of: BTreeMap<String, String> = BTreeMap::new();
    for race_key in corpus.race_keys() {
        for record in corpus.alternate_traits(race_key) {
            race_of.insert(record.data.key.clone(), race_key.to_string());
            for chain in &record.data.raw_bonus_chains {
                // `["SAVE", "Will", "2"]` / `["SKILL", "Swim", "4", "TYPE=Racial"]`.
                let (Some(head), Some(target), Some(magnitude)) =
                    (chain.qualifiers.first(), chain.qualifiers.get(1), chain.qualifiers.get(2))
                else {
                    continue;
                };
                if head != "SAVE" && head != "SKILL" {
                    continue;
                }
                // A formula, a variable, or a PCGen `%LIST` placeholder is out
                // of reach by `decisions.md §24` — it is not a plain integer.
                let Ok(magnitude) = magnitude.parse::<i32>() else {
                    continue;
                };
                for name in target.split(',') {
                    if let Some(total) = computed_totals.get(name.trim()) {
                        landing
                            .entry(record.data.key.clone())
                            .or_default()
                            .push(format!("{total} {magnitude:+}"));
                    }
                }
            }
        }
    }

    let reachable: Vec<(&str, Vec<&str>)> = landing
        .iter()
        .map(|(key, hits)| (key.as_str(), hits.iter().map(String::as_str).collect()))
        .collect();
    assert_eq!(
        reachable,
        vec![
            // ARG's original 11.
            ("Dwarf ~ Unstoppable", vec!["total_saves.fortitude +1"]), // ISR
            ("Elf ~ Spirit of the Waters", vec!["selected_skill_modifiers.swim +4"]),
            ("Gnome ~ Explorer", vec!["selected_skill_modifiers.climb +2"]),
            // ISR: the only alternate in the corpus landing on two different
            // computed totals at once.
            (
                "Gnome ~ Intrepid Settler",
                vec!["selected_skill_modifiers.climb +2", "selected_skill_modifiers.swim +2"],
            ),
            ("Goblin ~ Tree Runner", vec!["selected_skill_modifiers.climb +4"]),
            // SD-31 Epic 1-F2 (2026-08-15), the batch's own contribution to
            // this set: `BONUS:SKILL|Diplomacy,Intimidate|2|TYPE=Racial` --
            // Diplomacy is not one of the six tracked totals, so only
            // Intimidate lands here.
            ("Grippli ~ Princely", vec!["selected_skill_modifiers.intimidate +2"]),
            ("Half-Elf ~ Dual Minded", vec!["total_saves.will +2"]),
            // Horror Adventures' single contribution to this set, and the only
            // NEGATIVE magnitude in it -- `Half-Elf ~ Mismatched` trades a
            // reflex save away. The delta check below is therefore exercised
            // in both directions for the first time.
            ("Half-Elf ~ Mismatched", vec!["total_saves.reflex -2"]), // HA
            ("Half-Elf ~ Sea Legs", vec!["selected_skill_modifiers.swim +2"]), // ISR
            ("Half-Elf ~ Water Child", vec!["selected_skill_modifiers.swim +4"]),
            ("Half-Orc ~ Forest Walker", vec!["selected_skill_modifiers.climb +2"]),
            ("Half-Orc ~ Rock Climber", vec!["selected_skill_modifiers.climb +1"]),
            ("Hobgoblin ~ Authoritative", vec!["selected_skill_modifiers.intimidate +2"]), // ISR
            ("Hobgoblin ~ Bandy-Legged", vec!["selected_skill_modifiers.climb +2"]),
            ("Hobgoblin ~ Fearsome", vec!["selected_skill_modifiers.intimidate +4"]),
            ("Human ~ Heart of the Mountain", vec!["selected_skill_modifiers.climb +2"]),
            ("Human ~ Heart of the Sea", vec!["selected_skill_modifiers.swim +2"]),
            // SD-31-E6-F4-003 (2026-08-16), ARG's own 6-race chassis batch's
            // contribution: Strix is the only one of the 6 with a `BONUS:SAVE`
            // or `BONUS:SKILL` chain landing on a tracked total.
            ("Strix ~ Frightening", vec!["selected_skill_modifiers.intimidate +2"]),
            ("Strix ~ Nimble", vec!["total_saves.reflex +1"]),
            ("Strix ~ Tough", vec!["total_saves.fortitude +1"]),
            ("Strix ~ Wing-Clipped", vec!["selected_skill_modifiers.climb +2"]),
            // SD-33 Epic 6's fold of SD31-E6-F4-005's lost wave-11 Skinwalker
            // heritage lane (2026-08-26). Of the fold's 45 `Alternate`
            // records, exactly these 2 of the 9 kins' `~ Animal-Minded`
            // replacement rows land a `BONUS:SKILL` on a tracked total --
            // Werebear-Kin's `BONUS:SKILL|Climb|2|TYPE=Racial` and
            // Wereshark-Kin's `BONUS:SKILL|Swim|2|TYPE=Racial`
            // (`skinwalker_abilities_race_subrace.lst`). The other 7 kins'
            // `~ Animal-Minded` rows land on Fly/Perception/Stealth/Survival
            // /Perception-at-night, none of which is one of the six tracked
            // totals, so they contribute nothing here (a `BONUS:VAR`
            // `WildEmpathy` chain on Werebear-Kin's own row is also excluded
            // the same way, since VAR is not SAVE/SKILL).
            ("Werebear-Kin ~ Animal-Minded", vec!["selected_skill_modifiers.climb +2"]),
            ("Wereshark-Kin ~ Animal-Minded", vec!["selected_skill_modifiers.swim +2"]),
        ],
        "the reachable set is a measurement of today's engine; a change here is a real change. \
         Round 2 added ISR's 4 without moving this list, so it was RED on the branch; round 3 \
         moved it and added HA's 1 (SD-29 decisions.md 47); round 5 (SD-31-E6-F4-003) added \
         Strix's 4, 2026-08-16; SD-33 Epic 6's Skinwalker fold (2026-08-26) added Werebear-Kin \
         and Wereshark-Kin's `~ Animal-Minded` rows"
    );

    // Every one of them is genuinely wired: the engine's own before/after
    // delta must equal the magnitude the corpus declares. A row measured as
    // reachable but not applied would be exactly the browse-only defect this
    // cycle exists to close.
    for (key, hits) in &reachable {
        let race_token = race_of
            .get(*key)
            .unwrap_or_else(|| panic!("{key}: race_of must carry every reachable key"))
            .to_lowercase();
        let before = computation_for(&race_token, &[]);
        let after = computation_for(&race_token, &[key]);
        for hit in hits {
            let (total, magnitude) = hit.rsplit_once(' ').expect("'<total> +N'");
            let magnitude: i16 = magnitude.parse().expect("integer magnitude");
            let delta = match total {
                // `fortitude` and `reflex` were declared in `computed_totals`
                // above but had no arm here, because no ARG alternate landed on
                // them. Inner Sea Races' `Dwarf ~ Unstoppable` and Horror
                // Adventures' `Half-Elf ~ Mismatched` do, and this match's
                // `other =>` arm panics rather than skipping -- so the gap was
                // fail-loud, not silent, and is closed rather than worked
                // around (SD-29 `decisions.md §47`).
                "total_saves.fortitude" => after.total_saves.fortitude - before.total_saves.fortitude,
                "total_saves.reflex" => after.total_saves.reflex - before.total_saves.reflex,
                "total_saves.will" => after.total_saves.will - before.total_saves.will,
                "selected_skill_modifiers.climb" => {
                    after.selected_skill_modifiers.climb - before.selected_skill_modifiers.climb
                }
                "selected_skill_modifiers.intimidate" => {
                    after.selected_skill_modifiers.intimidate - before.selected_skill_modifiers.intimidate
                }
                "selected_skill_modifiers.swim" => {
                    after.selected_skill_modifiers.swim - before.selected_skill_modifiers.swim
                }
                other => panic!("{other} is not one of the totals this test knows"),
            };
            assert_eq!(delta, magnitude, "{key}: {total} must move by exactly {magnitude}");
        }
    }
}

/// **The invariant that makes summing the save table correct.**
///
/// `pilot_compute::alternate_trait_save_bonuses` adds its contributions rather
/// than maximising them, because one of the three is a *penalty*
/// (`Half-Elf ~ Mismatched`, -2 Reflex) and maximising would discard it. Summing
/// is only right while no race can contribute twice to the same save — with two
/// same-typed racial bonuses on one save, PF1 says the highest applies and the
/// sum would be wrong.
///
/// Derived from the corpus rather than from the table, so a future book that
/// breaks the invariant fails here even if nobody remembers the table exists.
/// SD-29 `decisions.md §47`.
#[test]
fn no_race_contributes_two_alternate_trait_bonuses_to_one_save() {
    let corpus = corpus();
    let mut per_race_save: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for race_key in corpus.race_keys() {
        for record in corpus.alternate_traits(race_key) {
            for chain in &record.data.raw_bonus_chains {
                let mut q = chain.qualifiers.iter();
                if q.next().map(String::as_str) != Some("SAVE") {
                    continue;
                }
                let Some(save) = q.next() else { continue };
                let Some(magnitude) = q.next() else { continue };
                if magnitude.parse::<i32>().is_err() {
                    continue;
                }
                if !["Fortitude", "Reflex", "Will"].contains(&save.as_str()) {
                    continue;
                }
                per_race_save
                    .entry((race_key.to_string(), save.clone()))
                    .or_default()
                    .push(record.data.key.clone());
            }
        }
    }
    for ((race, save), keys) in &per_race_save {
        assert_eq!(
            keys.len(),
            1,
            "{race}'s {save} save is moved by {} alternate racial traits ({keys:?}). \
             `alternate_trait_save_bonuses` SUMS its contributions, which is only correct while \
             this is 1. Decide the stacking rule for this pair before ingesting it",
            keys.len()
        );
    }
    // The scan must find something, or it proves nothing.
    assert_eq!(
        per_race_save.len(),
        5,
        "Half-Elf/Will (ARG Dual Minded), Dwarf/Fortitude (ISR Unstoppable), Half-Elf/Reflex \
         (HA Mismatched), Strix/Reflex (ARG Nimble), Strix/Fortitude (ARG Tough) -- the five \
         (race, save) pairs any alternate moves (SD-31-E6-F4-003 added the last two, \
         2026-08-16)"
    );
}

/// Two racial bonuses to the same skill do not stack — PF1's rule for same-typed
/// named bonuses is that only the highest applies, and all ten corpus chains
/// carry `TYPE=Racial`.
///
/// The pair is real and legal: `Half-Orc ~ Forest Walker` (+2 Climb) replaces
/// vision and `Half-Orc ~ Rock Climber` (+1 Climb) replaces Intimidating, so
/// they fire different flags and ARG's own `PREMULT` guard does not exclude
/// them from each other. A Half-Orc who takes both gets +2, not +3.
#[test]
fn two_racial_climb_bonuses_on_one_character_take_the_higher_rather_than_the_sum() {
    let base = computation_for("Half-Orc", &[]).selected_skill_modifiers.climb;
    let forest_walker = computation_for("Half-Orc", &["Half-Orc ~ Forest Walker"]);
    let rock_climber = computation_for("Half-Orc", &["Half-Orc ~ Rock Climber"]);
    let both = computation_for("Half-Orc", &["Half-Orc ~ Forest Walker", "Half-Orc ~ Rock Climber"]);

    assert_eq!(forest_walker.selected_skill_modifiers.climb, base + 2);
    assert_eq!(rock_climber.selected_skill_modifiers.climb, base + 1);
    assert_eq!(both.selected_skill_modifiers.climb, base + 2, "the higher applies, not +3");

    // Both swaps still happened: each suppressed its own standard trait.
    let selected = explanation(&both, "race.alternate_trait.selected").expect("selection record");
    assert!(selected.contains("HalfOrc_ReplaceVision"), "{selected}");
    assert!(selected.contains("HalfOrc_ReplaceIntimidating"), "{selected}");
}

/// Every other race keeps its own numbers when a Dwarf takes an alternate:
/// suppression is per-character, not global state.
#[test]
fn one_characters_swap_does_not_leak_into_another_races_computation() {
    let dwarf_swapped = computation_for("Dwarf", &["Dwarf ~ Saltbeard"]);
    assert_eq!(explanation_value(&dwarf_swapped, "race.dwarf.trait_bundle.greed"), None);

    for race in ["Elf", "Gnome", "Half-Elf", "Half-Orc", "Halfling", "Human"] {
        let plain = computation_for(race, &[]);
        let after_the_dwarf = computation_for(race, &[]);
        assert_eq!(plain.total_saves, after_the_dwarf.total_saves, "{race} total saves");
        assert_eq!(plain.baseline_armor_class, after_the_dwarf.baseline_armor_class, "{race} AC");
    }
    // ...and a second Dwarf who chose nothing still has Greed.
    assert_eq!(explanation_value(&computation_for("Dwarf", &[]), "race.dwarf.trait_bundle.greed"), Some(2));
}
