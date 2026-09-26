//! SD-36 Epic F3 polish (`docs/release/SD-36-consolidation/artifacts/epic-f/stage-f4-f5/
//! f2f3-polish.json`, P1-P5; receipt `f3p-receipt.md` in the same directory).
//!
//! Hand-worked from the book first (CRB page cited on each test). Every expected number is PF1's,
//! never the engine's own earlier output.

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::class_census::{load_sweep_fixture, sheet_dump_with_rules_text};
use codex::rules_core::class_seeds::canonical_seeds_for;
use codex::rules_core::pilot_compute::{PilotHeadlessReceipt, build_pilot_headless_receipt};
use codex::rules_core::sheet_rule::{CharacterFacts, HeldSeed, held_set};
use codex::rules_core::sheet_rule_package;

/// The census's mix builder shape (same as `sd36_multiclass_any_class.rs`).
fn mix(classes: &[(&str, u8)]) -> CharacterInput {
    let mut input = load_sweep_fixture().expect("shared fixture loads");
    input.case_id = Some(format!(
        "sd36_f3_polish.{}",
        classes.iter().map(|(n, l)| format!("{n}{l}")).collect::<Vec<_>>().join(".")
    ));
    input.chosen.class_levels = classes
        .iter()
        .map(|(name, level)| CharacterClassLevel { class_id: format!("class:{name}"), level: *level })
        .collect();
    for (name, level) in classes {
        let (choices, spells) = canonical_seeds_for(name, *level);
        input.chosen.selected_choices.extend(choices);
        input.chosen.spells_selected.extend(spells);
    }
    input
}

fn explanation<'a>(receipt: &'a PilotHeadlessReceipt, id: &str) -> &'a codex::rules_core::pilot_compute::ComputationExplanation {
    receipt.computation.explanations.iter().find(|e| e.id == id).unwrap_or_else(|| {
        panic!(
            "expected explanation {id}; got ids {:?}",
            receipt.computation.explanations.iter().map(|e| &e.id).collect::<Vec<_>>()
        )
    })
}

// ---- P1: a class line that reads the character's BAB, in a mix -----------------------------

/// CRB p.57 (Flurry of Blows): "the monk's base attack bonus from monk levels is equal to her
/// monk level" when flurrying, and "for the purpose of these attacks, the monk's base attack
/// bonus from other classes" adds; the flurry takes -2. Monk 4 + Rogue 3 (Rogue 3 BAB +2, CRB
/// p.67): 4 + 2 - 2 = **+4**. Monk 4 + Fighter 4 (Fighter 4 BAB +4, CRB p.55): 4 + 4 - 2 = **+6**.
/// Monk 4 alone: 4 - 2 = **+2** (unchanged).
#[test]
fn p1_flurry_attack_bonus_in_a_mix_adds_the_other_classes_bab() {
    for (classes, expected) in [
        (&[("monk", 4), ("rogue", 3)][..], 4),
        (&[("monk", 4), ("fighter", 4)][..], 6),
        (&[("rogue", 3), ("monk", 4)][..], 4),
    ] {
        let receipt = build_pilot_headless_receipt(&mix(classes));
        let line = explanation(&receipt, "multiclass.monk.class_chassis.monk.flurry_of_blows_attack_bonus");
        assert_eq!(line.value, expected, "{classes:?}: {}", line.detail);
    }
    let alone = build_pilot_headless_receipt(&mix(&[("monk", 4)]));
    assert_eq!(explanation(&alone, "class_chassis.monk.flurry_of_blows_attack_bonus").value, 2);
}

// ---- P2: the base-save explanation lists save terms ------------------------------------------

