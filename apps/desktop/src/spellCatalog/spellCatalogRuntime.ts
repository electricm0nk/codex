import { loadSpellCatalog, type SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `equipmentCatalog/equipmentCatalogRuntime.ts` convention. */
function buildPreviewCatalog(): SpellCatalogEntryDto[] {
  return [
    { key: 'Alarm', school: 'Abjuration', level: 1, description: 'Alarm creates a subtle ward on an area you select.' },
    { key: 'Acid Arrow', school: 'Conjuration', level: 2, description: 'An arrow of acid springs from your hand and speeds to its target.' },
    { key: 'Analyze Dweomer', school: 'Divination', level: 6, description: 'You can observe magical auras.' },
    { key: 'Aid', school: 'Enchantment', level: 2, description: 'Aid grants +1 morale bonus on attack rolls and saves vs fear effects.' },
    { key: 'Arcane Mark', school: 'Universal', level: 0, description: 'This spell allows you to inscribe your personal rune or mark.' },
  ];
}

export async function loadSpellCatalogRuntime(): Promise<SpellCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadSpellCatalog();
  return response.entries;
}
