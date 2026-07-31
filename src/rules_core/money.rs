//! Money/currency conversion (v0.6 alpha swarm).
//!
//! Before this file, no currency/wealth calculation existed anywhere in
//! this crate -- only per-item `cost_gp` pricing (confirmed by grep:
//! zero `gold`/`money`/`wealth`/`currency` hits in `src/rules_core/`
//! outside equipment pricing fields). This file grounds the standard PF1/
//! d20 denomination-conversion rule the alpha bar's "money conversion"
//! calculation item names.
//!
//! Ratios (per QA's `docs/release/v0.6/SWARM_REPORT.md` formula-spec
//! appendix): 1 platinum piece (pp) = 10 gold pieces (gp) = 100 silver
//! pieces (sp) = 1000 copper pieces (cp) -- standard open-content d20/PF1
//! currency. v0.6 alpha swarm item 3 re-verified this against the local
//! PCGen checkout (`/home/ubuntu/workspace/repos/pcgen`) directly: no
//! `.lst` data file defines a currency-denomination table (only
//! `system/gameModes/Pathfinder/miscinfo.lst`'s `CURRENCYUNITABBREV:gp`,
//! naming the display unit, not a ratio), no `.java` source file under the
//! checkout references "platinum" at all, and no fantasy output-sheet
//! template performs pp/gp/sp/cp conversion arithmetic either. The ratio is
//! not contradicted anywhere, but also not affirmatively pinned to a
//! specific PCGen source the way carry-capacity's `load.lst` table is --
//! it is universal D20/PF1 open-content arithmetic PCGen likely tracks
//! internally as a single "gold-piece-equivalent" total with the
//! denomination split applied only in display logic that never surfaced as
//! greppable data. Flagged as such; this is the same conclusion QA's own
//! appendix already reached, now backed by a direct second search rather
//! than left as an open item.
//!
//! Starting-wealth-by-class (the PCGen `GOLD:` token) was not found in any
//! `.lst` file QA/backend checked -- exhaustively searched and correctly
//! left unresolved as a content-provenance question, not an engineering
//! one (`docs/release/v0.6/risks-and-open-questions.md` item 7). Resolved
//! 2026-07-24: the operator provided the full table directly, cited to
//! d20pfsrd.com's "Character Creation" page (Pathfinder SRD/OGL content) --
//! see `starting_wealth_gp`'s own doc comment.


/// A copper-piece total broken into the four PF1 denominations, largest
/// first. `total_copper` on `Denominations` is *not* stored — callers hold
/// the canonical `total_copper: u64` themselves (e.g. as the persisted
/// balance) and call `copper_to_denominations` to derive a display
/// breakdown from it, so there is exactly one source of truth for the
/// amount, never two numbers that could drift apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Denominations {
    pub platinum: u64,
    pub gold: u64,
    pub silver: u64,
    pub copper: u64,
}

const COPPER_PER_SILVER: u64 = 10;
const COPPER_PER_GOLD: u64 = 100;
const COPPER_PER_PLATINUM: u64 = 1000;

/// Breaks a copper-piece total into platinum/gold/silver/copper, greedily
/// (as many platinum as fit, then gold from the remainder, then silver,
/// then whatever copper is left) -- the standard PF1 denomination-display
/// rule, per this module's own doc comment.
pub fn copper_to_denominations(total_copper: u64) -> Denominations {
    let platinum = total_copper / COPPER_PER_PLATINUM;
    let remainder = total_copper % COPPER_PER_PLATINUM;
    let gold = remainder / COPPER_PER_GOLD;
    let remainder = remainder % COPPER_PER_GOLD;
    let silver = remainder / COPPER_PER_SILVER;
    let copper = remainder % COPPER_PER_SILVER;

    Denominations { platinum, gold, silver, copper }
}

/// The inverse of `copper_to_denominations` -- sums a denomination
/// breakdown back into its total copper-piece value. Not merely a
/// round-trip check: real callers (e.g. a caller pricing a purchase in
/// mixed denominations) need this direction too.
pub fn denominations_to_copper(denominations: &Denominations) -> u64 {
    denominations.platinum * COPPER_PER_PLATINUM
        + denominations.gold * COPPER_PER_GOLD
        + denominations.silver * COPPER_PER_SILVER
        + denominations.copper
}

