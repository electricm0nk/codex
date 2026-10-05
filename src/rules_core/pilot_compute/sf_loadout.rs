//! The Starfinder 1e credits and bulk totals and the bulk limits, read from the CONVERTED
//! Starfinder package (`data/starfinder-1e/sheet_rules/`). SD-37 E4.5 (`epic-breakdown.md` Epic
//! E4; `decisions.md §5`: "bulk, and credits" are sheet totals).
//!
//! # Where each term comes from
//!
//! | Total | Term | Source |
//! |---|---|---|
//! | starting credits | Table 11-5 at the character's level, or the credits the build states | [`money::starting_wealth`] (SRD <https://www.aonsrd.com/Rules.aspx?ID=230>) |
//! | credits spent | each carried item's price × quantity | the item's `StatBlock "Price"` row (oracle `COST:`) |
//! | credits remaining | starting credits − credits spent | system arithmetic |
//! | bulk | each item's numeric bulk × quantity; light items ÷ 10, rounded down; negligible items 0 | the item's `StatBlock "Quality"` `Bulk: <n\|L\|->` row (oracle `QUALITY:Bulk`); [`encumbrance::bulk_terms`] (SRD Item Bulk) |
//! | credits spent, bulk | each applied `equipment_modifier` (upgrade, fusion, special material): its price, and its bulk when it states one | the modifier's `StatBlock "Price"` row (oracle `COST:`) and `Bulk:` row (oracle `SPROP:Bulk=`); SD-37 `decisions.md §20` |
//! | bulk | an augmentation adds 0 (it is part of the body); its price is spent | 144 of 144 augmentation records state no bulk, nor does the SRD augmentation page; SD-37 `decisions.md §20` |
//! | bulk limits, condition | ½ Strength score, Strength score | [`encumbrance::bulk_limits`] (SRD Bulk Limits) |
//!
//! The worn armour ([`SfBuild::armor`]) is carried and was bought, so it is one of the items
//! without being listed again. A term this reader cannot resolve (an item with no price, a bulk
//! of "Varies", a record that is not equipment, a loadout that costs more than the credits) is a
//! named [`SfChassisRefusal`], never a 0.

use super::sf_chassis::{SfChassisRefusal, SfTerm, SfTotal};
use super::sf_defense::SfBuild;
use crate::rules_core::encumbrance::{self, BulkCondition, BulkLimits, ItemBulk};
use crate::rules_core::game_system::GameSystem;
use crate::rules_core::money;
use crate::rules_core::sheet_rule::{split_rule_id, ProseFamily, ProsePiece, RuleId, SheetRule, SheetRulePackage};

/// What a Starfinder character carries, beyond the worn armour.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SfLoadout {
    /// The credits the character had to spend. `None` = Table 11-5 at the character's level (the
    /// Core Rulebook's 1,000 at 1st level; the table's budget for a character built above 1st).
    pub starting_credits: Option<i64>,
    /// `(equipment record id, quantity)` for every item carried other than the worn armour.
    pub carried: Vec<(RuleId, u32)>,
    /// `equipment_modifier` record ids applied to a carried item (armour upgrades, weapon
    /// fusions, special materials; the save's `applied_modifiers`). One with a `Price` row was
    /// bought, and one with a `Bulk:` quality adds its bulk. A fusion states neither: the fusion
    /// seal that pays for it is a carried equipment record of its own.
    pub applied: Vec<RuleId>,
}

/// The credits and bulk totals of one Starfinder character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfCarried {
    pub starting_credits: SfTotal,
    pub credits_spent: SfTotal,
    pub credits_remaining: SfTotal,
    pub bulk: SfTotal,
    pub limits: BulkLimits,
    pub condition: BulkCondition,
    /// Items of negligible bulk, printed by name (they add to bulk only if the GM rules so).
    pub negligible: Vec<RuleId>,
}

