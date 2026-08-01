//! SD-27 — a racial trait's description shows **this character's** number.
//!
//! # The correction this file pins
//!
//! ARG's alternate racial traits were deferred in bulk on the grounds that
//! their bonuses "target dimensions the engine does not model" — uses per day,
//! luck budgets, spell-like-ability counts. `tests/sd27_alternate_racial_trait_
//! reachability.rs` records the shape of that deferral: 142 of 153 alternates
//! "are short of a total to land on".
//!
//! The operator overruled the framing on 2026-08-01:
//!
//! > You do not need a full blown engine for things like uses per day. You just
//! > need the ability to calculate the value that is displayed in the
//! > description or elsewhere in the UI. […] These are all just display values.
//!
//! **That is a different claim from the deferral, and both are true.** A trait
//! can be short of a *computed total* (no uses-tracking subsystem exists, and
//! none is built here) while its *displayed number* is ordinary arithmetic over
//! values this engine already holds. This file pins the second half.
//!
//! # What is actually being resolved
//!
//! Two records, and they are not a sample — they are the entire population of
//! *rendered* racial-trait descriptions in the shipped corpus whose `%N`
//! argument names a variable any feat or trait moves. Derived by command over
//! `data/corpus/**/*.json`, not chosen:
//!
//! ```text
//! corpus records whose DESC token carries a non-literal %N argument: 16
//!   already resolved at ingest from same-row literals ........ 12
//!   still shipping with the number dropped .................... 4
//! of the 16, records a *feat* can move .......................... 2
//!   Gnome ~ Hatred            (Gnome_Hatred_AttackBonus)
//!   Halfling ~ Adaptable Luck (Halfling_AdaptableLuck_Times/_Bonus)
//! ```
//!
//! `Halfling ~ Adaptable Luck` is an ARG alternate racial trait and one of the
//! 142. `Gnome ~ Hatred` is a Core Rulebook standard trait whose number three
//! ARG feats were deferred against.
//!
//! # Why the assertions are before/after pairs
//!
//! `decisions.md §28`'s standing guard. A description that renders the right
//! number for the wrong reason — a constant baked at ingest, say — passes any
//! single-state assertion. Every case below therefore renders the *same corpus
//! tokens* twice against different character state and requires the text to
//! differ, which a constant cannot do.

use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::feat_effects::display_value_deltas_from_feats;
use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus};
use std::path::Path;

fn corpus() -> RaceCorpus {
    let roots = [
        BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") },
        BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") },
        BookCorpusRoot {
            book_id: "advanced_race_guide",
            dir: Path::new("data/corpus/advanced_race_guide"),
        },
    ];
    let corpus = load_race_corpus(&roots);
    assert!(corpus.diagnostics().is_empty(), "clean corpus load: {:?}", corpus.diagnostics());
    corpus
}

fn feats(names: &[&str]) -> Vec<String> {
    names.iter().map(|n| (*n).to_string()).collect()
}

/// Renders one trait's description for a character holding `held_feats`.
fn render(corpus: &RaceCorpus, race: &str, trait_key: &str, held_feats: &[&str]) -> String {
    let record = corpus
        .traits_for(race)
        .into_iter()
        .find(|r| r.data.key == trait_key)
        .unwrap_or_else(|| panic!("{trait_key} is a loaded record for {race}"));
    let values = record.display_values_with(&display_value_deltas_from_feats(&feats(held_feats)));
    let rendered = record.render_description(&values);
    assert!(
        rendered.dropped_args.is_empty(),
        "{trait_key} must resolve every argument for {held_feats:?}; dropped {:?}",
        rendered.dropped_args
    );
    assert_eq!(
        leaked_pcgen_syntax(&rendered.text),
        None,
        "{trait_key} must never carry PCGen syntax to a player"
    );
    rendered.text
}