/// Converts a `cost_gp`-style gold-piece value (as carried by
/// `EquipmentTableEntry.cost_gp` / `FeatTableEntry`-adjacent corpus
/// pricing, which is frequently fractional, e.g. `0.05` for an arrow) into
/// its exact copper-piece total, rounding to the nearest copper piece
/// (gold pricing in the corpus is never finer than copper-piece
/// resolution in practice, so this does not lose real precision).
pub fn gp_to_copper(value_in_gp: f64) -> u64 {
    (value_in_gp * COPPER_PER_GOLD as f64).round() as u64
}

/// PF1 Core Rulebook class-id strings this function recognizes, matching
/// `pilot_compute.rs`'s own `<CLASS>_CLASS_ID` literal values exactly
/// (verified by inspection against that file, e.g. `FIGHTER_CLASS_ID`,
/// `WIZARD_CLASS_ID`, `ROGUE_CLASS_ID`, `BARBARIAN_CLASS_ID`, ...).
/// Re-declared here rather than imported: those constants are private to
/// `pilot_compute.rs`, and `starting_wealth_gp` is called from
/// `apps/desktop/src-tauri` (a separate crate) where even a `pub(crate)`
/// item in this crate would not be visible -- the same "duplicate the
/// literal, cite the source" idiom `skill_allocation.rs`'s own
/// `FIGHTER_CLASS_ID`/`ROGUE_CLASS_ID`/`WIZARD_CLASS_ID` already
/// established for the identical cross-module problem.
const FIGHTER_CLASS_ID: &str = "class:fighter";
const WIZARD_CLASS_ID: &str = "class:wizard";
const ROGUE_CLASS_ID: &str = "class:rogue";
const BARBARIAN_CLASS_ID: &str = "class:barbarian";
const BARD_CLASS_ID: &str = "class:bard";
const CLERIC_CLASS_ID: &str = "class:cleric";
const DRUID_CLASS_ID: &str = "class:druid";
const MONK_CLASS_ID: &str = "class:monk";
const PALADIN_CLASS_ID: &str = "class:paladin";
const RANGER_CLASS_ID: &str = "class:ranger";
const SORCERER_CLASS_ID: &str = "class:sorcerer";

/// The 10 non-Core-Rulebook (APG/ACG) class id strings this function also
/// recognizes (v0.6 alpha swarm item 7, second phase, 2026-07-24 -- operator
/// go-ahead, re-confirming d20pfsrd.com as an approved source). Re-checked
/// directly against the real d20pfsrd "Character Creation" table before
/// implementing (rather than trusting this doc's own prose, which
/// miscounted "12" where the table itself always named exactly these 10):
/// every id/value pair here matches that table's own printed row verbatim,
/// zero mismatches, zero missing rows. No `class_tables()` row and no
/// `compute_class_chassis` dispatch arm exists for any of these -- they are
/// recognized *only* by this function, the same "recognized but narrow"
/// shape as any `class:<name>` string this crate has never built chassis
/// support for; a character using one of these ids still reaches `Blocked`
/// exactly like any other unsupported class (verified by test), so
/// recognizing the id here carries no risk of ever granting wealth to a
/// build that hasn't proven `Computed`.
const ALCHEMIST_CLASS_ID: &str = "class:alchemist";
const CAVALIER_CLASS_ID: &str = "class:cavalier";
const GUNSLINGER_CLASS_ID: &str = "class:gunslinger";
const INQUISITOR_CLASS_ID: &str = "class:inquisitor";
const MAGUS_CLASS_ID: &str = "class:magus";
const NINJA_CLASS_ID: &str = "class:ninja";
const ORACLE_CLASS_ID: &str = "class:oracle";
const SAMURAI_CLASS_ID: &str = "class:samurai";
const SUMMONER_CLASS_ID: &str = "class:summoner";
const WITCH_CLASS_ID: &str = "class:witch";
// Pathfinder Unchained (SD-27, 2026-07-31). Distinct id strings from the
// four classes they replace, so no comparison here can confuse the two.
const UNCHAINED_BARBARIAN_CLASS_ID: &str = "class:unchained_barbarian";
const UNCHAINED_MONK_CLASS_ID: &str = "class:unchained_monk";
const UNCHAINED_ROGUE_CLASS_ID: &str = "class:unchained_rogue";
const UNCHAINED_SUMMONER_CLASS_ID: &str = "class:unchained_summoner";

