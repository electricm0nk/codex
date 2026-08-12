import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for adding an equipment selection to a saved
 * character.
 *
 * Invokes the `add_equipment_selection` Tauri command, which loads the
 * saved character, appends the requested equipment selection, recomputes
 * via the real rules-core engine, and either re-saves and returns the
 * fresh `Saved` envelope or leaves the on-disk character untouched and
 * returns `Blocked` with the real diagnostics — same invariant as
 * `levelUpCharacter`. Returns the same tagged `CreateCharacterOutcome`
 * union `level_up_character` / `create_character` return, so callers share
 * one outcome-mapping path via `toCharacterMutationRefresh`.
 */

/** Mirrors `ActiveStateDto` in `character_hub.rs` — a bare string on the wire, not an object. */
export type ActiveStateDto = 'EquippedActive' | 'Absent' | 'SelectedInactive';

export interface AddEquipmentSelectionRequest {
  characterId: string;
  itemId: string;
  activeState: ActiveStateDto;
  savedAt: string;
}

export async function addEquipmentSelection(
  request: AddEquipmentSelectionRequest
): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adding an equipment selection');
  }

  try {
    return await invoke<CreateCharacterOutcome>('add_equipment_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to add equipment selection: ${formatError(cause)}`);
  }
}
