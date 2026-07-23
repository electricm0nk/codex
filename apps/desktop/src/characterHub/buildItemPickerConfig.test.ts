import { buildItemPickerConfig } from './CharacterSheet';
import type { ItemPickerEntry } from './itemPickerFilter';
import { assert, assertEqual } from '../testSupport/asserts';

const WEAPON_ENTRY: ItemPickerEntry = { key: 'equipment:longsword', name: 'Longsword', detail: 'Arms & Armor' };
const SPELL_ENTRY: ItemPickerEntry = { key: 'spell:fireball', name: 'spell:fireball', detail: 'Evocation · Level 3' };
const FEAT_ENTRY: ItemPickerEntry = { key: 'feat:dodge', name: 'Dodge', detail: 'Combat · +1 dodge bonus to AC' };

/**
 * Locks in the Add Weapon / Add Armor / Add Spell onClick wiring
 * (criterion 7.4). Before this cycle the dispatch table
 * (`itemPickerOpen` -> title / category / loader / mutate-handler) lived
 * inline inside `CharacterSheet`'s render body, so nothing could prove —
 * short of driving the actual DOM, which this repo's test runner cannot do
 * (no jsdom) — that clicking "Add Weapon" really narrows the real
 * `listEquipment` corpus query to `ArmsArmor` and really wires the
 * selection back to the real `addEquipmentSelection` mutation handler,
 * rather than a no-op. `buildItemPickerConfig` extracts that dispatch as a
 * pure, unit-testable function so this cycle's tests exercise the exact
 * wiring the rendered picker uses.
 */

async function main() {
  verifiesWeaponKindNarrowsEquipmentToArmsAndArmorAndWiresEquipmentHandler();
  verifiesArmorKindNarrowsEquipmentToArmsAndArmorAndWiresEquipmentHandler();
  verifiesSpellKindLoadsSpellCatalogAndWiresSpellHandler();
  verifiesFeatKindLoadsFeatCatalogAndWiresFeatHandler();
  verifiesNullKindProducesNoPickerConfig();
  verifiesWeaponAndArmorHaveDistinctTitles();
}

function makeDeps() {
  const loadEquipmentCalls: string[] = [];
  const loadSpellsCalls: number[] = [];
  const loadFeatsCalls: number[] = [];
  const equipmentSelections: ItemPickerEntry[] = [];
  const spellSelections: ItemPickerEntry[] = [];
  const featSelections: ItemPickerEntry[] = [];

  const deps = {
    loadEquipment: (category: string) => {
      loadEquipmentCalls.push(category);
      return Promise.resolve([WEAPON_ENTRY]);
    },
    loadSpells: () => {
      loadSpellsCalls.push(1);
      return Promise.resolve([SPELL_ENTRY]);
    },
    loadFeats: () => {
      loadFeatsCalls.push(1);
      return Promise.resolve([FEAT_ENTRY]);
    },
    onSelectEquipment: (entry: ItemPickerEntry) => equipmentSelections.push(entry),
    onSelectSpell: (entry: ItemPickerEntry) => spellSelections.push(entry),
    onSelectFeat: (entry: ItemPickerEntry) => featSelections.push(entry),
  };

  return { deps, loadEquipmentCalls, loadSpellsCalls, loadFeatsCalls, equipmentSelections, spellSelections, featSelections };
}

function verifiesWeaponKindNarrowsEquipmentToArmsAndArmorAndWiresEquipmentHandler() {
  const { deps, loadEquipmentCalls, equipmentSelections } = makeDeps();
  const config = buildItemPickerConfig('weapon', deps);
  assert(config !== null, 'weapon kind produces a picker config');
  if (!config) return;
  assertEqual(config.title, 'Add Weapon', 'weapon picker title');
  config.onSelect(WEAPON_ENTRY);
  assertEqual(equipmentSelections.length, 1, 'weapon selection is routed to the equipment handler, not dropped');
  assertEqual(equipmentSelections[0].key, WEAPON_ENTRY.key, 'the exact selected entry reaches the equipment handler');
  return config.loadEntries().then((entries) => {
    assertEqual(loadEquipmentCalls[0], 'ArmsArmor', 'the weapon picker queries the real equipment corpus narrowed to ArmsArmor');
    assertEqual(entries.length, 1, 'loadEntries resolves the entries the corpus loader returned');
  });
}

function verifiesArmorKindNarrowsEquipmentToArmsAndArmorAndWiresEquipmentHandler() {
  const { deps, loadEquipmentCalls } = makeDeps();
  const config = buildItemPickerConfig('armor', deps);
  assert(config !== null, 'armor kind produces a picker config');
  if (!config) return;
  assertEqual(config.title, 'Add Armor', 'armor picker title');
  return config.loadEntries().then(() => {
    assertEqual(loadEquipmentCalls[0], 'ArmsArmor', 'the armor picker also queries the real equipment corpus narrowed to ArmsArmor');
  });
}

function verifiesSpellKindLoadsSpellCatalogAndWiresSpellHandler() {
  const { deps, loadSpellsCalls, spellSelections } = makeDeps();
  const config = buildItemPickerConfig('spell', deps);
  assert(config !== null, 'spell kind produces a picker config');
  if (!config) return;
  assertEqual(config.title, 'Add Spell', 'spell picker title');
  config.onSelect(SPELL_ENTRY);
  assertEqual(spellSelections.length, 1, 'spell selection is routed to the spell handler, not dropped');
  return config.loadEntries().then((entries) => {
    assertEqual(loadSpellsCalls.length, 1, 'the spell picker queries the real spell catalog');
    assertEqual(entries[0].key, SPELL_ENTRY.key, 'loadEntries resolves the spell catalog loader output');
  });
}

function verifiesFeatKindLoadsFeatCatalogAndWiresFeatHandler() {
  const { deps, loadFeatsCalls, featSelections } = makeDeps();
  const config = buildItemPickerConfig('feat', deps);
  assert(config !== null, 'feat kind produces a picker config');
  if (!config) return;
  assertEqual(config.title, 'Add Feat', 'feat picker title');
  config.onSelect(FEAT_ENTRY);
  assertEqual(featSelections.length, 1, 'feat selection is routed to the feat handler, not dropped');
  return config.loadEntries().then((entries) => {
    assertEqual(loadFeatsCalls.length, 1, 'the feat picker queries the real feat catalog');
    assertEqual(entries[0].key, FEAT_ENTRY.key, 'loadEntries resolves the feat catalog loader output');
  });
}

function verifiesNullKindProducesNoPickerConfig() {
  const { deps } = makeDeps();
  const config = buildItemPickerConfig(null, deps);
  assertEqual(config, null, 'no picker kind means no config, and no catalog call happens');
}

function verifiesWeaponAndArmorHaveDistinctTitles() {
  const { deps } = makeDeps();
  const weaponConfig = buildItemPickerConfig('weapon', deps);
  const armorConfig = buildItemPickerConfig('armor', deps);
  assert(weaponConfig !== null && armorConfig !== null, 'both kinds produce a config');
  if (!weaponConfig || !armorConfig) return;
  assert(weaponConfig.title !== armorConfig.title, 'Add Weapon and Add Armor render distinct titles even though both narrow the same corpus category');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
