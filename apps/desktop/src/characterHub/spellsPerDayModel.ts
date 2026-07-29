import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';

/**
 * Projects the engine's own `class_spell.*.<total|base>_<spells|extracts>_per_day.*`
 * records into the sheet's spells-per-day rows.
 *
 * This replaces the deleted hardcoded Wizard-only spells-per-day table in
 * `characterProgression.ts` that covered levels 1-9 and no other caster at
 * all, standing in for real `class_spell.*` records the engine computes for
 * every grounded caster (Wizard, Cleric, Druid, Paladin, Ranger, Witch,
 * Arcanist, Warpriest, Bloodrager, Investigator, Shaman, ...).
 *
 * Authors no rules data: it groups records by class and spell level, keeps
 * every count and every word of derivation text as the engine emitted them,
 * and shows only the levels the engine actually grounded.
 */

/**
 * `class_spell.wizard.total_spells_per_day.spell_level_3` and
 * `class_spell.acg.investigator.base_extracts_per_day.extract_level_1`
 * both match. The class token is the segment immediately before the
 * `_per_day` segment, so a book-namespaced id (`acg`, `apg`) resolves to
 * the class rather than the book.
 */
const PER_DAY_ID = /^class_spell\.(?:.+\.)?([a-z_]+)\.(total|base)_(?:spells|extracts)_per_day\.(?:spell|extract)_level_(\d+)$/;

export interface SpellsPerDayRow {
  /** e.g. `'wizard'`. */
  classToken: string;
  /** 0 for cantrips/orisons. */
  spellLevel: number;
  count: number;
  /**
   * `'total'` when this is the full per-day count including the casting
   * stat's bonus spells; `'base'` when only the class table's own column
   * was grounded. Never conflated — a base count is not a total.
   */
  basis: 'total' | 'base';
  /** The engine's corpus-cited derivation, verbatim. */
  detail: string;
}

export interface SpellsPerDaySurface {
  /** Ordered by class token, then spell level. */
  rows: SpellsPerDayRow[];
  isEmpty: boolean;
}

/**
 * Builds the surface, preferring a `total_*` record over a `base_*` record
 * for the same class and spell level: the total is the number a player
 * actually casts, and showing both would read as two conflicting answers.
 */
export function buildSpellsPerDaySurface(
  explanations: readonly ExplanationDto[]
): SpellsPerDaySurface {
  const byKey = new Map<string, SpellsPerDayRow>();

  for (const explanation of explanations) {
    const match = PER_DAY_ID.exec(explanation.id);
    if (match === null) {
      continue;
    }
    const [, classToken, basis, spellLevelRaw] = match;
    const spellLevel = Number(spellLevelRaw);
    const key = `${classToken}:${spellLevel}`;
    const existing = byKey.get(key);
    if (existing !== undefined && existing.basis === 'total' && basis === 'base') {
      continue;
    }
    byKey.set(key, {
      classToken,
      spellLevel,
      count: explanation.value,
      basis: basis as 'total' | 'base',
      detail: explanation.detail,
    });
  }

  const rows = [...byKey.values()].sort(
    (left, right) =>
      left.classToken.localeCompare(right.classToken) || left.spellLevel - right.spellLevel
  );

  return { rows, isEmpty: rows.length === 0 };
}
