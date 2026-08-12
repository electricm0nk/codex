import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  CompanionAbilityDto,
  CompanionAttackDto,
  CompanionCatalogEntryDto,
  CompanionStatAdjustmentDto,
} from '../boundary/loadCompanionCatalog';
import { loadCompanionCatalogRuntime } from './companionCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const MAX_RENDERED_ROWS = 200;

/**
 * PCGen `SIZE:` codes -> the words a player reads.
 *
 * Derived from the live served roster rather than from the PF1 size ladder in
 * the abstract, the same discipline `MonsterCatalogScreen` states: the
 * registered companions use T, S, M and L. A label for a code no record carries
 * would be untested text.
 */
export const SIZE_LABELS: Record<string, string> = {
  D: 'Diminutive',
  T: 'Tiny',
  S: 'Small',
  M: 'Medium',
  L: 'Large',
  H: 'Huge',
};

/** The size codes a companion row can carry, smallest first. */
export const SIZE_ORDER = ['D', 'T', 'S', 'M', 'L', 'H'] as const;

/**
 * Wire book code -> the book's name. A row that named no book would leave a
 * reader unable to look the creature up.
 */
export const BOOK_LABELS: Record<string, string> = {
  ISC: 'Inner Sea Combat',
  MC: 'Monster Codex',
  ISI: 'Inner Sea Intrigue',
  HA: 'Horror Adventures',
  B5: 'Bestiary 5',
  B6: 'Bestiary 6',
  B2: 'Bestiary 2',
};

/** `'ISC'` -> `'Inner Sea Combat'`; an unmapped code falls through as itself. */
export function formatBook(code: string): string {
  return BOOK_LABELS[code] ?? code;
}

/**
 * The books the response actually contains, named in first-appearance order and
 * joined as prose. Derived from the served rows rather than written into the
 * blurb, so a sentence naming the books cannot go stale when a book is added or
 * stops being served — the exact staleness `MonsterCatalogScreen` was fixed for.
 */
export function formatServedBooks(entries: Pick<CompanionCatalogEntryDto, 'book'>[]): string {
  const seen: string[] = [];
  for (const entry of entries) {
    if (!seen.includes(entry.book)) seen.push(entry.book);
  }
  const names = seen.map(formatBook);
  if (names.length === 0) return 'no book';
  if (names.length === 1) return names[0];
  return `${names.slice(0, -1).join(', ')} and ${names[names.length - 1]}`;
}

/** `'M'` -> `'Medium'`; an unmapped code falls through as itself. */
export function formatSize(code: string | null): string {
  if (code === null) return 'Size not stated';
  return SIZE_LABELS[code] ?? code;
}

/**
 * `Companion (Griffon)` -> `Companion (Griffon)`, unchanged.
 *
 * Kept as a named function rather than inlined because the corpus's own display
 * name is the whole label and this is the one place a future rename would have
 * to happen. Nothing is stripped: `Companion (...)` and `Familiar (...)` are
 * what the book prints, and hiding the wrapper would make a familiar
 * indistinguishable from an animal companion in the row heading.
 */
export function formatName(entry: Pick<CompanionCatalogEntryDto, 'name'>): string {
  return entry.name;
}

/**
 * `'Magical Beast'` + `'Clockwork'` -> `'Magical Beast (Clockwork)'`.
 *
 * `raceSubtype` already arrives as a readable list — the backend joins the
 * corpus's `|` separator so it never reaches a player — so this only parenthesises.
 */
export function formatCreatureType(raceType: string | null, raceSubtype: string | null): string {
  if (raceType === null) return raceSubtype ?? '';
  return raceSubtype ? `${raceType} (${raceSubtype})` : raceType;
}

/**
 * The whole movement clause: `[Walk 30, Fly 40]` -> `'Walk 30 ft., fly 40 ft.'`
 * and `[]` -> `'No movement stated on this row'`.
 *
 * An empty list is a real corpus state and is said in words rather than printed
 * as "0 ft.", which would read as a broken number — the same rule
 * `MonsterCatalogScreen.formatSpeedClause` applies to a monster with no `Walk`
 * pair.
 */
