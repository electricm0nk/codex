//! SD18 per-class per-level widening proofs, one binary.
//!
//! AT-35-E1-003 (SD-35 Epic 1): the 89 `tests/sd18_*_widening.rs` files were each their own
//! integration-test binary — 89 links against the `codex` library per build. They are now
//! the rows of the roster table below, one module per (class, level) row, in this single
//! binary. Every module keeps its original `#[test]` functions and assertions unchanged; the
//! only edits were the three crate-root-relative constructs (`mod common;`, `use common::`,
//! `include_str!("fixtures/…")`), which now resolve through this file.
//!
//! Run one row: `cargo test --test sd18_widening <class>_level<N>::`
//! Run one class: `cargo test --test sd18_widening <class>_level`

#[path = "../common/mod.rs"]
mod common;

/// The roster table: `class => [module, …]`. Each module is one file in this directory and one
/// (class, level) row of the family. Adding a level is adding a row.
macro_rules! roster {
    ( $( $class:ident => [ $( $module:ident ),+ $(,)? ] ),+ $(,)? ) => {
        $( $( mod $module; )+ )+
    };
}

roster! {
    barbarian => [
        barbarian_level12,
        barbarian_level13,
        barbarian_level14,
        barbarian_level15,
        barbarian_level16,
        barbarian_level17,
        barbarian_level18,
        barbarian_level19,
        barbarian_level20,
    ],
    bard => [
        bard_level11_inspire,
        bard_level12,
        bard_level13,
        bard_level14,
        bard_level15,
        bard_level16,
        bard_level17,
        bard_level18,
        bard_level19,
        bard_level20,
    ],
    cleric => [
        cleric_level11,
        cleric_level12,
        cleric_level13,
        cleric_level14,
        cleric_level15,
        cleric_level16,
        cleric_level17,
        cleric_level18,
        cleric_level19,
        cleric_level20,
    ],
    druid => [
        druid_level11,
        druid_level12,
        druid_level13,
        druid_level14,
        druid_level15,
    ],
    fighter => [
        fighter_level12,
        fighter_level13,
        fighter_level14,
        fighter_level15,
        fighter_level16,
        fighter_level17,
        fighter_level18,
        fighter_level19,
        fighter_level20,
    ],
    monk => [
        monk_level12,
    ],
    paladin => [
        paladin_level12,
        paladin_level13,
        paladin_level14,
        paladin_level15,
        paladin_level16,
        paladin_level17,
        paladin_level18,
        paladin_level19,
        paladin_level20,
    ],
    ranger => [
        ranger_level12,
        ranger_level13,
        ranger_level14,
        ranger_level15,
        ranger_level18,
        ranger_level19,
        ranger_level20,
    ],
    rogue => [
        rogue_level12,
        rogue_level13,
        rogue_level14,
        rogue_level15,
        rogue_level16,
        rogue_level17,
        rogue_level18,
        rogue_level19,
        rogue_level20,
    ],
    sorcerer => [
        sorcerer_level11,
        sorcerer_level12,
        sorcerer_level13,
        sorcerer_level14,
        sorcerer_level15,
        sorcerer_level16,
        sorcerer_level17,
        sorcerer_level18,
        sorcerer_level19,
        sorcerer_level20,
    ],
    wizard => [
        wizard_level11,
        wizard_level12,
        wizard_level13,
        wizard_level14,
        wizard_level15,
        wizard_level16,
        wizard_level17,
        wizard_level18,
        wizard_level19,
        wizard_level20,
    ],
}
