import type { ExplanationDto, LoadSavedCharacterResponse, SheetLineDto } from '../boundary/loadSavedCharacterDetail';
import type { CharacterSummaryDto } from '../boundary/loadListSavedCharacters';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';

/**
 * The Starfinder 1e sheet layout (SD-37 E6.3).
 *
 * Every number this model hands the sheet is one engine explanation row's
 * `value`, formatted (a sign for a bonus) and nothing else: the Starfinder
 * adapter (`sf_adapter.rs`, `SfSheet::explanations`) sends one `sf.*` row per
 * sheet total, ability score and class level. This file holds labels and the
 * order they appear in -- no rule numbers. A row the layout does not name
 * still appears, under "Other totals", so an engine row is never dropped.
 * Starfinder has no CMB, CMD, touch AC or flat-footed AC; none is laid out.
 */

/** The `game_system` a Starfinder save envelope carries (`STARFINDER_RULE_SYSTEM_ID`). */
export const STARFINDER_GAME_SYSTEM = 'starfinder-1e';

/** A non-blocking diagnostic every computed Starfinder build carries: it is about the
 * Pathfinder-shaped snapshot fields this sheet does not show. */
const PATHFINDER_FIELDS_DIAGNOSTIC = 'sf_adapter.pathfinder_fields_zero';

/** Whether a saved character is a Starfinder character (it opens the Starfinder sheet). */
export function isStarfinderCharacter(summary: Pick<CharacterSummaryDto, 'gameSystem'>): boolean {
  return summary.gameSystem === STARFINDER_GAME_SYSTEM;
}

/** One number on the sheet: an engine row's value, formatted. */
export interface StarfinderSheetCell {
  rowId: string;
  value: string;
  /** The engine's own derivation text for the row, verbatim. */
  detail: string;
}

export interface StarfinderSheetRow {
  label: string;
  /** One cell per column; `null` where the engine sent no row (e.g. cantrips per day). */
  cells: (StarfinderSheetCell | null)[];
}

export interface StarfinderSheetSection {
  id: string;
  title: string;
  /** Column headings when the section has more than one value per row. */
  columns: string[] | null;
  rows: StarfinderSheetRow[];
}

export interface StarfinderSheet {
  name: string;
  /** The race's printed label (its sheet line), or `null` when no race line was sent. */
  raceLabel: string | null;
  sections: StarfinderSheetSection[];
  /** Diagnostics that stop the build from computing. */
  blocking: DiagnosticDto[];
  /** Other diagnostics worth showing (the Pathfinder-fields note is not). */
  notes: DiagnosticDto[];
  sheetLines: SheetLineDto[];
  sheetRulesUnavailableReason: string | null;
}

const ABILITIES: readonly { key: string; label: string }[] = [
  { key: 'strength', label: 'Strength' },
  { key: 'dexterity', label: 'Dexterity' },
  { key: 'constitution', label: 'Constitution' },
  { key: 'intelligence', label: 'Intelligence' },
  { key: 'wisdom', label: 'Wisdom' },
  { key: 'charisma', label: 'Charisma' },
];

/** The single-value sections, in sheet order: row id -> label, and whether it is a bonus. */
const SINGLE_VALUE_SECTIONS: readonly { id: string; title: string; rows: readonly { id: string; label: string; signed: boolean }[] }[] = [
  {
    id: 'vitals',
    title: 'Stamina, Hit Points and Resolve',
    rows: [
      { id: 'sf.stamina', label: 'Stamina Points', signed: false },
      { id: 'sf.hit_points', label: 'Hit Points', signed: false },
      { id: 'sf.resolve', label: 'Resolve Points', signed: false },
    ],
  },
  {
    id: 'defense',
    title: 'Armor Class',
    rows: [
      { id: 'sf.eac', label: 'Energy Armor Class (EAC)', signed: false },
      { id: 'sf.kac', label: 'Kinetic Armor Class (KAC)', signed: false },
      { id: 'sf.initiative', label: 'Initiative', signed: true },
    ],
  },
  {
    id: 'saves',
    title: 'Saving Throws',
    rows: [
      { id: 'sf.fortitude', label: 'Fortitude', signed: true },
      { id: 'sf.reflex', label: 'Reflex', signed: true },
      { id: 'sf.will', label: 'Will', signed: true },
    ],
  },
  {
    id: 'offense',
    title: 'Attack',
    rows: [
      { id: 'sf.base_attack_bonus', label: 'Base Attack Bonus', signed: true },
      { id: 'sf.attack.melee', label: 'Melee attack', signed: true },
      { id: 'sf.attack.ranged', label: 'Ranged attack', signed: true },
    ],
  },
  {
    id: 'carried',
    title: 'Credits and Bulk',
    rows: [
      { id: 'sf.credits.starting', label: 'Starting credits', signed: false },
      { id: 'sf.credits.spent', label: 'Credits spent', signed: false },
      { id: 'sf.credits.remaining', label: 'Credits remaining', signed: false },
      { id: 'sf.bulk', label: 'Bulk carried', signed: false },
      { id: 'sf.bulk_limit.unencumbered_max', label: 'Unencumbered up to (bulk)', signed: false },
      { id: 'sf.bulk_limit.overburdened_above', label: 'Overburdened above (bulk)', signed: false },
    ],
  },
];

