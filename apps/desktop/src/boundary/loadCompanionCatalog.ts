import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the ingested companion catalog.
 *
 * Invokes the `list_companion_catalog` Tauri command, which serves every
 * companion or familiar creature in `companion_chassis::COMPANION_BOOKS`, each
 * with the ability records its own book defines for it.
 *
 * Armor Class, hit points and saves are absent because they are not ingested:
 * PCGen computes them at runtime from the `MONSTERCLASS:` hit-dice token this
 * ingest carries verbatim. See `companion_catalog.rs`'s module doc comment.
 *
 * This is NOT the character sheet's Pets tab. That tab shows the *computed*
 * companion of the character in front of you — `pilot_compute`'s two
 * hand-grounded species — and can never show a Griffon or a Clockwork Spy. This
 * catalog is what the corpus contains.
 */

/** `"ISC"` / `"MC"` / `"ISI"` / `"HA"` — see `companion_catalog.rs`. */
export type CompanionBookDto = string;

/** One movement mode from the creature's `MOVE:` token. */
export interface CompanionSpeedDto {
  /** `"Walk"`, `"Fly"`, `"Swim"`, `"Climb"`, ... verbatim. */
  mode: string;
  feet: number;
}

export interface CompanionAttackDto {
  name: string;
  /**
   * The die expression only. `null` means the corpus names the attack and
   * prices it nowhere — the row prints the name alone, never a stand-in.
   */
  damageDice: string | null;
}

/**
 * One `BONUS:STAT` token.
 *
 * **An adjustment, never a score.** A Griffon's row states
 * `BONUS:STAT|STR|6` and a Griffon's Strength is not 6; PCGen computes the
 * actual score at runtime from a base plus this token plus the companion
 * class's own level advance. The screen labels the block accordingly.
 */
export interface CompanionStatAdjustmentDto {
  /** `"STR"`, `"DEX"`, ... the corpus abbreviation verbatim. */
  ability: string;
  amount: number;
}

/** One companion ability record, served attached to the creature that owns it. */
export interface CompanionAbilityDto {
  /** Canonical `<book>:companion:<slug>` key. Unique, so it is safe as a React list key. */
  key: string;
  /** The display name, which is not unique — Inner Sea Intrigue defines `Tinkering` twice. */
  name: string;
  /**
   * `'CompanionAdvancement'` / `'SpecialQuality'` / `'SpecialAttack'`, or
   * `null` for a row whose `TYPE:` states no facet the chassis models. The
   * screen falls back to `typeSegments` there rather than inventing a label.
   */
  facet: string | null;
  /** `'Supernatural'` / `'Extraordinary'` / `'SpellLike'`, or `null`. */
  delivery: string | null;
  /** Every `TYPE:` segment verbatim, so an unmodelled shape is visible. */
  typeSegments: string[];
  /**
   * The row's unconditional rules text, or `null`.
   *
   * `null` has TWO meanings and `descriptionVariants` is what separates them:
   * the corpus row carries no `DESC:` at all, or it carries several, each gated
   * on a different condition, so none of them is the row's unconditional text.
   */
  description: string | null;
  /**
   * The row's conditional rules texts, each with its gate in prose. Empty for
   * the ordinary row; Ultimate Wilderness is the first book to carry any, and
   * 22 of its ability rows do (`decisions.md §61.1`).
   */
  descriptionVariants: CompanionDescriptionVariantDto[];
  statAdjustments: CompanionStatAdjustmentDto[];
  sourcePage: string | null;
}

/** One conditional `DESC:` token of an ability row that states its text more than once. */
export interface CompanionDescriptionVariantDto {
  /** The variant's rules text, rendered by the same renderer as `description`. */
  text: string;
  /**
   * The gate in prose — `'master level 15 or higher'`, `'unconditionally'`.
   * Never empty: a row's single ungated token is served as `description`.
   */
  condition: string;
}

export interface CompanionCatalogEntryDto {
  /** Canonical `<book>:companion:<slug>` key. Unique. */
  key: string;
  book: CompanionBookDto;
  name: string;
  /** A single PCGen size code (`"M"`, `"L"`, `"T"`), or `null`. */
  size: string | null;
  /** Every movement mode on the row; empty is a real state, not a missing one. */
  speeds: CompanionSpeedDto[];
  /**
   * The `REACH:` token in feet. `0` is a real corpus value on the two Tiny
   * familiars and is not the same fact as `null` — render it as "reach 0 ft.",
   * never as "no reach stated".
   */
  reachFeet: number | null;
  raceType: string | null;
  /** `RACESUBTYPE:` as a readable list, never with the corpus's `|` separator. */
  raceSubtype: string | null;
  /**
   * The `MONSTERCLASS:` token (`"Companion:2"`) — what PCGen computes hit
   * points, AC and saves from, served verbatim in place of totals this ingest
   * deliberately does not compute.
   */
  monsterClass: string | null;
  /** Every `TYPE:` segment verbatim; empty for rows carrying no `TYPE:` token. */
  typeSegments: string[];
  naturalAttacks: CompanionAttackDto[];
  statAdjustments: CompanionStatAdjustmentDto[];
  /** `BONUS:VAR|AC_Natural_Armor|n|TYPE=Base`, when the row carries one. */
  naturalArmor: number | null;
  sourcePage: string | null;
  abilities: CompanionAbilityDto[];
  /** Ability names the row cites that its own book does not define. */
  externalAbilityRefs: string[];
}

export interface CompanionCatalogResponse {
  entries: CompanionCatalogEntryDto[];
}

export async function loadCompanionCatalog(): Promise<CompanionCatalogResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the companion catalog');
  }

  try {
    return await invoke<CompanionCatalogResponse>('list_companion_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load companion catalog: ${formatError(cause)}`);
  }
}
