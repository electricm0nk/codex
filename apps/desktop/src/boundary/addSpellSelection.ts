import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for adding a spell selection to a saved character.
 *
 * Invokes the `add_spell_selection` Tauri command, which loads the saved
 * character, appends the requested spell selection, recomputes via the
 * real rules-core engine, and either re-saves and returns the fresh
 * `Saved` envelope or leaves the on-disk character untouched and returns
 * `Blocked` with the real diagnostics — same invariant as
 * `levelUpCharacter`. Returns the same tagged `CreateCharacterOutcome`
 * union `level_up_character` / `create_character` return, so callers share
 * one outcome-mapping path via `toCharacterMutationRefresh`.
 */

/** Mirrors `AcquisitionModeDto` in `character_hub.rs` — a bare string on the wire, not an object. */
export type AcquisitionModeDto = 'Known' | 'Prepared' | 'Granted';

export interface AddSpellSelectionRequest {
  characterId: string;
  spellId: string;
  sourceClassId: string;
  acquisitionMode: AcquisitionModeDto;
  savedAt: string;
}

export async function addSpellSelection(request: AddSpellSelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adding a spell selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('add_spell_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to add spell selection: ${formatError(cause)}`);
  }
}
