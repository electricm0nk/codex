import { buildItemPickerConfig } from './CharacterSheet';
import type { ItemPickerEntry } from './itemPickerFilter';
import { assert, assertEqual } from '../testSupport/asserts';

const SPELL_ENTRY: ItemPickerEntry = { key: 'spell:fireball', name: 'spell:fireball', detail: 'Evocation · Level 3' };
const FEAT_ENTRY: ItemPickerEntry = { key: 'feat:dodge', name: 'Dodge', detail: 'Combat · +1 dodge bonus to AC' };
const FEAT_TARGET_ENTRY: ItemPickerEntry = { key: 'Longsword', name: 'Longsword', detail: '1d8 · threat 19-20/x2' };
const MODIFIER_ENTRY: ItemPickerEntry = {
  key: 'Special Ability ~ +1 ~ Weapon',
  name: 'Special Ability ~ +1 ~ Weapon',
  detail: 'Equipmods',
};

/**
 * Locks in the Add Spell / Add Feat / Attach Modifier onClick wiring
 * (criterion 7.4). Before this cycle the dispatch table
 * (`itemPickerOpen` -> title / category / loader / mutate-handler) lived
 * inline inside `CharacterSheet`'s render body, so nothing could prove —
 * short of driving the actual DOM, which this repo's test runner cannot do
 * (no jsdom) — that clicking "Attach Modifier" really narrows the real
 * `listEquipment` corpus query to `Equipmods` and really wires the
 * selection back to the real mutation handler, rather than a no-op. `buildItemPickerConfig` extracts that dispatch as a
 * pure, unit-testable function so this cycle's tests exercise the exact
 * wiring the rendered picker uses.
 */

async function main() {
  verifiesSpellKindLoadsSpellCatalogAndWiresSpellHandler();
  verifiesFeatKindLoadsFeatCatalogAndWiresFeatHandler();
  verifiesNullKindProducesNoPickerConfig();
  verifiesModifierKindNarrowsEquipmentToEquipmodsAndWiresModifierHandler();
}

function makeDeps() {
  const loadEquipmentCalls: string[] = [];
  const loadSpellsCalls: number[] = [];
  const loadFeatsCalls: number[] = [];
  const spellSelections: ItemPickerEntry[] = [];
  const featSelections: ItemPickerEntry[] = [];
  const modifierSelections: ItemPickerEntry[] = [];
  const featTargetSelections: ItemPickerEntry[] = [];
  const loadFeatTargetsCalls: number[] = [];

  const deps = {
    loadEquipment: (category: string) => {
      loadEquipmentCalls.push(category);
      return Promise.resolve(category === 'Equipmods' ? [MODIFIER_ENTRY] : []);
    },
    loadSpells: () => {
      loadSpellsCalls.push(1);
      return Promise.resolve([SPELL_ENTRY]);
    },
    loadFeatTargets: () => {
      loadFeatTargetsCalls.push(1);
      return Promise.resolve([FEAT_TARGET_ENTRY]);
    },
    onSelectFeatTarget: (entry: ItemPickerEntry) => featTargetSelections.push(entry),
    loadFeats: () => {
      loadFeatsCalls.push(1);
      return Promise.resolve([FEAT_ENTRY]);
    },
    onSelectSpell: (entry: ItemPickerEntry) => spellSelections.push(entry),
    onSelectFeat: (entry: ItemPickerEntry) => featSelections.push(entry),
    onSelectModifier: (entry: ItemPickerEntry) => modifierSelections.push(entry),
  };

  return {
    deps,
    featTargetSelections,
    loadFeatTargetsCalls,
    loadEquipmentCalls,
    loadSpellsCalls,
    loadFeatsCalls,
    spellSelections,
    featSelections,
    modifierSelections,
  };
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

/**
 * items-1-and-27-scoping.md sub-task 6: locks in the Attach Modifier
 * dispatch the same way every other picker kind is already locked in —
 * proves it narrows to the real `Equipmods` category and routes to its own handler.
 */
function verifiesModifierKindNarrowsEquipmentToEquipmodsAndWiresModifierHandler() {
  const { deps, loadEquipmentCalls, modifierSelections } = makeDeps();
  const config = buildItemPickerConfig('modifier', deps);
  assert(config !== null, 'modifier kind produces a picker config');
  if (!config) return;
  assertEqual(config.title, 'Attach Modifier', 'modifier picker title');
  config.onSelect(MODIFIER_ENTRY);
  assertEqual(modifierSelections.length, 1, 'modifier selection is routed to the modifier handler, not dropped');
  assertEqual(modifierSelections[0].key, MODIFIER_ENTRY.key, 'the exact selected entry reaches the modifier handler');
  return config.loadEntries().then((entries) => {
    assertEqual(loadEquipmentCalls[0], 'Equipmods', 'the modifier picker queries the real equipment corpus narrowed to Equipmods');
    assertEqual(entries[0].key, MODIFIER_ENTRY.key, 'loadEntries resolves the Equipmods catalog loader output');
  });
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});

