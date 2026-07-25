//! SD-20 Epic 7 (Level Up grant model): Druid — fourth core class per
//! Step 2's stated order (barbarian, bard, cleric, druid, ...;
//! `scope-draft.md` §1.7, `technical-design.md` §2.6).
//!
//! Per §1.7's per-cycle test description: advancing a `CharacterInput`
//! from level N to N+1 produces a `LevelUpPlan` whose `automatic_features`
//! matches the published CRB table at level N+1 for the character's class.
//! Uses the identical deterministic Human Druid fixture posture the
//! SD13/SD18 Druid proof suite already established
//! (`tests/fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt`
//! / `..._level15_sd18_widening_deterministic_input.txt`: race_id
//! `"race:human"`, ability scores Str 10 / Dex 14 / Con 13 / Int 8 / Wis 17
//! / Cha 12 — the exact input shape `pilot_compute.rs`'s
//! `explain_druid_level1_spell_baseline` requires to fire at all).
//!
//! Expected level-2 grants, cross-checked directly against
//! `pilot_compute.rs`'s own grounded druid explanations
//! (`tests/sd13_druid_base_attack_and_saves.rs`,
//! `tests/sd13_druid_level2_progression.rs`,
//! `tests/sd18_druid_level15_widening.rs` establish these same numbers
//! independently):
//! - base attack bonus 0 -> 1 (3/4 BAB, `classlevel * 3 / 4`)
//! - Fortitude save 2 -> 3 (good save, `classlevel/2+2`)
//! - Will save 2 -> 3 (good save, `classlevel/2+2`; Reflex stays +0 at
//!   level 2, `classlevel/3`, no grant)
//! - Wild Empathy 2 -> 3 (druid level + Charisma modifier +1)
//! - Woodland Stride newly granted (a 2nd-level Druid class feature, value
//!   always 0 whether absent or granted — the grant-state-change case, not
//!   a value-change case)
//!
//! Not yet at level 2: Trackless Step (3rd-level), Resist Nature's Lure
//! (4th-level) — neither should produce a grant for this transition,
//! proving the diff does not over-fire on every level-gated feature.
//! Nature Sense stays flat (+2, level-independent) and must not produce a
//! spurious grant either.
//!
//! **Table-store note (not a blocker on this cycle):** unlike Barbarian's
//! cycle, this module does NOT compose with
//! `rules_tables::crb::class_tables::class_tables()` for the base-save
//! columns. `class_tables.rs`'s `CLASS_META` entry for
//! `ClassId::Druid` carries `good_saves: GoodSaves { fortitude: false,
//! reflex: false, will: true }`, which computes a *poor* Fortitude save —
//! contradicting the already-grounded, primary-source-verified (d20pfsrd +
//! legacy.aonprd.com, cross-checked independently before any code was
//! written) `pilot_compute.rs` formula this test file cites above (good
//! Fortitude, poor Reflex, good Will — the same shape PF1's Cleric has,
//! and `class_tables.rs`'s own Cleric row shows the identical
//! `fortitude: false` pattern, suggesting the same transcription bug there
//! too). Composing with the buggy column would land a wrong number on a
//! future printed character sheet, so this cycle reads
//! `pilot_compute::explain_druid_level1_spell_baseline`'s own grounded
//! `class_chassis.druid.base_attack_bonus` /
//! `class_chassis.druid.base_save.{fortitude,reflex,will}` explanations
//! directly instead (composed read-only, not re-derived) for ALL four
//! class-generic pillars, keeping a single consistent, verified-correct
//! provenance rather than mixing a known-buggy column with a correct one.
//! `class_tables.rs` is outside this cycle's file-touch partition
//! (Epic 7 cycles may touch only `level_up.rs` / `level_up/<class>.rs` /
//! this test file), so the `CLASS_META` bug itself is not fixed here — it
//! is flagged in the progress doc for a future SD-19 cycle.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::{compute_level_up_grants, PickCategory};

