import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { BodySlotDto, CopyKeyDto, LocationDto, PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { ContainerLoadDto } from '../boundary/loadSavedCharacterDetail';
import { assert, assertEqual } from '../testSupport/asserts';
import { GearPanels } from './GearScreen';
import {
  copyId,
  DRAG_FORMAT,
  boxZoneId,
  dropTarget,
  filterGearRows,
  gearScreenModel,
  menuKeyStep,
  moveTargets,
  parseQuantity,
  slotKindsAllowed,
  quantityPrompt,
  stackLabel,
  type GearRow,
  type GearTreeItem,
} from './gearScreenModel';

function key(itemId: string, ordinal = 0, bonus = 0): CopyKeyDto {
  return { itemId, bonus, ordinal };
}

function held(name: string, location: LocationDto, options: { itemId?: string; bonus?: number; copies?: CopyKeyDto[]; quantity?: number; baseQuantity?: number; slot?: BodySlotDto; slotWarning?: string } = {}): PlacedEquipmentCopyDto {
  const itemId = options.itemId ?? `item:${name.toLowerCase().replace(/[^a-z]+/g, '_')}`;
  const bonus = options.bonus ?? 0;
  const copies = options.copies ?? [key(itemId, 0, bonus)];
  const activeState = location.kind === 'slot' || location.kind === 'equipped' ? 'EquippedActive' : location.kind === 'notCarried' ? 'Absent' : 'SelectedInactive';
  const slot = options.slot ?? (location.kind === 'slot' ? location.slot : 'none');
  const row: PlacedEquipmentCopyDto = { itemId, name, baseName: name, bonus, activeState, slot, location, ordinal: copies[0].ordinal, quantity: options.quantity ?? copies.length, copies, baseQuantity: options.baseQuantity ?? 1 };
  return options.slotWarning ? { ...row, slotWarning: options.slotWarning } : row;
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
  held('Dagger', { kind: 'carried' }, { slot: 'wield_main' }),
  held('Shield', { kind: 'notCarried' }, { itemId: 'item:heavy_steel_shield', slot: 'shield' }),
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
// The container boxes in screen order, each with its direct contents (SD-43 E1.2's boxes replace SD-41 §38's tables).
const boxes = (of: typeof model) => {
  const found: { copy: CopyKeyDto; title: string; loadLabel: string | null; rows: GearRow[] }[] = [];
  const walk = (items: GearTreeItem[]): void =>
    items.forEach((item) => {
      if (item.row.isContainer) {
        found.push({ copy: item.copy, title: item.row.label, loadLabel: item.capacityLabel, rows: item.children.map((child) => child.row) });
      }
      walk(item.children);
    });
  of.tree.forEach((place) => walk(place.items));
  return found;
};

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
assertEqual(names(section('carried').items), 'Dagger, Backpack', 'tree: Carried lists only what is not inside something, loose rows first (E1.2)');
const pack = section('carried').items[1];
assertEqual(names(pack.children), 'Arrows ×40, Belt Pouch', 'tree: the backpack holds the arrows and the pouch');
assertEqual(names(pack.children[1].children), 'Trail Rations ×2', 'tree: nested recursively, the pouch holds the rations');
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
const spareRing = gearScreenModel([...HELD, held('Ring of Sustenance', { kind: 'carried' }, { slot: 'ring' })], { perItem: [], containerLoads: [] });
assert(moveTargets(spareRing, spareRing.rows.find((candidate) => candidate.held.name === 'Ring of Sustenance')!).some((target) => target.id === 'slot-ring'), 'ring: one ring in, the second place is offered');
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
const ignoredPack = ignored.tree.find((candidate) => candidate.id === 'carried')!.items[1];
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
    onCustomizeName: noop,
    onCustomizeModifier: noop,
  }),
);
assert(html.includes('data-testid="gear-row-item:longsword-0-0"'), 'view: row testid');
assert(html.includes('data-testid="gear-move-to"'), 'view: Move to button');
assert(html.includes('data-testid="gear-row-customize-item:longsword-0-0"'), 'view: a Customize button on the row (E4.9)');
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
    onCustomizeName: noop,
    onCustomizeModifier: noop,
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

