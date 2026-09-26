//! SD-36 Epic F4pre (FS-21): selection pools with no converted pick option become converted
//! CHOICES whose options carry their own grants, so the SD-32 generic pool-group pass yields to
//! the held-set path and each member line prints at the level the record states.
//!
//! The oracle hands out a pick among a category's objects by two tokens, and the converter now
//! writes both as a choice (`pool_link.rs`, one rule):
//!
//! - `BONUS:ABILITYPOOL|<C>|<n>` into a child ability category with a `TYPE:` filter
//!   (`acg_abilities_class.lst:1386`, `Shaman ~ Spirit`: `BONUS:ABILITYPOOL|Shaman Spirit|1`;
//!   `acg_abilitycategories.lst:100`, `Shaman Spirit` = `CATEGORY:Special Ability
//!   TYPE:ShamanSpirit`) -> `offers: Rules { pool: special_ability, tags: [ShamanSpirit] }`;
//! - `BONUS:DOMAIN|NUMBER|<n>` (`cr_classes.lst:55`, `CLASS:Cleric ...
//!   BONUS:DOMAIN|NUMBER|ClericDomainCount`, `BONUS:VAR|ClericDomainCount|2`) ->
//!   `offers: Domains`.
//!
//! The engine then holds an option recorded under a choice the character holds when the choice's
//! option set selects it, and links a Path-A pick (`choice:cleric_domain -> domain:air`,
//! `choice:shaman_spirit -> spirit:battle`) to the option it names.
//!
//! Hand-worked (the books):
//!
//! - Air domain (CRB p.41): Lightning Arc (Sp) at 1st; Electricity Resistance (Ex) 10 at 6th.
//!   A cleric 5 holds Lightning Arc and not Electricity Resistance; a cleric 6 holds both.
//! - Battle spirit (ACG p.35 Table 1-8 / p.37): the spirit ability (Battle Spirit) at 1st, the
//!   greater spirit ability (Enemies' Bane) at 8th, the true spirit ability (Paragon of Battle) at
//!   16th. A shaman 5 holds Battle Spirit only; a shaman 8 adds Enemies' Bane; a shaman 16 adds
//!   Paragon of Battle.

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::class_census::load_sweep_fixture;
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::sheet_rule::{held_set, CharacterFacts, Choice, Expr, HeldSeed, HeldSet, OptionSet};
use codex::rules_core::sheet_rule_package;

fn with_pick(class: &str, level: u8, choice: &str, selection: &str) -> CharacterInput {
    let fixture = load_sweep_fixture().expect("fixture");
    let mut input = input_for(&fixture, class, level);
    input.chosen.selected_choices.retain(|c| c.choice_set_id != choice);
    input.chosen.selected_choices.push(codex::rules_core::character_input::SelectedChoice {
        choice_set_id: choice.to_owned(),
        selection_id: selection.to_owned(),
    });
    input
}

/// The held set the sheet builds for a single-class character with one Path-A pick, the same
/// construction `sheet_rule_package::linked_picks` uses.
fn held_for(class: &str, level: u8, choice: &str, selection: &str) -> HeldSet {
    let package = sheet_rule_package::package().as_ref().expect("package");
    let seed = HeldSeed { race: Some("human".into()), classes: vec![(class.into(), i64::from(level))], ..HeldSeed::default() };
    let mut facts = CharacterFacts {
        level: i64::from(level),
        race: Some("human".into()),
        class_levels: vec![(class.into(), i64::from(level))],
        ..CharacterFacts::default()
    };
    facts.record_pick(choice, selection);
    let facts = facts.with_linked_picks(package, &seed);
    held_set(package, &seed, &facts)
}

fn generic_values(input: &CharacterInput, prefix: &str) -> Vec<(String, i16)> {
    build_pilot_headless_receipt(input)
        .computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(prefix))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

#[test]
fn the_domain_count_and_the_spirit_pool_convert_as_choices() {
    let package = sheet_rule_package::package().as_ref().expect("package");
    let cleric = package.rule("core_rulebook:class:cleric#bonus1").expect("the cleric's domain-count line");
    let Some(Choice { id, count, from: OptionSet::Domains }) = &cleric.offers else { panic!("cleric domain count offers {:?}", cleric.offers) };
    assert_eq!(id, "core_rulebook:class:cleric#bonus1");
    assert!(matches!(count, Expr::Var(_)), "the count is ClericDomainCount: {count:?}");
    let shaman = package.rule("advanced_class_guide:class_feature:shaman_spirit").expect("Shaman ~ Spirit");
    let Some(Choice { id, from: OptionSet::Rules { pool, tags, .. }, .. }) = &shaman.offers else { panic!("shaman spirit offers {:?}", shaman.offers) };
    assert_eq!(id, "advanced_class_guide:class_feature:shaman_spirit");
    assert_eq!(pool, "special_ability");
    assert_eq!(tags, &vec!["ShamanSpirit".to_string()]);
}

#[test]
fn a_cleric_domain_pick_links_to_the_domain_choice_and_holds_the_domain() {
    let links = sheet_rule_package::linked_picks(&with_pick("cleric", 5, "choice:cleric_domain", "domain:air"));
    let link = links.iter().find(|l| l.choice_set_id == "choice:cleric_domain").unwrap_or_else(|| panic!("no link: {links:#?}"));
    assert_eq!(link.member, "air");
    assert_eq!(link.chooser, "core_rulebook:class:cleric#bonus1");
    assert_eq!(link.option, "core_rulebook:domain:air");
    assert!(link.option_held, "{link:?}");
}

