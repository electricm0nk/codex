import { useState, type CSSProperties } from 'react';
import { createPortal } from 'react-dom';
import { open, save } from '@tauri-apps/plugin-dialog';
import { hasTauriRuntime } from '../boundary/runtime';
import { customItemsFolder, exportCustomItems, importCustomItems } from '../boundary/customItemsExchange';
import type { CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import type { CharacterCustom, CustomRecord } from './customModel';
import { LOCAL_PLACE, exchangePlaces, runExport, runImport, type ExchangeDeps, type ExchangeOutcome } from './customItemsExchangeModel';

const JSON_FILTER = { name: 'Custom items', extensions: ['json'] };

/** Production wiring: the native dialogs and the three commands. */
export const DESKTOP_EXCHANGE_DEPS: ExchangeDeps = {
  hasRuntime: hasTauriRuntime,
  save: (defaultPath) => save({ title: 'Export custom items', defaultPath, filters: [JSON_FILTER] }),
  open: async (defaultPath) => {
    const picked = await open({ title: 'Import custom items', multiple: false, directory: false, defaultPath: defaultPath ?? undefined, filters: [JSON_FILTER] });
    return typeof picked === 'string' ? picked : null;
  },
  folder: customItemsFolder,
  exportItems: exportCustomItems,
  importItems: importCustomItems,
  now: () => new Date().toISOString(),
};

/**
 * Export and import custom items (item 4): tick this character's custom equipment and export it to
 * one file, or import a file of items into this character. An item with the same name as one the
 * character has edits it; a file from another game system is refused.
 */
export function CustomItemsExchangeDialog(props: {
  characterId: string;
  displayLabel: string;
  items: readonly CustomRecord[];
  memberships: readonly CampaignMembershipDto[];
  deps?: ExchangeDeps;
  onImported: (custom: CharacterCustom) => void;
  onClose: () => void;
}) {
  const deps = props.deps ?? DESKTOP_EXCHANGE_DEPS;
  const places = exchangePlaces(props.memberships);
  const [placeKey, setPlaceKey] = useState(LOCAL_PLACE.key);
  const [ticked, setTicked] = useState<string[]>(() => props.items.map((item) => item.id));
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState<{ text: string; error: boolean } | null>(null);
  const place = places.find((entry) => entry.key === placeKey) ?? LOCAL_PLACE;

  async function act(run: () => Promise<ExchangeOutcome>) {
    setBusy(true);
    setStatus(null);
    const outcome = await run();
    setBusy(false);
    if (outcome.kind === 'Imported') {
      props.onImported(outcome.custom);
    }
    if (outcome.kind !== 'Cancelled') {
      setStatus({ text: outcome.message, error: outcome.kind === 'Failed' });
    }
  }

  return createPortal(
    <div role="presentation" onClick={props.onClose} style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}>
      <div role="dialog" aria-modal="true" aria-label="Export and import custom items" onClick={(event) => event.stopPropagation()} style={dialogStyle}>
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Export and import custom items</h2>
        <p style={mutedStyle}>
          One file holds many items and names its game system. Importing an item with the same name as one this character has edits that item; a file from another game system is refused.
        </p>
        <label style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.5rem' }}>
          Where
          <select aria-label="Where" value={placeKey} onChange={(event) => setPlaceKey(event.target.value)} style={inputStyle}>
            {places.map((entry) => (
              <option key={entry.key} value={entry.key}>{entry.label}</option>
            ))}
          </select>
        </label>
        <fieldset aria-label="Items to export" style={{ border: '1px solid var(--color-border)', borderRadius: 6, margin: 0, maxHeight: '18rem', overflowY: 'auto', padding: '0.5rem 0.75rem' }}>
          <legend style={{ fontSize: '0.9rem', fontWeight: 600 }}>Items to export</legend>
          {props.items.length === 0 ? <p style={mutedStyle}>This character has no custom equipment to export. Import still adds items.</p> : null}
          {props.items.map((item) => (
            <label key={item.id} style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.4rem' }}>
              <input
                type="checkbox"
                aria-label={`Export ${item.name}`}
                checked={ticked.includes(item.id)}
                onChange={(event) => setTicked(event.target.checked ? [...ticked, item.id] : ticked.filter((id) => id !== item.id))}
              />
              {item.name}
            </label>
          ))}
        </fieldset>
        {status ? <p role={status.error ? 'alert' : 'status'} style={{ color: status.error ? 'var(--color-error)' : 'var(--color-text)', fontSize: '0.85rem', margin: 0 }}>{status.text}</p> : null}
        <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'flex-end' }}>
          <button type="button" onClick={props.onClose} style={buttonStyle}>Close</button>
          <button type="button" disabled={busy} onClick={() => void act(() => runImport(props.characterId, place, deps))} style={{ ...buttonStyle, opacity: busy ? 0.5 : 1 }}>Import…</button>
          <button
            type="button"
            disabled={busy || ticked.length === 0}
            onClick={() => void act(() => runExport({ characterId: props.characterId, displayLabel: props.displayLabel }, ticked, place, deps))}
            style={{ ...buttonStyle, backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)', opacity: busy || ticked.length === 0 ? 0.5 : 1 }}
          >
            Export…
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

const dialogStyle: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 12,
  display: 'flex',
  flexDirection: 'column',
  gap: '0.7rem',
  padding: '1.1rem 1.4rem',
  width: 'min(560px, 95vw)',
};
const buttonStyle: CSSProperties = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  fontWeight: 600,
  padding: '0.3rem 0.8rem',
};
const inputStyle: CSSProperties = { background: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', fontSize: '0.85rem', padding: '0.3rem 0.5rem' };
const mutedStyle: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.82rem', margin: 0 };