// 13b. A child row of a container box targets that container (nearest when nested); the box's own zone is the container's.
assertEqual(boxZoneId(model, 'item:trail_rations-0-0'), 'inside-item:belt_pouch-0-0', 'child row: a row in a pouch in the backpack targets the pouch, the nearest');
assertEqual(boxZoneId(model, 'item:belt_pouch-0-0'), 'inside-item:belt_pouch-0-0', 'child row: a nested container row is its own box');
assertEqual(boxZoneId(model, 'item:backpack-0-0'), 'inside-item:backpack-0-0', 'child row: a container row targets itself');
assertEqual(boxZoneId(model, 'item:arrows-0-0'), 'inside-item:backpack-0-0', 'child row: a leaf row targets its container');
assertEqual(boxZoneId(model, 'item:dagger-0-0'), null, 'child row: a loose row has no box');
assertEqual(JSON.stringify(dropTarget(dragging('item:dagger-0-0'), boxZoneId(model, 'item:arrows-0-0')!, model)), JSON.stringify({ kind: 'inside', container: BACKPACK }), 'child row: a drop on a child row moves into its container');
assertEqual(dropTarget(dragging('item:arrows-0-0'), boxZoneId(model, 'item:arrows-0-0')!, model), null, 'child row: a same-container drop is a no-op');
assertEqual(dropTarget(dragging('item:backpack-0-0'), boxZoneId(model, 'item:trail_rations-0-0')!, model), null, 'child row: a container onto its own descendant over the widened area is refused');

// 14. "Ignore body slot limits" on (`decisions.md §14`, SD-39 E6.2): Move to and drag offer a full slot too.
const overridden = gearScreenModel(HELD, { perItem: PER_ITEM, containerLoads: LOADS, ignoreSlotLimits: true });
const overriddenTargets = moveTargets(overridden, overridden.rows.find((candidate) => candidate.held.name === 'Dagger')!).map((target) => target.id);
assert(overriddenTargets.includes('slot-wield_main'), 'override: a full slot is offered');
assert(overriddenTargets.includes('slot-armor'), 'override: every full slot is offered');
assert(!moveTargets(overridden, overridden.rows.find((candidate) => candidate.held.name === 'Longsword')!).some((target) => target.id === 'slot-wield_main'), 'override: still not its own slot');
assertEqual(JSON.stringify(dropTarget(dragging('item:dagger-0-0'), 'slot-wield_main', overridden)), JSON.stringify({ kind: 'slot', slot: 'wield_main' }), 'override: a drop onto a full slot moves there');

// 15. One table per container row (`decisions.md §38`; SD-42 §27.5): a backpack holding 4 belt pouches and chalk, one pouch a whetstone —
// the engine makes the filled pouch its own row and the other three one stack → 3 tables.
const PACK = key('item:backpack');
const POUCHES = [0, 1, 2, 3].map((ordinal) => key('item:pouch_belt', ordinal));
const NEST: PlacedEquipmentCopyDto[] = [
  held('Backpack', { kind: 'carried' }),
  held('Pouch, Belt', { kind: 'inside', container: PACK }, { itemId: 'item:pouch_belt', copies: [POUCHES[0]] }),
  held('Pouch, Belt', { kind: 'inside', container: PACK }, { itemId: 'item:pouch_belt', copies: POUCHES.slice(1) }),
  held('Chalk', { kind: 'inside', container: PACK }),
  held('Whetstone', { kind: 'inside', container: POUCHES[0] }),
  held('Dagger', { kind: 'carried' }),
];
const NEST_LOADS: ContainerLoadDto[] = [
  { copy: PACK, contentsLbs: 3, capacity: { kind: 'pounds', lbs: 60 }, over: false },
  ...POUCHES.map((copy, index): ContainerLoadDto => ({ copy, contentsLbs: index === 0 ? 1 : 0, capacity: { kind: 'unlimited' }, over: false })),
];
const nest = gearScreenModel(NEST, { perItem: [], containerLoads: NEST_LOADS });
const tables = boxes(nest);
assertEqual(tables.length, 3, 'tables: backpack + the filled pouch + the stack of three');
assertEqual(tables.map((table) => table.title).join(' | '), 'Backpack | Pouch, Belt | Pouch, Belt ×3', 'tables: titled as the right panel names them, no #N (SD-43 E1.1)');
assertEqual(tables.map((table) => table.rows.length).join(','), '3,1,0', 'tables: direct contents per row');
assertEqual(tables[0].rows.map((candidate) => candidate.label).join(', '), 'Chalk, Pouch, Belt, Pouch, Belt ×3', 'tables: the backpack holds the chalk, then the filled pouch and the pouch stack');
assertEqual(tables[1].rows[0].label, 'Whetstone', 'tables: pouch #1 holds the whetstone');
assertEqual(tables.map((table) => table.loadLabel).join(' | '), '3 of 60 lb | 1 lb, no limit | 0 lb, no limit', 'tables: the load labels as containerLoads gives them');
assertEqual(nest.rows.length, NEST.length, 'tables: GearModel.rows unchanged');

