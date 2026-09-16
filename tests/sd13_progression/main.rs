//! SD13-E5 per-class per-level progression proofs, one binary.
//!
//! AT-35-E1-003 (SD-35 Epic 1): the 95 `tests/sd13_*progression*.rs` files were each their own
//! integration-test binary — 95 links against the `codex` library per build. They are now
//! the rows of the roster table below, one module per (class, level) row, in this single
//! binary. Every module keeps its original `#[test]` functions and assertions unchanged; the
//! only edits were the three crate-root-relative constructs (`mod common;`, `use common::`,
//! `include_str!("fixtures/…")`), which now resolve through this file.
//!
//! Run one row: `cargo test --test sd13_progression <class>_level<N>::`
//! Run one class: `cargo test --test sd13_progression <class>_level`

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
        barbarian_level2,
        barbarian_level3,
        barbarian_level4,
        barbarian_level5,
        barbarian_level6,
        barbarian_level7,
        barbarian_level8,
        barbarian_level9,
        barbarian_level10,
    ],
    bard => [
        bard_level2,
        bard_level3,
        bard_level4,
        bard_level5,
        bard_level6,
        bard_level7,
        bard_level8,
        bard_level9,
        bard_level10,
    ],
    cleric => [
        cleric_level2,
        cleric_level3,
        cleric_level4,
        cleric_level5,
        cleric_level6,
        cleric_level7,
        cleric_level8,
        cleric_level9,
        cleric_level10,
    ],
    druid => [
        druid_level2,
        druid_level3,
        druid_level4,
        druid_level5,
        druid_level6,
        druid_level7,
        druid_level8,
        druid_level9,
        druid_level10,
    ],
    fighter => [
        fighter_level2_level3,
        fighter_level4,
        fighter_level5,
        fighter_level6,
        fighter_level7,
        fighter_level8,
        fighter_level9_level10,
    ],
    monk => [
        monk_level2,
        monk_level3,
        monk_level4,
        monk_level5,
        monk_level6,
        monk_level7,
        monk_level8,
        monk_level9,
        monk_level10,
    ],
    paladin => [
        paladin_level4,
        paladin_level5,
        paladin_level6,
        paladin_level7,
        paladin_level8,
        paladin_level9,
        paladin_level10,
    ],
    ranger => [
        ranger_level2,
        ranger_level3,
        ranger_level4,
        ranger_level5,
        ranger_level6,
        ranger_level7,
        ranger_level8,
        ranger_level9,
        ranger_level10,
    ],
    rogue => [
        rogue_level2,
        rogue_level3,
        rogue_level4,
        rogue_level5,
        rogue_level6,
        rogue_level7,
        rogue_level8,
        rogue_level9,
        rogue_level10,
    ],
    sorcerer => [
        sorcerer_level2,
        sorcerer_level3,
        sorcerer_level4,
        sorcerer_level5,
        sorcerer_level6,
        sorcerer_level7,
        sorcerer_level8,
        sorcerer_level9,
        sorcerer_level10,
    ],
    wizard => [
        wizard_level2,
        wizard_level3,
        wizard_level4,
        wizard_level5,
        wizard_level6,
        wizard_level7,
        wizard_level8,
        wizard_level9,
        wizard_level10,
    ],
}
