//! SD-36 Epic F3b (`docs/release/SD-36-consolidation/epic-f-class-completion.md` §5,
//! acceptance F3.4 + this RED file): the multiclass gate and fold generalised to every
//! class with a chassis.
//!
//! Every expected number below comes from the hand-worked PF1 sheet
//! `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f2-f3/f3b-hand-worked.md`
//! (written before this file first ran; book citations there). Presence is not
//! correctness: each mix asserts BAB, base and total saves, and HP totals, not just
//! `Computed`.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::class_census::load_sweep_fixture;
use codex::rules_core::class_seeds::canonical_seeds_for;
use codex::rules_core::pilot_compute::{
    BaseSaves, HeadlessReceiptStatus, PilotHeadlessReceipt, build_pilot_headless_receipt,
};

/// The census's own mix builder shape: the shared fixture, `class_levels` replaced
/// (first entry = the class taken at character level 1), each class's canonical
/// seeds merged.
fn mix(classes: &[(&str, u8)]) -> CharacterInput {
    let mut input = load_sweep_fixture().expect("shared fixture loads");
    input.case_id = Some(format!(
        "sd36_multiclass_any_class.{}",
        classes.iter().map(|(n, l)| format!("{n}{l}")).collect::<Vec<_>>().join(".")
    ));
    input.chosen.class_levels = classes
        .iter()
        .map(|(name, level)| CharacterClassLevel { class_id: format!("class:{name}"), level: *level })
        .collect();
    for (name, _) in classes {
        let (choices, spells) = canonical_seeds_for(name);
        input.chosen.selected_choices.extend(choices);
        input.chosen.spells_selected.extend(spells);
    }
    input
}

fn blocking(receipt: &PilotHeadlessReceipt) -> Vec<String> {
    receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .map(|d| format!("{}: {}", d.id, d.message))
        .collect()
}

fn explanation_value(receipt: &PilotHeadlessReceipt, id: &str) -> i16 {
    receipt
        .computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation {id}; got ids {:?}",
                receipt.computation.explanations.iter().map(|e| &e.id).collect::<Vec<_>>()
            )
        })
        .value
}

/// One hand-worked mix (see the hand-worked sheet for every figure's working).
struct Oracle {
    classes: &'static [(&'static str, u8)],
    bab: i16,
    base_saves: (i16, i16, i16),
    total_saves: (i16, i16, i16),
    hit_points: i16,
    /// The PF1 class-term skill points (hand-worked, `f3b-hand-worked.md`); asserted as
    /// the fold's printed total since F3b2 converted skill ranks per level.
    skill_points_pf1: i16,
    /// `None`: the mix reaches `Computed`. `Some((id, class))`: the ONLY claim-blocking
    /// diagnostic is `id`, naming `class` -- a named data remainder, never a silent pass.
    blocked_only_by: Option<(&'static str, &'static str)>,
}

