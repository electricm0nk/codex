import { isClassSkill, skillIdFor } from './skillsModel';
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

async function main() {
  verifiesSkillIdForOnAParentheticalSkillName();
  verifiesSkillIdForOnMultiWordNonParentheticalNames();
  verifiesIsClassSkillForAConfirmedBoundary();
  verifiesIsClassSkillMulticlassUnion();
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
