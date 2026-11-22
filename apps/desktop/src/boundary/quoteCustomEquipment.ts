import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CustomEquipmentBuild } from './purchaseCustomEquipment';

/**
 * Read desktop boundary for the Buy Custom builder's running total (SD-41 `decisions.md §24`).
 *
 * Invokes `quote_custom_equipment`, the dry run of `purchase_custom_equipment`: one copy's price and
 * weight from the rules, or why the build cannot be bought. Nothing is saved.
 */

export interface CustomEquipmentQuote {
  /** The modifier keys the copy would hold, in order. */
  modifiers: string[];
  /** One copy's price in gp; `null` when a part has no price, and the buy is priced by Negotiate. */
  priceGp: number | null;
  /** The parts with no price, by key. */
  unpriced: string[];
  /** One copy's weight in lb. */
  weightLbs: number | null;
}

export async function quoteCustomEquipment(build: CustomEquipmentBuild): Promise<CustomEquipmentQuote> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for pricing custom equipment');
  }

  try {
    return await invoke<CustomEquipmentQuote>('quote_custom_equipment', { build });
  } catch (cause: unknown) {
    throw new Error(`Failed to price the custom item: ${formatError(cause)}`);
  }
}
