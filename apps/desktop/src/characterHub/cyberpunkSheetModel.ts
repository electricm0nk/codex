import type { CyberpunkRecord } from '../boundary/loadCyberpunkCatalog';
import type { ExplanationDto, LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import type { CharacterSummaryDto } from '../boundary/loadListSavedCharacters';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import type { CyberpunkSheetBoxId } from './cyberpunkSheetLayout';

/**
 * The Cyberpunk RED sheet's content (`CyberpunkCharacterSheet.tsx`).
 *
 * Every number this model hands the sheet is one engine explanation row's `value` and nothing
 * else: `CyberpunkAdapter::load_saved_character` (`cpr_adapter.rs`) sends the composed sheet's
 * `cpr.` rows (`compose_sheet`, `src/rules_core/cpr/sheet.rs`). This file holds labels and the
 * order they appear in -- no rule numbers. A row the boxes below do not name still appears, in
 * the `other` box under its row id, so an engine row is never dropped.
 */

/** The `game_system` a Cyberpunk RED save envelope carries (`GameSystem::CyberpunkRed`). */
export const CYBERPUNK_GAME_SYSTEM = 'cyberpunk-red';

/** Whether a saved character is a Cyberpunk RED character (it opens the Cyberpunk RED sheet). */
export function isCyberpunkCharacter(summary: Pick<CharacterSummaryDto, 'gameSystem'>): boolean {
  return summary.gameSystem === CYBERPUNK_GAME_SYSTEM;
}

/** One figure on the sheet: an engine row's value, or (`printed`) an engine printed line's
 * text -- dice as written, a weapon's name, "N/A". */
export interface CyberpunkSheetCell {
  rowId: string;
  value: string;
  /** The engine's own derivation text for the row (a printed line's prose), verbatim. */
  detail: string;
  /** A printed line (`sheetLines`), not a computed number. */
  printed?: boolean;
}

export interface CyberpunkSheetRow {
  label: string;
  /** One cell per column; `null` where the engine sent no row for it. */
  cells: (CyberpunkSheetCell | null)[];
  /** The heading the row sits under inside its box (a skill's category); `null` when the
   * catalog that names it has not arrived. */
  group?: string | null;
  /** A short text tag printed after the label (a skill's linked STAT; "Autofire"). */
  tag?: string;
  /** The label as an engine printed line (a weapon's name), shown in place of `label`. */
  labelCell?: CyberpunkSheetCell;
}

export interface CyberpunkSheetBox {
  id: CyberpunkSheetBoxId;
  title: string;
  /** Column headings when each row has one value per column -- words, or an engine row when the
   * heading is a number (a range band's m/yds); `null` when a row's values read as one figure
   * ("current / max"). */
  columns: (string | CyberpunkSheetCell)[] | null;
  rows: CyberpunkSheetRow[];
}

export interface CyberpunkSheet {
  name: string;
  /** The boxes that hold at least one engine row, keyed by layout id. */
  boxes: CyberpunkSheetBox[];
  /** Diagnostics that stop the build from computing. */
  blocking: DiagnosticDto[];
  /** Other diagnostics worth showing. */
  notes: DiagnosticDto[];
}

/**
 * The ten STATs in book order (`CprStat::ALL`): the entered value's row, and the row of the
 * current value where the engine adjusts it (armour on REF, DEX and MOVE; the cyberware BODY;
 * EMP after Humanity Loss).
 */
const STATS: readonly { id: string; label: string; current: string | null }[] = [
  { id: 'int', label: 'INT', current: null },
  { id: 'ref', label: 'REF', current: 'cpr.armor.ref' },
  { id: 'dex', label: 'DEX', current: 'cpr.armor.dex' },
  { id: 'tech', label: 'TECH', current: null },
  { id: 'cool', label: 'COOL', current: null },
  { id: 'will', label: 'WILL', current: null },
  { id: 'luck', label: 'LUCK', current: null },
  { id: 'move', label: 'MOVE', current: 'cpr.armor.move' },
  { id: 'body', label: 'BODY', current: 'cpr.cyberware.body' },
  { id: 'emp', label: 'EMP', current: 'cpr.derived.emp_current' },
];

/** The vitals box: label -> the row ids read as one figure (Humanity is "current / max"). */
const VITALS: readonly { label: string; ids: readonly string[] }[] = [
  { label: 'Hit Points', ids: ['cpr.derived.hit_points'] },
  { label: 'Seriously Wounded', ids: ['cpr.derived.seriously_wounded'] },
  { label: 'Death Save', ids: ['cpr.derived.death_save'] },
  { label: 'Humanity', ids: ['cpr.derived.humanity_current', 'cpr.derived.humanity_max'] },
  { label: 'MOVE', ids: ['cpr.derived.move'] },
  { label: 'LUCK', ids: ['cpr.derived.luck_pool'] },
];

const WOUND_PREFIX = 'cpr.wound.';
/** The wound-track rows that are not a Wound State threshold. */
const WOUND_SINGLES: readonly { id: string; label: string }[] = [
  { id: 'cpr.wound.death_save', label: 'Death Save' },
  { id: 'cpr.wound.base_death_save_penalty', label: 'Base Death Save Penalty' },
];

/** The armour box: label -> its SP (or shield HP) row and its penalty row. */
const ARMOUR: readonly { label: string; ids: readonly [string | null, string] }[] = [
  { label: 'Head', ids: ['cpr.armor.sp_head', 'cpr.armor.penalty_head'] },
  { label: 'Body', ids: ['cpr.armor.sp_body', 'cpr.armor.penalty_body'] },
  { label: 'Shield (HP)', ids: ['cpr.armor.shield_hp', 'cpr.armor.shield_penalty'] },
  { label: 'Armor Penalty (REF, DEX, MOVE)', ids: [null, 'cpr.armor.penalty'] },
];

/** The weapon box's columns: heading -> the field of `cpr.weapon.<n>.<field>` (row or line). */
const WEAPON_FIELDS: readonly { heading: string; field: string }[] = [
  { heading: 'DMG', field: 'damage' },
  { heading: 'ROF', field: 'rof' },
  { heading: 'Hands', field: 'hands' },
  { heading: 'Concealable', field: 'concealable' },
  { heading: 'Magazine', field: 'magazine' },
  { heading: 'Skill Base', field: 'skill_base' },
];
const WEAPON_ID = /^cpr\.weapon\.(\d+)\./;
const RANGE_BAND_PREFIX = 'cpr.range_band.';

const SKILL_PREFIX = 'cpr.skill.';
const SKILL_LEVEL_PREFIX = 'cpr.skill_level.';

/** The engine's id fragment of a skill name (`slug`, `src/rules_core/cpr/skills.rs`): lower
 * case, every run of other characters one `_`. `Conceal/Reveal Object` -> `conceal_reveal_object`. */
export function cyberpunkSkillSlug(name: string): string {
  return name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '_')
    .replace(/^_+|_+$/g, '');
}

