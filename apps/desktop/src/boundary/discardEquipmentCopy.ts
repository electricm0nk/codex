import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';
import type { CopyKeyDto } from './listHeldEquipment';

/**
 * Write desktop boundary for discarding held pieces for no money (SD-41 `decisions.md §27`).
 *
 * Invokes the `discard_equipment_copy` Tauri command, which removes `quantity` pieces (omitted:
 * the whole stack `copy` is in): whole lines leave the character and every equipment set, and a
 * line discarded in part keeps the rest. Nothing is refunded; Sell is `removeEquipmentSelection`.
 */

export interface DiscardEquipmentCopyRequest {
  characterId: string;
  copy: CopyKeyDto;
  /** Pieces to discard; omit for the whole stack. */
  quantity?: number;
  savedAt: string;
}

export async function discardEquipmentCopy(request: DiscardEquipmentCopyRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for discarding equipment');
  }

  try {
    return await invoke<CreateCharacterOutcome>('discard_equipment_copy', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to discard equipment: ${formatError(cause)}`);
  }
}
