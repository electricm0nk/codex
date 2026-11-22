/**
 * The Market's filter columns, per list: which values each column offers and which rows each value
 * holds, read from the row's PCGen `TYPE:` tags (`EquipmentCatalogEntryDto.types`). A presentation
 * grouping of real tags, like `equipmentCategories.ts`, which the Gear list reuses as is.
 *
 * The two-tag rule: a row is under every value of a column whose tags it carries (a piercing-and-
 * slashing weapon is under Piercing and under Slashing), and under `Other` when it carries none, so
 * per column the value counts plus `Other` cover every row and may add up to more than the rows.
 * Values were confirmed against the shown rows' tag counts (SD-43 E0.2's `market-rows.jsonl`).
 */
import type { MarketCategory } from '../boundary/listMarket';
import { EQUIPMENT_CATEGORY_ORDER, equipmentCategoryFor } from './equipmentCategories';

export const OTHER = 'Other';

export interface FilterEntry {
  category: string;
  types: string[];
}

export interface FilterValue {
  label: string;
  matches: (entry: FilterEntry) => boolean;
}

export interface FilterColumn {
  label: string;
  values: FilterValue[];
}

/** A value held by every row carrying any of `tags` (case-insensitive; `SLOT_Neck` reads `Neck`). */
function tagValue(label: string, ...tags: string[]): FilterValue {
  const wanted = new Set(tags.map((tag) => tag.toLowerCase()));
  return {
    label,
    matches: (entry) => entry.types.some((tag) => wanted.has(tag.toLowerCase().replace(/^slot_/, ''))),
  };
}

function column(label: string, values: FilterValue[]): FilterColumn {
  return { label, values };
}

// `Other` and `Uncategorized` are this model's `Other`, not values of their own.
const GEAR_KINDS = column(
  'Kind',
  EQUIPMENT_CATEGORY_ORDER.filter((label) => label !== 'Other' && label !== 'Uncategorized').map((label) => ({
    label,
    matches: (entry: FilterEntry) => equipmentCategoryFor(entry) === label,
  })),
);

export const MARKET_FILTERS: Record<MarketCategory, FilterColumn[]> = {
  weapon: [
    column('Kind', [
      tagValue('Melee', 'Melee'),
      tagValue('Ranged', 'Ranged'),
      tagValue('Thrown', 'Thrown'),
      // The ammunition tags `market_category_of` files under Weapons.
      tagValue('Ammunition', 'Ammunition', 'Ammo', 'Stack'),
    ]),
    column('Damage', [tagValue('Bludgeoning', 'Bludgeoning'), tagValue('Piercing', 'Piercing'), tagValue('Slashing', 'Slashing')]),
    column('Skill', [tagValue('Simple', 'Simple'), tagValue('Martial', 'Martial'), tagValue('Exotic', 'Exotic')]),
    column('Hands', [tagValue('Light', 'Light'), tagValue('One-handed', 'OneHanded'), tagValue('Two-handed', 'TwoHanded')]),
  ],
  armor: [column('Weight', [tagValue('Light', 'Light'), tagValue('Medium', 'Medium'), tagValue('Heavy', 'Heavy')])],
  shield: [column('Kind', [tagValue('Light', 'Light'), tagValue('Heavy', 'Heavy'), tagValue('Tower', 'Tower'), tagValue('Buckler', 'Buckler')])],
  gear: [GEAR_KINDS],
  magicItem: [
    column('Kind', [
      tagValue('Armor', 'Armor'),
      tagValue('Shield', 'Shield'),
      tagValue('Weapon', 'Weapon', 'Ammunition'),
      tagValue('Potion', 'Potion', 'Elixir'),
      tagValue('Ring', 'Ring'),
      tagValue('Rod', 'Rod'),
      tagValue('Scroll', 'Scroll'),
      tagValue('Staff', 'Staff'),
      tagValue('Wand', 'Wand'),
      tagValue('Wondrous', 'Wondrous', 'Wondrous Item'),
      tagValue('Artifact', 'Artifact', 'Minor Artifact'),
      tagValue('Psionic', 'Psionic'),
    ]),
    column('Slot', [
      tagValue('Belt', 'Belt'),
      tagValue('Body', 'Body', 'Robe'),
      tagValue('Chest', 'Chest', 'Shirt'),
      tagValue('Eyes', 'Eyes', 'Eyegear'),
      tagValue('Feet', 'Feet', 'Boot'),
      tagValue('Hands', 'Hands', 'Glove'),
      tagValue('Head', 'Head', 'Helm', 'Hat'),
      tagValue('Headband', 'Headband'),
      tagValue('Neck', 'Neck', 'Amulet', 'Necklace'),
      tagValue('Ring', 'Ring'),
      tagValue('Shoulders', 'Shoulders', 'Cloak'),
      tagValue('Wrists', 'Wrist', 'Wrists', 'Bracer'),
      tagValue('Slotless', 'None'),
    ]),
  ],
};

/** The values of `col` a row is under: every one its tags match, else `Other`. */
export function filterValuesOf(col: FilterColumn, entry: FilterEntry): string[] {
  const hit = col.values.filter((value) => value.matches(entry)).map((value) => value.label);
  return hit.length > 0 ? hit : [OTHER];
}

/** The values a column offers over these rows, in declared order, `Other` last; a value with no row is left out. */
export function filterOptions(col: FilterColumn, entries: ReadonlyArray<FilterEntry>): Array<{ label: string; count: number }> {
  const counts = new Map<string, number>();
  for (const entry of entries) {
    for (const label of filterValuesOf(col, entry)) counts.set(label, (counts.get(label) ?? 0) + 1);
  }
  return [...col.values.map((value) => value.label), OTHER]
    .filter((label) => counts.has(label))
    .map((label) => ({ label, count: counts.get(label) ?? 0 }));
}

/** Rows under every chosen value; a column with no choice (absent or `null`) is `All`. */
export function applyMarketFilters<T extends FilterEntry>(
  category: MarketCategory,
  entries: ReadonlyArray<T>,
  chosen: Readonly<Record<string, string | null | undefined>>,
): T[] {
  const active = MARKET_FILTERS[category].filter((col) => chosen[col.label]);
  return entries.filter((entry) => active.every((col) => filterValuesOf(col, entry).includes(chosen[col.label] as string)));
}
