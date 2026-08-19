import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  DamageDiceSourceDto,
  MonsterAbilityDto,
  MonsterCatalogEntryDto,
  MonsterSpellLikeAbilityDto,
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
 * live corpus rather than from the PF1 size ladder in the abstract — Bestiary
 * 1's records use D, T, S, M and L, and Bonus Bestiary's 14 rows add the first
 * `H` the catalog has ever served (Ant Lion (Mature)). `G` and `C` remain
 * deliberately absent: a label for a code no record carries would be untested
 * text.
 */
export const SIZE_LABELS: Record<string, string> = {
  D: 'Diminutive',
  T: 'Tiny',
  S: 'Small',
  M: 'Medium',
  L: 'Large',
  H: 'Huge',
};

/** The size codes the served roster uses, smallest first. */
export const SIZE_ORDER = ['D', 'T', 'S', 'M', 'L', 'H'] as const;

/**
 * Wire book code -> the book's name. A row that named no book would leave a
 * reader unable to look the creature up, and the catalog now serves six
 * books.
 *
 * `BOTD1`/`BOTD2` are the first codes here wider than two characters -- they
 * are the books' own `SOURCESHORT` tokens, exactly like `B1` and `MC`, and
 * nothing in this map or in `formatBook` assumes a width.
 */
export const BOOK_LABELS: Record<string, string> = {
  B1: 'Bestiary 1',
  BB: 'Bonus Bestiary',
  MC: 'Monster Codex',
  BOTD1: 'Book of the Damned, Volume 1',
  BOTD2: 'Book of the Damned, Volume 2',
  ISWG: 'Inner Sea World Guide',
  B2: 'Bestiary 2',
  B3: 'Bestiary 3',
  B4: 'Bestiary 4',
  ISB: 'Inner Sea Bestiary',
  ISG: 'Inner Sea Gods',
  // The one code here that is not its book's own `SOURCESHORT` (`UP`): the app
  // already serves this book's equipment and feats under `UPSI`, and one book
  // must not carry two codes across two screens. See `monster_catalog.rs`'s
  // `BOOK_UPSI` and `decisions.md §64.2`.
  UPSI: 'Ultimate Psionics',
};

/** `'BB'` -> `'Bonus Bestiary'`; an unmapped code falls through as itself. */
export function formatBook(code: string): string {
  return BOOK_LABELS[code] ?? code;
}

/**
 * The books the response actually contains, named in first-appearance order and
 * joined as prose ("Bestiary 1, Bonus Bestiary and Monster Codex").
 *
 * Derived from the served rows rather than written into the blurb, because the
 * hand-written sentence it replaces said "across Bestiary 1 and Bonus Bestiary"
 * and was already wrong the moment a third book was ingested — a stale sentence
 * on a screen a player reads, which no test pinned. This cannot go stale: a
 * book that stops being served stops being named.
 */
export function formatServedBooks(entries: Pick<MonsterCatalogEntryDto, 'book'>[]): string {
  const seen: string[] = [];
  for (const entry of entries) {
    if (!seen.includes(entry.book)) seen.push(entry.book);
  }
  const names = seen.map(formatBook);
  if (names.length === 0) return 'no book';
  if (names.length === 1) return names[0];
  return `${names.slice(0, -1).join(', ')} and ${names[names.length - 1]}`;
}

/**
 * How each damage-dice provenance reads on screen. Short enough for a chip;
 * the full sentence is the engine's own `groundingNote`, rendered verbatim
 * beneath the attack.
 */
export const DAMAGE_DICE_SOURCE_LABELS: Record<DamageDiceSourceDto, string> = {
  monsterRowToken: 'corpus row',
  corpusCrossReferenceToken: 'corpus cross-reference',
  publishedText: 'grounded from published text',
  notInCorpus: 'no dice in the corpus',
};

/** `'M'` -> `'Medium'`; an unmapped code falls through as itself. */
export function formatSize(code: string): string {
  return SIZE_LABELS[code] ?? code;
}

