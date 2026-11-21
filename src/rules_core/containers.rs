//! Container loads (SD-41 `decisions.md §7`, §25): how much each container copy in a set holds,
//! against what it can hold.

use std::collections::{BTreeMap, BTreeSet};

use crate::rules_core::encumbrance::copy_weight_lbs;
use crate::rules_core::equipment_effects::stand_in_record;
use crate::rules_core::equipment_record::ContainerCapacity;
use crate::rules_core::equipment_resolver::equipment_converted_resolve;
use crate::rules_core::size::SizeCategory;
use crate::rules_core::source_content::SourcePackageContent;
use crate::saved_character::equipment_sets::{location_of, CopyKey, EquipmentSet, Location};

/// One container copy's load in a set: what its contents weigh, and whether that is above a
/// stated pound capacity. Over is a warning, never a block (§7).
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerLoad {
    pub copy: CopyKey,
    pub contents_lbs: f64,
    pub capacity: ContainerCapacity,
    pub over: bool,
    /// What the contents weigh toward the carried total, in percent (SD-42 E4.7): 0 for a Bag of
    /// Holding, 100 for a plain container and a Create item. `contents_lbs` is always in full.
    pub contents_weight_pct: u8,
}

impl ContainerLoad {
    /// `over` only for `Pounds`: `Unlimited` and `Unstated` (DEF-1) are never over.
    pub fn new(copy: CopyKey, contents_lbs: f64, capacity: ContainerCapacity) -> Self {
        let over = matches!(capacity, ContainerCapacity::Pounds(lbs) if contents_lbs > lbs);
        Self { copy, contents_lbs, capacity, over, contents_weight_pct: 100 }
    }
}

/// The load of every container copy in `set`: each of `held` (a copy, its item id and its applied
/// modifiers, which can lighten it) whose
/// settled record is a container, in `held` order, then each Create item container (§25) with
/// its capacity in lb. Contents are summed recursively over `Inside` locations (a pouch in a
/// backpack adds to both), each copy weighing what `compute_encumbrance` weighs it: the
/// `weight_and_cost()` (or stand-in) weight times its bundles from `quantities` (§30), and each
/// Create item of `weighed` its own stated lb (§25), times its size's weight for a copy `item_sizes`
/// names (E2.12), or the weight per unit `weight_overrides` states for it (SD-42 E4.2); an unweighed
/// or unresolved copy adds nothing (SD-b).
#[allow(clippy::too_many_arguments)]
pub fn container_loads(
    set: &EquipmentSet,
    held: &[(CopyKey, &str, &[String])],
    corpus: &SourcePackageContent,
    created: &[(CopyKey, f64)],
    weighed: &[(CopyKey, f64)],
    quantities: &[(CopyKey, u32)],
    item_sizes: &[(CopyKey, SizeCategory)],
    weight_overrides: &[(CopyKey, f64)],
) -> Vec<ContainerLoad> {
    let mut weights: BTreeMap<&CopyKey, f64> = BTreeMap::new();
    let mut containers: Vec<(CopyKey, ContainerCapacity, u8)> = Vec::new();
    for (copy, item_id, modifiers) in held {
        let Some(record) = equipment_converted_resolve(item_id, corpus) else { continue };
        let stand_in = stand_in_record(record, corpus);
        let weight = copy_weight_lbs(record, stand_in, copy, modifiers, corpus, quantities, item_sizes, weight_overrides);
        weights.insert(copy, weight.unwrap_or(0.0));
        if record.is_container() {
            containers.push((copy.clone(), record.capacity(), record.contents_weight_pct));
        }
    }
    weights.extend(weighed.iter().map(|(copy, lbs)| (copy, *lbs)));
    containers.extend(created.iter().map(|(copy, lbs)| (copy.clone(), ContainerCapacity::Pounds(*lbs), 100)));

    // Each weighed copy's enclosing containers, innermost first; a cycle (a hand-edited file,
    // `project` logs it) stops the walk.
    let mut contents: BTreeMap<CopyKey, f64> = BTreeMap::new();
    for (copy, weight) in &weights {
        let mut seen = BTreeSet::from([(*copy).clone()]);
        let mut current = (*copy).clone();
        while let Location::Inside { container } = location_of(set, &current) {
            if !seen.insert(container.clone()) {
                break;
            }
            *contents.entry(container.clone()).or_insert(0.0) += weight;
            current = container;
        }
    }

    containers
        .into_iter()
        .map(|(copy, capacity, contents_weight_pct)| {
            let lbs = contents.get(&copy).copied().unwrap_or(0.0);
            ContainerLoad { contents_weight_pct, ..ContainerLoad::new(copy, lbs, capacity) }
        })
        .collect()
}

