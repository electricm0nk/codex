import { useEffect, useState } from 'react';
import { listCustomEquipmentOptions, type CustomEquipmentOptions, type CustomOption, type SizeCategory } from '../boundary/listCustomEquipmentOptions';
import type { CustomEquipmentBuild } from '../boundary/purchaseCustomEquipment';
import { quoteCustomEquipment, type CustomEquipmentQuote } from '../boundary/quoteCustomEquipment';
import { parseQuantity, quantityTotalLabel } from './marketModel';

/**
 * Buy Custom (SD-41 `decisions.md §24`): a Market row as the base, a custom name, masterwork, `+N`,
 * special abilities, a material and a wielder size. The options are `list_custom_equipment_options`'
 * and the running total is `quote_custom_equipment`'s: nothing here prices a part.
 */

const overlay = { alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.5)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 1200 } as const;
export const subDialog = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 12,
  boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
  display: 'flex',
  flexDirection: 'column',
  gap: '0.6rem',
  maxHeight: '88vh',
  overflowY: 'auto',
  padding: '1rem 1.25rem',
  width: 'min(560px, 92vw)',
} as const;
export const field = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  padding: '0.35rem 0.5rem',
} as const;
export const actionButton = { ...field, cursor: 'pointer', fontWeight: 600 } as const;

export interface BuyCustomDraft {
  name: string;
  masterwork: boolean;
  enhancement: number;
  abilities: string[];
  material: string;
  size: SizeCategory | '';
  quantity: string;
  negotiated: string;
}

export const EMPTY_BUY_CUSTOM: BuyCustomDraft = { name: '', masterwork: false, enhancement: 0, abilities: [], material: '', size: '', quantity: '1', negotiated: '' };

export function buildOf(baseItemId: string, draft: BuyCustomDraft): CustomEquipmentBuild {
  return {
    baseItemId,
    ...(draft.name.trim() ? { name: draft.name.trim() } : {}),
    masterwork: draft.masterwork,
    enhancement: draft.enhancement,
    specialAbilities: draft.abilities,
    ...(draft.material ? { material: draft.material } : {}),
    ...(draft.size ? { size: draft.size } : {}),
  };
}

function optionLabel(option: CustomOption): string {
  const notes = [option.plus > 0 ? `+${option.plus} equivalent` : null, option.unpriced ? 'price: negotiate' : null, option.prerequisite ? `needs ${option.prerequisite}` : null].filter(Boolean);
  return notes.length > 0 ? `${option.name} (${notes.join('; ')})` : option.name;
}

export interface BuyCustomDialogViewProps {
  baseName: string;
  currency: string;
  options: CustomEquipmentOptions | null;
  draft: BuyCustomDraft;
  onDraft: (draft: BuyCustomDraft) => void;
  quote: CustomEquipmentQuote | null;
  /** Why the build cannot be bought (the quote's refusal), or a failed buy. */
  error: string | null;
  busy: boolean;
  onBuy: () => void;
  onClose: () => void;
}

