import { copperToGp } from '../boundary/characterMoney';
import { marketPriceGp, type MarketRow } from './itemPickerFilter';
import { sellNegotiateGp } from './marketModel';
import type { PriceMode } from './priceMode';

/**
 * The gear screen's sell window (SD-43 `decisions.md §10`): Market Rate (`standard`, 50 %), Build
 * (`characterBuild`, 100 %) or Negotiate (a typed gp total). Cashless is left out. The choice is the
 * window's own: it never writes the session price mode, and `PRICE_MODE_OPTIONS` is not used here.
 */
export type SellOption = 'marketRate' | 'build' | 'negotiate';

export const SELL_OPTIONS: ReadonlyArray<{ value: SellOption; label: string }> = [
  { value: 'marketRate', label: 'Market Rate' },
  { value: 'build', label: 'Build' },
  { value: 'negotiate', label: 'Negotiate' },
];

export const DEFAULT_SELL_OPTION: SellOption = 'marketRate';

/** Negotiate's field opens on the Market Rate amount. */
const RULE: Record<SellOption, PriceMode> = { marketRate: 'standard', build: 'characterBuild', negotiate: 'standard' };

/** What `pieces` of a bundle of `baseQuantity` priced `bundleGp` sell for under `option`; `null` without a price. */
export function sellAmountGp(bundleGp: number | null, pieces: number, baseQuantity: number, option: SellOption): number | null {
  return sellNegotiateGp(bundleGp, pieces, baseQuantity, RULE[option]);
}

export function sellAmountText(amountGp: number | null): string {
  return amountGp === null ? '' : String(amountGp);
}

/**
 * The gp total the Sell write is sent (`onSell(row, quantity, negotiatedGp)`), or the refusal to show.
 * Market Rate and Build send their own amount, so the window's choice holds whatever the session mode is;
 * with no listed price nothing is sent and Sell still removes the item.
 */
export function sellNegotiatedGp(option: SellOption, amountGp: number | null, typed: string): number | undefined | string {
  if (option !== 'negotiate') {
    return amountGp ?? undefined;
  }
  const gp = typed.trim() === '' ? Number.NaN : Number(typed);
  return Number.isFinite(gp) && gp >= 0 ? gp : 'Enter the agreed total in gp (0 or more).';
}

/**
 * The bundle price the backend sells a held stack at (`copy_listed_copper`): the copy's own cost
 * override, else its record's price for a plain copy, else the Market's `+N` offer price; `null` when
 * none is listed.
 */
export function sellBundleGp(held: { bonus: number; costOverrideCopper?: number }, entryCostGp: number | null, marketRow: MarketRow | null): number | null {
  if (held.costOverrideCopper !== undefined) {
    return copperToGp(held.costOverrideCopper);
  }
  if (held.bonus === 0) {
    return entryCostGp;
  }
  return marketRow ? marketPriceGp(marketRow, held.bonus) : null;
}
