import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for stowing, re-equipping, or marking absent a
 * carried equipment item.
 *
 * Invokes the `set_equipment_active_state` Tauri command, which loads the
 * saved character, sets the named carried item's active state on the first
 * matching copy, recomputes via the real rules-core engine, and re-saves.
 */

export type ActiveStateDto = 'EquippedActive' | 'Absent' | 'SelectedInactive';

export interface SetEquipmentActiveStateRequest {
  characterId: string;
  itemId: string;
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
