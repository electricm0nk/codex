import { useEffect, useRef, useState, type CSSProperties, type KeyboardEvent } from 'react';
import type { PlacedEquipmentCopyDto } from '../boundary/listHeldEquipment';
import { parseQuantity, quantityPrompt } from './gearScreenModel';
import { formatMarketPrice } from './itemPickerFilter';
import { DEFAULT_SELL_OPTION, SELL_OPTIONS, sellAmountGp, sellAmountText, sellNegotiatedGp, type SellOption } from './sellDialogModel';

const button: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 4,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.75rem',
  padding: '0.25rem 0.6rem',
};
const input: CSSProperties = { fontSize: '0.8rem', padding: '0.15rem 0.3rem', width: '6rem' };
const quiet: CSSProperties = { color: 'var(--color-text-faint)', fontSize: '0.75rem' };
const danger: CSSProperties = { color: 'var(--color-danger, #c0392b)', fontSize: '0.8rem', margin: '0.4rem 0 0' };

export interface SellDialogViewProps {
  label: string;
  held: PlacedEquipmentCopyDto;
  option: SellOption;
  count: string;
  negotiated: string;
  /** The bundle price; `undefined` while it loads, `null` when none is listed. */
  bundleGp: number | null | undefined;
  priceError: string | null;
  error: string | null;
  busy: boolean;
  onOption: (option: SellOption) => void;
  onCount: (text: string) => void;
  onNegotiated: (text: string) => void;
  onSell: () => void;
  onCancel: () => void;
}

/** What the chosen pieces sell for under the window's option; `null` with no price or a bad count. */
function amountOf(props: { held: PlacedEquipmentCopyDto; count: string; bundleGp: number | null | undefined }, option: SellOption): number | null {
  const pieces = quantityPrompt(props.held) ? parseQuantity(props.count, props.held) : props.held.quantity;
  return typeof pieces === 'number' && props.bundleGp != null ? sellAmountGp(props.bundleGp, pieces, props.held.baseQuantity, option) : null;
}

/** The sell window's markup, without its state. */
export function SellDialogView(props: SellDialogViewProps) {
  const prompt = quantityPrompt(props.held);
  const amount = amountOf(props, props.option);
  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        props.onSell();
      }}
    >
      <h2 style={{ fontSize: '1rem', margin: '0 0 0.75rem' }}>{`Sell ${props.label}`}</h2>
      <fieldset style={{ border: 0, margin: 0, padding: 0 }}>
        <legend style={{ fontSize: '0.8rem', marginBottom: '0.25rem' }}>Price</legend>
        {SELL_OPTIONS.map((option) => (
          <label key={option.value} style={{ display: 'block', fontSize: '0.8rem' }}>
            <input type="radio" name="sell-option" data-testid={`sell-option-${option.value}`} checked={props.option === option.value} onChange={() => props.onOption(option.value)} />{' '}
            {option.label}
          </label>
        ))}
      </fieldset>
      {prompt ? (
        <label style={{ display: 'block', fontSize: '0.8rem', marginTop: '0.5rem' }}>
          How many?{' '}
          <input type="number" data-testid="sell-count" min={prompt.min} max={prompt.max} step={1} value={props.count} onChange={(event) => props.onCount(event.target.value)} style={input} />
          {` of ${prompt.max}`}
        </label>
      ) : null}
      {props.option === 'negotiate' ? (
        <label style={{ display: 'block', fontSize: '0.8rem', marginTop: '0.5rem' }}>
          Agreed total (gp){' '}
          <input type="number" data-testid="sell-negotiated" min={0} step="any" value={props.negotiated} onChange={(event) => props.onNegotiated(event.target.value)} style={input} />
        </label>
      ) : (
        <p data-testid="sell-amount" style={{ fontSize: '0.8rem', margin: '0.5rem 0 0' }}>
          {`Returns: ${props.bundleGp === undefined ? 'reading the price…' : amount === null ? '—' : formatMarketPrice(amount, 'gp')}`}
        </p>
      )}
      {props.bundleGp === null ? <p style={{ ...quiet, margin: '0.25rem 0 0' }}>No listed price: Sell removes it and returns nothing.</p> : null}
      {props.priceError ? <p style={{ ...quiet, margin: '0.25rem 0 0' }}>{`Price unavailable: ${props.priceError}`}</p> : null}
      {props.error ? <p role="alert" style={danger}>{props.error}</p> : null}
      <div style={{ display: 'flex', gap: '0.4rem', justifyContent: 'flex-end', marginTop: '0.75rem' }}>
        <button type="submit" data-testid="sell-confirm" disabled={props.busy || props.bundleGp === undefined} style={button}>
          Sell
        </button>
        <button type="button" data-testid="sell-cancel" onClick={props.onCancel} style={button}>
          Cancel
        </button>
      </div>
    </form>
  );
}

