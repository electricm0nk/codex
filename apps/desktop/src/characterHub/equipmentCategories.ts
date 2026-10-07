/**
 * Equipment kinds for the picker's category filter, derived from PCGen `TYPE:` tags
 * (`EquipmentCatalogEntryDto.types`, from `data/equipment_types.json`).
 *
 * The catalog's own `category` has four values, which cannot split thousands of items into
 * browsable kinds. These labels are a presentation grouping of the real tags: first match wins,
 * magic kinds before mundane ones, and a tag this table does not know yields `Other` rather than a
 * guess. A row with no type data at all is `Uncategorized`; equipment modifiers keep their own
 * category whatever their tags.
 */

/** Display order of the category list; every label `equipmentCategoryFor` can return appears here. */
export const EQUIPMENT_CATEGORY_ORDER: string[] = [
  'Weapons',
  'Ammunition',
  'Armor',
  'Shields',
  'Adventuring Gear',
  'Tools & Kits',
  'Containers',
  'Alchemical Items',
  'Poisons',
  'Gems',
  'Spell Components',
  'Trade Goods',
  'Mounts & Animals',
  'Services',
  'Scrolls',
  'Wands',
  'Potions',
  'Rings',
  'Rods',
  'Staves',
  'Wondrous Items',
  'Magic Weapons',
  'Magic Armor',
  'Artifacts',
  'Psionic Items',
  'Technological',
  'Mythic & Legendary',
  'Other Magic',
  'Equipment Mods',
  'Other',
  'Uncategorized',
];

const MAGIC_SECOND_TAG: Record<string, string> = {
  Scroll: 'Scrolls',
  Wand: 'Wands',
  Potion: 'Potions',
  Ring: 'Rings',
  Rod: 'Rods',
  Staff: 'Staves',
  Wondrous: 'Wondrous Items',
  'Wondrous Item': 'Wondrous Items',
  Weapon: 'Magic Weapons',
  Armor: 'Magic Armor',
  Shield: 'Magic Armor',
  Artifact: 'Artifacts',
  'Minor Artifact': 'Artifacts',
};

function magicKind(tags: string[]): string {
  return MAGIC_SECOND_TAG[tags[1] ?? ''] ?? 'Other Magic';
}

export function equipmentCategoryFor(entry: { category: string; types: string[] }): string {
  if (entry.category === 'Equipmods') {
    return 'Equipment Mods';
  }
  const tags = entry.types;
  if (tags.length === 0) {
    return 'Uncategorized';
  }
  const has = (tag: string) => tags.includes(tag);
  const first = tags[0];

  // Magic and other "special" item families first: a magic weapon is a Magic Weapon, not a Weapon.
  if (first === 'Magic') return magicKind(tags);
  if (first === 'Artifact') return 'Artifacts';
  if (first === 'Psionic') return 'Psionic Items';
  if (first === 'Technological' || first === 'Cybertech') return 'Technological';
  if (first === 'Mythic' || first === 'Legendary') return 'Mythic & Legendary';
  if (first === 'Wondrous' || first === 'Wondrous Item' || first === 'Ioun' || first === 'Amulet of Mighty Fists') return 'Wondrous Items';
  if (first === 'Rod') return 'Rods';
  if (first === 'SLOT_Ring') return 'Rings';

  // Mundane kinds, most specific first.
  if (has('Ammunition')) return 'Ammunition';
  if (has('Shield')) return 'Shields';
  if (has('Armor')) return 'Armor';
  if (has('Weapon') || has('Firearm') || has('Siege')) return 'Weapons';
  if (has('Gem')) return 'Gems';
  if (has('Spell Component')) return 'Spell Components';
  if (has('Poison')) return 'Poisons';
  if (has('Alchemical')) return 'Alchemical Items';
  if (has('Mount') || has('Animal') || has('Barding')) return 'Mounts & Animals';
  if (has('Service')) return 'Services';
  if (has('Trade')) return 'Trade Goods';
  if (has('Tools') || has('Kit')) return 'Tools & Kits';
  if (has('Container')) return 'Containers';
  if (has('Goods')) return 'Adventuring Gear';
  return 'Other';
}