#[test]
fn an_air_cleric_holds_each_domain_power_at_the_level_the_book_grants_it() {
    let arc = "core_rulebook:class_feature:domain_power_lightning_arc";
    let resist = "core_rulebook:class_feature:domain_power_electricity_resistance";
    let at5 = held_for("cleric", 5, "choice:cleric_domain", "domain:air");
    assert!(at5.holds("core_rulebook:domain:air"));
    assert!(at5.holds(arc), "Lightning Arc is a 1st-level power");
    assert!(!at5.holds(resist), "Electricity Resistance comes at 6th");
    let at6 = held_for("cleric", 6, "choice:cleric_domain", "domain:air");
    assert!(at6.holds(arc) && at6.holds(resist));
}

#[test]
fn the_pool_pass_prints_no_air_domain_value_for_a_linked_pick() {
    for level in [1u8, 5, 20] {
        let printed = generic_values(&with_pick("cleric", level, "choice:cleric_domain", "domain:air"), "class_feature.cleric.domain.generic.");
        assert!(printed.is_empty(), "cleric {level}: {printed:#?}");
    }
}

#[test]
fn a_shaman_spirit_pick_links_to_the_spirit_choice_and_holds_the_spirit() {
    let links = sheet_rule_package::linked_picks(&with_pick("shaman", 5, "choice:shaman_spirit", "spirit:battle"));
    let link = links.iter().find(|l| l.choice_set_id == "choice:shaman_spirit").unwrap_or_else(|| panic!("no link: {links:#?}"));
    assert_eq!(link.chooser, "advanced_class_guide:class_feature:shaman_spirit");
    assert_eq!(link.option, "advanced_class_guide:class_feature:shaman_spirit_battle");
    assert!(link.option_held, "{link:?}");
}

#[test]
fn a_battle_shaman_holds_each_spirit_ability_at_the_level_the_book_grants_it() {
    let base = "advanced_class_guide:class_feature:battle_spirit_battle_spirit";
    let greater = "advanced_class_guide:class_feature:battle_spirit_enemies_bane";
    let true_ = "advanced_class_guide:class_feature:battle_spirit_paragon_of_battle";
    let at5 = held_for("shaman", 5, "choice:shaman_spirit", "spirit:battle");
    assert!(at5.holds(base));
    assert!(!at5.holds(greater) && !at5.holds(true_));
    let at8 = held_for("shaman", 8, "choice:shaman_spirit", "spirit:battle");
    assert!(at8.holds(base) && at8.holds(greater) && !at8.holds(true_));
    let at16 = held_for("shaman", 16, "choice:shaman_spirit", "spirit:battle");
    assert!(at16.holds(base) && at16.holds(greater) && at16.holds(true_));
}

#[test]
fn the_pool_pass_prints_no_battle_spirit_value_for_a_linked_pick() {
    for level in [1u8, 5, 20] {
        let printed = generic_values(&with_pick("shaman", level, "choice:shaman_spirit", "spirit:battle"), "class_feature.acg.shaman.spirit.generic.");
        assert!(printed.is_empty(), "shaman {level}: {printed:#?}");
    }
}

/// A pick the package cannot link keeps its old answer: the engine never guesses an option.
#[test]
fn an_invented_domain_links_to_nothing() {
    let links = sheet_rule_package::linked_picks(&with_pick("cleric", 5, "choice:cleric_domain", "domain:no_such_domain"));
    assert!(links.iter().all(|l| l.choice_set_id != "choice:cleric_domain"), "{links:#?}");
}

/// The canonical Path-A seeds (`class_seeds::canonical_seeds_for`) are the new choices' defaults:
/// the cleric's `domain:good` and the shaman's `spirit:life` are the fixture's existing picks, and
/// each now links to its converted choice. No seed changed.
///
/// The shaman's Life spirit is held. The cleric's Good domain is linked but NOT held: its record
/// is gated on the character's alignment (`cr_domains.lst`, `Good ... PREALIGN:LG,NG,CG`) and the
/// character record carries no alignment (`CharacterFacts::alignment` is `None`), so the held set
/// never assumes one.
#[test]
fn the_canonical_seeds_are_the_new_choices_defaults() {
    let fixture = load_sweep_fixture().expect("fixture");
    for (class, choice, chooser, option, held) in [
        ("cleric", "choice:cleric_domain", "core_rulebook:class:cleric#bonus1", "core_rulebook:domain:good", false),
        ("shaman", "choice:shaman_spirit", "advanced_class_guide:class_feature:shaman_spirit", "advanced_class_guide:class_feature:shaman_spirit_life", true),
    ] {
        let links = sheet_rule_package::linked_picks(&input_for(&fixture, class, 1));
        let link = links.iter().find(|l| l.choice_set_id == choice).unwrap_or_else(|| panic!("{class}: no link: {links:#?}"));
        assert_eq!((link.chooser.as_str(), link.option.as_str(), link.option_held), (chooser, option, held), "{class}");
    }
}

/// A domain several books print is one option: the package's own printing
/// (`SheetRulePackage::find`), never an ambiguity. Scalykind is printed by Inner Sea World Guide,
/// Bestiary 6 and Ultimate Wilderness; the pick links to the first book id's printing.
#[test]
fn a_domain_three_books_print_links_to_one_printing() {
    let links = sheet_rule_package::linked_picks(&with_pick("cleric", 1, "choice:cleric_domain", "domain:scalykind"));
    let link = links.iter().find(|l| l.choice_set_id == "choice:cleric_domain").unwrap_or_else(|| panic!("no link: {links:#?}"));
    assert_eq!(link.option, "bestiary_6:domain:scalykind");
    assert!(link.option_held, "{link:?}");
}
