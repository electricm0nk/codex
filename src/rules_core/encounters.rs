//! DM-toolkit encounter-difficulty computation (SD-22 Epic 6, criterion 18).
//!
//! `epic-breakdown.md` criterion 18: `Encounter::new(party: &[CharacterSnapshot],
//! monsters: &[MonsterRef]) -> EncounterResult` computes the encounter's
//! difficulty rating (Easy / Medium / Hard / Deadly per PF1's "Encounter
//! Building" rules).
//!
//! ## Source grounding (per `decisions.md`'s "no fabricated data" discipline)
//!
//! This module is grounded in the Pathfinder RPG Core Rulebook's
//! "Gamemastering" chapter, verified 2026-07-20 against the public PRD
//! mirror `legacy.aonprd.com/corerulebook/gamemastering.html`:
//!
//! - **Table: Encounter Design** (difficulty relative to Average Party
//!   Level, compared against the encounter's computed Encounter Level):
//!   `Easy = APL-1`, `Average = APL`, `Challenging = APL+1`, `Hard = APL+2`,
//!   `Epic = APL+3` (or higher).
//! - **Table: CR Equivalencies** (combining N identical-CR creatures into
//!   one group Encounter Level): 1 creature = CR, 2 = CR+2, 3 = CR+3,
//!   4 = CR+4, 5-6 = CR+5, 7-8 = CR+6, 9-12 = CR+7, 13-16 = CR+8.
//! - **Table: Experience Point Awards** (per-creature XP by CR), verified
//!   CR 1 through CR 10: 400, 600, 800, 1,200, 1,600, 2,400, 3,200, 4,800,
//!   6,400, 9,600.
//!
//! [`group_encounter_level`] computes a (possibly mixed-CR) monster group's
//! combined Encounter Level by summing each monster's per-creature XP value
//! and reconverting the total back to the CR whose Table: Experience Point
//! Awards entry it matches or exceeds — the "alternative method" the
//! rulebook's own discussion of combining creatures describes as more
//! precise for groups that aren't all the same CR. For an identical-CR
//! group this reduces to exactly the Table: CR Equivalencies answer (e.g.
//! four CR-3 monsters: 4 x 800 XP = 3,200 XP = the CR-7 XP threshold,
//! matching `CR + 4 = 3 + 4 = 7` from the table directly).
//!
//! [`xp_for_cr`]'s table is directly verified for CR 1-10 only. Beyond CR
//! 10 (e.g. a Bestiary 1 Tarrasque at CR 25 — the extreme-CR case
//! `corpus-source-inventory.md` §3.2 requires this module handle without
//! erroring, once Epic 5 lands), XP is extrapolated by continuing the
//! verified table's own doubling-every-2-CR pattern (CR1→CR3→CR5→…→CR9 each
//! double the prior odd entry; CR2→CR4→…→CR10 likewise for even), which the
//! verified CR 1-10 run satisfies exactly at every step. This extrapolation
//! is a direct mathematical continuation of sourced data, not an invented
//! figure — but it is not independently re-verified against the rulebook
//! above CR 10, and is flagged here for that reason.
//!
//! ## Difficulty tier collapse (5 official tiers -> this bundle's 4)
//!
//! `epic-breakdown.md` and `corpus-source-inventory.md` §4 name a four-tier
//! `Difficulty` (Easy / Medium / Hard / Deadly), not the rulebook's five
//! (Easy / Average / Challenging / Hard / Epic). This module maps 1:1 where
//! the names align (`Easy` -> `Easy`, `Average` -> `Medium`, `Epic` ->
//! `Deadly`) and merges the two "harder than average, not yet extreme"
//! tiers (`Challenging` = APL+1, `Hard` = APL+2) into this module's single
//! `Hard` tier, since the bundle's own acceptance criteria only name four
//! buckets.
//!
//! ## A documented discrepancy against `corpus-source-inventory.md` §4.1
//!
//! §4.1's canonical case 2 ("4 level-3 PCs vs 4 CR-3 monsters") states an
//! expected result of `Hard`. Computed against the verified tables above:
//! APL = 3; group EL = 4 x CR-3 monsters = 3,200 total XP = CR 7 (the CR-7
//! XP threshold); `EL - APL = 7 - 3 = +4`, which is beyond even `Epic`
//! (APL+3) on Table: Encounter Design, so this module's grounded formula
//! computes `Difficulty::Deadly`, not `Hard`. Per this bundle's own
//! precedent for planning-doc content that doesn't hold up against a
//! verified source (the Gunslinger/Magus APG-roster correction, the ACG
//! Alchemist-roster correction), this cycle follows the verified rulebook
//! math rather than bending the formula to match an unverified fixture
//! table entry; see `tests` below (`encounter_of_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly_per_grounded_pf1_math`)
//! and `docs/release/SD-22/progress.md`'s cycle log for this cycle, which
//! flags the discrepancy for criterion 20's dedicated deterministic-test
//! cycle to reconcile (recompute against a second independent source, or
//! correct `corpus-source-inventory.md` §4.1's case-2 entry).