/// CRB p.30 / p.55 / p.31: Barbarian 12 good Fortitude 12/2 + 2 = 8, poor Reflex 12/3 = 4;
/// Fighter 1 good Fortitude 1/2 + 2 = 2.5, poor Reflex 1/3 = 0.333. The detail names each class's
/// own save term, never its base attack bonus.
#[test]
fn p2_multiclass_save_explanation_lists_each_class_s_save_term() {
    let receipt = build_pilot_headless_receipt(&mix(&[("barbarian", 12), ("fighter", 1)]));
    for (save, terms) in [
        ("fortitude", ["class:barbarian 12: Fortitude 8", "class:fighter 1: Fortitude 2.5"]),
        ("reflex", ["class:barbarian 12: Reflex 4", "class:fighter 1: Reflex 0.333"]),
        ("will", ["class:barbarian 12: Will 4", "class:fighter 1: Will 0.333"]),
    ] {
        let detail = &explanation(&receipt, &format!("class_chassis.base_save.{save}")).detail;
        assert!(!detail.contains("base attack bonus"), "{save}: {detail}");
        for term in terms {
            assert!(detail.contains(term), "{save}: missing {term:?} in {detail}");
        }
    }
}

// ---- P4: the hit-point Unknown names what is actually missing ---------------------------------

/// Monk has no converted BAB/save chassis record (its CRB principal states `Hit die`, so "no
/// record states this class's hit points" is false). The Unknown names the missing chassis.
#[test]
fn p4_hit_points_unknown_names_the_missing_chassis_record() {
    let receipt = build_pilot_headless_receipt(&mix(&[("monk", 4), ("rogue", 3)]));
    let unknown: Vec<_> = receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.id == "class_chassis.hit_points.unknown" && d.message.contains("class:monk"))
        .collect();
    assert_eq!(unknown.len(), 1, "{:#?}", receipt.computation.diagnostics);
    let message = &unknown[0].message;
    assert!(!message.contains("no converted class record states"), "{message}");
    assert!(message.contains("chassis"), "{message}");
}

// ---- P5: an option-gated grant needs the option -----------------------------------------------

const FIREARMS: &str = "ultimate_combat:class_feature:exotic_weapon_proficiency_firearms";
const EWP: &str = "core_rulebook:feat:exotic_weapon_proficiency";

/// UC p.18 (Samurai): proficient with the katana (Exotic Weapon Proficiency (Katana)), never with
/// firearms. The oracle's firearms grant is `PREABILITY:1,CATEGORY=FEAT,Exotic Weapon Proficiency
/// (Firearms)` (`uc_feats.lst:345`): the feat WITH the Firearms option.
#[test]
fn p5_samurai_alone_prints_no_firearms_line() {
    for build in [&[("samurai".to_owned(), 1u8)][..], &[("magus".to_owned(), 4), ("samurai".to_owned(), 2)][..]] {
        let fixture = load_sweep_fixture().expect("fixture");
        let dump = sheet_dump_with_rules_text(&fixture, build, None);
        assert!(!dump.contains(&format!("HELD| {FIREARMS}\n")), "{build:?} holds the firearms record");
        assert!(!dump.contains("Exotic Weapon Proficiency ~ Firearms"), "{build:?} prints a firearms line");
    }
}

/// Positive and negative control on the held set: Exotic Weapon Proficiency with the Firearms
/// option holds the firearms record; with the Katana option it does not.
#[test]
fn p5_the_firearms_option_pick_holds_the_firearms_record_and_another_option_does_not() {
    let package = sheet_rule_package::package().as_ref().expect("the converted package loads");
    let held = |option: &str| {
        let mut facts = CharacterFacts { level: 1, race: Some("human".into()), class_levels: vec![("fighter".into(), 1)], base_attack: 1, ..CharacterFacts::default() };
        facts.choices.insert(EWP.into(), vec![(option.into(), option.into())]);
        let seed = HeldSeed { race: Some("human".into()), classes: vec![("fighter".into(), 1)], feats: vec!["exotic_weapon_proficiency".into()], ..HeldSeed::default() };
        let set = held_set(package, &seed, &facts);
        set.holds(EWP) && set.holds(FIREARMS)
    };
    assert!(held("firearms"), "EWP (Firearms) must hold the firearms proficiency record");
    assert!(!held("katana"), "EWP (Katana) must not hold the firearms proficiency record");
}

