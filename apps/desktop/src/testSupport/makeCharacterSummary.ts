import type { CharacterSummaryDto } from '../boundary/loadListSavedCharacters';

/**
 * Canonical saved-character summary fixture for tests.
 *
 * Single source of truth for the summary shape, mirroring `makeSurface.ts`'s
 * pattern — keep additions here so consumers don't drift when fields change.
 */
export function makeCharacterSummary(overrides: Partial<CharacterSummaryDto> = {}): CharacterSummaryDto {
  return {
    characterId: 'char-test',
    displayLabel: 'Test Character',
    gameSystem: 'pf1',
    schemaVersion: 2,
    savedAt: '2026-07-08T00:00:00Z',
    raceId: 'race:human',
    classSummary: 'class:fighter:1',
    ...overrides,
  };
}
