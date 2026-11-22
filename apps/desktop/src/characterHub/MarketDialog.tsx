import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { gpToCopper } from '../boundary/characterMoney';
import type { HeldEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { MarketCategory } from '../boundary/listMarket';
import type { CustomEquipmentBuild } from '../boundary/purchaseCustomEquipment';
import { BuyCustomDialog } from './BuyCustomDialog';
import { CreateItemDialog, type CreateItemFields } from './CreateItemDialog';
import { wornLabel } from './gearScreenModel';
import { filterItemPickerEntries, formatMarketPrice, MARKET_ROW_LIMIT, marketPriceGp, marketShownRows } from './itemPickerFilter';
import {
  affordable,
  buyNegotiateGp,
  buyRoute,
  MARKET_SORT_COLUMNS,
  parseQuantity,
  quantityTotalLabel,
  sellNegotiateGp,
  sortEntries,
  type MarketSortColumn,
  type MarketTableRow,
  type SortDirection,
} from './marketModel';
import { PriceModeControl } from './PriceModeControl';
import type { PriceMode } from './priceMode';

/**
 * The one place a character buys and sells equipment (SD-41 `decisions.md §24–§26, §28`): category
 * tabs over `list_market` as a table (Item, Price, Weight, Slot, each sortable), a search, Only Show
 * Affordable, a description box for the highlighted row, a `+N` choice for a row that offers one, a
 * Quantity with Buy and Sell, Negotiate on both, Buy Custom and Create; and the character's own
 * stacks with Sell. Which rows exist, what they cost and whether a buy goes through are the backend's
 * answers; this file only shows them.
 */

export const MARKET_CATEGORIES: ReadonlyArray<{ value: MarketCategory; label: string }> = [
  { value: 'weapon', label: 'Weapons' },
  { value: 'armor', label: 'Armor' },
  { value: 'shield', label: 'Shields' },
  { value: 'gear', label: 'Gear' },
  { value: 'magicItem', label: 'Magic items' },
];

/** A held stack the Sell list shows; a Pathfinder stack also says its pieces. */
export type MarketHeldCopy = HeldEquipmentCopyDto & { quantity?: number; baseQuantity?: number };

/** Quantity (bundles to buy, pieces to sell) and the agreed total, when the player negotiated one. */
export interface MarketDeal {
  quantity: number;
  negotiatedGp?: number;
}

/** What Buy Custom and Create tell the player, or why nothing happened. */
export type MarketOutcome = { line: string } | { error: string };

export type MarketNegotiation = { kind: 'buy' } | { kind: 'sell'; copy: MarketHeldCopy };

const smallButton = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  padding: '0.3rem 0.65rem',
} as const;

const cell = { borderBottom: '1px solid var(--color-border)', padding: '0.3rem 0.6rem', textAlign: 'left' } as const;
const numberInput = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  padding: '0.3rem 0.4rem',
  width: '5rem',
} as const;

/** Quantity and Negotiate: the Pathfinder sheet's Market takes them; a system shell's does not. */
export interface MarketDealsProps {
  quantity: string;
  onQuantity: (quantity: string) => void;
  negotiation: MarketNegotiation | null;
  negotiatedGp: string;
  onNegotiatedGp: (gp: string) => void;
  onNegotiateBuy: () => void;
  onNegotiateSell: (copy: MarketHeldCopy) => void;
  onConfirmNegotiation: () => void;
  onCancelNegotiation: () => void;
}

/** What the whole held stack sells for under `mode`: the engine's sell rule (`sellNegotiateGp`), `null` without a listed price. */
export function sellValueGp(row: MarketTableRow | null, copy: MarketHeldCopy, mode: PriceMode): number | null {
  return row ? sellNegotiateGp(marketPriceGp(row, copy.bonus), copy.quantity ?? 1, copy.baseQuantity ?? 1, mode) : null;
}

/** A legacy `item:longsword` id and the Market's `Longsword (Base)` key name one record. */
const recordName = (id: string) => id.replace(/^item:/, '').replace(/ \(base\)$/i, '').replace(/[_\s]+/g, ' ').toLowerCase();

