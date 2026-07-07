import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';

/**
 * Read/write desktop boundary over character creation.
 *
 * Invokes the `create_character` Tauri command, which composes a
 * `CharacterInput` from the caller's race/class/level choice server-side
 * and computes it via the real rules-core engine. Returns a tagged
 * `Saved` / `Blocked` outcome verbatim — never fabricates a computed
 * result, and a `Blocked` outcome means nothing was persisted.
 */

export interface AbilityScoresDto {
  strength: number;
  dexterity: number;
  constitution: number;
  intelligence: number;
  wisdom: number;
  charisma: number;
}

export interface CreateCharacterRequest {
  characterId: string;
  displayLabel: string;
  raceId: string;
  classId: string;
  level: number;
  abilityScores: AbilityScoresDto;
  savedAt: string;
}

export interface BaseSavesDto {
  fortitude: number;
  reflex: number;
  will: number;
}

export interface SelectedSkillModifiersDto {
  climb: number;
  intimidate: number;
  swim: number;
}

export interface PilotSnapshotDto {
  abilityModifiers: AbilityScoresDto;
  baseAttackBonus: number;
  baseSaves: BaseSavesDto;
  baselineMeleeAttackBonus: number;
  baselineArmorClass: number;
  totalSaves: BaseSavesDto;
  selectedSkillModifiers: SelectedSkillModifiersDto;
}

export interface DiagnosticDto {
  id: string;
  message: string;
  claimBlocking: boolean;
}

export type CreateCharacterOutcome =
  | { kind: 'Saved'; summary: CharacterSummaryDto; snapshot: PilotSnapshotDto }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function loadCreateCharacter(
  request: CreateCharacterRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for creating a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('create_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to create character: ${formatError(cause)}`);
  }
}
