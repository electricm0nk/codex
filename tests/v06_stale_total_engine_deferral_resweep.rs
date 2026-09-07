//! v0.6 Receipt-to-Sheet slice 1, item 5: resweep the "no <X>-total engine
//! exists" deferral justifications
//! (`docs/release/v0.6/execution-engine-scoping.md` §5), the same disease
//! `docs/release/v0.6/stale-deferral-sweep.md` found in the damage-reduction
//! family and task #88 found on `defense.baseline_armor_class` /
//! `defense.total_save.*`.
//!
//! Four claims were nominated as false. **Three are; one is not**, and this
//! file pins all four outcomes executably, so neither the stale text nor an
//! over-eager future correction of it can return silently:
//!
//! | claim | verdict |
//! |---|---|
//! | "no skill-check-total engine exists" | FALSE — `SkillTotals` is one |
//! | "no saving-throw-total engine exists" | FALSE — `total_saves` is one |
//! | "no Armor-Class-total engine exists" | FALSE — `defense.baseline_armor_class` is one |
//! | "no speed-total engine exists" | **TRUE** — see below |
//!
//! The speed claim was nominated on the grounds that `base_land_speed_feet()`
//! exists. It does, but it is a race-table *base speed lookup*, not a total: it
//! takes a race id and returns that race's `GAIT:WALK|N` value. Nothing sums a
//! base speed with the speed modifiers this engine already computes
//! (`feat.standalone.base_speed_bonus`, Oracle's
//! `lame_curse.base_land_speed_penalty`, the Flame mystery's
//! `cinder_dance_speed_bonus`), `PilotBaseChassisComputation` has no speed
//! field, and no explanation id is a speed total. Monk Fast Movement's
//! deferral on "no speed-total engine" is therefore accurate and is left
//! standing. Building that total is scoping-doc §3 follow-on work, not a text
//! correction.
//!
//! Correcting the three false claims produced **no** integration work, and that
//! is a finding rather than an omission: every magnitude attached to them is
//! conditional in a way the existing total cannot express — Dwarf Hardy on the
//! save's source, Dwarf Defensive Training on the opponent's creature subtype,
//! Monk High Jump on the jump sub-use of Acrobatics. A conditional bonus no
//! total can express is a genuine deferral; "the total does not exist" was not.

use codex::rules_core::character_input::CharacterClassLevel;
use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::skill_allocation::allocate_skill_ranks;
mod common;
use common::load;

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

const DWARF_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_dwarf_fighter_level1_sd13_deterministic_input.txt");

const HARDY_ID: &str = "race.dwarf.trait_bundle.hardy";

/// The exact phrasings nominated as false. None may survive in any string this
/// engine actually ships to a player.
const FALSE_CLAIMS: &[&str] = &[
    "no skill-check-total engine exists",
    "no saving-throw-total engine exists",
    "no Armor-Class-total engine exists",
    "no armor-class-total engine exists",
];

fn explanation_detail(computation: &PilotBaseChassisComputation, id: &str) -> String {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("expected explanation id '{id}'"))
        .detail
        .clone()
}

/// A spread of postures wide enough to reach the race seams, the class
/// chassis, and the combat/save/skill totals at once.
fn postures() -> Vec<PilotBaseChassisComputation> {
    let mut out = vec![
        compute_pilot_base_chassis(&load(FIGHTER_FIXTURE)),
        compute_pilot_base_chassis(&load(DWARF_FIXTURE)),
    ];
    for (class_name, level) in [("wizard", 5), ("monk", 5), ("monk", 20), ("cleric", 10)] {
        let mut input = load(DWARF_FIXTURE);
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: format!("class:{class_name}"),
            level,
        }];
        out.push(compute_pilot_base_chassis(&input));
    }
    out
}

/// The headline guard: no string this engine ships — explanation `detail` or
/// diagnostic `message` — may assert that one of the three real totals does not
/// exist.
#[test]
fn no_shipped_string_still_claims_a_total_engine_that_really_exists_does_not() {
    for computation in postures() {
        for claim in FALSE_CLAIMS {
            for explanation in &computation.explanations {
                assert!(
                    !explanation.detail.contains(claim),
                    "explanation '{}' still ships the false claim '{claim}': {}",
                    explanation.id,
                    explanation.detail
                );
            }
            for diagnostic in &computation.diagnostics {
                assert!(
                    !diagnostic.message.contains(claim),
                    "diagnostic '{}' still ships the false claim '{claim}': {}",
                    diagnostic.id,
                    diagnostic.message
                );
            }
        }
    }
}

