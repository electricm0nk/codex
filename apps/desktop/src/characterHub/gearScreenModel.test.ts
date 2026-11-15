import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { ContainerLoadDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';
import { GearPanels } from './GearScreen';
import {
  filterGearRows,
  gearScreenModel,
  menuKeyStep,
  moveTargets,
  parseQuantity,
  quantityPrompt,
  stackLabel,
  type GearRow,
  type GearTreeItem,
} from './gearScreenModel';

function key(itemId: string, ordinal = 0, bonus = 0): CopyKeyDto {
  return { itemId, bonus, ordinal };
}

function held(name: string, location: LocationDto, options: { itemId?: string; bonus?: number; copies?: CopyKeyDto[]; quantity?: number; baseQuantity?: number } = {}): PlacedEquipmentCopyDto {
  const itemId = options.itemId ?? `item:${name.toLowerCase().replace(/[^a-z]+/g, '_')}`;
  const bonus = options.bonus ?? 0;
  const copies = options.copies ?? [key(itemId, 0, bonus)];
  const activeState = location.kind === 'slot' || location.kind === 'equipped' ? 'EquippedActive' : location.kind === 'notCarried' ? 'Absent' : 'SelectedInactive';
  const slot = location.kind === 'slot' ? location.slot : 'none';
  return { itemId, name, bonus, activeState, slot, location, ordinal: copies[0].ordinal, quantity: options.quantity ?? copies.length, copies, baseQuantity: options.baseQuantity ?? 1 };
}

const BACKPACK = key('item:backpack');
const POUCH = key('item:belt_pouch');
const HELD: PlacedEquipmentCopyDto[] = [
  held('Longsword', { kind: 'slot', slot: 'wield_main' }),
  held('Chain Shirt', { kind: 'slot', slot: 'armor' }),
  held('Ring of Protection +1', { kind: 'slot', slot: 'ring' }, { itemId: 'item:ring_of_protection', bonus: 1 }),
  held('Cloak of Resistance +1', { kind: 'equipped' }, { itemId: 'item:cloak_of_resistance', bonus: 1 }),
  held('Backpack', { kind: 'carried' }),
  held('Belt Pouch', { kind: 'inside', container: BACKPACK }),
  held('Trail Rations', { kind: 'inside', container: POUCH }, { copies: [key('item:trail_rations', 0), key('item:trail_rations', 1)], quantity: 2 }),
  held('Arrows (20)', { kind: 'inside', container: BACKPACK }, { itemId: 'item:arrows', quantity: 40, baseQuantity: 20 }),
  held('Dagger', { kind: 'carried' }),
  held('Shield', { kind: 'notCarried' }, { itemId: 'item:heavy_steel_shield' }),
];

const LOADS: ContainerLoadDto[] = [
  { copy: BACKPACK, contentsLbs: 61, capacity: { kind: 'pounds', lbs: 60 }, over: true },
  { copy: POUCH, contentsLbs: 1, capacity: { kind: 'unstated' }, over: false },
];

const PER_ITEM = [
  { itemId: 'item:longsword', weightLbs: 4 },
  { itemId: 'item:chain_shirt', weightLbs: 25 },
  { itemId: 'item:ring_of_protection', weightLbs: 0 },
  { itemId: 'item:cloak_of_resistance', weightLbs: 1 },
  { itemId: 'item:backpack', weightLbs: 2 },
  { itemId: 'item:belt_pouch', weightLbs: 0.5 },
  { itemId: 'item:trail_rations', weightLbs: 1 },
  { itemId: 'item:trail_rations', weightLbs: 1 },
  { itemId: 'item:arrows', weightLbs: 6 },
  { itemId: 'item:dagger', weightLbs: 1 },
];

const model = gearScreenModel(HELD, { perItem: PER_ITEM, containerLoads: LOADS, ignoreContainerCapacity: false });
const row = (name: string): GearRow => model.rows.find((candidate) => candidate.held.name === name)!;
const section = (id: string) => model.tree.find((candidate) => candidate.id === id)!;
const names = (items: GearTreeItem[]): string => items.map((item) => item.row.label).join(', ');

// 1. The filter: Equipped = in a slot or Equipped; Not equipped = the rest.
assertEqual(filterGearRows(model.rows, 'all').length, HELD.length, 'filter: All shows every row');
assertEqual(
  filterGearRows(model.rows, 'equipped').map((candidate) => candidate.held.name).join(', '),
  'Longsword, Chain Shirt, Ring of Protection +1, Cloak of Resistance +1',
  'filter: Equipped is the slot rows and the slotless worn row',
);
assertEqual(filterGearRows(model.rows, 'notEquipped').length, 6, 'filter: Not equipped is carried, inside and not carried');

// 2. Tree nesting: Carried holds the backpack, the backpack the pouch and the arrows, the pouch the rations.
assertEqual(model.tree.length, 17 + 3, 'tree: 17 slots, then Equipped, Carried, Not Carried');
assertEqual(names(section('carried').items), 'Backpack, Dagger', 'tree: Carried lists only what is not inside something');
const pack = section('carried').items[0];
assertEqual(names(pack.children), 'Belt Pouch, Arrows ×40', 'tree: the backpack holds the pouch and the arrows');
assertEqual(names(pack.children[0].children), 'Trail Rations ×2', 'tree: nested recursively, the pouch holds the rations');
assertEqual(names(section('notCarried').items), 'Shield', 'tree: Not Carried');
assertEqual(names(section('equipped').items), 'Cloak of Resistance +1', 'tree: Equipped (no slot)');
assertEqual(names(section('slot-wield_main').items), 'Longsword', 'tree: a slot row holds its copy');

// 3. Move targets exclude the row's own place, itself and anything inside it.
const packTargets = moveTargets(model, row('Backpack')).map((target) => target.id);
assert(!packTargets.includes('carried'), 'targets: not the row\'s own place');
assert(!packTargets.includes('inside-item:backpack-0-0'), 'targets: not the container itself');
assert(!packTargets.includes('inside-item:belt_pouch-0-0'), 'targets: not a container inside it');
assert(packTargets.includes('notCarried') && packTargets.includes('equipped'), 'targets: Equipped and Not Carried offered');
const daggerTargets = moveTargets(model, row('Dagger')).map((target) => target.id);
assert(daggerTargets.includes('inside-item:backpack-0-0') && daggerTargets.includes('inside-item:belt_pouch-0-0'), 'targets: every container copy for a row outside them');
const pouchTargets = moveTargets(model, row('Belt Pouch')).map((target) => target.id);
assert(!pouchTargets.includes('inside-item:backpack-0-0'), 'targets: not the container the row is already in');
assert(pouchTargets.includes('carried'), 'targets: out of the container to Carried');

// 4. Full slots are not offered; the Ring holds two.
assert(!daggerTargets.includes('slot-wield_main'), 'targets: a full slot is not offered');
assert(daggerTargets.includes('slot-wield_off'), 'targets: a free slot is');
assertEqual(section('slot-ring').places, '1 of 2', 'ring: two places, one taken');
assert(daggerTargets.includes('slot-ring'), 'ring: one ring in, the second place is offered');
const twoRings = gearScreenModel([...HELD, held('Ring of Sustenance', { kind: 'slot', slot: 'ring' })], { perItem: [], containerLoads: [] });
assertEqual(twoRings.tree.find((candidate) => candidate.id === 'slot-ring')!.places, '2 of 2', 'ring: two rings fill it');
assert(!moveTargets(twoRings, twoRings.rows.find((candidate) => candidate.held.name === 'Dagger')!).some((target) => target.id === 'slot-ring'), 'ring: full at two');

// 5. Over capacity: a warning row per over container, the "x of y lb" from the engine's load.
assertEqual(row('Backpack').capacityLabel, '61 of 60 lb', 'capacity: x of y lb from containerLoads');
assertEqual(row('Belt Pouch').capacityLabel, '1 lb', 'capacity: an unstated limit shows the contents only');
assertEqual(model.warnings.length, 1, 'warning: one over-capacity container');
assertEqual(model.warnings[0].text, 'Backpack holds 61 lb, over its 60 lb.', 'warning: words');
assertEqual(model.capacityOverride, false, 'no override, no tag');
assertEqual(pack.overrideTag, false, 'no override, no tag on the container');

// 6. Under "Ignore container capacity limits" the warning is suppressed, the container tagged, the figure kept.
const ignored = gearScreenModel(HELD, { perItem: PER_ITEM, containerLoads: LOADS, ignoreContainerCapacity: true });
assertEqual(ignored.warnings.length, 0, 'override: the warning is suppressed');
assertEqual(ignored.capacityOverride, true, 'override: the screen tags it');
const ignoredPack = ignored.tree.find((candidate) => candidate.id === 'carried')!.items[0];
assertEqual(ignoredPack.overrideTag, true, 'override: the over container carries the tag');
assertEqual(ignoredPack.capacityLabel, '61 of 60 lb', 'override: "x of y lb" stays');

// 7. A copy the set does not name is Carried (the held list says `carried`).
assertEqual(row('Dagger').locationLabel, 'Carried', 'unnamed copy reads Carried');
assertEqual(row('Belt Pouch').locationLabel, 'In Backpack', 'inside reads the container');
assertEqual(row('Ring of Protection +1').locationLabel, 'Ring', 'a slot reads its label');
assertEqual(row('Shield').locationLabel, 'Not Carried', 'not carried');

// 8. Stack labels count pieces; the bundle's "(20)" is dropped.
assertEqual(stackLabel(row('Arrows (20)').held), 'Arrows ×40', 'stack label: pieces, bundle name dropped');
assertEqual(stackLabel(row('Trail Rations').held), 'Trail Rations ×2', 'stack label: two copies');
assertEqual(stackLabel(row('Dagger').held), 'Dagger', 'stack label: one piece is the name');
assertEqual(stackLabel(held('Rope (50 ft.)', { kind: 'carried' })), 'Rope (50 ft.)', 'stack label: a name in brackets that is not a bundle stays');

// 9. The quantity prompt: only on a stack, 1 to the stack's pieces.
assertEqual(quantityPrompt(row('Dagger').held), null, 'quantity: one piece asks nothing');
assertEqual(JSON.stringify(quantityPrompt(row('Arrows (20)').held)), JSON.stringify({ min: 1, max: 40 }), 'quantity: bounds 1..pieces');
assertEqual(parseQuantity('20', row('Arrows (20)').held), 20, 'quantity: 20 of 40');
assertEqual(parseQuantity('0', row('Arrows (20)').held), 'Enter a whole number from 1 to 40.', 'quantity: below 1 refused');
assertEqual(parseQuantity('41', row('Arrows (20)').held), 'Enter a whole number from 1 to 40.', 'quantity: above the stack refused');
assertEqual(parseQuantity('2.5', row('Arrows (20)').held), 'Enter a whole number from 1 to 40.', 'quantity: a fraction refused');

// 10. Weight labels come from encumbrance.perItem; a copy the engine does not weigh reads "—".
assertEqual(row('Chain Shirt').weightLabel, '25 lb', 'weight: the engine row');
assertEqual(row('Arrows (20)').weightLabel, '6 lb', 'weight: one line of 40 pieces');
assertEqual(row('Trail Rations').weightLabel, '1 lb each', 'weight: two lines of one piece');
assertEqual(row('Shield').weightLabel, '—', 'weight: not carried is not weighed');
const unweighed = gearScreenModel([held('Mystery', { kind: 'carried' })], { perItem: [], containerLoads: [] });
assertEqual(unweighed.rows[0].weightLabel, '—', 'weight: no engine row');

// 11. The Move to menu's keys.
assertEqual(menuKeyStep(0, 'ArrowDown', 3), 1, 'menu: down');
assertEqual(menuKeyStep(2, 'ArrowDown', 3), 0, 'menu: down wraps');
assertEqual(menuKeyStep(0, 'ArrowUp', 3), 2, 'menu: up wraps');
assertEqual(menuKeyStep(1, 'Escape', 3), 'close', 'menu: Esc closes');
assertEqual(menuKeyStep(1, 'x', 3), null, 'menu: other keys pass');

// 12. The screen: rows, tree, filter, set picker, caption, buttons, warning.
const noop = () => undefined;
const html = renderToStaticMarkup(
  createElement(GearPanels, {
    model,
    armorClass: 17,
    carriedLbs: 40.5,
    sets: { active: 'Default', names: ['Default', 'Travel'] },
    busy: false,
    refusal: 'equipment.slot_taken: Main hand is full',
    onMove: noop,
    onDrop: noop,
    onSell: noop,
    onDiscard: noop,
    onSets: { activate: noop, create: noop, rename: noop, remove: noop },
  }),
);
assert(html.includes('data-testid="gear-row-item:longsword-0-0"'), 'view: row testid');
assert(html.includes('data-testid="gear-move-to"'), 'view: Move to button');
assert(html.includes('data-testid="gear-location-slot-ring"') && html.includes('data-testid="gear-location-notCarried"'), 'view: tree location testids');
assert(html.includes('data-testid="gear-set-select"') && html.includes('>Travel<'), 'view: the set picker lists the sets');
assert(html.includes('data-testid="gear-filter"'), 'view: filter');
assert(html.includes('Drop keeps the item and marks it Not Carried; Sell returns money under the pricing above; Discard removes it for nothing.'), 'view: caption');
assert(html.includes('>Drop<') && html.includes('>Sell<') && html.includes('>Discard<'), 'view: Drop / Sell / Discard');
assert(html.includes('Backpack holds 61 lb, over its 60 lb.'), 'view: the warning row');
assert(html.includes('equipment.slot_taken: Main hand is full'), 'view: a refusal word for word');
assert(html.includes('AC 17') && html.includes('40.5 lb carried'), 'view: the engine headline');
assert(html.includes('aria-haspopup="menu"'), 'view: Move to opens a menu');
const tagged = renderToStaticMarkup(
  createElement(GearPanels, {
    model: ignored,
    armorClass: 17,
    carriedLbs: 40.5,
    busy: false,
    refusal: null,
    onMove: noop,
    onDrop: noop,
    onSell: noop,
    onDiscard: noop,
    onSets: { activate: noop, create: noop, rename: noop, remove: noop },
  }),
);
assert(tagged.includes('Override: Ignore container capacity limits'), 'view: the override tag');
assert(!tagged.includes('over its 60 lb'), 'view: no warning under the override');
assert(!tagged.includes('gear-set-select'), 'view: no set picker without sets');

console.log('gearScreenModel.test.ts: 12 cases passed');
