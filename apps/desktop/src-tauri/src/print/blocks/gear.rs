//! Weapons, and section 3's Defense, Gear and Money (SD-40 E3.2). Weapons print page 1's totals (SD-40 E2.10): the attack total and
//! the engine's damage total (`1d8+4`, E2.14), the dice literal, nothing summed here. The
//! Inventory lists every owned stack off `list_held_equipment_stacks_at_root` where the active set
//! puts it -- worn, carried, in a container or Not Carried (SD-41 E4.1) -- and a copy not carried
//! is not weighed.

use super::{derivation, override_tag, pounds, trim_number, words_of, SheetSources};
use crate::character_hub::{ActiveStateDto, ContainerCapacityDto, ContainerLoadDto, HeldEquipmentCopyDto, WeaponDamageDto};
use codex::rules_core::body_slots::BodySlot;
use codex::saved_character::equipment_sets::Location;
use crate::print::layout::{signed, NO_VALUE};
use crate::print::model::{Look, SheetBlock, SheetRow};

/// The engine's per-weapon rows behind its weapon's line (the attack and damage totals are its
/// cells; the damage bonus and threat range print in Show the math): (id prefix, caption).
const WEAPON_ROWS: [(&str, &str); 4] = [
    ("combat.weapon_attack_bonus.", "Attack bonus"),
    ("combat.weapon_damage_total.", "Damage"),
    ("combat.weapon_damage_bonus.", "Damage bonus"),
    ("combat.weapon_threat_range_low.", "Threat range from"),
];

/// The key the engine files a weapon's rows under: its record's name without the `item:` prefix
/// and the ` (Base)` suffix, letters and digits only, lower-cased (`Longsword (Base)` reads
/// `longsword`), as `normalize_weapon_identity` in the rules core files them.
fn weapon_row_key(record: &str) -> String {
    let record = record.strip_prefix("item:").unwrap_or(record);
    record.strip_suffix(" (Base)").unwrap_or(record).chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect()
}

/// A per-weapon row as its cell prints it: the damage total as the engine prints it (its detail
/// opens with `1d8+4:`), the threat range's first number as is, a bonus signed.
fn weapon_cell(prefix: &str, row: Option<&crate::character_hub::ExplanationDto>) -> String {
    match row {
        Some(row) if prefix.ends_with("damage_total.") => row.detail.split_once(':').map_or_else(|| row.detail.clone(), |(printed, _)| printed.to_owned()),
        Some(row) if prefix.ends_with("threat_range_low.") => row.value.to_string(),
        Some(row) => signed(row.value),
        None => NO_VALUE.to_owned(),
    }
}

fn optional_signed(value: Option<i16>) -> String {
    value.map_or_else(|| NO_VALUE.to_owned(), signed)
}

/// The name the sheet gives an equipped item: its resolved record's name, else its record key.
fn item_name(sources: &SheetSources, item_id: &str, record_key: Option<&str>) -> String {
    sources
        .loaded
        .corpus_derived
        .equipped_items
        .iter()
        .find(|item| item.item_id == item_id)
        .map(|item| item.equipment_record_name.clone())
        .or_else(|| record_key.map(str::to_owned))
        .unwrap_or_else(|| item_id.to_owned())
}

/// A carried weapon's line says so (SD-41 `decisions.md §9, §33`: never an unmarked carried
/// weapon), and a nonlethal strike's (the Unarmed row, `decisions.md §22`).
fn marked(name: String, weapon: &WeaponDamageDto) -> String {
    let carried = if weapon.in_hand { "" } else { " (carried)" };
    let nonlethal = if weapon.nonlethal { " (nonlethal)" } else { "" };
    format!("{name}{carried}{nonlethal}")
}

