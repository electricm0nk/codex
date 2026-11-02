import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over what a character holds, one row per copy, for the Market's Sell
 * list. Invokes `list_held_equipment`, which reads the saved selections.
 */

export interface HeldEquipmentCopyDto {
  itemId: string;
  /** The copy as a player reads it: `+1 Longsword`, `Dagger`. */
  name: string;
  /** The copy's `+N`, 0 when plain; names this copy to `removeEquipmentSelection`. */
  bonus: number;
}

export async function listHeldEquipment(characterId: string): Promise<HeldEquipmentCopyDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing held equipment');
  }

  try {
    return await invoke<HeldEquipmentCopyDto[]>('list_held_equipment', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list held equipment: ${formatError(cause)}`);
  }
}
