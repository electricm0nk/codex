//! v0.6 Druid level-16..level-20 widening grounding proof.
//!
//! Widens the accepted Druid level-1..level-15 chassis
//! (`tests/sd18_druid_level15_widening.rs`, the prior Druid ceiling) to the
//! PF1 level cap, closing the last five blocked Druid levels so the class
//! computes at every level 1-20.
//!
//! Every level-16..20 value asserted here was transcribed from the PCGen PF1
//! Core Rulebook data set at
//! `data/pathfinder/paizo/roleplaying_game/core_rulebook/`, read as whole
//! records (including the `.MOD` blocks that a token-filtered grep hides),
//! rather than assumed to continue the level-1..15 pattern:
//!
//! - `cr_classes.lst` line 93 (`CLASS:Druid`) carries `MAXLEVEL:20` and the
//!   three progression formulas this widening relies on:
//!   `BONUS:COMBAT|BASEAB|classlevel(...)*3/4`,
//!   `BONUS:SAVE|BASE.Fortitude,BASE.Will|classlevel(...)/2+2`, and
//!   `BONUS:SAVE|BASE.Reflex|classlevel(...)/3`. Druid's BAB and save
//!   progressions are byte-for-byte identical to `CLASS:Cleric`
//!   (`cr_classes.lst` line 55), which already computes at all 20 levels.
//! - `cr_classes.lst` lines 131-135 are the real Druid `CAST:` rows for
//!   levels 16-20 -- `16 CAST:4,4,4,4,4,4,3,3,2`,
//!   `17 CAST:4,4,4,4,4,4,4,3,2,1`, `18 CAST:4,4,4,4,4,4,4,3,3,2`,
//!   `19 CAST:4,4,4,4,4,4,4,4,3,3`, `20 CAST:4,4,4,4,4,4,4,4,4,4`.
//!   These are byte-for-byte identical to Cleric's own rows at
//!   `cr_classes.lst` lines 85-89, which is what already justifies
//!   `druid_base_spells_per_day_table` delegating to
//!   `cleric_base_spells_per_day_table`. Level 16 has NINE columns; the
//!   tenth (9th-level druid spells) first appears at level 17.
//! - `cr_abilities_class.lst` lines 223-234 are the complete
//!   `CATEGORY=Class|Druid.MOD` class-feature block. Its highest gate is
//!   `PREVARGTEQ:Druid_CFP_Level,15` (Timeless Body, line 234). There is NO
//!   new named Druid class feature at 16, 17, 18, 19, or 20 — checked
//!   directly against the whole block rather than inferred. Nothing
//!   text-only is therefore left unshown to the player by this slice.
//! - The only level-16/18/20 "Special" column entries are Wild Shape
//!   frequency increments, from `cr_abilities_class.lst` line 853:
//!   `BONUS:VAR|WildShapeTimes|(DruidLVL>=4)+(DruidLVL>=6)+(DruidLVL>=8)+
//!   (DruidLVL>=10)+(DruidLVL>=12)+(DruidLVL>=14)+(DruidLVL>=16)+
//!   (DruidLVL>=18)+(DruidLVL>=20)` — 7/day at 16, 8/day at 18, and the
//!   terminal step at 20 that line 796's
//!   `DESC:You can change shape at will ...|PREVARGTEQ:WildShapeProgression,9`
//!   renders as "at will". Wild Shape's FORM capability does not improve
//!   past level 12: the `WildShapeAbilityLevel` grants at lines 857-861 stop
//!   at `PREVARGTEQ:DruidWildShape,12`. Wild Shape stays deliberately
//!   named-but-unproven at 16-20 exactly as it already is at levels
//!   4/6/8/10/12/14 — this repo has no shapeshifting execution engine, and
//!   this slice fabricates none.
//!
//! It deliberately does not touch the animal-companion execution burden, the
//! Wild Shape execution burden, or the prepared divine spell posture burden,
//! and it preserves the accepted Druid level-1..level-15 truth unchanged.

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
mod common;
use common::load;

const DRUID_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level15_sd18_widening_deterministic_input.txt"
);

const DRUID_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level16_v06_widening_deterministic_input.txt"
);

const DRUID_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_druid_level20_v06_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation
                    .explanations
                    .iter()
                    .map(|e| e.id.as_str())
                    .collect::<Vec<_>>()
            )
        })
}

fn at_level(level: u8) -> PilotBaseChassisComputation {
    let fixture = DRUID_LEVEL20_FIXTURE.replace("class:druid:20", &format!("class:druid:{level}"));
    compute_pilot_base_chassis(&load(&fixture))
}

// ----- Base attack bonus and saves across the whole 16..20 band -----

