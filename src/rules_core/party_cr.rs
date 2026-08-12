//! DM-toolkit party-challenge-rating computation (SD-22 Epic 6, criterion 19).
//!
//! `epic-breakdown.md` criterion 19: `party_challenge_rating(party:
//! &[CharacterSnapshot]) -> f32` computes the party's CR per PF1's
//! "Determining Party Strength" rules.
//!
//! ## Source grounding (per `decisions.md`'s "no fabricated data" discipline)
//!
//! Verified 2026-07-20 against the public PRD mirror
//! `legacy.aonprd.com/corerulebook/gamemastering.html`, Gamemastering chapter,
//! "Designing Encounters" -> "Step 1 -- Determine APL" (this is the concrete
//! heading the rulebook uses; `epic-breakdown.md`'s "Determining Party
//! Strength" phrase names the same underlying rule, confirmed by searching
//! the source page directly for a literal "Party Strength" heading and
//! finding none -- "Step 1 -- Determine APL" is the actual, only rule this
//! criterion can be grounding on):
//!
//! - "Determine the average level of your player characters -- this is
//!   their Average Party Level (APL for short). You should round this value
//!   to the nearest whole number."
//! - "Note that these encounter creation guidelines assume a group of four
//!   or five PCs. If your group contains six or more players, add one to
//!   their average level. If your group contains three or fewer players,
//!   subtract one from their average level."
//! - Worked example verbatim: "if your group consists of six players, two
//!   of which are 4th level and four of which are 5th level, their APL is
//!   6th (28 total levels, divided by six players, rounding up, and adding
//!   one to the final result)."
//!
//! This module treats the party's CR-equivalent as exactly this
//! rulebook-defined APL: sum levels, divide by party size, round to the
//! nearest whole number, then apply the +1 (six-or-more) / -1
//! (three-or-fewer) party-size adjustment. This is the same underlying
//! computation `src/rules_core/encounters.rs`'s `average_party_level`
//! performs for `Encounter::new` (criterion 18), extended here with the
//! rulebook's party-size adjustment step, which `encounters.rs` does not
//! need for its own per-encounter APL comparison but this module's
//! standalone party-CR criterion does name explicitly. Returns `f32` per
//! criterion 19's literal specified signature, even though the underlying
//! rule always produces a whole number.
//!
//! ## A documented discrepancy against `corpus-source-inventory.md` §4.1
//!
//! §4.1's canonical case 3 ("party CR of 4 level-3 PCs") states an expected
//! result of `~3.5`. Computed against the verified rule above: 4 characters
//! is within the rulebook's unadjusted "four or five PCs" band, so no
//! party-size adjustment applies; average level = (3+3+3+3)/4 = 3.0 exactly,
//! rounded to the nearest whole number is `3`. The grounded rule has no step
//! that would produce a `.5` fractional result for any input -- the
//! rulebook's own APL procedure is defined to always round to a whole
//! number. Per this bundle's own established precedent for planning-doc
//! content that doesn't hold up against a verified source (the
//! Gunslinger/Magus APG-roster correction, the ACG Alchemist-roster
//! correction, and criterion 18's own "Hard" vs. grounded-"Deadly"
//! discrepancy against the same §4.1 fixture table), this cycle follows the
//! verified rulebook math (`3.0`) rather than bending the formula to match
//! an unverified fixture-table entry. See `tests` below
//! (`party_cr_of_4_level_3_pcs_is_3_per_grounded_pf1_apl_rule`) and
//! `docs/release/SD-22/progress.md`'s cycle log for this cycle, which flags
//! the discrepancy for criterion 20's dedicated deterministic-test cycle (or
//! an operator/doc-correction pass) to reconcile.

use crate::rules_core::encounters::CharacterSnapshot;

