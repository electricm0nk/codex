import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { HitPointLevelDto } from './loadCreateCharacter';

/**
 * Read boundary over the hit point results saved with a character (`hit_points.json`, written when
 * the character was created from the Levels list): one die result per character level, in order.
 * The sheet's HP follows these; a character created without them (or before they existed) has an
 * empty list and uses the default rule (maximum, then average).
 */
export interface CharacterHitPointsDto {
  levels: HitPointLevelDto[];
}

/** Resolves to an empty list when none were saved, and outside a Tauri runtime (browser preview). */
export async function loadCharacterHitPoints(characterId: string): Promise<CharacterHitPointsDto> {
  if (!hasTauriRuntime()) {
    return { levels: [] };
  }
  try {
    return await invoke<CharacterHitPointsDto>('load_character_hit_points', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load saved hit points: ${formatError(cause)}`);
  }
}
