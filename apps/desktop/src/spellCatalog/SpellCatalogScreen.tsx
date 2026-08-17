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
 * Display labels for the wire's short book codes. Every code
 * `build_spell_catalog` can serve needs an entry here: a code with no label
 * reaches the user as a raw wire code, which names nothing a player can
 * recognise.
 */
export const BOOK_LABELS: Record<string, string> = {
  CRB: 'Core Rulebook',
  APG: "Advanced Player's Guide",
  ACG: 'Advanced Class Guide',
  ARG: 'Advanced Race Guide',
  UI: 'Ultimate Intrigue',
  UM: 'Ultimate Magic',
  OA: 'Occult Adventures',
  UC: 'Ultimate Combat',
  ISG: 'Inner Sea Gods',
};

/**
 * The served books in the order `spell_catalog.rs`'s `build_spell_catalog`
 * chains them (CRB -> APG -> ACG -> ARG -> UI -> UM, via
 * `spell_resolver::spell_catalog_rows()`), so the filter row reads in the
 * same order the rows themselves arrive.
 *
 * **Keep this in step with the Rust chain.** UI was served for a full
 * bundle before it appeared here: the screen showed 1286 spells under a
 * filter row whose chips summed to 1185, and named four books in its own
 * copy while serving five. Nothing failed — the frontend test's oracle was
 * a copy of this constant rather than a statement about the backend. See
 * `SpellCatalogScreen.test.ts`'s header. UM (SD31-E6-F2-002), OA
 * (SD31-E6-F2-003), UC (SD31-E6-F2-004) and ISG (SD31-E6-F10-001) are added
 * here deliberately, in the same edit as their respective Rust widenings,
 * to not reproduce that exact defect a second time.
 */
export const BOOK_ORDER = ['CRB', 'APG', 'ACG', 'ARG', 'UI', 'UM', 'OA', 'UC', 'ISG'] as const;

/**
 * Renders book codes as a prose list of their display labels, so the
 * screen's own copy can never name a different set of books than the filter
 * row offers. An unlabelled code falls through as its wire code rather than
 * being dropped — naming it awkwardly is honest; omitting it is not.
 */
export function formatBookList(codes: readonly string[]): string {
  const labels = codes.map((code) => BOOK_LABELS[code] ?? code);
  if (labels.length <= 1) return labels.join('');
  return `${labels.slice(0, -1).join(', ')} and ${labels[labels.length - 1]}`;
}

/**
 * Full spell catalog browser — every real corpus record across all eight
 * ingested books (CRB 652, APG 297, ACG 144, ARG 92, UI 101, UM 269, OA 144,
 * UC 146; 1845 in total), not a per-character sample. Counts are pinned
 * Rust-side by
 * `the_catalog_serves_every_ingested_book_not_only_crb` in
 * `spell_catalog.rs`. Distinct from the Character Sheet's Spells tab, which
 * only shows what one character has selected.
 */
export function SpellCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<SpellCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [school, setSchool] = useState<string | 'All'>('All');
  const [book, setBook] = useState<string | 'All'>('All');
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
      // A record whose corpus row has no `SCHOOL:` token is counted under
      // no school — it is genuinely unknown, not silently bucketed.
      if (entry.school === null) continue;
      counts[entry.school] = (counts[entry.school] ?? 0) + 1;
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

  const schoollessCount = useMemo(
    () => (entries ?? []).filter((entry) => entry.school === null).length,
    [entries]
  );

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (school !== 'All' && entry.school !== school) return false;
      if (book !== 'All' && entry.book !== book) return false;
      if (needle && !entry.key.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, school, book, query]);

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
            Every real corpus record the engine knows about — {totalCount} spells across the{' '}
            {formatBookList(BOOK_ORDER)}. Not what any one character has selected.
            {schoollessCount > 0 ? (
              <>
                {' '}
                {schoollessCount} carry no school in the corpus and appear only under
                &ldquo;All&rdquo;.
              </>
            ) : null}{' '}
            Level shown is each record&rsquo;s lowest class level, not any one class&rsquo;s level
            &mdash; e.g. Hideous Laughter is Bard 1 but Sorcerer/Wizard 2, and lists as Level 1.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button type="button" onClick={() => setBook('All')} style={schoolButtonStyle(book === 'All')}>
              All books ({totalCount})
            </button>
            {BOOK_ORDER.map((code) => (
              <button
                key={code}
                type="button"
                onClick={() => setBook(code)}
                style={schoolButtonStyle(book === code)}
                title={BOOK_LABELS[code]}
              >
                {code} ({bookCounts[code] ?? 0})
              </button>
            ))}
          </div>

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
              // `key` is unique across all five served books (see
              // `tests/spell_cross_book_identity.rs`), so it alone is a
              // safe React key.
              <div
                key={entry.key}
                style={{
                  borderBottom: '1px solid var(--color-border)',
                  padding: '0.5rem 0',
                }}
              >
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{entry.key}</span>
                    <span
                      style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}
                      title={BOOK_LABELS[entry.book] ?? entry.book}
                    >
                      {entry.book}
                    </span>
                    {/* Absent school/level are stated as absent — the corpus
                        genuinely omits them on some APG rows — never
                        defaulted to a plausible-looking value. */}
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      {entry.school ?? 'school not in corpus'}
                    </span>
                  </span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                    {entry.level === null ? 'level not in corpus' : `Level ${entry.level}`}
                  </span>
                </div>
                <p
                  style={{
                    color: entry.description === null ? 'var(--color-text-faint)' : 'var(--color-text-muted)',
                    fontSize: '0.75rem',
                    fontStyle: entry.description === null ? 'italic' : 'normal',
                    margin: '0.25rem 0 0',
                  }}
                >
                  {entry.description ?? 'No description in the corpus for this record.'}
                </p>
                {/* Only rendered when the corpus's own DURATION token
                    parses as a caster-level-linear formula (SD31-E6-F2-006)
                    — most records carry no such line, and nothing is shown
                    in its place for those. */}
                {entry.duration !== null ? (
                  <p
                    style={{
                      color: 'var(--color-text-muted)',
                      fontSize: '0.72rem',
                      margin: '0.15rem 0 0',
                    }}
                  >
                    Duration: {entry.duration}
                  </p>
                ) : null}
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
