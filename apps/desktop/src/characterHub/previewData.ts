import type { CharacterHubListSurface } from './buildCharacterHubListSurface';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';

/**
 * Sample data for the browser preview (localhost:1420) where the Tauri backend
 * is absent, so `list_saved_characters` / `load_saved_character` cannot run.
 * This keeps the Load → sheet flow walkable for UI work; the real desktop app
 * never uses it (guarded by `hasTauriRuntime()`).
 */

export const PREVIEW_CHARACTER_ID = 'preview:aldric-ironhand';

export function buildPreviewListSurface(): CharacterHubListSurface {
  return {
    rows: [
      {
        characterId: PREVIEW_CHARACTER_ID,
        displayLabel: 'Aldric Ironhand',
        gameSystemLabel: 'Pathfinder 1st Edition',
        raceLabel: 'Human',
        classSummary: 'class:fighter:1',
        savedAtLabel: 'Preview',
      },
    ],
    isEmpty: false,
    emptyStateMessage: null,
    unreadableNotice: 'Preview mode — no desktop backend detected; showing a sample character.',
  };
}

export function buildPreviewDetail(): LoadSavedCharacterResponse {
  return {
    summary: {
      characterId: PREVIEW_CHARACTER_ID,
      displayLabel: 'Aldric Ironhand',
      gameSystem: 'pf1',
      schemaVersion: 1,
      savedAt: 'preview',
      raceId: 'race:human',
      classSummary: 'class:fighter:1',
    },
    snapshot: {
      abilityModifiers: { strength: 3, dexterity: 1, constitution: 2, intelligence: 0, wisdom: 1, charisma: -1 },
      baseAttackBonus: 1,
      baseSaves: { fortitude: 2, reflex: 0, will: 0 },
      baselineMeleeAttackBonus: 4,
      baselineArmorClass: 14,
      totalSaves: { fortitude: 4, reflex: 1, will: 1 },
      selectedSkillModifiers: { climb: 4, intimidate: 3, swim: 4 },
    },
    diagnostics: [],
  };
}
