import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the full monster catalog.
 *
 * Invokes the `list_monster_catalog` Tauri command, which returns every
 * ingested Bestiary 1 monster stat block verbatim — 41 records, pinned
 * Rust-side by `the_catalog_serves_every_ingested_bestiary_1_monster` in
 * `monster_catalog.rs`. Bestiary 1 is the only ingested book with monster
 * records, so there is no book filter and every row is tagged `"B1"`.
 *
 * Armor Class, hit points and saves are absent because they are not ingested:
 * PCGen computes them at runtime from the `MONSTERCLASS:` hit-dice table, so
 * they are not literal tokens on a monster's `b1_races.lst` row. See
 * `monster_catalog.rs`'s module doc comment.
 */

/** `"B1"` — see `monster_catalog.rs`. */
export type MonsterBookDto = string;

/**
 * Where one attack's damage dice came from.
 *
 * `'monsterRowToken'` — transcribed from a `NATURALATTACKS:` token on the
 * monster's own row.
 *
 * `'corpusCrossReferenceToken'` — read from a real corpus token in another
 * file, reached through this monster's `ABILITY:Internal|AUTOMATIC|<Name>`
 * cross-reference.
 *
 * `'publishedText'` — the corpus carries no dice at any hop, so the value was
 * grounded from the published Bestiary 1 text under `SD-26 decisions.md §11.5`.
 */
export type DamageDiceSourceDto =
  | 'monsterRowToken'
  | 'corpusCrossReferenceToken'
  | 'publishedText'
  | 'notInCorpus';

export interface NaturalAttackDto {
  name: string;
  /**
   * The die expression only, with no Strength modifier. `"0"` is a real attack
   * that deals no damage; `null` means the corpus states no dice at all
   * (`damageDiceSource === 'notInCorpus'`) and the screen prints the attack's
   * name alone rather than a stand-in.
   */
  damageDice: string | null;
  damageDiceSource: DamageDiceSourceDto;
  /**
   * The engine's own provenance sentence for a grounded attack, rendered
   * verbatim. `null` for a plain corpus-row attack, which has nothing extra to
   * say — never a placeholder.
   */
  groundingNote: string | null;
}

/** One movement mode from the row's `MOVE:` token. */
export interface SpeedDto {
  /** `"Walk"`, `"Fly"`, `"Swim"`, ... verbatim. */
  mode: string;
  feet: number;
}

/**
 * One `monster_ability` record, served attached to the monster that owns it —
 * the wire form of `corpus-work-channels.md` §9.2's ruling that a monster
 * ability is to a monster what a race trait is to a race.
 */
export interface MonsterAbilityDto {
  /** The corpus `KEY:` token, namespaced where the book namespaces it. Unique. */
  key: string;
  name: string;
  /** `'SpecialAttack'` or `'SpecialQuality'`, verbatim from the row's `TYPE:`. */
  facet: string;
  /** `'Supernatural'` / `'Extraordinary'` / `'SpellLike'`, or `null`. */
  delivery: string | null;
  /** The row's rules text, or `null` where the corpus carries none. */
  description: string | null;
  sourcePage: string | null;
}

export interface MonsterCatalogEntryDto {
  /** Canonical `<book>:monster:<slug>` key. Unique, so it is safe as a React list key. */
  key: string;
  book: MonsterBookDto;
  name: string;
  challengeRating: number;
  /** A single PCGen size code (`"M"`, `"L"`, ...). */
  size: string;
  /**
   * Land speed in feet. `0` is a real value on three records (Shark, Squid,
   * Vargouille) whose rows carry no `Walk` pair at all — render it as *no land
   * speed*, never as "0 ft".
   */
  speedFt: number;
  raceType: string;
  /** `null` where the corpus row carries no `RACESUBTYPE:` token. */
  raceSubtype: string | null;
  sourcePage: string;
  naturalAttacks: NaturalAttackDto[];
  /**
   * Every movement mode on the row. A Bestiary 1 record carries the single
   * land-speed pair `speedFt` already states (empty when it has none); a Bonus
   * Bestiary record carries its whole `MOVE:` token, which is what keeps a
   * fly-only creature from reading as "no speed".
   */
  speeds: SpeedDto[];
  /** The `MONSTERCLASS:` token, or `null` for a book whose ingest did not capture it. */
  monsterClass: string | null;
  /** Empty for Bestiary 1, whose abilities are not ingested. */
  abilities: MonsterAbilityDto[];
  /** Ability names the row cites that its own book does not define. */
  externalAbilityRefs: string[];
  /**
   * PF1's "Spell-Like Abilities" universal monster rule (caster level = Hit
   * Dice), or `null` when this monster has no `BONUS:VAR|SLA_CL|` token on
   * its row at all (no spell-like abilities to attach a caster level to) —
   * or when it is served by Bestiary 1's SD-22 half, whose ingest does not
   * capture abilities and so cannot honestly answer either way
   * (SD31-E6-F1-002, `OPEN-ISSUES.md` row 44).
   */
  spellLikeAbilityCasterLevel: number | null;
}

export interface MonsterCatalogResponse {
  entries: MonsterCatalogEntryDto[];
}

export async function loadMonsterCatalog(): Promise<MonsterCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the monster catalog');
  }

  try {
    return await invoke<MonsterCatalogResponse>('list_monster_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load monster catalog: ${formatError(cause)}`);
  }
}
