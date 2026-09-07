import type { AbilityScoresDto, CreateCharacterRequest, TraitSkillChoiceDto } from '../boundary/loadCreateCharacter';
import type { AbilityKey } from './characterHubModel';

/**
 * Bakes a race's fixed ability-score adjustments into the raw entered/rolled
 * scores before submission. The compute engine's contract (see
 * `apply_human_ability_bonus`'s doc comment in `pilot_compute.rs`) is that
 * every non-Human race's submitted score already includes its racial
 * adjustment — Human is the one exception, whose raw score IS the pre-bonus
 * base, with the player's floating +2 choice applied server-side via
 * `abilityBonusTarget` instead. Human's `abilityAdjustments` is empty, so
 * this is a no-op for Human and doesn't need a special case here.
 */
export function applyRacialAbilityAdjustments(
  rawScores: AbilityScoresDto,
  raceAdjustments: Partial<Record<AbilityKey, number>>
): AbilityScoresDto {
  return {
    strength: rawScores.strength + (raceAdjustments.strength ?? 0),
    dexterity: rawScores.dexterity + (raceAdjustments.dexterity ?? 0),
    constitution: rawScores.constitution + (raceAdjustments.constitution ?? 0),
    intelligence: rawScores.intelligence + (raceAdjustments.intelligence ?? 0),
    wisdom: rawScores.wisdom + (raceAdjustments.wisdom ?? 0),
    charisma: rawScores.charisma + (raceAdjustments.charisma ?? 0),
  };
}

/**
 * The races whose freely-distributed "+2 to one ability score" the *backend*
 * applies, from `CreateCharacterRequest.abilityBonusTarget`.
 *
 * Human alone, and by construction rather than by choice:
 * `compose_character_input` (`pf1_adapter.rs`) pushes the
 * `choice:human_ability_bonus` slot only when `race_id == "race:human"`, and
 * `apply_human_ability_bonus` (`pilot_compute.rs`) returns every other race's
 * scores untouched. Both are outside this cycle's write scope
 * (`decisions.md §8` forbids `pilot_compute.rs`), so this list records where
 * the seam actually is rather than where it ought to be.
 */
export const SERVER_APPLIED_FLOATING_BONUS_RACE_IDS = ['race:human'];

/**
 * Bakes the player's distribution of a race's floating "+2 to one ability
 * score" into the submitted scores, for the races the backend does not apply
 * it for.
 *
 * PF1 gives Human, Half-Elf and Half-Orc this pool. The form has always let
 * the player distribute it and always shown the result in the calculated
 * score, but only Human's ever reached the engine — so a Half-Elf or Half-Orc
 * was created two points short of what the form displayed, silently. This is
 * the same shape, and the same fix, as
 * `applyRacialAbilityAdjustments` (whose own doc comment records the
 * identical defect for *fixed* adjustments).
 *
 * A race with no floating pool allocates nothing, so this is a no-op for it.
 */
export function applyFloatingAbilityAllocation(
  scores: AbilityScoresDto,
  allocation: Record<AbilityKey, number>,
  raceId: string
): AbilityScoresDto {
  if (SERVER_APPLIED_FLOATING_BONUS_RACE_IDS.includes(raceId)) {
    return { ...scores };
  }
  return {
    strength: scores.strength + allocation.strength,
    dexterity: scores.dexterity + allocation.dexterity,
    constitution: scores.constitution + allocation.constitution,
    intelligence: scores.intelligence + allocation.intelligence,
    wisdom: scores.wisdom + allocation.wisdom,
    charisma: scores.charisma + allocation.charisma,
  };
}

export interface CreateCharacterFormFields {
  displayLabel: string;
  raceId: string;
  classId: string;
  level: number;
  abilityScores: AbilityScoresDto;
  abilityBonusTarget: string;
  /**
   * ARG alternate racial trait corpus keys the player chose for this race.
   * Optional so every existing caller and test composes unchanged; an absent
   * field means "took none", which is a real and common answer rather than a
   * placeholder.
   */
  selectedAlternateTraitKeys?: readonly string[];
  /**
   * Character trait/drawback ids the player picked
   * (`"trait:trait_acrobat"`, from `loadCharacterTraits`). Optional so
   * every existing caller and test composes unchanged; an absent field
   * means "took none", the real and common answer for a form the picker
   * has not been wired into yet.
   */
  selectedTraits?: readonly string[];
  /**
   * The player's resolved skill choice for each *fixed-choice* `%LIST`
   * trait named in `selectedTraits` (AT-34-E4-002, second slice). Optional
   * so every existing caller and test composes unchanged; an absent field
   * means "no choice-based trait was taken", the real and common answer
   * when `selectedTraits` names only flat traits or none at all.
   */
  traitSkillChoices?: readonly TraitSkillChoiceDto[];
}

export interface ComposeCreateCharacterRequestDependencies {
  generateId: () => string;
  now: () => string;
}

/**
 * Pure request composer — dependency-injected id/clock so callers (and
 * tests) control identity and timestamp generation instead of this module
 * reaching for `crypto.randomUUID()` / `Date` directly.
 */
export function composeCreateCharacterRequest(
  fields: CreateCharacterFormFields,
  deps: ComposeCreateCharacterRequestDependencies
): CreateCharacterRequest {
  return {
    characterId: deps.generateId(),
    displayLabel: fields.displayLabel,
    raceId: fields.raceId,
    classId: fields.classId,
    level: fields.level,
    abilityScores: { ...fields.abilityScores },
    abilityBonusTarget: fields.abilityBonusTarget,
    savedAt: deps.now(),
    selectedAlternateTraitKeys: [...(fields.selectedAlternateTraitKeys ?? [])],
    selectedTraits: [...(fields.selectedTraits ?? [])],
    traitSkillChoices: [...(fields.traitSkillChoices ?? [])],
  };
}
