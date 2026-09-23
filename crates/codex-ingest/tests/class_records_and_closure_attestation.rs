//! SD-36 Epic F1c-3 (defects D3 and D4, `epic-f-class-completion.md` §3 F1 / §3.4).
//!
//! - D3: every class the census registry knows (`codex::rules_core::class_census::census()`) has a
//!   converted class principal rule (`package.find("class", <slug>)`). The four Pathfinder
//!   Unchained classes had none: PCGen declares no `CLASS:` object for them -- each is a
//!   `CATEGORY:CLASS` selection ability taken on its base class (`pu_abilities_class.lst:114-117`),
//!   so the converter filed the unit as a `class_feature` and wrote no `class` record at all.
//! - D4: the converter's `closure_complete` attestation on the class principal, read against the
//!   real package.

use codex::rules_core::class_census::census;
use codex::rules_core::sheet_rule_package;

/// D3. The miss list must be empty: no enumerated exemptions remain.
#[test]
fn every_census_class_has_a_converted_class_record() {
    let package = sheet_rule_package::package().as_ref().expect("package loads");
    let ids: Vec<String> = census().into_keys().collect();
    assert_eq!(ids.len(), 135, "census carries 135 class ids");
    let misses: Vec<&String> = ids
        .iter()
        .filter(|id| package.find("class", id.strip_prefix("class:").unwrap_or(id)).is_none())
        .collect();
    assert!(misses.is_empty(), "{} of {} census classes have no converted class record: {misses:?}", misses.len(), ids.len());
}

/// D3: a class-selection class is taken on its base class. The Unchained Monk's principal names
/// the Monk as its base (`Effect::TakenOnClass`), so the class-level walk holds the Monk's own
/// class line (the `Monk_CFP_Level` its features are gated on) and the Unchained selection, and
/// NOT the standard Monk's class features (`Monk ~ Standard Class` is shut by the
/// `AltMonkChoice`-tagged selection, `cr_abilities_class.lst:103`).
#[test]
fn an_unchained_class_is_taken_on_its_base_class() {
    use codex::rules_core::sheet_rule::{held_set, CharacterFacts, Effect, HeldSeed};
    let package = sheet_rule_package::package().as_ref().expect("package loads");
    for (class, base) in [("unchained_barbarian", "barbarian"), ("unchained_monk", "monk"), ("unchained_rogue", "rogue"), ("unchained_summoner", "summoner")] {
        let id = package.find("class", class).unwrap_or_else(|| panic!("{class} has a class record"));
        let principal = package.rule(id).expect("principal");
        assert!(principal.grants.contains(&Effect::TakenOnClass(base.to_string())), "{class}: {:?}", principal.grants);
        assert_eq!(package.base_class_of(class), Some(base));
    }
    let seed = HeldSeed { classes: vec![("unchained_monk".to_string(), 1)], ..HeldSeed::default() };
    let facts = CharacterFacts { level: 1, class_levels: vec![("unchained_monk".to_string(), 1)], ..CharacterFacts::default() };
    let held = held_set(package, &seed, &facts);
    for id in [
        "pathfinder_unchained:class:unchained_monk",
        "core_rulebook:class:monk",
        "core_rulebook:class_feature:monk_class",
        "pathfinder_unchained:class_feature:monk_unchained_class",
        "pathfinder_unchained:class_feature:unchained_monk_weapon_and_armor_proficiency",
    ] {
        assert!(held.rules.contains_key(id), "{id} must be held at unchained monk 1");
    }
    assert!(!held.rules.contains_key("core_rulebook:class_feature:monk_standard_class"), "the standard monk is shut by the selection");
}

