import { createElement, isValidElement, type ReactElement, type ReactNode } from 'react';
import { readFileSync } from 'node:fs';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { CustomizeCopyPanel, renameRequest, type CustomizeCopyPanelProps } from './CustomizeCopyDialog';

const sack = { itemId: 'Sack', bonus: 0, ordinal: 1 };
const held = (over: Partial<PlacedEquipmentCopyDto> = {}): PlacedEquipmentCopyDto => ({
  itemId: 'Sack',
  name: 'Sack',
  baseName: 'Sack',
  bonus: 0,
  activeState: 'SelectedInactive',
  slot: 'none',
  location: { kind: 'carried' },
  ordinal: 1,
  quantity: 1,
  copies: [sack],
  baseQuantity: 1,
  ...over,
});

const panelProps = (over: Partial<CustomizeCopyPanelProps> = {}): CustomizeCopyPanelProps => ({
  title: 'Sack #2',
  held: held(),
  name: 'Sack',
  busy: false,
  onNameChange: () => {},
  onSave: () => {},
  onAttach: () => {},
  onClose: () => {},
  ...over,
});

function collect(node: ReactNode, into: ReactElement<Record<string, unknown>>[] = []) {
  if (Array.isArray(node)) {
    node.forEach((child) => collect(child, into));
  } else if (isValidElement(node)) {
    const element = node as ReactElement<Record<string, unknown>>;
    into.push(element);
    collect(element.props.children as ReactNode, into);
  }
  return into;
}
const byTestId = (node: ReactNode, id: string) => collect(node).find((element) => element.props['data-testid'] === id);

function saveCallsRenameEquipmentCopyWithTheCopyAndTheTrimmedName() {
  assertEqual(JSON.stringify(renameRequest('char-1', sack, '  Camping Gear ')), JSON.stringify({ characterId: 'char-1', copy: sack, name: 'Camping Gear' }), 'the copy and the trimmed name');
  let saved = 0;
  (byTestId(CustomizeCopyPanel(panelProps({ onSave: () => (saved += 1) })), 'gear-customize-save')!.props.onClick as () => void)();
  assertEqual(saved, 1, 'Save runs the save');
  // The gear screen sends that request through the rename boundary, as one of its writes.
  const screen = readFileSync(new URL('./GearScreen.tsx', import.meta.url), 'utf8');
  assert(screen.includes('renameEquipmentCopy(renameRequest(characterId, copy, text))'), 'GearScreen renames through renameEquipmentCopy');
  const dialog = readFileSync(new URL('./CustomizeCopyDialog.tsx', import.meta.url), 'utf8');
  assert(dialog.includes('props.onRename(props.target.copy, name)'), 'the dialog saves this copy');
}

function blankNameRestoresTheBaseName() {
  assertEqual(renameRequest('char-1', sack, '   ').name, '', 'blank clears the custom name (the backend restores the record name)');
  const html = renderToStaticMarkup(createElement(CustomizeCopyPanel, panelProps({ held: held({ name: 'Camping Gear' }), name: 'Camping Gear' })));
  assert(html.includes('placeholder="Sack"'), 'the base name is the placeholder');
  assert(html.includes('Leave blank for Sack.'), 'and the words say so');
  assert(html.includes('value="Camping Gear"'), 'prefilled with the current name');
}

function attachModifierOpensTheModifierPickerForThisCopy() {
  let attached = 0;
  (byTestId(CustomizeCopyPanel(panelProps({ onAttach: () => (attached += 1) })), 'gear-customize-attach')!.props.onClick as () => void)();
  assertEqual(attached, 1, 'the button runs the attach');
  const sheet = readFileSync(new URL('./CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(sheet.includes('setPendingModifierAttachment(target);'), 'the sheet remembers the copy');
  assert(sheet.includes("setItemPickerOpen('modifier');"), 'and opens the Equipmods picker');
  assert(/copy: target\.copy,/.test(sheet), 'the pick attaches to that copy');
}

function appliedModifiersListWhatTheCopyCarries() {
  const html = renderToStaticMarkup(createElement(CustomizeCopyPanel, panelProps({ held: held({ appliedModifiers: ['+1', 'Flaming'] }) })));
  assert(html.includes('<li style="font-size:0.8rem">+1</li>') && html.includes('Flaming'), 'each modifier listed');
  assert(renderToStaticMarkup(createElement(CustomizeCopyPanel, panelProps())).includes('None attached.'), 'none when none');
  assert(renderToStaticMarkup(createElement(CustomizeCopyPanel, panelProps())).includes('role="dialog"'), 'a dialog');
}

saveCallsRenameEquipmentCopyWithTheCopyAndTheTrimmedName();
blankNameRestoresTheBaseName();
attachModifierOpensTheModifierPickerForThisCopy();
appliedModifiersListWhatTheCopyCarries();
console.log('CustomizeCopyDialog.test.ts: all assertions passed');
