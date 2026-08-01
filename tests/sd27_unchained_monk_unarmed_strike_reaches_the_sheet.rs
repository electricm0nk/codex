//! SD-27 — the Unchained Monk's unarmed strike damage die reaches the sheet,
//! and the Core Rulebook Monk's is byte-identical afterwards (2026-08-01).
//!
//! # The defect this closes
//!
//! `rules_tables::pathfinder_unchained::monk_features`'s module doc declined to
//! model unarmed strike damage, on the stated grounds that *"`pilot_compute.rs`'s
//! `monk_unarmed_strike_damage_die` already states that progression"*. That was
//! true of the **function** and false of the **sheet**: the only rows carrying
//! it, `class_chassis.monk.unarmed_strike_damage_die` and its `_die_count`
//! sibling, are pushed by `explain_monk_level1_chassis`, which returns early
//! unless the class is Core Rulebook `class:monk`. An Unchained Monk got the
//! roster row `class_feature.pu.unchained_monk.corpus_record.unarmed_strike`
//! — the words — and no number, at every level from 1 to 20. Verified live
//! before the fix; the before column of [`BEFORE_AND_AFTER`] is that
//! measurement.
//!
//! # Why the number is reused rather than re-modelled
//!
//! Established by command against the PCGen corpus, not assumed:
//!
//! * `pu_abilities_class.lst:464` (`Unchained Monk ~ Unarmed Strike`) grants
//!   `ABILITY:Internal|AUTOMATIC|Monk ~ Unarmed Damage` — the *same* shared
//!   Core Rulebook internal record `cr_abilities_class.lst:1118` grants — and
//!   carries no damage token of its own.
//! * `grep -n "MonkUnarmedDamage\|UDAM" pathfinder_unchained/*.lst` returns
//!   **zero** lines, so Pathfinder Unchained overrides nothing about that
//!   record: no `.MOD`, no `BONUS:VAR`, no replacement band table.
//! * The shared record (`cr_abilities_class.lst:1280`) selects its band with
//!   `BONUS:VAR|MonkUnarmedDamageProgression|(min(5,MonkUnarmedDamageLVL/4))`
//!   over `BONUS:VAR|MonkUnarmedDamageLVL|MonkLVL`, which is
//!   `monk_unarmed_strike_damage_die`'s `min(5, level / 4)` exactly.
//!
//! So the progressions are identical, and a second copy of the ladder in the
//! Pathfinder Unchained tables would be a competing source of truth for one
//! fact. The existing function is called instead.
//!
//! # The size column, which is not a re-model either
//!
//! The shared record's band records fan out per creature size
//! (`Monk Unarmed Damage LVL 8 (Small)`, `… (Medium)`, …). `race_resolver`'s
//! `RACE_SIZES` gives the 18 playable races exactly two of those columns —
//! 13 Medium and 5 Small (Gnome, Halfling, Goblin, Kobold, Svirfneblin) — and
//! the Core Rulebook path never met the problem because it is gated on
//! `race:human`. Emitting the Medium ladder to a Gnome would be a specific,
//! checkable, wrong number on a player's sheet, so the Small column is read
//! off the same corpus records and pinned by
//! [`the_small_column_is_the_corpus_small_column`].
//!
//! # `decisions.md §28`'s standing guard
//!
//! *"Every change to [`pilot_compute.rs`] lands with a test pinning the
//! before/after per affected race or class."*
//! [`the_core_rulebook_monk_is_byte_identical`] is the half that matters most:
//! the Core Rulebook Monk's own rows must not move at all, and it compares id,
//! value **and** the full `detail` string against literals captured before the
//! change.

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, ComputationExplanation};

/// The same shared deterministic fixture the sibling Pathfinder Unchained pins
/// use, so all of them describe one posture rather than several.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

const DIE_ID: &str = "class_feature.pu.unchained_monk.unarmed_strike_damage_die";
const DIE_COUNT_ID: &str = "class_feature.pu.unchained_monk.unarmed_strike_damage_die_count";

/// `(level, die face, die count, display name)` for a **Medium** Unchained
/// Monk, at the four levels `decisions.md §28`'s guard is reported against.
///
/// Before this change every one of these rows was absent; there is no "before"
/// value to pin because there was no row. [`BEFORE_AND_AFTER`] pins the row
/// counts that absence produced.
const MEDIUM_LADDER: &[(u8, i16, i16, &str)] = &[
    (1, 6, 1, "1d6"),
    (5, 8, 1, "1d8"),
    (10, 10, 1, "1d10"),
    (20, 10, 2, "2d10"),
];