/// Minimal DM-toolkit party-member snapshot consumed by [`Encounter::new`].
///
/// Scoped to the single field the grounded Average-Party-Level formula
/// needs (character level). A fuller party/character snapshot type may
/// consolidate with this one once `party_cr.rs` (criterion 19) lands —
/// mirroring `rules_tables/crb/class_tables.rs`'s precedent of shipping
/// only what the grounded formula requires rather than a speculative full
/// shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterSnapshot {
    pub level: u8,
}

impl CharacterSnapshot {
    pub fn new(level: u8) -> Self {
        Self { level }
    }
}

/// Minimal DM-toolkit monster reference consumed by [`Encounter::new`].
///
/// Scoped to the single field the encounter-difficulty formula needs
/// (Challenge Rating). Bestiary 1's own richer `MonsterRef` shape (Epic 5,
/// `corpus-source-inventory.md` §3.1: AC, HP, saves, attacks, etc.) is a
/// distinct, not-yet-landed type — Epic 5 is still blocked on its own
/// parser gap (`b1_races.lst`'s unprefixed bare-row records; see
/// `docs/release/SD-22/progress.md`'s `## Open blockers`). This module does
/// not depend on that unshipped type; a later cycle (criterion 21, the
/// happy-path integration test) is where the two get reconciled once Epic 5
/// lands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonsterRef {
    pub challenge_rating: f32,
}

impl MonsterRef {
    pub fn new(challenge_rating: f32) -> Self {
        Self { challenge_rating }
    }
}

/// This bundle's four-tier encounter-difficulty rating.
///
/// See the module doc comment's "Difficulty tier collapse" section for the
/// mapping from the rulebook's five official tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Deadly,
}

/// The result of [`Encounter::new`]'s difficulty computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncounterResult {
    pub difficulty: Difficulty,
    pub average_party_level: i32,
    pub encounter_level: i32,
}

/// Table: Experience Point Awards, verified CR 1-10 (per-creature XP).
const VERIFIED_XP_TABLE: [u64; 10] = [400, 600, 800, 1_200, 1_600, 2_400, 3_200, 4_800, 6_400, 9_600];

/// Per-creature XP value for a given (whole-number) Challenge Rating.
///
/// CR below 1 is floored to CR 1 — this module doesn't yet ground the
/// rulebook's fractional-CR sub-table (1/8, 1/6, 1/4, 1/3, 1/2); no fixture
/// in this cycle or `corpus-source-inventory.md` §4.1 exercises a
/// fractional-CR monster, so extending to it is deferred to whichever
/// future cycle first needs it. CR above 10 is extrapolated per the module
/// doc comment's doubling-every-2-CR continuation.
fn xp_for_cr(cr: i32) -> u64 {
    let cr = cr.max(1);
    if cr <= 10 {
        return VERIFIED_XP_TABLE[(cr - 1) as usize];
    }
    let (base_cr, base_xp) = if cr % 2 == 0 {
        (10, VERIFIED_XP_TABLE[9])
    } else {
        (9, VERIFIED_XP_TABLE[8])
    };
    let doublings = ((cr - base_cr) / 2) as u32;
    base_xp.saturating_mul(2u64.saturating_pow(doublings))
}

