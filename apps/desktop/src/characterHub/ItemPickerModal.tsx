import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { filterItemPickerEntries, type ItemPickerEntry } from './itemPickerFilter';

export type { ItemPickerEntry } from './itemPickerFilter';

/**
 * Generic "search input + filtered list + select" picker, shared by the
 * Add Weapon / Add Armor / Add Spell affordances on the character sheet.
 * Patterned after `ThemeBrowserModal`'s portal-based overlay (search on the
 * left/top, a list of results, a confirm action) so it reads as the same
 * app rather than a one-off.
 *
 * Deliberately catalog-agnostic: the caller supplies `loadEntries` (already
 * narrowed server-side — e.g. equipment filtered to the "ArmsArmor"
 * category, or the unfiltered spell catalog) and this component only
 * handles the search-box narrowing, the list, and the select/confirm
 * interaction. See `itemPickerFilter.ts` for the pure mapping/filtering
 * logic this component calls into (unit-tested there, since this repo has
 * no jsdom/testing-library to render the component itself in a test).
 */
export function ItemPickerModal(props: {
  open: boolean;
  title: string;
  searchPlaceholder: string;
  loadEntries: () => Promise<ItemPickerEntry[]>;
  onClose: () => void;
  onSelect: (entry: ItemPickerEntry) => void;
}) {
  const [entries, setEntries] = useState<ItemPickerEntry[] | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [selectedKey, setSelectedKey] = useState<string | null>(null);

  useEffect(() => {
    if (!props.open) {
      return;
    }
    setEntries(null);
    setLoadError(null);
    setSearch('');
    setSelectedKey(null);
    props
      .loadEntries()
      .then(setEntries)
      .catch((cause: unknown) => setLoadError(cause instanceof Error ? cause.message : 'Failed to load the catalog.'));
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open]);

  if (!props.open) {
    return null;
  }

  const filtered = filterItemPickerEntries(entries ?? [], search);
  const selected = filtered.find((entry) => entry.key === selectedKey) ?? null;

  function handleConfirm() {
    if (!selected) {
      return;
    }
    props.onSelect(selected);
    props.onClose();
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{
        alignItems: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.6)',
        display: 'flex',
        inset: 0,
        justifyContent: 'center',
        padding: '2rem',
        position: 'fixed',
        zIndex: 1100,
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label={props.title}
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          height: 'min(560px, 90vh)',
          width: 'min(560px, 94vw)',
        }}
      >
        <header
          style={{
            alignItems: 'center',
            borderBottom: '1px solid var(--color-border)',
            display: 'flex',
            justifyContent: 'space-between',
            padding: '1rem 1.5rem',
          }}
        >
          <h2 style={{ fontSize: '1.1rem', margin: 0 }}>{props.title}</h2>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </header>

        <div style={{ borderBottom: '1px solid var(--color-border)', padding: '0.75rem 1.5rem' }}>
          <input
            type="text"
            autoFocus
            placeholder={props.searchPlaceholder}
            value={search}
            onChange={(event) => setSearch(event.target.value)}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              boxSizing: 'border-box',
              color: 'var(--color-text)',
              padding: '0.5rem 0.6rem',
              width: '100%',
            }}
          />
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.5rem 0 0' }}>
            {entries ? `Showing ${filtered.length} of ${entries.length}` : loadError ? 'Catalog unavailable' : 'Loading…'}
          </p>
        </div>

        <div style={{ flex: 1, overflowY: 'auto' }}>
          {loadError ? <p style={{ color: 'var(--color-error, #c0392b)', fontSize: '0.85rem', padding: '1rem' }}>{loadError}</p> : null}
          {!entries && !loadError ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', padding: '1rem' }}>Loading…</p> : null}
          {entries && filtered.length === 0 ? (
            <p style={{ color: 'var(--color-text-faint)', fontSize: '0.85rem', padding: '1rem', textAlign: 'center' }}>No matches.</p>
          ) : null}
          {filtered.map((entry) => {
            const active = selectedKey === entry.key;
            return (
              <button
                key={entry.key}
                type="button"
                onClick={() => setSelectedKey(entry.key)}
                onDoubleClick={() => {
                  props.onSelect(entry);
                  props.onClose();
                }}
                style={{
                  background: active ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderBottom: '1px solid var(--color-border)',
                  cursor: 'pointer',
                  display: 'block',
                  padding: '0.55rem 0.75rem',
                  textAlign: 'left',
                  width: '100%',
                }}
              >
                <p style={{ color: active ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>{entry.name}</p>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.1rem 0 0' }}>{entry.detail}</p>
              </button>
            );
          })}
        </div>

        <footer style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', justifyContent: 'flex-end', padding: '0.85rem 1.5rem' }}>
          <button
            type="button"
            onClick={props.onClose}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-text)',
              cursor: 'pointer',
              fontSize: '0.85rem',
              fontWeight: 600,
              padding: '0.45rem 0.9rem',
            }}
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleConfirm}
            disabled={!selected}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-on-accent)',
              cursor: selected ? 'pointer' : 'default',
              fontSize: '0.85rem',
              fontWeight: 600,
              opacity: selected ? 1 : 0.6,
              padding: '0.45rem 0.9rem',
            }}
          >
            Add
          </button>
        </footer>
      </div>
    </div>,
    document.body
  );
}