/// D3 companion: PCGen's `PREABILITY` `[<key>]` item EXCLUDES that ability; it is never an
/// alternative. The Unchained Monk selection's gate (`pu_abilities_class.lst:115`,
/// `!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkArchetype,[Archetype Monk]`) names no missing
/// record once converted.
#[test]
fn a_preability_bracket_item_is_an_exclusion() {
    let package = sheet_rule_package::package().as_ref().expect("package loads");
    let rule = package.rule("pathfinder_unchained:class_feature:monk_unchained_class").expect("selection");
    let text = serde_json::to_string(&rule.applies).unwrap();
    assert!(!text.contains("MissingRule"), "{text}");
    assert!(text.contains("core_rulebook:class_feature:archetype_monk"), "the excluded rule is subtracted: {text}");
}

/// D4 on the real package: the attestation is written, is false on a closure that carries a
/// defect, and a class it holds for with no weapon grant reads Known and empty.
#[test]
fn the_real_package_attests_complete_closures_only() {
    use codex::rules_core::pilot_compute::class_proficiency_sheet_rules::{class_weapon_proficiency_view, ProficiencyAnswer};
    let package = sheet_rule_package::package().as_ref().expect("package loads");
    let principals: Vec<_> = package.rules_of_kind("class").filter(|r| !r.id.contains('#')).collect();
    let attested = principals.iter().filter(|r| r.closure_complete).count();
    println!("class principals attested closure-complete: {attested} of {}", principals.len());
    assert!(attested > 0 && attested < principals.len(), "{attested} of {}", principals.len());
    // Aldori Swordlord's own level lines name ten class features the corpus does not carry.
    let aldori = package.rule(package.find("class", "aldori_swordlord").expect("aldori")).expect("rule");
    assert!(!aldori.closure_complete);
    assert!(matches!(class_weapon_proficiency_view("aldori_swordlord", 1), ProficiencyAnswer::Unknown { .. }));
    // Divine Scion: "gains no additional weapon or armor proficiencies" (`ism_abilities_class.lst:29`).
    let scion = package.rule(package.find("class", "divine_scion").expect("scion")).expect("rule");
    assert!(scion.closure_complete);
    match class_weapon_proficiency_view("divine_scion", 1) {
        ProficiencyAnswer::Known(view) => assert!(view.tiers.is_empty() && view.named.is_empty() && view.sets.is_empty(), "{view:?}"),
        unknown => panic!("an attested closure with no weapon grant is Known(empty): {unknown:?}"),
    }
}

/// D6 on the real package: the Commoner's pick is linked to the one member of its pool, whose
/// weapon options are resolved at ingest to the oracle's Simple-tier weapon-proficiency names.
#[test]
fn the_commoner_pick_is_linked_to_the_simple_tier() {
    use codex::rules_core::rules_tables::crb::weapon_tables::{WeaponProficiency, WEAPON_TABLE};
    use codex::rules_core::sheet_rule::{Choice, OptionSet};
    let package = sheet_rule_package::package().as_ref().expect("package loads");
    let pick = package.rule("core_rulebook:class_feature:weapon_and_armor_proficiency_commoner").expect("pick");
    match &pick.offers {
        Some(Choice { from: OptionSet::Rules { pool, tags, .. }, .. }) => {
            assert_eq!(pool, "special_ability");
            assert_eq!(tags, &vec!["SingleSimpleWeaponProficiency".to_string()]);
        }
        other => panic!("the pick must be linked to its pool's members: {other:?}"),
    }
    let member = package.rule("core_rulebook:class_feature:single_simple_weapon_proficiency").expect("member");
    let Some(Choice { from: OptionSet::Weapons(options), .. }) = &member.offers else { panic!("{:?}", member.offers) };
    // Every Simple CRB weapon is offered, and no Martial or Exotic one is (a table row with no
    // tier states nothing either way).
    for entry in WEAPON_TABLE {
        let Some(name) = entry.proficiency_name else { continue };
        let offered = options.iter().any(|o| o.eq_ignore_ascii_case(name));
        match entry.proficiency {
            Some(WeaponProficiency::Simple) => assert!(offered, "Simple {name} must be offered"),
            Some(tier) => assert!(!offered, "{name} ({tier:?}) is offered but is not Simple"),
            None => {}
        }
    }
}