export function BuyCustomDialogView(props: BuyCustomDialogViewProps) {
  const { draft } = props;
  const set = (patch: Partial<BuyCustomDraft>) => props.onDraft({ ...draft, ...patch });
  const unpriced = props.quote !== null && props.quote.priceGp === null;
  const canBuy = props.quote !== null && !props.busy && (!unpriced || draft.negotiated.trim() !== '');
  return (
    <div role="dialog" aria-modal="true" aria-label="Buy Custom" onClick={(event) => event.stopPropagation()} style={subDialog}>
      <h3 style={{ margin: 0 }}>Buy Custom: {props.baseName}</h3>
      <label>
        Name (blank keeps {props.baseName}) <input aria-label="Custom name" value={draft.name} onChange={(event) => set({ name: event.target.value })} style={field} />
      </label>
      <label>
        <input type="checkbox" checked={draft.masterwork} onChange={(event) => set({ masterwork: event.target.checked })} /> Masterwork
      </label>
      <label>
        Enhancement{' '}
        <select aria-label="Enhancement" value={draft.enhancement} onChange={(event) => set({ enhancement: Number(event.target.value) })} style={field}>
          {[0, 1, 2, 3, 4, 5].map((plus) => (
            <option key={plus} value={plus}>
              {plus === 0 ? 'None' : `+${plus}`}
            </option>
          ))}
        </select>
      </label>
      {props.options === null ? <p style={{ margin: 0 }}>Loading the choices…</p> : null}
      {props.options && props.options.abilities.length > 0 ? (
        <fieldset style={{ border: '1px solid var(--color-border)', borderRadius: 6, maxHeight: 180, overflowY: 'auto' }}>
          <legend>Special abilities</legend>
          {props.options.abilities.map((option) => (
            <label key={option.key} style={{ display: 'block' }}>
              <input
                type="checkbox"
                checked={draft.abilities.includes(option.key)}
                onChange={(event) => set({ abilities: event.target.checked ? [...draft.abilities, option.key] : draft.abilities.filter((key) => key !== option.key) })}
              />{' '}
              {optionLabel(option)}
            </label>
          ))}
        </fieldset>
      ) : null}
      {props.options && props.options.materials.length > 0 ? (
        <label>
          Material{' '}
          <select aria-label="Material" value={draft.material} onChange={(event) => set({ material: event.target.value })} style={field}>
            <option value="">None</option>
            {props.options.materials.map((option) => (
              <option key={option.key} value={option.key}>
                {optionLabel(option)}
              </option>
            ))}
          </select>
        </label>
      ) : null}
      {props.options && props.options.sizes.length > 0 ? (
        <label>
          Size{' '}
          <select aria-label="Size" value={draft.size} onChange={(event) => set({ size: event.target.value as SizeCategory | '' })} style={field}>
            <option value="">Medium</option>
            {props.options.sizes.map((size) => (
              <option key={size} value={size}>
                {size}
              </option>
            ))}
          </select>
        </label>
      ) : null}
      <label>
        Quantity <input aria-label="Custom quantity" type="number" min={1} value={draft.quantity} onChange={(event) => set({ quantity: event.target.value })} style={{ ...field, width: '5rem' }} />
      </label>
      <p data-testid="buy-custom-total" style={{ fontWeight: 700, margin: 0 }}>
        {props.quote === null
          ? 'Total: …'
          : unpriced
            ? `price: negotiate (no price for ${props.quote.unpriced.join(', ')})`
            : `Total: ${quantityTotalLabel(props.quote.priceGp, parseQuantity(draft.quantity), props.currency)}`}
        {props.quote?.weightLbs != null ? ` · ${props.quote.weightLbs.toLocaleString('en-US', { maximumFractionDigits: 2 })} lb each` : ''}
      </p>
      <label>
        Negotiate ({props.currency}, total{unpriced ? ', required' : ', blank = the price above'}){' '}
        <input aria-label="Custom negotiated price" type="number" min={0} step="any" value={draft.negotiated} onChange={(event) => set({ negotiated: event.target.value })} style={{ ...field, width: '8rem' }} />
      </label>
      {props.error ? <p role="alert" style={{ color: 'var(--color-error, #c0392b)', margin: 0 }}>{props.error}</p> : null}
      <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'flex-end' }}>
        <button type="button" onClick={props.onClose} style={actionButton}>
          Cancel
        </button>
        <button type="button" disabled={!canBuy} onClick={props.onBuy} style={{ ...actionButton, opacity: canBuy ? 1 : 0.6 }}>
          Buy custom
        </button>
      </div>
    </div>
  );
}

/** The builder: loads the base's choices, re-quotes on every change, and hands the buy to the Market. */
export function BuyCustomDialog(props: {
  baseItemId: string;
  baseName: string;
  currency: string;
  busy: boolean;
  /** Resolves to why the buy failed, or `null` when it went through (the Market then closes this). */
  onBuy: (build: CustomEquipmentBuild, quantity: number, negotiatedGp?: number) => Promise<string | null>;
  onClose: () => void;
}) {
  const [options, setOptions] = useState<CustomEquipmentOptions | null>(null);
  const [draft, setDraft] = useState(EMPTY_BUY_CUSTOM);
  const [quote, setQuote] = useState<CustomEquipmentQuote | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let current = true;
    listCustomEquipmentOptions(props.baseItemId)
      .then((next) => current && setOptions(next))
      .catch((cause: unknown) => current && setError(cause instanceof Error ? cause.message : String(cause)));
    return () => {
      current = false;
    };
  }, [props.baseItemId]);

  const build = buildOf(props.baseItemId, draft);
  const buildKey = JSON.stringify(build);
  useEffect(() => {
    let current = true;
    setQuote(null);
    quoteCustomEquipment(build)
      .then((next) => {
        if (current) {
          setQuote(next);
          setError(null);
        }
      })
      .catch((cause: unknown) => current && setError(cause instanceof Error ? cause.message : String(cause)));
    return () => {
      current = false;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [buildKey]);

  return (
    <div role="presentation" onClick={(event) => {
        // Inside the Market's backdrop: closing this must not close the Market too.
        event.stopPropagation();
        props.onClose();
      }} style={overlay}>
      <BuyCustomDialogView
        baseName={props.baseName}
        currency={props.currency}
        options={options}
        draft={draft}
        onDraft={setDraft}
        quote={quote}
        error={error}
        busy={props.busy}
        onBuy={() => {
          const negotiated = draft.negotiated.trim() === '' ? undefined : Number(draft.negotiated);
          void props.onBuy(build, parseQuantity(draft.quantity), negotiated).then((failure) => failure && setError(failure));
        }}
        onClose={props.onClose}
      />
    </div>
  );
}