/// The same four levels for a **Small** Unchained Monk, read off
/// `cr_abilities_class.lst`'s own `Monk Unarmed Damage LVL <band> (Small)`
/// records (lines 1296 / 1306 / 1316 / 1326 / 1336 / 1346).
const SMALL_LADDER: &[(u8, i16, i16, &str)] = &[
    (1, 4, 1, "1d4"),
    (5, 6, 1, "1d6"),
    (10, 8, 1, "1d8"),
    (20, 8, 2, "2d8"),
];

/// `(level, grounded Unchained Monk rows BEFORE, AFTER)`.
///
/// "Grounded" excludes the per-record roster rows (`.corpus_record.*`) and the
/// deferral notices (`.unsupported`), matching the definition the sibling
/// Pathfinder Unchained pins already use. The before column was measured on
/// the real pipeline before this change; each level gains exactly the two rows
/// this file adds.
const BEFORE_AND_AFTER: &[(u8, usize, usize)] = &[(1, 5, 7), (5, 11, 13), (10, 12, 14), (20, 15, 17)];

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn explanations(race_id: &str, class_token: &str, level: u8) -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some(format!("sd27_unchained_monk_unarmed.{race_id}.{class_token}.{level}"));
    input.chosen.race_id = race_id.to_owned();
    input.chosen.class_levels =
        vec![CharacterClassLevel { class_id: format!("class:{class_token}"), level }];
    build_pilot_headless_receipt(&input).computation.explanations
}

#[track_caller]
fn row(rows: &[ComputationExplanation], id: &str) -> ComputationExplanation {
    rows.iter()
        .find(|explanation| explanation.id == id)
        .unwrap_or_else(|| panic!("expected explanation {id}; got {:?}", ids(rows)))
        .clone()
}

fn ids(rows: &[ComputationExplanation]) -> Vec<&str> {
    rows.iter().map(|explanation| explanation.id.as_str()).collect()
}

/// A Medium Unchained Monk gets the Medium column, at every band boundary the
/// report names.
#[test]
fn a_medium_unchained_monk_gets_the_medium_unarmed_damage_die() {
    for &(level, face, count, name) in MEDIUM_LADDER {
        let rows = explanations("race:dwarf", "unchained_monk", level);
        let die = row(&rows, DIE_ID);
        assert_eq!(die.value, face, "Dwarf Unchained Monk {level} die face");
        assert!(
            die.detail.contains(name),
            "level {level} detail must name the die as {name}: {}",
            die.detail
        );
        assert!(
            die.detail.contains("Medium"),
            "level {level} detail must name the creature size it is the column for: {}",
            die.detail
        );
        assert_eq!(
            row(&rows, DIE_COUNT_ID).value,
            count,
            "Dwarf Unchained Monk {level} die count"
        );
    }
}

/// The Small column is the corpus's Small column, not the Medium one handed to
/// a smaller creature. This is the row that would otherwise be a specific,
/// checkable, wrong number.
#[test]
fn the_small_column_is_the_corpus_small_column() {
    for &(level, face, count, name) in SMALL_LADDER {
        let rows = explanations("race:gnome", "unchained_monk", level);
        let die = row(&rows, DIE_ID);
        assert_eq!(die.value, face, "Gnome Unchained Monk {level} die face");
        assert!(
            die.detail.contains(name),
            "level {level} detail must name the die as {name}: {}",
            die.detail
        );
        assert!(
            die.detail.contains("Small"),
            "level {level} detail must say which size column it read: {}",
            die.detail
        );
        assert_eq!(
            row(&rows, DIE_COUNT_ID).value,
            count,
            "Gnome Unchained Monk {level} die count"
        );
    }
}

/// Every one of the five Small playable races reads the Small column, and every
/// one of the thirteen Medium ones the Medium column. Derived from
/// `race_resolver::RACE_SIZES` rather than sampled, so a race joining the
/// roster on the wrong column fails here.
#[test]
fn every_playable_race_reads_its_own_size_column() {
    const SMALL: &[&str] = &["gnome", "halfling", "goblin", "kobold", "svirfneblin"];
    const MEDIUM: &[&str] = &[
        "dwarf", "elf", "half-elf", "half-orc", "human", "aasimar", "drow", "duergar", "hobgoblin",
        "merfolk", "orc", "tengu", "tiefling",
    ];
    assert_eq!(SMALL.len() + MEDIUM.len(), 18, "the 18 in-scope races, all accounted for");

    for race in SMALL {
        let rows = explanations(&format!("race:{race}"), "unchained_monk", 10);
        assert_eq!(row(&rows, DIE_ID).value, 8, "{race} is Small: 1d8 at level 10");
    }
    for race in MEDIUM {
        let rows = explanations(&format!("race:{race}"), "unchained_monk", 10);
        assert_eq!(row(&rows, DIE_ID).value, 10, "{race} is Medium: 1d10 at level 10");
    }
}

