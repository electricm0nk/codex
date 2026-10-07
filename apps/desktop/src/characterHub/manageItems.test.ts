import { assert, assertEqual } from '../testSupport/asserts';
import type { CharacterTraitOptionDto } from '../boundary/loadCharacterTraits';
import type { AlternateTraitRow } from './alternateTraitSelection';
import type { RacialTraitRow } from './racialTraitsModel';
import { CHARACTER_TRAIT_LIMIT, alternateTraitItems, characterTraitItems, traitBonusSummary } from './manageItems';

const altRow = (over: Record<string, unknown> = {}): AlternateTraitRow =>
  ({
    alternate: { key: 'Dwarf ~ Saltbeard', name: 'Saltbeard', book: 'ARG', sourcePage: 'p.50', replaces: [{ name: 'Stability' }], setsFlags: [] },
    selected: false,
    disabledReason: null,
    description: 'You have a +2 bonus on Survival.',
    droppedArgs: [],
    ...over,
  }) as unknown as AlternateTraitRow;

const innateRow = (over: Record<string, unknown> = {}): RacialTraitRow =>
  ({ key: 'Dwarf ~ Hardy', name: 'Hardy', book: 'CRB', role: 'default', roleLabel: 'Standard', text: '+2 against poison.', droppedArgs: [], ...over }) as unknown as RacialTraitRow;

const trait = (over: Partial<CharacterTraitOptionDto> = {}): CharacterTraitOptionDto => ({
  id: 'trait:trait_acrobat',
  name: 'Acrobat',
  description: 'You have always been nimble.',
  skills: ['Acrobatics'],
  bonus: 1,
  skillOptions: [],
  choiceSetId: null,
  save: null,
  otherPillars: [],
  abilitySubstitution: null,
  ...over,
});

function verifiesAlternateTraitsBecomeSelectableItemsAndStandardOnesBecomeInnate() {
  const items = alternateTraitItems([altRow()], [innateRow()]);
  assertEqual(items.length, 2, 'one alternate and one innate');
  const alt = items[0];
  assertEqual(alt.id, 'Dwarf ~ Saltbeard', 'the corpus key is the id');
  assert(alt.label.includes('Saltbeard') && alt.label.includes('ARG'), 'label names the trait and its book');
  assert((alt.description ?? '').includes('+2 bonus on Survival'), 'the engine-rendered prose is the description');
  assert((alt.description ?? '').includes('Replaces Stability'), 'and says what it replaces');
  assertEqual(alt.qualified, true, 'an available alternate is qualified');
  const innate = items[1];
  assertEqual(innate.innate, true, 'a standard racial trait is innate');
  assert(innate.id !== 'Dwarf ~ Hardy', 'its id cannot collide with a selectable key');
  assertEqual(innate.description, '+2 against poison.', 'with the engine prose');
}

function verifiesALockedOutAlternateIsUnqualifiedWithTheBackendsReason() {
  const [locked] = alternateTraitItems([altRow({ disabledReason: 'Excluded by Dwarf ~ Other' })], []);
  assertEqual(locked.qualified, false, 'a locked-out alternate cannot be taken');
  assertEqual(locked.unqualifiedReason, 'Excluded by Dwarf ~ Other', 'the reason is the backend\'s own');
}

function verifiesAnIncompleteDescriptionIsFlagged() {
  const [alt] = alternateTraitItems([altRow({ droppedArgs: ['DC'] })], []);
  assert((alt.description ?? '').includes('could not resolve DC'), 'a description the engine could not complete says so');
}

function verifiesTraitBonusSummaries() {
  assertEqual(traitBonusSummary(trait()), '+1 Acrobatics', 'a flat skill bonus');
  assertEqual(traitBonusSummary(trait({ skills: [], save: 'Will', bonus: 1 })), '+1 Will save', 'a save bonus');
  assertEqual(
    traitBonusSummary(trait({ skills: [], skillOptions: [{ skillId: 'skill:climb', name: 'Climb' }, { skillId: 'skill:swim', name: 'Swim' }], bonus: 1 })),
    '+1 choice of Climb, Swim',
    'a fixed choice of skills'
  );
  assertEqual(
    traitBonusSummary(trait({ skills: [], otherPillars: [{ label: 'Initiative checks', bonus: 2 }] })),
    '+2 Initiative checks',
    'a non-skill pillar'
  );
}

function verifiesCharacterTraitsAreAllQualifiedAndCarryTheirNumbers() {
  const items = characterTraitItems([trait(), trait({ id: 'trait:other', name: 'Other', skills: ['Climb'], bonus: 1 })]);
  assertEqual(items.length, 2, 'every trait is an option');
  assertEqual(items[0].id, 'trait:trait_acrobat', 'the wire id is the item id');
  assert((items[0].description ?? '').includes('+1 Acrobatics'), 'the numbers lead the description');
  assert((items[0].description ?? '').includes('always been nimble'), 'followed by the trait text');
  assert(items.every((item) => item.qualified), 'a character trait has no prerequisites here');
  assertEqual(CHARACTER_TRAIT_LIMIT, 2, 'PF1 gives two traits');
}

verifiesAlternateTraitsBecomeSelectableItemsAndStandardOnesBecomeInnate();
verifiesALockedOutAlternateIsUnqualifiedWithTheBackendsReason();
verifiesAnIncompleteDescriptionIsFlagged();
verifiesTraitBonusSummaries();
verifiesCharacterTraitsAreAllQualifiedAndCarryTheirNumbers();
console.log('manageItems.test.ts: all assertions passed');
