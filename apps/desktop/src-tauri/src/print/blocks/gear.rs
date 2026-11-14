//! Weapons, and section 3's Defense, Gear and Money (SD-40 E3.2). Weapons print page 1's totals (SD-40 E2.10): the attack total and
//! the engine's damage total (`1d8+4`, E2.14), the dice literal, nothing summed here. The
//! Inventory lists every owned copy off `list_held_equipment_at_root` -- worn, carried or left
//! behind (E7.5, operator ruling R21) -- and a copy left behind weighs nothing.

use super::{derivation, override_tag, pounds, trim_number, words_of, SheetSources};
use crate::character_hub::{ActiveStateDto, WeaponDamageDto};
use codex::rules_core::body_slots::BodySlot;
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
/// own. A per-weapon row no weapon in hand matches still prints, as a line of its own. The `weapons`
/// look adds the blank lines and the ammunition write-in.
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
        rows.push(
            SheetRow::new(name, [attack, damage, critical(weapon), range])
                .with_notes(notes)
                .derived(found.iter().filter_map(|(_, row)| row.map(derivation)).collect()),
        );
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
                name,
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

/// The Gear block (SD-40 E3.2, section 3 Inventory): every owned copy off `list_held_equipment_at_root`
/// under one of three sections, Worn (in body-slot order, its slot beside it), Carried and Left
/// behind, each with the weight the engine carries it at (a copy left behind weighs nothing); then
/// the carried weight, the load and the three limits. The `gear` look adds two blank rows per section.
pub fn inventory(sources: &SheetSources) -> SheetBlock {
    let encumbrance = &sources.loaded.corpus_derived.encumbrance;
    // The engine weighs each copy it carries, in selection order; a copy left behind is not weighed.
    let mut weighed: Vec<Option<&crate::character_hub::CarriedItemDto>> = encumbrance.per_item.iter().map(Some).collect();
    let mut sections: [(&str, Vec<(BodySlot, SheetRow)>); 3] = [("Worn", Vec::new()), ("Carried", Vec::new()), ("Left behind", Vec::new())];
    for copy in &sources.held {
        let (section, slot, weight) = match copy.active_state {
            ActiveStateDto::Absent => (2, String::new(), "no weight".to_owned()),
            ActiveStateDto::EquippedActive | ActiveStateDto::SelectedInactive => {
                let weight = weighed
                    .iter_mut()
                    .find(|slot| slot.is_some_and(|item| item.item_id == copy.item_id))
                    .and_then(Option::take)
                    .map_or_else(|| NO_VALUE.to_owned(), |item| pounds(item.weight_lbs));
                if copy.active_state == ActiveStateDto::EquippedActive {
                    (0, slot_words(copy).unwrap_or_default(), weight)
                } else {
                    (1, String::new(), weight)
                }
            }
        };
        let row = SheetRow::new(copy.name.clone(), [slot, weight]).of_kind("item").with_notes(copy.override_flag.map(override_tag));
        sections[section].1.push((copy.slot, row));
    }
    sections[0].1.sort_by_key(|(slot, _)| *slot);
    let mut rows = vec![SheetRow::new("Item", ["Slot".to_owned(), "Weight".to_owned()]).of_kind("captions")];
    for (title, items) in sections {
        rows.push(SheetRow::new(title, Vec::<String>::new()).of_kind("section"));
        rows.extend(items.into_iter().map(|(_, row)| row));
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

/// Where a worn copy sits, in words (`wield_main` reads `Wield main`); none for a slotless item.
fn slot_words(copy: &crate::character_hub::HeldEquipmentCopyDto) -> Option<String> {
    let slot = serde_json::to_value(copy.slot).ok()?;
    let slot = slot.as_str()?;
    (slot != "none").then(|| words_of(slot))
}
