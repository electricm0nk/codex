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
//! Deliberately scoped to conversion only. Starting-wealth-by-class (the
//! PCGen `GOLD:` token) was not found in the class LST file QA checked --
//! unresolved, not guessed here. Per QA's own note, if v0.6's scope is
//! "track and spend money the player already has" rather than "auto-roll
//! starting gold," that gap does not block this file's scope.

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
