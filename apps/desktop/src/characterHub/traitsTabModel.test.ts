import { resolveSelectedTraits } from './traitsTabModel';
import type { CharacterTraitOptionDto } from '../boundary/loadCharacterTraits';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * F-3 (scout audit item 32): `selectedTraits` was loaded and carried through
 * every refresh with zero render sites, so a player could pick a trait at
 * creation and never see it again. These pin the resolver behind the sheet's
 * Traits section: every id resolves against the real
 * `list_available_character_traits` roster; the bonus line is written from
 * the engine's own `bonus`/`skills`/`save`/`otherPillars` fields verbatim;
 * an id the roster does not carry still renders (as its raw id) rather than
 * vanishing.
 */

function option(overrides: Partial<CharacterTraitOptionDto>): CharacterTraitOptionDto {
  return {
    id: 'x',
    name: 'X',
    description: 'desc',
    skills: [],
    bonus: 0,
    skillOptions: [],
    choiceSetId: null,
    save: null,
    otherPillars: [],
    abilitySubstitution: null,
    ...overrides,
  };
}

const catalog: CharacterTraitOptionDto[] = [
  option({ id: 'acrobat', name: 'Acrobat', description: 'Nimble.', skills: ['Acrobatics'], bonus: 1 }),
  option({ id: 'resilient', name: 'Resilient', description: 'Tough.', save: 'Fortitude', bonus: 1 }),
  option({ id: 'reactionary', name: 'Reactionary', description: 'Quick.', otherPillars: [{ label: 'Initiative checks', bonus: 2 }] }),
  option({ id: 'choosy', name: 'Choosy', description: 'Pick one.', bonus: 1, choiceSetId: 'cs1', skillOptions: [{ skillId: 'a', name: 'Appraise' }] }),
  option({ id: 'clever', name: 'Clever', description: 'Formula.', skills: ['Diplomacy'], abilitySubstitution: { formula: 'max(INT,CHA)-CHA', flatBonus: 0 } }),
];

const rows = resolveSelectedTraits(['acrobat', 'resilient', 'reactionary', 'choosy', 'clever', 'ghost'], catalog);

assertEqual(rows.length, 6, 'one row per selected id, nothing dropped');
assertEqual(rows[0].name, 'Acrobat', 'name from the roster');
assertEqual(rows[0].description, 'Nimble.', 'description verbatim');
assertEqual(rows[0].grants, '+1 Acrobatics', 'skill bonus written from engine fields');
assertEqual(rows[1].grants, '+1 Fortitude save', 'save bonus');
assertEqual(rows[2].grants, '+2 Initiative checks', 'other-pillar bonus');
assertEqual(rows[3].grants, '+1 to a chosen skill', 'choice trait: the chosen skill is not on the load surface, so say so');
assertEqual(rows[4].grants, 'Diplomacy: max(INT,CHA)-CHA', 'substitution formula is shown, never evaluated here');
assertEqual(rows[5].name, 'ghost', 'unknown id renders as its raw id');
assertEqual(rows[5].description, null, 'no fabricated description');
assertEqual(rows[5].grants, null, 'no fabricated bonus');
assert(resolveSelectedTraits([], catalog).length === 0, 'empty stays empty');

console.log('traitsTabModel tests passed');