export function formatSpeedClause(entry: Pick<CompanionCatalogEntryDto, 'speeds'>): string {
  if (entry.speeds.length === 0) return 'No movement stated on this row';
  const [first, ...rest] = entry.speeds;
  const head = `${first.mode} ${first.feet} ft.`;
  const tail = rest.map((speed) => `${speed.mode.toLowerCase()} ${speed.feet} ft.`);
  return [head, ...tail].join(', ');
}

/**
 * `0` -> `'reach 0 ft.'`, `5` -> `'reach 5 ft.'`, `null` -> `''`.
 *
 * `0` is a real corpus value on the two Tiny familiars and must render as a
 * value: a screen that showed nothing for it would be stating a different fact
 * from the one the row carries.
 */
export function formatReach(reachFeet: number | null): string {
  if (reachFeet === null) return '';
  return `reach ${reachFeet} ft.`;
}

/**
 * `{ ability: 'STR', amount: 6 }` -> `'STR +6'`; a negative amount keeps its
 * sign (`'INT -6'`). Never rendered without the "adjustments" heading beside
 * it — see [`STAT_ADJUSTMENT_CAPTION`].
 */
export function formatStatAdjustment(adjustment: CompanionStatAdjustmentDto): string {
  const sign = adjustment.amount >= 0 ? '+' : '';
  return `${adjustment.ability} ${sign}${adjustment.amount}`;
}

/**
 * The caption every stat-adjustment block carries.
 *
 * Load-bearing, not decoration: the corpus states `BONUS:STAT|STR|6` and a
 * Griffon's Strength is not 6. Printing these numbers under a heading that said
 * "Ability scores" would be the quieter lie this whole ingest is written to
 * avoid.
 */
export const STAT_ADJUSTMENT_CAPTION = 'Ability score adjustments (corpus BONUS:STAT tokens)';

/** `{ name: 'Bite', damageDice: null }` -> `'Bite'`; with dice -> `'Bite 1d6'`. */
export function formatNaturalAttack(attack: CompanionAttackDto): string {
  return attack.damageDice ? `${attack.name} ${attack.damageDice}` : attack.name;
}

/**
 * The heading one ability row prints: its name, then what the corpus says it
 * is.
 *
 * A row whose `TYPE:` states no facet the chassis models falls back to its
 * verbatim segments rather than an invented label — three Inner Sea Intrigue
 * rows are in that state, and printing "Special Quality" over them would be
 * asserting something the corpus does not.
 */
export function formatAbilityHeading(ability: CompanionAbilityDto): string {
  const descriptors = ability.facet
    ? [ability.facet, ability.delivery].filter((part): part is string => Boolean(part))
    : ability.typeSegments;
  if (descriptors.length === 0) return ability.name;
  return `${ability.name} — ${descriptors.join(' · ')}`;
}

export interface CompanionCatalogScreenProps {
  onClose: () => void;
}

