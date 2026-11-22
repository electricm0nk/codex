import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { listEquipment } from '../boundary/listEquipment';
import type { StartingWealthDto } from '../boundary/startingWealth';
import { ItemPickerModal } from './ItemPickerModal';
import { mapEquipmentCatalogEntries } from './itemPickerFilter';
import { PriceModeControl } from './PriceModeControl';
import type { PriceMode } from './priceMode';
import { canAfford, equipmentBudget } from './creationEquipmentModel';

/** One item chosen on the Create screen; `costGp` is the catalog price (`null`: none stated). */
export interface CreationEquipmentItem {
  itemId: string;
  name: string;
  costGp: number | null;
}

/**
 * The Create screen's equipment dialog. The character starts with the class's maximum starting
 * money; items come out of a categorised, searchable catalog and are paid for at the chosen
 * pricing (the choice is session state, never saved on the character). Everything chosen here is
 * bought atomically when the character is created.
 */
export function CreationEquipmentDialog(props: {
  open: boolean;
  wealth: StartingWealthDto | null;
  wealthError: string | null;
  mode: PriceMode;
  onModeChange: (mode: PriceMode) => void;
  chosen: CreationEquipmentItem[];
  onChange: (chosen: CreationEquipmentItem[]) => void;
  onClose: () => void;
}) {
  const [pickerOpen, setPickerOpen] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  // Prices of the rows the picker last listed, so a pick can be priced without a second query.
  const prices = useRef(new Map<string, { name: string; costGp: number | null }>());

  useEffect(() => {
    if (!props.open) {
      return undefined;
    }
    setMessage(null);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !pickerOpen) {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open, pickerOpen]);

  if (!props.open) {
    return null;
  }

  const budgetInput = {
    startingGp: props.wealth?.maxGp ?? null,
    mode: props.mode,
    costsGp: props.chosen.map((item) => item.costGp ?? 0),
    note: props.wealth?.note ?? null,
  };
  const budget = equipmentBudget(budgetInput);

  function handlePick(key: string) {
    const priced = prices.current.get(key);
    if (priced === undefined) {
      setMessage('That item is no longer in the catalog list. Search for it again.');
      return;
    }
    if (!canAfford(budgetInput, priced.costGp)) {
      setMessage(
        priced.costGp === null
          ? `${priced.name} has no catalog price, so it can only be added when pricing is Cashless.`
          : `${priced.name} costs ${priced.costGp} gp and only ${budget.remainingGp} gp is left.`
      );
      return;
    }
    setMessage(null);
    props.onChange([...props.chosen, { itemId: key, name: priced.name, costGp: priced.costGp }]);
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Manage equipment"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          display: 'flex',
          flexDirection: 'column',
          gap: '0.75rem',
          height: 'min(680px, 90vh)',
          padding: '1.1rem 1.4rem',
          width: 'min(720px, 94vw)',
        }}
      >
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Manage equipment</h2>
        <p role="status" style={{ color: budget.over ? 'var(--color-error)' : 'var(--color-accent)', fontSize: '0.95rem', fontWeight: 700, margin: 0 }}>
          {budget.text}
        </p>
        <PriceModeControl value={props.mode} onChange={props.onModeChange} />
        {props.wealthError !== null ? (
          <p style={{ color: 'var(--color-error)', fontSize: '0.8rem', margin: 0 }}>{props.wealthError}</p>
        ) : null}
        <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
          <strong style={{ fontSize: '0.9rem' }}>Chosen ({props.chosen.length})</strong>
          <button type="button" onClick={() => setPickerOpen(true)} style={buttonStyle}>
            Add item…
          </button>
        </div>
        {message !== null ? <p style={{ color: 'var(--color-warn)', fontSize: '0.8rem', margin: 0 }}>{message}</p> : null}
        <ul style={{ flex: 1, listStyle: 'none', margin: 0, overflowY: 'auto', padding: 0 }}>
          {props.chosen.length === 0 ? (
            <li style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>Nothing bought yet.</li>
          ) : (
            props.chosen.map((item, index) => (
              <li key={`${item.itemId}-${index}`} style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', fontSize: '0.85rem', justifyContent: 'space-between', padding: '0.3rem 0' }}>
                <span>
                  {item.name}
                  <span style={{ color: 'var(--color-text-muted)' }}> · {item.costGp === null ? 'no price' : `${item.costGp} gp`}</span>
                </span>
                <button
                  type="button"
                  aria-label={`Remove ${item.name}`}
                  onClick={() => props.onChange(props.chosen.filter((_, position) => position !== index))}
                  style={buttonStyle}
                >
                  Remove
                </button>
              </li>
            ))
          )}
        </ul>
        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <button type="button" onClick={props.onClose} style={{ ...buttonStyle, backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)' }}>
            Done
          </button>
        </div>
      </div>
      <ItemPickerModal
        open={pickerOpen}
        title="Add equipment"
        searchPlaceholder="Search equipment"
        loadEntries={async () => {
          const response = await listEquipment({ nameContains: null, category: null });
          prices.current = new Map(response.entries.map((entry) => [entry.key, { name: entry.name, costGp: entry.costGp }]));
          return mapEquipmentCatalogEntries(response.entries, response.typesError);
        }}
        onClose={() => setPickerOpen(false)}
        onSelect={(entry) => {
          handlePick(entry.key);
          setPickerOpen(false);
        }}
      />
    </div>,
    document.body
  );
}

const buttonStyle = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  fontWeight: 600,
  padding: '0.3rem 0.8rem',
} as const;
