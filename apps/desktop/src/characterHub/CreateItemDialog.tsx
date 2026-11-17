import { useState } from 'react';
import type { BodySlotDto } from '../boundary/listHeldEquipment';
import { actionButton, field, subDialog } from './BuyCustomDialog';
import { WORN_SLOTS } from './gearScreenModel';

/**
 * The Market's Create (SD-41 `decisions.md §25`): an item with no corpus base. Name, description and
 * weight are free; the price starts at 0 and above 0 is paid like a purchase; Container takes a capacity
 * in lb; the slot is optional. `create_custom_item` writes it; edit and remove live in the Custom tab.
 */

export interface CreateItemDraft {
  name: string;
  description: string;
  weight: string;
  price: string;
  container: boolean;
  capacity: string;
  slot: BodySlotDto;
}

export const EMPTY_CREATE_ITEM: CreateItemDraft = { name: '', description: '', weight: '', price: '0', container: false, capacity: '', slot: 'none' };

/** What `create_custom_item` takes from the draft; a blank or unreadable number is `null`. */
export interface CreateItemFields {
  name: string;
  description: string;
  weightLbs: number | null;
  priceGp: number | null;
  containerCapacityLbs: number | null;
  slot: BodySlotDto | null;
}

function amount(text: string): number | null {
  const value = Number(text);
  return text.trim() === '' || !Number.isFinite(value) || value < 0 ? null : value;
}

export function createItemFields(draft: CreateItemDraft): CreateItemFields {
  return {
    name: draft.name.trim(),
    description: draft.description.trim(),
    weightLbs: amount(draft.weight),
    priceGp: amount(draft.price),
    containerCapacityLbs: draft.container ? amount(draft.capacity) : null,
    slot: draft.slot === 'none' ? null : draft.slot,
  };
}

export function CreateItemDialogView(props: {
  currency: string;
  draft: CreateItemDraft;
  onDraft: (draft: CreateItemDraft) => void;
  error: string | null;
  busy: boolean;
  onCreate: () => void;
  onClose: () => void;
}) {
  const { draft } = props;
  const set = (patch: Partial<CreateItemDraft>) => props.onDraft({ ...draft, ...patch });
  const canCreate = draft.name.trim() !== '' && !props.busy && (!draft.container || amount(draft.capacity) !== null);
  return (
    <div role="dialog" aria-modal="true" aria-label="Create item" onClick={(event) => event.stopPropagation()} style={subDialog}>
      <h3 style={{ margin: 0 }}>Create an item</h3>
      <label>
        Name <input aria-label="Item name" value={draft.name} onChange={(event) => set({ name: event.target.value })} style={field} />
      </label>
      <label style={{ display: 'flex', flexDirection: 'column' }}>
        Description <textarea aria-label="Item description" rows={3} value={draft.description} onChange={(event) => set({ description: event.target.value })} style={field} />
      </label>
      <label>
        Weight (lb) <input aria-label="Item weight" type="number" min={0} step="any" value={draft.weight} onChange={(event) => set({ weight: event.target.value })} style={{ ...field, width: '6rem' }} />
      </label>
      <label>
        Price ({props.currency}) <input aria-label="Item price" type="number" min={0} step="any" value={draft.price} onChange={(event) => set({ price: event.target.value })} style={{ ...field, width: '8rem' }} />
      </label>
      <label>
        <input type="checkbox" checked={draft.container} onChange={(event) => set({ container: event.target.checked })} /> Container
      </label>
      {draft.container ? (
        <label>
          Holds (lb) <input aria-label="Container capacity" type="number" min={0} step="any" value={draft.capacity} onChange={(event) => set({ capacity: event.target.value })} style={{ ...field, width: '6rem' }} />
        </label>
      ) : null}
      <label>
        Slot{' '}
        <select aria-label="Item slot" value={draft.slot} onChange={(event) => set({ slot: event.target.value as BodySlotDto })} style={field}>
          {WORN_SLOTS.map(({ slot, label }) => (
            <option key={slot} value={slot}>
              {label}
            </option>
          ))}
        </select>
      </label>
      {props.error ? <p role="alert" style={{ color: 'var(--color-error, #c0392b)', margin: 0 }}>{props.error}</p> : null}
      <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'flex-end' }}>
        <button type="button" onClick={props.onClose} style={actionButton}>
          Cancel
        </button>
        <button type="button" disabled={!canCreate} onClick={props.onCreate} style={{ ...actionButton, opacity: canCreate ? 1 : 0.6 }}>
          Create item
        </button>
      </div>
    </div>
  );
}

/** The dialog with its draft; `onCreate` resolves to why it failed, or `null` (the Market then closes this). */
export function CreateItemDialog(props: { currency: string; busy: boolean; onCreate: (fields: CreateItemFields) => Promise<string | null>; onClose: () => void }) {
  const [draft, setDraft] = useState(EMPTY_CREATE_ITEM);
  const [error, setError] = useState<string | null>(null);
  return (
    <div
      role="presentation"
      onClick={(event) => {
        // Inside the Market's backdrop: closing this must not close the Market too.
        event.stopPropagation();
        props.onClose();
      }}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.5)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 1200 }}
    >
      <CreateItemDialogView
        currency={props.currency}
        draft={draft}
        onDraft={setDraft}
        error={error}
        busy={props.busy}
        onCreate={() => void props.onCreate(createItemFields(draft)).then((failure) => failure && setError(failure))}
        onClose={props.onClose}
      />
    </div>
  );
}
