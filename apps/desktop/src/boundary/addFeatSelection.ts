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
  /**
   * The target a chooser feat names — a weapon, skill or school, WITHOUT a
   * prefix. The backend reads the prefix and choice-set id from the feat's
   * own contract, so callers never assemble selection ids.
   *
   * Omit for feats that take no target. Omitting it for a chooser feat is
   * also legitimate and records the feat as untargeted; supplying one for a
   * feat that takes none is rejected rather than silently dropped.
   */
  target?: string | null;
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