// 16. A nested container is a row in its parent's table and has its own table; one level deep only.
assert(!tables[0].rows.some((candidate) => candidate.label === 'Whetstone'), 'tables: one level deep, the whetstone is not in the backpack\'s table');
assert(tables[0].rows.some((candidate) => candidate.isContainer) && tables.some((table) => table.copy.itemId === 'item:pouch_belt'), 'tables: the pouch is a row and a table');
const nestedTables = boxes(model);
assertEqual(nestedTables.map((table) => `${table.title}: ${table.rows.map((candidate) => candidate.label).join(', ')}`).join(' | '), 'Backpack: Arrows ×40, Belt Pouch | Belt Pouch: Trail Rations ×2', 'tables: the fixture\'s backpack and pouch');

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
    onCustomizeName: noop,
    onCustomizeModifier: noop,
  }),
);
assertEqual(nestHtml.split('data-testid="gear-container-').length - 1, 3, 'view: one box per container row');
assert(nestHtml.includes('data-testid="gear-row-customize-item:pouch_belt-0-1"'), 'view: each container row has its own Customize (E4.9)');
const pouchBox = nestHtml.slice(nestHtml.indexOf('data-testid="gear-container-item:pouch_belt-0-0"'), nestHtml.indexOf('data-testid="gear-container-item:pouch_belt-0-1"'));
assert(pouchBox.includes('1 lb, no limit'), 'view: the box line reads the load');
assert(pouchBox.includes('Whetstone') && pouchBox.includes('data-testid="gear-discard"') && pouchBox.includes('data-testid="gear-sell"') && pouchBox.includes('data-testid="gear-move-to"'), 'view: a content row keeps its buttons inside the box');
assert(nestHtml.includes('>Empty<'), 'view: an empty container reads Empty');
assert(nestHtml.indexOf('gear-row-item:dagger-0-0') < nestHtml.indexOf('data-testid="gear-container-item:backpack-0-0"'), 'view: the loose dagger comes before the backpack box');

// 18. Slot kinds (SD-42 E4.8, operator SLOT-1, `decisions.md §27.2`): a copy is offered only a slot of its own kind.
const slotIds = (targets: { id: string }[]) => targets.map((target) => target.id).filter((id) => id.startsWith('slot-'));
assertEqual(slotIds(moveTargets(model, row('Backpack'))).join(', '), '', 'slot kinds: a backpack is offered no slot section');
const shieldTargets = slotIds(moveTargets(model, row('Shield')));
assert(shieldTargets.includes('slot-shield') && !shieldTargets.includes('slot-head'), 'slot kinds: a shield is offered the Shield slot and not the Head');
assertEqual(shieldTargets.join(', '), 'slot-shield', 'slot kinds: a shield is offered only the Shield slot');
assertEqual(daggerTargets.filter((id) => id.startsWith('slot-')).join(', '), 'slot-wield_off, slot-wield_two_handed', 'slot kinds: a dagger is offered the free hands of the wield family');
assertEqual(slotKindsAllowed('wield_off').join(', '), 'wield_main, wield_off, wield_two_handed', 'slot kinds: the hands are one family');
const freeSlots = overridden.tree.filter((candidate) => candidate.location.kind === 'slot').map((candidate) => candidate.id);
assertEqual(slotIds(moveTargets(overridden, overridden.rows.find((candidate) => candidate.held.name === 'Backpack')!)).join(', '), freeSlots.join(', '), 'slot kinds: under the slot override every slot is offered');
assertEqual(dropTarget(dragging('item:backpack-0-0'), 'slot-shield', model), null, 'slot kinds: drop is null where the menu offers nothing');
assertEqual(dropTarget(dragging('item:heavy_steel_shield-0-0'), 'slot-head', model), null, 'slot kinds: a shield dropped on Head goes nowhere');