/// Inverse of [`xp_for_cr`]: the largest CR whose Table: Experience Point
/// Awards value is `<=` the given total XP. This is the rulebook's
/// "alternative method" for finding a mixed-CR (or same-CR) group's
/// combined Encounter Level from its total XP value.
fn xp_to_cr(xp: u64) -> i32 {
    let mut cr = 1;
    while xp_for_cr(cr + 1) <= xp {
        cr += 1;
    }
    cr
}

/// A monster group's combined Encounter Level, per the module doc comment's
/// "alternative method" (sum per-creature XP, reconvert to CR).
fn group_encounter_level(monsters: &[MonsterRef]) -> i32 {
    let total_xp: u64 = monsters
        .iter()
        .map(|m| xp_for_cr(m.challenge_rating.round() as i32))
        .sum();
    xp_to_cr(total_xp)
}

/// Average Party Level, rounded to the nearest whole level (round-half-up),
/// per the rulebook's stated APL rounding convention. An empty party has an
/// APL of 0 (not exercised by any of this cycle's fixtures; a defensive
/// default, not a load-bearing rule).
fn average_party_level(party: &[CharacterSnapshot]) -> i32 {
    if party.is_empty() {
        return 0;
    }
    let total: u32 = party.iter().map(|c| c.level as u32).sum();
    let count = party.len() as f64;
    ((total as f64 / count).round()) as i32
}

/// Maps `encounter_level - average_party_level` onto this bundle's
/// four-tier [`Difficulty`], per the module doc comment's "Difficulty tier
/// collapse" section.
fn difficulty_for_el_vs_apl(encounter_level: i32, average_party_level: i32) -> Difficulty {
    match encounter_level - average_party_level {
        d if d <= -1 => Difficulty::Easy,
        0 => Difficulty::Medium,
        1 | 2 => Difficulty::Hard,
        _ => Difficulty::Deadly,
    }
}

/// The DM-toolkit's encounter-difficulty computation.
///
/// `Encounter::new` per `epic-breakdown.md` criterion 18.
pub struct Encounter;

