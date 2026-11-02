import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { EquipmentCatalogEntryDto } from './loadEquipmentCatalog';

/**
 * Read-only desktop boundary over one Market list. Invokes `list_market`, which returns only the
 * requested category (the Armor list holds no ammunition, weapon or rod) and, on arms and armor
 * with a price, the `+1`…`+5` each row can be bought at.
 */

export type MarketCategory = 'weapon' | 'armor' | 'shield' | 'gear' | 'magicItem';

export interface MarketFilter {
  category: MarketCategory;
  /** Case-insensitive substring match against the row's shown name. */
  nameContains?: string | null;
  /** Exact match against a catalog book code; absent spans every book. */
  book?: string | null;
}

export interface MarketVariantDto {
  bonus: number;
  name: string;
  /** The whole price in gp: base + masterwork + the enhancement. */
  priceGp: number;
  /** The equipment modifier that carries the bonus; it has no price of its own. */
  modifierKey: string;
}

export interface MarketEntryDto extends EquipmentCatalogEntryDto {
  /** Empty when the row takes no `+N`. */
  variants: MarketVariantDto[];
}

export interface MarketResponse {
  entries: MarketEntryDto[];
  /** Why the equipment type data could not be read; `entries` is then empty. */
  typesError?: string | null;
}

export async function listMarket(filter: MarketFilter): Promise<MarketResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing the market');
  }

  try {
    return await invoke<MarketResponse>('list_market', { filter });
  } catch (cause: unknown) {
    throw new Error(`Failed to list the market: ${formatError(cause)}`);
  }
}
