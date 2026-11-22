import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { DiagnosticDto } from './loadCreateCharacter';
import type { CharacterMoneyDto } from './characterMoney';
import type { BodySlotDto, CopyKeyDto } from './listHeldEquipment';
import type { CharacterCustom } from '../characterHub/customModel';
import type { PriceMode } from '../characterHub/priceMode';

/**
 * Write desktop boundary for making an item with no corpus base (SD-41 `decisions.md §25`).
 *
 * Invokes `create_custom_item`, which writes a custom equipment record to `custom.json` with its
 * weight, price and container capacity, lists it Carried (no set names it), and deducts a price
 * above 0 at the price mode through the money path a purchase takes (`cashless` charges nothing).
 * Comes back `Blocked`, nothing written, when the funds are short
 * (`money.equipment_purchase.insufficient_funds`). Edit and remove live in the Custom tab.
 */

export interface CreateCustomItemRequest {
  characterId: string;
  name: string;
  description: string;
  weightLbs: number | null;
  priceGp: number | null;
  /** Set: the item is a container holding this many lb. */
  containerCapacityLbs: number | null;
  slot: BodySlotDto | null;
  savedAt: string;
  /** Absent means `standard`. */
  priceMode?: PriceMode;
}

export type CreateCustomItemOutcome =
  | {
      kind: 'Created';
      /** The character's custom data with the new record last. */
      custom: CharacterCustom;
      money: CharacterMoneyDto;
      /** The new copy, as `moveEquipmentCopy` names it. */
      copy: CopyKeyDto;
    }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function createCustomItem(request: CreateCustomItemRequest): Promise<CreateCustomItemOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for creating an item');
  }

  try {
    return await invoke<CreateCustomItemOutcome>('create_custom_item', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to create the item: ${formatError(cause)}`);
  }
}
