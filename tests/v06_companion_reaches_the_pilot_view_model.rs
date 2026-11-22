//! v0.6 alpha swarm: proves the animal companion / mount stat block the
//! engine already grounds actually reaches the `PilotViewModel` --- the
//! surface every consumer downstream of rules-core (including the desktop
//! sheet's Pets tab) reads.
//!
//! Why a separate integration test rather than only the unit tests in
//! `pilot_view_model.rs`: those drive `PilotSnapshot::from_receipt` with
//! synthetic receipts this file's author wrote, so on their own they prove
//! the projection is self-consistent, not that it matches what the engine
//! genuinely emits. If `pilot_compute.rs` ever renames a companion record
//! id or reorders the family, the synthetic tests would stay green while
//! the real Pets tab silently emptied. This file closes that gap by
//! running a real `CharacterInput` through the real
//! `build_pilot_headless_receipt` and asserting on the projection of the
//! genuine receipt.
//!
//! It grounds no new rules truth: every value asserted here is one the
//! engine already computed and `pilot_compute.rs`'s own
//! `single_class_druid_level1_with_animal_companion_reaches_computed`
//! already pins.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput, SelectedChoice, load_character_input_fixture};
use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
use codex::rules_core::pilot_view_model::PilotViewModel;

const FIGHTER_LEVEL_1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// Mirrors `pilot_compute.rs`'s own `human_druid_input_with_nature_bond`
/// test helper (which is private to that module's test scope): the
/// deterministic Human fighter fixture with its class levels replaced by
/// the requested Druid level, plus the animal-companion nature bond.
fn human_druid_with_animal_companion(level: u8) -> CharacterInput {
    let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    let mut input = result.character_input.expect("valid fixture");
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:druid".to_owned(),
        level,
    }];
    input.chosen.selected_choices.push(SelectedChoice {
        choice_set_id: "choice:druid_nature_bond".to_owned(),
        selection_id: "bond:animal_companion".to_owned(),
    });
    input
}

/// A plain Human fighter -- a class that grounds no companion at all.
fn human_fighter_level1() -> CharacterInput {
    let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
    assert!(result.diagnostics.is_empty());
    result.character_input.expect("valid fixture")
}

#[test]
fn a_real_level1_druids_wolf_companion_reaches_the_view_model_with_its_real_values() {
    let receipt = build_pilot_headless_receipt(&human_druid_with_animal_companion(1));
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "precondition: a level-1 Druid with an animal companion reaches Computed: {:?}",
        receipt.computation.diagnostics
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    let companion = view_model
        .snapshot
        .expect("a Computed receipt carries a snapshot")
        .companion
        .expect("the Druid's real animal companion must reach the view model");

    assert_eq!(companion.owner_class_label, "Druid");
    assert_eq!(companion.role_label, "Animal Companion");
    assert_eq!(companion.species, "Wolf");

    // Every value below is the engine's own, already pinned by
    // `single_class_druid_level1_with_animal_companion_reaches_computed`:
    // Wolf at 2 HD, BAB 2*3/4 = 1 plus Str 13's +1 modifier = 2; Fort/Ref
    // 2/2+2 = 3; Will 2/3 = 0; AC 10 + 2 natural armor = 12; bite damage
    // bonus floor(1.5 * +1) = 1; HP (8+2) + 1 * (5+2) = 17.
    let rendered: Vec<(&str, i16)> = companion
        .stats
        .iter()
        .map(|stat| (stat.label.as_str(), stat.value))
        .collect();
    assert_eq!(
        rendered,
        vec![
            ("Hit Points", 17),
            ("Armor Class", 12),
            ("Attack Bonus", 2),
            ("Bite Damage Bonus", 1),
            ("Fortitude Save", 3),
            ("Reflex Save", 3),
            ("Will Save", 0),
        ],
        "the projection must render the engine's real emitted record family, not a \
         self-consistent mirror of it"
    );

    for stat in &companion.stats {
        assert!(
            !stat.detail.trim().is_empty(),
            "every row carries the engine's own derivation prose: {stat:?}"
        );
    }
    assert!(
        companion.summary_detail.contains("Wolf"),
        "the recognition record's summary must reach the view model: {:?}",
        companion.summary_detail
    );

    // The honest "deliberately not grounded" list -- bonus tricks, the
    // companion's skills and feats, the player-chosen stat increase, the
    // size advance, Evasion/Devotion/Multiattack. Non-blocking, so it is
    // dropped from the diagnostics on a Computed load and would never
    // reach a player any other way.
    let advancement_note = companion
        .advancement_note
        .expect("the engine's non-blocking advancement-absent note must travel with the companion");
    for deferred in [
        "bonus tricks",
        "skill ranks and feats",
        "Companion Stat Increase",
        "size advance",
        "Evasion",
        "Devotion",
        "Multiattack",
    ] {
        assert!(
            advancement_note.contains(deferred),
            "the advancement note must still name {deferred:?} verbatim: {advancement_note}"
        );
    }

    assert!(
        companion
            .notes
            .iter()
            .any(|note| note.contains("Link") || note.contains("Share Spells")),
        "the provably-vacuous named-ability records must travel too: {:?}",
        companion.notes
    );
}

/// The progression is grounded across all twenty master levels, so the
/// view model must move with it rather than pinning a level-1 snapshot.
#[test]
fn the_companions_real_progression_moves_with_the_masters_level() {
    let level_one = build_pilot_headless_receipt(&human_druid_with_animal_companion(1));
    let level_twenty = build_pilot_headless_receipt(&human_druid_with_animal_companion(20));

    let hit_points = |receipt: &codex::rules_core::pilot_compute::PilotHeadlessReceipt| {
        PilotViewModel::from_receipt(receipt)
            .snapshot
            .expect("Computed")
            .companion
            .expect("companion")
            .stats
            .iter()
            .find(|stat| stat.label == "Hit Points")
            .expect("hit points")
            .value
    };

    assert!(
        hit_points(&level_twenty) > hit_points(&level_one),
        "a 20th-level master's companion (16 HD) must not read a 1st-level master's (2 HD)"
    );
}

/// The empty state is a real state, not a zeroed stat block.
#[test]
fn a_companionless_class_carries_no_companion_at_all() {
    let receipt = build_pilot_headless_receipt(&human_fighter_level1());
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed);

    let companion = PilotViewModel::from_receipt(&receipt)
        .snapshot
        .expect("Computed")
        .companion;

    assert!(
        companion.is_none(),
        "a Fighter has no animal companion -- the view model must say nothing rather than \
         render an empty or zeroed one: {companion:?}"
    );
}