/**
 * `1` -> `'CR 1'`. Fractional ratings print as fractions the way the book does,
 * so a CR 1/2 record can never render as `CR 0.5`.
 *
 * **`0` is a real corpus value and has its own branch.** Ultimate Psionics'
 * Psicrystal states `CR:0` (`up_races.lst:47`) and is the first such row this
 * catalog serves. Without the guard below it fell into the fraction branch,
 * where `Math.round(1 / 0)` is `Infinity` and the screen read `CR 1/Infinity` —
 * a defect no test caught because for eleven books the value could not occur.
 * SD-29 Epic 5 extend, round 10; `decisions.md §64.3`.
 */
export function formatChallengeRating(rating: number): string {
  if (rating === 0) return 'CR 0';
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

/**
 * `'Bite'` + `'2d6'` -> `'Bite 2d6'`; a `"0"` die expression states itself.
 *
 * A `null` die expression prints the attack's name alone. That is not the same
 * as `"0"` and must never render as it: `"0"` is a real attack that deals no
 * damage, `null` is an attack whose damage the corpus never states.
 */
export function formatNaturalAttack(attack: NaturalAttackDto): string {
  if (attack.damageDice === null) return attack.name;
  return attack.damageDice === '0'
    ? `${attack.name} (no damage)`
    : `${attack.name} ${attack.damageDice}`;
}

/**
 * `'Babble'` + `'SpecialAttack'` + `'Supernatural'` -> `'Babble — Special
 * Attack (Su)'`, the way the book prints it. The delivery abbreviation is the
 * published one; an unmapped delivery falls through as its own word rather than
 * being dropped.
 */
export const ABILITY_FACET_LABELS: Record<string, string> = {
  SpecialAttack: 'Special Attack',
  SpecialQuality: 'Special Quality',
};

export const ABILITY_DELIVERY_LABELS: Record<string, string> = {
  Supernatural: 'Su',
  Extraordinary: 'Ex',
  SpellLike: 'Sp',
};

export function formatAbilityHeading(ability: MonsterAbilityDto): string {
  const facet = ABILITY_FACET_LABELS[ability.facet] ?? ability.facet;
  if (!ability.delivery) return `${ability.name} — ${facet}`;
  const delivery = ABILITY_DELIVERY_LABELS[ability.delivery] ?? ability.delivery;
  return `${ability.name} — ${facet} (${delivery})`;
}

/**
 * One granted spell-like ability, as the player reads it:
 * `'3/day — blade barrier (6th, DC 16 + Cha)'`.
 *
 * **The DC is shown as the formula the corpus states, never as a number.** A
 * monster's ability SCORES are not a corpus-stated fact in this repo
 * (`SD31-E6-F1-002` refused to compute that family), so resolving `16+CHA` to
 * a DC would be a fabrication on a player-facing surface. The spell LEVEL, by
 * contrast, is a genuine derivation from PF1's Spell-Like Abilities universal
 * monster rule (`DC = 10 + spell level + ability modifier`) run backwards over
 * the row's own constant — `derived_evaluator_fixture_check::
 * spell_like_ability_save_dc`, fixtured against the granted spell's own PCGen
 * record in a different file (SD31-W15-MONSTER-SLA-001).
 *
 * Every absent part is simply omitted rather than rendered as a placeholder: a
 * spell that allows no save has no DC clause at all, which is the honest
 * reading of a row that states none.
 */
export function formatSpellLikeAbility(sla: MonsterSpellLikeAbilityDto): string {
  const frequency =
    sla.times === null
      ? null
      : sla.times.toUpperCase() === 'ATWILL'
        ? 'At will'
        : `${sla.times}/${(sla.timeUnit ?? 'day').toLowerCase()}`;
  const parenthetical: string[] = [];
  if (sla.derivedSpellLevel !== null) parenthetical.push(formatSpellLevel(sla.derivedSpellLevel));
  if (sla.saveDcToken !== null) {
    // `15+CHA` -> `DC 15 + Cha`. The token is split rather than reformatted
    // wholesale so an unexpected shape still renders verbatim instead of
    // being silently dropped.
    const [constant, ability] = sla.saveDcToken.split('+');
    parenthetical.push(
      ability === undefined
        ? `DC ${constant}`
        : `DC ${constant} + ${ability.charAt(0) + ability.slice(1).toLowerCase()}`,
    );
  }
  const tail = parenthetical.length === 0 ? '' : ` (${parenthetical.join(', ')})`;
  const spell = `${sla.spell.toLowerCase()}${tail}`;
  return frequency === null ? spell : `${frequency} — ${spell}`;
}

/** `0` -> `'cantrip'`, `1` -> `'1st'`, `2` -> `'2nd'`, `3` -> `'3rd'`, else `'Nth'`. */
export function formatSpellLevel(level: number): string {
  if (level === 0) return 'cantrip';
  const lastTwo = level % 100;
  if (lastTwo >= 11 && lastTwo <= 13) return `${level}th`;
  switch (level % 10) {
    case 1:
      return `${level}st`;
    case 2:
      return `${level}nd`;
    case 3:
      return `${level}rd`;
    default:
      return `${level}th`;
  }
}

/**
 * The whole movement clause from every mode on the row: `'Speed 30 ft., fly 60
 * ft.'`. Falls back to the land-speed-only clause when the row carries no
 * modes at all, which is what every Bestiary 1 record looks like.
 */
export function formatSpeedClause(entry: MonsterCatalogEntryDto): string {
  if (entry.speeds.length === 0) return formatLandSpeedClause(entry.speedFt);
  const walk = entry.speeds.find((speed) => speed.mode === 'Walk');
  const others = entry.speeds.filter((speed) => speed.mode !== 'Walk');
  const head = walk ? `Speed ${walk.feet} ft.` : 'No land speed';
  if (others.length === 0) return head;
  return `${head}, ${others.map((speed) => `${speed.mode.toLowerCase()} ${speed.feet} ft.`).join(', ')}`;
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
            Every real stat block the engine knows about, across {formatServedBooks(entries)} —{' '}
            {totalCount} monsters. Armor Class, hit points and saves are not shown because they are
            not ingested: PCGen derives them at runtime from the creature&rsquo;s hit-dice table
            rather than stating them on its corpus row, so the row prints that table&rsquo;s own
            token instead. Bestiary 1&rsquo;s rows carry the land speed only; the other books&rsquo;
            carry every movement mode and their special abilities.
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
                  {/* The page is appended only when the corpus row states one.
                      Two Bestiary 3 rows carry no `SOURCEPAGE:` token at all
                      (`b3_races.lst:215` and `:265`), and interpolating an
                      empty string left the book name with a dangling trailing
                      space. The ability rows below have always rendered their
                      page conditionally; this brings the monster row into line
                      with them. */}
                  {formatSpeedClause(entry)} · {formatBook(entry.book)}
                  {entry.sourcePage ? ` ${entry.sourcePage}` : ''}
                  {entry.monsterClass ? ` · Hit dice ${entry.monsterClass}` : ''}
                  {/* PF1's Spell-Like Abilities universal monster rule (caster
                      level = Hit Dice). `null` for a monster with no
                      BONUS:VAR|SLA_CL| token at all -- never a bare number
                      with nothing behind it (SD31-E6-F1-002,
                      `OPEN-ISSUES.md` row 44). */}
                  {entry.spellLikeAbilityCasterLevel !== null
                    ? ` · Spell-like abilities CL ${entry.spellLikeAbilityCasterLevel}`
                    : ''}
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
                {entry.abilities.length > 0 ? (
                  <div style={{ margin: '0.4rem 0 0' }}>
                    {entry.abilities.map((ability) => (
                      // `key` is the corpus `KEY:` token, which is namespaced
                      // where the book namespaces it — two creatures' abilities
                      // that share a display name are still distinct rows.
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
                      </div>
                    ))}
                  </div>
                ) : null}
                {entry.spellLikeAbilities.length > 0 ? (
                  <div style={{ margin: '0.4rem 0 0' }}>
                    <span style={{ fontSize: '0.75rem', fontWeight: 600 }}>
                      Spell-Like Abilities
                    </span>
                    {entry.spellLikeAbilities.map((sla) => (
                      // `spell` is the corpus row's own spell name, which is
                      // unique within a row's grants — two grants of the same
                      // spell at different frequencies do not occur in the
                      // registered books.
                      <p
                        key={sla.spell}
                        style={{
                          color: 'var(--color-text-muted)',
                          fontSize: '0.72rem',
                          margin: '0.1rem 0 0',
                        }}
                      >
                        {formatSpellLikeAbility(sla)}
                      </p>
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
