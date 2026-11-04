import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { OverrideFlagDto } from './loadSavedCharacterDetail';
import type { ActiveStateDto } from './setEquipmentActiveState';

/**
 * Read-only desktop boundary over what a character holds, one row per copy, for the Market's Sell
 * list and the Gear tab's Equipment control. Invokes `list_held_equipment`, which reads the saved
 * selections.
 */

export interface HeldEquipmentCopyDto {
  itemId: string;
  /** The copy as a player reads it: `+1 Longsword`, `Dagger`. */
  name: string;
  /** The copy's `+N`, 0 when plain; names this copy to `removeEquipmentSelection` and `setEquipmentActiveState`. */
  bonus: number;
}

/** `BodySlot` in `body_slots.rs`, on the wire. */
export type BodySlotDto =
  | 'armor'
  | 'shield'
  | 'belt'
  | 'body'
  | 'chest'
  | 'eyes'
  | 'feet'
  | 'hands'
  | 'head'
  | 'headband'
  | 'neck'
  | 'ring'
  | 'shoulders'
  | 'wrist'
  | 'wield_main'
  | 'wield_off'
  | 'wield_two_handed'
  | 'none';

/** A held copy with where it is. The Sell list needs only {@link HeldEquipmentCopyDto}. */
export interface PlacedEquipmentCopyDto extends HeldEquipmentCopyDto {
  activeState: ActiveStateDto;
  /** Where a worn copy sits, and what a carried one would take. */
  slot: BodySlotDto;
  /** The override that let this copy in; left out when the rules allowed it. */
  override?: OverrideFlagDto;
}

/** What the Market can buy back: a custom item is the GM's, and has no price. */
export function sellableCopies<T extends HeldEquipmentCopyDto>(copies: T[]): T[] {
  return copies.filter((copy) => !copy.itemId.startsWith('custom:'));
}

export async function listHeldEquipment(characterId: string): Promise<PlacedEquipmentCopyDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing held equipment');
  }

  try {
    return await invoke<PlacedEquipmentCopyDto[]>('list_held_equipment', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list held equipment: ${formatError(cause)}`);
  }
}
