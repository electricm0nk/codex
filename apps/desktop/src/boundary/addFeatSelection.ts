import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for adding a feat selection to a saved character.
 *
 * Invokes the `add_feat_selection` Tauri command, which loads the saved
 * character, appends the requested feat to `chosen.selected_feats`,
 * recomputes via the real rules-core engine, and either re-saves and
 * returns the fresh `Saved` envelope or leaves the on-disk character
 * untouched and returns `Blocked` with the real diagnostics — same
 * invariant as `addSpellSelection`/`addEquipmentSelection`. Returns the
 * same tagged `CreateCharacterOutcome` union those commands return.
 */

export interface AddFeatSelectionRequest {
  characterId: string;
  featId: string;
  savedAt: string;
}

export async function addFeatSelection(request: AddFeatSelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adding a feat selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('add_feat_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to add feat selection: ${formatError(cause)}`);
  }
}