const ORACLES: [Oracle; 5] = [
    Oracle {
        classes: &[("barbarian", 12), ("fighter", 1)],
        bab: 13,
        base_saves: (10, 4, 4),
        total_saves: (12, 6, 5),
        hit_points: 121,
        skill_points_pf1: 50,
        blocked_only_by: None,
    },
    Oracle {
        classes: &[("fighter", 6), ("arcane_archer", 3)],
        bab: 9,
        base_saves: (7, 4, 3),
        total_saves: (9, 6, 4),
        hit_points: 76,
        skill_points_pf1: 24,
        blocked_only_by: None,
    },
    Oracle {
        classes: &[("magus", 4), ("samurai", 2)],
        bab: 5,
        base_saves: (7, 2, 4),
        total_saves: (9, 4, 5),
        hit_points: 47,
        skill_points_pf1: 16,
        blocked_only_by: None,
    },
    Oracle {
        classes: &[("wizard", 5), ("loremaster", 2)],
        bab: 3,
        base_saves: (2, 2, 6),
        total_saves: (4, 4, 7),
        hit_points: 44,
        skill_points_pf1: 18,
        // Loremaster "gains no proficiency with any weapon or armor" (CRB p.385). Its
        // converted closure is attested complete since F3b2b: its one closure defect was
        // `SecretLore`, a variable no row of the pinned oracle declares, which the oracle
        // reads as 0 (`VariableProcessor.java:394-402`), so the proficiency reader answers
        // Known-empty and the mix reaches Computed (F3b2 and before: Blocked on
        // `combat.baseline_weapon_proficiency_unknown` naming class:loremaster).
        blocked_only_by: None,
    },
    // SD-36 F3c5 (`f3c5-hand-worked.md`, written first). Dragon Disciple "gains no
    // proficiency with any weapon or armor" (CRB p.380). Its converted closure is attested
    // complete since F3c5: its one closure defect was `Internal|Bite`
    // (`ce_abilities_race.lst:249`), a natural-attack helper that now converts as
    // `Fact::NaturalAttack("Bite")` on Dragon Bite, so the proficiency reader answers
    // Known-empty. F3c4 and before: Blocked on `combat.baseline_weapon_proficiency_unknown`
    // naming class:dragon_disciple.
    Oracle {
        classes: &[("sorcerer", 5), ("dragon_disciple", 3)],
        bab: 4,
        base_saves: (3, 3, 6),
        total_saves: (5, 5, 7),
        hit_points: 59,
        skill_points_pf1: 16,
        blocked_only_by: None,
    },
];

fn check(oracle: &Oracle) {
    let receipt = build_pilot_headless_receipt(&mix(oracle.classes));
    let c = &receipt.computation;
    match oracle.blocked_only_by {
        None => assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "{:?} must reach Computed; claim-blocking: {:#?}",
            oracle.classes,
            blocking(&receipt)
        ),
        Some((id, class)) => {
            assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked, "{:?}", oracle.classes);
            let blockers: Vec<_> = c.diagnostics.iter().filter(|d| d.claim_blocking).collect();
            assert!(
                !blockers.is_empty() && blockers.iter().all(|d| d.id == id && d.message.contains(class)),
                "{:?}: the only blocker must be {id} naming {class}: {:#?}",
                oracle.classes,
                blocking(&receipt)
            );
        }
    }
    assert_eq!(c.base_attack_bonus, oracle.bab, "{:?} BAB", oracle.classes);
    let (f, r, w) = oracle.base_saves;
    assert_eq!(c.base_saves, BaseSaves { fortitude: f, reflex: r, will: w }, "{:?} base saves", oracle.classes);
    let (f, r, w) = oracle.total_saves;
    assert_eq!(c.total_saves, BaseSaves { fortitude: f, reflex: r, will: w }, "{:?} total saves", oracle.classes);
    assert_eq!(explanation_value(&receipt, "multiclass.hit_points"), oracle.hit_points, "{:?} HP", oracle.classes);

    // Skill points (SD-36 F3b2): every class record now states its skill ranks per level
    // (converted from `STARTSKILLPTS`), so the fold prints the class total, and it must be
    // the hand-worked PF1 value written before F3b first ran (`f3b-hand-worked.md`).
    assert_eq!(
        explanation_value(&receipt, "multiclass.skill_points"),
        oracle.skill_points_pf1,
        "{:?} class skill points",
        oracle.classes
    );
    let unknown: Vec<_> =
        c.diagnostics.iter().filter(|d| d.id == "class_chassis.skill_points.unknown").collect();
    assert!(unknown.is_empty(), "{:?}: no class's skill points are Unknown now: {unknown:?}", oracle.classes);
}

#[test]
fn barbarian12_fighter1_computes_the_hand_worked_sheet() {
    check(&ORACLES[0]);
}

#[test]
fn fighter6_arcane_archer3_computes_the_hand_worked_sheet() {
    check(&ORACLES[1]);
}

#[test]
fn magus4_samurai2_computes_the_hand_worked_sheet() {
    check(&ORACLES[2]);
}

