//! SD-27: the true creature size of all 18 in-scope races, and the two
//! wrong-math defects that made three of them resolve the wrong one.
//!
//! # Why this file exists
//!
//! Creature size is not decoration. `encumbrance.rs` scales every carrying
//! capacity threshold by `SizeCategory::load_capacity_ratio` (`load.lst`'s
//! `SIZEMULT:S|0.75`), and the load tier that falls out of those thresholds
//! carries a max-Dex cap and an armor check penalty. A race resolved one
//! size wrong hands the player a wrong carry number, a wrong tier, a wrong
//! Dex cap and a wrong ACP — four visible numbers off one bad letter.
//!
//! Two independent defects each produced exactly that, and each is pinned
//! below.
//!
//! ## Defect 1 — the chassis row is not the whole truth about size
//!
//! `ResolvedRace::size` read the chassis row's `FACT:BaseSize|<code>` and
//! nothing else. For **Aasimar and Tiefling that token is `S`, and both
//! races are Medium.** The real declaration is on the race's own `~ Size`
//! racial-default trait row, which carries `TEMPLATE:SIZE_M`:
//!
//! ```text
//! # core_essentials/races/aasimar/aasimar_abilities_race.lst:19
//! Medium  KEY:Aasimar ~ Size  TYPE:RacialTraits.Aasimar Racial Trait.Aasimar Racial Default.SpecialQuality.Racial Size
//!         DESC:Aasimars are Medium creatures and have no bonuses or penalties due to their size.
//!         TEMPLATE:SIZE_M  SOURCEPAGE:p.7
//!
//! # core_essentials/races/tiefling/tiefling_abilities_race.lst:17
//! Medium  KEY:Tiefling ~ Size  ... TEMPLATE:SIZE_M ...
//! ```
//!
//! and `SIZE_M` is a real template whose whole body is a size assignment
//! (`core_essentials/ce_templates.lst:924-933`):
//!
//! ```text
//! SIZE_F SIZE:F   SIZE_D SIZE:D   SIZE_T SIZE:T   SIZE_S SIZE:S
//! SIZE_M SIZE:M   SIZE_L SIZE:L   SIZE_H SIZE:H   SIZE_G SIZE:G   SIZE_C SIZE:C
//! ```
//!
//! This is the same "chassis row is not the whole truth" shape
//! `race_resolver.rs` already documents for Goblin's and Hobgoblin's
//! `MOVE:Walk,0`. Reading `SIZE_M` off the row that declares it is
//! transcription, not formula interpretation, so `decisions.md §24` permits
//! it.
//!
//! ### Why the chassis says `S` — stated by PCGen, not inferred
//!
//! `FACT:BaseSize` does not mean "this race's size". Its own field
//! definition says what it means
//! (`core_essentials/ce__datacontrols.lst:22`):
//!
//! ```text
//! FACTDEF:RACE|BaseSize  DATAFORMAT:SIZEADJUSTMENT  REQUIRED:YES  VISIBLE:YES
//!     EXPLANATION:All Races must have a Size - in the case of multiple sizes,
//!                 use the SMALLEST allowed.
//! ```
//!
//! Aasimar and Tiefling each have a legal Small variant, granted opt-in by
//! a book outside this repo's scope — `blood_of_angels/boa_abilities_race.lst:16`
//! adds `Aasimar ~ Size (Small)` behind `PREABILITY:1,CATEGORY=Racial Size,Race Size ~ S`,
//! and `blood_of_fiends/bof_abilities_race.lst:14` does the same for Tiefling.
//! So `S` is their smallest *allowed* size, exactly as the `FACTDEF` directs,
//! and Medium is their default. Reading `FACT:BaseSize` as the race's size
//! was a misreading of a field whose own `EXPLANATION:` says otherwise.
//!
//! ## Defect 2 — `race_size_for_race_id` knew only 7 of the 18 races
//!
//! `rules_tables::crb::race_tables::race_size_for_race_id` is a
//! seven-variant `RaceId` lookup. It returned `None` for all 11 Bestiary 1
//! races, and both of its call sites
//! (`contract::to_pilot_receipt`, `pilot_compute_corpus::compute_pilot_with_corpus`)
//! did `.unwrap_or(SizeCategory::Medium)`. **Goblin, Kobold and Svirfneblin
//! are genuinely Small** and were therefore handed 4/3 of their true
//! carrying capacity — the exact defect `size.rs` was created to remove for
//! Gnome and Halfling, still live for three more races.
//!
//! # How the expected values below were established
//!
//! Not from memory. Per race, both declarations were read out of the PCGen
//! checkout the corpus is ingested from
//! (`$PCGEN/data/pathfinder/paizo/roleplaying_game/core_essentials/races/`):
//!
//! ```text
//! chassis:  grep -oE 'FACT:BaseSize\|[A-Z]+'   <race>/<race>_races.lst
//! trait:    grep -oE 'TEMPLATE:SIZE_[A-Z]+'    <race>/<race>_abilities_race.lst
//! ```
//!
//! 17 of the 18 carry a `TEMPLATE:SIZE_` on their `~ Size` trait; Human's
//! `~ Size` row carries none, and its chassis says `M`. The two disagree
//! for exactly two races, Aasimar and Tiefling, and in both the trait row
//! is right — its own `DESC:` says "Medium creatures" in prose on the same
//! line as the template, and the `FACTDEF` above explains the `S`.
//! `SIZE_TRUTH` below records both readings so the disagreement stays
//! visible rather than being smoothed away.

