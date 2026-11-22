//! SD-36 Epic F4 merge-readiness blocker 1 (`stage-f4-f5/merge-readiness-blockers.json`):
//! a Path-A pick links only to an option of the kind its own namespace names.
//!
//! The F4pre fallback in `link_path_a_picks` looked up a rule named `<member>` in every kind and
//! linked the pick to any held Domains/Rules chooser offering it. The shared fixture's Human
//! ability-bonus pick (`choice:human_ability_bonus -> ability:strength`, GE-06 fixture; the
//! desktop records the same shape, `pf1_adapter.rs`) therefore became
//! `core_rulebook:domain:strength` under the cleric's, the shaman's and the paladin's domain
//! count -- a domain the player never chose. The paladin's own count prints +0 (it offers no
//! domain at all).
//!
//! ONE rule, no per-class case:
//! - the bare `<member>` candidate answers only when the pick's namespace names the option's kind
//!   (`domain:air` -> kind `domain`; `ability:strength` names kind `ability`, never `domain`);
//! - the `<pool>_<member>` candidate is unchanged (`spirit:battle` -> `shaman_spirit_battle`);
//! - a chooser whose count evaluates to 0 offers nothing.

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::class_census::{census, load_sweep_fixture, sheet_dump_with_rules_text};
use codex::rules_core::class_seeds::input_for;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::sheet_rule::{held_set, id_slug, split_rule_id, CharacterFacts, Granter, HeldSeed};
use codex::rules_core::sheet_rule_package;

fn held_ids(class: &str, level: u8) -> Vec<String> {
    let fixture = load_sweep_fixture().expect("fixture");
    let dump = sheet_dump_with_rules_text(&fixture, &[(class.to_owned(), level)], None);
    dump.lines().filter_map(|l| l.strip_prefix("HELD| ")).map(str::to_owned).collect()
}

fn line_ids(class: &str, level: u8) -> Vec<String> {
    let fixture = load_sweep_fixture().expect("fixture");
    let dump = sheet_dump_with_rules_text(&fixture, &[(class.to_owned(), level)], None);
    dump.lines()
        .filter_map(|l| l.strip_prefix("LINE| id="))
        .map(|l| l.split(' ').next().unwrap_or("").to_owned())
        .collect()
}

#[test]
fn the_fixture_is_a_human_who_puts_the_ability_bonus_on_strength() {
    let fixture = load_sweep_fixture().expect("fixture");
    assert!(
        fixture
            .chosen
            .selected_choices
            .iter()
            .any(|c| c.choice_set_id == "choice:human_ability_bonus" && c.selection_id == "ability:strength"),
        "the premise of this file: {:#?}",
        fixture.chosen.selected_choices
    );
}

#[test]
fn a_human_cleric_with_the_bonus_on_strength_holds_no_strength_domain() {
    for level in [1u8, 5] {
        let held = held_ids("cleric", level);
        assert!(!held.iter().any(|id| id == "core_rulebook:domain:strength"), "cleric {level}: {held:#?}");
        let lines = line_ids("cleric", level);
        for fabricated in ["core_rulebook:domain:strength", "core_rulebook:class_feature:domain_power_strength_surge"] {
            assert!(!lines.iter().any(|id| id == fabricated), "cleric {level} prints {fabricated}");
        }
    }
}

#[test]
fn a_human_paladin_holds_no_domain() {
    let held = held_ids("paladin", 5);
    let domains: Vec<&String> = held.iter().filter(|id| split_rule_id(id).1 == "domain").collect();
    assert!(domains.is_empty(), "paladin 5 holds {domains:#?}");
}

#[test]
fn a_human_shaman_holds_no_strength_domain() {
    let held = held_ids("shaman", 5);
    assert!(!held.iter().any(|id| id == "core_rulebook:domain:strength"), "shaman 5: {held:#?}");
}

#[test]
fn the_ability_bonus_pick_links_to_nothing() {
    let fixture = load_sweep_fixture().expect("fixture");
    for class in ["cleric", "paladin", "shaman"] {
        let links = sheet_rule_package::linked_picks(&input_for(&fixture, class, 5));
        assert!(links.iter().all(|l| l.choice_set_id != "choice:human_ability_bonus"), "{class}: {links:#?}");
    }
}