/** The held copy's own Market row among `rows`, named as the Market names it (a custom name is not). */
export function rowOfHeldCopy(rows: MarketTableRow[], copy: MarketHeldCopy): MarketTableRow | null {
  const legacy = copy.itemId.startsWith('item:');
  return (
    rows.find(
      (row) =>
        (row.key === copy.itemId || (legacy && recordName(row.key) === recordName(copy.itemId))) &&
        (copy.bonus === 0 ? row.name : row.variants.find((variant) => variant.bonus === copy.bonus)?.name) === copy.name,
    ) ?? null
  );
}

export interface MarketDialogViewProps {
  category: MarketCategory;
  onCategory: (category: MarketCategory) => void;
  /** The tabs, when the system's catalog has fewer than Pathfinder's. */
  categories?: ReadonlyArray<{ value: MarketCategory; label: string }>;
  search: string;
  onSearch: (search: string) => void;
  /** The current category's rows, already searched, filtered and sorted. */
  rows: MarketTableRow[];
  loading: boolean;
  loadError: string | null;
  selectedKey: string | null;
  onSelectRow: (key: string) => void;
  /** The chosen `+N`; 0 is the plain item. */
  bonus: number;
  onBonus: (bonus: number) => void;
  currency: string;
  /** The character's balance, in `currency`. */
  balance: number;
  onBuy: () => void;
  /** What the character holds, one row per stack; `null` until it has loaded. */
  inventory: MarketHeldCopy[] | null;
  onSell: (copy: MarketHeldCopy) => void;
  /** The gp one held stack sells for under the live price option; `null` is no price. Absent, the row has no value cell. */
  sellValue?: (copy: MarketHeldCopy) => number | null;
  priceMode: PriceMode;
  onPriceModeChange: (mode: PriceMode) => void;
  /** A buy or sale is in flight: a second click must not charge twice. */
  busy: boolean;
  status: string | null;
  error: string | null;
  onClose: () => void;
  sort?: { column: MarketSortColumn; direction: SortDirection };
  onSort?: (column: MarketSortColumn) => void;
  onlyAffordable?: boolean;
  onOnlyAffordable?: (checked: boolean) => void;
  deals?: MarketDealsProps;
  /** Opens the builder on the highlighted row; absent, there is no Buy Custom. */
  onBuyCustom?: () => void;
  /** Opens Create; absent, there is no Create. */
  onCreate?: () => void;
}

function weightLabel(row: MarketTableRow): string {
  return row.weightLbs == null ? '—' : `${row.weightLbs.toLocaleString('en-US', { maximumFractionDigits: 2 })} lb`;
}

function slotLabel(row: MarketTableRow): string {
  return row.slot && row.slot !== 'none' ? wornLabel(row.slot) : '—';
}

