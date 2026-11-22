//! SD-22 Epic 6 acceptance test — DM-toolkit deterministic tests (criterion
//! 20): `epic-breakdown.md` criterion 20, "DM-toolkit tests cover both
//! modules' deterministic cases against canonical Paizo examples," covering
//! both `src/rules_core/encounters.rs` (criterion 18) and
//! `src/rules_core/party_cr.rs` (criterion 19) against the five canonical
//! cases named in `corpus-source-inventory.md` §4.1.
//!
//! ## Independent re-verification of two documented discrepancies
//!
//! Criterion 18's cycle (`artifacts/dm_toolkit/encounters_cycle_receipt.md`)
//! and criterion 19's cycle (`artifacts/dm_toolkit/party_cr_cycle_receipt.md`)
//! each independently found that `corpus-source-inventory.md` §4.1's stated
//! expected values for case 2 ("Hard") and case 3 ("~3.5") don't match the
//! real PF1 Core Rulebook math, and flagged the reconciliation for this
//! cycle rather than force-fitting the code to the doc.
//!
//! This cycle re-verified both discrepancies independently — not by trusting
//! the prior cycles' claims, but by re-fetching the same public PRD mirror
//! (`https://legacy.aonprd.com/corerulebook/gamemastering.html`) fresh and
//! reading the tables directly:
//!
//! - **Table: Encounter Design**: `Easy = APL-1`, `Average = APL`,
//!   `Challenging = APL+1`, `Hard = APL+2`, `Epic = APL+3`.
//! - **Table: CR Equivalencies**: 4 creatures of the same CR combine to
//!   `CR + 4`.
//! - **Table: Experience Point Awards**, CR 1-10: 400, 600, 800, 1,200,
//!   1,600, 2,400, 3,200, 4,800, 6,400, 9,600 (independently re-confirmed
//!   verbatim, matching both prior cycles' citations exactly).
//! - **Step 1 — Determine APL**: "You should round this value to the
//!   nearest whole number... If your group contains six or more players,
//!   add one to their average level. If your group contains three or fewer
//!   players, subtract one from their average level." No step in this rule
//!   can produce a fractional (non-integer) APL for any party size.
//!
//! **Case 2** (4 level-3 PCs vs. 4 CR-3 monsters): APL = 3. Four CR-3
//! monsters combine to CR 3+4 = 7 per Table: CR Equivalencies (independently
//! cross-checked via the XP method too: 4 × 800 XP = 3,200 XP = the CR-7
//! threshold on Table: Experience Point Awards — both methods agree). EL −
//! APL = 7 − 3 = +4, which is beyond even `Epic` (APL+3) on Table: Encounter
//! Design — i.e. `Difficulty::Deadly` under this bundle's 4-tier collapse,
//! not the `Hard` §4.1 originally stated. Confirmed independently: §4.1 was
//! wrong; `Deadly` is the grounded answer.
//!
//! **Case 3** (party CR of 4 level-3 PCs): 4 PCs is within the rulebook's
//! unadjusted "four or five PCs" band — no party-size adjustment applies.
//! Average level = (3+3+3+3)/4 = 3.0 exactly, already a whole number.
//! Confirmed independently: the rulebook's APL procedure has no mechanism
//! that can ever produce a `.5` result, so `~3.5` was wrong; `3.0` is the
//! grounded answer.
//!
//! Per this bundle's own established precedent (Gunslinger/Magus APG-roster
//! correction, ACG Alchemist-roster correction) — and because both
//! `encounters.rs` and `party_cr.rs` are already correctly implemented
//! against the grounded rulebook math (verified by this cycle's own reading
//! of their source, not just their doc comments) — this cycle corrects
//! `corpus-source-inventory.md` §4.1's fixture table to match the verified
//! real rules, rather than bending already-correct production code to match
//! an unverified fixture-table entry. No production code in `encounters.rs`
//! or `party_cr.rs` changes as part of this cycle.

