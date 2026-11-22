import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';

/**
 * SD-36 F7a (F7-1): the score the Abilities panel prints -- the ENGINE's effective score as the
 * load response carries it (`LoadSavedCharacterResponse.abilityScores`, filled by
 * `effective_ability_scores_dto`: the stored score with the Human +2 applied, the same scores the
 * engine derives every modifier from). One rule for every race. No score yet (a mutation refresh
 * before the re-read lands) prints a dash; the score is never reconstructed from the modifier,
 * which prints every odd score one low.
 */
export function printedAbilityScore(scores: AbilityScoresDto | null | undefined, key: keyof AbilityScoresDto): string {
  return scores ? String(scores[key]) : '—';
}
