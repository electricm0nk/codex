import { loadSpellCatalog, type SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `equipmentCatalog/equipmentCatalogRuntime.ts` convention. */
function buildPreviewCatalog(): SpellCatalogEntryDto[] {
  return [
    { key: 'Alarm', book: 'CRB', school: 'Abjuration', level: 1, description: 'Alarm creates a subtle ward on an area you select.', duration: null, range: null },
    { key: 'Acid Arrow', book: 'CRB', school: 'Conjuration', level: 2, description: 'An arrow of acid springs from your hand and speeds to its target.', duration: null, range: null },
    { key: 'Analyze Dweomer', book: 'CRB', school: 'Divination', level: 6, description: 'You can observe magical auras.', duration: null, range: null },
    { key: 'Aid', book: 'CRB', school: 'Enchantment', level: 2, description: 'Aid grants +1 morale bonus on attack rolls and saves vs fear effects.', duration: null, range: null },
    { key: 'Arcane Mark', book: 'CRB', school: 'Universal', level: 0, description: 'This spell allows you to inscribe your personal rune or mark.', duration: null, range: null },
    { key: 'Adhesive Blood', book: 'ACG', school: 'Transmutation', level: 2, description: 'Your blood thickens to become a glue-like substance upon contact with air.', duration: null, range: null },
    { key: 'Absorbing Touch', book: 'APG', school: 'Transmutation', level: 1, description: 'Your touch absorbs an object into your body.', duration: null, range: null },
  ];
}

export async function loadSpellCatalogRuntime(): Promise<SpellCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadSpellCatalog();
  return response.entries;
}
