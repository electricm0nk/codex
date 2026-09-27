import { classSkillLookup, isClassSkill, skillIdFor, totalSkillPointsAvailable } from './skillsModel';
import { LOADING_CLASS_FACTS, failedClassFacts, loadedClassFacts, type ClassFactsState } from './classFactsModel';
import { classFactsWire } from '../testSupport/classFactsWire';
import { installClassRoster } from './classRoster';
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

/** The engine's served facts for every roster class at level 1 (the committed live wire). */
function servedAtLevel1(): ClassFactsState {
  const wire = classFactsWire();
  return loadedClassFacts({ classes: wire.classes.filter((facts) => facts.level === 1) });
}

function lookup(classIds: string[], state: ClassFactsState = servedAtLevel1()) {
  return classSkillLookup(classIds.map(heldClass), state);
}

function verifiesIsClassSkillForAConfirmedBoundary() {
  assert(isClassSkill(lookup(['class:fighter']), 'Climb'), 'Climb is a Fighter class skill');
  assert(!isClassSkill(lookup(['class:fighter']), 'Bluff'), 'Bluff is not a Fighter class skill');
}

function verifiesIsClassSkillMulticlassUnion() {
  assert(
    isClassSkill(lookup(['class:fighter', 'class:rogue']), 'Bluff'),
    'a Fighter/Rogue multiclass counts Bluff as a class skill via the Rogue side of the union, even though Fighter alone does not grant it'
  );
}

/** A granted family covers each member: Wizard's Knowledge (all) and Craft. */
function verifiesAGrantedFamilyCoversItsMembers() {
  const wizard = lookup(['class:wizard']);
  assert(isClassSkill(wizard, 'Knowledge (Planes)'), 'Wizard gets Knowledge (all), including the Planes');
  assert(isClassSkill(wizard, 'Craft'), 'Wizard gets Craft');
  assert(isClassSkill(wizard, 'Spellcraft'), 'Spellcraft is a Wizard class skill');
  assert(!isClassSkill(wizard, 'Stealth') && !isClassSkill(wizard, 'Climb'), 'Stealth and Climb are not');
}

/**
 * SD-36 F6a: newly offered classes read their class skills off the engine, not a 12-row table.
 * Samurai (UC) and Magus (UM) had no list in the deleted table.
 */
function verifiesNewlyOfferedClassesHaveTheirClassSkills() {
  const samurai = lookup(['class:samurai']);
  assertEqual(samurai.unanswered.length, 0, 'Samurai answered');
  assert(isClassSkill(samurai, 'Ride') && isClassSkill(samurai, 'Intimidate'), 'Samurai: Ride, Intimidate');
  const magus = lookup(['class:magus']);
  assert(isClassSkill(magus, 'Spellcraft') && isClassSkill(magus, 'Use Magic Device'), 'Magus: Spellcraft, UMD');
}

/**
 * Denominator: the 59 roster classes, each at level 1, as `list_class_facts` serves them. The
 * engine answers 50; the 9 ACG classes are Unknown by one mechanism (their class line's
 * `Class|<Class>` grant is an unresolved reference in the converted package, so the reader's walk
 * never reaches the class's own `<Class> ~ Class Skills` record) and each is NAMED with that reason.
 */
function verifiesEveryRosterClassIsAnsweredOrNamed() {
  const served = classFactsWire().classes.filter((facts) => facts.level === 1);
  assertEqual(served.length, 59, 'every roster class served once at level 1');
  const answered = served.filter((facts) => facts.classSkills.status === 'known').map((facts) => facts.classId);
  const unknown = served.filter((facts) => facts.classSkills.status === 'unknown').map((facts) => facts.classId);
  assertEqual(answered.length, 50, 'roster classes the engine answers');
  assertEqual(
    unknown.join(','),
    'class:arcanist,class:brawler,class:hunter,class:investigator,class:shaman,class:skald,class:slayer,class:swashbuckler,class:warpriest',
    'the 9 Unknown roster classes'
  );
  for (const classId of unknown) {
    const named = lookup([classId]).unanswered;
    assertEqual(named.length, 1, `${classId} is named`);
    assert(named[0].reason.includes('reaches no class-skill grant'), `${classId}: reason carried (${named[0].reason})`);
  }
}

/** Loading and failure never score a skill as a class skill; both name every held class. */
function verifiesLoadingAndFailureNameEveryClass() {
  const loading = lookup(['class:fighter'], LOADING_CLASS_FACTS);
  assert(!isClassSkill(loading, 'Climb'), 'no bonus while loading');
  assertEqual(loading.unanswered[0].reason, 'loading', 'loading named');
  const failed = lookup(['class:fighter'], failedClassFacts(new Error('boom')));
  assertEqual(failed.unanswered[0].reason, 'class facts unavailable: boom', 'failure notice carried');
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
  verifiesAGrantedFamilyCoversItsMembers();
  verifiesNewlyOfferedClassesHaveTheirClassSkills();
  verifiesEveryRosterClassIsAnsweredOrNamed();
  verifiesLoadingAndFailureNameEveryClass();
  verifiesTotalSkillPointsAvailableReadsTheRoster();
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
