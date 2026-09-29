import {
  LOADING_CLASS_FACTS,
  classFactsNotice,
  classFactsQueries,
  failedClassFacts,
  loadedClassFacts,
  summarizeCasterLevel,
  summarizeWeaponProficiency,
  type ClassFactsState,
} from './classFactsModel';
import type { HeldClass } from './characterProgression';
import { classFactsWire } from '../testSupport/classFactsWire';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-36 F6a: the sheet's weapon proficiency and caster level fold the engine's served answers
 * (`list_class_facts`, the committed live wire at levels 1 and 7). The deleted tables marked
 * Samurai / Warrior / Magus "✗ Martial Weapons" and printed "—" for every caster outside six ids.
 */

function held(classId: string, classLabel: string, level: number): HeldClass {
  return { classId, classLabel, level };
}

/** The served wire narrowed to one level per class, as the sheet requests it. */
function served(level: 1 | 7): ClassFactsState {
  return loadedClassFacts({ classes: classFactsWire().classes.filter((facts) => facts.level === level) });
}

function tier(state: ClassFactsState, classes: HeldClass[], label: 'Simple' | 'Martial' | 'Exotic') {
  return summarizeWeaponProficiency(classes, state).tiers.find((entry) => entry.label === label)?.proficient;
}

function verifiesNewlyOfferedMartialClassesAreMartial() {
  for (const [classId, label] of [
    ['class:samurai', 'Samurai'],
    ['class:warrior', 'Warrior'],
    ['class:magus', 'Magus'],
    ['class:fighter', 'Fighter'],
  ]) {
    assertEqual(tier(served(7), [held(classId, label, 7)], 'Martial'), 'yes', `${label} 7: Martial ✓`);
  }
  assertEqual(tier(served(7), [held('class:sorcerer', 'Sorcerer', 7)], 'Martial'), 'no', 'Sorcerer: Martial ✗');
  const samurai = summarizeWeaponProficiency([held('class:samurai', 'Samurai', 1)], served(1)).alsoProficientWith;
  assert(
    ['Katana', 'Naginata', 'Wakizashi'].every((weapon) => samurai.includes(weapon)) && !samurai.includes('Samurai'),
    `Samurai prints its weapon-set members, not the selector's label: ${samurai.join(', ')}`
  );
  assertEqual(tier(served(7), [held('class:fighter', 'Fighter', 7)], 'Exotic'), 'no', 'no class grants Exotic');
}

/**
 * SD-36 F7a (F7-2): "Also proficient with" prints weapon records only. The Monk printed `Flurry
 * of Blows` and `Unarmed Strike` beside its weapons; the Magus printed its `Auto` set -- Grapple,
 * Spells (Ray), Spells (Touch), Splash Weapon, Unarmed Strike. None is a weapon record with a
 * proficiency category, so no roster class prints one at level 1 or 7.
 */
function verifiesNoRosterClassPrintsAPseudoWeapon() {
  const pseudo = ['Flurry of Blows', 'Spells (Ray)', 'Spells (Touch)', 'Splash Weapon', 'Unarmed Strike', 'Grapple', 'Mind Blade'];
  const monk = summarizeWeaponProficiency([held('class:monk', 'Monk', 1)], served(1)).alsoProficientWith;
  assert(monk.includes('Kama') && monk.includes('Sword (Short)'), `Monk keeps its weapons: ${monk.join(', ')}`);
  const magus = summarizeWeaponProficiency([held('class:magus', 'Magus', 1)], served(1)).alsoProficientWith;
  assertEqual(magus.join(', '), '', 'Magus prints no named weapon beyond Simple and Martial');
  for (const facts of classFactsWire().classes) {
    const level = facts.level as 1 | 7;
    const printed = summarizeWeaponProficiency([held(facts.classId, facts.classId, level)], served(level)).alsoProficientWith;
    const found = printed.filter((name) => pseudo.includes(name));
    assertEqual(found.join(', '), '', `${facts.classId} ${level} prints no pseudo-weapon`);
  }
}

/** Wizard has no blanket tier: five named weapons, printed; the table said "Simple ✓". */
function verifiesANamedListClassPrintsItsWeapons() {
  const wizard = summarizeWeaponProficiency([held('class:wizard', 'Wizard', 1)], served(1));
  assertEqual(wizard.tiers.find((entry) => entry.label === 'Simple')?.proficient, 'no', 'Wizard: no Simple tier');
  assertEqual(
    wizard.alsoProficientWith.join(', '),
    'Club, Crossbow (Heavy), Crossbow (Light), Dagger, Quarterstaff',
    'Wizard prints its five weapons'
  );
}

/** PF1 union: a Wizard/Fighter is Martial through the Fighter. */
function verifiesProficiencyIsTheUnionAcrossClasses() {
  const state = loadedClassFacts({
    classes: [
      ...classFactsWire().classes.filter((facts) => facts.classId === 'class:wizard' && facts.level === 7),
      ...classFactsWire().classes.filter((facts) => facts.classId === 'class:fighter' && facts.level === 1),
    ],
  });
  const classes = [held('class:wizard', 'Wizard', 7), held('class:fighter', 'Fighter', 1)];
  assertEqual(tier(state, classes, 'Martial'), 'yes', 'Martial via Fighter');
  assertEqual(tier(state, classes, 'Simple'), 'yes', 'Simple via Fighter');
}