/// Average starting wealth in gold pieces for a PF1 Core Rulebook class id
/// (v0.6 alpha swarm item 7, resolved 2026-07-24). The operator (Todd
/// Hintzmann) provided the full table directly, sourced from
/// d20pfsrd.com's "Character Creation" page
/// (<https://www.d20pfsrd.com/basics-ability-scores/character-creation/>,
/// Pathfinder SRD/OGL content) -- see
/// `docs/release/v0.6/risks-and-open-questions.md` item 7 for the full
/// table as recorded there. Also recognizes the 10 non-Core-Rulebook
/// (APG/ACG) classes named on the same real d20pfsrd table (Alchemist,
/// Cavalier, Gunslinger, Inquisitor, Magus, Ninja, Oracle, Samurai,
/// Summoner, Witch -- re-counted directly against the live source before
/// implementing, not assumed from this doc's own prose, which had
/// miscounted "12"). None of the 10 has a `class_tables()` row or a
/// `compute_class_chassis` dispatch arm -- recognized only by this
/// function, the same "recognized but narrow" shape; a build using one of
/// these ids still reaches `Blocked` like any other unsupported class
/// (verified by test), so no wealth is ever granted to an unproven build.
/// Returns `None` for any other (genuinely unrecognized) class id, rather
/// than fabricating a value.
///
/// PF1's own rule is to roll the die (e.g. Fighter: 5d6 x 10 gp) rather
/// than take a fixed value. This crate is deterministic throughout --
/// every computed value carries a machine-checkable explanation and is
/// exactly reproducible from its inputs, with no random-number generator
/// anywhere in `rules_core` -- so rolling would be a genuine architecture
/// departure, not a drop-in fix. The operator's own table already prints
/// an "Average" column (die count x 3.5 x 10, always a whole number for
/// every class); this function returns that column's value directly
/// rather than re-deriving it, so the values here are a citation, not an
/// independent computation. A future "reroll for real starting wealth"
/// affordance, if ever wanted, is a separate frontend/UX feature layered
/// on top of this deterministic default, not a change to this function.
pub fn starting_wealth_gp(class_id: &str) -> Option<u32> {
    match class_id {
        MONK_CLASS_ID => Some(35),
        DRUID_CLASS_ID | SORCERER_CLASS_ID | WIZARD_CLASS_ID | SUMMONER_CLASS_ID => Some(70),
        BARBARIAN_CLASS_ID | BARD_CLASS_ID | ALCHEMIST_CLASS_ID | ORACLE_CLASS_ID
        | SAMURAI_CLASS_ID | WITCH_CLASS_ID => Some(105),
        CLERIC_CLASS_ID | ROGUE_CLASS_ID | INQUISITOR_CLASS_ID | MAGUS_CLASS_ID
        | NINJA_CLASS_ID => Some(140),
        FIGHTER_CLASS_ID | PALADIN_CLASS_ID | RANGER_CLASS_ID | CAVALIER_CLASS_ID
        | GUNSLINGER_CLASS_ID => Some(175),
        // SD-27 (2026-07-31), Pathfinder Unchained's four classes. Each
        // returns its BASE class's value from the same table above, and the
        // reason is a corpus fact rather than an assumption: an Unchained
        // class is a `CATEGORY:CLASS` selection ability layered over the
        // base `CLASS:` record, and none of the four carries any token
        // touching starting wealth (their `raw_tokens` in
        // `data/corpus/pathfinder_unchained/class/*.json` are proficiency,
        // feature-grant and skill tokens only). The book changes what the
        // class DOES, not what it starts with.
        //
        // Written as four explicit arms rather than folded into the rows
        // above, so that a future errata to one side of a pair cannot
        // silently move the other.
        UNCHAINED_MONK_CLASS_ID => Some(35),
        UNCHAINED_SUMMONER_CLASS_ID => Some(70),
        UNCHAINED_BARBARIAN_CLASS_ID => Some(105),
        UNCHAINED_ROGUE_CLASS_ID => Some(140),
        _ => None,
    }
}