pub const REFUSED_NOT_HELD: &str = "sf_loadout.item_not_in_package";
pub const REFUSED_NOT_EQUIPMENT: &str = "sf_loadout.item_not_equipment";
pub const REFUSED_ITEM_PRICE: &str = "sf_loadout.item_price";
pub const REFUSED_ITEM_BULK: &str = "sf_loadout.item_bulk";
pub const REFUSED_NO_WEALTH_ROW: &str = "sf_loadout.no_wealth_row";
pub const REFUSED_OVER_BUDGET: &str = "sf_loadout.over_budget";

/// The tags that make an equipment record an augmentation (the oracle's `TYPE:` heads).
pub const AUGMENTATION_KINDS: [&str; 5] = ["Cybernetic", "Bio-Tech", "Magitech", "Necrograft", "Personal Upgrade"];

const SRD_WEALTH: &str = "SRD Table 11-5 Character Wealth per Level (https://www.aonsrd.com/Rules.aspx?ID=230)";
const SRD_BULK: &str = "SRD Carrying Capacity (https://www.aonsrd.com/Equipment.aspx)";
const BUILD_CREDITS: &str = "the build's stated credits";

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
}

fn total(terms: Vec<SfTerm>) -> SfTotal {
    SfTotal { total: terms.iter().map(|t| t.value).sum(), terms }
}

/// The text of every `StatBlock "<label>"` prose row of `rule`.
fn stat_rows<'r>(rule: &'r SheetRule, label: &'r str) -> impl Iterator<Item = String> + 'r {
    rule.prose.iter().filter_map(move |seg| match &seg.family {
        ProseFamily::StatBlock(l) if l == label => {
            seg.pieces.iter().map(|p| if let ProsePiece::Text(s) = p { Some(s.as_str()) } else { None }).collect::<Option<String>>()
        }
        _ => None,
    })
}

/// The item's price in credits: its one `StatBlock "Price"` row.
fn price(item: &SheetRule) -> Result<i64, SfChassisRefusal> {
    let rows: Vec<String> = stat_rows(item, "Price").collect();
    match rows.as_slice() {
        [one] => one.trim().parse::<i64>().map_err(|_| refuse(REFUSED_ITEM_PRICE, format!("{}: price {one:?} is not a number", item.id))),
        [] => Err(refuse(REFUSED_ITEM_PRICE, format!("{}: the record states no price", item.id))),
        many => Err(refuse(REFUSED_ITEM_PRICE, format!("{}: {} price rows {many:?}", item.id, many.len()))),
    }
}

/// The item's bulk: its one `StatBlock "Quality"` row reading `Bulk: <bulk>` (any case).
fn bulk(item: &SheetRule) -> Result<ItemBulk, SfChassisRefusal> {
    let rows: Vec<String> = stat_rows(item, "Quality")
        .filter_map(|q| {
            let (head, rest) = q.split_once(':')?;
            head.trim().eq_ignore_ascii_case("bulk").then(|| rest.trim().to_string())
        })
        .collect();
    match rows.as_slice() {
        [one] => ItemBulk::parse(one).ok_or_else(|| refuse(REFUSED_ITEM_BULK, format!("{}: bulk {one:?} is not a number, L or a dash", item.id))),
        [] => Err(refuse(REFUSED_ITEM_BULK, format!("{}: the record states no bulk", item.id))),
        many => Err(refuse(REFUSED_ITEM_BULK, format!("{}: {} bulk rows {many:?}", item.id, many.len()))),
    }
}

/// The equipment record `id`, checked to be equipment.
fn item<'p>(package: &'p SheetRulePackage, id: &str) -> Result<&'p SheetRule, SfChassisRefusal> {
    let rule = package.rule(id).ok_or_else(|| refuse(REFUSED_NOT_HELD, format!("{id}: no such record in the Starfinder package")))?;
    if !matches!(split_rule_id(id).1, "equipment" | "equipment_modifier") || id.contains('#') {
        return Err(refuse(REFUSED_NOT_EQUIPMENT, format!("{id}: not an equipment record")));
    }
    Ok(rule)
}