#[test]
fn druid_level16_base_attack_and_good_saves_rise_while_reflex_stays() {
    let computation = compute_pilot_base_chassis(&load(DRUID_LEVEL16_FIXTURE));

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Druid level 16 3/4-BAB progression (16 * 3 / 4) must genuinely rise to 12: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Druid level 16 good Fortitude (16/2+2) must genuinely rise to 10"
    );

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(
        will.value, 10,
        "Druid level 16 good Will (16/2+2) must genuinely rise to 10"
    );

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 5,
        "Druid level 16 poor Reflex (16/3) must STAY 5, an integer-division coincidence with \
         level 15 -- checked, not assumed to keep rising"
    );
}

#[test]
fn druid_level20_capstone_base_attack_and_saves_match_the_corpus_formulas() {
    let computation = compute_pilot_base_chassis(&load(DRUID_LEVEL20_FIXTURE));

    let base_attack = explanation(&computation, "class_chassis.druid.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Druid level 20 3/4-BAB progression (20 * 3 / 4) must be 15: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.druid.base_save.fortitude");
    assert_eq!(fortitude.value, 12, "Druid level 20 good Fortitude (20/2+2) must be 12");

    let will = explanation(&computation, "class_chassis.druid.base_save.will");
    assert_eq!(will.value, 12, "Druid level 20 good Will (20/2+2) must be 12");

    let reflex = explanation(&computation, "class_chassis.druid.base_save.reflex");
    assert_eq!(
        reflex.value, 6,
        "Druid level 20 poor Reflex (20/3) must be 6, STAYING at the level-18/19 value rather \
         than rising -- checked, not assumed"
    );
}

/// Every intermediate level in the newly opened band, pinned against the
/// `cr_classes.lst` line 93 formulas rather than spot-checked at the edges.
#[test]
fn druid_levels_16_through_20_match_the_corpus_progression_row_by_row() {
    // (level, base attack bonus, good save (Fort == Will), poor Reflex)
    for (level, bab, good_save, reflex) in [
        (16u8, 12i16, 10i16, 5i16),
        (17, 12, 10, 5),
        (18, 13, 11, 6),
        (19, 14, 11, 6),
        (20, 15, 12, 6),
    ] {
        let computation = at_level(level);

        assert_eq!(
            explanation(&computation, "class_chassis.druid.base_attack_bonus").value,
            bab,
            "Druid level {level} base attack bonus ({level} * 3 / 4) must be {bab}"
        );
        assert_eq!(
            explanation(&computation, "class_chassis.druid.base_save.fortitude").value,
            good_save,
            "Druid level {level} good Fortitude ({level}/2+2) must be {good_save}"
        );
        assert_eq!(
            explanation(&computation, "class_chassis.druid.base_save.will").value,
            good_save,
            "Druid level {level} good Will ({level}/2+2) must be {good_save}"
        );
        assert_eq!(
            explanation(&computation, "class_chassis.druid.base_save.reflex").value,
            reflex,
            "Druid level {level} poor Reflex ({level}/3) must be {reflex}"
        );
    }
}

// ----- Wild Empathy keeps rising on the same level-generic formula -----

#[test]
fn druid_wild_empathy_rises_across_the_newly_opened_band() {
    for (level, expected) in [(16u8, 17i16), (17, 18), (18, 19), (19, 20), (20, 21)] {
        let computation = at_level(level);
        let wild_empathy = explanation(&computation, "class_chassis.druid.wild_empathy");
        assert_eq!(
            wild_empathy.value, expected,
            "Druid level {level} Wild Empathy (druid level {level} + Charisma modifier +1) must \
             equal {expected}: {}",
            wild_empathy.detail
        );
    }
}

// ----- Spells per day, transcribed from cr_classes.lst lines 131-135 -----

/// Level 16's `CAST:4,4,4,4,4,4,3,3,2` (`cr_classes.lst` line 131) has nine
/// columns. The 6th/7th/8th-level entries are the ones that actually differ
/// from level 15's row, and none of them takes a Wisdom bonus at this
/// fixture's +4 Wisdom modifier, so they read the base table directly.
#[test]
fn druid_level16_base_spells_per_day_match_the_corpus_cast_row() {
    let computation = compute_pilot_base_chassis(&load(DRUID_LEVEL16_FIXTURE));

    for (spell_level, expected) in [(6usize, 3i16), (7, 3), (8, 2)] {
        let record = explanation(
            &computation,
            &format!("class_chassis.druid.base_spells_per_day.spell_level_{spell_level}"),
        );
        assert_eq!(
            record.value, expected,
            "Druid level 16 base spells per day at spell level {spell_level} must be {expected} \
             (cr_classes.lst line 131, CAST:4,4,4,4,4,4,3,3,2): {}",
            record.detail
        );
    }

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_spells_per_day.spell_level_9"),
        "Druid level 16 must have NO 9th-level spell slots: cr_classes.lst line 131 has only \
         nine columns, and the tenth first appears at level 17 (line 132): {:?}",
        computation
            .explanations
            .iter()
            .map(|e| e.id.as_str())
            .collect::<Vec<_>>()
    );
}