#[cfg(test)]
mod starting_wealth_tests {
    use super::*;

    #[test]
    fn matches_the_operator_cited_average_for_the_three_classes_that_reach_computed() {
        assert_eq!(starting_wealth_gp("class:fighter"), Some(175));
        assert_eq!(starting_wealth_gp("class:wizard"), Some(70));
        assert_eq!(starting_wealth_gp("class:rogue"), Some(140));
    }

    #[test]
    fn matches_the_operator_cited_average_for_every_other_crb_class() {
        assert_eq!(starting_wealth_gp("class:barbarian"), Some(105));
        assert_eq!(starting_wealth_gp("class:bard"), Some(105));
        assert_eq!(starting_wealth_gp("class:cleric"), Some(140));
        assert_eq!(starting_wealth_gp("class:druid"), Some(70));
        assert_eq!(starting_wealth_gp("class:monk"), Some(35));
        assert_eq!(starting_wealth_gp("class:paladin"), Some(175));
        assert_eq!(starting_wealth_gp("class:ranger"), Some(175));
        assert_eq!(starting_wealth_gp("class:sorcerer"), Some(70));
    }

    #[test]
    fn every_value_is_the_dice_count_times_35_confirming_the_average_column_arithmetic() {
        // Cross-check against the operator-cited dice formulas (die count x
        // 3.5 x 10 = die count x 35), not just the printed average column in
        // isolation -- an independent arithmetic check on the same citation.
        let dice_counts = [
            ("class:monk", 1),
            ("class:druid", 2),
            ("class:sorcerer", 2),
            ("class:wizard", 2),
            ("class:summoner", 2),
            ("class:barbarian", 3),
            ("class:bard", 3),
            ("class:alchemist", 3),
            ("class:oracle", 3),
            ("class:samurai", 3),
            ("class:witch", 3),
            ("class:cleric", 4),
            ("class:rogue", 4),
            ("class:inquisitor", 4),
            ("class:magus", 4),
            ("class:ninja", 4),
            ("class:fighter", 5),
            ("class:paladin", 5),
            ("class:ranger", 5),
            ("class:cavalier", 5),
            ("class:gunslinger", 5),
        ];
        for (class_id, dice_count) in dice_counts {
            assert_eq!(
                starting_wealth_gp(class_id),
                Some(dice_count * 35),
                "{class_id}: {dice_count}d6 x 10 should average to {dice_count} x 35 gp"
            );
        }
    }

    /// v0.6 alpha swarm item 7 (second phase, 2026-07-24): the 10 non-CRB
    /// (APG/ACG) classes from the same real d20pfsrd table, re-checked
    /// directly against the live source before implementing (not assumed
    /// from this doc's own prose, which had miscounted "12" -- the table
    /// itself always named exactly these 10, zero missing rows).
    #[test]
    fn matches_the_operator_cited_average_for_every_non_crb_class() {
        assert_eq!(starting_wealth_gp("class:alchemist"), Some(105));
        assert_eq!(starting_wealth_gp("class:cavalier"), Some(175));
        assert_eq!(starting_wealth_gp("class:gunslinger"), Some(175));
        assert_eq!(starting_wealth_gp("class:inquisitor"), Some(140));
        assert_eq!(starting_wealth_gp("class:magus"), Some(140));
        assert_eq!(starting_wealth_gp("class:ninja"), Some(140));
        assert_eq!(starting_wealth_gp("class:oracle"), Some(105));
        assert_eq!(starting_wealth_gp("class:samurai"), Some(105));
        assert_eq!(starting_wealth_gp("class:summoner"), Some(70));
        assert_eq!(starting_wealth_gp("class:witch"), Some(105));
    }