use codex::rules_core::contract::{encumbrance_size_for_race, UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID};
use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::encumbrance::carrying_capacity_thresholds;
use codex::rules_core::race_resolver::{
    load_race_corpus, race_size_for_race_token, RaceCorpus, SizeSource,
};
use codex::rules_core::size::SizeCategory;
use std::path::Path;

/// One row per in-scope race: the `race:<slug>` character-input token, the
/// corpus race key, what the **chassis** row's `FACT:BaseSize` says, and
/// what the race's size **actually is**.
///
/// Where the two columns differ, the fourth column is the answer and the
/// third is the defect. Both were read by command from the PCGen source —
/// see this file's module docs for the exact greps.
const SIZE_TRUTH: &[(&str, &str, SizeCategory, SizeCategory)] = &[
    // Core Rulebook's 7. Chassis and trait agree for every one of them,
    // which is why defect 1 never showed up here.
    ("race:dwarf", "Dwarf", SizeCategory::Medium, SizeCategory::Medium),
    ("race:elf", "Elf", SizeCategory::Medium, SizeCategory::Medium),
    ("race:gnome", "Gnome", SizeCategory::Small, SizeCategory::Small),
    ("race:half-elf", "Half-Elf", SizeCategory::Medium, SizeCategory::Medium),
    ("race:half-orc", "Half-Orc", SizeCategory::Medium, SizeCategory::Medium),
    ("race:halfling", "Halfling", SizeCategory::Small, SizeCategory::Small),
    ("race:human", "Human", SizeCategory::Medium, SizeCategory::Medium),
    // Bestiary 1's 11. Aasimar and Tiefling are the two disagreements.
    ("race:aasimar", "Aasimar", SizeCategory::Small, SizeCategory::Medium),
    ("race:drow", "Drow", SizeCategory::Medium, SizeCategory::Medium),
    ("race:duergar", "Duergar", SizeCategory::Medium, SizeCategory::Medium),
    ("race:goblin", "Goblin", SizeCategory::Small, SizeCategory::Small),
    ("race:hobgoblin", "Hobgoblin", SizeCategory::Medium, SizeCategory::Medium),
    ("race:kobold", "Kobold", SizeCategory::Small, SizeCategory::Small),
    ("race:merfolk", "Merfolk", SizeCategory::Medium, SizeCategory::Medium),
    ("race:orc", "Orc", SizeCategory::Medium, SizeCategory::Medium),
    ("race:svirfneblin", "Svirfneblin", SizeCategory::Small, SizeCategory::Small),
    ("race:tengu", "Tengu", SizeCategory::Medium, SizeCategory::Medium),
    ("race:tiefling", "Tiefling", SizeCategory::Small, SizeCategory::Medium),
    // Bestiary 2's 6, SD-31 Epic 1-F2 (2026-08-15). Chassis and trait agree
    // for every one of them (none carries an Aasimar/Tiefling-shaped
    // chassis/trait disagreement).
    ("race:fetchling", "Fetchling", SizeCategory::Medium, SizeCategory::Medium),
    ("race:grippli", "Grippli", SizeCategory::Small, SizeCategory::Small),
    ("race:ifrit", "Ifrit", SizeCategory::Medium, SizeCategory::Medium),
    ("race:oread", "Oread", SizeCategory::Medium, SizeCategory::Medium),
    ("race:sylph", "Sylph", SizeCategory::Medium, SizeCategory::Medium),
    ("race:undine", "Undine", SizeCategory::Medium, SizeCategory::Medium),
    // Bestiary 5's 1, the Skinwalker follow-on batch (2026-08-15). Same
    // chassis/trait disagreement shape as Aasimar/Tiefling: chassis
    // `FACT:BaseSize|S`, `~ Size` row `TEMPLATE:SIZE_M`.
    ("race:skinwalker", "Skinwalker", SizeCategory::Small, SizeCategory::Medium),
    // Advanced Race Guide's 6, SD-31-E6-F4-002 (2026-08-16). Chassis and
    // trait agree for every one of them (none carries an Aasimar/Tiefling/
    // Skinwalker-shaped disagreement).
    ("race:catfolk", "Catfolk", SizeCategory::Medium, SizeCategory::Medium),
    ("race:kitsune", "Kitsune", SizeCategory::Medium, SizeCategory::Medium),
    ("race:ratfolk", "Ratfolk", SizeCategory::Small, SizeCategory::Small),
    ("race:strix", "Strix", SizeCategory::Medium, SizeCategory::Medium),
    ("race:suli", "Suli", SizeCategory::Medium, SizeCategory::Medium),
    ("race:wayang", "Wayang", SizeCategory::Small, SizeCategory::Small),
    // Advanced Race Guide's 4-race follow-on, SD31-E6-F4-004 (2026-08-17).
    // Chassis and trait agree for every one of them (none carries an
    // Aasimar/Tiefling/Skinwalker-shaped disagreement).
    ("race:gillman", "Gillman", SizeCategory::Medium, SizeCategory::Medium),
    ("race:nagaji", "Nagaji", SizeCategory::Medium, SizeCategory::Medium),
    ("race:vanara", "Vanara", SizeCategory::Medium, SizeCategory::Medium),
    ("race:vishkanya", "Vishkanya", SizeCategory::Medium, SizeCategory::Medium),
    // Advanced Race Guide's 2-race follow-on, SD31-E6-F4-007 (2026-08-17),
    // closing `arg_races.lst`'s full 37-row playable-race roster. Chassis
    // and trait agree for both (`FACT:BaseSize|M` / `TEMPLATE:SIZE_M`, no
    // Aasimar/Tiefling/Skinwalker-shaped disagreement).
    ("race:changeling", "Changeling", SizeCategory::Medium, SizeCategory::Medium),
    ("race:samsaran", "Samsaran", SizeCategory::Medium, SizeCategory::Medium),
    // Bestiary 6's 1, SD-31 wave-24 (2026-08-20). Chassis and trait agree
    // (`FACT:BaseSize|M` / `TEMPLATE:SIZE_M`, no Aasimar/Tiefling/
    // Skinwalker-shaped disagreement).
    ("race:rougarou", "Rougarou", SizeCategory::Medium, SizeCategory::Medium),
    // Bestiary 2's Dhampir, SD-32 card-11 T2b lane (2026-08-23). Chassis
    // and trait agree (`FACT:BaseSize|M` / `TEMPLATE:SIZE_M`).
    ("race:dhampir", "Dhampir", SizeCategory::Medium, SizeCategory::Medium),
];

