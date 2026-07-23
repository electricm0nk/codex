//! v0.6 alpha swarm: money/currency conversion catalogue adoption.
//!
//! `src/rules_core/money.rs` (commit `67490ac`) grounds the alpha bar's
//! "money conversion" calculation, previously a complete production gap
//! (confirmed by QA's original wave-1 gap-list survey: zero `gold`/`money`/
//! `wealth`/`currency` hits anywhere in `src/rules_core/` outside per-item
//! `cost_gp` pricing fields). The module carries its own inline
//! `#[cfg(test)] mod tests` (backend's stopgap since `tests/**` is QA's
//! owned surface for this swarm, mirroring the same pattern
//! `pilot_compute.rs`'s multiclass BAB/save-stacking generalization used).
//! This file is QA's independent catalogue adoption: it exercises the same
//! public API with its own assertions rather than copying the inline
//! tests, so the codebase does not depend on a single author's coverage of
//! this surface.
//!
//! Ratios (1 pp = 10 gp = 100 sp = 1000 cp) are standard open-content d20/
//! PF1 currency, not independently confirmed against a PCGen data file (no
//! conversion-table `.lst` exists in the PCGen checkout QA searched during
//! the original formula-spec pass — consistent with this being simple
//! linear arithmetic rather than tabulated data). Flagged here as it is in
//! `money.rs`'s own doc comment and QA's `SWARM_REPORT.md` appendix.

use codex::rules_core::money::{copper_to_denominations, denominations_to_copper, gp_to_copper, Denominations};

#[test]
fn copper_to_denominations_greedily_prefers_the_largest_denomination_first() {
    // 2 pp (2000cp) + 0 gp + 7 sp (70cp) + 3cp = 2073cp. Zero gold pieces in
    // the middle exercises that a zero-valued denomination doesn't break the
    // greedy breakdown chain (distinct from money.rs's own inline coverage,
    // which never zeroes an intermediate denomination).
    assert_eq!(
        copper_to_denominations(2073),
        Denominations { platinum: 2, gold: 0, silver: 7, copper: 3 }
    );
}

#[test]
fn copper_to_denominations_handles_exact_denomination_boundaries() {
    assert_eq!(
        copper_to_denominations(1000),
        Denominations { platinum: 1, gold: 0, silver: 0, copper: 0 },
        "exactly 1000cp must be 1pp with nothing left over"
    );
    assert_eq!(
        copper_to_denominations(100),
        Denominations { platinum: 0, gold: 1, silver: 0, copper: 0 },
        "exactly 100cp must be 1gp with nothing left over"
    );
    assert_eq!(
        copper_to_denominations(10),
        Denominations { platinum: 0, gold: 0, silver: 1, copper: 0 },
        "exactly 10cp must be 1sp with nothing left over"
    );
}

#[test]
fn round_trip_is_exact_for_a_large_realistic_character_wealth_total() {
    // A mid-level character's plausible total wealth (thousands of gold),
    // well beyond money.rs's own inline round-trip test's largest sample
    // (999_999cp = 9,999.99gp) -- proving the round trip holds at a scale a
    // real high-level PF1 character could plausibly reach.
    let total_copper = 5_000_000u64; // 50,000 gp
    let denominations = copper_to_denominations(total_copper);
    assert_eq!(denominations_to_copper(&denominations), total_copper);
    assert_eq!(denominations, Denominations { platinum: 5000, gold: 0, silver: 0, copper: 0 });
}

#[test]
fn gp_to_copper_rounds_a_sub_copper_fractional_gold_value_to_the_nearest_copper() {
    // The corpus carries gp prices finer than money.rs's own inline test
    // exercises (e.g. a component costing a fraction of a copper once
    // converted) -- confirms the rounding direction at a genuine boundary
    // case (0.005gp = 0.5cp, rounds to the nearest copper, which for exact
    // .5 ties rounds away from zero per Rust's f64::round).
    assert_eq!(gp_to_copper(0.005), 1);
    assert_eq!(gp_to_copper(0.0), 0);
}

#[test]
fn gp_to_copper_matches_a_real_crb_corpus_item_price() {
    // A torch's real CRB cost_gp value (0.01gp, per the corpus's own
    // cr_equip_general.lst WT/COST tokens) must convert to exactly 1 copper
    // piece -- the smallest real corpus price this crate carries, and the
    // sharpest possible rounding-precision test for gp_to_copper.
    assert_eq!(gp_to_copper(0.01), 1, "a torch's real 0.01gp corpus price must be exactly 1 copper");
}

#[test]
fn denominations_default_is_a_true_zero_balance() {
    let zero = Denominations::default();
    assert_eq!(zero, Denominations { platinum: 0, gold: 0, silver: 0, copper: 0 });
    assert_eq!(denominations_to_copper(&zero), 0);
}
