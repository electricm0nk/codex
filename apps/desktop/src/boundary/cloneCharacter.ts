import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Duplicates a saved character's full build (race, classes, ability scores,
 * feats, skills, equipment, spells — and its portrait, if any) under a new
 * id and display label. Invokes the `clone_character` Tauri command, which
 * recomputes the copy via the real rules-core engine before saving it —
 * mirrors `create_character`'s tagged `Saved` / `Blocked` outcome.
 */

export interface CloneCharacterRequest {
  characterId: string;
  newCharacterId: string;
  newDisplayLabel: string;
  savedAt: string;
}

export async function cloneCharacter(request: CloneCharacterRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for cloning a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('clone_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to clone character: ${formatError(cause)}`);
  }
}
