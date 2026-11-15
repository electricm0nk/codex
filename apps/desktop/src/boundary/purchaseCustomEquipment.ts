import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { PurchaseEquipmentOutcome } from './purchaseEquipment';
import type { PriceMode } from '../characterHub/priceMode';

/**
 * Write desktop boundary for Buy Custom (SD-41 `decisions.md §24`).
 *
 * Invokes the `purchase_custom_equipment` Tauri command: the backend prices the build from the
 * rules (base + masterwork + enhancement + the special abilities', material's and size's costs),
 * charges it (or the negotiated total), and lands each copy Carried with its modifiers and its
 * custom name. A build with an unpriced part buys only at a negotiated price.
 */

/** What the copy is made of; also the request of `quote_custom_equipment` (the builder's dry run). */
export interface CustomEquipmentBuild {
  /** The Market row the copy is made from, by catalog key. */
  baseItemId: string;
  /** The custom name, shown wherever the copy is named. */
  name?: string;
  masterwork?: boolean;
  /** The `+N`, 0 to 5. */
  enhancement?: number;
  /** Special-ability modifier keys, from `listCustomEquipmentOptions`. */
  specialAbilities?: string[];
  material?: string;
  size?: string;
}

export interface PurchaseCustomEquipmentRequest extends CustomEquipmentBuild {
  characterId: string;
  /** Copies to buy. Absent buys one. */
  quantity?: number;
  /** The agreed total in copper, charged in place of the rules price; the only way to buy an unpriced build. */
  negotiatedPriceCopper?: number;
  savedAt: string;
  priceMode?: PriceMode;
}

export async function purchaseCustomEquipment(request: PurchaseCustomEquipmentRequest): Promise<PurchaseEquipmentOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for buying custom equipment');
  }

  try {
    return await invoke<PurchaseEquipmentOutcome>('purchase_custom_equipment', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to buy custom equipment: ${formatError(cause)}`);
  }
}
