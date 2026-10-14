/**
 * SD-38 E4.1 -- the Cyberpunk RED sheet core (`cyberpunkSheetModel.ts` and the sheet it lays out), engine-single-source.
 *
 * Renders the real `load_saved_character` responses of two Cyberpunk RED characters
 * (`__tests__/cyberpunkSheetFixtures/<id>.json`, kept equal to `CyberpunkAdapter`'s answer by
 * `cpr_adapter::tests::the_frontend_cyberpunk_sheet_fixtures_are_the_adapters_load_responses`):
 * a bare Solo Streetrat and a kitted Solo (armour, cyberware, two Critical Injuries), and proves:
 *
 * 1. A Cyberpunk RED character opens this sheet; a Pathfinder or Starfinder one does not.
 * 2. The sheet's boxes are placed by `cyberpunkSheetLayout.ts`, column by column, in `order`,
 *    and none of them is an encumbrance box (D-12): not in the registry, not in the render.
 * 3. The core tiles are there: the ten STATs, EMP as entered and current, Hit Points, Seriously
 *    Wounded, Death Save, Humanity, MOVE, LUCK and the wound track.
 * 4. Every number on the sheet is one engine row: each `data-cpr-row` value is that row's
 *    value, every row appears exactly once, no other digit is written, and moving every row by
 *    1000 moves every number by 1000.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { CyberpunkCharacterSheet } from './CyberpunkCharacterSheet';
import { CYBERPUNK_SHEET_COLUMNS, CYBERPUNK_SHEET_LAYOUT } from './cyberpunkSheetLayout';
import { buildCyberpunkSheet, isCyberpunkCharacter } from './cyberpunkSheetModel';
import { boxesInColumn } from './sheetLayout';
import { assert, assertEqual } from '../testSupport/asserts';

const CHARACTERS = ['cpr-solo-streetrat', 'cpr-solo-kitted'] as const;

/** The rows the criterion names, each of which must be a tile on every character's sheet. */
const CORE_ROWS = [
  ...['int', 'ref', 'dex', 'tech', 'cool', 'will', 'luck', 'move', 'body', 'emp'].map((stat) => `cpr.stat.${stat}`),
  'cpr.derived.emp_current',
  'cpr.derived.hit_points',
  'cpr.derived.seriously_wounded',
  'cpr.derived.death_save',
  'cpr.derived.humanity_current',
  'cpr.derived.humanity_max',
  'cpr.derived.move',
  'cpr.derived.luck_pool',
  'cpr.wound.lightly_wounded',
  'cpr.wound.seriously_wounded',
  'cpr.wound.mortally_wounded',
  'cpr.wound.death_save',
  'cpr.wound.base_death_save_penalty',
];

function fixture(id: string): LoadSavedCharacterResponse {
  return JSON.parse(readFileSync(new URL(`./__tests__/cyberpunkSheetFixtures/${id}.json`, import.meta.url), 'utf8')) as LoadSavedCharacterResponse;
}

function render(detail: LoadSavedCharacterResponse): string {
  return decode(renderToStaticMarkup(createElement(CyberpunkCharacterSheet, { detail, onClose: () => undefined, onOpen: () => undefined })));
}

