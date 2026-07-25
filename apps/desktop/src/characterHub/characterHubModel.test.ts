import { CLASS_OPTIONS, describeClassSupportLevel, getLevelOptionsForClass } from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesFighterGetsThreeLevels();
  verifiesRangerGetsFiveLevels();
  verifiesPaladinGetsFiveLevels();
  verifiesBarbarianGetsThreeLevels();
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

// Ranger genuinely reaches Computed past level 1 (verified against
// pilot_compute.rs and live-verified through the real dev build: a fresh
// Elf Ranger reaches Computed at level 1 and stays Computed leveling up
// through level 4, past the point spells first become accessible; only a
// single-class Human at level 1 specifically stays blocked). Not lumped in
// with the level-1-only classes below, mirroring how Fighter already isn't.
function verifiesRangerGetsFiveLevels() {
  const levels = getLevelOptionsForClass('class:ranger');
  assertEqual(levels.length, 5, 'ranger level option count');
  assertEqual(levels[0], 1, 'ranger level option 0');
  assertEqual(levels[4], 5, 'ranger level option 4');
}

// Paladin mirrors Ranger exactly (ee3c50ce, same explain_hybrid_level1_chassis
// shape) -- live-verified through the real dev build the same way: fresh Human
// Paladin 1 stays Blocked, fresh Elf Paladin 1 reaches Computed and stays
// Computed leveling up through level 4.
function verifiesPaladinGetsFiveLevels() {
  const levels = getLevelOptionsForClass('class:paladin');
  assertEqual(levels.length, 5, 'paladin level option count');
  assertEqual(levels[0], 1, 'paladin level option 0');
  assertEqual(levels[4], 5, 'paladin level option 4');
}

// Barbarian is genuinely `full` (no hybrid-level-1/Human carve-out at all --
// unlike Ranger/Paladin, it never appears in hybrid_level1_class's match
// arms). Live-verified through the real dev build: a fresh Human Barbarian 1
// (the default, not-raging posture) and a fresh Dwarf Barbarian 1 both
// reached Computed/Saved; leveling the Dwarf character up reached level 2
// cleanly, disk-confirmed.
function verifiesBarbarianGetsThreeLevels() {
  const levels = getLevelOptionsForClass('class:barbarian');
  assertEqual(levels.length, 3, 'barbarian level option count');
  assertEqual(levels[0], 1, 'barbarian level option 0');
  assertEqual(levels[2], 3, 'barbarian level option 2');
}

function verifiesEveryOtherClassGetsLevelOneOnly() {
  for (const option of CLASS_OPTIONS) {
    if (
      option.id === 'class:fighter' ||
      option.id === 'class:ranger' ||
      option.id === 'class:paladin' ||
      option.id === 'class:barbarian'
    )
      continue;
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
    describeClassSupportLevel('partial-human-only', 'Wizard').toLowerCase().includes('human'),
    'partial-human-only copy should mention Human'
  );
  assert(
    describeClassSupportLevel('human-diagnostics-only', 'Monk').toLowerCase().includes('human'),
    'human-diagnostics-only copy should still mention Human (it gets named diagnostics)'
  );
  assert(
    describeClassSupportLevel('human-diagnostics-only', 'Monk').toLowerCase().includes('for any race yet, including human'),
    'human-diagnostics-only copy should be explicit that Human never computes either'
  );
  assert(
    describeClassSupportLevel('none', 'Rogue').toLowerCase().includes("isn't computed"),
    'none copy should say it is not computed'
  );
  assert(
    describeClassSupportLevel('full-except-human-level-1', 'Ranger').toLowerCase().includes('human'),
    'full-except-human-level-1 copy should mention Human'
  );
  assert(
    describeClassSupportLevel('full-except-human-level-1', 'Ranger').toLowerCase().includes('level 1'),
    'full-except-human-level-1 copy should be explicit that the exception is level 1 specifically'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
