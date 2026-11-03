import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { assert, assertEqual } from '../testSupport/asserts';
import { EquipmentPanelView, equipmentPanelLists, wornLabel } from './EquipmentPanel';

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

// An Absent copy is not held; the backend leaves it out, and nothing here would list it as worn.
assertEqual(equipmentPanelLists([copy('Gone', 'Absent', 'head')]).worn.length, 0, 'Absent is not worn');
assertEqual(equipmentPanelLists([copy('Gone', 'Absent', 'head')]).carried.length, 0, 'Absent is not carried');

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

const refusal = 'head slot is taken by Hat of Disguise';
assert(render({ refusal }).includes(`role="alert"`), 'a refusal is an alert');
assert(render({ refusal }).includes(`>${refusal}<`), 'the refusal is shown word for word, alone in its element');
assert(!html.includes('role="alert"'), 'no alert without a refusal');

assert(render({ copies: null }).includes('Loading'), 'not yet read is not the same as holding nothing');
assert(render({ copies: [] }).includes('Nothing held yet.'), 'an empty list says so');
const failed = render({ copies: null, loadError: 'Failed to list held equipment: gone' });
assert(failed.includes('Failed to list held equipment: gone') && !failed.includes('Loading'), 'a failed read says why');
assert(render({ busy: true }).includes('disabled'), 'buttons wait while a change is saving');