function decode(text: string): string {
  return text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

/** `data-cpr-row` id -> rendered text, in render order. */
function renderedRows(html: string): Array<[string, string]> {
  return [...html.matchAll(/<span[^>]*data-cpr-row="([^"]+)"[^>]*>([^<]*)<\/span>/g)].map((m) => [m[1], m[2]]);
}

// 2. The registry has no encumbrance box (D-12), and each column holds its boxes in `order`.
assertEqual(
  CYBERPUNK_SHEET_LAYOUT.filter((box) => /encumbr/i.test(box.id)).length,
  0,
  'D-12: the Cyberpunk RED sheet layout has no encumbrance box',
);

let checkedRows = 0;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  assertEqual(detail.summary.gameSystem, 'cyberpunk-red', `${id}: the fixture is a Cyberpunk RED character`);
  assert(detail.explanations.length > 0 && detail.explanations.every((row) => row.id.startsWith('cpr.')), `${id}: the engine sent cpr. rows only`);

  // 1. It opens the Cyberpunk RED sheet.
  assert(isCyberpunkCharacter(detail.summary), `${id}: opens the Cyberpunk RED sheet`);

  const html = render(detail);
  assert(!/encumbr/i.test(html), `${id}: D-12: no encumbrance box on the rendered sheet`);

  // 2. Boxes render column by column, in the registry's order.
  const renderedBoxes = [...html.matchAll(/data-cpr-box="([^"]+)"/g)].map((m) => m[1]);
  const placed = new Set(buildCyberpunkSheet(detail).boxes.map((box) => box.id));
  const expectedBoxes = CYBERPUNK_SHEET_COLUMNS.flatMap((column) => boxesInColumn(CYBERPUNK_SHEET_LAYOUT, column).map((box) => box.id)).filter((box) =>
    placed.has(box),
  );
  assertEqual(renderedBoxes.join(','), expectedBoxes.join(','), `${id}: boxes follow cyberpunkSheetLayout.ts`);
  for (const box of ['stats', 'vitals', 'woundTrack']) {
    assert(renderedBoxes.includes(box), `${id}: the ${box} box is laid out`);
  }

  // 3. The core tiles, each inside its box (never in "Other totals").
  const rows = renderedRows(html);
  const renderedIds = new Set(rows.map(([rowId]) => rowId));
  const other = buildCyberpunkSheet(detail).boxes.find((box) => box.id === 'other');
  for (const rowId of CORE_ROWS) {
    assert(renderedIds.has(rowId), `${id}: the ${rowId} tile is on the sheet`);
    assert(other === undefined || other.rows.every((row) => row.label !== rowId), `${id}: ${rowId} fell to Other totals`);
  }
  for (const label of ['Hit Points', 'Seriously Wounded', 'Death Save', 'Humanity', 'MOVE', 'LUCK', 'EMP', 'Lightly Wounded', 'Mortally Wounded']) {
    assert(html.includes(`>${label}<`), `${id}: the ${label} label is laid out`);
  }

  // 4a. Every data-cpr-row value is its row's value; every row appears exactly once.
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  for (const [rowId, text] of rows) {
    const value = byId.get(rowId);
    assert(value !== undefined, `${id}: rendered ${rowId} is not an engine row`);
    assertEqual(text, String(value), `${id}: ${rowId}`);
  }
  assertEqual(
    rows.map(([rowId]) => rowId).sort().join(','),
    detail.explanations.map((row) => row.id).sort().join(','),
    `${id}: the rendered rows are the engine's rows, each once`,
  );
  checkedRows += rows.length;

  // 4b. No digit is written anywhere else (the character's name is the player's text; a label
  // read from a row id -- "cpr.weapon.0.rof" -- carries only digits of its own row ids).
  let rest = html.replace(/<span[^>]*data-cpr-row="[^"]+"[^>]*>[^<]*<\/span>/g, '');
  rest = rest.replace(/<[a-z0-9]+[^>]*data-cpr-name[^>]*>[^<]*<\/[a-z0-9]+>/g, '');
  rest = rest.replace(/<[a-z0-9]+[^>]*data-cpr-label-of="([^"]+)"[^>]*>([^<]*)<\/[a-z0-9]+>/g, (_whole, ids: string, label: string) => {
    for (const digits of label.match(/\d+/g) ?? []) {
      assert(ids.includes(digits), `${id}: label ${label} has a digit outside its rows ${ids}`);
    }
    return '';
  });
  const stray = rest.replace(/<[^>]*>/g, ' ').match(/[^\s]*\d[^\s]*/g);
  assert(stray === null, `${id}: numbers on the sheet that are no engine row: ${JSON.stringify(stray)}`);

  // 4c. Move every row by 1000: every number on the sheet moves with it.
  const moved = { ...detail, explanations: detail.explanations.map((row) => ({ ...row, value: row.value + 1000 })) };
  const movedRows = renderedRows(render(moved));
  assertEqual(movedRows.length, rows.length, `${id}: the shifted sheet shows the same rows`);
  for (const [rowId, text] of movedRows) {
    assertEqual(text, String((byId.get(rowId) as number) + 1000), `${id}: ${rowId} follows its row`);
  }

  // A refused build lays out no totals and shows its blocking diagnostic.
  const refused = buildCyberpunkSheet({
    ...detail,
    explanations: [],
    diagnostics: [{ id: 'cpr.refused', message: 'refused', claimBlocking: true }],
  });
  assertEqual(refused.boxes.length, 0, `${id}: a refused build has no totals`);
  assertEqual(refused.blocking.length, 1, `${id}: a refused build shows its blocking diagnostic`);
}

// The kitted Solo's EMP current and entered are two different rows with two different values
// (Humanity Loss lowered it), and both are on the sheet: the EMP tile is max and current.
const kitted = fixture('cpr-solo-kitted');
const value = (rowId: string) => kitted.explanations.find((row) => row.id === rowId)?.value;
assert(value('cpr.derived.emp_current') !== value('cpr.stat.emp'), 'the kitted fixture lowers EMP, so max/current is exercised');
assert(value('cpr.wound.base_death_save_penalty') !== 0, 'the kitted fixture raises the Base Death Save Penalty');

for (const gameSystem of ['pf1', 'starfinder-1e']) {
  assertEqual(isCyberpunkCharacter({ gameSystem }), false, `a ${gameSystem} character does not open the Cyberpunk RED sheet`);
}
assert(checkedRows > 0, 'rows were checked');
console.log(`cyberpunkSheetModel: ${CHARACTERS.length} characters, ${checkedRows} engine rows rendered, each = its row`);