function verifiesCasterLevelIsTheEnginesPerClass() {
  assertEqual(summarizeCasterLevel([held('class:magus', 'Magus', 7)], served(7)).display, '7', 'Magus 7: CL 7');
  assertEqual(summarizeCasterLevel([held('class:wizard', 'Wizard', 7)], served(7)).display, '7', 'Wizard 7: CL 7');
  assertEqual(summarizeCasterLevel([held('class:paladin', 'Paladin', 7)], served(7)).display, '4', 'Paladin 7: CL 4 (level - 3)');
  assertEqual(summarizeCasterLevel([held('class:paladin', 'Paladin', 1)], served(1)).display, '—', 'Paladin 1 casts nothing yet');
  assertEqual(summarizeCasterLevel([held('class:samurai', 'Samurai', 7)], served(7)).display, '—', 'Samurai casts nothing');
  assertEqual(summarizeCasterLevel([held('class:psion', 'Psion', 7)], served(7)).display, '7', 'Psion 7: manifester level 7');
  // F6 merge-readiness B1: the Bloodrager's caster-level rule opens at Bloodrager level 4
  // (acg_classes.lst:44); the engine's chassis prints 0 below it.
  assertEqual(summarizeCasterLevel([held('class:bloodrager', 'Bloodrager', 1)], served(1)).display, '—', 'Bloodrager 1 casts nothing yet');
  assertEqual(summarizeCasterLevel([held('class:bloodrager', 'Bloodrager', 7)], served(7)).display, '7', 'Bloodrager 7: CL 7');
}

/** PF1 caster level is per class: a Wizard 7 / Cleric 1 prints both, never a summed 8. */
function verifiesMulticlassCasterLevelIsPerClass() {
  const state = loadedClassFacts({
    classes: [
      ...classFactsWire().classes.filter((facts) => facts.classId === 'class:wizard' && facts.level === 7),
      ...classFactsWire().classes.filter((facts) => facts.classId === 'class:cleric' && facts.level === 1),
    ],
  });
  const summary = summarizeCasterLevel([held('class:wizard', 'Wizard', 7), held('class:cleric', 'Cleric', 1)], state);
  assertEqual(summary.display, 'Wizard 7 / Cleric 1', 'per-class caster levels');
}

/** Every roster class answers a caster level at levels 1 and 7 (denominator: 59 x 2 served rows). */
function verifiesEveryRosterClassAnswersACasterLevel() {
  const rows = classFactsWire().classes;
  assertEqual(rows.length, 118, '59 roster classes x levels 1 and 7');
  const unknown = rows.filter((facts) => facts.casterLevel.status === 'unknown').map((facts) => `${facts.classId}@${facts.level}`);
  assertEqual(unknown.length, 0, `caster level Unknown: ${unknown.join(', ')}`);
  const weaponUnknown = rows.filter((facts) => facts.weaponProficiency.status === 'unknown').map((facts) => facts.classId);
  assertEqual(weaponUnknown.length, 0, `weapon proficiency Unknown: ${weaponUnknown.join(', ')}`);
}

function verifiesLoadingFailureAndUnservedAreUnknownNeverDefaults() {
  const fighter = [held('class:fighter', 'Fighter', 3)];
  assertEqual(summarizeCasterLevel(fighter, LOADING_CLASS_FACTS).display, '…', 'loading');
  assertEqual(tier(LOADING_CLASS_FACTS, fighter, 'Martial'), 'unknown', 'no Martial guess while loading');
  const failed = failedClassFacts(new Error('no runtime'));
  assertEqual(classFactsNotice(failed), 'class facts unavailable: no runtime', 'visible notice on failure');
  assertEqual(summarizeCasterLevel(fighter, failed).display, 'Unknown', 'failure: caster level Unknown');
  assertEqual(tier(failed, fighter, 'Simple'), 'unknown', 'failure: Simple Unknown, not a ✓ default');
  // A level the wire was not asked for is not served: Unknown, not the level-1 answer.
  assertEqual(summarizeCasterLevel([held('class:wizard', 'Wizard', 3)], served(1)).display, 'Unknown', 'unserved level');
  assert(
    summarizeWeaponProficiency([held('class:no_such', 'Nobody', 1)], served(1)).unknown[0].startsWith('Nobody: '),
    'an unserved class is named'
  );
}

function verifiesQueriesAskForEachHeldClassAtItsLevel() {
  assertEqual(
    JSON.stringify(classFactsQueries([held('class:fighter', 'Fighter', 3), held('class:wizard', 'Wizard', 1)])),
    JSON.stringify([
      { classId: 'class:fighter', level: 3 },
      { classId: 'class:wizard', level: 1 },
    ]),
    'one query per held class'
  );
}

async function main() {
  verifiesNewlyOfferedMartialClassesAreMartial();
  verifiesANamedListClassPrintsItsWeapons();
  verifiesProficiencyIsTheUnionAcrossClasses();
  verifiesCasterLevelIsTheEnginesPerClass();
  verifiesMulticlassCasterLevelIsPerClass();
  verifiesEveryRosterClassAnswersACasterLevel();
  verifiesLoadingFailureAndUnservedAreUnknownNeverDefaults();
  verifiesQueriesAskForEachHeldClassAtItsLevel();
  verifiesNoRosterClassPrintsAPseudoWeapon();
}

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