/// The pick path: a feat the character records WITH its option -- the catalog picker's
/// `"Exotic Weapon Proficiency (Firearms)"` -- holds the feat and records the option under the
/// feat's chooser (`sheet_rule::feat_sub_choices`, `CharacterFacts::with_linked_picks`), so the
/// firearms record is held for that character and not for one whose option is Katana. The same
/// facts every sheet path builds (`class_census::sheet_dump_with_rules_text`, the desktop).
#[test]
fn p5_a_feat_recorded_with_its_option_holds_exactly_that_option_s_records() {
    let package = sheet_rule_package::package().as_ref().expect("the converted package loads");
    let held = |feat: &str| {
        let mut input = mix(&[("fighter", 4)]);
        input.chosen.selected_feats.push(feat.to_owned());
        let receipt = build_pilot_headless_receipt(&input);
        let seed = HeldSeed::from_character(&input, &receipt.computation);
        let facts = CharacterFacts::from_character(&input, &receipt.computation).with_linked_picks(package, &seed);
        let set = held_set(package, &seed, &facts);
        (set.holds(EWP), set.holds(FIREARMS), facts.choices.get(EWP).cloned().unwrap_or_default())
    };
    let (ewp, firearms, picks) = held("Exotic Weapon Proficiency (Firearms)");
    assert!(ewp && firearms, "EWP (Firearms): feat held {ewp}, firearms held {firearms}, picks {picks:?}");
    assert_eq!(picks, vec![("firearms".to_owned(), "Firearms".to_owned())]);
    let (ewp, firearms, picks) = held("Exotic Weapon Proficiency (Katana)");
    assert!(ewp && !firearms, "EWP (Katana): feat held {ewp}, firearms held {firearms}, picks {picks:?}");
}

/// P1 scan (denominator: every class line -- `class_feature.*`, `class_spell.*`,
/// `class_chassis.*` other than the character totals the fold computes once -- that any census id
/// prints alone at any level 1..=its max level). A line whose detail speaks of a base attack
/// bonus is classified, and the classification is pinned (a new BAB-reading line fails here until
/// it is classified):
/// - [`READS_CHARACTER_BAB`]: its value is the character's BAB with its class's own levels
///   substituted, plus a constant -- the fold adds the other classes' BAB
///   (`multiclass_fold::LINES_READING_CHARACTER_BAB`, which must equal this list);
/// - [`READS_CHARACTER_BAB_CLASS_CANNOT_JOIN_A_MIX`]: reads the character's BAB, but its class has
///   no chassis a mix can fold (asserted below), so the fold never copies it;
/// - [`READS_OWN_CLASS_ONLY`]: reads only its own class's levels (a per-class BAB line, a
///   companion's own HD, a delta on top of the class's own BAB, or the words only mention a BAB).
#[test]
fn p1_scan_every_class_line_that_speaks_of_a_base_attack_bonus_is_classified() {
    use codex::rules_core::class_census::census;
    use std::collections::{BTreeMap, BTreeSet};
    let builds: Vec<(String, u8)> = census()
        .values()
        .flat_map(|entry| {
            let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id).to_owned();
            (1..=entry.max_level).map(move |level| (slug.clone(), level))
        })
        .collect();
    let chunks: Vec<&[(String, u8)]> = builds.chunks(builds.len().div_ceil(8)).collect();
    let results: Vec<(BTreeSet<String>, BTreeMap<String, String>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| {
                scope.spawn(move || {
                    let mut all_lines = BTreeSet::new();
                    let mut bab_lines = BTreeMap::new();
                    for (slug, level) in chunk.iter() {
                        let receipt = build_pilot_headless_receipt(&mix(&[(slug.as_str(), *level)]));
                        for e in &receipt.computation.explanations {
                            let class_line = (e.id.starts_with("class_feature.")
                                || e.id.starts_with("class_spell.")
                                || e.id.starts_with("class_chassis."))
                                && e.id != "class_chassis.base_attack_bonus"
                                && e.id != "class_chassis.skill_points"
                                && !e.id.starts_with("class_chassis.base_save.")
                                && !e.id.ends_with(".level_1_hit_points");
                            if !class_line {
                                continue;
                            }
                            all_lines.insert(e.id.clone());
                            if e.detail.to_ascii_lowercase().contains("base attack bonus") {
                                bab_lines.entry(e.id.clone()).or_insert_with(|| format!("{slug} {level}: {}", e.detail));
                            }
                        }
                    }
                    (all_lines, bab_lines)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("scan thread")).collect()
    });
    let mut all_lines: BTreeSet<String> = BTreeSet::new();
    let mut bab_lines: BTreeMap<String, String> = BTreeMap::new();
    for (lines, bab) in results {
        all_lines.extend(lines);
        for (id, example) in bab {
            bab_lines.entry(id).or_insert(example);
        }
    }
    println!(
        "P1 scan: {} single-class builds; {} distinct class-line ids; {} speak of a base attack bonus",
        builds.len(),
        all_lines.len(),
        bab_lines.len()
    );
    for (id, example) in &bab_lines {
        println!("  {id} -- {}", &example[..example.len().min(200)]);
    }
    let classified: BTreeSet<&str> = READS_CHARACTER_BAB
        .iter()
        .chain(READS_CHARACTER_BAB_CLASS_CANNOT_JOIN_A_MIX)
        .chain(READS_OWN_CLASS_ONLY)
        .copied()
        .collect();
    let found: BTreeSet<&str> = bab_lines.keys().map(String::as_str).collect();
    assert_eq!(found, classified, "a class line speaking of a base attack bonus is unclassified (or a pinned one vanished)");

    // The class of a READS_CHARACTER_BAB_CLASS_CANNOT_JOIN_A_MIX line is refused by the mix gate,
    // by name, so the fold never copies the line.
    let receipt = build_pilot_headless_receipt(&mix(&[("unchained_monk", 6), ("fighter", 2)]));
    assert!(
        receipt.computation.diagnostics.iter().any(|d| d.claim_blocking
            && d.id.starts_with("multiclass.")
            && d.message.contains("class:unchained_monk")),
        "{:#?}",
        receipt.computation.diagnostics
    );
    assert!(
        !receipt.computation.explanations.iter().any(|e| e.id.starts_with("multiclass.unchained_monk.")),
        "the fold must not copy an unchained monk line"
    );
}

