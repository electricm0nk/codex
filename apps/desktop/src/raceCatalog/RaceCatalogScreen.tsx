import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type { RaceCatalogEntryDto } from '../boundary/loadRaceCatalog';
import { loadRaceCatalogRuntime } from './raceCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Races in the same "corpus-natural order" the SD-19 loop uses. */
const RACE_ORDER = ['Human', 'Dwarf', 'Elf', 'Gnome', 'HalfElf', 'HalfOrc', 'Halfling'] as const;

const RACE_LABELS: Record<string, string> = {
  Human: 'Human',
  Dwarf: 'Dwarf',
  Elf: 'Elf',
  Gnome: 'Gnome',
  HalfElf: 'Half-Elf',
  HalfOrc: 'Half-Orc',
  Halfling: 'Halfling',
};

const MAX_RENDERED_ROWS = 200;

/**
 * Full race trait catalog browser — every real corpus-grounded trait row
 * across all 7 CRB races (49 trait rows: ability modifiers/bonus, size,
 * speed, senses, and each race's named special traits), not a per-character
 * sample. Distinct from the Character Sheet, which only shows one
 * character's own chosen race.
 */
export function RaceCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<RaceCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [raceId, setRaceId] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadRaceCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown race catalog failure');
      });
  }, []);

  const raceCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.raceId] = (counts[entry.raceId] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries
      .filter((entry) => {
        if (raceId !== 'All' && entry.raceId !== raceId) return false;
        if (needle && !entry.traitName.toLowerCase().includes(needle) && !entry.raceId.toLowerCase().includes(needle)) {
          return false;
        }
        return true;
      })
      .sort((a, b) => (a.raceId === b.raceId ? a.traitName.localeCompare(b.traitName) : a.raceId.localeCompare(b.raceId)));
  }, [entries, raceId, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Race Traits</h2>
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
            Every real corpus-grounded racial trait the engine knows about — {totalCount} trait rows across all 7
            CRB races. Not what any one character has selected.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setRaceId('All')}
              style={raceButtonStyle(raceId === 'All')}
            >
              All ({totalCount})
            </button>
            {RACE_ORDER.map((race) => (
              <button
                key={race}
                type="button"
                onClick={() => setRaceId(race)}
                style={raceButtonStyle(raceId === race)}
              >
                {RACE_LABELS[race]} ({raceCounts[race] ?? 0})
              </button>
            ))}
          </div>

          <input
            type="text"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search by trait or race name…"
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
              ? 'No race traits match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching rows — refine your search to narrow further.`
                : `${filtered.length} matching row${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              <div
                key={`${entry.raceId}:${entry.traitName}`}
                style={{
                  borderBottom: '1px solid var(--color-border)',
                  padding: '0.5rem 0',
                }}
              >
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{entry.traitName}</span>
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      {RACE_LABELS[entry.raceId] ?? entry.raceId}
                    </span>
                  </span>
                  {entry.value !== 0 ? (
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                      {entry.value > 0 ? `+${entry.value}` : entry.value}
                    </span>
                  ) : null}
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  {entry.detail}
                </p>
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

function raceButtonStyle(active: boolean): CSSProperties {
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