/// Computes `party`'s Challenge-Rating equivalent per PF1's Average Party
/// Level rule (Core Rulebook, Gamemastering -> Designing Encounters ->
/// Step 1 -- Determine APL; see the module doc comment for the verbatim
/// rule text and party-size adjustment).
///
/// An empty party returns `0.0` -- a defensive default (the rulebook's own
/// procedure assumes at least one PC; this isn't exercised by any of this
/// cycle's canonical fixtures, mirroring `encounters.rs`'s identical
/// empty-party convention).
pub fn party_challenge_rating(party: &[CharacterSnapshot]) -> f32 {
    if party.is_empty() {
        return 0.0;
    }
    let total: u32 = party.iter().map(|c| c.level as u32).sum();
    let count = party.len();
    let average = total as f64 / count as f64;
    let rounded_apl = average.round() as i32;
    let adjusted = if count >= 6 {
        rounded_apl + 1
    } else if count <= 3 {
        rounded_apl - 1
    } else {
        rounded_apl
    };
    adjusted as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `corpus-source-inventory.md` §4.1 case 3: party CR of 4 level-3 PCs.
    /// §4.1 states an expected result of `~3.5`; this module's grounded
    /// rule computes `3.0` instead -- see the module doc comment's "A
    /// documented discrepancy" section. 4 PCs falls in the rulebook's
    /// unadjusted four-or-five-PC band, so average level (3.0) is returned
    /// unchanged after rounding.
    #[test]
    fn party_cr_of_4_level_3_pcs_is_3_per_grounded_pf1_apl_rule() {
        let party: Vec<CharacterSnapshot> = (0..4).map(|_| CharacterSnapshot::new(3)).collect();
        let result = party_challenge_rating(&party);
        assert_eq!(result, 3.0);
    }

    /// Verified worked example, quoted verbatim in the module doc comment:
    /// six players, two 4th-level and four 5th-level -> APL 6th (28 total
    /// levels / 6 players = 4.667, rounds to 5, +1 for the six-or-more
    /// party-size adjustment = 6).
    #[test]
    fn party_cr_applies_plus_one_adjustment_for_six_or_more_players() {
        let mut party: Vec<CharacterSnapshot> = (0..2).map(|_| CharacterSnapshot::new(4)).collect();
        party.extend((0..4).map(|_| CharacterSnapshot::new(5)));
        assert_eq!(party.len(), 6);
        let result = party_challenge_rating(&party);
        assert_eq!(result, 6.0);
    }

    /// Rulebook rule (module doc comment): "If your group contains three or
    /// fewer players, subtract one from their average level." Three
    /// level-5 PCs: average 5.0, minus 1 for the three-or-fewer adjustment
    /// = 4.0.
    #[test]
    fn party_cr_applies_minus_one_adjustment_for_three_or_fewer_players() {
        let party: Vec<CharacterSnapshot> = (0..3).map(|_| CharacterSnapshot::new(5)).collect();
        let result = party_challenge_rating(&party);
        assert_eq!(result, 4.0);
    }

    /// Five PCs is still within the rulebook's unadjusted "four or five"
    /// band -- no party-size adjustment, confirming the band's upper
    /// boundary is inclusive of 5, distinct from the six-or-more case.
    #[test]
    fn party_cr_applies_no_adjustment_for_five_players() {
        let party: Vec<CharacterSnapshot> = (0..5).map(|_| CharacterSnapshot::new(3)).collect();
        let result = party_challenge_rating(&party);
        assert_eq!(result, 3.0);
    }

    /// A single-PC party is within the three-or-fewer band: average level 1,
    /// minus 1 for the adjustment = 0.0. Exercises the adjustment's lower
    /// extreme without going negative.
    #[test]
    fn party_cr_of_1_level_1_pc_applies_minus_one_adjustment() {
        let party = vec![CharacterSnapshot::new(1)];
        let result = party_challenge_rating(&party);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn party_cr_of_empty_party_is_zero() {
        let party: Vec<CharacterSnapshot> = vec![];
        let result = party_challenge_rating(&party);
        assert_eq!(result, 0.0);
    }
}
