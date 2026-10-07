import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assert, assertEqual } from '../testSupport/asserts';
import { EQUIPMENT_CATEGORY_ORDER, categoryTabs, equipmentCategoryFor } from './equipmentCategories';

const cat = (types: string[], category = 'MagicItems') => equipmentCategoryFor({ category, types });

function verifiesCategoryTabsListOnlyKindsThatHaveItemsInDisplayOrder() {
  const entry = (category: string, types: string[]) => ({ category, types });
  const tabs = categoryTabs([
    entry('MagicItems', ['Magic', 'Ring']),
    entry('ArmsArmor', ['Weapon', 'Melee']),
    entry('ArmsArmor', ['Weapon', 'Melee']),
    entry('Equipmods', []),
  ]);
  assertEqual(tabs.map((tab) => `${tab.label}:${tab.count}`).join('|'), 'Weapons:2|Rings:1|Equipment Mods:1', 'display order, counts, no empty kinds');
  assertEqual(categoryTabs([]).length, 0, 'nothing loaded, no tabs');
}

function verifiesMagicItemKinds() {
  assertEqual(cat(['Magic', 'Scroll', 'Arcane', 'Consumable', 'Combat Gear']), 'Scrolls', 'arcane scroll');
  assertEqual(cat(['Magic', 'Wand', 'Combat Gear']), 'Wands', 'wand');
  assertEqual(cat(['Magic', 'Potion', 'Consumable']), 'Potions', 'potion');
  assertEqual(cat(['Magic', 'Ring']), 'Rings', 'ring');
  assertEqual(cat(['Magic', 'Rod', 'Combat Gear']), 'Rods', 'rod');
  assertEqual(cat(['Magic', 'Staff']), 'Staves', 'staff');
  assertEqual(cat(['Magic', 'Wondrous', 'Belt']), 'Wondrous Items', 'wondrous');
  assertEqual(cat(['Wondrous', 'Ioun']), 'Wondrous Items', 'a wondrous first tag');
  assertEqual(cat(['Magic', 'Weapon']), 'Magic Weapons', 'magic weapon');
  assertEqual(cat(['Magic', 'Armor']), 'Magic Armor', 'magic armor');
  assertEqual(cat(['Artifact']), 'Artifacts', 'artifact');
  assertEqual(cat(['Psionic', 'Wondrous']), 'Psionic Items', 'psionic');
}

function verifiesMundaneKinds() {
  const g = 'General';
  assertEqual(cat(['Weapon', 'Melee', 'Martial'], 'ArmsArmor'), 'Weapons', 'melee weapon');
  assertEqual(cat(['Ammunition', 'Weapon'], 'ArmsArmor'), 'Ammunition', 'ammunition beats weapon');
  assertEqual(cat(['Armor', 'Light'], 'ArmsArmor'), 'Armor', 'armor');
  assertEqual(cat(['Shield'], 'ArmsArmor'), 'Shields', 'shield');
  assertEqual(cat(['Gem'], g), 'Gems', 'gem');
  assertEqual(cat(['Spell Component', 'Consumable'], g), 'Spell Components', 'spell component');
  assertEqual(cat(['Goods', 'Poison', 'Consumable'], g), 'Poisons', 'poison');
  assertEqual(cat(['Goods', 'Alchemical'], g), 'Alchemical Items', 'alchemical');
  assertEqual(cat(['Goods', 'Trade'], g), 'Trade Goods', 'trade goods');
  assertEqual(cat(['Goods', 'Tools'], g), 'Tools & Kits', 'tools');
  assertEqual(cat(['Goods', 'Container'], g), 'Containers', 'container');
  assertEqual(cat(['Goods', 'General'], g), 'Adventuring Gear', 'general goods');
  assertEqual(cat(['Mount'], g), 'Mounts & Animals', 'mount');
  assertEqual(cat(['Service'], g), 'Services', 'service');
}

function verifiesFallbacks() {
  assertEqual(cat([], 'Equipmods'), 'Equipment Mods', 'modifiers are their own category regardless of tags');
  assertEqual(cat(['Totally Unknown Tag'], 'General'), 'Other', 'a tag we do not know is Other, not a guess');
  assertEqual(cat([], 'General'), 'Uncategorized', 'no type data is Uncategorized');
}

function verifiesEveryCategoryHasAPlaceInTheDisplayOrder() {
  const produced = new Set<string>();
  for (const types of [
    ['Magic', 'Scroll'], ['Magic', 'Wand'], ['Magic', 'Potion'], ['Magic', 'Ring'], ['Magic', 'Rod'], ['Magic', 'Staff'],
    ['Magic', 'Wondrous'], ['Magic', 'Weapon'], ['Magic', 'Armor'], ['Artifact'], ['Psionic'], ['Weapon'], ['Ammunition'],
    ['Armor'], ['Shield'], ['Gem'], ['Spell Component'], ['Goods', 'Poison'], ['Goods', 'Alchemical'], ['Goods', 'Trade'],
    ['Goods', 'Tools'], ['Goods', 'Container'], ['Goods'], ['Mount'], ['Service'], ['Nope'], [],
  ]) {
    produced.add(cat(types, 'General'));
  }
  produced.add(cat([], 'Equipmods'));
  for (const label of produced) {
    assert(EQUIPMENT_CATEGORY_ORDER.includes(label), `category "${label}" is missing from EQUIPMENT_CATEGORY_ORDER`);
  }
}

// The mapping is only useful if it names most of the real data. Measured over the committed sidecar.
function verifiesTheRealSidecarIsMostlyCategorised() {
  const path = join(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..', 'data', 'equipment_types.json');
  const { types } = JSON.parse(readFileSync(path, 'utf8')) as { types: Record<string, string[]> };
  const counts = new Map<string, number>();
  for (const tags of Object.values(types)) {
    const label = equipmentCategoryFor({ category: 'General', types: tags });
    counts.set(label, (counts.get(label) ?? 0) + 1);
  }
  const total = Object.keys(types).length;
  const other = counts.get('Other') ?? 0;
  assert(other / total <= 0.08, `${other} of ${total} typed identities fall into Other (${((100 * other) / total).toFixed(1)}%); extend the mapping`);
  assert((counts.get('Wondrous Items') ?? 0) > 1000, 'wondrous items are the largest family');
}

verifiesCategoryTabsListOnlyKindsThatHaveItemsInDisplayOrder();
verifiesMagicItemKinds();
verifiesMundaneKinds();
verifiesFallbacks();
verifiesEveryCategoryHasAPlaceInTheDisplayOrder();
verifiesTheRealSidecarIsMostlyCategorised();
console.log('equipmentCategories.test.ts: all assertions passed');
