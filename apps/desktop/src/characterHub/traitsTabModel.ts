import type { CharacterTraitOptionDto } from '../boundary/loadCharacterTraits';

/**
 * Pure presentation behind the sheet's Traits section (v0.8 F-3, scout
 * audit item 32). `LoadSavedCharacterResponse.selectedTraits` carries the
 * persisted trait ids; `list_available_character_traits` carries what each
 * one is. This joins the two. Every bonus written here is copied from the
 * roster's own `bonus` / `skills` / `save` / `otherPillars` /
 * `abilitySubstitution` fields — nothing is computed, and a substitution
 * formula is shown as text, never evaluated.
 */
export interface ResolvedTraitRow {
  id: string;
  /** Roster name, or the raw id when the roster does not carry it. */
  name: string;
  description: string | null;
  /** One-line summary of what the roster says this trait grants, or `null` when unknown. */
  grants: string | null;
}

function signed(value: number): string {
  return value < 0 ? String(value) : `+${value}`;
}

function describeGrant(option: CharacterTraitOptionDto): string | null {
  const parts: string[] = [];
  if (option.abilitySubstitution !== null) {
    const skill = option.skills.join(', ');
    parts.push(`${skill}: ${option.abilitySubstitution.formula}`);
    if (option.abilitySubstitution.flatBonus !== 0) {
      parts.push(`${signed(option.abilitySubstitution.flatBonus)} ${skill}`);
    }
  } else if (option.skills.length > 0) {
    parts.push(`${signed(option.bonus)} ${option.skills.join(', ')}`);
  } else if (option.choiceSetId !== null) {
    // The chosen skill lives in `chosen.trait_skill_choices`, which the
    // load surface does not yet expose — say so rather than guess.
    parts.push(`${signed(option.bonus)} to a chosen skill`);
  }
  if (option.save !== null) {
    parts.push(`${signed(option.bonus)} ${option.save} save`);
  }
  for (const pillar of option.otherPillars) {
    parts.push(`${signed(pillar.bonus)} ${pillar.label}`);
  }
  return parts.length === 0 ? null : parts.join('; ');
}

export function resolveSelectedTraits(
  selectedTraitIds: readonly string[],
  catalog: readonly CharacterTraitOptionDto[],
): ResolvedTraitRow[] {
  return selectedTraitIds.map((id) => {
    const option = catalog.find((candidate) => candidate.id === id);
    if (!option) {
      return { id, name: id, description: null, grants: null };
    }
    return { id, name: option.name, description: option.description, grants: describeGrant(option) };
  });
}