fn all_books() -> RaceCorpus {
    let roots = [
        BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") },
        BookCorpusRoot { book_id: "beastiary", dir: Path::new("data/corpus/beastiary") },
        BookCorpusRoot {
            book_id: "advanced_race_guide",
            dir: Path::new("data/corpus/advanced_race_guide"),
        },
        // Bestiary 2, SD-31 Epic 1-F2 (2026-08-15).
        BookCorpusRoot { book_id: "bestiary_2", dir: Path::new("data/corpus/bestiary_2") },
        // Bestiary 5, the Skinwalker follow-on batch (2026-08-15).
        BookCorpusRoot { book_id: "bestiary_5", dir: Path::new("data/corpus/bestiary_5") },
        // Bestiary 6, SD-31 wave-24 (2026-08-20) -- missing from this local
        // hardcoded root list until SD-32 card-11 T2b lane (2026-08-23)
        // found it: the exact "stale hardcoded roots" defect class
        // `ingest_race_traits.rs`'s own module doc names (SD-29
        // `decisions.md §44.2`/`§44.5`), masked here because the omission
        // happened to leave this file's own count coincidentally matching
        // `SIZE_TRUTH`'s 37 until Dhampir's landing (below) pushed it past.
        BookCorpusRoot { book_id: "bestiary_6", dir: Path::new("data/corpus/bestiary_6") },
    ];
    let corpus = load_race_corpus(&roots);
    assert!(corpus.diagnostics().is_empty(), "clean load expected: {:?}", corpus.diagnostics());
    corpus
}

