import { loadClassCatalog, type ClassCatalogEntryDto } from '../boundary/loadClassCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `spellCatalog/spellCatalogRuntime.ts` convention. */
function buildPreviewCatalog(): ClassCatalogEntryDto[] {
  return [
    { classId: 'Fighter', level: 1, baseAttackBonus: 1, fortSave: 2, refSave: 0, willSave: 0 },
    { classId: 'Fighter', level: 2, baseAttackBonus: 2, fortSave: 3, refSave: 0, willSave: 0 },
    { classId: 'Wizard', level: 1, baseAttackBonus: 0, fortSave: 0, refSave: 0, willSave: 2 },
    { classId: 'Rogue', level: 1, baseAttackBonus: 0, fortSave: 0, refSave: 2, willSave: 0 },
    { classId: 'Cleric', level: 1, baseAttackBonus: 0, fortSave: 0, refSave: 0, willSave: 2 },
  ];
}

export async function loadClassCatalogRuntime(): Promise<ClassCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadClassCatalog();
  return response.entries;
}