/** The dialog's face, without state or portal, so a test can render it. */
export function MarketDialogView(props: MarketDialogViewProps) {
  const selected = props.rows.find((row) => row.key === props.selectedKey) ?? null;
  const shown = marketShownRows(props.rows) as MarketTableRow[];
  const price = selected ? marketPriceGp(selected, props.bonus) : null;
  const deals = props.deals;
  const quantity = deals ? parseQuantity(deals.quantity) : 1;
  const negotiation = deals?.negotiation ?? null;
  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-label="Market"
      onClick={(event) => event.stopPropagation()}
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-border)',
        borderRadius: 12,
        boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
        display: 'flex',
        flexDirection: 'column',
        height: 'min(860px, 94vh)',
        width: 'min(1200px, 96vw)',
      }}
    >
      <header style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '0.85rem 1.5rem' }}>
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Market</h2>
        <span style={{ alignItems: 'center', display: 'flex', gap: '1rem' }}>
          <span style={{ fontSize: '0.9rem', fontWeight: 700 }}>Balance: {formatMarketPrice(props.balance, props.currency)}</span>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </span>
      </header>

      <div style={{ borderBottom: '1px solid var(--color-border)', padding: '0.75rem 1.5rem 0' }}>
        <PriceModeControl value={props.priceMode} onChange={props.onPriceModeChange} groupName="market-price-mode" />
        <div role="tablist" aria-label="Market categories" style={{ display: 'flex', flexWrap: 'wrap', gap: '0.25rem', marginBottom: '0.6rem' }}>
          {(props.categories ?? MARKET_CATEGORIES).map((category) => {
            const active = category.value === props.category;
            return (
              <button
                key={category.value}
                type="button"
                role="tab"
                aria-selected={active}
                onClick={() => props.onCategory(category.value)}
                style={{
                  background: active ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderBottom: active ? '3px solid var(--color-accent)' : '3px solid transparent',
                  color: active ? 'var(--color-accent)' : 'var(--color-text)',
                  cursor: 'pointer',
                  fontWeight: active ? 700 : 500,
                  padding: '0.4rem 0.9rem',
                }}
              >
                {category.label}
              </button>
            );
          })}
        </div>
        <input
          type="text"
          autoFocus
          aria-label="Search the market"
          placeholder="Search the market…"
          value={props.search}
          onChange={(event) => props.onSearch(event.target.value)}
          style={{
            backgroundColor: 'var(--color-surface-2)',
            border: '1px solid var(--color-border)',
            borderRadius: 6,
            boxSizing: 'border-box',
            color: 'var(--color-text)',
            marginBottom: '0.5rem',
            padding: '0.5rem 0.6rem',
            width: '100%',
          }}
        />
        {props.onOnlyAffordable ? (
          <label style={{ display: 'block', fontSize: '0.85rem', marginBottom: '0.5rem' }}>
            <input
              type="checkbox"
              data-testid="market-only-affordable"
              checked={props.onlyAffordable ?? false}
              onChange={(event) => props.onOnlyAffordable?.(event.target.checked)}
            />{' '}
            Only Show Affordable
          </label>
        ) : null}
      </div>

      <div data-testid="market-body" style={{ display: 'flex', flex: 1, minHeight: 0 }}>
        <div style={{ display: 'flex', flex: '2 1 480px', flexDirection: 'column', minHeight: 0 }}>
          <div style={{ flex: 1, minHeight: 0, overflowY: 'auto' }}>
            {props.loadError ? <p role="alert" style={{ color: 'var(--color-error, #c0392b)', fontSize: '0.85rem', padding: '1rem' }}>{props.loadError}</p> : null}
            {props.loading ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', padding: '1rem' }}>Loading…</p> : null}
            {!props.loading && !props.loadError && props.rows.length === 0 ? (
              <p style={{ color: 'var(--color-text-faint)', fontSize: '0.85rem', padding: '1rem', textAlign: 'center' }}>No matches.</p>
            ) : null}
            {shown.length < props.rows.length ? (
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: 0, padding: '0.5rem 0.75rem' }}>
                Showing the first {MARKET_ROW_LIMIT} of {props.rows.length.toLocaleString('en-US')}. Search to narrow the list.
              </p>
            ) : null}
            <table data-testid="market-table" style={{ borderCollapse: 'collapse', fontSize: '0.85rem', width: '100%' }}>
              <thead>
                <tr>
                  {MARKET_SORT_COLUMNS.map(({ column, label }) => {
                    const sorted = props.sort?.column === column;
                    const arrow = sorted ? (props.sort?.direction === 'asc' ? ' ▲' : ' ▼') : '';
                    return (
                      <th key={column} style={cell} aria-sort={sorted ? (props.sort?.direction === 'asc' ? 'ascending' : 'descending') : 'none'}>
                        {props.onSort ? (
                          <button
                            type="button"
                            data-testid={`market-sort-${column}`}
                            onClick={() => props.onSort?.(column)}
                            style={{ background: 'none', border: 'none', color: 'var(--color-text)', cursor: 'pointer', fontWeight: 700, padding: 0 }}
                          >
                            {label}
                            {arrow}
                          </button>
                        ) : (
                          label
                        )}
                      </th>
                    );
                  })}
                </tr>
              </thead>
              <tbody>
                {shown.map((row, index) => {
                  const active = row.key === props.selectedKey;
                  return (
                    <tr key={`${row.key}::${index}`} aria-selected={active} onClick={() => props.onSelectRow(row.key)} style={{ background: active ? 'var(--color-surface-2)' : 'none', cursor: 'pointer' }}>
                      <td style={cell}>
                        <button
                          type="button"
                          aria-pressed={active}
                          onClick={(event) => {
                            event.stopPropagation();
                            props.onSelectRow(row.key);
                          }}
                          style={{ background: 'none', border: 'none', color: active ? 'var(--color-accent)' : 'var(--color-text)', cursor: 'pointer', fontWeight: 700, padding: 0, textAlign: 'left' }}
                        >
                          {row.name}
                        </button>
                      </td>
                      <td style={cell}>{formatMarketPrice(row.priceGp, props.currency)}</td>
                      <td style={cell}>{weightLabel(row)}</td>
                      <td style={cell}>{slotLabel(row)}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
          <div
            data-testid="market-description"
            aria-label="Description"
            style={{ borderTop: '1px solid var(--color-border)', fontSize: '0.8rem', maxHeight: '9rem', minHeight: '4rem', overflowY: 'auto', padding: '0.5rem 0.75rem', whiteSpace: 'pre-wrap' }}
          >
            {selected ? (selected.description ?? selected.detail ?? '') || 'No description.' : 'Pick an item to read its description.'}
          </div>
        </div>

        <aside aria-label="Inventory" style={{ borderLeft: '1px solid var(--color-border)', flex: '1 1 280px', minHeight: 0, overflowY: 'auto', padding: '0.5rem 0.9rem' }}>
          <h3 style={{ fontSize: '0.8rem', letterSpacing: '0.04em', margin: '0.25rem 0 0.5rem', textTransform: 'uppercase' }}>Inventory</h3>
          {props.inventory?.length === 0 ? <p style={{ color: 'var(--color-text-faint)', fontSize: '0.85rem', margin: 0 }}>Nothing held yet.</p> : null}
          {props.inventory && props.inventory.length > 0 ? (
            <table style={{ borderCollapse: 'collapse', fontSize: '0.8rem', width: '100%' }}>
              <tbody>
                {props.inventory.map((copy, index) => (
                  <tr key={`${copy.itemId}::${index}`}>
                    <td style={{ ...cell, width: '100%' }}>
                      <button type="button" disabled={props.busy} onClick={() => props.onSell(copy)} style={{ ...smallButton, textAlign: 'left', width: '100%' }}>
                        {`Sell ${copy.name}${copy.activeState === 'Absent' ? ' (left behind)' : ''}`}
                      </button>
                    </td>
                    <td style={cell}>{copy.quantity !== undefined && copy.quantity > 1 ? `×${copy.quantity}` : ''}</td>
                    {props.sellValue ? (
                      <td data-testid="market-sell-value" style={{ ...cell, whiteSpace: 'nowrap' }}>
                        {props.priceMode === 'cashless' ? '\u2014' : (() => {
                          const gp = props.sellValue(copy);
                          return gp === null ? '\u2014' : formatMarketPrice(gp, props.currency);
                        })()}
                      </td>
                    ) : null}
                    {deals ? (
                      <td style={cell}>
                        <button type="button" disabled={props.busy} onClick={() => deals.onNegotiateSell(copy)} style={smallButton}>
                          Negotiate
                        </button>
                      </td>
                    ) : null}
                  </tr>
                ))}
              </tbody>
            </table>
          ) : null}
        </aside>
      </div>

      <footer style={{ alignItems: 'center', borderTop: '1px solid var(--color-border)', display: 'flex', flexWrap: 'wrap', gap: '0.75rem', justifyContent: 'space-between', padding: '0.75rem 1.5rem' }}>
        <div style={{ flex: '1 1 320px', minWidth: 0 }}>
          {props.error ? <p role="alert" style={{ color: 'var(--color-error, #c0392b)', fontSize: '0.8rem', margin: '0 0 0.3rem' }}>{props.error}</p> : null}
          {props.status ? <p role="status" style={{ fontSize: '0.8rem', margin: '0 0 0.3rem' }}>{props.status}</p> : null}
          {selected ? (
            <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
              <span style={{ fontWeight: 700, marginRight: '0.4rem' }}>{selected.name}</span>
              {selected.variants.length > 0
                ? [{ bonus: 0, label: 'Plain' }, ...selected.variants.map((variant) => ({ bonus: variant.bonus, label: `+${variant.bonus}` }))].map((choice) => (
                    <button
                      key={choice.bonus}
                      type="button"
                      aria-pressed={choice.bonus === props.bonus}
                      onClick={() => props.onBonus(choice.bonus)}
                      style={{ ...smallButton, borderColor: choice.bonus === props.bonus ? 'var(--color-accent)' : 'var(--color-border)' }}
                    >
                      {choice.label}
                    </button>
                  ))
                : null}
            </div>
          ) : (
            <p style={{ color: 'var(--color-text-faint)', fontSize: '0.85rem', margin: 0 }}>Pick an item to buy.</p>
          )}
          {negotiation ? (
            <div role="group" aria-label="Negotiate" style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginTop: '0.4rem' }}>
              <span style={{ fontSize: '0.85rem' }}>
                {negotiation.kind === 'buy' ? `Pay for ${quantity} × ${selected?.name ?? ''}` : `Ask for ${quantity} of ${negotiation.copy.name}`} ({props.currency}, total
                {negotiation.kind === 'sell' ? ', blank = the pricing above' : price === null ? ', required' : ', blank = the listed price'})
              </span>
              <input aria-label="Negotiated price" type="number" min={0} step="any" value={deals?.negotiatedGp ?? ''} onChange={(event) => deals?.onNegotiatedGp(event.target.value)} style={numberInput} />
              <button
                type="button"
                disabled={props.busy || (negotiation.kind === 'buy' && price === null && (deals?.negotiatedGp ?? '').trim() === '')}
                onClick={deals?.onConfirmNegotiation}
                style={smallButton}
              >
                {negotiation.kind === 'buy' ? 'Buy at this price' : 'Sell at this price'}
              </button>
              <button type="button" onClick={deals?.onCancelNegotiation} style={smallButton}>
                Cancel
              </button>
            </div>
          ) : null}
        </div>
        <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.6rem' }}>
          {selected ? <span style={{ fontWeight: 700 }}>{deals ? quantityTotalLabel(price, quantity, props.currency) : formatMarketPrice(price, props.currency)}</span> : null}
          {deals ? (
            <label style={{ fontSize: '0.85rem' }}>
              Quantity{' '}
              <input data-testid="market-quantity" aria-label="Quantity" type="number" min={1} value={deals.quantity} onChange={(event) => deals.onQuantity(event.target.value)} style={numberInput} />
            </label>
          ) : null}
          {deals ? (
            <button type="button" data-testid="market-negotiate" disabled={!selected || props.busy} onClick={deals.onNegotiateBuy} style={smallButton}>
              Negotiate
            </button>
          ) : null}
          <button
            type="button"
            disabled={!selected || props.busy}
            onClick={props.onBuy}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-on-accent)',
              cursor: selected && !props.busy ? 'pointer' : 'default',
              fontWeight: 600,
              opacity: selected && !props.busy ? 1 : 0.6,
              padding: '0.45rem 1.1rem',
            }}
          >
            Buy
          </button>
          {props.onBuyCustom ? (
            <button type="button" data-testid="market-buy-custom" disabled={!selected || props.busy} onClick={props.onBuyCustom} style={smallButton}>
              Buy Custom
            </button>
          ) : null}
          {props.onCreate ? (
            <button type="button" data-testid="market-create" disabled={props.busy} onClick={props.onCreate} style={smallButton}>
              Create
            </button>
          ) : null}
        </div>
      </footer>
    </div>
  );
}