fn human_druid_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_levelup_druid".to_string()),
        source_package_id: "sd20_levelup_druid".to_string(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:druid".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 14,
                constitution: 13,
                intelligence: 8,
                wisdom: 17,
                charisma: 12,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn druid_level_1_to_2_grants_bab_rise_fort_will_rise_and_woodland_stride() {
    let character = human_druid_input(1);
    let plan = compute_level_up_grants(&character, 1, 2);

    let grant = |name_fragment: &str| {
        plan.automatic_features
            .iter()
            .find(|grant| grant.name.contains(name_fragment))
    };

    let bab_grant = grant("base attack bonus")
        .unwrap_or_else(|| panic!("expected a base attack bonus grant: {:?}", plan.automatic_features));
    assert_eq!(bab_grant.effects.len(), 1);
    assert_eq!(bab_grant.effects[0].value, 1, "druid level 2 base attack bonus (3/4 BAB) must be +1");
    assert_eq!(bab_grant.source_table.table, "pilot_compute::explain_druid_level1_spell_baseline");
    assert_eq!(bab_grant.source_table.row_key, "druid:2");
    assert_eq!(bab_grant.source_table.column_key, "class_chassis.druid.base_attack_bonus");

    let fort_grant = grant("base save fortitude")
        .unwrap_or_else(|| panic!("expected a base save fortitude grant: {:?}", plan.automatic_features));
    assert_eq!(fort_grant.effects[0].value, 3, "druid level 2 good Fortitude save must be +3");

    let will_grant = grant("base save will")
        .unwrap_or_else(|| panic!("expected a base save will grant: {:?}", plan.automatic_features));
    assert_eq!(will_grant.effects[0].value, 3, "druid level 2 good Will save must be +3");

    // Reflex stays +0 at level 2 (poor save, level / 3 = 0 at both 1 and
    // 2) — must NOT produce a grant.
    assert!(
        grant("base save reflex").is_none(),
        "reflex save must not change 1 -> 2"
    );

    let wild_empathy_grant = grant("wild empathy")
        .unwrap_or_else(|| panic!("expected a wild empathy grant: {:?}", plan.automatic_features));
    assert_eq!(
        wild_empathy_grant.effects[0].value, 3,
        "druid level 2 Wild Empathy (level + Cha mod 1) must be 3"
    );

    let woodland_stride_grant = grant("woodland stride").unwrap_or_else(|| {
        panic!(
            "expected a woodland stride grant at level 2: {:?}",
            plan.automatic_features
        )
    });
    assert!(
        woodland_stride_grant.effects[0]
            .description
            .contains("granted at druid level 2"),
        "woodland stride grant must cite the real grounded explanation text: {:?}",
        woodland_stride_grant.effects[0]
    );

    // Not yet granted at level 2 — must NOT produce a spurious grant.
    assert!(
        grant("trackless step").is_none(),
        "trackless step is a 3rd-level feature, not 2nd"
    );
    assert!(
        grant("resist natures lure").is_none(),
        "resist nature's lure is a 4th-level feature, not 2nd"
    );

    // Nature Sense is flat and level-independent — must NOT produce a
    // spurious grant.
    assert!(
        grant("nature sense").is_none(),
        "nature sense is flat and level-independent; it must not re-fire on every level"
    );

    assert!(!plan.capstone_threshold, "level 2 is not the capstone level");

    // No resource pool exists for Druid at these grounded levels (Wild
    // Shape uses/day is explicitly named-but-unproven in
    // pilot_compute.rs, not a flat value) — must stay honestly empty.
    assert!(
        plan.resource_pool_change.pools.is_empty(),
        "no grounded resource pool exists for druid at level 2"
    );

    // No pick-list catalog exists for Druid in this repo — must stay
    // honestly empty rather than fabricated.
    assert!(
        plan.pick_from_lists.is_empty(),
        "no Druid pick-list candidate catalog exists yet; pick_from_lists must not be fabricated"
    );
}

#[test]
fn druid_level_14_to_15_grants_timeless_body_and_is_not_yet_capstone() {
    let character = human_druid_input(14);
    let plan = compute_level_up_grants(&character, 14, 15);

    let timeless_body_grant = plan
        .automatic_features
        .iter()
        .find(|grant| grant.name.contains("timeless body"))
        .unwrap_or_else(|| {
            panic!(
                "expected a timeless body grant at level 15: {:?}",
                plan.automatic_features
            )
        });
    assert!(
        timeless_body_grant.effects[0]
            .description
            .contains("granted at druid level 15"),
        "timeless body grant must cite the real grounded explanation text: {:?}",
        timeless_body_grant.effects[0]
    );

    assert!(
        !plan.capstone_threshold,
        "level 15 is Druid's grounded ceiling, not the PF1 Core Rulebook capstone (level 20)"
    );
}

#[test]
fn druid_level_19_to_20_crosses_the_capstone_threshold_without_fabricating_features() {
    let character = human_druid_input(19);
    let plan = compute_level_up_grants(&character, 19, 20);

    // Level 20 is the PF1 Core Rulebook capstone for every core class —
    // a pure level-number fact, not derived from any per-class data
    // source, so this stays true even beyond Druid's own grounded
    // chassis ceiling (`MAX_SUPPORTED_DRUID_LEVEL = 15` in
    // pilot_compute.rs).
    assert!(
        plan.capstone_threshold,
        "level 20 is the PF1 Core Rulebook capstone level for every core class"
    );

    // (v0.6 alpha swarm, risks item 8) Beyond Druid's grounded CLASS-FEATURE
    // ceiling (level 15), pilot_compute.rs's chassis still produces no
    // class_chassis.druid.* or class_feature.druid.* explanations -- that
    // part of this test's original premise still holds. But the seventh
    // slice's prepared-divine spell grounding (ground_druid_prepared_spells)
    // is checked unconditionally for any druid class level, not gated by
    // MAX_SUPPORTED_DRUID_LEVEL, and reuses Cleric's real, full-range PF1
    // Core Rulebook spells-per-day table (byte-for-byte identical, valid
    // through level 20) -- so class_spell.druid.total_spells_per_day.*
    // genuinely keeps rising past level 15 too. This is real, grounded
    // spell math, not fabrication: the plan must reflect it rather than
    // over-claiming an emptiness that no longer holds.
    assert!(
        !plan.automatic_features.is_empty(),
        "class_spell.druid.total_spells_per_day.* is real, grounded spell math (Cleric's shared \
         table, valid through level 20) and must still surface a grant on the level 19 -> 20 \
         transition, even beyond Druid's own class-feature ceiling of 15: {:?}",
        plan.automatic_features
    );
    assert!(
        plan.automatic_features
            .iter()
            .all(|grant| grant.source_table.column_key.starts_with("class_spell.druid.")),
        "beyond Druid's grounded class-feature ceiling (level 15), only real class_spell.druid.* \
         spell-math grants may appear -- no class_chassis.druid.* or class_feature.druid.* \
         explanation is fabricated: {:?}",
        plan.automatic_features
    );
}

#[test]
fn non_druid_class_returns_an_honestly_empty_plan() {
    let mut character = human_druid_input(1);
    character.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:oracle".to_string(),
        level: 1,
    }];
    let plan = compute_level_up_grants(&character, 1, 2);

    assert!(plan.automatic_features.is_empty());
    assert!(plan.pick_from_lists.is_empty());
    assert!(plan.resource_pool_change.pools.is_empty());
    assert!(!plan.capstone_threshold);

    // Sanity: the type is actually reachable and PickCategory has the
    // variant this module's sibling (barbarian.rs) doc comment names
    // for a future cycle — Druid's own module does not name a
    // Druid-specific pick category, but the shared enum must still
    // compile and be usable here.
    let _ = PickCategory::Spell;
}
