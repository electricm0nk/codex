import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CopyKeyDto, HeldEquipmentCopyDto } from './listHeldEquipment';

/**
 * Write desktop boundary for naming one held copy (SD-41 `decisions.md §24` (b)).
 *
 * Invokes the `rename_equipment_copy` Tauri command, which keeps the name in the character's
 * `equipment_sets.json` copies map (a blank name clears it) and returns the held stacks, which
 * show the copy by that name.
 */

export interface RenameEquipmentCopyRequest {
  characterId: string;
  copy: CopyKeyDto;
  /** The copy's name; blank clears it. */
  name: string;
}

export async function renameEquipmentCopy(request: RenameEquipmentCopyRequest): Promise<HeldEquipmentCopyDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for renaming equipment');
  }

  try {
    return await invoke<HeldEquipmentCopyDto[]>('rename_equipment_copy', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to rename equipment: ${formatError(cause)}`);
  }
}