// 19. A copy saved in a wrong slot loads where it is and its row carries the engine's warning.
const saved = gearScreenModel([held('Hat of Disguise', { kind: 'slot', slot: 'shield' }, { slot: 'head', slotWarning: 'in the Shield slot, a Head item' })], { perItem: [], containerLoads: [], ignoreContainerCapacity: true });
assertEqual(names(saved.tree.find((candidate) => candidate.id === 'slot-shield')!.items), 'Hat of Disguise', 'saved wrong slot: it stays in the Shield slot');
assertEqual(saved.warnings.map((warning) => warning.text).join(' | '), 'Hat of Disguise is in the Shield slot, a Head item.', 'saved wrong slot: the row carries the warning');

// 20. Stacks (SD-42 E4.11, operator STK-1, `decisions.md §27.5`): identical copies at one place are one row, one tree node,
// one container table and one Into target; a copy holding anything is its own row (the engine's stack builder splits it).
const SACKS = [0, 1, 2, 3].map((ordinal) => key('item:sack', ordinal));
const sackLoads = (filled: number | null): ContainerLoadDto[] =>
  SACKS.map((copy, index) => ({ copy, contentsLbs: index === filled ? 1 : 0, capacity: { kind: 'pounds', lbs: 30 }, over: false }));
const fourSacks = gearScreenModel([held('Sack', { kind: 'carried' }, { copies: SACKS }), held('Dagger', { kind: 'carried' })], { perItem: [], containerLoads: sackLoads(null) });
const fourSackNodes = fourSacks.tree.find((candidate) => candidate.id === 'carried')!.items;
assertEqual(names(fourSackNodes), 'Dagger, Sack ×4', 'stacks: a container stack is one tree node');
assertEqual(copyId(fourSackNodes[1].copy), 'item:sack-0-0', 'stacks: the node stands for the stack\'s first copy');
assertEqual(boxes(fourSacks).map((table) => `${table.title} ${copyId(table.copy)}`).join(' | '), 'Sack ×4 item:sack-0-0', 'stacks: one container table titled Sack ×4');
const daggerInto = moveTargets(fourSacks, fourSacks.rows.find((candidate) => candidate.held.name === 'Dagger')!).filter((target) => target.id.startsWith('inside-'));
assertEqual(daggerInto.map((target) => `${target.id} ${target.label}`).join(' | '), 'inside-item:sack-0-0 Into Sack ×4', 'stacks: one Into target for identical empty containers');
const oneFilled = gearScreenModel(
  [held('Sack', { kind: 'carried' }, { copies: [SACKS[0]] }), held('Sack', { kind: 'carried' }, { copies: SACKS.slice(1) }), held('Dagger', { kind: 'inside', container: SACKS[0] })],
  { perItem: [], containerLoads: sackLoads(0) },
);
const filledNodes = oneFilled.tree.find((candidate) => candidate.id === 'carried')!.items;
assertEqual(names(filledNodes), 'Sack, Sack ×3', 'stacks: a filled copy is its own node');
assertEqual(names(filledNodes[0].children), 'Dagger', 'stacks: the filled node holds the dagger');
assertEqual(boxes(oneFilled).map((table) => table.title).join(' | '), 'Sack | Sack ×3', 'stacks: two container rows of one name are told apart by count, not #N');

