import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { OverrideFlagDto } from './loadSavedCharacterDetail';
import type { ActiveStateDto } from './setEquipmentActiveState';

/**
 * Read-only desktop boundary over what a character owns, one row per stack of identical copies,
 * for the Market's Sell list and the Gear tab's Equipment control. Invokes `list_held_equipment`,
 * which reads the saved selections and `equipment_sets.json`.
 */

export interface HeldEquipmentCopyDto {
  itemId: string;
  /** The copy as a player reads it: `+1 Longsword`, `Dagger`. */
  name: string;
  /** The copy's `+N`, 0 when plain; names this copy to `removeEquipmentSelection` and `setEquipmentActiveState`. */
  bonus: number;
  /** Where the copy is, for a list that knows it; `Absent` is owned and left behind. */
  activeState?: ActiveStateDto;
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

/** One owned copy (`CopyKey`): the `ordinal`-th copy of `itemId` at `bonus` in saved order. */
export interface CopyKeyDto {
  itemId: string;
  bonus: number;
  ordinal: number;
}

/** Where a copy is in the active equipment set (`Location` in `equipment_sets.rs`). */
export type LocationDto =
  | { kind: 'slot'; slot: BodySlotDto }
  | { kind: 'equipped' }
  | { kind: 'carried' }
  | { kind: 'notCarried' }
  | { kind: 'inside'; container: CopyKeyDto };

/**
 * A held stack with where it is: identical copies (one `itemId`, `bonus`, name and location) are
 * one row. The Sell list needs only {@link HeldEquipmentCopyDto}.
 */
export interface PlacedEquipmentCopyDto extends HeldEquipmentCopyDto {
  activeState: ActiveStateDto;
  /** Where a worn copy sits, and what a carried one would take. */
  slot: BodySlotDto;
  /** The override that let this copy in; left out when the rules allowed it. */
  override?: OverrideFlagDto;
  /** Where the stack is in the active set. */
  location: LocationDto;
  /** The first copy's ordinal. */
  ordinal: number;
  /** Pieces in the stack (60 arrows, not 3 bundles). */
  quantity: number;
  /** Every copy the row stands for, in saved order. */
  copies: CopyKeyDto[];
  /** Pieces in one bundle as the book sells it; 1 for a single item. */
  baseQuantity: number;
  /** The copy's properties (a `.pcg` line's `SPROP`), printed under it; absent when it has none. */
  properties?: string[];
  /** The die the copy deals instead of its record's, as written; the Weapons row rolls it. */
  damageOverride?: string;
  /** One unit's weight its file states; the engine weighs the copy by it. */
  weightOverrideLbs?: number;
  /** One unit's price in copper its file states; Sell lists the copy at it. */
  costOverrideCopper?: number;
  /** Saved in a slot not of its kind (`"in the Shield slot, a Head item"`); absent when it is not. */
  slotWarning?: string;
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