/**
 * The dialog: loads a category when it is first shown, holds the search, sort, filter, pick, quantity
 * and negotiation, and hands a buy or a sale to the sheet, which owns the character. `onBuy` and
 * `onSell` resolve to what to tell the player, or `null` when the sheet's `error` already says why
 * nothing happened. `takesDeals` adds Quantity and Negotiate (the sheet's handlers take a `deal`);
 * `onBuyCustom` and `onCreate` add those buttons.
 */
export function MarketDialog(props: {
  initialCategory: MarketCategory;
  categories?: ReadonlyArray<{ value: MarketCategory; label: string }>;
  loadRows: (category: MarketCategory) => Promise<MarketTableRow[]>;
  currency: string;
  balance: number;
  loadInventory: () => Promise<MarketHeldCopy[]>;
  onBuy: (row: MarketTableRow, bonus: number, deal?: MarketDeal) => Promise<string | null>;
  onSell: (copy: MarketHeldCopy, deal?: MarketDeal) => Promise<string | null>;
  takesDeals?: boolean;
  onBuyCustom?: (build: CustomEquipmentBuild, quantity: number, negotiatedGp?: number) => Promise<MarketOutcome>;
  onCreate?: (fields: CreateItemFields) => Promise<MarketOutcome>;
  priceMode: PriceMode;
  onPriceModeChange: (mode: PriceMode) => void;
  error: string | null;
  onClose: () => void;
}) {
  const [category, setCategory] = useState(props.initialCategory);
  const [loaded, setLoaded] = useState<Partial<Record<MarketCategory, MarketTableRow[]>>>({});
  const [loadError, setLoadError] = useState<string | null>(null);
  const [inventory, setInventory] = useState<MarketHeldCopy[] | null>(null);
  const [search, setSearch] = useState('');
  const [sort, setSort] = useState<{ column: MarketSortColumn; direction: SortDirection }>({ column: 'item', direction: 'asc' });
  const [onlyAffordable, setOnlyAffordable] = useState(false);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);
  const [bonus, setBonus] = useState(0);
  const [quantity, setQuantity] = useState('1');
  const [negotiation, setNegotiation] = useState<MarketNegotiation | null>(null);
  const [negotiatedGp, setNegotiatedGp] = useState('');
  const [subDialog, setSubDialog] = useState<'buyCustom' | 'create' | null>(null);
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState<string | null>(null);
  // Synchronous: two clicks in one frame both see `busy` false in state.
  const inFlight = useRef(false);

  const rows = loaded[category];
  useEffect(() => {
    if (rows) {
      return;
    }
    let current = true;
    setLoadError(null);
    props
      .loadRows(category)
      .then((next) => {
        if (current) {
          setLoaded((held) => ({ ...held, [category]: next }));
        }
      })
      .catch((cause: unknown) => {
        if (current) {
          setLoadError(cause instanceof Error ? cause.message : 'Failed to load the market.');
        }
      });
    return () => {
      current = false;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [category, rows]);

  function reloadInventory() {
    return props
      .loadInventory()
      .then(setInventory)
      .catch((cause: unknown) => setLoadError(cause instanceof Error ? cause.message : 'Failed to load the inventory.'));
  }

  useEffect(() => {
    void reloadInventory();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function act(action: () => Promise<string | null>) {
    if (inFlight.current) {
      return;
    }
    inFlight.current = true;
    setBusy(true);
    setStatus(null);
    try {
      // The list first, so the line saying what happened never sits over a list that has not caught up.
      const outcome = await action();
      await reloadInventory();
      setStatus(outcome);
      if (outcome !== null) {
        setNegotiation(null);
        setNegotiatedGp('');
      }
    } finally {
      inFlight.current = false;
      setBusy(false);
    }
  }

  /** Buy Custom and Create: the error stays in the sub-dialog; a line closes it and lands in the Market. */
  async function actInSubDialog(action: () => Promise<MarketOutcome>): Promise<string | null> {
    let failure: string | null = null;
    await act(async () => {
      const outcome = await action();
      if ('error' in outcome) {
        failure = outcome.error;
        return null;
      }
      setSubDialog(null);
      return outcome.line;
    });
    return failure;
  }

  const searched = filterItemPickerEntries(rows ?? [], search) as MarketTableRow[];
  const funds = gpToCopper(props.balance);
  const filtered = onlyAffordable ? searched.filter((row) => affordable(row, 0, props.priceMode, funds)) : searched;
  const table = sortEntries(filtered, sort.column, sort.direction);
  const selectedRow = table.find((candidate) => candidate.key === selectedKey) ?? null;
  const count = parseQuantity(quantity);
  const deal = (negotiated?: string): MarketDeal => {
    const gp = negotiated === undefined || negotiated.trim() === '' ? undefined : Number(negotiated);
    return gp === undefined || !Number.isFinite(gp) ? { quantity: count } : { quantity: count, negotiatedGp: gp };
  };
  const sellPieces = (copy: MarketHeldCopy) => Math.min(count, copy.quantity ?? 1);
  /** The held copy's own Market row, when it is loaded and named as the Market names it (a custom name is not). */
  const rowOfCopy = (copy: MarketHeldCopy) => rowOfHeldCopy(Object.values(loaded).flat().filter((row) => row !== undefined), copy);

  const deals: MarketDealsProps | undefined = props.takesDeals
    ? {
        quantity,
        onQuantity: setQuantity,
        negotiation,
        negotiatedGp,
        onNegotiatedGp: setNegotiatedGp,
        onNegotiateBuy: () => {
          if (selectedRow) {
            const prefill = buyNegotiateGp(selectedRow, bonus, count, props.priceMode);
            setNegotiation({ kind: 'buy' });
            setNegotiatedGp(prefill === null ? '' : String(prefill));
          }
        },
        onNegotiateSell: (copy) => {
          const row = rowOfCopy(copy);
          const prefill = row ? sellNegotiateGp(marketPriceGp(row, copy.bonus), sellPieces(copy), copy.baseQuantity ?? 1, props.priceMode) : null;
          setNegotiation({ kind: 'sell', copy });
          setNegotiatedGp(prefill === null ? '' : String(prefill));
        },
        onConfirmNegotiation: () => {
          if (negotiation?.kind === 'buy' && selectedRow) {
            void act(() => props.onBuy(selectedRow, bonus, deal(negotiatedGp)));
          } else if (negotiation?.kind === 'sell') {
            const copy = negotiation.copy;
            void act(() => props.onSell(copy, { ...deal(negotiatedGp), quantity: sellPieces(copy) }));
          }
        },
        onCancelNegotiation: () => {
          setNegotiation(null);
          setNegotiatedGp('');
        },
      }
    : undefined;

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.6)', display: 'flex', inset: 0, justifyContent: 'center', padding: '2rem', position: 'fixed', zIndex: 1100 }}
    >
      <MarketDialogView
        category={category}
        onCategory={(next) => {
          setCategory(next);
          setSelectedKey(null);
          setBonus(0);
          setNegotiation(null);
        }}
        categories={props.categories}
        search={search}
        onSearch={setSearch}
        rows={table}
        loading={!rows && !loadError}
        loadError={loadError}
        selectedKey={selectedKey}
        onSelectRow={(key) => {
          setSelectedKey(key);
          setBonus(0);
          setNegotiation(null);
        }}
        bonus={bonus}
        onBonus={setBonus}
        currency={props.currency}
        balance={props.balance}
        onBuy={() => {
          if (!selectedRow) {
            return;
          }
          if (deals && buyRoute(selectedRow, bonus) === 'negotiate') {
            // No listed price: the player names one (`decisions.md §30.1`).
            setNegotiation({ kind: 'buy' });
            setNegotiatedGp('');
            return;
          }
          void act(() => props.onBuy(selectedRow, bonus, deals ? { quantity: count } : undefined));
        }}
        inventory={inventory}
        sellValue={(copy) => sellValueGp(rowOfCopy(copy), copy, props.priceMode)}
        onSell={(copy) => void act(() => props.onSell(copy, deals ? { quantity: sellPieces(copy) } : undefined))}
        priceMode={props.priceMode}
        onPriceModeChange={props.onPriceModeChange}
        busy={busy}
        status={status}
        error={props.error}
        onClose={props.onClose}
        sort={sort}
        onSort={(column) => setSort((held) => ({ column, direction: held.column === column && held.direction === 'asc' ? 'desc' : 'asc' }))}
        onlyAffordable={onlyAffordable}
        onOnlyAffordable={setOnlyAffordable}
        deals={deals}
        onBuyCustom={props.onBuyCustom ? () => setSubDialog('buyCustom') : undefined}
        onCreate={props.onCreate ? () => setSubDialog('create') : undefined}
      />
      {subDialog === 'buyCustom' && selectedRow && props.onBuyCustom ? (
        <BuyCustomDialog
          baseItemId={selectedRow.key}
          baseName={selectedRow.name}
          currency={props.currency}
          busy={busy}
          onBuy={(build, pieces, negotiated) => actInSubDialog(() => props.onBuyCustom!(build, pieces, negotiated))}
          onClose={() => setSubDialog(null)}
        />
      ) : null}
      {subDialog === 'create' && props.onCreate ? (
        <CreateItemDialog currency={props.currency} busy={busy} onCreate={(fields) => actInSubDialog(() => props.onCreate!(fields))} onClose={() => setSubDialog(null)} />
      ) : null}
    </div>,
    document.body,
  );
}