// 21. Container truth (SD-43 E1.1, E0.4 gear-repro.md 5.N, 5.R, Q-5.1): no `#N`; a renamed copy keeps its name;
// tables and Into targets bind to the copy that holds items; an item put into the stack is a new single row.
const BEDROLL = held('Bedroll', { kind: 'inside', container: SACKS[0] });
const BLANKET = held('Blanket', { kind: 'inside', container: SACKS[0] });
const holding = gearScreenModel(
  [held('Sack', { kind: 'carried' }, { copies: [SACKS[0]] }), held('Sack', { kind: 'carried' }, { copies: SACKS.slice(1) }), BEDROLL, BLANKET, held('Dagger', { kind: 'carried' })],
  { perItem: [], containerLoads: sackLoads(0) },
);
const holdingTables = boxes(holding);
assertEqual(holdingTables.map((table) => `${table.title} ${copyId(table.copy)}`).join(' | '), 'Sack item:sack-0-0 | Sack ×3 item:sack-0-1', 'container truth: Sack and Sack ×3, each bound to its own copy');
assertEqual(names(holding.tree.find((candidate) => candidate.id === 'carried')!.items), 'Dagger, Sack, Sack ×3', 'container truth: the tree names them the same');
const holdingInto = moveTargets(holding, holding.rows.find((candidate) => candidate.held.name === 'Dagger')!).filter((target) => target.id.startsWith('inside-'));
assertEqual(holdingInto.map((target) => target.label).join(' | '), 'Into Sack | Into Sack ×3', 'container truth: Move to names them as the tables do');
assert(![...holdingTables.map((table) => table.title), ...holdingInto.map((target) => target.label)].some((label) => label.includes('#')), 'container truth: no #N anywhere');
// A renamed copy (`copies: {"Sack|0|1": {"name": "Custom Sack Name"}}`, E0.4 E9): its own name on the table, the tree and Move to.
const renamed = gearScreenModel(
  [{ ...held('Custom Sack Name', { kind: 'carried' }, { itemId: 'item:sack', copies: [SACKS[1]] }), baseName: 'Sack' }, held('Sack', { kind: 'carried' }, { copies: [SACKS[0], SACKS[2], SACKS[3]] }), held('Dagger', { kind: 'carried' })],
  { perItem: [], containerLoads: sackLoads(null) },
);
assertEqual(boxes(renamed).map((table) => table.title).join(' | '), 'Custom Sack Name | Sack ×3', 'container truth: a renamed copy titles its table by its own name');
assertEqual(names(renamed.tree.find((candidate) => candidate.id === 'carried')!.items), 'Dagger, Custom Sack Name, Sack ×3', 'container truth: and the tree');
assertEqual(moveTargets(renamed, renamed.rows.find((candidate) => candidate.held.name === 'Dagger')!).filter((target) => target.id.startsWith('inside-')).map((target) => target.label).join(' | '), 'Into Custom Sack Name | Into Sack ×3', 'container truth: and Move to');
// Bound to the copy holding items, not `copies[0]`: a row whose third copy holds the dagger reads that copy.
const misbound = gearScreenModel([held('Sack', { kind: 'carried' }, { copies: SACKS }), held('Dagger', { kind: 'inside', container: SACKS[2] })], { perItem: [], containerLoads: sackLoads(2) });
assertEqual(boxes(misbound).map((table) => `${copyId(table.copy)} ${table.loadLabel}: ${table.rows.map((candidate) => candidate.label).join(', ')}`).join(' | '), 'item:sack-0-2 1 of 30 lb: Dagger', 'container truth: the table binds to the copy holding the dagger');
assertEqual(copyId(misbound.tree.find((candidate) => candidate.id === 'carried')!.items[0].copy), 'item:sack-0-2', 'container truth: the tree node binds to it too');
// Into the stack (Q-5.1): before, Sack ×4 is one Into target; after the write the engine lists the filled copy as its own row.
const beforePut = moveTargets(fourSacks, fourSacks.rows.find((candidate) => candidate.held.name === 'Dagger')!).find((target) => target.id.startsWith('inside-'))!;
const afterPut = gearScreenModel(
  [held('Sack', { kind: 'carried' }, { copies: [SACKS[0]] }), held('Sack', { kind: 'carried' }, { copies: SACKS.slice(1) }), held('Dagger', { kind: 'inside', container: beforePut.location.kind === 'inside' ? beforePut.location.container : SACKS[3] })],
  { perItem: [], containerLoads: sackLoads(0) },
);
assertEqual(boxes(afterPut).map((table) => `${table.title}: ${table.rows.map((candidate) => candidate.label).join(', ') || 'Empty'}`).join(' | '), 'Sack: Dagger | Sack ×3: Empty', 'container truth: an item put into the stack is a new single row holding it');

