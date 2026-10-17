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
 * 6. (E4.3) The weapons, range DV and armour blocks, over a third character carrying one weapon
 *    of each of the 11 ranged types (and a melee weapon), head and body armour and a shield:
 *    each weapon's name, damage dice, ROF, Hands, Concealable, magazine and Skill Base are its
 *    engine rows or printed lines; each weapon shows the D-1 single-shot range DV row of
 *    `range_dv.json` that lists its type (and its Autofire row), read in the engine's range
 *    bands; armour shows SP and penalty per location, the shield and the Armor Penalty.
 *    Printed lines (dice, words) are `data-cpr-printed` spans whose text is the engine's line.
 * 7. (E4.4) The cyberware, Role Ability and deck blocks, over a fourth character, a Netrunner
 *    with four pieces of cyberware (Neural Link, Cyberarm) and a loaded Cyberdeck: each installed
 *    piece is its printed name with its Humanity Loss row (= the piece's `humanity_loss_preset`
 *    in `cyberware.json`), the pieces sum to the Humanity Loss row; each option-slot container
 *    shows its used / total rows; every `cpr.role.` row sits in the Role Ability box; the
 *    Netrunner shows deck slots used / available, NET Actions (= the p. 197 table for its
 *    Interface rank), the deck and each Program and Hardware by its printed name.
 * 8. (E4.5) The Reputation, Improvement Points, Lifepath and critical-injuries blocks: the
 *    Reputation Level and Facedown rows, the resolved Facedown roll ("9 + 1d10", the engine's
 *    printed dice line) and every recorded Reputation event; Improvement Points current / total;
 *    every saved Lifepath answer by its table and column; and a checklist of the 22 Critical
 *    Injuries of `critical_injury.json` (`list_cyberpunk_catalog('critical_injury')`) under their
 *    two tables, ticked exactly where the engine printed a recorded injury. Ticking builds the
 *    list `save_cyberpunk_criticals` saves (the round trip and the moved Base Death Save Penalty
 *    are `cpr_adapter::tests::ticking_a_critical_injury_saves_it_and_moves_the_base_death_save_penalty`).
 * 9. (E4.6) Every printed line of each fixture response appears on the sheet exactly once: a box
 *    line as its printed cell, a rule line (`cpr_sheet_print.rs`: Role Ability, cyberware effects,
 *    Humanity Loss (D-6), weapon specials, brain damage (D-2)) under "Rules and features" with its
 *    label, figures and prose as printed; no line prints "-2 EMP".
 * 10. (E4.8) Cash, Ammunition and Rent, over a fifth character carrying cash, two ammunition stacks
 *    and housing (the p121 sheet's Cash / Ammunition / Rent fields): each sits under its label in
 *    the `ledger` box -- every ammunition count under its ammunition's name, the engine's printed
 *    `cpr.ammo.<i>.name` line -- each cell its row; no fixture has an Other totals box; every
 *    Critical Injury checkbox carries a name of its own (its table and injury), so a ui-smoke row
 *    can tick it.
 */
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { RULES_AND_FEATURES_ID } from './CharacterSheet';
import { CyberpunkCharacterSheet } from './CyberpunkCharacterSheet';
import { CYBERPUNK_SHEET_COLUMNS, CYBERPUNK_SHEET_LAYOUT } from './cyberpunkSheetLayout';
import {
  buildCyberpunkSheet,
  isCyberpunkCharacter,
  recordedCyberpunkCriticals,
  toggleCyberpunkCritical,
} from './cyberpunkSheetModel';
import { boxesInColumn } from './sheetLayout';
import { assert, assertEqual } from '../testSupport/asserts';

const CHARACTERS = ['cpr-solo-streetrat', 'cpr-solo-kitted', 'cpr-solo-arsenal', 'cpr-netrunner-decked', 'cpr-solo-ledger'] as const;

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

/** The Critical Injury tables as `list_cyberpunk_catalog('critical_injury')` serves them. */
const CRITICALS = JSON.parse(
  readFileSync(new URL('../../../../data/cyberpunk-red/corpus/critical_injury.json', import.meta.url), 'utf8'),
) as CyberpunkRecord[];

function render(
  detail: LoadSavedCharacterResponse,
  skillCatalog: readonly CyberpunkRecord[] | null = SKILLS,
  criticalCatalog: readonly CyberpunkRecord[] | null = CRITICALS,
): string {
  return decode(
    renderToStaticMarkup(
      createElement(CyberpunkCharacterSheet, {
        detail,
        onClose: () => undefined,
        onOpen: () => undefined,
        ...(skillCatalog === null ? {} : { skillCatalog }),
        ...(criticalCatalog === null ? {} : { criticalCatalog }),
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

/**
 * The html with its "Rules and features" section's engine texts removed: every rule line's label,
 * value, `also` figures and prose, each written as the engine printed it. Anything left in that
 * section (a digit the engine did not print) is then caught by the stray-digit check.
 */
function stripRuleLines(html: string, detail: LoadSavedCharacterResponse, id: string): string {
  const start = html.indexOf(`id="${RULES_AND_FEATURES_ID}"`);
  if (start < 0) {
    return html;
  }
  const original = html.slice(start);
  const texts: string[] = [];
  for (const line of detail.sheetLines) {
    if (!original.includes(`data-rule-id="${line.id}"`)) {
      continue;
    }
    for (const text of [line.label, line.value, ...line.also, line.prose].filter((t) => t.length > 0)) {
      assert(original.includes(`>${text}<`), `${id}: rule line ${line.id} prints ${JSON.stringify(text)}`);
      texts.push(text);
    }
  }
  const section = texts.reduce((rest, text) => rest.split(`>${text}<`).join('><'), original);
  return html.slice(0, start) + section.replace(/data-rule-id="[^"]*"/g, '');
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
  const placed = new Set(buildCyberpunkSheet(detail, SKILLS, CRITICALS).boxes.map((box) => box.id));
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
  // read from a row id -- "cpr.weapon.0.rof" -- carries only digits of its own row ids). The
  // "Rules and features" section (E4.6) prints the engine's rule lines verbatim; its texts are
  // checked and removed first (section 9 proves each line is there).
  let rest = stripRuleLines(html, detail, id).replace(/<span[^>]*data-cpr-row="[^"]+"[^>]*>[^<]*<\/span>/g, '');
  // A printed line (dice stay literal; a weapon's name; "N/A") is the engine's own text.
  const lines = new Map(detail.sheetLines.map((line) => [line.id, line.value === '' ? line.prose : line.value]));
  rest = rest.replace(/<span[^>]*data-cpr-printed="([^"]+)"[^>]*>([^<]*)<\/span>/g, (_whole, lineId: string, text: string) => {
    assertEqual(text, lines.get(lineId), `${id}: printed ${lineId} is the engine's line`);
    return '';
  });
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
assertEqual(checkedSkills, 66 * CHARACTERS.length, 'every character: 66 skills each checked');
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

// 6. (E4.3) Weapons, range DV and armour.
/** The weapon table and the range DV table as the corpus holds them (`weapon.json`, `range_dv.json`). */
const WEAPONS = JSON.parse(readFileSync(new URL('../../../../data/cyberpunk-red/corpus/weapon.json', import.meta.url), 'utf8')) as CyberpunkRecord[];
const RANGE_DV = JSON.parse(readFileSync(new URL('../../../../data/cyberpunk-red/corpus/range_dv.json', import.meta.url), 'utf8')) as Array<{
  mode: string;
  weapon_types: string[];
  bands: Array<{ max_m_yds: number; dv: number | null }>;
}>;
const RANGED_TYPES = WEAPONS.filter((record) => record.weapon_class === 'ranged').map((record) => String(record.name));
assertEqual(RANGED_TYPES.length, 11, 'the weapon table holds 11 ranged types');
let checkedWeapons = 0;
let checkedDvCells = 0;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  const lines = new Map(detail.sheetLines.map((line) => [line.id, line.value === '' ? line.prose : line.value]));
  const sheet = buildCyberpunkSheet(detail, SKILLS);
  const other = sheet.boxes.find((box) => box.id === 'other');
  assert(
    other === undefined || other.rows.every((row) => !/^cpr\.(weapon|armor|range_band)\./.test(row.label)),
    `${id}: no weapon, armour or range row is left in Other totals: ${other?.rows.map((row) => row.label).join(',')}`,
  );
  const html = render(detail);
  const printed = new Set([...html.matchAll(/data-cpr-printed="([^"]+)"/g)].map((m) => m[1]));
  for (const line of detail.sheetLines) {
    if (line.id.startsWith('cpr.weapon.')) {
      assert(printed.has(line.id), `${id}: the printed weapon line ${line.id} is on the sheet`);
    }
  }

  // Armour: SP and penalty per location, the shield, and the Armor Penalty taken once.
  const armour = sheet.boxes.find((box) => box.id === 'armour');
  assert(armour !== undefined, `${id}: the armour box is on the sheet`);
  const armourCells = (label: string) => armour?.rows.find((row) => row.label === label)?.cells.map((cell) => cell?.rowId ?? null).join(',');
  assertEqual(armour?.columns?.join(','), 'SP,Penalty', `${id}: armour shows SP and Penalty`);
  assertEqual(armourCells('Head'), 'cpr.armor.sp_head,cpr.armor.penalty_head', `${id}: head armour`);
  assertEqual(armourCells('Body'), 'cpr.armor.sp_body,cpr.armor.penalty_body', `${id}: body armour`);
  assertEqual(armourCells('Armor Penalty (REF, DEX, MOVE)'), ',cpr.armor.penalty', `${id}: the Armor Penalty`);
  if (byId.has('cpr.armor.shield_hp')) {
    assertEqual(armourCells('Shield (HP)'), 'cpr.armor.shield_hp,cpr.armor.shield_penalty', `${id}: the shield`);
  }

  const weapons = sheet.boxes.find((box) => box.id === 'weapons');
  const indexes = [...new Set(detail.explanations.concat().map((row) => row.id).concat(detail.sheetLines.map((line) => line.id)).flatMap((rowId) => {
    const m = /^cpr\.weapon\.(\d+)\./.exec(rowId);
    return m === null ? [] : [Number(m[1])];
  }))].sort((a, b) => a - b);
  if (indexes.length === 0) {
    assert(weapons === undefined, `${id}: no weapon, no weapons box`);
    continue;
  }
  assert(weapons !== undefined, `${id}: the weapons box is on the sheet`);
  assertEqual(weapons?.columns?.join(','), 'DMG,ROF,Hands,Concealable,Magazine,Skill Base', `${id}: the weapon columns`);
  assertEqual(weapons?.rows.length, indexes.length, `${id}: one weapon row per weapon`);
  const ranges = sheet.boxes.find((box) => box.id === 'rangeDv');
  const bandRows = detail.explanations.filter((row) => row.id.startsWith('cpr.range_band.'));
  assertEqual(
    ranges?.columns?.map((column) => (typeof column === 'string' ? column : column.rowId)).join(','),
    bandRows.map((row) => row.id).join(','),
    `${id}: the range DV table is headed by the engine's range bands`,
  );
  for (const [n, index] of indexes.entries()) {
    const at = (field: string) => `cpr.weapon.${index}.${field}`;
    const row = weapons?.rows[n];
    const name = lines.get(at('name'));
    assert(name !== undefined, `${id}: weapon ${index} has its printed name`);
    assertEqual(row?.labelCell?.value, name, `${id}: weapon ${index} is labelled with its name`);
    const expected = ['damage', 'rof', 'hands', 'concealable', 'magazine', 'skill_base'].map((field) =>
      byId.has(at(field)) ? String(byId.get(at(field))) : (lines.get(at(field)) ?? null),
    );
    assertEqual(JSON.stringify(row?.cells.map((cell) => cell?.value ?? null)), JSON.stringify(expected), `${id}: ${name} row = its rows and lines`);
    for (const cell of row?.cells ?? []) {
      assert(cell === null || cell.rowId.startsWith(`cpr.weapon.${index}.`), `${id}: ${name}: ${cell?.rowId} is its own row`);
    }
    // Every ranged weapon carries its Skill Base, the twin of its weapon skill's row.
    const record = WEAPONS.find((weapon) => weapon.name === name);
    if (record?.weapon_class === 'ranged') {
      assertEqual(byId.get(at('skill_base')), byId.get(`cpr.skill.${slugOf(String(record.skill))}`), `${id}: ${name}: Skill Base = the ${String(record.skill)} row`);
      assert(byId.has(at('hands')), `${id}: ${name}: Hands is an engine row`);
    }

    // D-1: the single-shot row whose weapon_types names this type, band by band; and Autofire.
    for (const mode of ['single_shot', 'autofire'] as const) {
      const table = RANGE_DV.find((dv) => dv.mode === mode && dv.weapon_types.includes(String(name)));
      const label = mode === 'autofire' ? 'Autofire' : undefined;
      const dvRow = ranges?.rows.find((r) => r.labelCell?.rowId === at('name') && r.tag === label);
      if (table === undefined) {
        assert(record?.weapon_class !== 'ranged' || mode === 'autofire', `${id}: ${name}: a ranged type has a D-1 row`);
        assert(dvRow === undefined, `${id}: ${name}: no ${mode} DV row`);
        continue;
      }
      assert(dvRow !== undefined, `${id}: ${name} shows its ${mode} range DV row`);
      const field = mode === 'autofire' ? 'autofire_dv' : 'range_dv';
      for (const [b, band] of table.bands.entries()) {
        const dvCell = dvRow?.cells[b] ?? null;
        assertEqual(dvCell?.rowId, at(`${field}.${b}`), `${id}: ${name} ${mode} band ${b} cell`);
        assertEqual(dvCell?.value, band.dv === null ? 'N/A' : String(band.dv), `${id}: ${name} ${mode} band ${b} = range_dv.json`);
        assertEqual(dvCell?.printed === true, band.dv === null, `${id}: ${name} ${mode} band ${b}: N/A is printed, a DV is a row`);
        assertEqual(Number(byId.get(bandRows[b]?.id ?? '')), band.max_m_yds, `${id}: band ${b} reads to ${band.max_m_yds} m/yds`);
        checkedDvCells += 1;
      }
    }
    checkedWeapons += 1;
  }
}
const arsenalNames = new Set(fixture('cpr-solo-arsenal').sheetLines.filter((line) => line.id.endsWith('.name')).map((line) => line.value));
for (const type of RANGED_TYPES) {
  assert(arsenalNames.has(type), `the arsenal fixture carries a ${type}`);
}
assertEqual(checkedWeapons, 1 + 12, 'kitted Heavy Pistol + the arsenal\'s 11 ranged types and 1 melee weapon');

// 7. (E4.4) Cyberware, the Role Ability and the Netrunner's deck.
const CYBERWARE = JSON.parse(readFileSync(new URL('../../../../data/cyberpunk-red/corpus/cyberware.json', import.meta.url), 'utf8')) as CyberpunkRecord[];
/** NET Actions per Interface Rank 1-10, typed from the p. 197 table (pages/p198), not the corpus. */
const NET_ACTIONS_P197 = [2, 2, 2, 3, 3, 3, 4, 4, 4, 5];
let checkedPieces = 0;
let checkedContainers = 0;
let checkedRoleRows = 0;
let checkedDeckLines = 0;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  const lines = new Map(detail.sheetLines.map((line) => [line.id, line.value === '' ? line.prose : line.value]));
  const sheet = buildCyberpunkSheet(detail, SKILLS);
  const box = (boxId: string) => sheet.boxes.find((b) => b.id === boxId);
  const other = box('other');
  assert(
    other === undefined || other.rows.every((row) => !/^cpr\.(cyberware|role|netrunning)\./.test(row.label)),
    `${id}: no cyberware, Role Ability or deck row is left in Other totals: ${other?.rows.map((row) => row.label).join(',')}`,
  );
  const cellIds = (row: { cells: ({ rowId: string } | null)[] } | undefined) => row?.cells.map((cell) => cell?.rowId ?? null).join(',');

  // Cyberware: each piece by name with its Humanity Loss; the total; slots per container.
  const cyberware = box('cyberware');
  assert(cyberware !== undefined, `${id}: the cyberware box is on the sheet`);
  const pieces = detail.sheetLines.filter((line) => /^cpr\.cyberware\.piece\.\d+\.name$/.test(line.id));
  let pieceSum = 0;
  for (const piece of pieces) {
    const lossId = piece.id.replace(/\.name$/, '.humanity_loss');
    const row = cyberware?.rows.find((r) => r.labelCell?.rowId === piece.id);
    assertEqual(row?.labelCell?.value, piece.value, `${id}: cyberware ${piece.value} is labelled by its printed name`);
    assertEqual(cellIds(row), lossId, `${id}: ${piece.value} shows its Humanity Loss row`);
    const record = CYBERWARE.find((c) => c.name === piece.value);
    assert(record !== undefined, `${id}: ${piece.value} is a cyberware.json record`);
    assertEqual(row?.cells[0]?.value, String(record?.humanity_loss_preset), `${id}: ${piece.value} Humanity Loss = cyberware.json humanity_loss_preset`);
    pieceSum += Number(byId.get(lossId));
    checkedPieces += 1;
  }
  const total = cyberware?.rows.find((r) => r.label === 'Humanity Loss');
  assertEqual(cellIds(total), 'cpr.cyberware.humanity_loss', `${id}: the Humanity Loss total`);
  assertEqual(Number(total?.cells[0]?.value), pieceSum, `${id}: the pieces sum to the Humanity Loss row`);
  const containers = detail.explanations.flatMap((row) => {
    const m = /^cpr\.cyberware\.slots\.(.+)\.used$/.exec(row.id);
    return m === null ? [] : [m[1]];
  });
  assert(containers.length >= 3, `${id}: Fashionware, Internal and External Body at least`);
  for (const container of containers) {
    const used = `cpr.cyberware.slots.${container}.used`;
    const capacity = `cpr.cyberware.slots.${container}.capacity`;
    const row = cyberware?.rows.find((r) => r.cells[0]?.rowId === used);
    assertEqual(cellIds(row), `${used},${capacity}`, `${id}: ${container} slots used / total`);
    assertEqual(row?.cells.map((cell) => cell?.value).join('/'), `${String(byId.get(used))}/${String(byId.get(capacity))}`, `${id}: ${container} = its rows`);
    checkedContainers += 1;
  }
  assertEqual(cyberware?.columns ?? null, null, `${id}: the cyberware box reads each row as one figure ("used / total")`);

  // Role Ability: every cpr.role. row, the Rank first.
  const roleRows = detail.explanations.filter((row) => row.id.startsWith('cpr.role.')).map((row) => row.id);
  const role = box('roleAbility');
  if (roleRows.length === 0) {
    assert(role === undefined, `${id}: no Role Ability rows, no Role Ability box`);
  } else {
    const shown = role?.rows.flatMap((row) => row.cells.flatMap((cell) => (cell === null ? [] : [cell.rowId]))) ?? [];
    assertEqual([...shown].sort().join(','), [...roleRows].sort().join(','), `${id}: the Role Ability box holds every cpr.role. row`);
    assert(/^cpr\.role\.[a-z]+\.rank$/.test(shown[0] ?? ''), `${id}: the Role Ability's Rank comes first`);
    checkedRoleRows += shown.length;
  }

  // The Netrunner's deck.
  const deck = box('netrunning');
  const rank = byId.get('cpr.role.netrunner.rank');
  if (rank === undefined) {
    assert(deck === undefined, `${id}: not a Netrunner, no deck box`);
    continue;
  }
  assert(deck !== undefined, `${id}: the Netrunner's deck box is on the sheet`);
  assertEqual(cellIds(deck?.rows.find((r) => r.label === 'NET Actions')), 'cpr.netrunning.net_actions', `${id}: NET Actions`);
  assertEqual(byId.get('cpr.netrunning.net_actions'), NET_ACTIONS_P197[rank - 1], `${id}: NET Actions = the p. 197 table at Interface Rank ${rank}`);
  assertEqual(
    cellIds(deck?.rows.find((r) => r.label === 'Deck slots (used / available)')),
    'cpr.netrunning.deck_slots_used,cpr.netrunning.deck_slots_available',
    `${id}: deck slots used / available`,
  );
  const html = render(detail);
  const printed = new Set([...html.matchAll(/data-cpr-printed="([^"]+)"/g)].map((m) => m[1]));
  const deckLines = detail.sheetLines.filter((line) => /^cpr\.netrunning\.(deck|program\.\d+|hardware\.\d+)$/.test(line.id));
  assert(deckLines.some((line) => line.id.startsWith('cpr.netrunning.program.')), `${id}: the fixture loads Programs`);
  for (const line of deckLines) {
    assert(printed.has(line.id), `${id}: ${line.id} (${line.value}) is printed on the sheet`);
    const row = deck?.rows.find((r) => r.cells.some((cell) => cell?.rowId === line.id));
    assertEqual(row?.cells.find((cell) => cell?.rowId === line.id)?.value, lines.get(line.id), `${id}: ${line.id} is the engine's line`);
    checkedDeckLines += 1;
  }
}
// The Netrunner: 4 pieces + the kitted Solo's 1; 3 pools x 4 characters + Neural Link + Cyberarm;
// Rank + one Role value for the 3 characters with a Role; the deck, 6 Programs, 1 Hardware.
assertEqual(checkedPieces, 1 + 4, 'cyberware pieces checked');
assertEqual(checkedContainers, 3 * CHARACTERS.length + 2, 'option-slot containers checked');
assertEqual(checkedRoleRows, 2 * 3, 'Role Ability rows checked');
assertEqual(checkedDeckLines, 1 + 6 + 1, 'deck lines checked');
{
  const netrunner = new Map(fixture('cpr-netrunner-decked').explanations.map((row) => [row.id, row.value]));
  // p. 196: a Standard deck has 7 slots; the Cyberarm deck and the Bodyweight Suit add 1 each (A-7);
  // the p. 157 deck (Killer is Black ICE, 2 slots) plus 1 Hardware uses 8.
  assertEqual(netrunner.get('cpr.netrunning.deck_slots_available'), 7 + 1 + 1, 'the Netrunner fixture has 9 deck slots');
  assertEqual(netrunner.get('cpr.netrunning.deck_slots_used'), 7 + 1, 'the Netrunner fixture uses 8');
}

// 8. (E4.5) Reputation, Improvement Points, Lifepath and the critical-injuries checklist.
const TABLES: Readonly<Record<string, string>> = { body: 'Critical Injuries to the Body', head: 'Critical Injuries to the Head' };
assertEqual(CRITICALS.length, 22, 'the Critical Injury tables hold 22 injuries');
let checkedEvents = 0;
let checkedLifepath = 0;
let checkedTicks = 0;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const byId = new Map(detail.explanations.map((row) => [row.id, row.value]));
  const lines = new Map(detail.sheetLines.map((line) => [line.id, line]));
  const sheet = buildCyberpunkSheet(detail, SKILLS, CRITICALS);
  const box = (boxId: string) => sheet.boxes.find((b) => b.id === boxId);
  const cellIds = (row: { cells: ({ rowId: string } | null)[] } | undefined) => row?.cells.map((cell) => cell?.rowId ?? null).join(',');
  const other = box('other');
  assert(other === undefined, `${id}: every engine row is laid out in a box, Other totals is empty: ${other?.rows.map((row) => row.label).join(',')}`);

  // Reputation: Level, Facedown base, the resolved roll (dice literal) and every event, in order.
  const reputation = box('reputation');
  assert(reputation !== undefined, `${id}: the Reputation box is on the sheet`);
  assertEqual(cellIds(reputation?.rows.find((r) => r.label === 'Reputation Level')), 'cpr.reputation.level', `${id}: Reputation Level`);
  assertEqual(cellIds(reputation?.rows.find((r) => r.label === 'Facedown (COOL + Reputation)')), 'cpr.reputation.facedown', `${id}: Facedown`);
  const roll = reputation?.rows.find((r) => r.label === 'Facedown roll')?.cells[0];
  assertEqual(roll?.printed, true, `${id}: the Facedown roll is a printed line`);
  assertEqual(roll?.value, lines.get('cpr.reputation.facedown_roll')?.value, `${id}: the Facedown roll is the engine's line`);
  assertEqual(roll?.value, `${String(byId.get('cpr.reputation.facedown'))} + 1d10`, `${id}: the roll resolves the Facedown row and keeps the die literal`);
  const events = detail.sheetLines.filter((line) => /^cpr\.reputation\.event\.\d+$/.test(line.id));
  const shownEvents = reputation?.rows.filter((r) => r.cells[0]?.rowId.startsWith('cpr.reputation.event.')) ?? [];
  assertEqual(shownEvents.map((r) => r.cells[0]?.value).join('|'), events.map((line) => line.value).join('|'), `${id}: every Reputation event, in order`);
  checkedEvents += shownEvents.length;

  // Improvement Points: current / total.
  assertEqual(cellIds(box('improvementPoints')?.rows[0]), 'cpr.ip.current,cpr.ip.total', `${id}: Improvement Points current / total`);

  // Lifepath: every saved answer, labelled by its table and column.
  const answers = detail.sheetLines.filter((line) => /^cpr\.lifepath\.\d+$/.test(line.id));
  const lifepath = box('lifepath');
  assertEqual(lifepath?.rows.length ?? 0, answers.length, `${id}: one Lifepath row per saved answer`);
  for (const [n, line] of answers.entries()) {
    const row = lifepath?.rows[n];
    assertEqual(row?.label, line.label, `${id}: ${line.id} is labelled by its table and column`);
    assertEqual(row?.cells[0]?.value, line.value, `${id}: ${line.id} is the saved answer`);
    assertEqual(row?.cells[0]?.printed, true, `${id}: ${line.id} is printed`);
    checkedLifepath += 1;
  }

  // The checklist: all 22 injuries under their tables, ticked exactly where one is recorded.
  const checklist = box('criticalInjuries');
  assertEqual(checklist?.rows.length, 22, `${id}: the checklist lists the 22 Critical Injuries`);
  assertEqual(
    [...new Set(checklist?.rows.map((r) => r.group))].join(','),
    [TABLES.body, TABLES.head].join(','),
    `${id}: the checklist is grouped by the two tables`,
  );
  const recorded = recordedCyberpunkCriticals(detail);
  const ticked = checklist?.rows.filter((r) => r.cells[0] !== null) ?? [];
  assertEqual(
    ticked.map((r) => `${String(r.group)}:${r.label}`).sort().join('|'),
    recorded.map((c) => `${TABLES[c.location]}:${c.name}`).sort().join('|'),
    `${id}: ticked rows are the recorded injuries`,
  );
  for (const critical of recorded) {
    const row = checklist?.rows.find((r) => r.group === TABLES[critical.location] && r.label === critical.name);
    assert(row?.cells[0]?.printed === true, `${id}: ${critical.name} (${critical.location}) is ticked with the engine's printed line`);
    checkedTicks += 1;
  }
  assertEqual(ticked.length, recorded.length, `${id}: as many ticks as recorded injuries`);
  const html = render(detail);
  const boxes = [...html.matchAll(/<input[^>]*type="checkbox"[^>]*>/g)].map((m) => m[0]);
  assertEqual(boxes.length, 22, `${id}: 22 checkboxes rendered`);
  assertEqual(boxes.filter((tag) => /\bchecked=""/.test(tag)).length, recorded.length, `${id}: the rendered ticks are the recorded injuries`);
  // Without the catalog the checklist still shows the recorded injuries, ticked.
  const bare = buildCyberpunkSheet(detail, SKILLS, null).boxes.find((b) => b.id === 'criticalInjuries');
  assertEqual(bare?.rows.length ?? 0, recorded.length, `${id}: without the catalog only the recorded injuries are listed`);
}
// The kitted Solo: 2 events, 5 Lifepath answers, 2 recorded injuries (Collapsed Lung, Broken Ribs).
assertEqual(checkedEvents, 2, 'Reputation events checked');
assertEqual(checkedLifepath, 5, 'Lifepath answers checked');
assertEqual(checkedTicks, 2, 'ticked injuries checked');
{
  const kittedDetail = fixture('cpr-solo-kitted');
  const kittedValues = new Map(kittedDetail.explanations.map((row) => [row.id, row.value]));
  // p. 193: the later Level 2 event does not replace the Level 3 one; COOL 4 + 3 = 7 (p. 194).
  assertEqual(kittedValues.get('cpr.reputation.level'), 3, 'the kitted Solo keeps Reputation 3');
  assertEqual(kittedValues.get('cpr.reputation.facedown'), 4 + 3, 'Facedown = COOL 4 + Reputation 3');
  assertEqual(`${String(kittedValues.get('cpr.ip.current'))}/${String(kittedValues.get('cpr.ip.total'))}`, '20/40', 'IP 20 of 40');
  const recorded = recordedCyberpunkCriticals(kittedDetail);
  assertEqual(JSON.stringify(recorded), JSON.stringify([
    { location: 'body', name: 'Collapsed Lung' },
    { location: 'body', name: 'Broken Ribs' },
  ]), 'the kitted Solo records two injuries in order');
  // Ticking adds at the end; unticking removes only that injury (Foreign Object is on both tables).
  const added = toggleCyberpunkCritical(recorded, { location: 'head', name: 'Foreign Object' });
  assertEqual(JSON.stringify(added.map((c) => `${c.location}:${c.name}`)), JSON.stringify(['body:Collapsed Lung', 'body:Broken Ribs', 'head:Foreign Object']), 'tick appends');
  const removed = toggleCyberpunkCritical(added, { location: 'body', name: 'Collapsed Lung' });
  assertEqual(JSON.stringify(removed.map((c) => `${c.location}:${c.name}`)), JSON.stringify(['body:Broken Ribs', 'head:Foreign Object']), 'untick removes that one');
  assertEqual(toggleCyberpunkCritical(removed, { location: 'body', name: 'Foreign Object' }).length, 3, 'the Body Foreign Object is its own tick');
}


// 9. (E4.6) Every printed line of each fixture response appears on the sheet: a box line as its
// `data-cpr-printed` cell, every other line (the rule lines `cpr_sheet_print.rs` adds: Role
// Ability, cyberware effects, the Humanity Loss line, weapon specials, the brain-damage rule) under
// "Rules and features" (`RulesAndFeaturesSection`, the Starfinder precedent) with its label, its
// resolved figures and its prose as the engine printed them. No line is dropped or shown twice.
let checkedPrinted = 0;
let checkedRuleLines = 0;
const ruleKinds = new Set<string>();
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const html = render(detail);
  const sheet = buildCyberpunkSheet(detail, SKILLS, CRITICALS);
  const inBoxes = new Set([...html.matchAll(/data-cpr-printed="([^"]+)"/g)].map((m) => m[1]));
  const rulesStart = html.indexOf(`id="${RULES_AND_FEATURES_ID}"`);
  const rulesHtml = rulesStart < 0 ? '' : html.slice(rulesStart);
  const inRules = [...rulesHtml.matchAll(/data-rule-id="([^"]+)"/g)].map((m) => m[1]);
  assertEqual(new Set(inRules).size, inRules.length, `${id}: each rule line once`);
  assertEqual(
    sheet.rules.map((line) => line.id).join(','),
    inRules.join(','),
    `${id}: the rules section renders the model's rule lines, in the engine's order`,
  );
  for (const line of detail.sheetLines) {
    const boxed = inBoxes.has(line.id);
    const ruled = inRules.includes(line.id);
    assert(boxed !== ruled, `${id}: printed line ${line.id} appears on the sheet exactly once (box ${String(boxed)}, rules ${String(ruled)})`);
    if (ruled) {
      ruleKinds.add(line.kind);
      checkedRuleLines += 1;
    }
    checkedPrinted += 1;
  }
  // Every rule line the engine adds (any kind but the box lines') is under Rules and features.
  for (const line of detail.sheetLines.filter((l) => l.kind !== 'cpr')) {
    assert(inRules.includes(line.id), `${id}: rule line ${line.id} (${line.kind}) is under Rules and features`);
  }
}
// The kitted Solo, the arsenal and the Netrunner carry all five rule blocks between them.
assertEqual([...ruleKinds].sort().join(','), 'cyberware,humanity,netrunning,role_ability,weapon_special', 'the five rule blocks render');
{
  const netrunner = fixture('cpr-netrunner-decked');
  const html = render(netrunner);
  const brain = netrunner.sheetLines.find((line) => line.id === 'cpr.netrunning.brain_damage');
  assert(brain !== undefined && html.includes(`>${brain.prose}<`), 'D-2: the brain-damage rule is printed on the Netrunner sheet');
  const hp = netrunner.explanations.find((row) => row.id === 'cpr.derived.hit_points')?.value;
  assertEqual(brain?.also[0], `Hit Points ${String(hp)}`, 'D-2: the brain-damage line carries the Hit Points row');
  for (const line of CHARACTERS.flatMap((c) => fixture(c).sheetLines)) {
    assert(![line.label, line.value, line.prose, ...line.also].some((t) => t.includes('-2 EMP')), `D-6: ${line.id} never prints "-2 EMP"`);
  }
}

// 10. (E4.8) Cash, each Ammunition stack by its name, and Rent sit under labels in the ledger box;
// no character's sheet has an Other totals box; each Critical Injury checkbox is named.
let checkedLedger = 0;
const LEDGER_ROW = /^cpr\.(cash|ammo\.\d+|housing\.rent)$/;
for (const id of CHARACTERS) {
  const detail = fixture(id);
  const sheet = buildCyberpunkSheet(detail, SKILLS, CRITICALS);
  const other = sheet.boxes.find((b) => b.id === 'other');
  assert(other === undefined, `${id}: no Other totals box (rows ${other?.rows.map((row) => row.label).join(',') ?? ''})`);
  const ledgerRows = detail.explanations.filter((row) => LEDGER_ROW.test(row.id));
  const ledger = sheet.boxes.find((b) => b.id === 'ledger');
  assertEqual(ledger === undefined, ledgerRows.length === 0, `${id}: the ledger box is there exactly when the engine sent a cash, ammunition or rent row`);
  const shown = (ledger?.rows ?? []).flatMap((row) => row.cells.flatMap((cell) => (cell === null ? [] : [cell.rowId])));
  assertEqual(shown.sort().join(','), ledgerRows.map((row) => row.id).sort().join(','), `${id}: the ledger box holds every cash, ammunition and rent row`);
  for (const row of ledger?.rows ?? []) {
    const cell = row.cells[0];
    if (cell === null) {
      throw new Error(`${id}: ${row.label} has no cell`);
    }
    assertEqual(cell.value, String(detail.explanations.find((r) => r.id === cell.rowId)?.value), `${id}: ${cell.rowId} is its row`);
    checkedLedger += 1;
  }
  // Every checkbox of the checklist is named by its table and injury, each name once.
  const names = [...render(detail).matchAll(/<input[^>]*type="checkbox"[^>]*>/g)].map((m) => /aria-label="([^"]+)"/.exec(m[0])?.[1] ?? '');
  assertEqual(names.length, 22, `${id}: 22 checkboxes`);
  assertEqual(new Set(names).size, 22, `${id}: each checkbox has a name of its own: ${names.join('|')}`);
  assert(names.includes('Critical Injuries to the Body: Collapsed Lung') && names.includes('Critical Injuries to the Head: Foreign Object'), `${id}: a checkbox is named by its table and injury`);
}
assertEqual(checkedLedger, 4, 'the ledger Solo: cash, two ammunition stacks and rent');
{
  const ledger = fixture('cpr-solo-ledger');
  const sheet = buildCyberpunkSheet(ledger, SKILLS, CRITICALS);
  const box = sheet.boxes.find((b) => b.id === 'ledger');
  // Label (the ammunition's printed name for a stack) and the row each value is.
  const laidOut = (box?.rows ?? []).map((row) => `${row.group ?? ''}|${row.labelCell?.value ?? row.label}|${row.cells.map((cell) => cell?.rowId ?? '').join(',')}`);
  assertEqual(
    laidOut.join(' ; '),
    ['|Cash|cpr.cash', 'Ammunition|Basic VH Pistol Ammunition|cpr.ammo.0', 'Ammunition|Basic Rifle Ammunition|cpr.ammo.1', '|Rent|cpr.housing.rent'].join(' ; '),
    'the ledger box: Cash, each Ammunition stack by name, Rent',
  );
  for (const n of [0, 1]) {
    const row = box?.rows.find((r) => r.cells[0]?.rowId === `cpr.ammo.${n}`);
    assertEqual(row?.labelCell?.printed, true, `cpr.ammo.${n} is labelled by the engine's printed name line`);
    assertEqual(row?.labelCell?.rowId, `cpr.ammo.${n}.name`, `cpr.ammo.${n} is labelled by cpr.ammo.${n}.name`);
  }
  const html = render(ledger);
  for (const text of ['>Cash<', '>Ammunition<', '>Rent<', '>Basic VH Pistol Ammunition<', '>Basic Rifle Ammunition<', '>1450<', '>30<', '>70<', '>1000<']) {
    assert(html.includes(text), `the ledger sheet renders ${text}`);
  }
  assert(!/>cpr\.(cash|ammo|housing)/.test(html), 'no ledger row is shown under its raw id');
  assert(sheet.rules.every((line) => !line.id.startsWith('cpr.ammo.')), 'the ammunition names are box labels, not rule lines');
  // A housing whose rent the book prints as words (no eb figure) is the engine's printed line,
  // under the same Rent label.
  const words = buildCyberpunkSheet(
    {
      ...ledger,
      explanations: ledger.explanations.filter((row) => row.id !== 'cpr.housing.rent'),
      sheetLines: [...ledger.sheetLines, { ...ledger.sheetLines[0], id: 'cpr.housing.rent', kind: 'cpr', label: 'Rent', form: 'words' as const, value: '', also: [], prose: 'Living on The Street rent: free (p379)' }],
    },
    SKILLS,
    CRITICALS,
  );
  const rent = words.boxes.find((b) => b.id === 'ledger')?.rows.find((row) => row.label === 'Rent');
  assertEqual(rent?.cells[0]?.printed, true, 'a worded rent is printed under Rent');
  assert(words.rules.every((line) => line.id !== 'cpr.housing.rent'), 'a worded rent is not left under Rules and features');
}

for (const gameSystem of ['pf1', 'starfinder-1e']) {
  assertEqual(isCyberpunkCharacter({ gameSystem }), false, `a ${gameSystem} character does not open the Cyberpunk RED sheet`);
}
assert(checkedRows > 0, 'rows were checked');
console.log(
  `cyberpunkSheetModel: ${CHARACTERS.length} characters, ${checkedRows} engine rows rendered, each = its row; ${checkedSkills} skill rows in ${CATEGORIES.length} groups; ${checkedWeapons} weapons, ${checkedDvCells} range DV cells; ${checkedPieces} cyberware pieces, ${checkedContainers} slot containers, ${checkedRoleRows} Role Ability rows, ${checkedDeckLines} deck lines; ${checkedEvents} Reputation events, ${checkedLifepath} Lifepath answers, ${checkedTicks} ticked injuries of 22; ${checkedPrinted} printed lines on the sheet, ${checkedRuleLines} under Rules and features; ${checkedLedger} cash, ammunition and rent rows`,
);
