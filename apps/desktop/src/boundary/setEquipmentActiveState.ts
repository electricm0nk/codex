import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for stowing, re-equipping, or marking absent a
 * carried equipment item.
 *
 * Invokes the `set_equipment_active_state` Tauri command, a move of one copy in the active
 * equipment set (`moveEquipmentCopy`): the first copy of the item not already in that state (of the
 * copies `bonus` names, when given) goes to its own slot when `EquippedActive` (or Equipped when it
 * takes none), Carried when `SelectedInactive`, Not Carried when `Absent`; the set is projected,
 * recomputed via the real rules-core engine, and re-saved. Wearing an item whose slot is held comes
 * back `Blocked`, with a diagnostic reading "head slot is taken by <item>"; nothing is saved.
 */

export type ActiveStateDto = 'EquippedActive' | 'Absent' | 'SelectedInactive';

export interface SetEquipmentActiveStateRequest {
  characterId: string;
  itemId: string;
  /** Which copy, when the character holds several: `0` a plain one, `N` a `+N`. Omit for the first. */
  bonus?: number;
  /** The state the row shows this copy in, which says which of two alike moves. Omit for the first not already in `activeState`. */
  fromState?: ActiveStateDto;
  activeState: ActiveStateDto;
  savedAt: string;
}

export async function setEquipmentActiveState(request: SetEquipmentActiveStateRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for changing an equipment item\'s active state');
  }

  try {
    return await invoke<CreateCharacterOutcome>('set_equipment_active_state', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to set equipment active state: ${formatError(cause)}`);
  }
}
