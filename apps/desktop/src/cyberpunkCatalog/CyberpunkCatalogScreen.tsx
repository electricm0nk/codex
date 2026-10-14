import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import {
  loadCyberpunkCatalog,
  type CyberpunkCatalogResponse,
  type CyberpunkRecord,
} from '../boundary/loadCyberpunkCatalog';
import {
  CYBERPUNK_CATALOG_GROUPS,
  CYBERPUNK_CATALOG_TABS,
  cyberpunkCatalogTab,
  describeCyberpunkCatalog,
  filterCyberpunkRecords,
  recordFields,
  recordName,
} from './cyberpunkCatalogModel';
import { CyberpunkDeckScreen } from './deck/CyberpunkDeckScreen';
import { CyberpunkGmPage } from './gm/CyberpunkGmPage';

/**
 * SD-38 E6.2 -- the Cyberpunk RED catalogs: one tab per player-facing MANIFEST kind, each read
 * from `data/cyberpunk-red/corpus` by `list_cyberpunk_catalog`. Opened from the landing
 * screen's "Browse" links when Cyberpunk Red is the selected rule set.
 */

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

/** Rows rendered at once; the search narrows the rest (the largest kind holds 100). */
export const MAX_RENDERED_RECORDS = 200;

function RecordCard(props: { kind: string; index: number; record: CyberpunkRecord }) {
  const { record } = props;
  const fields = recordFields(record);
  return (
    <li data-cpr-catalog-entry={`${props.kind}:${props.index}`} style={{ ...panel, listStyle: 'none', padding: '0.75rem 1rem' }}>
      <div style={{ alignItems: 'baseline', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', justifyContent: 'space-between' }}>
        <strong>{recordName(record)}</strong>
        {typeof record.source_page === 'string' ? (
          <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>{`p. ${record.source_page.replace(/^p0*/, '')}`}</span>
        ) : null}
      </div>
      {typeof record.text === 'string' && record.text.length > 0 ? (
        <p style={{ margin: '0.5rem 0 0', whiteSpace: 'pre-wrap' }}>{record.text}</p>
      ) : null}
      {fields.length > 0 ? (
        <ul style={{ margin: '0.5rem 0 0', paddingLeft: '1.1rem' }}>
          {fields.map((entry) => (
            <li key={entry.field} style={{ fontSize: '0.85rem' }}>
              <span style={{ fontWeight: 600 }}>{entry.label}:</span> {entry.text}
            </li>
          ))}
        </ul>
      ) : null}
    </li>
  );
}

/** The catalog body for one loaded response (pure: the screen below loads it). */
export function CyberpunkCatalogView(props: { response: CyberpunkCatalogResponse; query: string }) {
  const { response } = props;
  const filtered = useMemo(() => filterCyberpunkRecords(response.records, props.query), [response, props.query]);
  const shown = filtered.slice(0, MAX_RENDERED_RECORDS);
  const indexOf = useMemo(() => new Map(response.records.map((record, index) => [record, index])), [response]);
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
      <p data-cpr-catalog-summary style={{ color: 'var(--color-text-muted)', margin: 0 }}>
        {describeCyberpunkCatalog(cyberpunkCatalogTab(response.kind).title, response.records.length, response.expected, filtered.length)}
      </p>
      {filtered.length > shown.length ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>
          Showing the first {shown.length} matches; search to narrow the list.
        </p>
      ) : null}
      <ul style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', margin: 0, padding: 0 }}>
        {shown.map((record) => (
          <RecordCard key={indexOf.get(record)} kind={response.kind} index={indexOf.get(record) ?? 0} record={record} />
        ))}
      </ul>
    </div>
  );
}

function pill(selected: boolean): CSSProperties {
  return {
    backgroundColor: selected ? 'var(--color-accent)' : 'var(--color-surface-2)',
    border: '1px solid var(--color-border)',
    borderRadius: 999,
    color: selected ? 'var(--color-on-accent)' : 'var(--color-text-secondary)',
    cursor: 'pointer',
    padding: '0.35rem 0.9rem',
  };
}

export function CyberpunkCatalogScreen(props: { initialKind: string; onClose: () => void }) {
  const [kind, setKind] = useState<string>(cyberpunkCatalogTab(props.initialKind).kind);
  const [query, setQuery] = useState('');
  const [response, setResponse] = useState<CyberpunkCatalogResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const active = cyberpunkCatalogTab(kind);

  useEffect(() => {
    let live = true;
    setResponse(null);
    if (active.tool === 'deck') {
      setError(null);
      return undefined;
    }
    setError(null);
    loadCyberpunkCatalog(kind)
      .then((loaded) => {
        if (live) setResponse(loaded);
      })
      .catch((cause: unknown) => {
        if (live) setError(cause instanceof Error ? cause.message : String(cause));
      });
    return () => {
      live = false;
    };
  }, [kind, active.tool]);

  return (
    <section style={{ display: 'flex', flexDirection: 'column', gap: '1rem', marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>Cyberpunk Red — {active.title}</h2>
        <button
          type="button"
          onClick={props.onClose}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Close
        </button>
      </div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        {CYBERPUNK_CATALOG_GROUPS.map((group) => (
          <button
            key={group.id}
            type="button"
            aria-pressed={group.id === active.group}
            onClick={() => setKind((CYBERPUNK_CATALOG_TABS.find((entry) => entry.group === group.id) ?? active).kind)}
            style={pill(group.id === active.group)}
          >
            {group.title}
          </button>
        ))}
      </div>
      <div role="tablist" style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        {CYBERPUNK_CATALOG_TABS.filter((entry) => entry.group === active.group).map((entry) => (
          <button
            key={entry.kind}
            type="button"
            role="tab"
            aria-selected={entry.kind === kind}
            onClick={() => setKind(entry.kind)}
            style={pill(entry.kind === kind)}
          >
            {entry.title}
          </button>
        ))}
      </div>
      {active.tool === 'deck' ? (
        <CyberpunkDeckScreen />
      ) : null}
      {active.tool !== 'deck' ? (
      <label style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem', fontSize: '0.85rem' }}>
        Search by name or text
        <input
          type="search"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          style={{ border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.5rem 0.75rem' }}
        />
      </label>
      ) : null}
      {active.tool === 'deck' ? null : error ? (
        <p role="alert" style={{ color: 'var(--color-danger, #c0392b)', margin: 0 }}>
          {error}
        </p>
      ) : response && active.tool === 'gm' ? (
        <CyberpunkGmPage response={response} query={query} />
      ) : response ? (
        <CyberpunkCatalogView response={response} query={query} />
      ) : (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Loading…</p>
      )}
    </section>
  );
}