/** `sf.weapon.<equipment slug>.<field>`: one row per carried weapon (SD-37 E7.1). The damage
 * cell is the number added to the weapon's dice; the dice print on the weapon's own line. */
const WEAPON_FIELDS: readonly { field: string; column: string }[] = [
  { field: 'attack', column: 'Attack' },
  { field: 'damage', column: 'Damage bonus' },
];

const SPELL_FIELDS: readonly { field: string; column: string }[] = [
  { field: 'per_day', column: 'Per day' },
  { field: 'known', column: 'Known' },
  { field: 'save_dc', column: 'Save DC' },
];

/** `life_science` -> `Life Science`. */
function humanise(slug: string): string {
  return slug
    .split('_')
    .filter((word) => word.length > 0)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

function formatValue(value: number, signed: boolean): string {
  return signed && value >= 0 ? `+${value}` : String(value);
}

/**
 * The label of one `sf.*` row, from the same tables the sheet lays out with (the level-up's
 * change lines use it). A row the layout does not name keeps its id, as on the sheet.
 */
export function starfinderRowLabel(id: string): string {
  for (const section of SINGLE_VALUE_SECTIONS) {
    const row = section.rows.find((candidate) => candidate.id === id);
    if (row !== undefined) {
      return row.label;
    }
  }
  const [, group, first, second, third] = id.split('.');
  const ability = ABILITIES.find((candidate) => candidate.key === first);
  if (group === 'ability_score' && ability !== undefined) {
    return ability.label;
  }
  if (group === 'ability_modifier' && ability !== undefined) {
    return `${ability.label} modifier`;
  }
  if (group === 'class_level' && first !== undefined) {
    return `${humanise(first)} level`;
  }
  if (group === 'skill' && first !== undefined) {
    return humanise(first);
  }
  const weaponField = WEAPON_FIELDS.find((candidate) => candidate.field === second);
  if (group === 'weapon' && first !== undefined && weaponField !== undefined) {
    return `${humanise(first)} ${weaponField.column.toLowerCase()}`;
  }
  const field = SPELL_FIELDS.find((candidate) => candidate.field === third);
  if (group === 'spells' && first !== undefined && second !== undefined && field !== undefined) {
    return `${humanise(first)} spells level ${second}, ${field.column.toLowerCase()}`;
  }
  return id;
}

/** Whether the sheet prints the row as a bonus (with its sign). */
export function starfinderRowSigned(id: string): boolean {
  for (const section of SINGLE_VALUE_SECTIONS) {
    const row = section.rows.find((candidate) => candidate.id === id);
    if (row !== undefined) {
      return row.signed;
    }
  }
  return id.startsWith('sf.ability_modifier.') || id.startsWith('sf.skill.') || id.startsWith('sf.weapon.');
}

/** The printed label of the held record `<book>:<kind>:<slug>` (no `#` sub-line), if sent. */
function recordLabel(lines: readonly SheetLineDto[], kind: string, slug: string): string | null {
  const line = lines.find((candidate) => candidate.kind === kind && !candidate.id.includes('#') && candidate.id.endsWith(`:${kind}:${slug}`));
  return line?.label ?? null;
}

/** Lays out one loaded Starfinder character. Every cell is one explanation row. */
export function buildStarfinderSheet(detail: LoadSavedCharacterResponse): StarfinderSheet {
  const rows = new Map<string, ExplanationDto>(detail.explanations.map((row) => [row.id, row]));
  const placed = new Set<string>();
  const cell = (id: string, signed: boolean): StarfinderSheetCell | null => {
    const row = rows.get(id);
    if (row === undefined) {
      return null;
    }
    placed.add(id);
    return { rowId: id, value: formatValue(row.value, signed), detail: row.detail };
  };
  const idsWithPrefix = (prefix: string) => detail.explanations.map((row) => row.id).filter((id) => id.startsWith(prefix));
  const sections: StarfinderSheetSection[] = [];
  const push = (section: StarfinderSheetSection) => {
    const kept = section.rows.filter((row) => row.cells.some((c) => c !== null));
    if (kept.length > 0) {
      sections.push({ ...section, rows: kept });
    }
  };

  push({
    id: 'classes',
    title: 'Class',
    columns: ['Level'],
    rows: idsWithPrefix('sf.class_level.').map((id) => {
      const slug = id.slice('sf.class_level.'.length);
      return { label: recordLabel(detail.sheetLines, 'class', slug) ?? humanise(slug), cells: [cell(id, false)] };
    }),
  });
  push({
    id: 'abilities',
    title: 'Ability Scores',
    columns: ['Score', 'Modifier'],
    rows: ABILITIES.map((ability) => ({
      label: ability.label,
      cells: [cell(`sf.ability_score.${ability.key}`, false), cell(`sf.ability_modifier.${ability.key}`, true)],
    })),
  });
  for (const section of SINGLE_VALUE_SECTIONS) {
    push({ id: section.id, title: section.title, columns: null, rows: section.rows.map((row) => ({ label: row.label, cells: [cell(row.id, row.signed)] })) });
  }
  // `sf.weapon.<equipment slug>.<field>`: one row per carried weapon.
  const weaponSlugs = [...new Set(idsWithPrefix('sf.weapon.').map((id) => id.split('.')[2]).filter((slug): slug is string => slug !== undefined))];
  push({
    id: 'weapons',
    title: 'Weapons',
    columns: WEAPON_FIELDS.map((field) => field.column),
    rows: weaponSlugs.map((slug) => ({
      label: recordLabel(detail.sheetLines, 'equipment', slug) ?? humanise(slug),
      cells: WEAPON_FIELDS.map((field) => cell(`sf.weapon.${slug}.${field.field}`, true)),
    })),
  });
  push({
    id: 'skills',
    title: 'Skills',
    columns: null,
    rows: idsWithPrefix('sf.skill.').map((id) => ({ label: humanise(id.slice('sf.skill.'.length)), cells: [cell(id, true)] })),
  });

  // `sf.spells.<class>.<level>.<field>`: one table per casting class, one row per spell level.
  const spellLevels = new Map<string, Set<string>>();
  for (const id of idsWithPrefix('sf.spells.')) {
    const [, , castingClass, level] = id.split('.');
    if (castingClass !== undefined && level !== undefined) {
      spellLevels.set(castingClass, (spellLevels.get(castingClass) ?? new Set<string>()).add(level));
    }
  }
  for (const [castingClass, levels] of spellLevels) {
    const className = recordLabel(detail.sheetLines, 'class', castingClass) ?? humanise(castingClass);
    push({
      id: `spells-${castingClass}`,
      title: `${className} Spells`,
      columns: SPELL_FIELDS.map((field) => field.column),
      rows: [...levels].map((level) => ({
        label: `Level ${level}`,
        cells: SPELL_FIELDS.map((field) => cell(`sf.spells.${castingClass}.${level}.${field.field}`, false)),
      })),
    });
  }

  // Any `sf.*` row the layout above does not name: shown, never dropped.
  push({
    id: 'other',
    title: 'Other totals',
    columns: null,
    rows: detail.explanations
      .filter((row) => !placed.has(row.id))
      .map((row) => ({ label: row.id, cells: [cell(row.id, false)] })),
  });

  const race = detail.sheetLines.find((line) => line.kind === 'race' && !line.id.includes('#'));
  return {
    name: detail.summary.displayLabel,
    raceLabel: race?.label ?? null,
    sections,
    blocking: detail.diagnostics.filter((d) => d.claimBlocking),
    notes: detail.diagnostics.filter((d) => !d.claimBlocking && d.id !== PATHFINDER_FIELDS_DIAGNOSTIC),
    sheetLines: detail.sheetLines,
    sheetRulesUnavailableReason: detail.sheetRulesUnavailableReason,
  };
}
