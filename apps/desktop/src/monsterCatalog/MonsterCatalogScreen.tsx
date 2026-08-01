import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  DamageDiceSourceDto,
  MonsterCatalogEntryDto,
  NaturalAttackDto,
} from '../boundary/loadMonsterCatalog';
import { loadMonsterCatalogRuntime } from './monsterCatalogRuntime';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const MAX_RENDERED_ROWS = 200;

/**
 * PCGen `SIZE:` codes -> the words a player reads.
 *
 * Every code the served roster actually uses needs an entry: an unmapped code
 * reaches the screen as a bare letter, which names nothing. Derived from the
 * live corpus rather than from the PF1 size ladder in the abstract — the 41
 * ingested records use D (2), T (1), S (9), M (22) and L (7). The two unused
 * PF1 codes (H, G, C) are deliberately absent: adding a label for a code no
 * record carries would be untested text.
 */
export const SIZE_LABELS: Record<string, string> = {
  D: 'Diminutive',
  T: 'Tiny',
  S: 'Small',
  M: 'Medium',
  L: 'Large',
};

/** The size codes the served roster uses, smallest first. */
export const SIZE_ORDER = ['D', 'T', 'S', 'M', 'L'] as const;

/**
 * How each damage-dice provenance reads on screen. Short enough for a chip;
 * the full sentence is the engine's own `groundingNote`, rendered verbatim
 * beneath the attack.
 */
export const DAMAGE_DICE_SOURCE_LABELS: Record<DamageDiceSourceDto, string> = {
  monsterRowToken: 'corpus row',
  corpusCrossReferenceToken: 'corpus cross-reference',
  publishedText: 'grounded from published text',
};

/** `'M'` -> `'Medium'`; an unmapped code falls through as itself. */
export function formatSize(code: string): string {
  return SIZE_LABELS[code] ?? code;
}

/**
 * `1` -> `'CR 1'`. Fractional ratings print as fractions the way the book does,
 * so a CR 1/2 record can never render as `CR 0.5`. The current roster is CR
 * 1-3 only; the fraction branch exists because the ingest's own type is `f32`
 * and the sub-CR-1 rows are real Bestiary 1 content the roster may yet take.
 */
export function formatChallengeRating(rating: number): string {
  if (rating >= 1) return `CR ${rating % 1 === 0 ? rating : rating.toFixed(1)}`;
  const denominator = Math.round(1 / rating);
  return `CR 1/${denominator}`;
}

/**
 * The whole speed clause, not just the number: `30` -> `'Speed 30 ft.'`, `0` ->
 * `'No land speed'`.
 *
 * Three records (Shark, Squid, Vargouille) carry no `Walk` pair on their corpus
 * row at all, which is the published "Speed 0 ft." stat line. Printing "0 ft."
 * would read as a broken number and printing nothing would hide a real fact —
 * but so does bolting an absence onto a label written for a value, which is why
 * this returns the clause rather than a fragment the caller prefixes ("Speed no
 * land speed" was the first thing that reached the screen).
 */
export function formatLandSpeedClause(speedFt: number): string {
  return speedFt > 0 ? `Speed ${speedFt} ft.` : 'No land speed';
}

/** `('Humanoid', 'Gnoll')` -> `'Humanoid (Gnoll)'`; a null subtype is simply absent. */
export function formatCreatureType(raceType: string, raceSubtype: string | null): string {
  return raceSubtype ? `${raceType} (${raceSubtype})` : raceType;
}

/** `'Bite'` + `'2d6'` -> `'Bite 2d6'`; a `"0"` die expression states itself. */
export function formatNaturalAttack(attack: NaturalAttackDto): string {
  return attack.damageDice === '0'
    ? `${attack.name} (no damage)`
    : `${attack.name} ${attack.damageDice}`;
}

/**
 * Full Bestiary 1 monster browser — every real corpus stat block the engine
 * knows about (41 records), not an encounter roster and not a character's pet.
 *
 * Counts are pinned Rust-side by
 * `the_catalog_serves_every_ingested_bestiary_1_monster` in
 * `monster_catalog.rs`.
 *
 * **Armor Class, hit points and saves are absent on purpose.** They are not
 * ingested — PCGen computes them at runtime from the `MONSTERCLASS:` hit-dice
 * table, so they are not tokens on a monster's `b1_races.lst` row. The screen
 * says so once, in the header, rather than showing 41 empty columns.
 */
