import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import { loadSpellCatalogRuntime } from './spellCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Schools in the same "corpus-natural order" the SD-19 loop uses. */
const SCHOOL_ORDER = [
  'Abjuration',
  'Conjuration',
  'Divination',
  'Enchantment',
  'Evocation',
  'Illusion',
  'Necromancy',
  'Transmutation',
  'Universal',
] as const;

const MAX_RENDERED_ROWS = 200;

/**
 * Full spell catalog browser — every real corpus record across all 9 PF1
 * strict schools (652 spells), not a per-character sample. Distinct from
 * the Character Sheet's Spells tab, which only shows what one character
 * has selected.
 */
export function SpellCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<SpellCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [school, setSchool] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadSpellCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown spell catalog failure');
      });
  }, []);

  const schoolCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.school] = (counts[entry.school] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (school !== 'All' && entry.school !== school) return false;
      if (needle && !entry.key.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, school, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Spell Catalog</h2>
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
            Every real corpus record the engine knows about — {totalCount} spells across all 9 PF1 strict
            schools. Not what any one character has selected.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setSchool('All')}
              style={schoolButtonStyle(school === 'All')}
            >
              All ({totalCount})
            </button>
            {SCHOOL_ORDER.map((sch) => (
              <button
                key={sch}
                type="button"
                onClick={() => setSchool(sch)}
                style={schoolButtonStyle(school === sch)}
              >
                {sch} ({schoolCounts[sch] ?? 0})
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
              ? 'No spells match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching spells — refine your search to narrow further.`
                : `${filtered.length} matching spell${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              <div
                key={`${entry.school}:${entry.key}`}
                style={{
                  borderBottom: '1px solid var(--color-border)',
                  padding: '0.5rem 0',
                }}
              >
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{entry.key}</span>
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      {entry.school}
                    </span>
                  </span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                    Level {entry.level}
                  </span>
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  {entry.description}
                </p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

function schoolButtonStyle(active: boolean): CSSProperties {
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
