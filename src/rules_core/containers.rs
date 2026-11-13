//! Container loads (SD-41 `decisions.md §7`, §25): how much each container copy in a set holds,
//! against what it can hold.

use std::collections::{BTreeMap, BTreeSet};

use crate::rules_core::equipment_effects::stand_in_record;
use crate::rules_core::equipment_record::ContainerCapacity;
use crate::rules_core::equipment_resolver::equipment_converted_resolve;
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
}

impl ContainerLoad {
    /// `over` only for `Pounds`: `Unlimited` and `Unstated` (DEF-1) are never over.
    pub fn new(copy: CopyKey, contents_lbs: f64, capacity: ContainerCapacity) -> Self {
        let over = matches!(capacity, ContainerCapacity::Pounds(lbs) if contents_lbs > lbs);
        Self { copy, contents_lbs, capacity, over }
    }
}

/// The load of every container copy in `set`: each of `held` (a copy and its item id) whose
/// settled record is a container, in `held` order, then each Create item container (§25) with
/// its capacity in lb. Contents are summed recursively over `Inside` locations (a pouch in a
/// backpack adds to both), each copy weighing what `weight_and_cost()` (or its stand-in record)
/// gives, as encumbrance weighs it; an unweighed or unresolved copy adds nothing (SD-b).
pub fn container_loads(
    set: &EquipmentSet,
    held: &[(CopyKey, &str)],
    corpus: &SourcePackageContent,
    created: &[(CopyKey, f64)],
) -> Vec<ContainerLoad> {
    let mut weights: BTreeMap<&CopyKey, f64> = BTreeMap::new();
    let mut containers: Vec<(CopyKey, ContainerCapacity)> = Vec::new();
    for (copy, item_id) in held {
        let Some(record) = equipment_converted_resolve(item_id, corpus) else { continue };
        let weight = record.weight_and_cost().0.or_else(|| stand_in_record(record, corpus).and_then(|base| base.weight_lbs));
        weights.insert(copy, weight.unwrap_or(0.0));
        if record.is_container() {
            containers.push((copy.clone(), record.capacity()));
        }
    }
    containers.extend(created.iter().map(|(copy, lbs)| (copy.clone(), ContainerCapacity::Pounds(*lbs))));

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
        .map(|(copy, capacity)| {
            let lbs = contents.get(&copy).copied().unwrap_or(0.0);
            ContainerLoad::new(copy, lbs, capacity)
        })
        .collect()
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

    fn held(keys: &[CopyKey]) -> Vec<(CopyKey, &str)> {
        keys.iter().map(|copy| (copy.clone(), copy.item_id.as_str())).collect()
    }

    fn load_of<'a>(loads: &'a [ContainerLoad], copy: &CopyKey) -> &'a ContainerLoad {
        loads.iter().find(|load| &load.copy == copy).unwrap_or_else(|| panic!("no load for {copy:?} in {loads:#?}"))
    }

    const RATIONS: &str = "item:Rations (Trail/Per Day)"; // 1 lb, not a container
    const POUCH: &str = "item:Pouch (Belt)"; // 0.5 lb, Unlimited
    const SACK: &str = "item:Sack"; // 0.5 lb, Unlimited
    const SLED: &str = "item:Sled"; // 300 lb, Pounds(2000)
    const WATERSKIN: &str = "item:Waterskin"; // no WT, Pounds(4)

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

        let loads = container_loads(&set, &held(&keys), &corpus, &created);
        assert_eq!(loads.iter().map(|load| load.copy.clone()).collect::<Vec<_>>(), vec![sled.clone(), pouch.clone(), backpack.clone()], "copies in held order, then Create items");
        assert_eq!(load_of(&loads, &pouch).contents_lbs, 10.0);
        assert_eq!(load_of(&loads, &pouch).capacity, ContainerCapacity::Unlimited);
        assert_eq!(load_of(&loads, &backpack).contents_lbs, 10.5, "the pouch's rations and the pouch itself");
        assert_eq!(load_of(&loads, &backpack).capacity, ContainerCapacity::Pounds(60.0));
        assert_eq!(load_of(&loads, &sled).contents_lbs, 0.0);
        assert_eq!(load_of(&loads, &sled).capacity, ContainerCapacity::Pounds(2000.0), "a stock record's settled capacity");
        assert!(loads.iter().all(|load| !load.over));

        let reversed = EquipmentSet { name: set.name.clone(), locations: set.locations.iter().rev().cloned().collect() };
        assert_eq!(container_loads(&reversed, &held(&keys), &corpus, &created), loads, "deterministic in the set's order");
    }

    /// `over` is contents above a `Pounds` capacity: 60 lb in a 60 lb backpack is not over, 61 is.
    #[test]
    fn containers_over_capacity_flag() {
        let corpus = core_rulebook();
        let backpack = key("custom:backpack", 0);
        for (rations_lbs, over) in [(60, false), (61, true)] {
            let rations = copies(RATIONS, rations_lbs);
            let set = EquipmentSet { name: "Default".to_owned(), locations: inside(&rations, &backpack) };
            let loads = container_loads(&set, &held(&rations), &corpus, &[(backpack.clone(), 60.0)]);
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
        let loads = container_loads(&set, &held(&keys), &corpus, &[]);
        assert_eq!(loads, vec![ContainerLoad { copy: sack, contents_lbs: 500.0, capacity: ContainerCapacity::Unlimited, over: false }]);

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
        let loads = container_loads(&set, &held(&keys), &corpus, &[]);
        assert_eq!(load_of(&loads, &waterskin).contents_lbs, 1.0, "the ration inside the waterskin");
        assert_eq!(load_of(&loads, &waterskin).capacity, ContainerCapacity::Pounds(4.0));
        assert_eq!(load_of(&loads, &sack).contents_lbs, 1.0, "the ration only: the waterskin and the unknown id weigh nothing");
    }
}
