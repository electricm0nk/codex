import {
  CLASS_OPTIONS,
  MAX_CLASS_LEVEL,
  canTakeAnotherLevelIn,
  clampLevelForClass,
  describeClassSupportLevel,
  getLevelOptionsForClass,
} from './characterHubModel';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * `levelOptions` is the app's honest claim about which levels of a class a
 * player can actually build. Its ground truth is
 * `cargo run --bin v06_class_state_dump`, which sweeps every class over
 * levels 1-20 through the real `build_pilot_headless_receipt` pipeline under
 * the exact posture `compose_character_input` produces. The run of
 * 2026-07-29 reports 11 of 27 classes `Computed` at every level 1-20 —
 * barbarian, bard, cleric, druid, fighter, paladin, ranger, rogue, sorcerer,
 * wizard (CRB) and arcanist (ACG) — and every other class `Blocked` at every
 * level 1-20.
 *
 * These tests pin exactly that split. A class either offers the full 1-20
 * range because the engine computes it there, or it offers only level 1 (the
 * "let the player see the real blocking diagnostics" posture) because the
 * engine computes it nowhere.
 */

/** The classes the engine dump reports `Computed` at every level 1-20. */
const FULLY_COMPUTED_CLASS_IDS = [
  'class:barbarian',
  'class:bard',
  'class:cleric',
  'class:druid',
  'class:fighter',
  'class:paladin',
  'class:ranger',
  'class:rogue',
  'class:sorcerer',
  'class:wizard',
  'class:arcanist',
];

async function main() {
  verifiesEveryEngineComputedClassOffersAllTwentyLevels();
  verifiesArcanistIsSelectableAtAll();
  verifiesEngineBlockedClassesStayAtLevelOneOnly();
  verifiesUnknownClassFallsBackToLevelOneOnly();
  verifiesClampLevelForClassKeepsLevelsInsideTheClassRange();
  verifiesCanTakeAnotherLevelInStopsAtTheVerifiedCeiling();
  verifiesSupportLevelCopyPerLevel();
}

function verifiesEveryEngineComputedClassOffersAllTwentyLevels() {
  assertEqual(MAX_CLASS_LEVEL, 20, 'PF1 class ceiling');
  for (const classId of FULLY_COMPUTED_CLASS_IDS) {
    const levels = getLevelOptionsForClass(classId);
    assertEqual(levels.length, 20, `${classId} level option count`);
    assertEqual(levels[0], 1, `${classId} lowest level option`);
    assertEqual(levels[19], 20, `${classId} highest level option`);
    for (let index = 0; index < 20; index += 1) {
      assertEqual(levels[index], index + 1, `${classId} level option ${index}`);
    }
  }
}

// Arcanist reaches `Computed` at every level 1-20 in the same engine dump the
// CRB classes are read from, and `pf1_adapter.rs` already seeds its canonical
// Metamagic Knowledge choice + starter spellbook on the real creation path --
// it was simply absent from this picker, so no player could select it.
function verifiesArcanistIsSelectableAtAll() {
  const arcanist = CLASS_OPTIONS.find((option) => option.id === 'class:arcanist');
  assert(arcanist !== undefined, 'Arcanist must be offered in the class picker');
  assertEqual(arcanist?.label, 'Arcanist', 'arcanist label');
  assertEqual(arcanist?.supportLevel, 'full', 'arcanist support level');
  assertEqual(arcanist?.hitDie, 6, 'arcanist hit die');
}

function verifiesEngineBlockedClassesStayAtLevelOneOnly() {
  for (const option of CLASS_OPTIONS) {
    if (FULLY_COMPUTED_CLASS_IDS.includes(option.id)) {
      continue;
    }
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

// Switching the class picker must never leave a level selected that the newly
// chosen class does not actually offer -- e.g. Fighter 20 -> Monk has to fall
// back to 1, the only level Monk offers, rather than submitting a level the
// engine reports Blocked.
function verifiesClampLevelForClassKeepsLevelsInsideTheClassRange() {
  assertEqual(clampLevelForClass('class:fighter', 20), 20, 'fighter keeps level 20');
  assertEqual(clampLevelForClass('class:monk', 20), 1, 'monk clamps down to its only level');
  assertEqual(clampLevelForClass('class:arcanist', 12), 12, 'arcanist keeps a mid level');
  assertEqual(clampLevelForClass('class:does-not-exist', 7), 1, 'unknown class clamps to 1');
  assertEqual(clampLevelForClass('class:wizard', 0), 1, 'a level below the range clamps up to 1');
}

// The level-up dialog must not offer a class level past what the engine dump
// verified. PF1 itself stops at 20, and the dump sweeps exactly 1-20, so a
// 21st level is not "supported but untested" -- it is unverified reach.
function verifiesCanTakeAnotherLevelInStopsAtTheVerifiedCeiling() {
  assert(canTakeAnotherLevelIn('class:fighter', 0), 'a brand-new Fighter level is available');
  assert(canTakeAnotherLevelIn('class:fighter', 19), 'Fighter 19 -> 20 is available');
  assert(!canTakeAnotherLevelIn('class:fighter', 20), 'Fighter 20 -> 21 must not be offered');
  assert(canTakeAnotherLevelIn('class:monk', 0), 'a first Monk level is still offered (it shows real diagnostics)');
  assert(!canTakeAnotherLevelIn('class:monk', 1), 'Monk 1 -> 2 must not be offered');
  assert(canTakeAnotherLevelIn('class:arcanist', 5), 'Arcanist 5 -> 6 is available');
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
  assert(
    describeClassSupportLevel('headless-only', 'Sorcerer').toLowerCase().includes('sorcerer'),
    'headless-only copy should name the class'
  );
  assert(
    describeClassSupportLevel('headless-only', 'Sorcerer').toLowerCase().includes('picker'),
    'headless-only copy should name the real cause (a missing picker), not a vague "not computed"'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