/// Credits and bulk totals, bulk limits and condition of `build` carrying `loadout`, read
/// from `package` (the Starfinder package).
pub fn compute(package: &SheetRulePackage, build: &SfBuild, loadout: &SfLoadout) -> Result<SfCarried, SfChassisRefusal> {
    let level: i64 = build.chassis.classes.iter().map(|(_, l)| i64::from(*l)).sum();
    let first_class = build.chassis.classes.first().map(|(c, _)| c.as_str()).unwrap_or("");
    let starting = match loadout.starting_credits {
        Some(c) => SfTerm { label: "credits stated by the build".into(), value: c, source: BUILD_CREDITS.into() },
        None => {
            let c = money::starting_wealth(GameSystem::Starfinder1e, first_class, level)
                .ok_or_else(|| refuse(REFUSED_NO_WEALTH_ROW, format!("no Table 11-5 row for character level {level}")))?;
            SfTerm { label: format!("wealth at character level {level}"), value: c, source: SRD_WEALTH.into() }
        }
    };

    let mut items: Vec<(&SheetRule, u32)> = Vec::new();
    if let Some(armor) = &build.armor {
        items.push((item(package, armor)?, 1));
    }
    for (id, quantity) in &loadout.carried {
        items.push((item(package, id)?, *quantity));
    }
    for id in &loadout.applied {
        if split_rule_id(id).1 != "equipment_modifier" {
            return Err(refuse(REFUSED_NOT_EQUIPMENT, format!("{id}: applied to an item, but not an equipment modifier")));
        }
        items.push((item(package, id)?, 1));
    }

    let mut spent = Vec::new();
    let mut bulks = Vec::new();
    let mut bulk_terms = Vec::new();
    let mut light_items = Vec::new();
    let mut negligible = Vec::new();
    for (rule, quantity) in &items {
        let modifier = split_rule_id(&rule.id).1 == "equipment_modifier";
        let states_bulk = bulk(rule).is_ok();
        // An applied modifier that states neither a price nor a bulk (a fusion) adds no term.
        if modifier && stat_rows(rule, "Price").next().is_none() && !states_bulk {
            continue;
        }
        let p = price(rule)?;
        spent.push(SfTerm { label: format!("{} × {quantity}", rule.label), value: p * i64::from(*quantity), source: rule.id.clone() });
        // An augmentation adds no bulk; neither does an applied modifier that states none (a
        // special material changes the item it is applied to, already counted).
        let augmentation = AUGMENTATION_KINDS.iter().find(|k| rule.tags.iter().any(|t| t == *k));
        if let Some(kind) = augmentation.filter(|_| !states_bulk) {
            bulk_terms.push(SfTerm { label: format!("{} × {quantity} ({kind} augmentation, no bulk)", rule.label), value: 0, source: rule.id.clone() });
            continue;
        }
        if modifier && !states_bulk {
            continue;
        }
        let b = bulk(rule)?;
        bulks.push((b, *quantity));
        match b {
            ItemBulk::Units(n) => bulk_terms.push(SfTerm { label: format!("{} × {quantity}", rule.label), value: n * i64::from(*quantity), source: rule.id.clone() }),
            ItemBulk::Light => light_items.push(format!("{} × {quantity}", rule.label)),
            ItemBulk::Negligible => negligible.push(rule.id.clone()),
        }
    }
    let (_, light) = encumbrance::bulk_terms(bulks);
    if light > 0 {
        bulk_terms.push(SfTerm {
            label: format!("{light} light items ÷ 10, rounded down ({})", light_items.join(", ")),
            value: light / 10,
            source: SRD_BULK.into(),
        });
    }
    let credits_spent = total(spent);
    let remaining = starting.value - credits_spent.total;
    if remaining < 0 {
        return Err(refuse(REFUSED_OVER_BUDGET, format!("the loadout costs {} credits; {} were available", credits_spent.total, starting.value)));
    }
    let credits_remaining = total(vec![
        starting.clone(),
        SfTerm { label: "credits spent".into(), value: -credits_spent.total, source: "the carried items' prices".into() },
    ]);
    let bulk = total(bulk_terms);
    let limits = encumbrance::bulk_limits(build.chassis.ability_scores[0]);
    let condition = BulkCondition::of(bulk.total, limits);
    Ok(SfCarried { starting_credits: total(vec![starting]), credits_spent, credits_remaining, bulk, limits, condition, negligible })
}