/// What a weapon that fires ammunition prints beside its tick boxes: `Arrow: 60 carried`, the
/// engine's count of matching pieces carried (`decisions.md §23, §28`); empty when it counts none.
fn ammunition_words(weapon: &WeaponDamageDto) -> String {
    if weapon.ammunition_carried == 0 {
        return String::new();
    }
    format!("{}: {} carried", weapon.ammunition_family.as_deref().unwrap_or("Ammunition"), weapon.ammunition_carried)
}

fn critical(weapon: &WeaponDamageDto) -> String {
    let range = weapon.critical_threat_range.map(|[low, high]| if low == high { low.to_string() } else { format!("{low}-{high}") });
    let multiplier = weapon.critical_multiplier.map(|m| format!("x{m}"));
    match (range, multiplier) {
        (Some(range), Some(multiplier)) => format!("{range}/{multiplier}"),
        (Some(one), None) | (None, Some(one)) => one,
        (None, None) => NO_VALUE.to_owned(),
    }
}

/// The Weapons block (SD-40 E2.3; E2.10 totals only, `decisions.md §23.1` item 4): one line per
/// weapon in hand, its cells the attack total (`combat.weapon_attack_bonus.<weapon>`), the damage
/// total (`combat.weapon_damage_total.<weapon>`), the critical and the range increment, then its feat
/// notes; the engine's four per-weapon rows are behind that line (Show the math), not lines of their
/// own. A per-weapon row no weapon in hand matches still prints, as a line of its own. The engine
/// orders the lines: in hand, then carried "(carried)", then the Unarmed strike last (SD-41
/// `decisions.md §9, §22`). A weapon that fires ammunition is a line of kind `ammunition` with a
/// fifth value, its count's words (`decisions.md §23`); the `weapons` look prints its 20 tick boxes
/// under it. Only these lines print: no blank line, no empty box (SD-41 `decisions.md §34` (5)).
pub fn weapons(sources: &SheetSources) -> SheetBlock {
    let mut rows = vec![SheetRow::new("Weapon", ["Attack", "Damage", "Critical", "Range"].map(str::to_owned))];
    let mut placed: Vec<&str> = Vec::new();
    for weapon in &sources.loaded.weapon_damage {
        let name = item_name(sources, &weapon.weapon_item_id, weapon.weapon_record_key.as_deref());
        let key = weapon_row_key(weapon.weapon_record_key.as_deref().unwrap_or(&name));
        let found: Vec<(&str, Option<&crate::character_hub::ExplanationDto>)> = WEAPON_ROWS
            .iter()
            .map(|(prefix, _)| (*prefix, sources.loaded.explanations.iter().find(|row| row.id.strip_prefix(prefix) == Some(key.as_str()))))
            .collect();
        placed.extend(found.iter().filter_map(|(_, row)| row.map(|row| row.id.as_str())));
        let range = weapon.range_increment.map_or_else(|| NO_VALUE.to_owned(), |feet| format!("{feet} ft."));
        let notes = weapon.feat_effects.iter().map(|effect| format!("{}: {} damage", effect.feat_key, signed(effect.damage_bonus)));
        let [attack, damage] = [0, 1].map(|index| weapon_cell(found[index].0, found[index].1));
        let mut values = vec![attack, damage, critical(weapon), range];
        if weapon.uses_ammunition {
            values.push(ammunition_words(weapon));
        }
        let line = SheetRow::new(marked(name, weapon), values).with_notes(notes).derived(found.iter().filter_map(|(_, row)| row.map(derivation)).collect());
        rows.push(if weapon.uses_ammunition { line.of_kind("ammunition") } else { line });
    }
    for row in &sources.loaded.explanations {
        if placed.contains(&row.id.as_str()) {
            continue;
        }
        for (prefix, label) in WEAPON_ROWS {
            if let Some(weapon) = row.id.strip_prefix(prefix) {
                rows.push(SheetRow::new(format!("{label} ({})", weapon.replace('_', " ")), [weapon_cell(prefix, Some(row))]).derived(vec![derivation(row)]));
            }
        }
    }
    if rows.len() == 1 {
        rows.push(SheetRow::new("No weapon is wielded.", Vec::<String>::new()));
    }
    SheetBlock::new("Weapons", rows).in_look(Look::Weapons)
}