/** `seriously_wounded` -> `Seriously Wounded`. */
function humanise(slug: string): string {
  return slug
    .split('_')
    .filter((word) => word.length > 0)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

/**
 * Lays out one loaded Cyberpunk RED character. Every cell is one explanation row.
 *
 * `skillCatalog` is the skill catalog (`list_cyberpunk_catalog('skill')`): it names each skill,
 * its category and its linked STAT. Until it arrives (`null`) the skills still render, ungrouped
 * and labelled from their row ids.
 */
export function buildCyberpunkSheet(
  detail: LoadSavedCharacterResponse,
  skillCatalog: readonly CyberpunkRecord[] | null,
): CyberpunkSheet {
  const rows = new Map<string, ExplanationDto>(detail.explanations.map((row) => [row.id, row]));
  const placed = new Set<string>();
  const cell = (id: string | null): CyberpunkSheetCell | null => {
    if (id === null) {
      return null;
    }
    const row = rows.get(id);
    if (row === undefined || placed.has(id)) {
      return null;
    }
    placed.add(id);
    return { rowId: id, value: String(row.value), detail: row.detail };
  };
  // A build that did not compute lays out nothing, its printed lines included.
  const computed = !detail.diagnostics.some((d) => d.claimBlocking);
  const lines = new Map((computed ? detail.sheetLines : []).map((line) => [line.id, line]));
  /** An engine row when the engine computed one, else its printed line (dice, words, "N/A"). */
  const figure = (id: string): CyberpunkSheetCell | null => {
    const computed = cell(id);
    if (computed !== null) {
      return computed;
    }
    const line = lines.get(id);
    return line === undefined ? null : { rowId: id, value: line.value === '' ? line.prose : line.value, detail: line.prose, printed: true };
  };
  const boxes: CyberpunkSheetBox[] = [];
  const push = (box: CyberpunkSheetBox) => {
    const kept = box.rows.filter((row) => row.cells.some((c) => c !== null));
    if (kept.length > 0) {
      boxes.push({ ...box, rows: kept });
    }
  };

  push({
    id: 'stats',
    title: 'STATs',
    columns: ['STAT', 'Current'],
    rows: STATS.map((stat) => ({ label: stat.label, cells: [cell(`cpr.stat.${stat.id}`), cell(stat.current)] })),
  });
  push({
    id: 'skills',
    title: 'Skills',
    columns: ['Level', 'Base'],
    rows: skillRows(detail, skillCatalog, cell),
  });
  push({
    id: 'vitals',
    title: 'Hit Points, Humanity, MOVE and LUCK',
    columns: null,
    rows: VITALS.map((vital) => ({ label: vital.label, cells: vital.ids.map((id) => cell(id)) })),
  });

  // `cpr.wound.<state>` and `cpr.wound.<state>_dv`, in the engine's order (the Wound States as
  // the corpus lists them), then the Death Save and the Base Death Save Penalty.
  const singles = new Set(WOUND_SINGLES.map((single) => single.id));
  const states = detail.explanations
    .map((row) => row.id)
    .filter((id) => id.startsWith(WOUND_PREFIX) && !id.endsWith('_dv') && !singles.has(id));
  push({
    id: 'woundTrack',
    title: 'Wound track',
    columns: ['Below HP', 'Stabilize DV'],
    rows: [
      ...states.map((id) => ({ label: humanise(id.slice(WOUND_PREFIX.length)), cells: [cell(id), cell(`${id}_dv`)] })),
      ...WOUND_SINGLES.map((single) => ({ label: single.label, cells: [cell(single.id), null] })),
    ],
  });

  push({
    id: 'armour',
    title: 'Armor',
    columns: ['SP', 'Penalty'],
    rows: ARMOUR.map((piece) => ({ label: piece.label, cells: piece.ids.map((id) => cell(id)) })),
  });

  // The weapons, in the engine's order (`cpr.weapon.<n>.`), each labelled by its printed name.
  const weaponIndexes = [
    ...new Set([...detail.explanations.map((row) => row.id), ...lines.keys()].flatMap((id) => {
      const match = WEAPON_ID.exec(id);
      return match === null ? [] : [Number(match[1])];
    })),
  ].sort((a, b) => a - b);
  const weaponName = (n: number) => figure(`cpr.weapon.${n}.name`) ?? undefined;
  push({
    id: 'weapons',
    title: 'Weapons',
    columns: WEAPON_FIELDS.map((column) => column.heading),
    rows: weaponIndexes.map((n) => ({
      label: `cpr.weapon.${n}`,
      labelCell: weaponName(n),
      cells: WEAPON_FIELDS.map((column) => figure(`cpr.weapon.${n}.${column.field}`)),
    })),
  });

  // Range DV: the engine's range bands head the columns; each weapon's single-shot row (D-1),
  // then its Autofire row; a band the book prints N/A is the engine's printed "N/A".
  const bandIds = detail.explanations.map((row) => row.id).filter((id) => id.startsWith(RANGE_BAND_PREFIX));
  const dvRows = (n: number, field: string, tag?: string): CyberpunkSheetRow[] => {
    const cells = bandIds.map((_band, b) => figure(`cpr.weapon.${n}.${field}.${b}`));
    return cells.every((c) => c === null) ? [] : [{ label: `cpr.weapon.${n}.${field}`, labelCell: weaponName(n), ...(tag === undefined ? {} : { tag }), cells }];
  };
  const rangeRows = weaponIndexes.flatMap((n) => [...dvRows(n, 'range_dv'), ...dvRows(n, 'autofire_dv', 'Autofire')]);
  if (rangeRows.length > 0) {
    push({
      id: 'rangeDv',
      title: 'Range DV (m/yds up to)',
      columns: bandIds.map((id) => cell(id) ?? id),
      rows: rangeRows,
    });
  }

  // Any `cpr.` row the boxes above do not name: shown under its id, never dropped.
  push({
    id: 'other',
    title: 'Other totals',
    columns: null,
    rows: detail.explanations
      .filter((row) => !placed.has(row.id))
      .map((row) => ({ label: row.id, cells: [cell(row.id)] })),
  });

  return {
    name: detail.summary.displayLabel,
    boxes,
    blocking: detail.diagnostics.filter((d) => d.claimBlocking),
    notes: detail.diagnostics.filter((d) => !d.claimBlocking),
  };
}

/**
 * The skills box: one row per `cpr.skill.<skill>` (or `cpr.skill.<skill>.<specialty>`) row, in
 * the engine's order, with its `cpr.skill_level.` twin. With the catalog, rows sit under their
 * skill's category (the categories in catalog order) and name their linked STAT; a row whose
 * skill the catalog does not hold is left for Other totals. The Base is the engine's: a REF or
 * DEX skill's already carries the armour penalty (`compose_sheet`, step 5).
 */
function skillRows(
  detail: LoadSavedCharacterResponse,
  skillCatalog: readonly CyberpunkRecord[] | null,
  cell: (id: string | null) => CyberpunkSheetCell | null,
): CyberpunkSheetRow[] {
  const bases = detail.explanations.map((row) => row.id).filter((id) => id.startsWith(SKILL_PREFIX));
  const split = (id: string) => {
    const rest = id.slice(SKILL_PREFIX.length);
    const dot = rest.indexOf('.');
    return { rest, skill: dot < 0 ? rest : rest.slice(0, dot), specialty: dot < 0 ? null : rest.slice(dot + 1) };
  };
  const named = (name: string, specialty: string | null) => (specialty === null ? name : `${name} (${humanise(specialty)})`);
  const rowOf = (id: string, label: string, group: string | null, tag?: string): CyberpunkSheetRow => ({
    label,
    group,
    ...(tag === undefined ? {} : { tag }),
    cells: [cell(`${SKILL_LEVEL_PREFIX}${split(id).rest}`), cell(id)],
  });

  if (skillCatalog === null) {
    return bases.map((id) => rowOf(id, named(humanise(split(id).skill), split(id).specialty), null));
  }
  const bySlug = new Map(skillCatalog.map((record) => [cyberpunkSkillSlug(String(record.name)), record]));
  const categories: string[] = [];
  for (const record of skillCatalog) {
    const category = String(record.category);
    if (!categories.includes(category)) {
      categories.push(category);
    }
  }
  return categories.flatMap((category) =>
    bases.flatMap((id) => {
      const { skill, specialty } = split(id);
      const record = bySlug.get(skill);
      return record === undefined || String(record.category) !== category
        ? []
        : [rowOf(id, named(String(record.name), specialty), category, String(record.stat))];
    }),
  );
}
