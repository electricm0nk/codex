import { printedAbilityScore } from './abilityScoresModel';
import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import { assertEqual } from '../testSupport/asserts';

/**
 * SD-36 F7a (F7-1): the Abilities panel prints the ENGINE's effective score
 * (`LoadSavedCharacterResponse.abilityScores`, `effective_ability_scores_dto`), never
 * `10 + 2 x modifier`, which prints every odd score one low.
 */

/** Elowen Ashgrave: Str 8, Dex 14, Con 13, Int 16 + 2 Human = 18, Wis 12, Cha 10. */
const ELOWEN: AbilityScoresDto = { strength: 8, dexterity: 14, constitution: 13, intelligence: 18, wisdom: 12, charisma: 10 };
/** Aldric Ironhand: Str 17 + 2 Human = 19, Dex 13, Con 14, Int 14, Wis 12, Cha 8. */
const ALDRIC: AbilityScoresDto = { strength: 19, dexterity: 13, constitution: 14, intelligence: 14, wisdom: 12, charisma: 8 };
/** A Dwarf: the stored score already carries the fixed +2 Con (CRB p.21): 13 + 2 = 15. */
const DWARF: AbilityScoresDto = { strength: 12, dexterity: 10, constitution: 15, intelligence: 10, wisdom: 13, charisma: 8 };

function verifiesOddScoresPrintAsTheEngineStatesThem() {
  assertEqual(printedAbilityScore(ELOWEN, 'constitution'), '13', 'Elowen Con 13 (was 12)');
  assertEqual(printedAbilityScore(ALDRIC, 'strength'), '19', 'Aldric Str 19 = 17 + 2 Human (was 18)');
  assertEqual(printedAbilityScore(ALDRIC, 'dexterity'), '13', 'Aldric Dex 13 (was 12)');
  assertEqual(printedAbilityScore(DWARF, 'constitution'), '15', 'Dwarf Con 15 (was 14)');
  assertEqual(printedAbilityScore(DWARF, 'wisdom'), '13', 'Dwarf Wis 13 (was 12)');
  assertEqual(printedAbilityScore(ELOWEN, 'intelligence'), '18', 'even scores are unchanged');
}

function verifiesAnAbsentScorePrintsADash() {
  assertEqual(printedAbilityScore(undefined, 'strength'), '—', 'no engine score yet: a dash, never a derived number');
}

verifiesOddScoresPrintAsTheEngineStatesThem();
verifiesAnAbsentScorePrintsADash();
