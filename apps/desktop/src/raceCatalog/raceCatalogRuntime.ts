import { loadRaceCatalog, type RaceCatalogEntryDto } from '../boundary/loadRaceCatalog';
import { hasTauriRuntime } from '../boundary/runtime';

/** Sample data for the browser preview (no Tauri backend) — keeps the
 * catalog screen walkable without the desktop runtime, matching the
 * `classCatalog/classCatalogRuntime.ts` convention. */
function buildPreviewCatalog(): RaceCatalogEntryDto[] {
  return [
    { raceId: 'Human', traitName: 'Ability Bonus', value: 0, detail: 'Player-chosen +2 to any one ability score.' },
    { raceId: 'Human', traitName: 'Speed', value: 30, detail: '30 ft base land speed.' },
    { raceId: 'Dwarf', traitName: 'Stonecunning', value: 2, detail: '+2 Perception to notice unusual stonework.' },
    { raceId: 'Elf', traitName: 'Keen Senses', value: 2, detail: '+2 Perception skill checks.' },
    { raceId: 'Halfling', traitName: 'Sure-Footed', value: 2, detail: '+2 Acrobatics and Climb skill checks.' },
  ];
}

export async function loadRaceCatalogRuntime(): Promise<RaceCatalogEntryDto[]> {
  if (!hasTauriRuntime()) {
    return buildPreviewCatalog();
  }
  const response = await loadRaceCatalog();
  return response.entries;
}
