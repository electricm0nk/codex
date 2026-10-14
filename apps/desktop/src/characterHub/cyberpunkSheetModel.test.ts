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
 * 5. (E4.2) The skills block: all 66 skills, grouped under the 9 categories of the skill
 *    catalog (`list_cyberpunk_catalog('skill')`, here read from the same corpus file), each
 *    row's Level and Base the `cpr.skill_level.` / `cpr.skill.` rows -- the Base of a REF or
 *    DEX skill being the engine's armour-penalised one -- and no skill row left in Other totals.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
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

/** The skill catalog records as `list_cyberpunk_catalog('skill')` serves them: the corpus file's rows. */
const SKILLS = JSON.parse(
  readFileSync(new URL('../../../../data/cyberpunk-red/corpus/skill.json', import.meta.url), 'utf8'),
) as CyberpunkRecord[];

function render(detail: LoadSavedCharacterResponse, skillCatalog: readonly CyberpunkRecord[] | null = SKILLS): string {
  return decode(
    renderToStaticMarkup(
      createElement(CyberpunkCharacterSheet, {
        detail,
        onClose: () => undefined,
        onOpen: () => undefined,
        ...(skillCatalog === null ? {} : { skillCatalog }),
      }),
    ),
  );
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
  const placed = new Set(buildCyberpunkSheet(detail, SKILLS).boxes.map((box) => box.id));
  const expectedBoxes = CYBERPUNK_SHEET_COLUMNS.flatMap((column) => boxesInColumn(CYBERPUNK_SHEET_LAYOUT, column).map((box) => box.id)).filter((box) =>
    placed.has(box),
  );
  assertEqual(renderedBoxes.join(','), expectedBoxes.join(','), `${id}: boxes follow cyberpunkSheetLayout.ts`);
  for (const box of ['stats', 'skills', 'vitals', 'woundTrack']) {
    assert(renderedBoxes.includes(box), `${id}: the ${box} box is laid out`);
  }

  // 3. The core tiles, each inside its box (never in "Other totals").
  const rows = renderedRows(html);
  const renderedIds = new Set(rows.map(([rowId]) => rowId));
  const other = buildCyberpunkSheet(detail, SKILLS).boxes.find((box) => box.id === 'other');
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
  const refused = buildCyberpunkSheet(
    { ...detail, explanations: [], diagnostics: [{ id: 'cpr.refused', message: 'refused', claimBlocking: true }] },
    SKILLS,
  );
  assertEqual(refused.boxes.length, 0, `${id}: a refused build has no totals`);
  assertEqual(refused.blocking.length, 1, `${id}: a refused build shows its blocking diagnostic`);
}

