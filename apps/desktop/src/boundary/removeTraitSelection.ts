import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for removing a character trait/drawback selection
 * from a saved character.
 *
 * Invokes the `remove_trait_selection` Tauri command, which loads the saved
 * character, removes the named trait together with any skill choice recorded
 * for it, recomputes via the real rules-core engine, and re-saves. The
 * inverse of `addTraitSelection`.
 */

export interface RemoveTraitSelectionRequest {
  characterId: string;
  traitId: string;
  savedAt: string;
}

export async function removeTraitSelection(request: RemoveTraitSelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for removing a trait selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('remove_trait_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to remove trait selection: ${formatError(cause)}`);
  }
}
