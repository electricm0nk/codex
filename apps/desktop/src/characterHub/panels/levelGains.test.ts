import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../../testSupport/asserts';
import { classRosterWire } from '../../testSupport/classRosterWire';
import { installClassRoster } from '../classRoster';
import { buildLevelEntries, describeLevelGains, type HeldClass } from '../characterProgression';
import { LevelGainsPanel } from './LevelGains';

// Brief 2 L4: the read-only per-level summary under the Create screen's HP roller,
// "Monk 1: +1 feat, +N skills, ...". Feat, skills and ability increase come from
// buildLevelEntries (the universal PF1 benefits + the roster's skill ranks per level).

installClassRoster(classRosterWire());

const MONK_2_FIGHTER_2: HeldClass[] = [
  { classId: 'class:monk', classLabel: 'Monk', level: 2 },
  { classId: 'class:fighter', classLabel: 'Fighter', level: 2 },
];

function verifiesEachLevelReadsFeatSkillsAndAbilityIncrease() {
  const entries = buildLevelEntries(MONK_2_FIGHTER_2);
  // Monk skill ranks 4 (roster wire), Fighter 2; Int modifier +1, not human.
  const lines = entries.map((entry) => describeLevelGains(entry, 1, false));
  assertEqual(lines[0], 'Monk 1: +1 feat, +5 skills', 'character level 1 grants a feat');
  assertEqual(lines[1], 'Monk 2: +5 skills', 'even level: no feat');
  assertEqual(lines[2], 'Fighter 1: +1 feat, +3 skills', 'character level 3 grants a feat; class label resets to Fighter 1');
  assertEqual(lines[3], 'Fighter 2: +3 skills, +1 ability score', 'character level 4 grants an ability score increase');
}

function verifiesHumanBonusAndFloor() {
  const [first] = buildLevelEntries([{ classId: 'class:fighter', classLabel: 'Fighter', level: 1 }]);
  assertEqual(describeLevelGains(first, 0, true), 'Fighter 1: +1 feat, +3 skills', 'a human gains one more skill rank');
  assertEqual(describeLevelGains(first, -4, false), 'Fighter 1: +1 feat, +1 skills', 'skill ranks floor at 1');
}

function verifiesAnUnknownClassSaysUnknownSkills() {
  const [first] = buildLevelEntries([{ classId: 'class:not_in_roster', classLabel: 'Mystery', level: 1 }]);
  assertEqual(describeLevelGains(first, 0, false), 'Mystery 1: +1 feat, skills Unknown', 'no guessed skill ranks');
}

function verifiesThePanelListsOneLinePerLevelInOrder() {
  const html = renderToStaticMarkup(
    createElement(LevelGainsPanel, { heldClasses: MONK_2_FIGHTER_2, intelligenceModifier: 1, isHuman: false }),
  );
  assert(html.includes('aria-label="Level gains"'), 'the panel is labelled');
  const order = ['Monk 1: +1 feat', 'Monk 2: +5 skills', 'Fighter 1: +1 feat', 'Fighter 2: +3 skills, +1 ability score'].map((line) =>
    html.indexOf(line),
  );
  assert(order.every((at) => at >= 0), `every level line renders, got ${order.join(',')}`);
  assert(order.every((at, i) => i === 0 || at > order[i - 1]), 'lines run in character-level order');
  assert(!html.includes('<button'), 'informational only: no controls');
}

function verifiesThePanelIsEmptyWithoutLevels() {
  const html = renderToStaticMarkup(createElement(LevelGainsPanel, { heldClasses: [], intelligenceModifier: 0, isHuman: false }));
  assertEqual(html, '', 'no levels, no summary');
}

verifiesEachLevelReadsFeatSkillsAndAbilityIncrease();
verifiesHumanBonusAndFloor();
verifiesAnUnknownClassSaysUnknownSkills();
verifiesThePanelListsOneLinePerLevelInOrder();
verifiesThePanelIsEmptyWithoutLevels();
console.log('levelGains.test.ts: all assertions passed');