/// Test support: the four seeds' loadouts (`seed-builds.md` §1–§4 "Armour and weapons" and
/// "Other gear", less the worn armour, which [`SfBuild::armor`] already names).
#[cfg(test)]
pub(crate) mod seed_loadouts {
    use super::*;

    pub fn loadouts() -> Vec<(&'static str, SfLoadout)> {
        let l = |items: &[(&str, u32)]| SfLoadout {
            starting_credits: None,
            carried: items.iter().map(|(id, q)| (format!("core:equipment:{id}"), *q)).collect(),
            applied: Vec::new(),
        };
        vec![
            ("SF-Soldier-3", l(&[("laser_rifle_azimuth", 1), ("baton_tactical", 1), ("battery", 2), ("serum_of_healing_mk_1", 2)])),
            (
                "SF-Mystic-5",
                l(&[("laser_pistol_azimuth", 1), ("baton_tactical", 1), ("battery", 1), ("medkit_basic", 1), ("serum_of_healing_mk_1", 2)]),
            ),
            ("SF-Technomancer-5", l(&[("laser_pistol_azimuth", 1), ("battery", 2), ("medkit_basic", 1), ("serum_of_healing_mk_1", 2)])),
            ("SF-Envoy-3", l(&[("semi_auto_pistol_tactical", 1), ("baton_tactical", 1), ("serum_of_healing_mk_1", 2)])),
        ]
    }
}

#[cfg(test)]
mod sf_seed {
    use super::super::sf_defense::seed_support::*;
    use super::seed_loadouts::loadouts;
    use super::*;

    fn seed_build(name: &str) -> SfBuild {
        seeds().into_iter().find(|(s, _)| *s == name).unwrap_or_else(|| panic!("no seed {name}")).1
    }

    #[test]
    fn sf_seed_credits_bulk_and_bulk_limits_match_the_srd_hand_values() {
        let hand = hand_values();
        let mut bad = Vec::new();
        let mut checked = 0;
        for (seed, loadout) in loadouts() {
            let c = compute(package(), &seed_build(seed), &loadout).unwrap_or_else(|r| panic!("{seed}: {r:?}"));
            for (field, got) in [
                ("Starting credits", c.starting_credits.total),
                ("Credits spent", c.credits_spent.total),
                ("Credits remaining", c.credits_remaining.total),
                ("Bulk", c.bulk.total),
                ("Bulk limit unencumbered", c.limits.unencumbered_max),
                ("Bulk limit overburdened", c.limits.overburdened_above),
            ] {
                let want = hand.get(&(seed.to_string(), field.to_string())).unwrap_or_else(|| panic!("{seed} {field}: no hand value"));
                checked += 1;
                if *want != Hand::Value(got) {
                    bad.push(format!("{seed} {field}: engine {got}, SRD {want:?} -- {c:?}"));
                }
            }
        }
        assert_eq!(checked, 24, "4 seeds x 6 fields");
        assert!(bad.is_empty(), "{} of 24 mismatch:\n{}", bad.len(), bad.join("\n"));
    }

    /// `seed-builds.md` §5: no seed is encumbered.
    #[test]
    fn sf_seed_no_seed_is_encumbered() {
        for (seed, loadout) in loadouts() {
            let c = compute(package(), &seed_build(seed), &loadout).unwrap();
            assert_eq!(c.condition, BulkCondition::Unencumbered, "{seed}: {c:?}");
            assert_eq!((c.condition.max_dex_cap(), c.condition.check_penalty()), (None, 0), "{seed}");
        }
    }

