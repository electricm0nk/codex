import { useState } from 'react';
import type { HeldEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { MarketCategory } from '../boundary/listMarket';
import {
  buySystemMarket,
  listSystemMarket,
  loadSystemMarketState,
  sellSystemMarket,
  type MarketShellOutcome,
  type MarketShellRowDto,
  type MarketShellStateDto,
  type MarketSystem,
} from '../boundary/systemMarket';
import { formatMarketPrice, type MarketRow } from './itemPickerFilter';
import { MARKET_CATEGORIES, MarketDialog } from './MarketDialog';
import { DEFAULT_PRICE_MODE, type PriceMode } from './priceMode';

/**
 * The Market on the Starfinder and Cyberpunk RED sheets: the Pathfinder sheet's dialog over the
 * system's own catalog and currency. It has no `+N` picker, because a row here offers no variant.
 */

export const STARFINDER_MARKET_CATEGORIES = MARKET_CATEGORIES;
export const CYBERPUNK_MARKET_CATEGORIES = MARKET_CATEGORIES.filter((category) => category.value !== 'magicItem');

export function rowsOfShell(rows: MarketShellRowDto[]): MarketRow[] {
  return rows.map((row) => ({ key: row.key, name: row.name, detail: row.detail, priceGp: row.price, variants: [] }));
}

/** The held copies in the dialog's row shape; `itemId` is the key the sale names the copy by. */
export function copiesOfState(state: MarketShellStateDto): HeldEquipmentCopyDto[] {
  return state.held.map((copy) => ({ itemId: copy.key, name: copy.name, bonus: 0 }));
}

export function marketLine(action: 'buy' | 'sell', name: string, moved: number, currency: string): string {
  if (moved === 0) {
    return `${action === 'buy' ? 'Added' : 'Removed'} ${name}; no money moved.`;
  }
  return `${action === 'buy' ? 'Bought' : 'Sold'} ${name} for ${formatMarketPrice(moved, currency)}.`;
}

export function SystemMarketDialog(props: {
  system: MarketSystem;
  categories: ReadonlyArray<{ value: MarketCategory; label: string }>;
  characterId: string;
  /** The state the sheet read to open the dialog: the first balance and the currency. */
  initial: MarketShellStateDto;
  /** A buy or sale was saved: the sheet re-reads the character. */
  onChanged: (line: string) => void;
  onClose: () => void;
}) {
  const { system, characterId } = props;
  const [state, setState] = useState(props.initial);
  const [priceMode, setPriceMode] = useState<PriceMode>(DEFAULT_PRICE_MODE);
  const [error, setError] = useState<string | null>(null);

  async function act(action: 'buy' | 'sell', key: string, name: string, run: typeof buySystemMarket): Promise<string | null> {
    setError(null);
    try {
      const outcome: MarketShellOutcome = await run(system, { characterId, key, priceMode, savedAt: new Date().toISOString() });
      if (outcome.kind === 'Blocked') {
        setError(outcome.diagnostics.map((diagnostic) => diagnostic.message).join('; '));
        return null;
      }
      const line = marketLine(action, name, outcome.moved, state.currency);
      props.onChanged(line);
      return line;
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
      return null;
    }
  }

  return (
    <MarketDialog
      initialCategory={props.categories[0].value}
      categories={props.categories}
      loadRows={(category) => listSystemMarket(system, category).then(rowsOfShell)}
      currency={state.currency}
      balance={state.balance}
      loadInventory={() =>
        loadSystemMarketState(system, characterId).then((next) => {
          setState(next);
          return copiesOfState(next);
        })
      }
      onBuy={(row) => act('buy', row.key, row.name, buySystemMarket)}
      onSell={(copy) => act('sell', copy.itemId, copy.name, sellSystemMarket)}
      priceMode={priceMode}
      onPriceModeChange={setPriceMode}
      error={error}
      onClose={props.onClose}
    />
  );
}