/// Dwarf Hardy is the one shipped `detail` that carried a nominated claim. Its
/// deferral is correct and stays; only the justification changes, from a false
/// statement about the engine to the true statement about the bonus.
#[test]
fn dwarf_hardy_keeps_its_deferral_but_states_the_real_reason_for_it() {
    let computation = compute_pilot_base_chassis(&load(DWARF_FIXTURE));
    let detail = explanation_detail(&computation, HARDY_ID);

    assert!(
        !detail.contains("no saving-throw-total engine"),
        "Hardy must no longer justify itself with a total engine that exists: {detail}"
    );
    // The real reason: the bonus is conditional on what the save is against,
    // and the total has no by-source dimension.
    assert!(
        detail.contains("conditional"),
        "Hardy must name the conditionality that actually keeps it out of the total: {detail}"
    );
    assert!(
        detail.contains("total_saves") || detail.contains("defense.total_save"),
        "Hardy must point at the save total that really exists: {detail}"
    );
    // Unchanged: the magnitude and both save categories are still grounded.
    assert!(detail.contains("poison"), "{detail}");
    assert!(detail.contains("spell"), "{detail}");
    assert!(detail.contains("+2"), "{detail}");
}

/// Executable proof of claim 1's falsity: a real, class-skill-aware per-skill
/// total exists.
#[test]
fn a_skill_check_total_engine_really_exists() {
    let totals = allocate_skill_ranks(&load(FIGHTER_FIXTURE));
    assert!(
        !totals.totals.is_empty(),
        "allocate_skill_ranks must produce real per-skill totals"
    );
    assert!(
        !totals.class_skills.is_empty(),
        "SkillTotals must carry a real class-skill set"
    );
    // Deliberately NOT re-deriving the production formula here -- a check that
    // recomputes what it is checking proves nothing. What makes this a *total*
    // rather than a passthrough is that ranks and the class-skill bonus
    // actually move the number away from the bare ability modifier.
    let integrated = totals
        .totals
        .iter()
        .find(|(_, t)| t.ranks > 0 && t.total_modifier != t.ability_modifier);
    let (skill_id, total) = integrated.unwrap_or_else(|| {
        panic!(
            "at least one skill must show ranks folded into a total that differs from the bare \
             ability modifier, which is what makes this an integrated total: {:?}",
            totals.totals
        )
    });
    assert!(
        total.ranks > 0,
        "'{skill_id}' must carry real allocated ranks: {total:?}"
    );
    // Class-skill handling is real, not a stub: every bonus is either PF1's
    // +3 or absent, never an arbitrary number, and a skill carrying the +3 is
    // on the class-skill list this engine derived.
    for (id, t) in &totals.totals {
        assert!(
            t.class_skill_bonus == 0 || t.class_skill_bonus == 3,
            "'{id}' class-skill bonus must be PF1's +3 or nothing: {t:?}"
        );
        if t.class_skill_bonus == 3 {
            assert!(
                totals.class_skills.contains(id),
                "'{id}' got the class-skill bonus without being on the derived class-skill list"
            );
        }
    }
}

/// Executable proof of claim 2's falsity: a real integrated saving-throw total
/// exists, and it is not merely a copy of the base saves.
#[test]
fn a_saving_throw_total_engine_really_exists() {
    let computation = compute_pilot_base_chassis(&load(FIGHTER_FIXTURE));
    for suffix in ["fortitude", "reflex", "will"] {
        let id = format!("defense.total_save.{suffix}");
        assert!(
            computation.explanations.iter().any(|e| e.id == id),
            "expected a real integrated save total '{id}'"
        );
    }
    let totals = computation.total_saves;
    let base = computation.base_saves;
    assert!(
        totals.fortitude != 0 || totals.reflex != 0 || totals.will != 0,
        "total saves must be genuinely computed, not left at the default"
    );
    assert!(
        (totals.fortitude, totals.reflex, totals.will)
            != (base.fortitude, base.reflex, base.will)
            || computation.ability_modifiers.constitution == 0,
        "total saves must fold ability modifiers into the base saves"
    );
}

/// Executable proof of claim 3's falsity: a real integrated Armor Class total
/// exists (gated to the GE-06 equipment posture, which this fixture is).
#[test]
fn an_armor_class_total_engine_really_exists() {
    let computation = compute_pilot_base_chassis(&load(FIGHTER_FIXTURE));
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "defense.baseline_armor_class"),
        "expected the real integrated AC total 'defense.baseline_armor_class'"
    );
    assert!(
        computation.baseline_armor_class > 0,
        "the AC total must be genuinely computed, not left at 0"
    );
}

/// Claim 4 is the one that was NOT false. This pins that finding: nothing in
/// this engine is a speed total, so Monk Fast Movement's deferral on that
/// ground is accurate and must not be "corrected" away.
///
/// The moment a genuine speed total is built (scoping doc §3), this test fails
/// loudly and the Fast Movement text becomes stale for real — which is exactly
/// when it should be revisited.
#[test]
fn no_speed_total_engine_exists_yet_so_that_deferral_is_still_accurate() {
    for computation in postures() {
        for explanation in &computation.explanations {
            let id = &explanation.id;
            assert!(
                !(id.contains("speed") && (id.contains("total") || id.ends_with(".speed_total"))),
                "a speed total now exists ('{id}') — Monk Fast Movement's deferral text is now \
                 genuinely stale and must be revisited"
            );
        }
    }
}
