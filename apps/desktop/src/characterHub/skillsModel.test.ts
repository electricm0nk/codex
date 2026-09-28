import {
  allocationFromPersisted,
  classSkillLookup,
  DEFAULT_PICK_MARKER,
  featSkillBonusFor,
  isClassSkill,
  persistedFromAllocation,
  skillIdFor,
  skillModifier,
  SKILLS,
  skillPointsSpent,
  totalSkillPointsAvailable,
} from './skillsModel';
import { featSkillBonusWire } from '../testSupport/featSkillBonusWire';
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

/**
 * SD-36 F6b: Alertness (CRB p.117) on the census fixture (Human Fighter 1, Wis 12 = +1) adds +2 to
 * Perception and Sense Motive, off the engine's served fold. Perception with 0 ranks:
 * +1 Wis + 0 ranks + 2 Alertness = +3; Sense Motive the same. No other skill moves.
 */
function verifiesAlertnessAddsTwoToPerceptionAndSenseMotive() {
  const served = featSkillBonusWire();
  assertEqual(featSkillBonusFor(served, 'Perception'), 2, 'Alertness +2 Perception, off the served fold');
  assertEqual(featSkillBonusFor(served, 'Sense Motive'), 2, 'Alertness +2 Sense Motive');
  assertEqual(skillModifier(1, 0, false, featSkillBonusFor(served, 'Perception')), 3, 'Perception total: Wis +1, 0 ranks, Alertness +2');
  const moved = SKILLS.filter((skill) => featSkillBonusFor(served, skill.name) !== 0).map((skill) => skill.name);
  assertEqual(moved.join(','), 'Perception,Sense Motive', 'exactly the two skills Alertness names');
}

/** A family bonus (`SkillGroup`) reaches every member of the family, and only them. */
function verifiesAFamilyBonusReachesEveryMember() {
  const bonuses = { skills: { knowledge_local: 1 }, groups: { knowledge: 2 }, contributions: [], situational: [], unknown: [] };
  assertEqual(featSkillBonusFor(bonuses, 'Knowledge (Local)'), 3, 'family +2 and the exact +1 add');
  assertEqual(featSkillBonusFor(bonuses, 'Knowledge (Arcana)'), 2, 'family only');
  assertEqual(featSkillBonusFor(bonuses, 'Linguistics'), 0, 'not a member');
}

/** The served wizard facts re-keyed to `level` (the wizard's class-skill list does not move with level). */
function wizardFactsAt(level: number): ClassFactsState {
  const wizard = classFactsWire().classes.find((entry) => entry.classId === 'class:wizard' && entry.level === 1);
  if (!wizard) {
    throw new Error('no served wizard facts');
  }
  return loadedClassFacts({ classes: [{ ...wizard, level }] });
}

/**
 * SD-36 F7a (F7-8): Elowen Ashgrave as the seed persists her (`character_hub.rs`
 * `ELOWEN_SEED_SKILL_RANKS`): Human Wizard 5, Int 18 (+4).
 * Earned: (2 wizard + 4 Int + 1 Human Skilled) x 5 levels = 35.
 * Persisted: the create path's Climb, Intimidate, Swim at 1 (the GE-06 posture; cross-class, one
 * point each) + Spellcraft, Fly and Knowledge (Arcana, Dungeoneering, Planes, Religion) at 5 (max
 * ranks = character level 5) + Linguistics 2 = 3 + 30 + 2 = 35 -> 0 unallocated.
 * Spellcraft: 5 ranks + 3 class skill + 4 Int = +12.
 */
function verifiesElowenPersistedAllocationLeavesNothingUnallocated() {
  installClassRoster(classRosterWire());
  const posture = [
    { skillId: 'skill:climb', ranks: 1 },
    { skillId: 'skill:intimidate', ranks: 1 },
    { skillId: 'skill:swim', ranks: 1 },
  ];
  const wizardSkills = [
    { skillId: 'skill:spellcraft', ranks: 5 },
    { skillId: 'skill:knowledge_arcana', ranks: 5 },
    { skillId: 'skill:knowledge_planes', ranks: 5 },
    { skillId: 'skill:knowledge_dungeoneering', ranks: 5 },
    { skillId: 'skill:knowledge_religion', ranks: 5 },
    { skillId: 'skill:fly', ranks: 5 },
    { skillId: 'skill:linguistics', ranks: 2 },
  ];
  const allocation = allocationFromPersisted([...posture, ...wizardSkills]);
  assertEqual(allocation['Spellcraft'], 5, 'the persisted id maps to its panel row');
  assertEqual(allocation['Knowledge (Arcana)'], 5, 'a parenthetical id maps back to its row');
  const held = [{ classId: 'class:wizard', classLabel: 'Wizard', level: 5 }];
  const lookup = classSkillLookup(held, wizardFactsAt(5));
  for (const entry of wizardSkills) {
    const name = SKILLS.find((skill) => skillIdFor(skill.name) === entry.skillId)?.name ?? entry.skillId;
    assert(isClassSkill(lookup, name), `${name} is a wizard class skill`);
  }
  assert(!isClassSkill(lookup, 'Climb'), 'Climb is cross-class for a Wizard');
  const earned = totalSkillPointsAvailable(held, 4, true);
  assertEqual(earned, 35, '(2 + 4 + 1) x 5');
  assertEqual((earned ?? 0) - skillPointsSpent(allocation), 0, 'Elowen loads with 0 unallocated');
  assertEqual(skillModifier(4, allocation['Spellcraft'] ?? 0, isClassSkill(lookup, 'Spellcraft')), 12, 'Spellcraft 5 + 3 + 4');
}

