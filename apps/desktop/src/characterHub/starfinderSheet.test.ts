/**
 * SD-37 E6.3 -- the Starfinder sheet layout, engine-single-source.
 *
 * Renders each of the four Starfinder seeds' real `load_saved_character`
 * responses (`starfinderSheetFixtures/<seed>.json`, kept equal to the adapter's
 * answer by `sf_adapter::tests::the_frontend_starfinder_sheet_fixtures_are_the_adapters_load_responses`)
 * and proves two things:
 *
 * 1. No Pathfinder-only tile: the render holds no `CMB`, `CMD`, `Touch` or
 *    `Flat-Footed` the sheet itself wrote. The only such words are the
 *    engine's own printed spell text ("Range: Touch"), counted from the
 *    `sheetLines` and matched one for one; with the printed lines removed the
 *    render holds none.
 * 2. Every number on the sheet is an engine explanation row: each
 *    `data-sf-row` value is that row's value, every row appears exactly once,
 *    no other digit is written outside the printed engine text, and moving
 *    every row by 1000 moves every number by 1000.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { LoadSavedCharacterResponse, SheetLineDto } from '../boundary/loadSavedCharacterDetail';
import { StarfinderCharacterSheet } from './StarfinderCharacterSheet';
import { buildStarfinderSheet, isStarfinderCharacter } from './starfinderSheetModel';
import { assert, assertEqual } from '../testSupport/asserts';

const SEEDS = ['SF-Soldier-3', 'SF-Mystic-5', 'SF-Technomancer-5', 'SF-Envoy-3'] as const;
const PATHFINDER_ONLY = /CMB|CMD|Touch|Flat-Footed/g;

function fixture(seed: string): LoadSavedCharacterResponse {
  return JSON.parse(readFileSync(new URL(`./starfinderSheetFixtures/${seed}.json`, import.meta.url), 'utf8')) as LoadSavedCharacterResponse;
}

function render(detail: LoadSavedCharacterResponse): string {
  return renderToStaticMarkup(createElement(StarfinderCharacterSheet, { detail, onClose: () => undefined, onOpen: () => undefined }));
}

function count(text: string, pattern: RegExp): number {
  return [...text.matchAll(pattern)].length;
}

function decode(text: string): string {
  return text.replace(/&quot;/g, '"').replace(/&#x27;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

/** The engine's printed text on the lines, as the sheet renders it (label, value, also, condition, prose). */
function printedText(lines: readonly SheetLineDto[]): string {
  return lines.map((line) => [line.label, line.value, ...line.also, line.condition ?? '', line.prose].join('\n')).join('\n');
}

function signed(id: string): boolean {
  return /^sf\.(ability_modifier\.|skill\.|fortitude$|reflex$|will$|initiative$|base_attack_bonus$)/.test(id);
}

function expectedText(id: string, value: number): string {
  return signed(id) && value >= 0 ? `+${value}` : String(value);
}

/** `data-sf-row` id -> rendered text, in render order. */
function renderedRows(html: string): Array<[string, string]> {
  return [...html.matchAll(/<span[^>]*data-sf-row="([^"]+)"[^>]*>([^<]*)<\/span>/g)].map((m) => [m[1], decode(m[2])]);
}

let checkedRows = 0;
for (const seed of SEEDS) {
  const detail = fixture(seed);
  assert(detail.snapshot !== null, `${seed}: the fixture computed`);
  assert(isStarfinderCharacter(detail.summary), `${seed}: opens the Starfinder sheet`);

  // 1. No CMB / CMD / Touch / Flat-Footed of the sheet's own.
  const html = decode(render(detail));
  const enginePrinted = count(printedText(detail.sheetLines), PATHFINDER_ONLY);
  assertEqual(count(html, PATHFINDER_ONLY), enginePrinted, `${seed}: every CMB|CMD|Touch|Flat-Footed in the render is the engine's printed text`);
  const layoutOnly = decode(render({ ...detail, sheetLines: [] }));
  assertEqual(count(layoutOnly, PATHFINDER_ONLY), 0, `${seed}: CMB|CMD|Touch|Flat-Footed in the layout`);
  for (const tile of ['Energy Armor Class (EAC)', 'Kinetic Armor Class (KAC)', 'Stamina Points', 'Hit Points', 'Resolve Points']) {
    assert(layoutOnly.includes(tile), `${seed}: the ${tile} tile is laid out`);
  }

  // 2a. Every data-sf-row value is its row's value; every row appears exactly once.
  const rows = renderedRows(layoutOnly);
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  for (const [id, text] of rows) {
    const value = byId.get(id);
    assert(value !== undefined, `${seed}: rendered ${id} is not an engine row`);
    assertEqual(text, expectedText(id, value as number), `${seed}: ${id}`);
  }
  assertEqual(
    rows.map(([id]) => id).sort().join(','),
    detail.explanations.map((row) => row.id).sort().join(','),
    `${seed}: the rendered rows are the engine's rows, each once`,
  );
  checkedRows += rows.length;

  // 2b. No digit is written anywhere else (the character's name is the player's text; a
  // label read from a row id -- "Level 2" -- carries only digits of that row's id).
  let rest = layoutOnly.replace(/<span[^>]*data-sf-row="[^"]+"[^>]*>[^<]*<\/span>/g, '');
  rest = rest.replace(/<[a-z0-9]+[^>]*data-sf-name[^>]*>[^<]*<\/[a-z0-9]+>/g, '');
  rest = rest.replace(/<[a-z0-9]+[^>]*data-sf-label-of="([^"]+)"[^>]*>([^<]*)<\/[a-z0-9]+>/g, (_whole, ids: string, label: string) => {
    for (const digits of label.match(/\d+/g) ?? []) {
      assert(ids.split(' ').some((id) => id.split('.').includes(digits)), `${seed}: label ${label} has a digit outside its rows ${ids}`);
    }
    return '';
  });
  const text = rest.replace(/<[^>]*>/g, ' ');
  const stray = text.match(/[^\s]*\d[^\s]*/g);
  assert(stray === null, `${seed}: numbers on the sheet that are no engine row: ${JSON.stringify(stray)}`);

  // 2c. Move every row by 1000: every number on the sheet moves with it.
  const moved = { ...detail, sheetLines: [], explanations: detail.explanations.map((row) => ({ ...row, value: row.value + 1000 })) };
  for (const [id, text] of renderedRows(decode(render(moved)))) {
    assertEqual(text, expectedText(id, (byId.get(id) as number) + 1000), `${seed}: ${id} follows its row`);
  }

  // The model never lays out a row that is not there: a refused build shows its diagnostics, no totals.
  const refused = buildStarfinderSheet({
    ...detail,
    snapshot: null,
    explanations: [],
    diagnostics: [{ id: 'sf_chassis.refused', message: 'refused', claimBlocking: true }],
  });
  assertEqual(refused.sections.length, 0, `${seed}: a refused build has no totals`);
  assertEqual(refused.blocking.length, 1, `${seed}: a refused build shows its blocking diagnostic`);
}

assertEqual(isStarfinderCharacter({ gameSystem: 'pf1' }), false, 'a Pathfinder character opens the Pathfinder sheet');
assert(checkedRows > 0, 'rows were checked');
console.log(`starfinderSheet: ${SEEDS.length} seeds, ${checkedRows} engine rows rendered, each = its row`);
