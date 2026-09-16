import type { ClassCatalogEntryDto } from '../boundary/loadClassCatalog';

/**
 * The class preview beside the create form's class select (v0.8 F-11,
 * scout audit item 9): the `list_class_catalog` row for the picked class
 * at the picked level, formatted. Every number is the engine's; the only
 * work here is the join (the catalog keys rows by display name, the form
 * by `CLASS_OPTIONS` id, and the option's label is that display name) and
 * sign formatting. Skill points per level are not on the catalog DTO and
 * are not derived here.
 */
export type ClassPreview =
  | { kind: 'Loading' }
  | { kind: 'Unavailable'; message: string }
  | { kind: 'Row'; level: number; baseAttackBonus: string; fortSave: string; refSave: string; willSave: string };

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

export function buildClassPreview(
  catalog: readonly ClassCatalogEntryDto[] | null,
  classLabel: string,
  level: number,
): ClassPreview {
  if (catalog === null) {
    return { kind: 'Loading' };
  }
  const forClass = catalog.filter((entry) => entry.classId === classLabel);
  if (forClass.length === 0) {
    return { kind: 'Unavailable', message: `The class catalog has no progression rows for ${classLabel} yet.` };
  }
  const row = forClass.find((entry) => entry.level === level);
  if (!row) {
    return { kind: 'Unavailable', message: `The class catalog has no level ${level} row for ${classLabel}.` };
  }
  return {
    kind: 'Row',
    level: row.level,
    baseAttackBonus: signed(row.baseAttackBonus),
    fortSave: signed(row.fortSave),
    refSave: signed(row.refSave),
    willSave: signed(row.willSave),
  };
}
