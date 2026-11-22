import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';
import type { EquipmentSetsSummaryDto } from './loadSavedCharacterDetail';

/**
 * Write desktop boundaries for a character's named equipment sets (`equipment_sets.json`).
 *
 * Create and rename change only the file and resolve to the sets' names. Activate and delete
 * project the (new) active set onto the saved character, recompute and re-save, so they resolve to
 * a `CreateCharacterOutcome`. The last set cannot be deleted; deleting the active set activates
 * `Default` or the first remaining.
 */

async function call<T>(command: string, request: object, what: string): Promise<T> {
  if (!hasTauriRuntime()) {
    throw new Error(`Tauri runtime not available for ${what}`);
  }
  try {
    return await invoke<T>(command, { request });
  } catch (cause: unknown) {
    throw new Error(`Failed ${what}: ${formatError(cause)}`);
  }
}

/** A new set: everything Carried, or (`copyActive`) a copy of the active set. */
export function createEquipmentSet(request: { characterId: string; name: string; copyActive: boolean }): Promise<EquipmentSetsSummaryDto> {
  return call('create_equipment_set', request, 'creating an equipment set');
}

export function renameEquipmentSet(request: { characterId: string; from: string; to: string }): Promise<EquipmentSetsSummaryDto> {
  return call('rename_equipment_set', request, 'renaming an equipment set');
}

export function deleteEquipmentSet(request: { characterId: string; name: string; savedAt: string }): Promise<CreateCharacterOutcome> {
  return call('delete_equipment_set', request, 'deleting an equipment set');
}

export function activateEquipmentSet(request: { characterId: string; name: string; savedAt: string }): Promise<CreateCharacterOutcome> {
  return call('activate_equipment_set', request, 'activating an equipment set');
}