/// Level 17's `CAST:4,4,4,4,4,4,4,3,2,1` (`cr_classes.lst` line 132) is the
/// first Druid row with a tenth column — the real 9th-level-spell boundary.
#[test]
fn druid_level17_opens_ninth_level_spells_for_the_first_time() {
    let computation = at_level(17);
    let ninth = explanation(
        &computation,
        "class_chassis.druid.base_spells_per_day.spell_level_9",
    );
    assert_eq!(
        ninth.value, 1,
        "Druid level 17 must gain exactly one base 9th-level slot (cr_classes.lst line 132, \
         CAST:4,4,4,4,4,4,4,3,2,1): {}",
        ninth.detail
    );
}

/// Level 20's `CAST:4,4,4,4,4,4,4,4,4,4` (`cr_classes.lst` line 135) is the
/// only Druid row where every spell level from orisons through 9th is 4.
#[test]
fn druid_level20_base_spells_per_day_are_a_flat_four_at_every_spell_level() {
    let computation = compute_pilot_base_chassis(&load(DRUID_LEVEL20_FIXTURE));

    for spell_level in 0usize..=9 {
        let record = explanation(
            &computation,
            &format!("class_chassis.druid.base_spells_per_day.spell_level_{spell_level}"),
        );
        assert_eq!(
            record.value, 4,
            "Druid level 20 base spells per day at spell level {spell_level} must be 4 \
             (cr_classes.lst line 135, CAST:4,4,4,4,4,4,4,4,4,4): {}",
            record.detail
        );
    }
}

// ----- No new named class feature exists at 16-20, so none is fabricated -----

/// `cr_abilities_class.lst` lines 223-234 are the complete
/// `CATEGORY=Class|Druid.MOD` block and top out at
/// `PREVARGTEQ:Druid_CFP_Level,15`. Timeless Body is genuinely Druid's last
/// named CRB class feature, so the capstone must carry exactly the same
/// feature set as level 15 — no more, no less.
#[test]
fn druid_level20_grounds_no_class_feature_beyond_timeless_body() {
    let level15 = compute_pilot_base_chassis(&load(DRUID_LEVEL15_FIXTURE));
    let level20 = compute_pilot_base_chassis(&load(DRUID_LEVEL20_FIXTURE));

    let feature_ids = |c: &PilotBaseChassisComputation| {
        let mut ids: Vec<String> = c
            .explanations
            .iter()
            .filter(|e| e.id.starts_with("class_feature.druid."))
            .map(|e| e.id.clone())
            .collect();
        ids.sort();
        ids
    };

    assert_eq!(
        feature_ids(&level20),
        feature_ids(&level15),
        "levels 16-20 add no new named Druid class feature (cr_abilities_class.lst lines \
         223-234 stop at Druid_CFP_Level,15), so the level-20 feature set must equal level 15's \
         exactly -- any extra id here is fabricated"
    );

    for (id, expected) in [
        ("class_feature.druid.woodland_stride", 0i16),
        ("class_feature.druid.trackless_step", 0),
        ("class_feature.druid.resist_natures_lure", 4),
        ("class_feature.druid.venom_immunity", 0),
        ("class_feature.druid.a_thousand_faces", 0),
        ("class_feature.druid.timeless_body", 0),
    ] {
        let record = explanation(&level20, id);
        assert_eq!(
            record.value, expected,
            "'{id}' must carry over unchanged at the level-20 capstone: {}",
            record.detail
        );
    }

    let nature_sense = explanation(&level20, "class_chassis.druid.nature_sense");
    assert_eq!(nature_sense.value, 2, "Nature Sense must stay the flat +2 at level 20");
}

/// Wild Shape's 16/18/20 frequency increments are real corpus facts
/// (`cr_abilities_class.lst` line 853), but this repo still has no
/// shapeshifting execution engine. It must stay named-but-unproven exactly
/// as at levels 4/6/8/10/12/14 rather than being half-invented here.
#[test]
fn druid_capstone_does_not_fabricate_wild_shape_execution() {
    for level in [16u8, 18, 20] {
        let computation = at_level(level);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.to_lowercase().contains("wild_shape")),
            "level-{level} Druid must not fabricate any wild-shape explanation record: {:?}",
            computation
                .explanations
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>()
        );
    }
}

