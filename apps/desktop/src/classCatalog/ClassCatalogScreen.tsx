import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type { ClassCatalogEntryDto } from '../boundary/loadClassCatalog';
import { loadClassCatalogRuntime } from './classCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Classes in the same "corpus-natural order" the SD-19 loop uses. */
const CLASS_ORDER = [
  'Barbarian',
  'Bard',
  'Cleric',
  'Druid',
  'Fighter',
  'Monk',
  'Paladin',
  'Ranger',
  'Rogue',
  'Sorcerer',
  'Wizard',
] as const;

const MAX_RENDERED_ROWS = 200;

/**
 * Full class progression catalog browser — every real corpus-grounded row
 * across all 11 CRB classes (207 level rows: BAB and the three base saves),
 * not a per-character sample. Distinct from the Character Sheet, which only
 * shows one character's own class and level.
 */
export function ClassCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<ClassCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [classId, setClassId] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadClassCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown class catalog failure');
      });
  }, []);

  const classCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.classId] = (counts[entry.classId] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries
      .filter((entry) => {
        if (classId !== 'All' && entry.classId !== classId) return false;
        if (needle && !entry.classId.toLowerCase().includes(needle)) return false;
        return true;
      })
      .sort((a, b) => (a.classId === b.classId ? a.level - b.level : a.classId.localeCompare(b.classId)));
  }, [entries, classId, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Class Progression</h2>
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
            Every real corpus-grounded level row the engine knows about — {totalCount} rows across all 11
            CRB classes. Not what any one character has selected.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setClassId('All')}
              style={classButtonStyle(classId === 'All')}
            >
              All ({totalCount})
            </button>
            {CLASS_ORDER.map((cls) => (
              <button
                key={cls}
                type="button"
                onClick={() => setClassId(cls)}
                style={classButtonStyle(classId === cls)}
              >
                {cls} ({classCounts[cls] ?? 0})
              </button>
            ))}
          </div>

          <input
            type="text"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search by class name…"
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
              ? 'No class levels match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching rows — refine your search to narrow further.`
                : `${filtered.length} matching row${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              <div
                key={`${entry.classId}:${entry.level}`}
                style={{
                  borderBottom: '1px solid var(--color-border)',
                  padding: '0.5rem 0',
                }}
              >
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{entry.classId}</span>
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      Level {entry.level}
                    </span>
                  </span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                    BAB +{entry.baseAttackBonus}
                  </span>
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  Fort +{entry.fortSave} · Ref +{entry.refSave} · Will +{entry.willSave}
                </p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

function classButtonStyle(active: boolean): CSSProperties {
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