use codex::rules_core::encounters::{CharacterSnapshot, Difficulty, Encounter, MonsterRef};
use codex::rules_core::party_cr::party_challenge_rating;

fn party_of_4_level_3_pcs() -> Vec<CharacterSnapshot> {
    (0..4).map(|_| CharacterSnapshot::new(3)).collect()
}

/// `corpus-source-inventory.md` §4.1 case 1 (unchanged by this cycle's
/// reconciliation): 4 level-3 PCs vs. 1 CR-2 monster → Easy. APL 3; a
/// single CR-2 monster is EL 2 (Table: CR Equivalencies, 1 creature = CR);
/// EL − APL = −1 = APL−1 ⇒ Easy per Table: Encounter Design.
#[test]
fn encounters_4_level_3_pcs_vs_1_cr_2_monster_is_easy() {
    let party = party_of_4_level_3_pcs();
    let monsters = vec![MonsterRef::new(2.0)];
    let result = Encounter::new(&party, &monsters);
    assert_eq!(result.difficulty, Difficulty::Easy);
}

/// `corpus-source-inventory.md` §4.1 case 2, **corrected this cycle** from
/// the originally-stated `Hard` to the independently re-verified `Deadly`
/// (see this file's module doc comment for the full re-derivation): APL 3,
/// group EL 7 (4 × CR-3 monsters = CR+4 = 7 per Table: CR Equivalencies,
/// cross-checked via the XP-sum method), EL − APL = +4, beyond even Epic
/// (APL+3) on Table: Encounter Design.
#[test]
fn encounters_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly() {
    let party = party_of_4_level_3_pcs();
    let monsters: Vec<MonsterRef> = (0..4).map(|_| MonsterRef::new(3.0)).collect();
    let result = Encounter::new(&party, &monsters);
    assert_eq!(result.encounter_level, 7);
    assert_eq!(result.difficulty, Difficulty::Deadly);
}

/// `corpus-source-inventory.md` §4.1 case 3, **corrected this cycle** from
/// the originally-stated `~3.5` to the independently re-verified `3.0` (see
/// this file's module doc comment for the full re-derivation): 4 PCs falls
/// within the rulebook's unadjusted "four or five PCs" band, so average
/// level (3.0) passes through unchanged — the grounded APL rule has no step
/// that can produce a fractional result for any party.
#[test]
fn party_cr_of_4_level_3_pcs_equals_3() {
    let party = party_of_4_level_3_pcs();
    let result = party_challenge_rating(&party);
    assert_eq!(result, 3.0);
}

/// `corpus-source-inventory.md` §4.1 case 4 (unchanged by this cycle's
/// reconciliation): 4 level-3 PCs vs. no monsters → Easy (no threat) — a
/// direct business rule, not an EL-vs-APL comparison.
#[test]
fn encounters_empty_monsters_returns_easy() {
    let party = party_of_4_level_3_pcs();
    let monsters: Vec<MonsterRef> = vec![];
    let result = Encounter::new(&party, &monsters);
    assert_eq!(result.difficulty, Difficulty::Easy);
}

/// `corpus-source-inventory.md` §4.1 case 5 (unchanged by this cycle's
/// reconciliation): 1 level-1 PC vs. 1 CR-1 monster → "returns valid
/// difficulty" (any of the four tiers is acceptable per that row's own
/// description). Asserts a `Difficulty` was returned via an exhaustive
/// match (compile-time guarantee no fifth variant sneaks in), and pins the
/// grounded computed value (APL 1, EL 1, EL−APL = 0 ⇒ Medium) as a
/// regression guard.
#[test]
fn encounters_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty() {
    let party = vec![CharacterSnapshot::new(1)];
    let monsters = vec![MonsterRef::new(1.0)];
    let result = Encounter::new(&party, &monsters);
    match result.difficulty {
        Difficulty::Easy | Difficulty::Medium | Difficulty::Hard | Difficulty::Deadly => {}
    }
    assert_eq!(result.difficulty, Difficulty::Medium);
}