#[test]
fn wizard5_loremaster2_computes_the_hand_worked_sheet() {
    check(&ORACLES[3]);
}

#[test]
fn sorcerer5_dragon_disciple3_computes_the_hand_worked_sheet() {
    check(&ORACLES[4]);
}

/// A prestige class in a mix PRINTS its entry requirements (met or unmet), never
/// claim-blocks on them.
#[test]
fn a_prestige_class_in_a_mix_prints_its_entry_requirements_without_blocking() {
    for (classes, prestige) in [
        (&[("fighter", 6), ("arcane_archer", 3)][..], "class:arcane_archer"),
        (&[("wizard", 5), ("loremaster", 2)][..], "class:loremaster"),
    ] {
        let receipt = build_pilot_headless_receipt(&mix(classes));
        let printed: Vec<_> = receipt
            .computation
            .diagnostics
            .iter()
            .filter(|d| d.id.starts_with("multiclass.prestige_entry_gate.") && d.message.contains(prestige))
            .collect();
        assert_eq!(printed.len(), 1, "{classes:?}: {:?}", receipt.computation.diagnostics);
        assert!(!printed[0].claim_blocking, "{classes:?}: entry requirements print, never block");
    }
}

/// Each class's own isolated-run feature lines come through verbatim, re-scoped
/// `multiclass.<class>.<original id>`, one set per class.
#[test]
fn each_class_keeps_its_own_feature_lines_rescoped_by_class() {
    let receipt = build_pilot_headless_receipt(&mix(&[("barbarian", 12), ("fighter", 1)]));
    let ids: Vec<&str> = receipt.computation.explanations.iter().map(|e| e.id.as_str()).collect();
    for class in ["barbarian", "fighter"] {
        assert!(
            ids.iter().any(|id| id.starts_with(&format!("multiclass.{class}."))),
            "no multiclass.{class}.* line: {ids:?}"
        );
    }
}

/// Negative control: a mix of prestige classes only is not a legal character (F2's
/// rule); it states the game rule.
#[test]
fn a_mix_of_prestige_classes_only_states_the_game_rule() {
    let receipt = build_pilot_headless_receipt(&mix(&[("arcane_archer", 3), ("loremaster", 2)]));
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
    assert!(
        receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking && d.id == "prestige_class.requires_base_class_levels"),
        "{:?}",
        receipt.computation.diagnostics
    );
}

/// Negative control: a class whose save progression is Unrecognized (Sentinel's
/// converted Fortitude `classlevel+1/2`, F3a's named list) stays Blocked in a mix with
/// a NAMED diagnostic -- never folded in as a poor save.
#[test]
fn an_unrecognized_save_shape_blocks_the_mix_by_name() {
    let receipt = build_pilot_headless_receipt(&mix(&[("fighter", 6), ("sentinel", 1)]));
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);
    let named = receipt
        .computation
        .diagnostics
        .iter()
        .find(|d| d.id == "multiclass.save_shape.unrecognized")
        .unwrap_or_else(|| panic!("{:?}", receipt.computation.diagnostics));
    assert!(named.claim_blocking);
    assert!(named.message.contains("class:sentinel") && named.message.contains("Fortitude"), "{}", named.message);
}

/// Weapon proficiency is a union: one class that grants the weapon decides it, whatever
/// another class's answer. Fighter grants every martial weapon (CRB p.55). Dragon Disciple
/// served as the class with no answer until F3c5 attested its closure (its `Internal|Bite`
/// helper now converts as a natural-attack fact); Diabolist serves now: its closure names
/// `Special Ability|Hunter's Bond ~ Companion`, a KEY no row of the pinned oracle declares
/// (`reader-remainder.md` G-U), so the reader has no answer for it. (Loremaster served here
/// until F3b2b.)
#[test]
fn a_class_with_no_proficiency_answer_cannot_undo_another_class_s_grant() {
    let receipt = build_pilot_headless_receipt(&mix(&[("fighter", 6), ("diabolist", 2)]));
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.id == "combat.baseline_weapon_proficiency_unknown"),
        "{:#?}",
        blocking(&receipt)
    );
    assert_eq!(receipt.status, HeadlessReceiptStatus::Computed, "{:#?}", blocking(&receipt));
}