/// The table itself is a claim about the corpus, so it is checked against
/// the corpus before anything is asserted with it: every row's chassis
/// column must equal the `FACT:BaseSize` token actually on disk. If the
/// ingest changes, this fails first and names the row.
#[test]
fn the_expected_table_covers_all_eighteen_races_and_matches_the_on_disk_chassis_tokens() {
    let corpus = all_books();
    assert_eq!(
        SIZE_TRUTH.len(),
        39,
        "39 in-scope races: CRB 7 + Bestiary 1's 11 + Bestiary 2's 7 (the original 6, SD-31 \
         Epic 1-F2, plus Dhampir, SD-32 card-11 T2b lane, 2026-08-23) + \
         Bestiary 5's 1 (Skinwalker follow-on batch) + Advanced Race Guide's 12 \
         (SD-31-E6-F4-002, 2026-08-16 + SD31-E6-F4-004 + SD31-E6-F4-007, both 2026-08-17) -- \
         the full `arg_races.lst` 37-row playable-race roster, closed -- plus Bestiary 6's 1 \
         (Rougarou, SD-31 wave-24, 2026-08-20, missing from this file's own local `all_books()` \
         root list until this cycle -- see that function's doc comment)"
    );
    assert_eq!(corpus.race_keys().len(), 39, "and the corpus must carry all 39");

    for (_, key, chassis_size, _) in SIZE_TRUTH {
        let chassis = corpus.chassis(key).unwrap_or_else(|| panic!("{key} must have a chassis"));
        let on_disk = chassis
            .data
            .base_size
            .as_deref()
            .and_then(SizeCategory::from_base_size_code)
            .unwrap_or_else(|| panic!("{key} chassis must carry a readable FACT:BaseSize"));
        assert_eq!(on_disk, *chassis_size, "{key}: chassis column disagrees with the corpus record");
    }
}

/// **Defect 1, the whole-set pin.** Every one of the 18 races resolves its
/// real size out of the corpus.
#[test]
fn every_one_of_the_eighteen_races_resolves_its_true_size() {
    let corpus = all_books();
    for (_, key, _, true_size) in SIZE_TRUTH {
        let resolved = corpus.resolve(key, &[]).unwrap_or_else(|| panic!("{key} must resolve"));
        assert_eq!(resolved.size, Some(*true_size), "{key} resolved the wrong size");
    }
}

/// **Defect 1, named.** Aasimar and Tiefling are Medium even though their
/// chassis row says `S`, and the resolver must say *where* the Medium came
/// from — a right answer from an unexplained source is not much better than
/// a wrong one.
#[test]
fn aasimar_and_tiefling_are_medium_despite_a_small_chassis_row() {
    let corpus = all_books();
    for race in ["Aasimar", "Tiefling"] {
        let resolved = corpus.resolve(race, &[]).expect("resolves");
        assert_eq!(
            resolved.chassis_size,
            Some(SizeCategory::Small),
            "{race}'s chassis really does say FACT:BaseSize|S — that is the defect, not a typo here"
        );
        assert_eq!(resolved.size, Some(SizeCategory::Medium), "{race} is a Medium creature");
        assert_eq!(
            resolved.size_source,
            SizeSource::Trait(format!("{race} ~ Size")),
            "{race}'s real size must be attributed to the trait row that declares TEMPLATE:SIZE_M"
        );
        // The prose on the very same corpus row agrees, which is the
        // independent second reading of the same fact.
        let size_trait = resolved
            .traits
            .iter()
            .find(|t| t.key == format!("{race} ~ Size"))
            .expect("the ~ Size racial default must apply");
        assert!(
            size_trait.description.as_deref().unwrap_or_default().contains("Medium creatures"),
            "{race}: the row's own DESC must say Medium"
        );
    }
}

