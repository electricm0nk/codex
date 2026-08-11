//! The corpus equipment gap lane: every `equipment`/`equipment_modifier`
//! record belonging to an already-compiled book that no hand-authored
//! per-book table holds.
//!
//! These tests pin the properties that make the lane real rather than
//! decorative: the rows reach `equipment_catalog_rows()` (the one function
//! both the desktop catalog and the headless cost resolver read), they add a
//! book the catalog could not serve at all before, they never fabricate a
//! price, and — the non-regression that matters most — every key the
//! hand-authored tables already answered still resolves to the identical
//! hand-authored row at the identical price.

use codex::rules_core::equipment_resolver::{
    equipment_catalog_row_by_key, equipment_catalog_rows, hand_authored_equipment_rows,
    EQUIPMENT_BOOK_UW,
};
use codex::rules_core::rules_tables::equipment_gap_tables::equipment_gap_rows;

/// The generator's own per-book output, re-derived here from the table rather
/// than transcribed from its stdout. Each figure is that book's
/// `not-ingested` `equipment` + `equipment_modifier` unit count in
/// `docs/work-inventory.json` at the time this lane landed.
const EXPECTED_PER_BOOK: &[(&str, usize)] = &[
    ("CRB", 335),
    ("APG", 37),
    ("ACG", 50),
    ("ARG", 15),
    ("UC", 20),
    ("UI", 7),
    ("UE", 65),
    ("UPSI", 113),
    ("UW", 127),
];

#[test]
fn the_gap_lane_carries_one_row_per_previously_not_ingested_unit() {
    let total: usize = equipment_gap_rows().count();
    let expected: usize = EXPECTED_PER_BOOK.iter().map(|(_, n)| *n).sum();
    assert_eq!(
        total, expected,
        "gap row count moved; regenerate with `cargo run --bin gen_equipment_gap_tables` and \
         re-derive the per-book figures from docs/work-inventory.json before changing them"
    );
    assert_eq!(total, 769, "769 is the inventory's own not-ingested equipment population");
}

#[test]
fn every_books_gap_row_count_is_pinned() {
    for (code, expected) in EXPECTED_PER_BOOK {
        let actual = equipment_gap_rows().filter(|row| row.book == *code).count();
        assert_eq!(actual, *expected, "gap row count for book {code}");
    }
}

#[test]
fn every_gap_row_reaches_the_shared_catalog() {
    let catalog = equipment_catalog_rows();
    assert_eq!(
        catalog.len(),
        hand_authored_equipment_rows().len() + equipment_gap_rows().count(),
        "the catalog must be exactly the hand tables plus the gap rows — no row dropped, \
         none double-counted"
    );
    // Not a length check alone: a sample of real identities must be findable.
    for row in equipment_gap_rows() {
        assert!(
            catalog.iter().any(|c| c.key == row.key && c.book == row.book),
            "gap row {:?} ({}) never reached equipment_catalog_rows()",
            row.key,
            row.book
        );
    }
}

#[test]
fn ultimate_wilderness_reaches_the_catalog_only_through_the_gap_lane() {
    assert!(
        !hand_authored_equipment_rows().iter().any(|r| r.book == EQUIPMENT_BOOK_UW),
        "UW has no hand-authored equipment table; if one lands, this lane's UW rows are \
         duplicates and must be regenerated"
    );
    let uw = equipment_catalog_rows().iter().filter(|r| r.book == EQUIPMENT_BOOK_UW).count();
    assert_eq!(uw, 127, "every UW catalog row comes from the gap lane");
}

#[test]
fn widening_leaves_every_hand_authored_key_resolving_to_its_original_row() {
    // The gap rows are chained LAST precisely so first-match key lookup is
    // unchanged. Proven, not asserted: for every hand-authored key, the row
    // the shared resolver returns is a hand-authored one with the same price.
    for hand in hand_authored_equipment_rows() {
        let resolved = equipment_catalog_row_by_key(hand.key)
            .unwrap_or_else(|| panic!("hand-authored key {:?} no longer resolves", hand.key));
        // Compared by VALUE, not by pointer: `equipment_catalog_rows()` owns a
        // copied `Vec`, so every row it hands back is a distinct allocation
        // from the hand table's own — a first draft used `std::ptr::eq` and
        // failed on `Arrow (Base)` for that reason alone, not because the row
        // was wrong.
        let original = hand_authored_equipment_rows()
            .iter()
            .find(|h| h.key == hand.key)
            .expect("just iterated it");
        assert_eq!(
            resolved, original,
            "key {:?} now resolves to a different row than its first hand-authored one",
            hand.key
        );
    }
}

#[test]
fn no_gap_row_fabricates_an_identity_or_a_price() {
    for row in equipment_gap_rows() {
        assert!(!row.key.is_empty(), "empty key");
        assert!(!row.name.is_empty(), "empty name on {:?}", row.key);
        assert!(!row.book.is_empty(), "empty book on {:?}", row.key);
        assert!(
            matches!(row.category, "General" | "ArmsArmor" | "MagicItems" | "Equipmods"),
            "unknown category {:?} on {:?}",
            row.category,
            row.key
        );
        if let Some(cost) = row.cost_gp {
            assert!(cost.is_finite() && cost >= 0.0, "nonsense cost on {:?}", row.key);
        }
        if let Some(weight) = row.weight_lbs {
            assert!(weight.is_finite() && weight >= 0.0, "nonsense weight on {:?}", row.key);
        }
        if let Some(desc) = row.description {
            assert!(!desc.trim().is_empty(), "empty description on {:?}", row.key);
        }
    }
}

#[test]
fn a_real_priced_row_and_a_real_honestly_unpriced_row_are_both_present() {
    // A lane that emitted `None` for every price would pass every check above
    // while having ingested nothing usable.
    let priced = equipment_gap_rows().filter(|r| r.cost_gp.is_some()).count();
    assert!(priced > 0, "not one gap row carries a real corpus COST: token");
    let unpriced = equipment_gap_rows().filter(|r| r.cost_gp.is_none()).count();
    assert!(
        unpriced > 0,
        "every row priced is itself suspicious — PCGen formula costs must stay None, not be \
         evaluated into a fabricated flat number"
    );
}
