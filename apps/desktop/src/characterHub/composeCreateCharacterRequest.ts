import type { AbilityScoresDto, CreateCharacterRequest } from '../boundary/loadCreateCharacter';

export interface CreateCharacterFormFields {
  displayLabel: string;
  raceId: string;
  level: number;
  abilityScores: AbilityScoresDto;
}

export interface ComposeCreateCharacterRequestDependencies {
  generateId: () => string;
  now: () => string;
}

/**
 * Pure request composer — dependency-injected id/clock so callers (and
 * tests) control identity and timestamp generation instead of this module
 * reaching for `crypto.randomUUID()` / `Date` directly.
 */
export function composeCreateCharacterRequest(
  fields: CreateCharacterFormFields,
  deps: ComposeCreateCharacterRequestDependencies
): CreateCharacterRequest {
  return {
    characterId: deps.generateId(),
    displayLabel: fields.displayLabel,
    raceId: fields.raceId,
    level: fields.level,
    abilityScores: { ...fields.abilityScores },
    savedAt: deps.now(),
  };
}
