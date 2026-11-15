import { gpToCopper, copperToGp } from '../boundary/characterMoney';
import type { BodySlotDto } from '../boundary/listHeldEquipment';
import type { MarketEntryDto } from '../boundary/listMarket';
import { formatMarketPrice, mapEquipmentCatalogEntries, marketPriceGp, type MarketRow } from './itemPickerFilter';
import { wornLabel } from './gearScreenModel';
import { refundPreviewCopper, type PriceMode } from './priceMode';

/**
 * The Market table's pure model (SD-41 `decisions.md §24, §26, §28`): the columns, the sort, Only Show
 * Affordable, the Quantity label and the Negotiate prefills. No price is computed here: every figure is
 * the backend's listed price, times a count the player typed, or `refundPreviewCopper`'s mirror of the
 * sell rule.
 */

/** A Market row with the table's Weight and Slot columns and the description box's text. A system
 * shell's rows (Starfinder, Cyberpunk RED) carry none of the three. */
export interface MarketTableRow extends MarketRow {
  weightLbs?: number | null;
  slot?: BodySlotDto;
  description?: string | null;
}

export type MarketSortColumn = 'item' | 'price' | 'weight' | 'slot';
export type SortDirection = 'asc' | 'desc';

export const MARKET_SORT_COLUMNS: ReadonlyArray<{ column: MarketSortColumn; label: string }> = [
  { column: 'item', label: 'Item' },
  { column: 'price', label: 'Price' },
  { column: 'weight', label: 'Weight' },
  { column: 'slot', label: 'Slot' },
];

export function mapMarketTableRows(entries: MarketEntryDto[]): MarketTableRow[] {
  const picked = mapEquipmentCatalogEntries(entries);
  const rows = entries.map((entry, index) => ({
    ...picked[index],
    priceGp: entry.costGp,
    variants: entry.variants,
    weightLbs: entry.weightLbs,
    slot: entry.slot,
    description: entry.description,
  }));
  return sortEntries(rows, 'item', 'asc');
}

/** What the column compares; `null` (no price, no weight, no slot) sorts last in both directions. */
function sortValue(row: MarketTableRow, column: MarketSortColumn): string | number | null {
  switch (column) {
    case 'item':
      return row.name;
    case 'price':
      return row.priceGp;
    case 'weight':
      return row.weightLbs ?? null;
    case 'slot':
      return row.slot && row.slot !== 'none' ? wornLabel(row.slot) : null;
  }
}

/** The rows by `column`, a new list; ties read by name. */
export function sortEntries<T extends MarketTableRow>(rows: readonly T[], column: MarketSortColumn, direction: SortDirection): T[] {
  const sign = direction === 'asc' ? 1 : -1;
  return [...rows].sort((a, b) => {
    const left = sortValue(a, column);
    const right = sortValue(b, column);
    if (left === null || right === null) {
      return left === right ? a.name.localeCompare(b.name) : left === null ? 1 : -1;
    }
    const order = typeof left === 'number' && typeof right === 'number' ? left - right : String(left).localeCompare(String(right));
    return order === 0 ? a.name.localeCompare(b.name) : sign * order;
  });
}

/**
 * Whether one of `row` at `bonus` fits the funds at the price mode (buying is 100% in `standard` and
 * `characterBuild`, nothing when `cashless`). A row with no price is kept: it buys through Negotiate.
 */
export function affordable(row: MarketRow, bonus: number, mode: PriceMode, fundsCopper: number): boolean {
  const price = marketPriceGp(row, bonus);
  return mode === 'cashless' || price === null || gpToCopper(price) <= fundsCopper;
}

/** Buy opens Negotiate when the row lists no price at `bonus` (`decisions.md §30.1`). */
export function buyRoute(row: MarketRow, bonus: number): 'buy' | 'negotiate' {
  return marketPriceGp(row, bonus) === null ? 'negotiate' : 'buy';
}

/** The Quantity field's count: whole, at least one. */
export function parseQuantity(text: string): number {
  const count = Math.floor(Number(text));
  return Number.isFinite(count) && count >= 1 ? count : 1;
}

/** The price beside Buy: `3 × 2 gp = 6 gp`, or the one price. */
export function quantityTotalLabel(priceGp: number | null, quantity: number, currency: string): string {
  if (priceGp === null || quantity === 1) {
    return formatMarketPrice(priceGp, currency);
  }
  return `${quantity} × ${formatMarketPrice(priceGp, currency)} = ${formatMarketPrice(copperToGp(gpToCopper(priceGp) * quantity), currency)}`;
}

/** Buy's Negotiate prefill: the listed price × quantity at the mode; `null` when nothing is listed. */
export function buyNegotiateGp(row: MarketRow, bonus: number, quantity: number, mode: PriceMode): number | null {
  const price = marketPriceGp(row, bonus);
  if (price === null) {
    return null;
  }
  return mode === 'cashless' ? 0 : copperToGp(gpToCopper(price) * quantity);
}

/**
 * Sell's Negotiate prefill: the sell rule on `pieces` of a bundle of `baseQuantity` priced `bundleGp`,
 * floored to a copper first as the backend does (`refund_copper`, mirrored by `refundPreviewCopper`).
 */
export function sellNegotiateGp(bundleGp: number | null, pieces: number, baseQuantity: number, mode: PriceMode): number | null {
  if (bundleGp === null) {
    return null;
  }
  const share = Math.floor((gpToCopper(bundleGp) * pieces) / Math.max(1, baseQuantity));
  return copperToGp(refundPreviewCopper(share, mode));
}