/// A race whose chassis and trait agree still reports the trait as the
/// source, so the override is unconditional rather than a "only when the
/// chassis looks wrong" special case — the same rule
/// `race_resolver.rs` already applies to walk speed.
#[test]
fn the_size_trait_is_the_source_even_when_it_agrees_with_the_chassis() {
    let corpus = all_books();
    let dwarf = corpus.resolve("Dwarf", &[]).expect("resolves");
    assert_eq!(dwarf.chassis_size, Some(SizeCategory::Medium));
    assert_eq!(dwarf.size, Some(SizeCategory::Medium));
    assert_eq!(dwarf.size_source, SizeSource::Trait("Dwarf ~ Size".to_string()));

    // Human is the one in-scope race whose `~ Size` row carries no
    // `TEMPLATE:SIZE_`, so its size legitimately comes off the chassis.
    // Pinned so that "the chassis is still a real source" stays true and
    // visible rather than becoming an accident.
    let human = corpus.resolve("Human", &[]).expect("resolves");
    assert_eq!(human.size, Some(SizeCategory::Medium));
    assert_eq!(human.size_source, SizeSource::Chassis);
}

/// Selecting every ARG alternate a race offers must not change its size:
/// no in-scope alternate declares a `TEMPLATE:SIZE_`, and none sets a
/// `<Race>_ReplaceSize` flag. Derived, not assumed — if ARG ever gains a
/// size-changing alternate this fails and someone models it deliberately.
#[test]
fn no_alternate_racial_trait_changes_any_races_size_today() {
    let corpus = all_books();
    for (_, key, _, true_size) in SIZE_TRUTH {
        let keys: Vec<String> =
            corpus.alternate_traits(key).iter().map(|t| t.data.key.clone()).collect();
        let refs: Vec<&str> = keys.iter().map(String::as_str).collect();
        let resolved = corpus.resolve(key, &refs).expect("resolves");
        assert_eq!(
            resolved.size,
            Some(*true_size),
            "{key}: selecting all {} alternates changed its size",
            refs.len()
        );
    }
}

/// **Defect 2, the whole-set pin.** The pure token lookup both encumbrance
/// call sites use covers all 18 races and agrees, race for race, with what
/// the real on-disk corpus resolves. The table cannot drift from the
/// corpus without this failing.
#[test]
fn the_race_token_size_lookup_covers_all_eighteen_and_agrees_with_the_corpus() {
    let corpus = all_books();
    for (token, key, _, true_size) in SIZE_TRUTH {
        assert_eq!(
            race_size_for_race_token(token),
            Some(*true_size),
            "{token} must resolve its real size"
        );
        let resolved = corpus.resolve(key, &[]).expect("resolves");
        assert_eq!(
            race_size_for_race_token(token),
            resolved.size,
            "{token}: the hand-modelled table and the corpus must not disagree"
        );
    }
}

/// **Defect 2, named.** These three were the silent Medium.
#[test]
fn goblin_kobold_and_svirfneblin_are_small_not_a_defaulted_medium() {
    for token in ["race:goblin", "race:kobold", "race:svirfneblin"] {
        assert_eq!(race_size_for_race_token(token), Some(SizeCategory::Small), "{token}");
    }
    // The complete Small set across all 18, stated as a whole-set
    // assertion because "which races are Small" is the exact fact carrying
    // capacity depends on.
    let small: Vec<&str> = SIZE_TRUTH
        .iter()
        .filter(|(token, _, _, _)| race_size_for_race_token(token) == Some(SizeCategory::Small))
        .map(|(_, key, _, _)| *key)
        .collect();
    assert_eq!(
        small,
        vec!["Gnome", "Halfling", "Goblin", "Kobold", "Svirfneblin", "Grippli", "Ratfolk", "Wayang"],
        "Grippli added by SD-31 Epic 1-F2 (2026-08-15); Ratfolk and Wayang added by \
         SD-31-E6-F4-002's Advanced Race Guide batch (2026-08-16)"
    );
}

