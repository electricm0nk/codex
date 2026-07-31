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
export const CATEGORY_ORDER = ['ArmsArmor', 'General', 'MagicItems', 'Equipmods'] as const;

const MAX_RENDERED_ROWS = 200;

/** Display labels for the wire's short book codes. */
export const BOOK_LABELS: Record<string, string> = {
  CRB: 'Core Rulebook',
  APG: "Advanced Player's Guide",
  ACG: 'Advanced Class Guide',
  B1: 'Bestiary 1',
  ARG: 'Advanced Race Guide',
  PU: 'Pathfinder Unchained',
};

/**
 * The book codes `equipment_catalog.rs` exports as
 * `EQUIPMENT_CATALOG_BOOKS`, in the order `build_equipment_catalog` chains
 * them.
 */
export const BOOK_ORDER = ['CRB', 'APG', 'ACG', 'B1', 'ARG', 'PU'] as const;

const KNOWN_BOOK_CODES: ReadonlySet<string> = new Set<string>(BOOK_ORDER);

/**
 * Renders book codes as a prose list of their real names. A code with no
 * label falls back to the raw code rather than being dropped or given an
 * invented name — a book the adapter starts serving must still be visible
 * here, plainly, until it gets a label.
 */
export function formatBookList(codes: readonly string[]): string {
  const labels = codes.map((code) => BOOK_LABELS[code] ?? code);
  if (labels.length === 0) return '';
  if (labels.length === 1) return labels[0];
  return `${labels.slice(0, -1).join(', ')} and ${labels[labels.length - 1]}`;
}

/**
 * Full equipment catalog browser — every real corpus record across every
 * ingested book, not a per-character sample. The Rust adapter chains CRB,
 * APG, ACG, Bestiary 1, ARG and Pathfinder Unchained into one catalog
 * (3830 records, per `equipment_catalog.rs`'s pinned per-book counts).
 * Distinct from a character sheet's Gear tab, which only shows what one
 * character has equipped.
 *
 * The book and category summaries below are derived from the records that
 * actually loaded rather than hardcoded, so the screen cannot claim a book
 * or a count the data does not back.
 */
export function EquipmentCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<EquipmentCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [category, setCategory] = useState<string | 'All'>('All');
  const [book, setBook] = useState<string | 'All'>('All');
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

  const bookCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.book] = (counts[entry.book] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  /**
   * Books actually present in the loaded records: the known codes in chain
   * order, then any code the adapter served that this screen has no label
   * for. Driving the filter buttons off this list — not off `BOOK_ORDER`
   * alone — is what guarantees no served book is reachable only by
   * scrolling the whole catalog.
   */
  const booksPresent = useMemo(() => {
    const seen = new Set((entries ?? []).map((entry) => entry.book));
    const known = BOOK_ORDER.filter((code) => seen.has(code));
    const unlabelled = [...seen].filter((code) => !KNOWN_BOOK_CODES.has(code)).sort();
    return [...known, ...unlabelled];
  }, [entries]);

  /** Records the corpus gives no flat gp cost for — they render as "—". */
  const costlessCount = useMemo(
    () => (entries ?? []).filter((entry) => entry.costGp === null).length,
    [entries]
  );

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (category !== 'All' && entry.category !== category) return false;
      if (book !== 'All' && entry.book !== book) return false;
      if (needle && !entry.name.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, category, book, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  const summary =
    booksPresent.length === 0
      ? `Every real corpus record the engine knows about — ${totalCount} items. Not what any one character has equipped.`
      : `Every real corpus record the engine knows about — ${totalCount} items across ` +
        `${booksPresent.length} ingested book${booksPresent.length === 1 ? '' : 's'}: ` +
        `${formatBookList(booksPresent)}. Not what any one character has equipped.`;

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
            {summary}
            {costlessCount > 0 ? (
              <>
                {' '}
                {costlessCount} carry no flat gp cost in the corpus and show a dash rather than a
                made-up price.
              </>
            ) : null}
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button type="button" onClick={() => setBook('All')} style={categoryButtonStyle(book === 'All')}>
              All books ({totalCount})
            </button>
            {booksPresent.map((code) => (
              <button
                key={code}
                type="button"
                onClick={() => setBook(code)}
                style={categoryButtonStyle(book === code)}
                title={BOOK_LABELS[code] ?? code}
              >
                {code} ({bookCounts[code] ?? 0})
              </button>
            ))}
          </div>

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
            {visible.map((entry, index) => (
              // 316 keys genuinely repeat within CRB (e.g. `Holy Symbol
              // (Silver)`), so book + category + key is still not unique;
              // the row's position disambiguates the real duplicates rather
              // than hiding them behind a colliding React key.
              <div
                key={`${entry.book}:${entry.category}:${entry.key}:${index}`}
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
                  <span
                    style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}
                    title={BOOK_LABELS[entry.book] ?? entry.book}
                  >
                    {entry.book}
                  </span>
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