/// Reads the character's BAB with its class's levels substituted (the fold adds the other
/// classes' BAB). CRB p.57.
const READS_CHARACTER_BAB: &[&str] = &["class_chassis.monk.flurry_of_blows_attack_bonus"];
/// Reads the character's BAB (the line's own count steps at total BAB 6/11/16), but Unchained
/// Monk has no converted chassis a mix can fold.
const READS_CHARACTER_BAB_CLASS_CANNOT_JOIN_A_MIX: &[&str] = &["class_feature.pu.unchained_monk.flurry_attack_count"];
/// Reads only its own class's levels: unchanged by another class in a mix.
const READS_OWN_CLASS_ONLY: &[&str] = &[
    // Each class's own BAB term (the fold sums these).
    "class_chassis.barbarian.base_attack_bonus",
    "class_chassis.bard.base_attack_bonus",
    "class_chassis.cleric.base_attack_bonus",
    "class_chassis.druid.base_attack_bonus",
    "class_chassis.monk.base_attack_bonus",
    "class_chassis.paladin.base_attack_bonus",
    "class_chassis.ranger.base_attack_bonus",
    "class_chassis.rogue.base_attack_bonus",
    "class_chassis.sorcerer.base_attack_bonus",
    "class_chassis.wizard.base_attack_bonus",
    // A companion's own BAB from its own HD, which its master's class level sets (CRB p.52).
    "class_chassis.cavalier.mount.base_attack_bonus",
    "class_chassis.druid.animal_companion.base_attack_bonus",
    "class_chassis.hunter.animal_companion.base_attack_bonus",
    // The eidolon's BAB equals the summoner's level (APG p.56).
    "class_feature.apg.summoner.eidolon.base_attack_bonus",
    // Maneuver Training: the DELTA monk level - monk's own 3/4 BAB on top of the BAB already
    // counted (CRB p.58); another class's BAB is in the counted BAB, not the delta.
    "class_chassis.monk.maneuver_training_cmb_bonus",
    // The words mention a BAB; the value is a claw damage die.
    "class_feature.apg.alchemist.discovery.feral_mutagen_claw_damage_die",
    // The words describe the chassis; the value is a deferral marker.
    "class_feature.pu.unchained_monk.other_features_deferred.unsupported",
];
