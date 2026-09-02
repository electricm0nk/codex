import { composeCreationBio } from './creationBio';
import { assertEqual } from '../testSupport/asserts';

/**
 * F-1 (scout audit item 2): the create form collected alignment, deity,
 * sex, age, eyes, hair, height and weight and dropped all eight on submit;
 * `update_character_bio` already accepts exactly those. This pins the
 * mapping from form state to the bio DTO — pure formatting, in the same
 * shapes the sheet's bio panel reads back (sex as the raw select value,
 * height as feet'inches", weight as "N lb", blanks when the race has no
 * body profile). Player name is not here: it waits on backend B-1.
 */
const bio = composeCreationBio({
  alignment: 'Lawful Good',
  deity: 'Torag',
  sex: 'female',
  age: 'Middle Age',
  eyes: 'grey',
  hair: 'red',
  heightInches: 50,
  weightLb: 140,
});
assertEqual(bio.alignment, 'Lawful Good', 'alignment verbatim');
assertEqual(bio.deity, 'Torag', 'deity verbatim');
assertEqual(bio.sex, 'female', 'sex is the raw select value the sheet reads back');
assertEqual(bio.age, 'Middle Age', 'age category verbatim');
assertEqual(bio.eyes, 'grey', 'eyes verbatim');
assertEqual(bio.hair, 'red', 'hair verbatim');
assertEqual(bio.height, `4'2"`, 'height formatted the way the form displayed it');
assertEqual(bio.weight, '140 lb', 'weight with unit');
assertEqual(Object.keys(bio).length, 8, 'exactly the eight fields the DTO carries');

const noBody = composeCreationBio({ alignment: 'True Neutral', deity: '', sex: 'male', age: 'Adult', eyes: '', hair: '', heightInches: null, weightLb: null });
assertEqual(noBody.height, '', 'no body profile → blank, not a placeholder string');
assertEqual(noBody.weight, '', 'no body profile → blank');

console.log('creationBio tests passed');
