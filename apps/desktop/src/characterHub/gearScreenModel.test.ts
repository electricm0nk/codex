import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { ContainerLoadDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';
import { GearPanels } from './GearScreen';
import {
  containerTables,
  DRAG_FORMAT,
  dropTarget,
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
// SD-42 E4.7: a weightless container's title says so; a plain one's is unchanged.
const weightless = gearScreenModel([held('Bag of Holding', { kind: 'carried' }), held('Sack', { kind: 'carried' })], {
  perItem: [],
  containerLoads: [
    { copy: key('item:bag_of_holding'), contentsLbs: 300, capacity: { kind: 'pounds', lbs: 250 }, over: true, contentsWeightPct: 0 },
    { copy: key('item:sack'), contentsLbs: 2, capacity: { kind: 'unlimited' }, over: false, contentsWeightPct: 60 },
  ],
});
const weightlessRow = (name: string) => weightless.rows.find((candidate) => candidate.held.name === name)!;
assertEqual(weightlessRow('Bag of Holding').capacityLabel, '300 of 250 lb, contents weigh nothing', 'weightless: the bag says its contents weigh nothing');
assertEqual(weightlessRow('Sack').capacityLabel, '2 lb, no limit, contents weigh 40 % less', 'weightless: a reducing container says how much less');
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
// E4.MC: three bundles of 20 bought one at a time stack as one row of 60 pieces; the engine weighs three
// lines of 3 lb, and the row says so as the print does (`3 × 3 lb`), not "—".
const bundles = [key('item:arrows_flight', 0), key('item:arrows_flight', 1), key('item:arrows_flight', 2)];
const bundled = gearScreenModel([held('Arrow, Flight (20)', { kind: 'carried' }, { itemId: 'item:arrows_flight', copies: bundles, quantity: 60, baseQuantity: 20 })], {
  perItem: [3, 3, 3].map((weightLbs) => ({ itemId: 'item:arrows_flight', weightLbs })),
  containerLoads: [],
});
assertEqual(bundled.rows[0].weightLabel, '3 × 3 lb', 'weight: a stack of bundles, the engine weight per bundle');

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

assert(html.includes('draggable="true"'), 'view: rows are draggable');

// 13. dropTarget: the dragged row (its id in the drag data) and the target's id give the menu's location, or null.
const dragging = (id: string) => ({ getData: (format: string) => (format === DRAG_FORMAT ? id : '') });
assertEqual(JSON.stringify(dropTarget(dragging('item:dagger-0-0'), 'inside-item:backpack-0-0', model)), JSON.stringify({ kind: 'inside', container: BACKPACK }), 'drop: a row onto a container goes inside it');
assertEqual(JSON.stringify(dropTarget(dragging('item:dagger-0-0'), 'slot-wield_off', model)), JSON.stringify({ kind: 'slot', slot: 'wield_off' }), 'drop: onto a free slot');
assertEqual(JSON.stringify(dropTarget(dragging('item:backpack-0-0'), 'notCarried', model)), JSON.stringify({ kind: 'notCarried' }), 'drop: onto Not Carried');
assertEqual(dropTarget(dragging('item:backpack-0-0'), 'inside-item:backpack-0-0', model), null, 'drop: null onto itself');
assertEqual(dropTarget(dragging('item:backpack-0-0'), 'inside-item:belt_pouch-0-0', model), null, 'drop: null onto a descendant');
assertEqual(dropTarget(dragging('item:dagger-0-0'), 'slot-wield_main', model), null, 'drop: null onto a full slot');
assertEqual(dropTarget(dragging('item:backpack-0-0'), 'inside-item:dagger-0-0', model), null, 'drop: null onto a non-container');
assertEqual(dropTarget(dragging('item:dagger-0-0'), 'carried', model), null, 'drop: null onto its own place');
assertEqual(dropTarget(dragging(''), 'carried', model), null, 'drop: null without a gear row in the drag');

// 14. "Ignore body slot limits" on (`decisions.md §14`, SD-39 E6.2): Move to and drag offer a full slot too.
const overridden = gearScreenModel(HELD, { perItem: PER_ITEM, containerLoads: LOADS, ignoreSlotLimits: true });
const overriddenTargets = moveTargets(overridden, overridden.rows.find((candidate) => candidate.held.name === 'Dagger')!).map((target) => target.id);
assert(overriddenTargets.includes('slot-wield_main'), 'override: a full slot is offered');
assert(overriddenTargets.includes('slot-armor'), 'override: every full slot is offered');
assert(!moveTargets(overridden, overridden.rows.find((candidate) => candidate.held.name === 'Longsword')!).some((target) => target.id === 'slot-wield_main'), 'override: still not its own slot');
assertEqual(JSON.stringify(dropTarget(dragging('item:dagger-0-0'), 'slot-wield_main', overridden)), JSON.stringify({ kind: 'slot', slot: 'wield_main' }), 'override: a drop onto a full slot moves there');

// 15. One table per container copy (`decisions.md §38`): a backpack holding 4 belt pouches and chalk, pouch #1 a whetstone → 5 tables.
const PACK = key('item:backpack');
const POUCHES = [0, 1, 2, 3].map((ordinal) => key('item:pouch_belt', ordinal));
const NEST: PlacedEquipmentCopyDto[] = [
  held('Backpack', { kind: 'carried' }),
  held('Pouch, Belt', { kind: 'inside', container: PACK }, { itemId: 'item:pouch_belt', copies: POUCHES }),
  held('Chalk', { kind: 'inside', container: PACK }),
  held('Whetstone', { kind: 'inside', container: POUCHES[0] }),
  held('Dagger', { kind: 'carried' }),
];
const NEST_LOADS: ContainerLoadDto[] = [
  { copy: PACK, contentsLbs: 3, capacity: { kind: 'pounds', lbs: 60 }, over: false },
  ...POUCHES.map((copy, index): ContainerLoadDto => ({ copy, contentsLbs: index === 0 ? 1 : 0, capacity: { kind: 'unlimited' }, over: false })),
];
const nest = gearScreenModel(NEST, { perItem: [], containerLoads: NEST_LOADS });
const tables = containerTables(nest);
assertEqual(tables.length, 5, 'tables: backpack + 4 pouches');
assertEqual(tables.map((table) => table.title).join(' | '), 'Backpack | Pouch, Belt #1 | Pouch, Belt #2 | Pouch, Belt #3 | Pouch, Belt #4', 'tables: titled as Move to titles them');
assertEqual(tables.map((table) => table.rows.length).join(','), '2,1,0,0,0', 'tables: direct contents per copy');
assertEqual(tables[0].rows.map((candidate) => candidate.label).join(', '), 'Pouch, Belt ×4, Chalk', 'tables: the backpack holds the pouch stack and the chalk');
assertEqual(tables[1].rows[0].label, 'Whetstone', 'tables: pouch #1 holds the whetstone');
assertEqual(tables.map((table) => table.loadLabel).join(' | '), '3 of 60 lb | 1 lb, no limit | 0 lb, no limit | 0 lb, no limit | 0 lb, no limit', 'tables: the load labels as containerLoads gives them');
assertEqual(nest.rows.length, NEST.length, 'tables: GearModel.rows unchanged');

// 16. A nested container is a row in its parent's table and has its own table; one level deep only.
assert(!tables[0].rows.some((candidate) => candidate.label === 'Whetstone'), 'tables: one level deep, the whetstone is not in the backpack\'s table');
assert(tables[0].rows.some((candidate) => candidate.isContainer) && tables.some((table) => table.copy.itemId === 'item:pouch_belt'), 'tables: the pouch is a row and a table');
const nestedTables = containerTables(model);
assertEqual(nestedTables.map((table) => `${table.title}: ${table.rows.map((candidate) => candidate.label).join(', ')}`).join(' | '), 'Backpack: Belt Pouch, Arrows ×40 | Belt Pouch: Trail Rations ×2', 'tables: the fixture\'s backpack and pouch');

// 17. The view: one table per container copy, each content row with Move to, Sell and Discard inside it; the top level keeps the rest.
const nestHtml = renderToStaticMarkup(
  createElement(GearPanels, {
    model: nest,
    armorClass: null,
    carriedLbs: null,
    busy: false,
    refusal: null,
    onMove: noop,
    onDrop: noop,
    onSell: noop,
    onDiscard: noop,
    onSets: { activate: noop, create: noop, rename: noop, remove: noop },
  }),
);
assertEqual(nestHtml.split('data-testid="gear-container-').length - 1, 5, 'view: one table per container copy');
const pouchTable = nestHtml.slice(nestHtml.indexOf('data-testid="gear-container-item:pouch_belt-0-0"'), nestHtml.indexOf('data-testid="gear-container-item:pouch_belt-0-1"'));
assert(pouchTable.includes('Pouch, Belt #1 · 1 lb, no limit'), 'view: the caption reads title · load');
assert(pouchTable.includes('Whetstone') && pouchTable.includes('data-testid="gear-discard"') && pouchTable.includes('data-testid="gear-sell"') && pouchTable.includes('data-testid="gear-move-to"'), 'view: a content row keeps its buttons inside the table');
assert(nestHtml.includes('>Empty<'), 'view: an empty container reads Empty');
const topLevel = nestHtml.slice(0, nestHtml.indexOf('data-testid="gear-container-'));
assert(topLevel.includes('gear-row-item:dagger-0-0') && !topLevel.includes('gear-row-item:chalk-0-0'), 'view: the top level keeps only rows not inside a container');

console.log('gearScreenModel.test.ts: 17 cases passed');
