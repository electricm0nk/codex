import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for leveling up a saved character.
 *
 * Invokes the `level_up_character` Tauri command, which increments (or
 * adds, for a fresh multiclass dip) the requested class's level on the
 * saved character, recomputes via the real rules-core engine, and either
 * re-saves and returns the fresh `Saved` envelope or leaves the on-disk
 * character untouched and returns `Blocked` with the real diagnostics —
 * mirrors `create_character`'s "never persist an unproven build" invariant.
 * Returns the same tagged `CreateCharacterOutcome` union `create_character`
 * / `clone_character` return, so callers can share one outcome-mapping path.
 */

export interface LevelUpCharacterRequest {
  characterId: string;
  classId: string;
  savedAt: string;
}

export async function levelUpCharacter(
  request: LevelUpCharacterRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for leveling up a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('level_up_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to level up character: ${formatError(cause)}`);
  }
}