/// Token matching accepts the shapes real inputs carry, and refuses
/// everything else with `None` rather than a guessed size.
#[test]
fn an_unresolvable_race_token_is_none_rather_than_a_defaulted_medium() {
    assert_eq!(race_size_for_race_token("race:HALF-ORC"), Some(SizeCategory::Medium));
    assert_eq!(race_size_for_race_token("  race:goblin "), Some(SizeCategory::Small));
    assert_eq!(race_size_for_race_token("Goblin"), Some(SizeCategory::Small));
    // Kasatha (ARG's reprint of an Inner Sea Races race) is not ingested; a
    // real absence must stay an absence. Dhampir gained a chassis, SD-32
    // card-11 T2b lane (2026-08-23), so it no longer stands in here.
    assert_eq!(race_size_for_race_token("race:kasatha"), None);
    assert_eq!(race_size_for_race_token("race:nonexistent"), None);
    assert_eq!(race_size_for_race_token(""), None);
}

/// **The silent default, made loud.** Both encumbrance call sites now go
/// through one helper that still has to pick *something* to compute a
/// capacity with, but can no longer do it quietly: an unresolvable race
/// yields a claim-blocking diagnostic naming the token, so the receipt
/// says its carrying capacity is not real data.
#[test]
fn an_unknown_race_produces_a_claim_blocking_diagnostic_not_a_quiet_medium() {
    // Kasatha stands in here for the same reason noted above: Dhampir
    // gained a chassis, SD-32 card-11 T2b lane (2026-08-23).
    let (size, diagnostic) = encumbrance_size_for_race("race:kasatha");
    assert_eq!(size, SizeCategory::Medium, "the baseline still has to be something");
    let diagnostic = diagnostic.expect("an unresolvable race must be reported");
    assert_eq!(diagnostic.id, UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID);
    assert!(diagnostic.claim_blocking, "a guessed size makes the capacity numbers unclaimable");
    assert!(
        diagnostic.message.contains("race:kasatha"),
        "the diagnostic must name the token it could not resolve: {}",
        diagnostic.message
    );

    // ...and every one of the 18 real races goes through silently, because
    // there is nothing to report.
    for (token, _, _, true_size) in SIZE_TRUTH {
        let (size, diagnostic) = encumbrance_size_for_race(token);
        assert_eq!(size, *true_size, "{token}");
        assert!(diagnostic.is_none(), "{token} must not emit a diagnostic");
    }
}

/// The player-visible consequence, end to end through the same
/// `carrying_capacity_thresholds` the receipt uses: at Strength 14 a
/// Goblin's heavy maximum is `SIZEMULT:S|0.75` of a Human's, not equal to
/// it. Before this fix both computed the Medium column.
#[test]
fn a_small_bestiary_race_now_gets_the_small_carrying_capacity_column() {
    let strength = 14;
    let human = carrying_capacity_thresholds(
        strength,
        encumbrance_size_for_race("race:human").0,
    );
    for token in ["race:goblin", "race:kobold", "race:svirfneblin"] {
        let small = carrying_capacity_thresholds(strength, encumbrance_size_for_race(token).0);
        assert_eq!(
            small.heavy_max_lbs,
            (human.heavy_max_lbs * 3.0 / 4.0).trunc(),
            "{token}: Strength 14 heavy max must be the Small column"
        );
        assert!(
            small.heavy_max_lbs < human.heavy_max_lbs,
            "{token}: a Small race must not carry as much as a Medium one"
        );
    }
    // And the two Medium races the old chassis read got wrong must NOT be
    // penalised: Aasimar and Tiefling carry the full Medium column.
    for token in ["race:aasimar", "race:tiefling"] {
        let medium = carrying_capacity_thresholds(strength, encumbrance_size_for_race(token).0);
        assert_eq!(
            medium.heavy_max_lbs, human.heavy_max_lbs,
            "{token} is Medium and must carry a Medium creature's load"
        );
    }
}
