import {
  classSkillListCoverage,
  heldClassesWithoutClassSkillList,
  isClassSkill,
  skillIdFor,
  totalSkillPointsAvailable,
} from './skillsModel';
import { classOptionsFromRoster, installClassRoster } from './classRoster';
import { LOADING_CLASS_CATALOG, setClassCatalog } from './classCatalog';
import { classRosterWire } from '../testSupport/classRosterWire';
import type { HeldClass } from './characterProgression';
import { assert, assertEqual } from '../testSupport/asserts';

function heldClass(classId: string): HeldClass {
  return { classId, classLabel: classId, level: 1 };
}

/**
 * risks-and-open-questions.md item 25: only 5 of `skillIdFor`'s 35 mappings
 * are confirmed against the compute engine's `skill_key_ability_modifier`
 * (Climb, Swim, Intimidate, Diplomacy, Disable Device) — the other 30,
 * including every parenthetical Knowledge skill and every multi-word name,
 * had zero regression coverage pinning down the exact transform.
 */
function verifiesSkillIdForOnAParentheticalSkillName() {
  assertEqual(
    skillIdFor('Knowledge (Arcana)'),
    'skill:knowledge_arcana',
    'a parenthetical skill name collapses to one underscore-joined segment, not a double/trailing underscore artifact'
  );
}

function verifiesSkillIdForOnMultiWordNonParentheticalNames() {
  assertEqual(skillIdFor('Sense Motive'), 'skill:sense_motive', 'a two-word skill name joins on underscore');
  assertEqual(skillIdFor('Sleight of Hand'), 'skill:sleight_of_hand', 'a three-word skill name joins on underscore');
}

function verifiesIsClassSkillForAConfirmedBoundary() {
  assert(isClassSkill([heldClass('class:fighter')], 'Climb'), 'Climb is a Fighter class skill');
  assert(!isClassSkill([heldClass('class:fighter')], 'Bluff'), 'Bluff is not a Fighter class skill');
}

function verifiesIsClassSkillMulticlassUnion() {
  const heldClasses = [heldClass('class:fighter'), heldClass('class:rogue')];
  assert(
    isClassSkill(heldClasses, 'Bluff'),
    'a Fighter/Rogue multiclass counts Bluff as a class skill via the Rogue side of the union, even though Fighter alone does not grant it'
  );
}

// Arcanist became selectable in the class picker once the engine dump confirmed
// it computes at every level 1-20. A selectable class with no entry in
// CLASS_SKILLS silently reports *every* skill as a cross-class skill on the
// Skills tab -- a wrong sheet, not an absent one -- so its list has to land
// with it. ACG Arcanist: Appraise, Craft, Fly, Knowledge (all), Linguistics,
// Profession, Spellcraft, Use Magic Device.
function verifiesIsClassSkillCoversArcanist() {
  const arcanist = [heldClass('class:arcanist')];
  assert(isClassSkill(arcanist, 'Spellcraft'), 'Spellcraft is an Arcanist class skill');
  assert(isClassSkill(arcanist, 'Use Magic Device'), 'Use Magic Device is an Arcanist class skill (unlike Wizard)');
  assert(isClassSkill(arcanist, 'Knowledge (Planes)'), 'Arcanist gets Knowledge (all), including the Planes');
  assert(isClassSkill(arcanist, 'Knowledge (Nature)'), 'Arcanist gets Knowledge (all), including Nature');
  assert(!isClassSkill(arcanist, 'Stealth'), 'Stealth is not an Arcanist class skill');
  assert(!isClassSkill(arcanist, 'Perception'), 'Perception is not an Arcanist class skill');
}

/**
 * SD-36 F4c: the Create picker offers the served roster (59 classes), and this module's class-skill
 * lists are a 12-row hand table. A held class with no list must be NAMED (the Skills panel prints
 * it), never silently scored as all-cross-class. Denominator: the 59 roster ids.
 */
function verifiesEveryRosterClassWithoutAClassSkillListIsNamed() {
  const rosterIds = classOptionsFromRoster(classRosterWire()).map((option) => option.id);
  const coverage = classSkillListCoverage(rosterIds);
  assertEqual(coverage.covered.length + coverage.uncovered.length, 59, 'every roster id counted once');
  assertEqual(coverage.covered.length, 12, 'roster classes with a class-skill list here');
  assertEqual(coverage.uncovered.length, 47, 'roster classes without one');
  for (const classId of coverage.uncovered) {
    const named = heldClassesWithoutClassSkillList([{ classId, classLabel: classId, level: 1 }]);
    assertEqual(named.join(','), classId, `${classId} is named, not silently all-cross-class`);
  }
  assertEqual(heldClassesWithoutClassSkillList([heldClass('class:fighter')]).length, 0, 'Fighter has its list');
}

/** Skill points available read the served skill ranks; unknown ranks are Unknown, not 2. */
function verifiesTotalSkillPointsAvailableReadsTheRoster() {
  installClassRoster(classRosterWire());
  // Inquisitor 2: (6 + 0 Int + 1 human) x 2.
  assertEqual(totalSkillPointsAvailable([{ classId: 'class:inquisitor', classLabel: 'Inquisitor', level: 2 }], 0, true), 14, 'Inquisitor 6 ranks');
  setClassCatalog(LOADING_CLASS_CATALOG);
  assertEqual(totalSkillPointsAvailable([heldClass('class:fighter')], 0, false), null, 'Unknown while the roster loads');
  installClassRoster(classRosterWire());
}

async function main() {
  verifiesSkillIdForOnAParentheticalSkillName();
  verifiesSkillIdForOnMultiWordNonParentheticalNames();
  verifiesIsClassSkillForAConfirmedBoundary();
  verifiesIsClassSkillMulticlassUnion();
  verifiesIsClassSkillCoversArcanist();
  verifiesEveryRosterClassWithoutAClassSkillListIsNamed();
  verifiesTotalSkillPointsAvailableReadsTheRoster();
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
