import type { AbilityScoresDto, CreateCharacterRequest } from '../boundary/loadCreateCharacter';
import type { AbilityKey } from './characterHubModel';

/**
 * Bakes a race's fixed ability-score adjustments into the raw entered/rolled
 * scores before submission. The compute engine's contract (see
 * `apply_human_ability_bonus`'s doc comment in `pilot_compute.rs`) is that
 * every non-Human race's submitted score already includes its racial
 * adjustment — Human is the one exception, whose raw score IS the pre-bonus
 * base, with the player's floating +2 choice applied server-side via
 * `abilityBonusTarget` instead. Human's `abilityAdjustments` is empty, so
 * this is a no-op for Human and doesn't need a special case here.
 */
export function applyRacialAbilityAdjustments(
  rawScores: AbilityScoresDto,
  raceAdjustments: Partial<Record<AbilityKey, number>>
): AbilityScoresDto {
  return {
    strength: rawScores.strength + (raceAdjustments.strength ?? 0),
    dexterity: rawScores.dexterity + (raceAdjustments.dexterity ?? 0),
    constitution: rawScores.constitution + (raceAdjustments.constitution ?? 0),
    intelligence: rawScores.intelligence + (raceAdjustments.intelligence ?? 0),
    wisdom: rawScores.wisdom + (raceAdjustments.wisdom ?? 0),
    charisma: rawScores.charisma + (raceAdjustments.charisma ?? 0),
  };
}

export interface CreateCharacterFormFields {
  displayLabel: string;
  raceId: string;
  classId: string;
  level: number;
  abilityScores: AbilityScoresDto;
  abilityBonusTarget: string;
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
    classId: fields.classId,
    level: fields.level,
    abilityScores: { ...fields.abilityScores },
    abilityBonusTarget: fields.abilityBonusTarget,
    savedAt: deps.now(),
  };
}
