//! v0.6 alpha swarm: equipment cost resolution catalogue adoption.
//!
//! `equipment_cost_gp_headless_resolve` (`src/rules_core/equipment_resolver.rs`,
//! commit `29e67515`) grounds the item-id-to-`cost_gp` resolution the new
//! `purchase_equipment` command needs to atomically couple an equipment
//! purchase to the money balance (risks-and-open-questions.md item 9).
//! Unlike `equipment_id_resolve`, it needs no corpus access at all — real
//! `cost_gp` values live directly on the compiled-in `equipment_tables()`.
//! The module carries 3 inline `#[cfg(test)]` tests (backend's stopgap
//! since `tests/**` is QA's owned surface). This file is QA's independent
//! catalogue adoption: different real items and a genuinely distinct
//! "resolves, but that entry legitimately carries no cost" case the inline
//! tests didn't exercise (only "resolves with a real cost" and "doesn't
//! resolve at all" — this file adds the third, easily-confused-with-free
//! case explicitly, since callers must treat all three differently: a
//! found-with-cost item is purchasable, but both "not found" and
//! "found-but-costless" must Block, never silently treat as free).

use codex::rules_core::equipment_resolver::equipment_cost_gp_headless_resolve;

#[test]
fn resolves_a_real_items_cost_by_the_legacy_item_prefix() {
    // A different real item than the inline tests' own Longsword sample.
    assert_eq!(equipment_cost_gp_headless_resolve("item:torch"), Some(0.01));
}

#[test]
fn resolves_a_real_items_cost_via_the_normalized_name_fallback_tier_specifically() {
    // "backpack" (lowercase) matches neither the real entry's exact key
    // ("Backpack") nor its exact name ("Backpack") at the resolver's first two
    // match tiers (case-sensitive equality) -- only the third, normalized-name
    // tier (lowercase, spaces-to-underscores) catches it. The inline tests only
    // exercised the first two tiers (exact key, exact corpus key); this is the
    // one they didn't cover.
    assert_eq!(equipment_cost_gp_headless_resolve("item:backpack"), Some(2.0));
}

#[test]
fn returns_none_for_a_real_item_that_legitimately_carries_no_cost_gp() {
    // "Bonded Object" (an Arcane Bond equipmod special quality, not a purchasable
    // item in its own right) resolves to a genuine equipment_tables() entry --
    // unlike an unknown item_id, this is a real match -- but that entry's own
    // cost_gp field is None (a formula-priced/template record, per
    // EquipmentTableEntry.cost_gp's own doc comment). The caller-facing contract
    // (per the module's doc comment) is that this must be treated identically to
    // "not found" -- Blocked, never a free purchase -- so it's important this is
    // genuinely None, not a fabricated 0.0.
    assert_eq!(
        equipment_cost_gp_headless_resolve("Special Quality ~ Bonded Object"),
        None
    );
    assert_eq!(equipment_cost_gp_headless_resolve("Bonded Object"), None);
}

#[test]
fn returns_none_for_an_item_id_with_no_matching_entry_at_all() {
    assert_eq!(
        equipment_cost_gp_headless_resolve("item:this_item_does_not_exist_in_any_table"),
        None
    );
}

#[test]
fn resolves_consistently_regardless_of_the_legacy_item_prefix() {
    // The `item:` prefix stripping must not change which entry resolves --
    // cross-checks that prefixed and bare forms of the same real item id agree,
    // a property none of the inline tests assert explicitly (each inline test
    // only checks one form or the other, never both against the same item).
    assert_eq!(
        equipment_cost_gp_headless_resolve("item:torch"),
        equipment_cost_gp_headless_resolve("Torch")
    );
}
