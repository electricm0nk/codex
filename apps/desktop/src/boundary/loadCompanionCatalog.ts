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
 * One extra-damage-on-attack statement the creature's row
 * states — extra damage on a named attack.
 *
 * **A rule, not a number.** The dominant corpus formula is `max(0,(STR/2))`,
 * PCGen's encoding of PF1 CRB p.182's "if a creature has only one natural
 * attack, it adds 1-1/2 times its Strength bonus on damage rolls": the base
 * attack applies the full modifier and this token adds the other half, clamped
 * at zero because the rule is stated about a Strength BONUS. A catalog browser
 * has no character and therefore no Strength modifier, so the engine renders
 * the rule in words rather than inventing a total.
 *
 * `attack` is the token's OWN selector and is not guaranteed to name one of
 * `naturalAttacks` — Advanced Player's Guide's Parrot states a Claw damage
 * bonus and has only a Bite. Served as its own list for exactly that reason.
 */
export interface CompanionDamageBonusDto {
  /** The `WEAPONPROF=` selector verbatim: `"Bite"`, `"Claw"`, `"Slam"`, ... */
  attack: string;
  /** The rule in words: `"+1/2 Str modifier (minimum +0)"`, `"+5"`, ... */
  bonus: string;
  /**
   * The formula verbatim for a shape the engine refuses to interpret (`STR/2`
   * — an unclamped halving whose negative-odd rounding PCGen does not state).
   * `null` once `bonus` carries the rendered rule.
   */
  unparsedFormula: string | null;
}

/**
 * One ability-difference skill bonus the creature's row states — a
 * skill-check bonus computed as the DIFFERENCE between two ability
 * modifiers, rather than a flat number.
 *
 * **A rule, not a number.** The dominant (and, corpus-wide, only) formula is
 * `DEX-STR`: familiars and small companions whose Dexterity typically
 * exceeds their Strength get Climb and Swim checks computed from the
 * difference between the two rather than from Strength alone. A catalog
 * browser has no character and therefore no modifiers to subtract, so the
 * engine renders the rule in words rather than inventing a total.
 */
export interface CompanionSkillBonusDto {
  /** Every skill the token names, e.g. `["Climb", "Swim"]`. */
  skills: string[];
  /** The rule in words: `"Dex modifier − Str modifier"`. */
  bonus: string;
  /**
   * The formula verbatim for a shape the engine refuses to interpret.
   * `null` once `bonus` carries the rendered rule.
   */
  unparsedFormula: string | null;
}

/**
 * One companion ability's save DC, stated entirely in a description slot —
 * a base value, plus half the creature's Hit Dice, plus an ability modifier.
 *
 * **A rule, not a number**, same posture `CompanionSkillBonusDto` and
 * `CompanionDamageBonusDto` both take. This is the ONLY place the DC reaches
 * a player at all: the engine's description render drops the slot
 * placeholder from `description` entirely (no formula interpreter), so
 * without this field the DC number is silently missing from the ability's
 * own prose.
 */
export interface CompanionSaveDcDto {
  /** The rule in words: `"10 + 1/2 HD + Con modifier"`. */
  formula: string;
  /**
   * The token's raw argument, for a shape the engine refuses to interpret.
   * `null` once `formula` carries the rendered rule.
   */
  unparsedFormula: string | null;
}

/**
 * One ability-score adjustment.
 *
 * **An adjustment, never a score.** A Griffon's row states
 * a +6 Strength adjustment and a Griffon's Strength is not 6; the upstream engine computes the
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
   * The row's rules text, from the converted record, or `null` when that record states no
   * descriptive prose at all.
   *
   * **A row whose text is stated once per condition carries all of it here**, each variant
   * under the condition that selects it. Until SD-35 `AT-35-E6-003` cycle 8 such a row was
   * `null` here and a separate `descriptionVariants` array carried them, rendered at run time
   * from the ingest format; `decisions.md §11` removed that renderer from the live side and the
   * converter states the whole family instead.
   */
  description: string | null;
  statAdjustments: CompanionStatAdjustmentDto[];
  /**
   * Every save-DC formula this row's stored arguments state, in words. Empty for most rows —
   * see `CompanionSaveDcDto`.
   */
  saveDcFormulas: CompanionSaveDcDto[];
  sourcePage: string | null;
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
  /**
   * Every extra-damage-on-attack statement on the row. Empty for
   * most rows, which is a real corpus state.
   */
  naturalAttackDamageBonuses: CompanionDamageBonusDto[];
  /**
   * Every ability-difference skill bonus on the row. Empty for most
   * rows — see [`CompanionSkillBonusDto`].
   */
  skillAbilityDiffBonuses: CompanionSkillBonusDto[];
  statAdjustments: CompanionStatAdjustmentDto[];
  /** The row's base-typed natural-armour bonus, when it states one. */
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
