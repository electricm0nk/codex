import { invoke } from '@tauri-apps/api/core';
import type { PriceMode } from '../characterHub/priceMode';
import type { MarketCategory } from './listMarket';
import type { DiagnosticDto } from './loadCreateCharacter';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the Starfinder and Cyberpunk RED Markets (`sf_adapter.rs`, `sf_catalog.rs`,
 * `cpr_adapter.rs`, `cpr_catalog.rs`). Each system answers from its own catalog and in its own
 * currency: the rows, the prices, the balance and what a buy or sale moved are the backend's.
 */

export type MarketSystem = 'starfinder' | 'cyberpunk';

export interface MarketShellRowDto {
  /** What a buy names the row by. */
  key: string;
  name: string;
  detail: string;
  /** The whole price, in the system's currency; `null` when the record lists none. */
  price: number | null;
}

export interface MarketShellCopyDto {
  /** What a sale names this copy by. */
  key: string;
  name: string;
}

export interface MarketShellStateDto {
  currency: string;
  balance: number;
  held: MarketShellCopyDto[];
}

export type MarketShellOutcome =
  | { kind: 'Saved'; moved: number }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export interface MarketShellRequest {
  characterId: string;
  key: string;
  priceMode: PriceMode;
  savedAt: string;
}

const COMMANDS: Record<MarketSystem, { list: string; state: string; buy: string; sell: string }> = {
  starfinder: {
    list: 'list_starfinder_market',
    state: 'starfinder_market_state',
    buy: 'buy_starfinder_market',
    sell: 'sell_starfinder_market',
  },
  cyberpunk: {
    list: 'list_cyberpunk_market',
    state: 'cyberpunk_market_state',
    buy: 'buy_cyberpunk_market',
    sell: 'sell_cyberpunk_market',
  },
};

async function call<T>(system: MarketSystem, command: string, args: Record<string, unknown>): Promise<T> {
  if (!hasTauriRuntime()) {
    throw new Error(`Tauri runtime not available for the ${system} Market`);
  }
  try {
    return await invoke<T>(command, args);
  } catch (cause: unknown) {
    throw new Error(`The ${system} Market failed: ${formatError(cause)}`);
  }
}

export function listSystemMarket(system: MarketSystem, category: MarketCategory): Promise<MarketShellRowDto[]> {
  return call(system, COMMANDS[system].list, { category });
}

export function loadSystemMarketState(system: MarketSystem, characterId: string): Promise<MarketShellStateDto> {
  return call(system, COMMANDS[system].state, { characterId });
}

export function buySystemMarket(system: MarketSystem, request: MarketShellRequest): Promise<MarketShellOutcome> {
  return call(system, COMMANDS[system].buy, { request });
}

export function sellSystemMarket(system: MarketSystem, request: MarketShellRequest): Promise<MarketShellOutcome> {
  return call(system, COMMANDS[system].sell, { request });
}