    /// SD-27: each Unchained class starts with exactly what the class it
    /// replaces starts with, and the pair is asserted side by side so a
    /// future edit to one that forgets the other fails here.
    #[test]
    fn each_unchained_class_starts_with_its_base_classs_wealth() {
        for (unchained, base) in [
            ("class:unchained_barbarian", "class:barbarian"),
            ("class:unchained_monk", "class:monk"),
            ("class:unchained_rogue", "class:rogue"),
            ("class:unchained_summoner", "class:summoner"),
        ] {
            assert_eq!(
                starting_wealth_gp(unchained),
                starting_wealth_gp(base),
                "{unchained} must match {base}"
            );
        }
        // Pinned as literals too, so the test cannot pass by both sides
        // becoming `None` together.
        assert_eq!(starting_wealth_gp("class:unchained_barbarian"), Some(105));
        assert_eq!(starting_wealth_gp("class:unchained_monk"), Some(35));
        assert_eq!(starting_wealth_gp("class:unchained_rogue"), Some(140));
        assert_eq!(starting_wealth_gp("class:unchained_summoner"), Some(70));
    }

    #[test]
    fn returns_none_for_a_class_id_it_does_not_recognize() {
        assert_eq!(starting_wealth_gp("not-a-real-class-id"), None);
        assert_eq!(starting_wealth_gp("class:samurai "), None, "trailing whitespace must not fuzzy-match");
    }

    #[test]
    fn converts_cleanly_to_a_starting_copper_balance_via_the_existing_conversion() {
        // Not a new conversion path -- proves starting_wealth_gp composes
        // with the already-grounded gp_to_copper exactly the way a caller
        // initializing a fresh character's money.json would use it.
        assert_eq!(
            gp_to_copper(f64::from(starting_wealth_gp("class:fighter").unwrap())),
            17_500
        );
        assert_eq!(gp_to_copper(f64::from(starting_wealth_gp("class:wizard").unwrap())), 7_000);
        assert_eq!(gp_to_copper(f64::from(starting_wealth_gp("class:rogue").unwrap())), 14_000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copper_to_denominations_breaks_down_a_mixed_total_greedily() {
        // 1 pp (1000cp) + 2 gp (200cp) + 3 sp (30cp) + 4 cp = 1234 cp.
        assert_eq!(
            copper_to_denominations(1234),
            Denominations { platinum: 1, gold: 2, silver: 3, copper: 4 }
        );
    }

    #[test]
    fn copper_to_denominations_handles_zero() {
        assert_eq!(copper_to_denominations(0), Denominations::default());
    }

    #[test]
    fn copper_to_denominations_handles_an_amount_under_one_silver() {
        assert_eq!(
            copper_to_denominations(7),
            Denominations { platinum: 0, gold: 0, silver: 0, copper: 7 }
        );
    }

    #[test]
    fn denominations_to_copper_is_the_real_inverse_of_copper_to_denominations() {
        for total in [0, 7, 42, 1234, 999_999] {
            let denominations = copper_to_denominations(total);
            assert_eq!(
                denominations_to_copper(&denominations),
                total,
                "round trip must be exact for {total} cp: {denominations:?}"
            );
        }
    }

    #[test]
    fn denominations_to_copper_sums_an_arbitrary_mixed_breakdown_not_just_a_round_trip() {
        // A caller pricing a purchase directly in mixed denominations, not
        // one this module's own copper_to_denominations ever produces
        // (e.g. 5 gold pieces expressed alongside 15 silver, which a
        // greedy breakdown would never emit together -- 15sp always
        // collapses to 1gp+5sp on the way out).
        let denominations = Denominations { platinum: 0, gold: 5, silver: 15, copper: 0 };
        assert_eq!(denominations_to_copper(&denominations), 650);
    }

    #[test]
    fn gp_to_copper_converts_whole_and_fractional_gold_values() {
        assert_eq!(gp_to_copper(1.0), 100);
        assert_eq!(gp_to_copper(0.05), 5, "an arrow's real corpus cost_gp value");
        assert_eq!(gp_to_copper(2.5), 250);
    }
}
