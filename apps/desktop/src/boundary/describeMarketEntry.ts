import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { ContainerStatsDto } from './describeHeldEquipment';
import type { ResolvedEquipmentEffectDto } from './loadCreateCharacter';
import type { MarketEntryDto } from './listMarket';
import type { WeaponDamageDto } from './loadSavedCharacterDetail';

/**
 * Read desktop boundary for the Market's description box (SD-43 `decisions.md §13`). Invokes the
 * `describe_market_entry` Tauri command: the row as the Market lists it (at a `+N`, that variant's
 * name and price), the weapon row and armor stats the engine computes for that one item in hand,
 * and its container facts. `null` is a value the record does not state, never 0.
 */

export interface MarketEntryDetailDto {
  entry: Omit<MarketEntryDto, 'variants'>;
  weapon: WeaponDamageDto | null;
  armor: ResolvedEquipmentEffectDto | null;
  container: ContainerStatsDto | null;
}

export interface DescribeMarketEntryRequest {
  characterId: string;
  itemId: string;
  /** The chosen `+N`; 0 is the plain item. */
  bonus: number;
}

export async function describeMarketEntry(request: DescribeMarketEntryRequest): Promise<MarketEntryDetailDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for describing a Market item');
  }

  try {
    return await invoke<MarketEntryDetailDto>('describe_market_entry', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to describe the Market item: ${formatError(cause)}`);
  }
}