/// Show the math's Weapons breakdown (SD-40 E3.5, `decisions.md §23.1` item 4): a line per weapon in
/// hand, its cells the ones page 1 dropped at E2.10: the attack total, the damage dice, the Strength
/// and enhancement damage, the damage bonus (`combat.weapon_damage_bonus.<weapon>`), the threat
/// range's first number, the critical and the wield; then each feat's damage note under it.
pub(super) fn weapons_breakdown(sources: &SheetSources) -> SheetBlock {
    let mut rows = vec![super::core::captions(&["Attack", "Dice", "Str", "Enh.", "Damage", "Threat", "Critical", "Wield"])];
    for weapon in &sources.loaded.weapon_damage {
        let name = item_name(sources, &weapon.weapon_item_id, weapon.weapon_record_key.as_deref());
        let key = weapon_row_key(weapon.weapon_record_key.as_deref().unwrap_or(&name));
        let cell = |prefix: &str| weapon_cell(prefix, sources.loaded.explanations.iter().find(|row| row.id.strip_prefix(prefix) == Some(key.as_str())));
        let dice = weapon.base_dice.as_ref().map_or_else(|| NO_VALUE.to_owned(), |dice| format!("{}d{}", dice.count, dice.die_size));
        let wield = weapon.wield_category.as_deref().map_or_else(|| NO_VALUE.to_owned(), spaced_words);
        let notes = weapon.feat_effects.iter().map(|effect| format!("{}: {} damage", effect.feat_key, signed(effect.damage_bonus)));
        rows.push(
            SheetRow::new(
                marked(name, weapon),
                [
                    cell("combat.weapon_attack_bonus."),
                    dice,
                    optional_signed(weapon.str_damage_modifier),
                    optional_signed(weapon.enhancement_damage_bonus),
                    cell("combat.weapon_damage_bonus."),
                    cell("combat.weapon_threat_range_low."),
                    critical(weapon),
                    wield,
                ],
            )
            .with_notes(notes),
        );
    }
    if rows.len() == 1 {
        rows.push(SheetRow::new("No weapon is wielded.", Vec::<String>::new()));
    }
    SheetBlock::new("Weapon Breakdown", rows).in_look(Look::Breakdown)
}