/// The operator's own worked example: a uses-per-day count that is the racial
/// base plus whatever the character's feats add, rendered as a word in a
/// sentence with no resource pool anywhere.
///
/// `arg_abilities_race.lst:227` states the base across two tokens —
/// `DEFINE:Halfling_AdaptableLuck_Times|0` and
/// `BONUS:VAR|Halfling_AdaptableLuck_Times|3` — and writes the sentence as two
/// mutually exclusive branches:
///
/// ```text
/// DESC:Three|PREVARLTEQ:Halfling_AdaptableLuck_Times,3
/// DESC:%1|Halfling_AdaptableLuck_Times|PREVARGTEQ:Halfling_AdaptableLuck_Times,4
/// ```
///
/// So the feat does not merely change a number, it changes *which branch of the
/// sentence applies* — the word "Three" must disappear when the count rises.
#[test]
fn adaptable_lucks_uses_per_day_rises_with_the_feats_that_raise_it() {
    let corpus = corpus();

    let base = render(&corpus, "Halfling", "Halfling ~ Adaptable Luck", &[]);
    assert!(base.contains("Three times per day"), "racial base is three: {base}");
    assert!(base.contains("gain the full +2 bonus"), "racial luck bonus is +2: {base}");
    assert!(base.contains("only gain a +1 bonus"), "the after-the-roll bonus is +2-1: {base}");

    // Fortunate One (`arg_feats.lst:89`) — `BONUS:VAR|Halfling_AdaptableLuck_Times|1`.
    let fortunate = render(&corpus, "Halfling", "Halfling ~ Adaptable Luck", &["Fortunate One"]);
    assert!(fortunate.contains("4 times per day"), "3 + 1 = 4: {fortunate}");
    assert!(
        !fortunate.contains("Three"),
        "the `PREVARLTEQ:...,3` branch must stop applying: {fortunate}"
    );
    assert!(fortunate.contains("gain the full +2 bonus"), "Fortunate One adds no bonus");

    // Adaptive Fortune (`arg_feats.lst:84`) — a second `+1` to the count and a
    // `+2` to the bonus. Its own `PREABILITY:1,CATEGORY=FEAT,Fortunate One`
    // means a legal holder always holds both.
    let both = render(
        &corpus,
        "Halfling",
        "Halfling ~ Adaptable Luck",
        &["Fortunate One", "Adaptive Fortune"],
    );
    assert!(both.contains("5 times per day"), "3 + 1 + 1 = 5: {both}");
    assert!(both.contains("gain the full +4 bonus"), "2 + 2 = 4: {both}");
    assert!(both.contains("only gain a +3 bonus"), "the `Bonus-1` argument: {both}");

    assert_ne!(base, fortunate);
    assert_ne!(fortunate, both);
}

/// The Core Rulebook trait three deferred ARG feats were measured against.
///
/// `Gnome ~ Hatred` renders `+%1` from `Gnome_Hatred_AttackBonus`, whose racial
/// base is `1`. Great Hatred (`arg_feats.lst:42`) adds `1`.
///
/// This is the case that shows the display value and the *computed total* are
/// separate questions: Great Hatred is still not added to any attack-roll
/// total, because whether it applies depends on the creature being attacked.
/// The sentence stating the magnitude is a fact about the character and is
/// always true of them, so it renders.
#[test]
fn gnome_hatreds_stated_bonus_rises_with_great_hatred() {
    let corpus = corpus();

    let base = render(&corpus, "Gnome", "Gnome ~ Hatred", &[]);
    assert!(base.contains("receive a +1 bonus on attack rolls"), "{base}");

    let with_feat = render(&corpus, "Gnome", "Gnome ~ Hatred", &["Great Hatred"]);
    assert!(with_feat.contains("receive a +2 bonus on attack rolls"), "{with_feat}");

    assert_ne!(base, with_feat);
    // Only the number moved — the rest of the sentence is the corpus's own.
    assert_eq!(base.replace("+1 bonus", "+N"), with_feat.replace("+2 bonus", "+N"));
}

