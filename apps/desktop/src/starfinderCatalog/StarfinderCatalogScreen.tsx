import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import {
  loadStarfinderCatalog,
  type StarfinderCatalogEntryDto,
  type StarfinderCatalogKind,
  type StarfinderCatalogResponse,
} from '../boundary/loadStarfinderCatalog';
import {
  bookLabel,
  describeStarfinderCatalog,
  filterStarfinderCatalog,
  STARFINDER_CATALOG_TABS,
  starfinderCatalogTab,
} from './starfinderCatalogModel';

/**
 * SD-37 E6.4 -- the Starfinder 1e catalogs: races, themes, classes, feats,
 * spells and equipment, each read from the converted Starfinder package by
 * `list_starfinder_catalog`. Opened from the landing screen's "Browse" links
 * when Starfinder 1e is the selected rule set.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Rows rendered at once; the search narrows the rest (Equipment holds thousands). */
export const MAX_RENDERED_ENTRIES = 200;

function EntryCard(props: { entry: StarfinderCatalogEntryDto }) {
  const { entry } = props;
  return (
    <li data-sf-catalog-entry={entry.id} style={{ ...panel, listStyle: 'none', padding: '0.75rem 1rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', justifyContent: 'space-between' }}>
        <strong>{entry.name}</strong>
        <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>{bookLabel(entry.book)}</span>
      </div>
      {entry.tags.length > 0 ? (
        <div style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', marginTop: '0.2rem' }}>{entry.tags.join(' · ')}</div>
      ) : null}
      {entry.description ? <p style={{ margin: '0.5rem 0 0', whiteSpace: 'pre-wrap' }}>{entry.description}</p> : null}
      {entry.rows.length > 0 ? (
        <ul style={{ margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
          {entry.rows.map((row) => (
            <li key={row.id} data-sf-catalog-row={row.id} style={{ fontSize: '0.85rem' }}>
              <span style={{ fontWeight: 600 }}>{row.label}:</span> {row.summary}
            </li>
          ))}
        </ul>
      ) : null}
    </li>
  );
}

/** The catalog body for one loaded response (pure: the screen below loads it). */
export function StarfinderCatalogView(props: { response: StarfinderCatalogResponse; query: string }) {
  const { response } = props;
  const filtered = useMemo(() => filterStarfinderCatalog(response.entries, props.query), [response, props.query]);
  const shown = filtered.slice(0, MAX_RENDERED_ENTRIES);
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
      <p data-sf-catalog-summary style={{ color: 'var(--color-text-muted)', margin: 0 }}>
        {describeStarfinderCatalog(response.kind, response.entries, filtered.length)}
      </p>
      {filtered.length > shown.length ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>
          Showing the first {shown.length} matches; search to narrow the list.
        </p>
      ) : null}
      <ul style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', margin: 0, padding: 0 }}>
        {shown.map((entry) => (
          <EntryCard key={entry.id} entry={entry} />
        ))}
      </ul>
    </div>
  );
}

export function StarfinderCatalogScreen(props: { initialKind: StarfinderCatalogKind; onClose: () => void }) {
  const [kind, setKind] = useState<StarfinderCatalogKind>(props.initialKind);
  const [query, setQuery] = useState('');
  const [response, setResponse] = useState<StarfinderCatalogResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let live = true;
    setResponse(null);
    setError(null);
    loadStarfinderCatalog(kind)
      .then((loaded) => {
        if (live) setResponse(loaded);
      })
      .catch((cause: unknown) => {
        if (live) setError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      live = false;
    };
  }, [kind]);

  return (
    <section style={{ display: 'flex', flexDirection: 'column', gap: '1rem', marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>Starfinder 1e — {starfinderCatalogTab(kind).title}</h2>
        <button
          type="button"
          onClick={props.onClose}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Close
        </button>
      </div>
      <div role="tablist" style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        {STARFINDER_CATALOG_TABS.map((tab) => (
          <button
            key={tab.kind}
            type="button"
            role="tab"
            aria-selected={tab.kind === kind}
            onClick={() => setKind(tab.kind)}
            style={{
              backgroundColor: tab.kind === kind ? 'var(--color-accent)' : 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 999,
              color: tab.kind === kind ? 'var(--color-on-accent)' : 'var(--color-text-secondary)',
              cursor: 'pointer',
              padding: '0.35rem 0.9rem',
            }}
          >
            {tab.title}
          </button>
        ))}
      </div>
      <label style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem', fontSize: '0.85rem' }}>
        Search by name, book, tag or text
        <input
          type="search"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          style={{ border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.5rem 0.75rem' }}
        />
      </label>
      {error ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', margin: 0 }}>
          {error}
        </p>
      ) : response ? (
        <StarfinderCatalogView response={response} query={query} />
      ) : (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Loading…</p>
      )}
    </section>
  );
}
