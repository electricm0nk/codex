import type { BaseSavesDto } from '../boundary/loadCreateCharacter';

/**
 * Pure presentation backing the Defense tab's saving-throw rows (F-4, scout
 * audit item 58).
 *
 * `PilotSnapshotDto.baseSaves` (class progression only) and `totalSaves`
 * (with the ability modifier and every other engine-known contributor
 * applied) have both crossed the Tauri boundary on every load since the
 * snapshot DTO was introduced; the tab just never rendered them and showed
 * a "coming soon" line instead. This module formats those two engine
 * values side by side. It deliberately does **not** subtract them to show a
 * "modifier" column: the engine owns the breakdown of what sits between
 * base and total, and inventing a single lump here would be TS rules math.
 */
export interface SaveRow {
  label: 'Fortitude' | 'Reflex' | 'Will';
  base: number;
  total: number;
  renderedBase: string;
  renderedTotal: string;
}

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

export function buildSaveRows(base: BaseSavesDto, total: BaseSavesDto): SaveRow[] {
  const rows: Array<[SaveRow['label'], keyof BaseSavesDto]> = [
    ['Fortitude', 'fortitude'],
    ['Reflex', 'reflex'],
    ['Will', 'will'],
  ];
  return rows.map(([label, key]) => ({
    label,
    base: base[key],
    total: total[key],
    renderedBase: signed(base[key]),
    renderedTotal: signed(total[key]),
  }));
}