// 5. (E4.2) The skills block.
const CATEGORIES = SKILLS.reduce<string[]>((seen, record) => {
  const category = String(record.category);
  return seen.includes(category) ? seen : [...seen, category];
}, []);
assertEqual(SKILLS.length, 66, 'the skill catalog holds the 66 skills');
assertEqual(CATEGORIES.length, 9, 'the skill catalog holds 9 categories');
/** The engine's id fragment of a name (`slug`, `src/rules_core/cpr/skills.rs`), for the test's own lookup. */
const slugOf = (name: string) => name.toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_+|_+$/g, '');
const SKILL_BY_SLUG = new Map(SKILLS.map((record) => [slugOf(String(record.name)), record]));
let checkedSkills = 0;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  const sheet = buildCyberpunkSheet(detail, SKILLS);
  const skills = sheet.boxes.find((box) => box.id === 'skills');
  assert(skills !== undefined, `${id}: the skills box is on the sheet`);
  if (skills === undefined) {
    break;
  }
  assertEqual(skills.columns?.join(','), 'Level,Base', `${id}: the skills box shows Level and Base`);
  assertEqual(skills.rows.length, 66, `${id}: 66 skill rows`);
  const groups = skills.rows.reduce<string[]>((seen, row) => (seen.includes(String(row.group)) ? seen : [...seen, String(row.group)]), []);
  assertEqual(groups.join(','), CATEGORIES.join(','), `${id}: the skills are grouped under the 9 categories, in the catalog's order`);
  for (const row of skills.rows) {
    const [level, base] = row.cells;
    assert(base !== null && base.rowId.startsWith('cpr.skill.'), `${id}: ${row.label} has its Base row`);
    assert(level !== null && level.rowId === `cpr.skill_level.${base?.rowId.slice('cpr.skill.'.length)}`, `${id}: ${row.label} has its Level row`);
    if (base === null || level === null) {
      continue;
    }
    assertEqual(base.value, String(byId.get(base.rowId)), `${id}: ${base.rowId} base = its row`);
    assertEqual(level.value, String(byId.get(level.rowId)), `${id}: ${level.rowId} level = its row`);
    const record = SKILL_BY_SLUG.get(base.rowId.slice('cpr.skill.'.length).split('.')[0]);
    assert(record !== undefined, `${id}: ${base.rowId} is a catalog skill`);
    assertEqual(row.group, record?.category, `${id}: ${base.rowId} sits under its catalog category`);
    assert(row.label.startsWith(String(record?.name)), `${id}: ${base.rowId} is labelled ${String(record?.name)}`);
    assertEqual(row.tag, record?.stat, `${id}: ${base.rowId} names its linked STAT`);
    // The armour penalty on REF and DEX skills: the Base is the penalised STAT + level.
    const stat = String(record?.stat);
    if (stat === 'REF' || stat === 'DEX') {
      const current = byId.get(`cpr.armor.${stat.toLowerCase()}`);
      assertEqual(Number(base.value), Number(current) + Number(level.value), `${id}: ${base.rowId} = armour-penalised ${stat} + level`);
    }
    checkedSkills += 1;
  }
  const html = render(detail);
  const skillsHtml = html.slice(html.indexOf('data-cpr-box="skills"'));
  assertEqual(
    [...skillsHtml.slice(0, skillsHtml.indexOf('</table>')).matchAll(/data-cpr-skill-group="([^"]+)"/g)].map((m) => m[1]).join(','),
    CATEGORIES.join(','),
    `${id}: the rendered skills block heads its 9 groups`,
  );
  const other = sheet.boxes.find((box) => box.id === 'other');
  assert(other === undefined || other.rows.every((row) => !/^cpr\.skill(_level)?\./.test(row.label)), `${id}: no skill row is left in Other totals`);
}
assertEqual(checkedSkills, 132, 'both characters: 66 skills each checked');
// The kitted Solo wears Flak: its REF and DEX skills show the penalty (the check above bites).
const kittedRows = new Map(fixture('cpr-solo-kitted').explanations.map((row) => [row.id, row.value]));
assert(kittedRows.get('cpr.armor.ref') !== kittedRows.get('cpr.stat.ref'), 'the kitted fixture lowers REF by its armour');
assert(kittedRows.get('cpr.armor.dex') !== kittedRows.get('cpr.stat.dex'), 'the kitted fixture lowers DEX by its armour');

// A skill held with specialties is one row per specialty, under its skill's category.
{
  const base = fixture('cpr-solo-streetrat');
  const specialised = {
    ...base,
    explanations: base.explanations.flatMap((row) =>
      row.id === 'cpr.skill.language' || row.id === 'cpr.skill_level.language'
        ? ['streetslang', 'spanish'].map((spec) => ({ ...row, id: `${row.id}.${spec}` }))
        : [row],
    ),
  };
  const skills = buildCyberpunkSheet(specialised, SKILLS).boxes.find((box) => box.id === 'skills');
  const language = skills?.rows.filter((row) => row.cells[1]?.rowId.startsWith('cpr.skill.language.')) ?? [];
  assertEqual(language.map((row) => row.label).join(','), 'Language (Streetslang),Language (Spanish)', 'a specialty row per specialty');
  assert(language.every((row) => row.group === 'Education'), 'specialty rows sit under their skill category');
  assertEqual(skills?.rows.length, 67, 'two Language specialties make 67 skill rows');
}

// Before the catalog arrives (or if it cannot load) the skills still render, ungrouped, each row once.
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const html = render(detail, null);
  const skills = buildCyberpunkSheet(detail, null).boxes.find((box) => box.id === 'skills');
  assertEqual(skills?.rows.length, 66, `${id}: without the catalog the 66 skills still render`);
  assert(skills?.rows.every((row) => row.group === null) === true, `${id}: without the catalog the skills are ungrouped`);
  assertEqual(
    renderedRows(html).map(([rowId]) => rowId).sort().join(','),
    detail.explanations.map((row) => row.id).sort().join(','),
    `${id}: without the catalog every row is still on the sheet once`,
  );
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
console.log(`cyberpunkSheetModel: ${CHARACTERS.length} characters, ${checkedRows} engine rows rendered, each = its row; ${checkedSkills} skill rows in ${CATEGORIES.length} groups`);