/// What each copy in `set` weighs toward the carried total (SD-42 E4.7, `decisions.md §27.1`):
/// the product of its enclosing `Inside` containers' [`contents_weight_factor`]s, so a copy in a
/// Backpack in a Bag of Holding is 0 and one in the Versatile Vest 0.6. Only copies below 1 are
/// listed; a container's own weight is its enclosing containers' business, not its own. `held` is
/// [`container_loads`]' list; a Create item container (no flag in `custom.json`) is 1.
///
/// [`contents_weight_factor`]: crate::rules_core::equipment_record::CorpusEquipmentRecord::contents_weight_factor
pub fn contents_weight_factors(set: &EquipmentSet, held: &[(CopyKey, &str, &[String])], corpus: &SourcePackageContent) -> Vec<(CopyKey, f64)> {
    let factor_of: BTreeMap<&CopyKey, f64> = held
        .iter()
        .filter_map(|(copy, item_id, _)| Some((copy, equipment_converted_resolve(item_id, corpus)?.contents_weight_factor())))
        .filter(|(_, factor)| *factor < 1.0)
        .collect();
    let mut factors = Vec::new();
    for entry in &set.locations {
        let mut seen = BTreeSet::from([entry.copy.clone()]);
        let (mut current, mut factor) = (entry.copy.clone(), 1.0_f64);
        while let Location::Inside { container } = location_of(set, &current) {
            if !seen.insert(container.clone()) {
                break;
            }
            factor *= factor_of.get(&container).copied().unwrap_or(1.0);
            current = container;
        }
        if factor < 1.0 {
            factors.push((entry.copy.clone(), factor));
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
    use crate::saved_character::equipment_sets::SetEntry;
    use std::path::Path;

    fn core_rulebook() -> SourcePackageContent<'static> {
        load_equipment_corpus(&[BookCorpusRoot { book_id: "core_rulebook", dir: Path::new("data/corpus/core_rulebook") }])
    }

    fn key(item_id: &str, ordinal: u32) -> CopyKey {
        CopyKey { item_id: item_id.to_owned(), bonus: 0, ordinal }
    }

    /// `n` copies of `item_id`, ordinals 0..n.
    fn copies(item_id: &str, n: u32) -> Vec<CopyKey> {
        (0..n).map(|ordinal| key(item_id, ordinal)).collect()
    }

    fn inside(copies: &[CopyKey], container: &CopyKey) -> Vec<SetEntry> {
        copies.iter().map(|copy| SetEntry { copy: copy.clone(), location: Location::Inside { container: container.clone() } }).collect()
    }

    fn held(keys: &[CopyKey]) -> Vec<(CopyKey, &str, &[String])> {
        keys.iter().map(|copy| (copy.clone(), copy.item_id.as_str(), &[][..])).collect()
    }

    fn load_of<'a>(loads: &'a [ContainerLoad], copy: &CopyKey) -> &'a ContainerLoad {
        loads.iter().find(|load| &load.copy == copy).unwrap_or_else(|| panic!("no load for {copy:?} in {loads:#?}"))
    }

    const RATIONS: &str = "item:Rations (Trail/Per Day)"; // 1 lb, not a container
    const POUCH: &str = "item:Pouch (Belt)"; // 0.5 lb, Unlimited
    const SACK: &str = "item:Sack"; // 0.5 lb, Unlimited
    const SLED: &str = "item:Sled"; // 300 lb, Pounds(2000)
    const WATERSKIN: &str = "item:Waterskin"; // no WT, Pounds(4)
    const ARROWS: &str = "item:Arrows (20)"; // 3 lb a bundle of 20, not a container

    /// A 60 lb backpack (a Create item, §25: the CRB Backpack reads `UNLIM|Any` until E1.12)
    /// holding a pouch holding 10 lb of rations: the pouch holds 10, the backpack 10 + the pouch's
    /// own 0.5. The same set in reverse entry order gives the same loads in the same order.
    #[test]
    fn containers_nested_sum_reaches_the_outer() {
        let corpus = core_rulebook();
        let backpack = key("custom:backpack", 0);
        let pouch = key(POUCH, 0);
        let sled = key(SLED, 0);
        let rations = copies(RATIONS, 10);
        let mut keys = vec![sled.clone(), pouch.clone()];
        keys.extend(rations.clone());
        let mut locations = inside(&rations, &pouch);
        locations.extend(inside(std::slice::from_ref(&pouch), &backpack));
        let set = EquipmentSet { name: "Default".to_owned(), locations };
        let created = [(backpack.clone(), 60.0)];

        let loads = container_loads(&set, &held(&keys), &corpus, &created, &[], &[], &[], &[]);
        assert_eq!(loads.iter().map(|load| load.copy.clone()).collect::<Vec<_>>(), vec![sled.clone(), pouch.clone(), backpack.clone()], "copies in held order, then Create items");
        assert_eq!(load_of(&loads, &pouch).contents_lbs, 10.0);
        assert_eq!(load_of(&loads, &pouch).capacity, ContainerCapacity::Unlimited);
        assert_eq!(load_of(&loads, &backpack).contents_lbs, 10.5, "the pouch's rations and the pouch itself");
        assert_eq!(load_of(&loads, &backpack).capacity, ContainerCapacity::Pounds(60.0));
        assert_eq!(load_of(&loads, &sled).contents_lbs, 0.0);
        assert_eq!(load_of(&loads, &sled).capacity, ContainerCapacity::Pounds(2000.0), "a stock record's settled capacity");
        assert!(loads.iter().all(|load| !load.over));

        let reversed = EquipmentSet { name: set.name.clone(), locations: set.locations.iter().rev().cloned().collect() };
        assert_eq!(container_loads(&reversed, &held(&keys), &corpus, &created, &[], &[], &[], &[]), loads, "deterministic in the set's order");
    }

    /// `over` is contents above a `Pounds` capacity: 60 lb in a 60 lb backpack is not over, 61 is.
    #[test]
    fn containers_over_capacity_flag() {
        let corpus = core_rulebook();
        let backpack = key("custom:backpack", 0);
        for (rations_lbs, over) in [(60, false), (61, true)] {
            let rations = copies(RATIONS, rations_lbs);
            let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &backpack) };
            let loads = container_loads(&set, &held(&rations), &corpus, &[(backpack.clone(), 60.0)], &[], &[], &[], &[]);
            assert_eq!(loads.len(), 1, "{loads:#?}");
            assert_eq!((loads[0].contents_lbs, loads[0].over), (f64::from(rations_lbs), over));
        }
    }

    /// `Unlimited` and `Unstated` are never over, whatever they hold.
    #[test]
    fn containers_unlimited_is_never_over() {
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let rations = copies(RATIONS, 500);
        let mut keys = vec![sack.clone()];
        keys.extend(rations.clone());
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &sack) };
        let loads = container_loads(&set, &held(&keys), &corpus, &[], &[], &[], &[], &[]);
        assert_eq!(loads, vec![ContainerLoad { copy: sack, contents_lbs: 500.0, capacity: ContainerCapacity::Unlimited, over: false, contents_weight_pct: 100 }]);

        let unstated = ContainerLoad::new(key("custom:x", 0), 500.0, ContainerCapacity::Unstated);
        assert!(!unstated.over, "Unstated (DEF-1) is never over");
    }

    /// A copy whose weight is unknown (the Waterskin states no WT; an id no record answers) adds
    /// nothing to its container (SD-b); the unweighed Waterskin is still a container with a load.
    #[test]
    fn containers_unweighed_copy_adds_nothing() {
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let waterskin = key(WATERSKIN, 0);
        let unknown = key("item:No Such Record Anywhere", 0);
        let ration = key(RATIONS, 0);
        let keys = vec![sack.clone(), waterskin.clone(), unknown.clone(), ration.clone()];
        let mut locations = inside(&[waterskin.clone(), unknown], &sack);
        locations.extend(inside(&[ration], &waterskin));
        let set = EquipmentSet { name: "Default".to_owned(), locations };
        let loads = container_loads(&set, &held(&keys), &corpus, &[], &[], &[], &[], &[]);
        assert_eq!(load_of(&loads, &waterskin).contents_lbs, 1.0, "the ration inside the waterskin");
        assert_eq!(load_of(&loads, &waterskin).capacity, ContainerCapacity::Pounds(4.0));
        assert_eq!(load_of(&loads, &sack).contents_lbs, 1.0, "the ration only: the waterskin and the unknown id weigh nothing");
    }

    /// A stack weighs its pieces (§30) and a Create item its own weight (§25) inside a container,
    /// exactly as `compute_encumbrance` weighs them: 60 CRB `Arrows (20)` (3 lb a bundle) are 9 lb,
    /// a 5 lb created "Food" is 5 lb; the sack holds 14, never one bundle's 3.
    #[test]
    fn containers_weigh_stacks_and_created_items_as_encumbrance_does() {
        use crate::rules_core::encumbrance::{compute_encumbrance, WeighedCopy};
        use crate::rules_core::size::SizeCategory;
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let arrows = key(ARROWS, 0);
        let food = key("custom:food", 0);
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&[arrows.clone(), food.clone()], &sack) };
        let quantities = [(arrows.clone(), 60)];
        let loads = container_loads(&set, &held(&[sack.clone(), arrows.clone()]), &corpus, &[], &[(food, 5.0)], &quantities, &[], &[]);

        let selection = EquipmentSelection { item_id: ARROWS.to_owned(), equipped_or_active: false, active_state: ActiveState::SelectedInactive, applied_modifiers: Vec::new() };
        let food_copy = WeighedCopy { id: "custom:food".to_owned(), weight_lbs: 5.0, active_state: ActiveState::SelectedInactive };
        let encumbrance = compute_encumbrance(&[selection], &corpus, 10, SizeCategory::Medium, &[food_copy], &quantities);
        assert_eq!(encumbrance.total_carried_weight_lbs, 14.0, "{encumbrance:#?}");
        assert_eq!(load_of(&loads, &sack).contents_lbs, encumbrance.total_carried_weight_lbs, "one weight rule, two sums");
    }

    /// A mithral longsword (SD-41 E2.6) weighs half its 4 lb in a sack and on the character: the
    /// copy's applied mithral divides it in both sums.
    #[test]
    fn containers_weigh_a_lightened_copy_as_encumbrance_does() {
        use crate::rules_core::encumbrance::compute_encumbrance;
        use crate::rules_core::size::SizeCategory;
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let sword = key("Longsword (Base)", 0);
        let mithral = vec!["Material ~ Mithril ~ Weapon / Item".to_owned()];
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(std::slice::from_ref(&sword), &sack) };
        let held = [(sword.clone(), sword.item_id.as_str(), mithral.as_slice()), (sack.clone(), SACK, &[][..])];
        let loads = container_loads(&set, &held, &corpus, &[], &[], &[], &[], &[]);
        let selection = EquipmentSelection { item_id: sword.item_id.clone(), equipped_or_active: false, active_state: ActiveState::SelectedInactive, applied_modifiers: mithral.clone() };
        let encumbrance = compute_encumbrance(&[selection], &corpus, 10, SizeCategory::Medium, &[], &[]);
        assert_eq!(encumbrance.total_carried_weight_lbs, 2.0, "{encumbrance:#?}");
        assert_eq!(load_of(&loads, &sack).contents_lbs, 2.0, "one weight rule, two sums");
    }

    /// SD-42 E4.2 (`decisions.md §6, §24`): a copy whose file states its own weight (a `.pcg` line's
    /// `WT`) weighs that per unit in a sack and on the character, its size and material already in
    /// it: a Large longsword written at 0.5 lb weighs 0.5 (not 8), and 60 Arrows (20) written at 1 lb
    /// a bundle weigh 3 (not 9). One weight rule, two sums.
    #[test]
    fn copy_override_weight_in_encumbrance_and_containers() {
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        use crate::rules_core::encumbrance::compute_encumbrance_sized;
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let sword = key("Longsword (Base)", 0);
        let arrows = key(ARROWS, 0);
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&[sword.clone(), arrows.clone()], &sack) };
        let held = [(sword.clone(), sword.item_id.as_str(), &[][..]), (arrows.clone(), ARROWS, &[][..]), (sack.clone(), SACK, &[][..])];
        let sizes = [(sword.clone(), SizeCategory::Large)];
        let quantities = [(arrows.clone(), 60)];
        let overrides = [(sword.clone(), 0.5), (arrows.clone(), 1.0)];
        let loads = container_loads(&set, &held, &corpus, &[], &[], &quantities, &sizes, &overrides);
        assert_eq!(load_of(&loads, &sack).contents_lbs, 3.5, "{loads:?}");
        let carried = |item: &str| EquipmentSelection { item_id: item.to_owned(), equipped_or_active: false, active_state: ActiveState::SelectedInactive, applied_modifiers: Vec::new() };
        let encumbrance = compute_encumbrance_sized(&[carried(&sword.item_id), carried(ARROWS)], &corpus, 10, SizeCategory::Medium, &[], &quantities, &sizes, &overrides, &[]);
        assert_eq!(encumbrance.total_carried_weight_lbs, 3.5, "{encumbrance:#?}");
        assert_eq!(encumbrance.total_carried_cost_gp, 30.0 + 3.0, "the price is not the weight's: the Large sword's 30 gp, three bundles at 1 gp");
    }

    /// A longsword made for a Large wielder (SD-41 E2.12) weighs twice its 4 lb in a sack and on
    /// the character: the copy's size multiplies it in both sums.
    #[test]
    fn containers_weigh_a_sized_copy_as_encumbrance_does() {
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        use crate::rules_core::encumbrance::compute_encumbrance_sized;
        let corpus = core_rulebook();
        let sack = key(SACK, 0);
        let sword = key("Longsword (Base)", 0);
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(std::slice::from_ref(&sword), &sack) };
        let held = [(sword.clone(), sword.item_id.as_str(), &[][..]), (sack.clone(), SACK, &[][..])];
        let sizes = [(sword.clone(), SizeCategory::Large)];
        let loads = container_loads(&set, &held, &corpus, &[], &[], &[], &sizes, &[]);
        let selection = EquipmentSelection { item_id: sword.item_id.clone(), equipped_or_active: false, active_state: ActiveState::SelectedInactive, applied_modifiers: Vec::new() };
        let encumbrance = compute_encumbrance_sized(&[selection], &corpus, 10, SizeCategory::Medium, &[], &[], &sizes, &[], &[]);
        assert_eq!(encumbrance.total_carried_weight_lbs, 8.0, "{encumbrance:#?}");
        assert_eq!(encumbrance.total_carried_cost_gp, 30.0, "the Large copy costs twice the 15 gp");
        assert_eq!(load_of(&loads, &sack).contents_lbs, 8.0, "one weight rule, two sums");
    }

    // SD-42 E4.7 (`decisions.md §27.1`, operator BAG-1 / HVS-1): what sits in a weightless
    // container adds nothing to the carried total; the capacity check still counts it.
    const BAG_I: &str = "item:Bag of Holding (Type I)"; // 15 lb, CONTAINS:*250
    const HAVERSACK: &str = "item:Handy Haversack"; // 5 lb, CONTAINS:*120
    const BACKPACK: &str = "Backpack"; // 2 lb, 60 lb by the §32 override

    /// Every copy of `keys` carried, weighed with the set's weight factors: the carried total, the
    /// factors, and the loads.
    fn weigh_carried(set: &EquipmentSet, keys: &[CopyKey]) -> (f64, Vec<(CopyKey, f64)>, Vec<ContainerLoad>) {
        use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
        use crate::rules_core::encumbrance::compute_encumbrance_sized;
        let corpus = core_rulebook();
        let factors = contents_weight_factors(set, &held(keys), &corpus);
        let selections: Vec<EquipmentSelection> = keys
            .iter()
            .map(|copy| EquipmentSelection { item_id: copy.item_id.clone(), equipped_or_active: false, active_state: ActiveState::SelectedInactive, applied_modifiers: Vec::new() })
            .collect();
        let encumbrance = compute_encumbrance_sized(&selections, &corpus, 10, SizeCategory::Medium, &[], &[], &[], &[], &factors);
        assert!(encumbrance.unresolved_item_ids.is_empty(), "{encumbrance:#?}");
        let loads = container_loads(set, &held(keys), &corpus, &[], &[], &[], &[], &[]);
        (encumbrance.total_carried_weight_lbs, factors, loads)
    }

    /// A Bag of Holding (Type I), 15 lb, holding 300 lb of rations, and a sack beside it: the total
    /// is the bag and the sack, 15.5; the bag's load still reads 300 of 250 lb, over.
    #[test]
    fn weightless_bag_of_holding_contents_add_nothing_to_the_carried_total() {
        let bag = key(BAG_I, 0);
        let rations = copies(RATIONS, 300);
        let mut keys = vec![bag.clone(), key(SACK, 0)];
        keys.extend(rations.clone());
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &bag) };
        let (total, factors, loads) = weigh_carried(&set, &keys);
        assert_eq!(total, 15.5, "the bag's own 15 lb and the sack's 0.5");
        assert_eq!(factors.len(), 300, "each ration inside the bag weighs nothing");
        assert_eq!(load_of(&loads, &bag), &ContainerLoad { copy: bag.clone(), contents_lbs: 300.0, capacity: ContainerCapacity::Pounds(250.0), over: true, contents_weight_pct: 0 });
    }

    /// The Handy Haversack (5 lb, `*120`) holding 100 lb: the total is the haversack's 5.
    #[test]
    fn weightless_handy_haversack_same_rule() {
        let sack = key(HAVERSACK, 0);
        let rations = copies(RATIONS, 100);
        let mut keys = vec![sack.clone()];
        keys.extend(rations.clone());
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &sack) };
        let (total, _, loads) = weigh_carried(&set, &keys);
        assert_eq!(total, 5.0);
        assert_eq!((load_of(&loads, &sack).contents_lbs, load_of(&loads, &sack).capacity, load_of(&loads, &sack).over), (100.0, ContainerCapacity::Pounds(120.0), false));
    }

    /// A Backpack (2 lb) holding 20 lb of rations, inside the bag: the total is the bag's 15 alone;
    /// the backpack's own load is 20 of 60, not over; the bag holds 22.
    #[test]
    fn weightless_nested_backpack_inside_a_bag_adds_nothing() {
        let (bag, pack) = (key(BAG_I, 0), key(BACKPACK, 0));
        let rations = copies(RATIONS, 20);
        let mut keys = vec![bag.clone(), pack.clone()];
        keys.extend(rations.clone());
        let mut locations = inside(&rations, &pack);
        locations.extend(inside(std::slice::from_ref(&pack), &bag));
        let set = EquipmentSet { name: "Default".to_owned(), locations };
        let (total, factors, loads) = weigh_carried(&set, &keys);
        assert_eq!(total, 15.0, "{factors:?}");
        assert_eq!((load_of(&loads, &pack).contents_lbs, load_of(&loads, &pack).over, load_of(&loads, &pack).contents_weight_pct), (20.0, false, 100));
        assert_eq!(load_of(&loads, &bag).contents_lbs, 22.0);
    }

    /// The settled Core Rulebook records read the capacity behind the flag: `*250` → 250 lb, `*120`
    /// → 120 lb, `*UNLIM` → unlimited, each with its contents at 0 %; `UNLIM|Any` (the Sack) stays
    /// unlimited at 100 %. (`40%26|Any` is in no Codex corpus book: the converter's own test holds it.)
    #[test]
    fn weightless_capacity_parses_through_the_flag() {
        use crate::rules_core::equipment_resolver::equipment_converted_resolve;
        let corpus = core_rulebook();
        let read = |id: &str| {
            let record = equipment_converted_resolve(id, &corpus).unwrap_or_else(|| panic!("no record {id}"));
            (record.capacity(), record.contents_weight_pct)
        };
        assert_eq!(read(BAG_I), (ContainerCapacity::Pounds(250.0), 0));
        assert_eq!(read(HAVERSACK), (ContainerCapacity::Pounds(120.0), 0));
        assert_eq!(read("item:Portable Hole"), (ContainerCapacity::Unlimited, 0));
        assert_eq!(read(SACK), (ContainerCapacity::Unlimited, 100));
    }

    /// A plain Sack holding 20 lb changes nothing: the total is the sack and its contents, no factor.
    #[test]
    fn weightless_plain_sack_changes_nothing() {
        let sack = key(SACK, 0);
        let rations = copies(RATIONS, 20);
        let mut keys = vec![sack.clone()];
        keys.extend(rations.clone());
        let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &sack) };
        let (total, factors, loads) = weigh_carried(&set, &keys);
        assert_eq!((total, factors), (20.5, vec![]));
        assert_eq!(load_of(&loads, &sack).contents_weight_pct, 100);
    }
}