export function CompanionCatalogScreen(props: CompanionCatalogScreenProps) {
  const [entries, setEntries] = useState<CompanionCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [book, setBook] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadCompanionCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown companion catalog failure');
      });
  }, []);

  // Books come from the served rows themselves rather than a static list, for
  // the same reason the blurb's book names do.
  const books = useMemo(() => {
    const seen: string[] = [];
    for (const entry of entries ?? []) {
      if (!seen.includes(entry.book)) seen.push(entry.book);
    }
    return seen;
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (book !== 'All' && entry.book !== book) return false;
      if (needle && !entry.name.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, book, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Companion Catalog</h2>
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
            Every companion and familiar the engine has ingested, across{' '}
            {formatServedBooks(entries)} — {totalCount} creatures. Hit points, Armor Class and saves
            are not shown because they are not ingested: PCGen derives them at runtime from the
            creature&rsquo;s hit-dice table rather than stating them on its corpus row, so the row
            prints that table&rsquo;s own token instead. This is the corpus, not your
            character&rsquo;s pet — the character sheet&rsquo;s Pets tab shows the companion your
            own build computes.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button type="button" onClick={() => setBook('All')} style={chipStyle(book === 'All')}>
              All books ({totalCount})
            </button>
            {books.map((code) => (
              <button
                key={code}
                type="button"
                onClick={() => setBook(code)}
                style={chipStyle(book === code)}
              >
                {formatBook(code)} ({(entries ?? []).filter((entry) => entry.book === code).length})
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
              ? 'No companions match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching companions — refine your search to narrow further.`
                : `${filtered.length} matching companion${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              // `key` is the canonical `<book>:companion:<slug>` identity and is
              // unique across the catalog (pinned Rust-side by
              // `every_served_key_matches_a_corpus_record_file`).
              <div key={entry.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{formatName(entry)}</span>
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      {formatSize(entry.size)} {formatCreatureType(entry.raceType, entry.raceSubtype)}
                    </span>
                  </span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                    {formatBook(entry.book)}
                    {entry.sourcePage ? ` ${entry.sourcePage}` : ''}
                  </span>
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  {formatSpeedClause(entry)}
                  {formatReach(entry.reachFeet) ? ` · ${formatReach(entry.reachFeet)}` : ''}
                  {entry.monsterClass ? ` · Hit dice ${entry.monsterClass}` : ''}
                  {entry.naturalArmor !== null ? ` · Natural armor +${entry.naturalArmor}` : ''}
                </p>
                {entry.statAdjustments.length > 0 ? (
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.25rem 0 0' }}>
                    <span style={{ color: 'var(--color-text-faint)' }}>{STAT_ADJUSTMENT_CAPTION}: </span>
                    {entry.statAdjustments.map(formatStatAdjustment).join(', ')}
                  </p>
                ) : null}
                {entry.naturalAttacks.length === 0 ? (
                  <p
                    style={{
                      color: 'var(--color-text-faint)',
                      fontSize: '0.75rem',
                      fontStyle: 'italic',
                      margin: '0.25rem 0 0',
                    }}
                  >
                    No natural attack on this corpus row.
                  </p>
                ) : (
                  <p style={{ color: 'var(--color-text)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                    Attacks: {entry.naturalAttacks.map(formatNaturalAttack).join(', ')}
                  </p>
                )}
                {entry.abilities.length > 0 ? (
                  <div style={{ margin: '0.4rem 0 0' }}>
                    {entry.abilities.map((ability) => (
                      // `key` is the corpus `KEY:` token, namespaced where the
                      // book namespaces it — Inner Sea Intrigue's two
                      // `Tinkering` rows are distinct rows here.
                      <div key={ability.key} style={{ margin: '0 0 0.35rem' }}>
                        <span style={{ fontSize: '0.75rem', fontWeight: 600 }}>
                          {formatAbilityHeading(ability)}
                        </span>
                        {ability.sourcePage ? (
                          <span style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', marginLeft: '0.4rem' }}>
                            {ability.sourcePage}
                          </span>
                        ) : null}
                        {ability.description ? (
                          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.1rem 0 0' }}>
                            {ability.description}
                          </p>
                        ) : (
                          <p
                            style={{
                              color: 'var(--color-text-faint)',
                              fontSize: '0.7rem',
                              fontStyle: 'italic',
                              margin: '0.1rem 0 0',
                            }}
                          >
                            The corpus row states this ability&rsquo;s name and type but carries no
                            rules text.
                          </p>
                        )}
                        {ability.statAdjustments.length > 0 ? (
                          <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.1rem 0 0' }}>
                            {STAT_ADJUSTMENT_CAPTION}:{' '}
                            {ability.statAdjustments.map(formatStatAdjustment).join(', ')}
                          </p>
                        ) : null}
                      </div>
                    ))}
                  </div>
                ) : null}
                {entry.externalAbilityRefs.length > 0 ? (
                  <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.2rem 0 0' }}>
                    Also has, defined in another book: {entry.externalAbilityRefs.join(', ')}.
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