// 22. Order (SD-43 E1.2, operator 5-b): non-containers A–Z, then containers A–Z, at every depth; natural number
// order (`Sack #2` before `Sack #10`, `decisions.md §5` collator default).
const SACK_10 = key('item:sack', 10);
const SACK_2 = key('item:sack', 2);
const INNER_SACK = key('item:sack', 5);
const ORDER_PACK = key('item:backpack');
const ORDER_POUCH = key('item:belt_pouch');
const ORDERED: PlacedEquipmentCopyDto[] = [
  held('Sack #10', { kind: 'carried' }, { itemId: 'item:sack', copies: [SACK_10] }),
  held('Sack #2', { kind: 'carried' }, { itemId: 'item:sack', copies: [SACK_2] }),
  held('Belt Pouch', { kind: 'carried' }),
  held('Backpack', { kind: 'carried' }),
  held('Rope', { kind: 'carried' }),
  held('Dagger', { kind: 'carried' }),
  held('Whetstone', { kind: 'inside', container: ORDER_PACK }),
  held('Sack', { kind: 'inside', container: ORDER_PACK }, { itemId: 'item:sack', copies: [INNER_SACK] }),
  held('Chalk', { kind: 'inside', container: ORDER_PACK }),
  held('flint and steel', { kind: 'inside', container: ORDER_PACK }),
  held('Twine', { kind: 'inside', container: INNER_SACK }),
  held('Candle', { kind: 'inside', container: INNER_SACK }),
  held('Torch', { kind: 'notCarried' }),
  held('Bedroll', { kind: 'notCarried' }),
];
const unlimited = (copy: CopyKeyDto): ContainerLoadDto => ({ copy, contentsLbs: 0, capacity: { kind: 'unlimited' }, over: false });
const ordered = gearScreenModel(ORDERED, { perItem: [], containerLoads: [SACK_10, SACK_2, INNER_SACK, ORDER_PACK, ORDER_POUCH].map(unlimited) });
const orderedCarried = ordered.tree.find((candidate) => candidate.id === 'carried')!.items;
assertEqual(names(orderedCarried), 'Dagger, Rope, Backpack, Belt Pouch, Sack #2, Sack #10', 'order: loose A–Z, then containers A–Z, Sack #2 before Sack #10');
const orderedPack = orderedCarried.find((item) => item.row.label === 'Backpack')!;
assertEqual(names(orderedPack.children), 'Chalk, flint and steel, Whetstone, Sack', 'order: inside the backpack, loose A–Z (case-insensitive) then the sack');
assertEqual(names(orderedPack.children[3].children), 'Candle, Twine', 'order: and inside the nested sack');
assertEqual(names(ordered.tree.find((candidate) => candidate.id === 'notCarried')!.items), 'Bedroll, Torch', 'order: Not Carried too');

// 23. One column (operator 5-a, 5-c): every location a section of one list, containers as boxes holding their
// contents, Not Carried its own section; the carried weight is the engine's `totalCarriedWeightLbs` as passed.
const columnHtml = renderToStaticMarkup(
  createElement(GearPanels, {
    model: ordered,
    armorClass: null,
    carriedLbs: 12.5,
    busy: false,
    refusal: null,
    onMove: noop,
    onDrop: noop,
    onSell: noop,
    onDiscard: noop,
    onSets: { activate: noop, create: noop, rename: noop, remove: noop },
    onCustomizeName: noop,
    onCustomizeModifier: noop,
  }),
);
assert(!columnHtml.includes('grid-template-columns') && !columnHtml.includes('<table'), 'one column: no side-by-side panels, no tables');
assert(columnHtml.includes('12.5 lb carried'), 'one column: the carried weight is the engine figure, with Not Carried items present');
const at = (testId: string) => columnHtml.indexOf(`data-testid="${testId}"`);
assert(at('gear-location-carried') < at('gear-row-item:dagger-0-0') && at('gear-row-item:dagger-0-0') < at('gear-row-item:sack-0-2') && at('gear-row-item:sack-0-2') < at('gear-row-item:sack-0-10'), 'one column: Carried lists its rows in order');
const notCarriedHtml = columnHtml.slice(at('gear-location-notCarried'));
assert(notCarriedHtml.includes('Not counted in carried weight') && notCarriedHtml.includes('gear-row-item:torch-0-0') && notCarriedHtml.includes('gear-row-item:bedroll-0-0'), 'one column: Not Carried is its own section, not counted');
assert(at('gear-row-item:torch-0-0') > at('gear-location-notCarried') && at('gear-row-item:dagger-0-0') < at('gear-location-notCarried'), 'one column: Not Carried rows are not in Carried');
const packBox = columnHtml.slice(at('gear-container-item:backpack-0-0'), at('gear-container-item:belt_pouch-0-0'));
assert(packBox.includes('gear-row-item:backpack-0-0') && packBox.includes('gear-row-item:chalk-0-0') && packBox.includes('gear-container-item:sack-0-5') && packBox.includes('gear-row-item:candle-0-0'), 'one column: the backpack box holds its contents and the nested sack box');
const chalkLine = packBox.slice(packBox.indexOf('gear-row-item:chalk-0-0'), packBox.indexOf('gear-row-item:flint_and_steel-0-0'));
assert(['Chalk', '>Customize<', '>Move to<', '>Drop<', '>Sell<', '>Discard<'].every((part) => chalkLine.includes(part)), 'one column: the buttons sit on the item\'s line');
assert(columnHtml.slice(at('gear-container-item:belt_pouch-0-0')).includes('>Empty<'), 'one column: an empty container box reads Empty');

console.log('gearScreenModel.test.ts: 23 cases passed');
