import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  IntelligentItemComponentDto,
  IntelligentItemMechanicDto,
} from '../boundary/loadIntelligentItemCatalog';
import { loadIntelligentItemCatalogRuntime } from './intelligentItemCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const MAX_RENDERED_ROWS = 200;

/** Corpus book directory -> the name a player reads. */
export const BOOK_LABELS: Record<string, string> = {
  core_rulebook: 'Core Rulebook',
  mythic_adventures: 'Mythic Adventures (Legendary Item)',
};

/** `'core_rulebook'` -> `'Core Rulebook'`; an unmapped directory falls through as itself. */
export function formatBook(book: string): string {
  return BOOK_LABELS[book] ?? book;
}

/**
 * The families in a fixed, rulebook-like reading order — Base first (every
 * intelligent item requires it), then the choices a player layers on top.
 * A family the served rows carry but this list does not name still renders,
 * appended after the named ones, so an unanticipated future corpus shape is
 * never silently dropped from the screen.
 */
const FAMILY_ORDER = [
  'Base',
  'Ability Score',
  'Alignment',
  'Communication',
  'Sense',
  'Power',
  'Purpose',
  'Purpose Power',
  'Movement',
  'Skill Ranks',
  'Spellcasting',
];

function familySortIndex(family: string): number {
  const index = FAMILY_ORDER.indexOf(family);
  return index === -1 ? FAMILY_ORDER.length : index;
}

/** `1000` -> `'1,000 gp'`; `null` -> `'no separate cost stated'`. */
export function formatCost(costGp: number | null): string {
  if (costGp === null) return 'no separate cost stated';
  return `${costGp.toLocaleString('en-US')} gp`;
}

/**
 * One mechanic row: `"Ego +2"`, `"Intelligence +4"`,
 * `"Negative levels while attuned: 1+IntItemNegativeLevel (if wielder's
 * alignment is not Lawful Good)"`. Never a resolved character-specific
 * number — see `intelligent_item_catalog.rs`'s module doc for why.
 */
export function formatMechanic(mechanic: IntelligentItemMechanicDto): string {
  const typeTag = mechanic.bonusType ? ` [${mechanic.bonusType}]` : '';
  const conditionTag = mechanic.condition ? ` (if ${mechanic.condition})` : '';
  // The Base row's price-band formula already reads as a full sentence
  // ("Base Ego from item price (cumulative): ...") — printing "Ego: Base
  // Ego from..." would be a redundant, worse-reading label.
  if (mechanic.formula.startsWith(mechanic.effect) || mechanic.formula.startsWith('Base Ego')) {
    return `${mechanic.formula}${typeTag}${conditionTag}`;
  }
  return `${mechanic.effect} ${mechanic.formula}${typeTag}${conditionTag}`;
}

export interface IntelligentItemCatalogScreenProps {
  onClose: () => void;
}

export function IntelligentItemCatalogScreen(props: IntelligentItemCatalogScreenProps) {
  const [components, setComponents] = useState<IntelligentItemComponentDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [book, setBook] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadIntelligentItemCatalogRuntime()
      .then(setComponents)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown intelligent item catalog failure');
      });
  }, []);

  const books = useMemo(() => {
    const seen: string[] = [];
    for (const component of components ?? []) {
      if (!seen.includes(component.book)) seen.push(component.book);
    }
    return seen;
  }, [components]);

  const filtered = useMemo(() => {
    if (!components) return [];
    const needle = query.trim().toLowerCase();
    return components.filter((component) => {
      if (book !== 'All' && component.book !== book) return false;
      if (needle && !component.name.toLowerCase().includes(needle) && !component.family.toLowerCase().includes(needle)) {
        return false;
      }
      return true;
    });
  }, [components, book, query]);

  const grouped = useMemo(() => {
    const byFamily = new Map<string, IntelligentItemComponentDto[]>();
    for (const component of filtered.slice(0, MAX_RENDERED_ROWS)) {
      const list = byFamily.get(component.family) ?? [];
      list.push(component);
      byFamily.set(component.family, list);
    }
    return [...byFamily.entries()].sort((a, b) => familySortIndex(a[0]) - familySortIndex(b[0]));
  }, [filtered]);

  const totalCount = components?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Intelligent Item Components</h2>
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
      ) : !components ? (
        <p style={{ color: 'var(--color-text-faint)', margin: 0 }}>Loading catalog…</p>
      ) : (
        <>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 1rem' }}>
            Every purchasable ability score, alignment, communication, sense, power and purpose
            option for building an intelligent (or Mythic legendary) magic item — {totalCount}{' '}
            components. An item&rsquo;s total Ego score is never shown as one number here: which
            components a specific item carries is a choice this reference does not make for you, so
            each row states its own literal contribution, and the shared Base row states the full
            price-bracket formula instead of a guess.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button type="button" onClick={() => setBook('All')} style={chipStyle(book === 'All')}>
              All books ({totalCount})
            </button>
            {books.map((code) => (
              <button key={code} type="button" onClick={() => setBook(code)} style={chipStyle(book === code)}>
                {formatBook(code)} ({(components ?? []).filter((component) => component.book === code).length})
              </button>
            ))}
          </div>

          <input
            type="text"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search by name or family…"
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
              ? 'No components match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching components — refine your search to narrow further.`
                : `${filtered.length} matching component${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 520, overflowY: 'auto', padding: grouped.length ? '0.25rem 1rem' : '1rem' }}>
            {grouped.map(([family, rows]) => (
              <div key={family} style={{ margin: '0.5rem 0' }}>
                <h3
                  style={{
                    borderBottom: '1px solid var(--color-border)',
                    color: 'var(--color-text)',
                    fontSize: '0.9rem',
                    margin: '0.5rem 0 0.35rem',
                    paddingBottom: '0.25rem',
                  }}
                >
                  {family}
                </h3>
                {rows.map((component) => (
                  // `key` is the corpus `KEY:` token, unique across the
                  // catalog (pinned Rust-side).
                  <div key={component.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.4rem 0' }}>
                    <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                      <span style={{ fontWeight: 700, fontSize: '0.85rem' }}>{component.name}</span>
                      <span style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', whiteSpace: 'nowrap' }}>
                        {formatBook(component.book)} · {formatCost(component.costGp)}
                      </span>
                    </div>
                    {component.description ? (
                      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.2rem 0 0' }}>
                        {component.description}
                      </p>
                    ) : null}
                    {component.mechanics.length > 0 ? (
                      <ul style={{ color: 'var(--color-text)', fontSize: '0.72rem', margin: '0.25rem 0 0', paddingLeft: '1.1rem' }}>
                        {component.mechanics.map((mechanic) => (
                          <li key={`${component.key}:${mechanic.variable}`}>{formatMechanic(mechanic)}</li>
                        ))}
                      </ul>
                    ) : null}
                  </div>
                ))}
              </div>
            ))}
          </div>
        </>
      )}
    </section>
  );
}

function chipStyle(active: boolean): CSSProperties {
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
