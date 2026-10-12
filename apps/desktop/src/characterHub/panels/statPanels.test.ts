import type { ReactElement, ReactNode } from 'react';
import { assert, assertEqual } from '../../testSupport/asserts';
import { AbilitiesPanel } from './AbilitiesPanel';
import { ArmorClassPanel, AttackPanel, SavingThrowsPanel, SpeedPanel } from './CombatPanels';
import { StatBox, StatTile } from './StatBox';

// Brief 2 L1: the shared stat panels the sheet renders from its layout registry (and Create will
// reuse in L3). Panels are plain function components, so these tests call them and inspect the
// returned element tree; there is no DOM on this box.

type Element = ReactElement<{ children?: ReactNode; style?: Record<string, unknown>; title?: string; label?: string; value?: ReactNode }>;

function childrenOf(element: Element): Element[] {
  const kids = element.props.children;
  return (Array.isArray(kids) ? kids.flat() : [kids]).filter((kid): kid is Element => typeof kid === 'object' && kid !== null);
}

function textOf(node: ReactNode): string {
  if (node === null || node === undefined || typeof node === 'boolean') return '';
  if (typeof node === 'string' || typeof node === 'number') return String(node);
  if (Array.isArray(node)) return node.map(textOf).join('');
  return textOf((node as Element).props.children);
}

const SCORES = { strength: 16, dexterity: 13, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
const MODS = { strength: 3, dexterity: 1, constitution: 2, intelligence: 0, wisdom: 1, charisma: -1 };

function verifiesAbilitiesRenderAsOneVerticalColumn() {
  const box = AbilitiesPanel({ scores: SCORES, abilities: MODS }) as Element;
  assertEqual(box.type, StatBox, 'Abilities is a titled stat box');
  assertEqual(box.props.title, 'Abilities', 'its title is Abilities');
  const [rows] = childrenOf(box);
  assertEqual(rows.props.style?.flexDirection, 'column', 'the six scores stack top to bottom');
  assert(rows.props.style?.gridTemplateColumns === undefined, 'no six-across grid remains');
  const rowElements = childrenOf(rows);
  assertEqual(rowElements.length, 6, 'one row per ability');
  const labels = rowElements.map((row) => textOf(childrenOf(row)[0]));
  assertEqual(labels.join('|'), 'STR|DEX|CON|INT|WIS|CHA', 'abilities run STR to CHA');
  const strRow = textOf(rowElements[0]);
  assert(strRow.includes('16') && strRow.includes('+3'), `STR row shows score 16 and modifier +3, got "${strRow}"`);
  const chaRow = textOf(rowElements[5]);
  assert(chaRow.includes('8') && chaRow.includes('-1'), `CHA row shows score 8 and modifier -1, got "${chaRow}"`);
}

function tileLabels(box: Element): string[] {
  const [row] = childrenOf(box);
  return childrenOf(row).map((tile) => {
    assertEqual(tile.type, StatTile, 'each value is a stat tile');
    return String(tile.props.label);
  });
}

function verifiesCombatPanelsAreOwnTitledBoxes() {
  const ac = ArmorClassPanel({ ac: 17, touch: 11, flatFooted: 16 }) as Element;
  assertEqual(ac.props.title, 'Armor Class', 'AC box title');
  assertEqual(tileLabels(ac).join('|'), 'AC|Touch|Flat-Footed', 'AC tiles unchanged');

  const attack = AttackPanel({ baseAttackBonus: 1, melee: 4, cmb: 4, cmd: 15 }) as Element;
  assertEqual(attack.props.title, 'Attack', 'Attack box title');

  const speed = SpeedPanel({ land: '30 ft.' }) as Element;
  assertEqual(speed.props.title, 'Speed', 'Speed box title');
  assertEqual(tileLabels(speed).join('|'), 'Land|Fly|Swim|Climb', 'Speed tiles unchanged');

  const saves = SavingThrowsPanel({ saves: { fortitude: 4, reflex: 1, will: -1 } }) as Element;
  assertEqual(saves.props.title, 'Saving Throws', 'Saves box title');
  assertEqual(tileLabels(saves).join('|'), 'Fortitude|Reflex|Will', 'Saves tiles unchanged');
  const values = childrenOf(childrenOf(saves)[0]).map((tile) => String(tile.props.value));
  assertEqual(values.join('|'), '+4|+1|-1', 'saves are signed');
}

verifiesAbilitiesRenderAsOneVerticalColumn();
verifiesCombatPanelsAreOwnTitledBoxes();
console.log('statPanels.test.ts: all assertions passed');
