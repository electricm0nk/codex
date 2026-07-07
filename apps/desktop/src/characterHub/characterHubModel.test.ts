import { CLASS_OPTIONS, describeClassSupportLevel, getLevelOptionsForClass } from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesFighterGetsThreeLevels();
  verifiesEveryOtherClassGetsLevelOneOnly();
  verifiesUnknownClassFallsBackToLevelOneOnly();
  verifiesSupportLevelCopyPerLevel();
}

function verifiesFighterGetsThreeLevels() {
  const levels = getLevelOptionsForClass('class:fighter');
  assertEqual(levels.length, 3, 'fighter level option count');
  assertEqual(levels[0], 1, 'fighter level option 0');
  assertEqual(levels[2], 3, 'fighter level option 2');
}

function verifiesEveryOtherClassGetsLevelOneOnly() {
  for (const option of CLASS_OPTIONS) {
    if (option.id === 'class:fighter') continue;
    const levels = getLevelOptionsForClass(option.id);
    assertEqual(levels.length, 1, `${option.id} level option count`);
    assertEqual(levels[0], 1, `${option.id} level option 0`);
  }
}

function verifiesUnknownClassFallsBackToLevelOneOnly() {
  const levels = getLevelOptionsForClass('class:does-not-exist');
  assertEqual(levels.length, 1, 'unknown class level option count');
  assertEqual(levels[0], 1, 'unknown class level option 0');
}

function verifiesSupportLevelCopyPerLevel() {
  assert(describeClassSupportLevel('full', 'Fighter').includes('Fighter'), 'full copy should name the class');
  assert(
    describeClassSupportLevel('partial-human-only', 'Paladin').toLowerCase().includes('human'),
    'partial-human-only copy should mention Human'
  );
  assert(
    describeClassSupportLevel('none', 'Rogue').toLowerCase().includes("isn't computed"),
    'none copy should say it is not computed'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
