import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { readFileSync } from 'node:fs';
import { assert, assertEqual } from '../../testSupport/asserts';
import type { EncumbranceDto } from '../../boundary/loadCreateCharacter';
import type { ExplanationDto } from '../../boundary/loadSavedCharacterDetail';
import { SQUARE_BLOCK_LAYOUT, boxesInColumn } from '../sheetLayout';
import { INITIATIVE_TOTAL_ID, SquareBlock, conditionsLabel, initiativeRow } from './SquareBlock';

// Brief 2 L2: the 2x2 square block under the name on the sheet (Hit Points, Initiative,
// Encumbrance, Conditions). Rendered to static markup; there is no DOM on this box.

const INITIATIVE: ExplanationDto = {
  id: 'initiative.total',
  value: 7,
  detail: 'Initiative +7: Dexterity modifier +3; Improved Initiative +4.',
} as ExplanationDto;

const LIGHT_LOAD: EncumbranceDto = {
  totalCarriedWeightLbs: 29,
  totalCarriedCostGp: 115,
  lightMaxLbs: 76,
  mediumMaxLbs: 153,
  heavyMaxLbs: 230,
  level: 'Light',
  loadArmorCheckPenalty: 0,
  perItem: [],
  unresolvedItemIds: [],
};

const render = (over: Partial<Parameters<typeof SquareBlock>[0]> = {}) =>
  renderToStaticMarkup(
    createElement(SquareBlock, { hp: 12, initiative: INITIATIVE, encumbrance: LIGHT_LOAD, conditions: [], ...over }),
  );

function verifiesTheBlockIsRegisteredTwoByTwoInReadingOrder() {
  const ids = boxesInColumn(SQUARE_BLOCK_LAYOUT, 'square').map((box) => box.id);
  assertEqual(ids.join('|'), 'hitPoints|initiative|encumbrance|conditions', 'HP, Initiative / Encumbrance, Conditions');
  for (const box of SQUARE_BLOCK_LAYOUT) {
    assertEqual(box.boxStyle.aspectRatio, '1', `${box.id} is a square box`);
  }
  const html = render();
  assert(html.includes('grid-template-columns:1fr 1fr'), 'the block is a two-column grid');
  const order = ['Hit Points', 'Initiative', 'Encumbrance', 'Conditions'].map((label) => html.indexOf(label));
  assert(order.every((at) => at >= 0), `every box is drawn, got ${order.join(',')}`);
  assert(order.every((at, i) => i === 0 || at > order[i - 1]), 'boxes render in registry order');
}

function verifiesInitiativeIsTheEngineRowVerbatimAndProminent() {
  assertEqual(INITIATIVE_TOTAL_ID, 'initiative.total', 'the engine id lane E emits');
  const rows = [{ id: 'defense.touch_armor_class', value: 11, detail: 'x' } as ExplanationDto, INITIATIVE];
  assertEqual(initiativeRow(rows)?.value, 7, 'initiativeRow finds the initiative.total row');
  assertEqual(initiativeRow([]), null, 'no row is null, never a defaulted 0');
  const html = render();
  assert(html.includes('+7'), 'the engine value renders signed');
  assert(html.includes(INITIATIVE.detail), 'the engine detail renders verbatim');
  assert(html.includes('font-size:2rem'), 'initiative is the prominent number of the block');
}

function verifiesAMissingInitiativeRowReadsAbsent() {
  const html = render({ initiative: null });
  assert(html.includes('Initiative'), 'the box still renders');
  assert(html.includes('—'), 'an absent engine row reads as an em dash');
  assert(!html.includes('+0'), 'no made-up +0');
}

function verifiesConditionsReadNormalWhenEmpty() {
  assertEqual(conditionsLabel([]), 'Normal', 'empty list reads Normal');
  assertEqual(conditionsLabel(['Shaken', 'Fatigued']), 'Shaken, Fatigued', 'real conditions are listed');
  assert(render().includes('Normal'), 'the box shows Normal for an empty list');
  const html = render({ conditions: ['Shaken'] });
  assert(html.includes('Shaken') && !html.includes('Normal'), 'a listed condition replaces Normal');
}

function verifiesEncumbranceUsesDescribeEncumbrance() {
  const html = render();
  assert(html.includes('Light Load'), 'the load tier label from describeEncumbrance');
  assert(html.includes('29 lb'), 'the carried weight');
  assert(html.includes('76 / 153 / 230 lb'), 'the capacity thresholds');
  const absent = render({ encumbrance: undefined });
  assert(absent.includes('Encumbrance') && absent.includes('—'), 'no engine encumbrance reads as an em dash');
}

function verifiesHitPoints() {
  assert(render().includes('12 / 12'), 'hit points render as current / max');
  assert(render({ hp: null }).includes('Unknown'), 'an unknown hit die reads Unknown');
}

function verifiesTheSheetRendersTheBlockFromTheEngineRow() {
  const source = readFileSync(new URL('../CharacterSheet.tsx', import.meta.url), 'utf8');
  assert(!source.includes('InitiativeHpPanel'), 'the Dex-only initiative panel is retired');
  assert(!/initiative=\{dexMod\}/.test(source), 'no dexMod initiative shortcut remains');
  assert(source.includes('<SquareBlock'), 'the sheet renders the 2x2 block');
  assert(source.includes('initiativeRow(engineRecords.explanations)'), 'initiative comes from the engine explanations');
  assert(source.includes('conditions={custom.conditions}'), 'conditions come from the custom record');
  const name = source.indexOf('<IdentityPanel');
  const block = source.indexOf('<SquareBlock');
  const stats = source.indexOf('SHEET_COLUMNS.map');
  assert(name >= 0 && name < block && block < stats, 'the block sits under the name, above the stat block');
}

verifiesTheBlockIsRegisteredTwoByTwoInReadingOrder();
verifiesInitiativeIsTheEngineRowVerbatimAndProminent();
verifiesAMissingInitiativeRowReadsAbsent();
verifiesConditionsReadNormalWhenEmpty();
verifiesEncumbranceUsesDescribeEncumbrance();
verifiesHitPoints();
verifiesTheSheetRendersTheBlockFromTheEngineRow();
console.log('squareBlock.test.ts: all assertions passed');