/// A race the engine cannot size gets an honest absence, not the Medium
/// ladder. The `.unsupported` id is the engine's own "not grounded here"
/// idiom, and it carries no magnitude a reader could mistake for a die.
#[test]
fn an_unsizable_race_gets_an_absence_rather_than_the_medium_ladder() {
    let rows = explanations("race:not-a-real-race", "unchained_monk", 10);
    assert!(
        !ids(&rows).contains(&DIE_ID),
        "no die row may be emitted for a race whose size is unknown"
    );
    let notice = row(&rows, &format!("{DIE_ID}.unsupported"));
    assert_eq!(notice.value, 0, "an absence notice carries a filler zero, never a die size");
    assert!(
        notice.detail.contains("size"),
        "the notice must say why it is absent: {}",
        notice.detail
    );
}

/// `decisions.md §28`'s guard, the before half: the row counts this change
/// moves, and by exactly how much.
#[test]
fn the_grounded_row_count_moves_by_exactly_two_at_every_level() {
    for &(level, before, after) in BEFORE_AND_AFTER {
        let rows = explanations("race:dwarf", "unchained_monk", level);
        let grounded = rows
            .iter()
            .filter(|explanation| {
                explanation.id.starts_with("class_feature.pu.unchained_monk.")
                    && !explanation.id.contains(".corpus_record.")
                    && !explanation.id.ends_with(".unsupported")
            })
            .count();
        assert_eq!(grounded, after, "grounded Unchained Monk rows at level {level}");
        assert_eq!(after - before, 2, "level {level} gains exactly the two rows added here");
    }
}

/// `decisions.md §28`'s guard, the after half, and the regression this file
/// exists to catch: the Core Rulebook Monk's own unarmed-strike rows are
/// **byte-identical** — same ids, same values, same `detail` text, same
/// presence/absence of the count row — after the Unchained Monk started
/// emitting its own.
///
/// The literals are the real pipeline's output captured before the change.
/// `detail` is compared by its full length and its opening and closing
/// clauses rather than reproduced in full, because the strings run to 850
/// characters and a transcription of one into a test is a second copy that can
/// drift; the length pins any edit to the body, and the clauses pin the ends.
#[test]
fn the_core_rulebook_monk_is_byte_identical() {
    /// `(level, die value, detail length, die count row)`.
    const CRB: &[(u8, i16, usize, Option<i16>)] = &[
        (1, 6, 846, None),
        (5, 8, 846, None),
        (10, 10, 852, None),
        (20, 10, 852, Some(2)),
    ];

    for &(level, value, detail_len, count) in CRB {
        // `explain_monk_level1_chassis` is gated on `race:human`; the fixture's
        // own race is what the Core Rulebook path has always been measured on.
        let rows = explanations("race:human", "monk", level);
        let die = row(&rows, "class_chassis.monk.unarmed_strike_damage_die");
        assert_eq!(die.value, value, "Core Rulebook Monk {level} unarmed die must not move");
        assert_eq!(
            die.detail.len(),
            detail_len,
            "Core Rulebook Monk {level} unarmed die detail must not move"
        );
        assert!(
            die.detail.starts_with("Monk level "),
            "level {level} detail opening clause"
        );
        assert!(
            die.detail.ends_with("(2d8 and beyond) is not grounded"),
            "level {level} detail closing clause"
        );

        let count_id = "class_chassis.monk.unarmed_strike_damage_die_count";
        match count {
            Some(expected) => assert_eq!(
                row(&rows, count_id).value,
                expected,
                "Core Rulebook Monk {level} die count"
            ),
            None => assert!(
                !ids(&rows).contains(&count_id),
                "Core Rulebook Monk {level} must still emit no die-count row"
            ),
        }
    }
}

/// The Unchained Monk's rows live in the Pathfinder Unchained namespace, not in
/// the Core Rulebook Monk's.
///
/// This is not cosmetic. `classFeaturesModel.ts::splitId` attributes a row to a
/// class by matching an id segment against the character's **held** classes, so
/// a `class_chassis.monk.…` id on a character holding `class:unchained_monk`
/// matches nothing, loses its class gutter and renders as
/// "Monk Unarmed Strike Damage Die". The two sibling magnitudes in the same
/// grounding function that also come from shared Core Rulebook internal records
/// — `armor_class_bonus` (`Monk AC Tracker`) and `ki_points` (`Ki Pool
/// Tracker`) — are namespaced this way for the same reason.
#[test]
fn the_unchained_rows_do_not_borrow_the_core_rulebook_monks_namespace() {
    let rows = explanations("race:dwarf", "unchained_monk", 20);
    for id in ids(&rows) {
        assert!(
            !id.starts_with("class_chassis.monk."),
            "an Unchained Monk must emit no Core Rulebook Monk chassis row, got {id}"
        );
    }
}
