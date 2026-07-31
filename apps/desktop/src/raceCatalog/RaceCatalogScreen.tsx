import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type { RaceCatalogEntryDto } from '../boundary/loadRaceCatalog';
import { loadRaceCatalogRuntime } from './raceCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/**
 * Display labels for `RaceId` variant names whose variant spelling is not
 * the name a player reads. Only the two hyphenated CRB races qualify: every
 * other variant name the adapter emits (`Dwarf`, `Svirfneblin`, `Tengu`, …)
 * is already its own display name.
 *
 * Deliberately an *override* map rather than a roster. The screen used to
 * carry a seven-entry `RACE_ORDER` list and a matching label table, so any
 * race the adapter started serving beyond those seven would have had no
 * filter button at all — the same defect that left 204 of 690 feat rows
 * under raw wire codes in the item picker. Anything not listed here falls
 * back to its raw variant name, which is visible and plain, never invented.
 */
export const RACE_LABEL_OVERRIDES: Record<string, string> = {
  HalfElf: 'Half-Elf',
  HalfOrc: 'Half-Orc',
};

export function raceLabel(raceId: string): string {
  return RACE_LABEL_OVERRIDES[raceId] ?? raceId;
}

export interface RaceFacet {
  raceId: string;
  label: string;
  count: number;
}

/**
 * One filter facet per race the adapter actually served, in the order the
 * rows arrived (`race_traits()` groups its rows by race, so this is the
 * catalog's own corpus-natural order rather than a frontend opinion).
 */
export function deriveRaceFacets(entries: readonly RaceCatalogEntryDto[]): RaceFacet[] {
  const facets: RaceFacet[] = [];
  const byRaceId = new Map<string, RaceFacet>();
  for (const entry of entries) {
    const existing = byRaceId.get(entry.raceId);
    if (existing) {
      existing.count += 1;
      continue;
    }
    const facet: RaceFacet = { raceId: entry.raceId, label: raceLabel(entry.raceId), count: 1 };
    byRaceId.set(entry.raceId, facet);
    facets.push(facet);
  }
  return facets;
}

/**
 * The screen's one-line description of its own contents, derived from the
 * loaded rows. It counts rows and races only. `RaceCatalogEntryDto.book`
 * does carry each row's real sourcebook, so a book breakdown here would be
 * derivable rather than invented — it is simply not built yet, and the
 * sentence deliberately claims nothing about books rather than pinning a
 * roster the way the old "all 7 CRB races" wording did.
 */
export function describeRaceCatalog(entries: readonly RaceCatalogEntryDto[]): string {
  const rowCount = entries.length;
  const raceCount = new Set(entries.map((entry) => entry.raceId)).size;
  const rowWord = rowCount === 1 ? 'trait row' : 'trait rows';
  const raceWord = raceCount === 1 ? 'race' : 'races';
  return `${rowCount} ${rowWord} across ${raceCount} ${raceWord}`;
}

const MAX_RENDERED_ROWS = 200;

/**
 * Full race trait catalog browser — every real corpus-grounded trait row the
 * adapter serves (ability modifiers/bonus, size, speed, senses, and each
 * race's named special traits), not a per-character sample. Distinct from
 * the Character Sheet, which only shows one character's own chosen race.
 *
 * The race list, the per-race counts and the summary line are all derived
 * from the rows that actually loaded, so the screen cannot claim a race or a
 * count the data does not back — the same rule `EquipmentCatalogScreen`
 * already follows for books and categories.
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

  const facets = useMemo(() => deriveRaceFacets(entries ?? []), [entries]);

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
            Every real corpus-grounded racial trait the engine knows about — {describeRaceCatalog(entries)}. Not
            what any one character has selected.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setRaceId('All')}
              style={raceButtonStyle(raceId === 'All')}
            >
              All ({totalCount})
            </button>
            {facets.map((facet) => (
              <button
                key={facet.raceId}
                type="button"
                onClick={() => setRaceId(facet.raceId)}
                style={raceButtonStyle(raceId === facet.raceId)}
              >
                {facet.label} ({facet.count})
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
                      {raceLabel(entry.raceId)}
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
