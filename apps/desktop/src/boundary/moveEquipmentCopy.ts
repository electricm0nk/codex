import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';
import type { CopyKeyDto, LocationDto } from './listHeldEquipment';

/**
 * Write desktop boundary for moving a held copy in the active equipment set.
 *
 * Invokes the `move_equipment_copy` Tauri command, which moves `quantity` pieces (omitted: the
 * whole stack `copy` is in) to `location`, projects the active set onto the saved character,
 * recomputes via the real rules-core engine, and re-saves. Part of a stack splits: the moved pieces
 * join a stack of the same item at `location`, or start one. Drop is a move to `notCarried`.
 * Comes back `Blocked`, nothing saved, for a full slot (`equipment.slot_taken`), a target that is
 * not a container (`equipment.not_a_container`) or one inside the moved copy
 * (`equipment.container_cycle`).
 */

export interface MoveEquipmentCopyRequest {
  characterId: string;
  copy: CopyKeyDto;
  location: LocationDto;
  /** Pieces to move; omit for the whole stack. */
  quantity?: number;
  savedAt: string;
}

export async function moveEquipmentCopy(request: MoveEquipmentCopyRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for moving equipment');
  }

  try {
    return await invoke<CreateCharacterOutcome>('move_equipment_copy', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to move equipment: ${formatError(cause)}`);
  }
}
