import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { CreateSpellDto } from '../boundary/loadCreateCharacter';
import { buildSpellsPerDaySurface } from './spellsPerDayModel';
import type { TransferItem } from './transferListModel';

/**
 * The Create screen's spells dialog: how many spells of each level the draft character may pick
 * (the engine's own spells-per-day rows, one quota per class and spell level), the spells that
 * fill each quota, and the race's innate spell-like abilities for the read-only Innate column.
 *
 * Authors no rules data. A spell's level for a class comes from `list_class_spell_levels`; a quota
 * comes from the `class_spell.*.total_spells_per_day.*` explanations; innate names come from the
 * race's own trait records. A spell-level with no slots offers no spells.
 */

export interface SpellDialogInput {
  perDay: readonly ExplanationDto[];
  classLevels: readonly ClassSpellLevelsDto[];
  catalog: readonly SpellCatalogEntryDto[];
  innate: readonly string[];
}

export interface SpellDialog {
  items: TransferItem[];
  groupLimits: Record<string, number>;
  groupLabels: Record<string, string>;
}

const ID_SEPARATOR = '|';

function titleCase(token: string): string {
  return token
    .split('_')
    .map((word) => (word.length === 0 ? word : word[0]!.toUpperCase() + word.slice(1)))
    .join(' ');
}

export function creationSpellDialog(input: SpellDialogInput): SpellDialog {
  const surface = buildSpellsPerDaySurface(input.perDay);
  const groupLimits: Record<string, number> = {};
  const groupLabels: Record<string, string> = {};
  const items: TransferItem[] = [];
  const byKey = new Map(input.catalog.map((entry) => [entry.key, entry]));

  for (const row of surface.rows) {
    if (row.count <= 0) {
      continue;
    }
    const classId = `class:${row.classToken}`;
    const list = input.classLevels.find((entry) => entry.classId === classId);
    const group = `${row.classToken}:${row.spellLevel}`;
    groupLimits[group] = row.count;
    groupLabels[group] = `${titleCase(row.classToken)} level ${row.spellLevel}`;
    for (const listed of list?.entries ?? []) {
      if (listed.level !== row.spellLevel) {
        continue;
      }
      const entry = byKey.get(listed.key);
      items.push({
        id: `${classId}${ID_SEPARATOR}${listed.key}`,
        label: listed.key,
        description: entry?.description ?? 'No description available.',
        qualified: true,
        group,
      });
    }
  }

  for (const name of input.innate) {
    const entry = input.catalog.find((candidate) => candidate.key === name);
    items.push({
      id: `innate${ID_SEPARATOR}${name}`,
      label: name,
      description: entry?.description ?? 'A racial spell-like ability. The catalog has no description for it.',
      qualified: true,
      innate: true,
    });
  }

  return { items, groupLimits, groupLabels };
}

/** The create requests for the chosen item ids; every pick is added as Known. Innate ids are never requests. */
export function spellSelectionsFromIds(ids: readonly string[]): CreateSpellDto[] {
  return ids.flatMap((id) => {
    const [classId, spellId] = id.split(ID_SEPARATOR);
    if (classId === undefined || spellId === undefined || classId === 'innate') {
      return [];
    }
    return [{ spellId, sourceClassId: classId, acquisitionMode: 'Known' as const }];
  });
}

/** Keeps only the picks the dialog still offers (not innate, class and level still have slots). */
export function keepOfferedSpells(items: readonly TransferItem[], selected: readonly string[]): string[] {
  const offered = new Set(items.filter((item) => item.innate !== true).map((item) => item.id));
  return selected.filter((id) => offered.has(id));
}

/** One sentence per quota group with more picks than slots (empty when every group fits). */
export function spellQuotaOverruns(dialog: SpellDialog, selected: readonly string[]): string[] {
  return Object.entries(dialog.groupLimits).flatMap(([group, limit]) => {
    const chosen = selected.filter((id) => dialog.items.find((item) => item.id === id)?.group === group).length;
    return chosen > limit ? [`${dialog.groupLabels[group] ?? group}: ${chosen} chosen, ${limit} allowed`] : [];
  });
}