export function MonsterCatalogScreen(props: { onClose: () => void }) {
  const [entries, setEntries] = useState<MonsterCatalogEntryDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [size, setSize] = useState<string | 'All'>('All');
  const [creatureType, setCreatureType] = useState<string | 'All'>('All');
  const [query, setQuery] = useState('');

  useEffect(() => {
    loadMonsterCatalogRuntime()
      .then(setEntries)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown monster catalog failure');
      });
  }, []);

  const sizeCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const entry of entries ?? []) {
      counts[entry.size] = (counts[entry.size] ?? 0) + 1;
    }
    return counts;
  }, [entries]);

  // Creature types come from the served rows themselves rather than a static
  // list: the roster's 12 `RACETYPE:` values are corpus data, and a hand-kept
  // copy here would be one more thing to drift when a subset is added.
  const creatureTypes = useMemo(() => {
    const seen = new Set<string>();
    for (const entry of entries ?? []) seen.add(entry.raceType);
    return [...seen].sort((left, right) => left.localeCompare(right));
  }, [entries]);

  const filtered = useMemo(() => {
    if (!entries) return [];
    const needle = query.trim().toLowerCase();
    return entries.filter((entry) => {
      if (size !== 'All' && entry.size !== size) return false;
      if (creatureType !== 'All' && entry.raceType !== creatureType) return false;
      if (needle && !entry.name.toLowerCase().includes(needle)) return false;
      return true;
    });
  }, [entries, size, creatureType, query]);

  const visible = filtered.slice(0, MAX_RENDERED_ROWS);
  const totalCount = entries?.length ?? 0;

  return (
    <section style={{ marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Monster Catalog</h2>
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
            Every real Bestiary 1 stat block the engine knows about — {totalCount} monsters. Armor
            Class, hit points and saves are not shown because they are not ingested: PCGen derives
            them at runtime from the creature&rsquo;s hit-dice table rather than stating them on its
            corpus row. Speed is the land speed only; swim, climb and fly speeds were out of scope
            for this ingest.
          </p>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button type="button" onClick={() => setSize('All')} style={chipStyle(size === 'All')}>
              All sizes ({totalCount})
            </button>
            {SIZE_ORDER.filter((code) => (sizeCounts[code] ?? 0) > 0).map((code) => (
              <button
                key={code}
                type="button"
                onClick={() => setSize(code)}
                style={chipStyle(size === code)}
                title={`${formatSize(code)} (SIZE:${code})`}
              >
                {formatSize(code)} ({sizeCounts[code] ?? 0})
              </button>
            ))}
          </div>

          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
            <button
              type="button"
              onClick={() => setCreatureType('All')}
              style={chipStyle(creatureType === 'All')}
            >
              All types ({totalCount})
            </button>
            {creatureTypes.map((type) => (
              <button
                key={type}
                type="button"
                onClick={() => setCreatureType(type)}
                style={chipStyle(creatureType === type)}
              >
                {type} ({(entries ?? []).filter((entry) => entry.raceType === type).length})
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
              ? 'No monsters match.'
              : filtered.length > MAX_RENDERED_ROWS
                ? `Showing first ${MAX_RENDERED_ROWS} of ${filtered.length} matching monsters — refine your search to narrow further.`
                : `${filtered.length} matching monster${filtered.length === 1 ? '' : 's'}.`}
          </p>

          <div style={{ ...panel, maxHeight: 480, overflowY: 'auto', padding: visible.length ? '0.25rem 1rem' : '1rem' }}>
            {visible.map((entry) => (
              // `key` is the canonical `beastiary1:monster:<slug>` identity and
              // is unique across the catalog (pinned by
              // `no_key_is_served_twice_so_a_row_is_unambiguous`).
              <div key={entry.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.5rem 0' }}>
                <div style={{ alignItems: 'baseline', display: 'flex', gap: '0.75rem', justifyContent: 'space-between' }}>
                  <span>
                    <span style={{ fontWeight: 700 }}>{entry.name}</span>
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', marginLeft: '0.5rem' }}>
                      {formatSize(entry.size)} {formatCreatureType(entry.raceType, entry.raceSubtype)}
                    </span>
                  </span>
                  <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', whiteSpace: 'nowrap' }}>
                    {formatChallengeRating(entry.challengeRating)}
                  </span>
                </div>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.25rem 0 0' }}>
                  {formatLandSpeedClause(entry.speedFt)} · Bestiary 1 {entry.sourcePage}
                </p>
                {entry.naturalAttacks.length === 0 ? (
                  <p
                    style={{
                      color: 'var(--color-text-faint)',
                      fontSize: '0.75rem',
                      fontStyle: 'italic',
                      margin: '0.25rem 0 0',
                    }}
                  >
                    No natural attack on this corpus row — this creature fights with manufactured
                    weapons.
                  </p>
                ) : (
                  <div style={{ margin: '0.25rem 0 0' }}>
                    {entry.naturalAttacks.map((attack) => (
                      <div key={attack.name} style={{ fontSize: '0.75rem' }}>
                        <span style={{ color: 'var(--color-text)' }}>{formatNaturalAttack(attack)}</span>
                        <span
                          style={{ color: 'var(--color-text-faint)', marginLeft: '0.5rem' }}
                          title={attack.groundingNote ?? undefined}
                        >
                          ({DAMAGE_DICE_SOURCE_LABELS[attack.damageDiceSource]})
                        </span>
                        {attack.groundingNote ? (
                          <p
                            style={{
                              color: 'var(--color-text-faint)',
                              fontSize: '0.7rem',
                              margin: '0.15rem 0 0.25rem',
                            }}
                          >
                            {attack.groundingNote}
                          </p>
                        ) : null}
                      </div>
                    ))}
                  </div>
                )}
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