export interface SellDialogProps {
  label: string;
  held: PlacedEquipmentCopyDto;
  busy: boolean;
  /** Reads the stack's bundle price (`sellBundleGp`); resolves to `null` when none is listed. */
  loadBundleGp: () => Promise<number | null>;
  /** The existing Sell write: `negotiatedGp` is the gp total the window settled on. */
  onConfirm: (quantity?: number, negotiatedGp?: number) => void;
  onCancel: () => void;
}

/** Sell's window (SD-43 `decisions.md §10`): traps focus while open, Escape cancels, focus returns on close. */
export function SellDialog(props: SellDialogProps) {
  const [option, setOption] = useState<SellOption>(DEFAULT_SELL_OPTION);
  const [count, setCount] = useState(String(props.held.quantity));
  const [negotiated, setNegotiated] = useState('');
  const [bundleGp, setBundleGp] = useState<number | null | undefined>(undefined);
  const [priceError, setPriceError] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const dialog = useRef<HTMLDivElement>(null);
  const { loadBundleGp } = props;

  useEffect(() => {
    const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    dialog.current?.querySelector<HTMLElement>('input:checked')?.focus();
    return () => opener?.focus();
  }, []);
  useEffect(() => {
    let current = true;
    loadBundleGp()
      .then((gp) => current && setBundleGp(gp))
      .catch((cause: unknown) => {
        if (current) {
          setBundleGp(null);
          setPriceError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      current = false;
    };
    // Read once per opening: `loadBundleGp` is a new closure each render of the gear screen.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const chooseOption = (next: SellOption) => {
    setOption(next);
    setError(null);
    if (next === 'negotiate') {
      setNegotiated(sellAmountText(amountOf({ held: props.held, count, bundleGp }, next)));
    }
  };
  const sell = () => {
    const quantity = quantityPrompt(props.held) ? parseQuantity(count, props.held) : undefined;
    if (typeof quantity === 'string') {
      setError(quantity);
      return;
    }
    const gp = sellNegotiatedGp(option, amountOf({ held: props.held, count, bundleGp }, option), negotiated);
    if (typeof gp === 'string') {
      setError(gp);
      return;
    }
    props.onConfirm(quantity, gp);
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Escape') {
      event.preventDefault();
      props.onCancel();
    } else if (event.key === 'Tab' && dialog.current) {
      const focusable = Array.from(dialog.current.querySelectorAll<HTMLElement>('input:not([type="radio"]), input:checked, button:not(:disabled)'));
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (event.shiftKey ? document.activeElement === first : document.activeElement === last) {
        event.preventDefault();
        (event.shiftKey ? last : first)?.focus();
      }
    }
  };
  return (
    <div role="presentation" onClick={props.onCancel} style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}>
      <div
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-label={`Sell ${props.label}`}
        data-testid="gear-sell-dialog"
        onClick={(event) => event.stopPropagation()}
        onKeyDown={onKeyDown}
        style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', maxWidth: '90vw', padding: '1rem', width: '320px' }}
      >
        <SellDialogView
          label={props.label}
          held={props.held}
          option={option}
          count={count}
          negotiated={negotiated}
          bundleGp={bundleGp}
          priceError={priceError}
          error={error}
          busy={props.busy}
          onOption={chooseOption}
          onCount={setCount}
          onNegotiated={setNegotiated}
          onSell={sell}
          onCancel={props.onCancel}
        />
      </div>
    </div>
  );
}