// ----- Negative control: level-15 truth is unchanged by this widening -----

#[test]
fn druid_level15_truth_is_unchanged_by_this_slice() {
    let computation = compute_pilot_base_chassis(&load(DRUID_LEVEL15_FIXTURE));

    assert_eq!(
        explanation(&computation, "class_chassis.druid.base_attack_bonus").value,
        11,
        "Druid level 15 base attack bonus must stay 11"
    );
    assert_eq!(
        explanation(&computation, "class_chassis.druid.base_save.fortitude").value,
        9,
        "Druid level 15 good Fortitude must stay 9"
    );
    assert_eq!(
        explanation(&computation, "class_chassis.druid.wild_empathy").value,
        16,
        "Druid level 15 Wild Empathy must stay 16"
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.druid.base_spells_per_day.spell_level_9"),
        "Druid level 15 must still have no 9th-level slots (cr_classes.lst line 130)"
    );
}

// ----- Negative control: the druid path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_druid_capstone_recognition() {
    let fighter = compute_pilot_base_chassis(&load(FIGHTER_FIXTURE));
    assert!(
        !fighter
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.druid.")
                || e.id.starts_with("class_feature.druid.")),
        "the Fighter chassis must not surface any druid-namespaced explanation: {:?}",
        fighter
            .explanations
            .iter()
            .map(|e| e.id.as_str())
            .collect::<Vec<_>>()
    );
}

// ----- Negative control: multiclass Druid is still not promoted -----

/// Only the records behind the `supported_druid_level` gate are checked here.
/// The animal-companion and prepared-spell blocks in
/// `explain_druid_level1_spell_baseline` sit BEFORE that gate and fire for any
/// Druid level including multiclass, so asserting on the whole
/// `class_chassis.druid.` / `class_feature.druid.` namespace would be
/// asserting something that was never true.
#[test]
fn multiclass_druid_level20_is_not_promoted_by_this_slice() {
    let multiclass = DRUID_LEVEL20_FIXTURE.replace(
        "class_level=class:druid:20",
        "class_level=class:druid:20\nclass_level=class:fighter:1",
    );
    let computation = compute_pilot_base_chassis(&load(&multiclass));

    for gated in [
        "class_chassis.spell_baseline.druid",
        "class_chassis.druid.base_attack_bonus",
        "class_chassis.druid.base_save.fortitude",
        "class_chassis.druid.base_save.reflex",
        "class_chassis.druid.base_save.will",
        "class_chassis.druid.wild_empathy",
        "class_chassis.druid.nature_sense",
        "class_chassis.druid.nature_bond_choice",
        "class_feature.druid.woodland_stride",
        "class_feature.druid.trackless_step",
        "class_feature.druid.resist_natures_lure",
        "class_feature.druid.venom_immunity",
        "class_feature.druid.a_thousand_faces",
        "class_feature.druid.timeless_body",
    ] {
        assert!(
            !computation.explanations.iter().any(|e| e.id == gated),
            "multiclass Druid must not gain the gated druid record '{gated}': {:?}",
            computation
                .explanations
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>()
        );
    }

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Druid must stay claim-blocked in this slice"
    );
}

// ----- The remaining burden diagnostics still fire at the capstone -----

/// The point of this slice is to open levels 16-20, never to make a burden go
/// quiet by widening past it. Comparing the druid-namespaced diagnostic id set
/// at level 20 against the already-accepted level-15 set states that directly,
/// and stays correct as the separate animal-companion work moves that burden
/// between its claim-blocking and non-blocking forms.
#[test]
fn druid_capstone_silently_drops_no_burden_that_level15_reports() {
    let druid_diagnostic_ids = |fixture: &str| {
        let computation = compute_pilot_base_chassis(&load(fixture));
        let mut ids: Vec<String> = computation
            .diagnostics
            .iter()
            .filter(|d| d.id.contains("druid"))
            .map(|d| d.id.clone())
            .collect();
        ids.sort();
        ids.dedup();
        ids
    };

    let level15 = druid_diagnostic_ids(DRUID_LEVEL15_FIXTURE);
    assert!(
        !level15.is_empty(),
        "level-15 Druid is expected to still carry at least one druid-namespaced burden \
         diagnostic; if that is no longer true this test has lost its subject"
    );
    assert_eq!(
        druid_diagnostic_ids(DRUID_LEVEL20_FIXTURE),
        level15,
        "the level-20 capstone must report exactly the druid burdens level 15 reports -- \
         widening the level ceiling must never silence one"
    );
}