/// SD-36 F3b3: a class alone prints the same class skill-point term the fold sums, so a class
/// alone and the same class in a mix print one number. Hand-worked (`f3b-hand-worked.md`,
/// CRB p.30; Int 10, +0): Barbarian 12 x 4 = 48; Fighter 1 x 2 = 2 (the mix's 50 = 48 + 2);
/// Wizard 5 x 2 = 10 (CRB p.77); Rogue 3 x 8 = 24 (CRB p.67).
#[test]
fn a_class_alone_prints_the_class_skill_points_term_the_fold_sums() {
    for (class, level, expected) in [("barbarian", 12, 48), ("fighter", 1, 2), ("wizard", 5, 10), ("rogue", 3, 24)] {
        let receipt = build_pilot_headless_receipt(&mix(&[(class, level)]));
        assert_eq!(explanation_value(&receipt, "class_chassis.skill_points"), expected, "{class} {level}");
        assert!(
            !receipt.computation.diagnostics.iter().any(|d| d.id == "class_chassis.skill_points.unknown"),
            "{class} {level}: {:?}",
            receipt.computation.diagnostics
        );
    }
    // The mix does not copy the per-class line; the fold prints the character total once.
    let receipt = build_pilot_headless_receipt(&mix(&[("barbarian", 12), ("fighter", 1)]));
    assert!(
        !receipt.computation.explanations.iter().any(|e| e.id.ends_with(".class_chassis.skill_points")),
        "a mix prints multiclass.skill_points, not per-class copies"
    );
}

/// `class_chassis.skill_points.unknown` fires only for a class whose converted record still
/// lacks a chassis with the skill-ranks row, and names it: over every census id whose class alone
/// passes the chassis gate at level 1 (the population the line is written for), the Unknown set
/// is exactly the named list below.
#[test]
fn skill_points_unknown_fires_only_for_a_class_still_lacking_the_row() {
    use codex::rules_core::class_census::census;
    let mut printed = 0usize;
    let mut unknown = Vec::new();
    let mut population = 0usize;
    for entry in census().values() {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let receipt = build_pilot_headless_receipt(&mix(&[(slug, 1)]));
        let has_line = receipt.computation.explanations.iter().any(|e| e.id == "class_chassis.skill_points");
        let named = receipt
            .computation
            .diagnostics
            .iter()
            .find(|d| d.id == "class_chassis.skill_points.unknown")
            .map(|d| d.message.clone());
        assert!(!(has_line && named.is_some()), "{}: both a total and an Unknown", entry.class_id);
        if has_line || named.is_some() {
            population += 1;
        }
        if has_line {
            printed += 1;
        }
        if let Some(message) = named {
            assert!(message.contains(&entry.class_id) || message.contains(slug), "{message}");
            unknown.push(entry.class_id.clone());
        }
    }
    println!("census ids with the line or its Unknown at level 1: {population}; printed {printed}; Unknown {}: {unknown:?}", unknown.len());
    assert_eq!(unknown, EXPECTED_SKILL_POINTS_UNKNOWN, "the named remainder moved");
}

/// Census ids whose class alone reaches the line and whose converted record (or, for a
/// class-selection class, its base class's) states no skill ranks per level. Empty since F3b3:
/// Monk's degraded CRB principal still states `Skill ranks per level` (4), and the four Pathfinder
/// Unchained class selections are taken on a base class whose row states it. The positive control
/// (a record with no row answers Unknown, by name) is `multiclass_fold::tests::
/// a_class_whose_record_states_no_skill_ranks_is_named_unknown`.
const EXPECTED_SKILL_POINTS_UNKNOWN: &[&str] = &[];