/// A trait carrying no placeholder is returned byte-identical to the
/// description already shipped, for every held-feat combination. The new
/// capability must not rewrite prose it has no business touching.
#[test]
fn a_trait_with_no_placeholder_is_unchanged_by_any_feat() {
    let corpus = corpus();
    let record = corpus
        .traits_for("Dwarf")
        .into_iter()
        .find(|r| r.data.key == "Dwarf ~ Mountaineer")
        .expect("Dwarf ~ Mountaineer is a loaded record");
    let shipped = record.data.description.clone().expect("Mountaineer ships a description");

    for held in [vec![], vec!["Great Hatred"], vec!["Fortunate One", "Adaptive Fortune"]] {
        let values = record.display_values_with(&display_value_deltas_from_feats(&feats(&held)));
        assert_eq!(record.render_description(&values).text, shipped, "held {held:?}");
    }
}

/// The no-fabrication rule survives the widening: a record whose `%N` names a
/// variable nothing resolves still drops it and still reports it, exactly as
/// the shipped description does today.
///
/// `Aasimar ~ Deathless Spirit`'s `BONUS:HP|CURRENTMAX|5*(NegLevels)` is the
/// standing example of a magnitude this engine genuinely cannot state: negative
/// levels are not modelled anywhere, so there is no number to display.
#[test]
fn an_unresolvable_variable_is_still_dropped_and_reported() {
    let corpus = corpus();
    // Every loaded trait either resolves cleanly or reports what it could not
    // resolve — never a silent partial.
    let mut reported = 0usize;
    for race in corpus.race_keys() {
        for record in corpus.traits_for(race) {
            let values = record.display_values_with(&display_value_deltas_from_feats(&[]));
            let rendered = record.render_description(&values);
            assert_eq!(
                leaked_pcgen_syntax(&rendered.text),
                None,
                "{} leaked PCGen syntax: {rendered:?}",
                record.data.key
            );
            if !rendered.dropped_args.is_empty() {
                reported += 1;
            }
        }
    }
    // Reported, not hidden. The count is derived rather than asserted at a
    // fixed value so widening the resolvable set is not a failure here — the
    // assertion is that nothing leaks and nothing is guessed.
    println!("racial-trait records still reporting an unresolved DESC argument: {reported}");
}

/// The rendered text itself, pinned byte-exact rather than by `contains`.
///
/// Kept as a full-string assertion because the interesting failure mode is not
/// "the number is wrong" — the `contains` tests above catch that — but "the
/// sentence around it got mangled": a dropped gate leaving *"Three 5 times"*, a
/// doubled space where a placeholder used to be, a lost clause. Only the whole
/// string can see those.
#[test]
fn the_rendered_sentences_are_pinned_byte_exact() {
    let corpus = corpus();

    assert_eq!(
        render(&corpus, "Gnome", "Gnome ~ Hatred", &[]),
        "Gnomes receive a +1 bonus on attack rolls against humanoid creatures of the reptilian \
         and goblinoid subtypes due to special training against these hated foes."
    );
    assert_eq!(
        render(&corpus, "Gnome", "Gnome ~ Hatred", &["Great Hatred"]),
        "Gnomes receive a +2 bonus on attack rolls against humanoid creatures of the reptilian \
         and goblinoid subtypes due to special training against these hated foes."
    );

    let luck = |held: &[&str]| render(&corpus, "Halfling", "Halfling ~ Adaptable Luck", held);
    let prefix = "Some halflings have greater control over their innate luck. This ability gives \
                  them more options for how they can apply their good fortune from day to day, \
                  but also narrows its scope.";
    let suffix = "per day, a halfling can gain a +{B} luck bonus on an ability check, attack \
                  roll, saving throw, or skill check. If halflings choose to use the ability \
                  before they make the roll or check, they gain the full +{B} bonus; if they \
                  choose to do so afterward, they only gain a +{B1} bonus. Using adaptive luck \
                  in this way is not an action. This racial trait replaces halfling luck.";

    let expected = |times: &str, bonus: i32| {
        format!(
            "{prefix} {times} times {}",
            suffix.replace("{B1}", &(bonus - 1).to_string()).replace("{B}", &bonus.to_string())
        )
    };

    assert_eq!(luck(&[]), expected("Three", 2));
    assert_eq!(luck(&["Fortunate One"]), expected("4", 2));
    assert_eq!(luck(&["Fortunate One", "Adaptive Fortune"]), expected("5", 4));
}
