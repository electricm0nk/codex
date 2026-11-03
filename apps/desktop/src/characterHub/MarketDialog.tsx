import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import type { HeldEquipmentCopyDto } from '../boundary/listHeldEquipment';
import type { MarketCategory } from '../boundary/listMarket';
import {
  filterItemPickerEntries,
  formatMarketPrice,
  MARKET_ROW_LIMIT,
  marketPriceGp,
  marketShownRows,
  type MarketRow,
} from './itemPickerFilter';
import { PriceModeControl } from './PriceModeControl';
import type { PriceMode } from './priceMode';

/**
 * The one place a character buys and sells equipment: category tabs over `list_market`, a search
 * box, a `+N` choice for a row that offers one, the price with its currency, Buy, and the character's
 * own items with Sell. Which rows exist, what they cost and whether a buy goes through are the
 * backend's answers; this file only shows them.
 */

export const MARKET_CATEGORIES: ReadonlyArray<{ value: MarketCategory; label: string }> = [
  { value: 'weapon', label: 'Weapons' },
  { value: 'armor', label: 'Armor' },
  { value: 'shield', label: 'Shields' },
  { value: 'gear', label: 'Gear' },
  { value: 'magicItem', label: 'Magic items' },
];

const smallButton = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  padding: '0.3rem 0.65rem',
} as const;

export interface MarketDialogViewProps {
  category: MarketCategory;
  onCategory: (category: MarketCategory) => void;
  /** The tabs, when the system's catalog has fewer than Pathfinder's. */
  categories?: ReadonlyArray<{ value: MarketCategory; label: string }>;
  search: string;
  onSearch: (search: string) => void;
  /** The current category's rows, already searched. */
  rows: MarketRow[];
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
  /** What the character holds, one row per copy; `null` until it has loaded. */
  inventory: HeldEquipmentCopyDto[] | null;
  onSell: (copy: HeldEquipmentCopyDto) => void;
  priceMode: PriceMode;
  onPriceModeChange: (mode: PriceMode) => void;
  /** A buy or sale is in flight: a second click must not charge twice. */
  busy: boolean;
  status: string | null;
  error: string | null;
  onClose: () => void;
}

/** The dialog's face, without state or portal, so a test can render it. */
export function MarketDialogView(props: MarketDialogViewProps) {
  const selected = props.rows.find((row) => row.key === props.selectedKey) ?? null;
  const shown = marketShownRows(props.rows);
  const price = selected ? marketPriceGp(selected, props.bonus) : null;
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
        height: 'min(820px, 92vh)',
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
      </div>

      <div style={{ display: 'flex', flex: 1, flexWrap: 'wrap', minHeight: 0, overflowY: 'auto' }}>
        <div style={{ flex: '2 1 420px', minHeight: 0, overflowY: 'auto' }}>
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
          {shown.map((row, index) => {
            const active = row.key === props.selectedKey;
            return (
              <button
                key={`${row.key}::${index}`}
                type="button"
                aria-pressed={active}
                onClick={() => props.onSelectRow(row.key)}
                style={{
                  background: active ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderBottom: '1px solid var(--color-border)',
                  cursor: 'pointer',
                  display: 'block',
                  padding: '0.5rem 0.75rem',
                  textAlign: 'left',
                  width: '100%',
                }}
              >
                <p style={{ color: active ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>{row.name}</p>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.1rem 0 0' }}>{row.detail}</p>
              </button>
            );
          })}
        </div>

        <aside aria-label="Inventory" style={{ borderLeft: '1px solid var(--color-border)', flex: '1 1 280px', minHeight: 0, overflowY: 'auto', padding: '0.5rem 0.9rem' }}>
          <h3 style={{ fontSize: '0.8rem', letterSpacing: '0.04em', margin: '0.25rem 0 0.5rem', textTransform: 'uppercase' }}>Inventory</h3>
          {props.inventory?.length === 0 ? <p style={{ color: 'var(--color-text-faint)', fontSize: '0.85rem', margin: 0 }}>Nothing held yet.</p> : null}
          {props.inventory?.map((copy, index) => (
            <div key={`${copy.itemId}::${index}`} style={{ marginBottom: '0.35rem' }}>
              <button type="button" disabled={props.busy} onClick={() => props.onSell(copy)} style={{ ...smallButton, textAlign: 'left', width: '100%' }}>
                {`Sell ${copy.name}`}
              </button>
            </div>
          ))}
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
        </div>
        <div style={{ alignItems: 'center', display: 'flex', gap: '0.75rem' }}>
          {selected ? <span style={{ fontWeight: 700 }}>{formatMarketPrice(price, props.currency)}</span> : null}
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
        </div>
      </footer>
    </div>
  );
}

/**
 * The dialog: loads a category when it is first shown, holds the search and the pick, and hands a
 * buy or a sale to the sheet, which owns the character. `onBuy` and `onSell` resolve to what to tell
 * the player, or `null` when the sheet's `error` already says why nothing happened.
 */
export function MarketDialog(props: {
  initialCategory: MarketCategory;
  categories?: ReadonlyArray<{ value: MarketCategory; label: string }>;
  loadRows: (category: MarketCategory) => Promise<MarketRow[]>;
  currency: string;
  balance: number;
  loadInventory: () => Promise<HeldEquipmentCopyDto[]>;
  onBuy: (row: MarketRow, bonus: number) => Promise<string | null>;
  onSell: (copy: HeldEquipmentCopyDto) => Promise<string | null>;
  priceMode: PriceMode;
  onPriceModeChange: (mode: PriceMode) => void;
  error: string | null;
  onClose: () => void;
}) {
  const [category, setCategory] = useState(props.initialCategory);
  const [loaded, setLoaded] = useState<Partial<Record<MarketCategory, MarketRow[]>>>({});
  const [loadError, setLoadError] = useState<string | null>(null);
  const [inventory, setInventory] = useState<HeldEquipmentCopyDto[] | null>(null);
  const [search, setSearch] = useState('');
  const [selectedKey, setSelectedKey] = useState<string | null>(null);
  const [bonus, setBonus] = useState(0);
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
      setStatus(await action());
      await reloadInventory();
    } finally {
      inFlight.current = false;
      setBusy(false);
    }
  }

  const searched = filterItemPickerEntries(rows ?? [], search) as MarketRow[];
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
        }}
        categories={props.categories}
        search={search}
        onSearch={setSearch}
        rows={searched}
        loading={!rows && !loadError}
        loadError={loadError}
        selectedKey={selectedKey}
        onSelectRow={(key) => {
          setSelectedKey(key);
          setBonus(0);
        }}
        bonus={bonus}
        onBonus={setBonus}
        currency={props.currency}
        balance={props.balance}
        onBuy={() => {
          const row = searched.find((candidate) => candidate.key === selectedKey);
          if (row) {
            void act(() => props.onBuy(row, bonus));
          }
        }}
        inventory={inventory}
        onSell={(copy) => void act(() => props.onSell(copy))}
        priceMode={props.priceMode}
        onPriceModeChange={props.onPriceModeChange}
        busy={busy}
        status={status}
        error={props.error}
        onClose={props.onClose}
      />
    </div>,
    document.body,
  );
}