impl Encounter {
    /// Computes the difficulty of `party` facing `monsters`, per the
    /// grounded PF1 Encounter Design / CR Equivalencies / Experience Point
    /// Awards tables cited in this module's doc comment.
    ///
    /// An empty `monsters` slice is an automatic `Difficulty::Easy` (no
    /// threat) — this is a direct business rule, not derived from the EL
    /// vs. APL comparison (which is undefined for a CR-less "encounter").
    ///
    /// Named `new` (rather than a `compute`/`difficulty_of`-shaped name) to
    /// match `epic-breakdown.md` criterion 18's literal signature
    /// (`Encounter::new(party, monsters) -> EncounterResult`); clippy's
    /// `new_ret_no_self` convention lint is intentionally overridden here
    /// since the acceptance criterion's own API shape returns the result
    /// type, not `Self`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(party: &[CharacterSnapshot], monsters: &[MonsterRef]) -> EncounterResult {
        let apl = average_party_level(party);
        if monsters.is_empty() {
            return EncounterResult {
                difficulty: Difficulty::Easy,
                average_party_level: apl,
                encounter_level: 0,
            };
        }
        let el = group_encounter_level(monsters);
        let difficulty = difficulty_for_el_vs_apl(el, apl);
        EncounterResult {
            difficulty,
            average_party_level: apl,
            encounter_level: el,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn party_of_4_level_3_pcs() -> Vec<CharacterSnapshot> {
        (0..4).map(|_| CharacterSnapshot::new(3)).collect()
    }

    /// `corpus-source-inventory.md` §4.1 case 1: 4 level-3 PCs vs 1 CR-2
    /// monster -> Easy. Group EL for a single CR-2 monster is 2 (Table: CR
    /// Equivalencies, 1 creature = CR); APL 3; EL - APL = -1 = APL-1 =>
    /// Easy per Table: Encounter Design.
    #[test]
    fn encounter_of_4_level_3_pcs_vs_1_cr_2_monster_is_easy() {
        let party = party_of_4_level_3_pcs();
        let monsters = vec![MonsterRef::new(2.0)];
        let result = Encounter::new(&party, &monsters);
        assert_eq!(result.difficulty, Difficulty::Easy);
        assert_eq!(result.average_party_level, 3);
        assert_eq!(result.encounter_level, 2);
    }

    /// `corpus-source-inventory.md` §4.1 case 4: 4 level-3 PCs vs no
    /// monsters -> Easy (no threat).
    #[test]
    fn encounter_empty_monsters_returns_easy() {
        let party = party_of_4_level_3_pcs();
        let monsters: Vec<MonsterRef> = vec![];
        let result = Encounter::new(&party, &monsters);
        assert_eq!(result.difficulty, Difficulty::Easy);
    }

    /// `corpus-source-inventory.md` §4.1 case 5: 1 level-1 PC vs 1 CR-1
    /// monster -> "returns valid difficulty" (any of the four tiers is
    /// acceptable per that row's own description; the exhaustive match
    /// below is the assertion that a `Difficulty` was returned at all, not
    /// a specific tier). Computed: APL 1, group EL 1 (1 creature = CR),
    /// EL - APL = 0 => Medium under this module's grounded formula.
    #[test]
    fn encounter_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty() {
        let party = vec![CharacterSnapshot::new(1)];
        let monsters = vec![MonsterRef::new(1.0)];
        let result = Encounter::new(&party, &monsters);
        match result.difficulty {
            Difficulty::Easy | Difficulty::Medium | Difficulty::Hard | Difficulty::Deadly => {}
        }
        assert_eq!(result.difficulty, Difficulty::Medium);
    }

    /// `corpus-source-inventory.md` §4.1 case 2: 4 level-3 PCs vs 4 CR-3
    /// monsters. §4.1 states an expected result of `Hard`; this module's
    /// grounded formula computes `Deadly` instead — see the module doc
    /// comment's "A documented discrepancy" section for the full
    /// verified-table derivation (APL 3, group EL 7, EL-APL = +4, beyond
    /// even Epic/APL+3 on Table: Encounter Design). This test asserts the
    /// grounded result, not the unverified fixture-table entry, per this
    /// bundle's own precedent of following a verified source over planning
    /// prose that doesn't hold up (Gunslinger/Magus, ACG Alchemist).
    #[test]
    fn encounter_of_4_level_3_pcs_vs_4_cr_3_monsters_is_deadly_per_grounded_pf1_math() {
        let party = party_of_4_level_3_pcs();
        let monsters: Vec<MonsterRef> = (0..4).map(|_| MonsterRef::new(3.0)).collect();
        let result = Encounter::new(&party, &monsters);
        assert_eq!(result.encounter_level, 7);
        assert_eq!(result.difficulty, Difficulty::Deadly);
    }

    #[test]
    fn xp_table_matches_verified_cr_1_through_10() {
        assert_eq!(xp_for_cr(1), 400);
        assert_eq!(xp_for_cr(2), 600);
        assert_eq!(xp_for_cr(3), 800);
        assert_eq!(xp_for_cr(4), 1_200);
        assert_eq!(xp_for_cr(5), 1_600);
        assert_eq!(xp_for_cr(6), 2_400);
        assert_eq!(xp_for_cr(7), 3_200);
        assert_eq!(xp_for_cr(8), 4_800);
        assert_eq!(xp_for_cr(9), 6_400);
        assert_eq!(xp_for_cr(10), 9_600);
    }

    /// Extreme-CR handling (`corpus-source-inventory.md` §3.2's Tarrasque
    /// invariant, forward-looking to Epic 5): a CR-25 monster must not
    /// panic, and its computed difficulty against a low-level party is
    /// Deadly.
    #[test]
    fn encounter_handles_extreme_cr_without_panicking() {
        let party = vec![CharacterSnapshot::new(1)];
        let monsters = vec![MonsterRef::new(25.0)];
        let result = Encounter::new(&party, &monsters);
        assert_eq!(result.difficulty, Difficulty::Deadly);
    }
}
