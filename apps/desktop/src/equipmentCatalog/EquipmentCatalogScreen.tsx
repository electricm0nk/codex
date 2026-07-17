import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type { EquipmentCatalogEntryDto } from '../boundary/loadEquipmentCatalog';
import { loadEquipmentCatalogRuntime } from './equipmentCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const CATEGORY_LABELS: Record<string, string> = {
  ArmsArmor: 'Arms & Armor',
  General: 'General',
  MagicItems: 'Magic Items',
  Equipmods: 'Equipment Mods',
};

/** Categories in the same "corpus-natural order" the SD-19 loop uses. */
const CATEGORY_ORDER = ['ArmsArmor', 'General', 'MagicItems', 'Equipmods'] as const;

const MAX_RENDERED_ROWS = 200;

/**
 * Full equipment catalog browser — every real corpus record across all 4
 * core-rulebook categories (~2,977 items), not a per-character sample.
 * Distinct from a character sheet's Gear tab, which only shows what one
 * character has equipped.
 */
export function EquipmentCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<EquipmentCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [category, setCategory] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadEquipmentCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown equipment catalog failure');
      });
  }, []);

  const categoryCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.category] = (counts[entry.category] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (category !== 'All' && entry.category !== category) return false;
      if (needle && !entry.name.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, category, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Equipment Catalog</h2>
        <button
          type="button"
          onClick={props.onClose}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>

      {error ? (
        <p style={{ color: 'var(--color-danger, #d33)', margin: 0 }}>{error}</p>
      ) : !entries ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0 }}>Loading catalog…</p>
      ) : (
        <>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 1rem' }}>
            Every real corpus record the engine knows about — {totalCount} items across all 4 core-rulebook
            equipment categories. Not what any one character has equipped.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setCategory('All')}
              style={categoryButtonStyle(category === 'All')}
            >
              All ({totalCount})
            </button>
            {CATEGORY_ORDER.map((cat) => (
              <button
                key={cat}
                type="button"
                onClick={() => setCategory(cat)}
                style={categoryButtonStyle(category === cat)}
              >
                {CATEGORY_LABELS[cat]} ({categoryCounts[cat] ?? 0})
              </button>
            ))}
          </div>

          <input
            type="text"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search by name…"
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 8,
              color: 'var(--color-text)',
              marginBottom: '0.75rem',
              padding: '0.5rem 0.75rem',
              width: '100%',
            }}
          />

          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
            {filtered.length === 0
              ? 'No items match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching items — refine your search to narrow further.`
                : `${filtered.length} matching item${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              <div
                key={`${entry.category}:${entry.key}`}
                style={{
                  alignItems: 'baseline',
                  borderBottom: '1px solid var(--color-border)',
                  display: 'flex',
                  gap: '0.75rem',
                  justifyContent: 'space-between',
                  padding: '0.5rem 0',
                }}
              >
                <span>
                  <span style={{ fontWeight: 700 }}>{entry.name}</span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                    {CATEGORY_LABELS[entry.category] ?? entry.category}
                  </span>
                </span>
                <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                  {entry.costGp === null ? '—' : `${entry.costGp} gp`}
                </span>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

function categoryButtonStyle(active: boolean): CSSProperties {
  return {
    backgroundColor: active ? 'var(--color-accent)' : 'var(--color-surface-2)',
    border: '1px solid var(--color-border)',
    borderRadius: 999,
    color: active ? 'var(--color-on-accent)' : 'var(--color-text)',
    cursor: 'pointer',
    fontSize: '0.8rem',
    padding: '0.4rem 0.9rem',
  };
}