    /// Every price and numeric-bulk term names the item record it was read from; the worn armour
    /// is one of them, once.
    #[test]
    fn sf_seed_every_term_names_its_item_record() {
        for (seed, loadout) in loadouts() {
            let build = seed_build(seed);
            let c = compute(package(), &build, &loadout).unwrap();
            let armor = build.armor.clone().unwrap();
            assert_eq!(c.credits_spent.terms.iter().filter(|t| t.source == armor).count(), 1, "{seed}: {:?}", c.credits_spent);
            assert_eq!(c.credits_spent.terms.len(), loadout.carried.len() + 1, "{seed}");
            for t in c.credits_spent.terms.iter().chain(&c.bulk.terms) {
                assert!(t.source.starts_with("core:equipment:") || t.source.starts_with("SRD "), "{seed}: {t:?}");
            }
            assert!(c.negligible.iter().all(|id| id == "core:equipment:battery"), "{seed}: {:?}", c.negligible);
        }
    }

    /// The light-item rule is the SRD's, not a per-item 0: 10 serums (L) are 1 bulk, 19 still 1.
    #[test]
    fn sf_seed_ten_light_items_add_one_bulk() {
        let soldier = seed_build("SF-Soldier-3");
        let base = compute(package(), &soldier, &SfLoadout { starting_credits: Some(100_000), carried: vec![], applied: vec![] }).unwrap().bulk.total;
        for (n, extra) in [(9, 0), (10, 1), (19, 1), (20, 2)] {
            let loadout = SfLoadout { starting_credits: Some(100_000), carried: vec![("core:equipment:serum_of_healing_mk_1".into(), n)], applied: vec![] };
            assert_eq!(compute(package(), &soldier, &loadout).unwrap().bulk.total, base + extra, "{n} light items");
        }
    }

    /// Over the limits: the Envoy (Str 8) carrying 5 medkits (1 bulk each) + armour 1 = 6 > 4 is
    /// encumbered; 8 more = 9 > 8 is overburdened.
    #[test]
    fn sf_seed_bulk_over_the_limits_gives_the_condition() {
        let envoy = seed_build("SF-Envoy-3");
        let with = |n: u32| {
            let loadout = SfLoadout { starting_credits: Some(100_000), carried: vec![("core:equipment:medkit_basic".into(), n)], applied: vec![] };
            compute(package(), &envoy, &loadout).unwrap()
        };
        assert_eq!(with(3).condition, BulkCondition::Unencumbered);
        let e = with(5);
        assert_eq!((e.bulk.total, e.condition), (6, BulkCondition::Encumbered));
        assert_eq!((e.condition.max_dex_cap(), e.condition.check_penalty()), (Some(2), -5));
        assert_eq!(with(8).condition, BulkCondition::Overburdened);
    }

    /// `decisions.md §20` ruling 2: an augmentation (tagged `Cybernetic`, `Bio-Tech`, `Magitech`,
    /// `Necrograft` or `Personal Upgrade`) states no bulk and adds none; its price is spent.
    #[test]
    fn sf_an_augmentation_adds_its_price_and_no_bulk() {
        let techno = seed_build("SF-Technomancer-5");
        let (_, base) = loadouts().into_iter().find(|(s, _)| *s == "SF-Technomancer-5").unwrap();
        let before = compute(package(), &techno, &base).unwrap();
        let mut with = base.clone();
        with.carried.push(("core:equipment:cybernetic_vocal_modulator".into(), 1));
        let after = compute(package(), &techno, &with).unwrap_or_else(|r| panic!("an augmented character computes: {r:?}"));
        assert_eq!(after.credits_spent.total, before.credits_spent.total + 125, "{:?}", after.credits_spent);
        assert_eq!(after.bulk.total, before.bulk.total, "{:?}", after.bulk);
        let term = after.bulk.terms.iter().find(|t| t.source == "core:equipment:cybernetic_vocal_modulator").expect("the augmentation's bulk term prints");
        assert_eq!(term.value, 0, "{term:?}");
        assert!(term.label.contains("augmentation"), "{term:?}");
    }