/// Whether `link` is reached from its own pick: the option is the kind the pick's namespace names
/// (bare `<member>`), or the option is -- or grants -- the record `<pool>_<member>`.
fn link_is_the_picks_own(input: &CharacterInput, link: &codex::rules_core::sheet_rule::LinkedPick) -> bool {
    let package = sheet_rule_package::package().as_ref().expect("package");
    let pool = link.choice_set_id.strip_prefix("choice:").unwrap_or(&link.choice_set_id);
    let record_slug = format!("{pool}_{}", link.member);
    let namespaces: Vec<String> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == link.choice_set_id && id_slug(&c.selection_id) == link.member)
        .filter_map(|c| c.selection_id.rsplit(':').nth(1).map(str::to_owned))
        .collect();
    let (_, kind, option_slug) = split_rule_id(&link.option);
    if namespaces.iter().any(|ns| ns == kind) && option_slug == link.member {
        return true;
    }
    if option_slug == record_slug {
        return true;
    }
    package.find_all("class_feature", &record_slug).iter().any(|record| {
        package
            .rule(record)
            .is_some_and(|r| r.granted_by.iter().any(|g| matches!(&g.by, Granter::Rule(o) if o == &link.option)))
    })
}

/// The population scan: every non-prestige census class at min(5, max level) on the shared
/// fixture. Every linked pick must come from its own pool or namespace: 0 foreign links.
#[test]
fn no_linked_pick_over_the_non_prestige_population_comes_from_a_foreign_pool() {
    let fixture = load_sweep_fixture().expect("fixture");
    let entries = census();
    let classes: Vec<_> = entries.values().filter(|e| !e.is_prestige).collect();
    assert_eq!(classes.len(), 63, "the non-prestige denominator");
    let package = sheet_rule_package::package().as_ref().expect("package");
    let mut total = 0usize;
    let mut reached_total = 0usize;
    let mut foreign = Vec::new();
    for entry in &classes {
        let slug = entry.class_id.strip_prefix("class:").unwrap_or(&entry.class_id);
        let level = entry.max_level.min(5);
        let input = input_for(&fixture, slug, level);
        let links = sheet_rule_package::linked_picks(&input);
        for link in &links {
            total += 1;
            if !link_is_the_picks_own(&input, link) {
                foreign.push(format!("{slug} {level}: {} -> {} via {}", link.choice_set_id, link.option, link.chooser));
            }
        }
        // Every held rule reached through a linked pick: the held set with the links recorded,
        // minus the held set without them (the census's own construction).
        let receipt = build_pilot_headless_receipt(&input);
        let seed = HeldSeed::from_character(&input, &receipt.computation);
        let bare = CharacterFacts::from_character(&input, &receipt.computation);
        let linked = bare.with_linked_picks(package, &seed);
        let without = held_set(package, &seed, &bare);
        let with = held_set(package, &seed, &linked);
        let reached: Vec<&String> = with.rules.keys().filter(|id| !without.rules.contains_key(*id)).collect();
        reached_total += reached.len();
        let summary: Vec<String> = links.iter().map(|l| format!("{} -> {}", l.choice_set_id, l.option)).collect();
        eprintln!("ROW| {slug} {level}: links {} reached {} | {}", links.len(), reached.len(), summary.join("; "));
        for id in reached {
            eprintln!("REACHED| {slug} {level}: {id}");
        }
    }
    eprintln!(
        "linked picks over 63 non-prestige classes at min(5, max level): {total}; foreign {}; held rules reached through a linked pick: {reached_total}",
        foreign.len()
    );
    assert!(foreign.is_empty(), "{} of {total} links come from a foreign pool: {foreign:#?}", foreign.len());
}

/// A chooser whose count evaluates to 0 offers nothing: the paladin's domain count
/// (`core_rulebook:class:paladin#bonus1`, `Paladin (domains)`, printed +0) takes no domain even
/// when the record carries a domain pick whose namespace names the kind.
#[test]
fn a_domain_count_of_zero_offers_no_domain() {
    let fixture = load_sweep_fixture().expect("fixture");
    let mut input = input_for(&fixture, "paladin", 5);
    input.chosen.selected_choices.push(codex::rules_core::character_input::SelectedChoice {
        choice_set_id: "choice:cleric_domain".to_owned(),
        selection_id: "domain:air".to_owned(),
    });
    let links = sheet_rule_package::linked_picks(&input);
    assert!(links.iter().all(|l| l.chooser != "core_rulebook:class:paladin#bonus1"), "{links:#?}");
}