/// `OneHanded` reads `One Handed`.
fn spaced_words(raw: &str) -> String {
    let mut out = String::new();
    for (index, ch) in raw.chars().enumerate() {
        if index > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

/// The Defense block (SD-40 E3.2, section 3 Inventory): the armor and shield table, a line per worn
/// piece (AC bonus, max Dex, check, failure, and the weight the engine carries it at), then the
/// worn totals the engine states. The `defense` look adds one blank row.
pub fn armor(sources: &SheetSources) -> SheetBlock {
    let effects = &sources.loaded.corpus_derived.equipment_effects;
    let weights = &sources.loaded.corpus_derived.encumbrance.per_item;
    let mut rows = vec![SheetRow::new("Armor or shield", ["AC", "Max Dex", "Check", "Failure", "Weight"].map(str::to_owned)).of_kind("captions")];
    for item in effects.per_item.iter().filter(|item| item.armor_class_bonus.is_some()) {
        let weight = weights.iter().find(|carried| carried.item_id == item.item_id).map_or_else(|| NO_VALUE.to_owned(), |carried| pounds(carried.weight_lbs));
        rows.push(
            SheetRow::new(
                item_name(sources, &item.item_id, Some(&item.equipment_record_key)),
                [
                    optional_signed(item.armor_class_bonus),
                    item.max_dex.map_or_else(|| NO_VALUE.to_owned(), signed),
                    item.armor_check_penalty.map_or_else(|| NO_VALUE.to_owned(), |v| v.to_string()),
                    item.spell_failure.map_or_else(|| NO_VALUE.to_owned(), |v| format!("{}%", trim_number(f64::from(v)))),
                    weight,
                ],
            )
            .derived(Vec::new()),
        );
    }
    if rows.len() == 1 {
        rows.push(SheetRow::new("No armor or shield is worn.", Vec::<String>::new()));
    } else {
        rows.push(SheetRow::new("Armor check penalty, all worn", [effects.armor_check_penalty_total.to_string()]).of_kind("total").derived(Vec::new()));
        if let Some(cap) = effects.max_dex_cap {
            rows.push(SheetRow::new("Max Dex, all worn", [signed(cap)]).of_kind("total").derived(Vec::new()));
        }
        if let Some(chance) = effects.spell_failure_chance {
            rows.push(SheetRow::new("Arcane spell failure, all worn", [format!("{}%", trim_number(f64::from(chance)))]).of_kind("total").derived(Vec::new()));
        }
    }
    SheetBlock::new("Armor and Shields", rows).in_look(Look::Defense)
}

/// An item line's kind by how deep it sits under containers: `item` at the top of its section,
/// `in-<n>` under `n` containers (the `gear` look indents by `n`).
// shortcut: a line nested deeper than 5 prints at depth 5; add kinds if sets ever nest deeper.
const NESTING: [&str; 6] = ["item", "in-1", "in-2", "in-3", "in-4", "in-5"];

/// `"<name> ×<n>"` in pieces; a bundle's `(20)` is dropped once bought (`gearScreenModel.ts`'s
/// `stackLabel`, SD-41 `decisions.md §28, §30`). A renamed copy prints its record's name beside
/// its own, `Camping Gear (Sack)` (SD-42 `decisions.md §27.3`).
fn stack_label(copy: &HeldEquipmentCopyDto, quantity: u32) -> String {
    let base = match copy.base_name.trim_end().strip_suffix(')').and_then(|rest| rest.rsplit_once('(')) {
        Some((name, count)) if copy.base_quantity > 1 && !count.is_empty() && count.chars().all(|c| c.is_ascii_digit()) => name.trim_end(),
        _ => copy.base_name.as_str(),
    };
    let name = if copy.name == copy.base_name { base.to_owned() } else { format!("{} ({base})", copy.name) };
    if quantity > 1 { format!("{name} ×{quantity}") } else { name }
}

/// A row's weight as the engine weighs each copy: `3 lb`, `3 × 3 lb` for a stack of like copies,
/// `2 lb + 3 lb` for unlike ones; the no-value mark when the engine weighed not every copy. Nothing
/// is summed here (NR-2).
fn weight_words(weights: &[f64], copies: usize) -> String {
    match weights {
        _ if weights.len() != copies || weights.is_empty() => NO_VALUE.to_owned(),
        [one] => pounds(*one),
        [first, ..] if weights.iter().all(|weight| weight == first) => format!("{copies} × {}", pounds(*first)),
        _ => weights.iter().map(|weight| pounds(*weight)).collect::<Vec<_>>().join(" + "),
    }
}

/// A container copy's load as the engine states it (`gearScreenModel.ts`'s `capacityLabel`), with the
/// over-capacity flag unless the character ignores container capacity limits (`decisions.md §32`).
fn load_note(load: &ContainerLoadDto, ignore_capacity: bool) -> String {
    let contents = trim_number(load.contents_lbs);
    let holds = match load.capacity {
        ContainerCapacityDto::Pounds { lbs } => format!("Holds {contents} of {} lb", trim_number(lbs)),
        ContainerCapacityDto::Unlimited => format!("Holds {contents} lb, no limit"),
        ContainerCapacityDto::Unstated => format!("Holds {contents} lb"),
    };
    if load.over && !ignore_capacity { format!("{holds}, over capacity") } else { holds }
}

/// The Gear block's lines, located as the active set places each stack (SD-41 `decisions.md §9`).
struct Lines<'a> {
    held: &'a [HeldEquipmentCopyDto],
    /// Each stack's copies' weights, as the engine weighs them.
    weights: Vec<Vec<f64>>,
    loads: &'a [ContainerLoadDto],
    ignore_capacity: bool,
    placed: Vec<bool>,
}

impl<'a> Lines<'a> {
    /// The stack `index` (its slot's words in `slot`) and, under each container copy of it, what that
    /// copy holds one level deeper. `weighed` is false under Not Carried: names only.
    fn push(&mut self, index: usize, slot: String, depth: usize, weighed: bool, out: &mut Vec<SheetRow>) {
        self.placed[index] = true;
        let (held, loads): (&'a [HeldEquipmentCopyDto], &'a [ContainerLoadDto]) = (self.held, self.loads);
        let row = &held[index];
        let kind = NESTING[depth.min(NESTING.len() - 1)];
        let containers: Vec<(usize, &ContainerLoadDto)> =
            row.copies.iter().enumerate().filter_map(|(at, copy)| loads.iter().find(|load| &load.copy == copy).map(|load| (at, load))).collect();
        // The copy's properties print under it as text (SD-42 `decisions.md §6`), weighed or not.
        let properties = row.properties.iter().cloned();
        if containers.is_empty() {
            let line = if weighed {
                SheetRow::new(stack_label(row, row.quantity), [slot, weight_words(&self.weights[index], row.copies.len())]).with_notes(row.override_flag.map(override_tag).into_iter().chain(properties))
            } else {
                SheetRow::new(stack_label(row, row.quantity), Vec::<String>::new()).with_notes(properties)
            };
            out.push(line.of_kind(kind));
            return;
        }
        // A container prints one line per copy, so each holds its own contents (`decisions.md §38`).
        for (at, load) in containers {
            let name = stack_label(row, 1);
            let label = if row.copies.len() > 1 { format!("{name} #{}", at + 1) } else { name };
            let line = if weighed {
                let weight = self.weights[index].get(at).filter(|_| self.weights[index].len() == row.copies.len()).map_or_else(|| NO_VALUE.to_owned(), |lbs| pounds(*lbs));
                SheetRow::new(label, [slot.clone(), weight]).with_notes(row.override_flag.map(override_tag).into_iter().chain([load_note(load, self.ignore_capacity)]).chain(properties.clone()))
            } else {
                SheetRow::new(label, Vec::<String>::new()).with_notes(properties.clone())
            };
            out.push(line.of_kind(kind));
            let contents: Vec<usize> = (0..self.held.len())
                .filter(|other| !self.placed[*other] && matches!(&self.held[*other].location, Location::Inside { container } if *container == load.copy))
                .collect();
            for other in contents {
                if !self.placed[other] {
                    self.push(other, String::new(), depth + 1, weighed, out);
                }
            }
        }
    }
}

/// The Gear block (SD-40 E3.2; SD-41 E4.1, `decisions.md §9` A7): the active set by location. Worn:
/// each slot in use in body-slot order, the slot's words beside the copy (a slotless worn copy reads
/// `Equipped`); then Carried; each container's contents under it, one level deeper per nesting,
/// the container's load beside it; then Not Carried, names only. Identical copies print as one
/// stack (`<name> ×<pieces>`), each line with the weight the engine weighs its copies at. Then the
/// carried weight, the load and the three limits, the engine's rows. A copy's properties (SD-42
/// E4.2) print as notes under its line. The `gear` look indents the
/// nested lines and adds two blank rows per section.
pub fn inventory(sources: &SheetSources) -> SheetBlock {
    let encumbrance = &sources.loaded.corpus_derived.encumbrance;
    let held = &sources.held;
    // The engine weighs each copy it carries, in saved order; a copy not carried is not weighed.
    let mut weighed: Vec<Option<&crate::character_hub::CarriedItemDto>> = encumbrance.per_item.iter().map(Some).collect();
    let weights = held
        .iter()
        .map(|stack| {
            if stack.active_state == ActiveStateDto::Absent {
                return Vec::new();
            }
            stack
                .copies
                .iter()
                .filter_map(|_| weighed.iter_mut().find(|slot| slot.is_some_and(|item| item.item_id == stack.item_id)).and_then(Option::take).map(|item| item.weight_lbs))
                .collect()
        })
        .collect();
    let mut lines = Lines { held, weights, loads: &sources.loaded.container_loads, ignore_capacity: sources.loaded.ignore_container_capacity, placed: vec![false; held.len()] };
    let worn_slot = |stack: &HeldEquipmentCopyDto| match stack.location {
        Location::Slot { slot } => Some(slot),
        Location::Equipped => Some(BodySlot::None),
        _ => None,
    };
    let mut worn: Vec<(BodySlot, usize)> = held.iter().enumerate().filter_map(|(index, stack)| worn_slot(stack).map(|slot| (slot, index))).collect();
    worn.sort_by_key(|(slot, _)| *slot);
    let mut sections: [(&str, Vec<SheetRow>); 3] = [("Worn", Vec::new()), ("Carried", Vec::new()), ("Not Carried", Vec::new())];
    for (slot, index) in worn {
        let words = if slot == BodySlot::None { "Equipped".to_owned() } else { slot_words(slot) };
        lines.push(index, words, 0, true, &mut sections[0].1);
    }
    for (section, location, weighs) in [(1, Location::Carried, true), (2, Location::NotCarried, false)] {
        for (index, stack) in held.iter().enumerate() {
            if !lines.placed[index] && stack.location == location {
                lines.push(index, String::new(), 0, weighs, &mut sections[section].1);
            }
        }
    }
    // A copy inside a container no stack holds still prints, under Carried.
    for (index, stack) in held.iter().enumerate() {
        if !lines.placed[index] {
            lines.push(index, String::new(), 0, stack.active_state != ActiveStateDto::Absent, &mut sections[1].1);
        }
    }
    let mut rows = vec![SheetRow::new("Item", ["Slot".to_owned(), "Weight".to_owned()]).of_kind("captions")];
    for (title, items) in sections {
        rows.push(SheetRow::new(title, Vec::<String>::new()).of_kind("section"));
        rows.extend(items);
    }
    rows.extend(
        [
            SheetRow::new("Carried weight", [pounds(encumbrance.total_carried_weight_lbs)]).derived(Vec::new()),
            SheetRow::new("Load", [encumbrance.level.clone()]),
            SheetRow::new("Light load up to", [pounds(encumbrance.light_max_lbs)]).derived(Vec::new()),
            SheetRow::new("Medium load up to", [pounds(encumbrance.medium_max_lbs)]).derived(Vec::new()),
            SheetRow::new("Heavy load up to", [pounds(encumbrance.heavy_max_lbs)]).derived(Vec::new()),
        ]
        .map(|row| row.of_kind("total")),
    );
    SheetBlock::new("Gear", rows).in_look(Look::Gear)
}

/// The Money block (SD-40 E3.2, section 3 Inventory): the Gear tab's four coin tiles, as
/// `load_character_money` breaks the saved copper down; an empty box where there is none. The
/// `money` look adds the four-line ledger.
pub fn money(sources: &SheetSources) -> SheetBlock {
    let coins = sources.money.as_ref().map(|money| [money.platinum, money.gold, money.silver, money.copper]);
    let rows = ["PP", "GP", "SP", "CP"]
        .into_iter()
        .enumerate()
        .map(|(index, label)| SheetRow::new(label, [coins.map_or_else(String::new, |coins| coins[index].to_string())]))
        .collect();
    SheetBlock::new("Money", rows).in_look(Look::Money)
}

/// A body slot in words (`wield_main` reads `Wield main`).
fn slot_words(slot: BodySlot) -> String {
    serde_json::to_value(slot).ok().and_then(|slot| slot.as_str().map(words_of)).unwrap_or_default()
}