    /// `decisions.md §20` ruling 1: an `equipment_modifier` applied to a carried item (the save's
    /// `applied_modifiers`) feeds credits spent with its price and bulk with its `Bulk:` row. A
    /// fusion states neither (the fusion seal that pays for it is carried equipment) and adds no
    /// term; a special material states a price and no bulk and adds its price only; a modifier
    /// that states a bulk but no price is refused by name.
    #[test]
    fn sf_an_applied_modifier_feeds_credits_and_bulk() {
        let soldier = seed_build("SF-Soldier-3");
        let (_, mut base) = loadouts().into_iter().find(|(s, _)| *s == "SF-Soldier-3").unwrap();
        // A budget that holds the 2,500-credit material too (the seed's own is 4,000).
        base.starting_credits = Some(100_000);
        let before = compute(package(), &soldier, &base).unwrap();
        let with = |ids: &[&str]| {
            let mut l = base.clone();
            l.applied = ids.iter().map(|id| format!("core:equipment_modifier:{id}")).collect();
            compute(package(), &soldier, &l)
        };
        let loader = with(&["armor_automated_loader"]).unwrap();
        assert_eq!(loader.credits_spent.total, before.credits_spent.total + 750, "{:?}", loader.credits_spent);
        assert_eq!(loader.bulk.total, before.bulk.total + 1, "{:?}", loader.bulk);
        assert!(loader.credits_spent.terms.iter().any(|t| t.source == "core:equipment_modifier:armor_automated_loader"));
        let fusion = with(&["weapon_ominous"]).unwrap();
        assert_eq!((fusion.credits_spent.total, fusion.bulk.total), (before.credits_spent.total, before.bulk.total), "{fusion:?}");
        assert!(fusion.credits_spent.terms.iter().all(|t| !t.source.contains("weapon_ominous")));
        let material = with(&["weapon_adamantine_alloy"]).unwrap_or_else(|r| panic!("a special material computes: {r:?}"));
        assert_eq!(material.credits_spent.total, before.credits_spent.total + 2500);
        assert_eq!(material.bulk.total, before.bulk.total);
        let mut malformed = base.clone();
        malformed.applied = vec!["armory:equipment_modifier:mobility_enhancer_mk".into()];
        assert_eq!(compute(package(), &soldier, &malformed).unwrap_err().id, REFUSED_ITEM_PRICE);
        let mut not_modifier = base.clone();
        not_modifier.applied = vec!["core:class:soldier".into()];
        assert_eq!(compute(package(), &soldier, &not_modifier).unwrap_err().id, REFUSED_NOT_EQUIPMENT);
    }

    #[test]
    fn sf_seed_unresolvable_items_and_budgets_are_refused_by_name() {
        let soldier = seed_build("SF-Soldier-3");
        let one = |id: &str| SfLoadout { starting_credits: None, carried: vec![(id.to_string(), 1)], applied: vec![] };
        assert_eq!(compute(package(), &soldier, &one("core:class:soldier")).unwrap_err().id, REFUSED_NOT_EQUIPMENT);
        assert_eq!(compute(package(), &soldier, &one("core:equipment:no_such_item")).unwrap_err().id, REFUSED_NOT_HELD);
        assert_eq!(compute(package(), &soldier, &one("armory:equipment:musical_instrument_basic")).unwrap_err().id, REFUSED_ITEM_BULK);
        let broke = SfLoadout { starting_credits: Some(100), carried: vec![], applied: vec![] };
        assert_eq!(compute(package(), &soldier, &broke).unwrap_err().id, REFUSED_OVER_BUDGET);
        let mut level_21 = soldier.clone();
        level_21.chassis.classes = vec![("core:class:soldier".into(), 21)];
        assert_eq!(compute(package(), &level_21, &SfLoadout::default()).unwrap_err().id, REFUSED_NO_WEALTH_ROW);
    }
}
