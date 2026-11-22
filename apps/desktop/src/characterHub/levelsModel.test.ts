import { assert, assertEqual } from '../testSupport/asserts';
import {
  MAX_CHARACTER_LEVEL,
  addLevel,
  canAddLevel,
  averageHitDieValue,
  characterLevel,
  creationRequestShape,
  heldClassesOf,
  levelHitPoints,
  removeLevel,
  rerollLevel,
  rollHitDie,
  totalHitPoints,
  type CreationLevel,
} from './levelsModel';

const fighter = (entries: CreationLevel[]) => addLevel(entries, 'class:fighter', 10);
const wizard = (entries: CreationLevel[]) => addLevel(entries, 'class:wizard', 6);

function verifiesAverageIsHalfThePlusOne() {
  assertEqual(averageHitDieValue(6), 4, 'd6 averages 4');
  assertEqual(averageHitDieValue(8), 5, 'd8 averages 5');
  assertEqual(averageHitDieValue(10), 6, 'd10 averages 6');
  assertEqual(averageHitDieValue(12), 7, 'd12 averages 7');
}

function verifiesLevelOneAlwaysGetsTheMaximum() {
  const entries = fighter([]);
  assertEqual(entries[0].value, 10, 'level 1 is the full die');
  assertEqual(entries[0].rolled, false, 'level 1 is not a roll');
  const wizardFirst = wizard([]);
  assertEqual(wizardFirst[0].value, 6, 'a wizard first level is the full d6 too');
}

function verifiesLaterLevelsDefaultToTheAverage() {
  const entries = wizard(fighter([]));
  assertEqual(entries[1].value, 4, 'the second level uses the class it was added with (d6 -> 4)');
  assertEqual(entries[1].classId, 'class:wizard', 'and records that class');
  const more = fighter(entries);
  assertEqual(more[2].value, 6, 'a d10 level after level 1 defaults to 6');
}

function verifiesAddingDoesNotMutateTheInput() {
  const entries = fighter([]);
  fighter(entries);
  assertEqual(entries.length, 1, 'addLevel returns a new list');
}

function verifiesRollsStayInsideTheDie() {
  assertEqual(rollHitDie(10, () => 0), 1, 'the lowest roll is 1');
  assertEqual(rollHitDie(10, () => 0.999999), 10, 'the highest roll is the die size');
  for (const die of [6, 8, 10, 12]) {
    for (let i = 0; i < 200; i += 1) {
      const roll = rollHitDie(die);
      assert(roll >= 1 && roll <= die && Number.isInteger(roll), `d${die} rolled ${roll}`);
    }
  }
}

function verifiesRerollReplacesOneLevelAndMarksItRolled() {
  const entries = fighter(fighter(fighter([])));
  const rerolled = rerollLevel(entries, 1, () => 0.5);
  assertEqual(rerolled[1].value, 6, 'a d10 at 0.5 rolls 6');
  assertEqual(rerolled[1].rolled, true, 'the level is marked rolled');
  assertEqual(rerolled[2].value, entries[2].value, 'other levels are untouched');
  assertEqual(entries[1].rolled, false, 'the input list is untouched');
}

function verifiesLevelOneCannotBeRerolled() {
  const entries = fighter(fighter([]));
  const same = rerollLevel(entries, 0, () => 0);
  assertEqual(same[0].value, 10, 'level 1 stays at the maximum');
  assertEqual(same[0].rolled, false, 'and is never a roll');
}

function verifiesRemovingTheFirstLevelPromotesTheNextToTheMaximum() {
  const entries = wizard(fighter([]));
  const after = removeLevel(entries, 0);
  assertEqual(after.length, 1, 'one level left');
  assertEqual(after[0].classId, 'class:wizard', 'the wizard level is now first');
  assertEqual(after[0].value, 6, 'and now takes the full d6');
  assertEqual(after[0].rolled, false, 'not a roll');
}