/**
 * PF1 (CRB Chapter 4, Acquiring Skills): one skill point buys one rank, class skill or not; a
 * class skill adds +3 instead. A Human Wizard 1 with Int 10 carrying the create path's Climb,
 * Intimidate and Swim (all cross-class) at 1 rank has spent 3 of its 3 points, not 6.
 */
function verifiesACrossClassRankCostsOnePoint() {
  const allocation = allocationFromPersisted([
    { skillId: 'skill:climb', ranks: 1 },
    { skillId: 'skill:intimidate', ranks: 1 },
    { skillId: 'skill:swim', ranks: 1 },
  ]);
  assertEqual(skillPointsSpent(allocation), 3, 'three ranks, three points');
}

/** An id with no panel row is kept verbatim, counted, and written back unchanged. */
function verifiesAnUnlistedIdRoundTrips() {
  const allocation = allocationFromPersisted([
    { skillId: 'skill:knowledge_psionics', ranks: 2 },
    { skillId: 'skill:climb', ranks: 1 },
  ]);
  assertEqual(skillPointsSpent(allocation), 3, 'the unlisted id is counted');
  const back = persistedFromAllocation(allocation);
  assertEqual(JSON.stringify(back), JSON.stringify([{ skillId: 'skill:knowledge_psionics', ranks: 2 }, { skillId: 'skill:climb', ranks: 1 }]), 'round trip');
}

/**
 * SD-36 F7c: a class skill held only through a Path-A canonical seed is a default pick, read off
 * the served wire. Expert's ten (CRB p.450, "any ten", the player's choice) are all default picks;
 * a Fighter's fixed list has none; an Expert/Fighter multiclass's Climb is granted outright by the
 * Fighter side, so it is no longer a default pick; Samurai's list carries no `samurai_mount`.
 */
function verifiesCanonicalClassSkillPicksAreDefaultPicks() {
  assertEqual(DEFAULT_PICK_MARKER, 'default pick', 'the marker is the engine class_seeds::DEFAULT_PICK_MARKER');
  const expert = lookup(['class:expert']);
  for (const name of ['Acrobatics', 'Appraise', 'Bluff', 'Climb', 'Diplomacy', 'Disable Device', 'Disguise', 'Escape Artist', 'Fly', 'Handle Animal']) {
    assert(isClassSkill(expert, name), `${name} is an Expert class skill`);
    assert(expert.isDefaultPick(name), `${name} is an Expert default pick`);
  }
  assert(!isClassSkill(expert, 'Swim') && !expert.isDefaultPick('Swim'), 'Swim is neither');
  const fighter = lookup(['class:fighter']);
  assert(isClassSkill(fighter, 'Climb') && !fighter.isDefaultPick('Climb'), 'Fighter Climb is a fixed class skill');
  const both = lookup(['class:expert', 'class:fighter']);
  assert(isClassSkill(both, 'Climb') && !both.isDefaultPick('Climb'), 'the Fighter side grants Climb outright');
  assert(both.isDefaultPick('Bluff'), 'Bluff still holds only through the Expert pick');
  const samurai = classFactsWire().classes.find((facts) => facts.classId === 'class:samurai' && facts.level === 1);
  if (samurai === undefined || samurai.classSkills.status !== 'known') {
    throw new Error('samurai is served');
  }
  assert(!samurai.classSkills.skills.includes('samurai_mount'), 'samurai_mount is not a skill');
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
  verifiesAlertnessAddsTwoToPerceptionAndSenseMotive();
  verifiesAFamilyBonusReachesEveryMember();
  verifiesElowenPersistedAllocationLeavesNothingUnallocated();
  verifiesACrossClassRankCostsOnePoint();
  verifiesAnUnlistedIdRoundTrips();
  verifiesCanonicalClassSkillPicksAreDefaultPicks();
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
