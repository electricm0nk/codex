import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { assert, assertEqual } from '../testSupport/asserts';
import { EquipmentPanelView, equipmentPanelLists, equipmentRowActions, wornLabel } from './EquipmentPanel';

function copy(name: string, activeState: PlacedEquipmentCopyDto['activeState'], slot: PlacedEquipmentCopyDto['slot'], bonus = 0): PlacedEquipmentCopyDto {
  return { itemId: name, name, bonus, activeState, slot };
}

const HELD: PlacedEquipmentCopyDto[] = [
  copy('Ring of Sustenance', 'EquippedActive', 'ring'),
  copy('Backpack', 'EquippedActive', 'none'),
  copy('Dagger', 'EquippedActive', 'wield_off'),
  copy('Longsword', 'EquippedActive', 'wield_main'),
  copy('Ring of Protection +1', 'EquippedActive', 'ring'),
  copy('Hat of Disguise', 'SelectedInactive', 'head'),
  copy('+1 Chain Shirt', 'SelectedInactive', 'armor', 1),
  copy('Chain Shirt', 'SelectedInactive', 'armor'),
];

const lists = equipmentPanelLists(HELD);
assertEqual(
  lists.worn.map((row) => row.name).join(', '),
  'Longsword, Dagger, Ring of Sustenance, Ring of Protection +1, Backpack',
  'worn rows run by slot, the hands first and what takes no slot last; two in one slot stay in saved order',
);
assertEqual(lists.carried.map((row) => row.name).join(', '), 'Hat of Disguise, +1 Chain Shirt, Chain Shirt', 'carried rows stay in saved order');
assertEqual(wornLabel('wield_off'), 'Off hand', 'the slot the backend placed the copy in');
assertEqual(wornLabel('none'), 'No slot', 'a worn item with no slot says so');

// An Absent copy is owned and on neither list of what the character has with them.
const away = equipmentPanelLists([copy('Gone', 'Absent', 'head'), copy('+1 Dagger', 'Absent', 'wield_main', 1)]);
assertEqual(away.worn.length, 0, 'Absent is not worn');
assertEqual(away.carried.length, 0, 'Absent is not carried');
assertEqual(away.leftBehind.map((row) => row.name).join(', '), 'Gone, +1 Dagger', 'Absent has its own group, in saved order');

const calls: Array<[string, number, string]> = [];
function render(props: Partial<Parameters<typeof EquipmentPanelView>[0]>) {
  return renderToStaticMarkup(
    createElement(EquipmentPanelView, {
      copies: HELD,
      loadError: null,
      refusal: null,
      busy: false,
      onSetState: (row, state) => calls.push([row.name, row.bonus, state]),
      ...props,
    }),
  );
}

const html = render({});
assert(html.includes('Equipment'), 'the control is named');
assert(html.includes('Main hand: Longsword'), 'a worn row reads slot then copy');
assert(html.includes('Off hand: Dagger'), 'the second weapon reads as the off hand');
assert(html.includes('>Stow +1 Chain Shirt<') === false, 'a carried copy is not offered Stow');
assert(html.includes('>Equip +1 Chain Shirt<'), 'a carried copy is offered Equip, by the name of that copy');
assert(html.includes('>Stow Longsword<'), 'a worn copy is offered Stow');
assert(html.includes('>Equip Longsword<') === false, 'a worn copy is not offered Equip');

// Every owned copy has three states; each row offers what it can move to.
assert(html.includes('>Leave Longsword behind<'), 'a worn copy can be left behind');
assert(html.includes('>Leave +1 Chain Shirt behind<'), 'a carried copy can be left behind');
assert(html.includes('>Left behind<'), 'the third group is named');
assert(html.includes('Nothing left behind.'), 'and says when it is empty');
const owned = render({ copies: [copy('+1 Dagger', 'Absent', 'wield_main', 1)] });
assert(owned.includes('>+1 Dagger<') || owned.includes('+1 Dagger</span>'), 'a left-behind copy stays on the panel and keeps its +N');
assert(owned.includes('>Pick up +1 Dagger<'), 'and can be picked up');
assert(!owned.includes('Leave +1 Dagger behind'), 'it is already left behind');
assert(!owned.includes('Equip +1 Dagger'), 'it comes back through Carried');
assert(!owned.includes('Nothing held yet.'), 'a character whose items are all left behind holds something');
const moves = (row: PlacedEquipmentCopyDto) => equipmentRowActions(row).map((action) => `${action.label} > ${action.to}`).join('; ');
assertEqual(moves(copy('Sword', 'EquippedActive', 'wield_main')), 'Stow Sword > SelectedInactive; Leave Sword behind > Absent', 'a worn row');
assertEqual(moves(copy('Sword', 'SelectedInactive', 'wield_main')), 'Equip Sword > EquippedActive; Leave Sword behind > Absent', 'a carried row');
assertEqual(moves(copy('Sword', 'Absent', 'wield_main')), 'Pick up Sword > SelectedInactive', 'a left-behind row');

const refusal = 'head slot is taken by Hat of Disguise';
assert(render({ refusal }).includes(`role="alert"`), 'a refusal is an alert');
assert(render({ refusal }).includes(`>${refusal}<`), 'the refusal is shown word for word, alone in its element');
assert(!html.includes('role="alert"'), 'no alert without a refusal');

assert(render({ copies: null }).includes('Loading'), 'not yet read is not the same as holding nothing');
assert(render({ copies: [] }).includes('Nothing held yet.'), 'an empty list says so');
const failed = render({ copies: null, loadError: 'Failed to list held equipment: gone' });
assert(failed.includes('Failed to list held equipment: gone') && !failed.includes('Loading'), 'a failed read says why');
assert(render({ busy: true }).includes('disabled'), 'buttons wait while a change is saving');

const tagged = render({ copies: [{ ...copy('Hat of Disguise', 'EquippedActive', 'head'), override: 'ignoreSlotLimits' }, copy('Helm of Brilliance', 'EquippedActive', 'head'), { ...copy('Full Plate', 'SelectedInactive', 'armor'), override: 'ignoreEncumbrance' }] });
assert(tagged.includes('Override: Ignore body slot limits'), 'a worn copy the override let on says so');
assert(tagged.includes('Override: Ignore encumbrance'), 'a carried copy the override let in says so');
assert(!html.includes('Override:'), 'a copy no override let in has no tag');