function verifiesHitPointsAddConstitutionToEveryLevelAndFloorAtOne() {
  const entries = wizard(fighter([]));   // 10, then 4
  assertEqual(levelHitPoints(entries[0], 2), 12, 'level 1: 10 + 2');
  assertEqual(levelHitPoints(entries[1], 2), 6, 'level 2: 4 + 2');
  assertEqual(totalHitPoints(entries, 2), 18, 'total is the sum');
  assertEqual(levelHitPoints(entries[1], -5), 1, 'a level never gives less than 1 hp');
  assertEqual(totalHitPoints(entries, -5), 6, 'total: (10-5) + max(1, 4-5)');
  assertEqual(totalHitPoints([], 3), 0, 'no levels, no hit points');
}

function verifiesCharacterLevelAndHeldClasses() {
  const entries = fighter(wizard(fighter(fighter([]))));
  assertEqual(characterLevel(entries), 4, 'one level per entry');
  assertEqual(JSON.stringify(heldClassesOf(entries)), JSON.stringify([
    { classId: 'class:fighter', level: 3 },
    { classId: 'class:wizard', level: 1 },
  ]), 'classes in order of first appearance with their level counts');
}

function verifiesTheCreationRequestShape() {
  assertEqual(creationRequestShape([]), null, 'nothing to create without a level');
  const single = creationRequestShape(fighter(fighter([])));
  assertEqual(JSON.stringify(single), JSON.stringify({
    primaryClassId: 'class:fighter', primaryLevel: 2, additionalLevels: [],
    hitPointLevels: [{ classId: 'class:fighter', value: 10 }, { classId: 'class:fighter', value: 6 }],
  }), 'a single class is its class and level, with every level\'s hit die value');
  const multi = creationRequestShape(fighter(wizard(fighter([]))));
  assert(multi !== null, 'a multiclass shape exists');
  assertEqual(multi?.primaryClassId, 'class:fighter', 'the first level\'s class is the primary');
  assertEqual(multi?.primaryLevel, 2, 'the primary class carries all of its own levels');
  assertEqual(JSON.stringify(multi?.additionalLevels), JSON.stringify(['class:wizard']), 'other classes arrive as additional levels, in order');
  assertEqual(multi?.hitPointLevels.length, 3, 'every level has a stored value');
}

verifiesAverageIsHalfThePlusOne();
verifiesLevelOneAlwaysGetsTheMaximum();
verifiesLaterLevelsDefaultToTheAverage();
verifiesAddingDoesNotMutateTheInput();
verifiesRollsStayInsideTheDie();
verifiesRerollReplacesOneLevelAndMarksItRolled();
verifiesLevelOneCannotBeRerolled();
verifiesRemovingTheFirstLevelPromotesTheNextToTheMaximum();
verifiesHitPointsAddConstitutionToEveryLevelAndFloorAtOne();
verifiesCharacterLevelAndHeldClasses();
verifiesTheCreationRequestShape();
verifiesWhetherALevelCanBeAdded();

function verifiesWhetherALevelCanBeAdded() {
  const fighterOption = { id: 'class:fighter', label: 'Fighter', hitDie: 10, levelOptions: [1, 2, 3] };
  assertEqual(MAX_CHARACTER_LEVEL, 20, 'PF1 characters top out at 20 levels');
  assertEqual(canAddLevel([], fighterOption).ok, true, 'a first level can be added');
  const atCap = addLevel(addLevel(addLevel([], 'class:fighter', 10), 'class:fighter', 10), 'class:fighter', 10);
  const capped = canAddLevel(atCap, fighterOption);
  assertEqual(capped.ok, false, 'a class cannot go past the levels the engine computes for it');
  assert((capped.reason ?? '').includes('Fighter'), 'and the reason names the class');
  const noDie = canAddLevel([], { ...fighterOption, hitDie: null });
  assertEqual(noDie.ok, false, 'a class with no known hit die cannot be added');
  assert((noDie.reason ?? '').includes('hit die'), 'and the reason says why');
  let full: CreationLevel[] = [];
  for (let i = 0; i < MAX_CHARACTER_LEVEL; i += 1) {
    full = addLevel(full, i % 2 === 0 ? 'class:fighter' : 'class:wizard', 8);
  }
  const wizardOption = { id: 'class:rogue', label: 'Rogue', hitDie: 8, levelOptions: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] };
  const over = canAddLevel(full, wizardOption);
  assertEqual(over.ok, false, 'a 21st level cannot be added');
  assert((over.reason ?? '').includes('20'), 'and the reason names the cap');
}

console.log('levelsModel.test.ts: all assertions passed');
