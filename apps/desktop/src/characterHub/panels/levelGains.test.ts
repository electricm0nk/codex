import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../../testSupport/asserts';
import { classRosterWire } from '../../testSupport/classRosterWire';
import { installClassRoster } from '../classRoster';
import { buildLevelEntries, describeLevelGains, type HeldClass } from '../characterProgression';
import { LEVEL_FEATURES_LOADING, LevelGainsPanel, loadLevelFeatures } from './LevelGains';

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

// Brief 2 L4b: each line also prints the engine's class features for that class level
// (`draft_level_up_preview`, one call per line), verbatim; loading and refusals say so.

function verifiesEachLineAppendsTheEnginesClassFeatures() {
  const html = renderToStaticMarkup(
    createElement(LevelGainsPanel, {
      heldClasses: MONK_2_FIGHTER_2,
      intelligenceModifier: 1,
      isHuman: false,
      classFeatures: { kind: 'ready', byLevel: { 1: ['flurry of blows'], 2: ['evasion'], 3: ['Fighter base_attack_bonus at level 1'], 4: [] } },
    }),
  );
  assert(html.includes('Monk 1: +1 feat, +5 skills; class features: flurry of blows'), html);
  assert(html.includes('Monk 2: +5 skills; class features: evasion'), html);
  assert(html.includes('Fighter 1: +1 feat, +3 skills; class features: Fighter base_attack_bonus at level 1'), html);
  assert(html.includes('Fighter 2: +3 skills, +1 ability score; class features: none reported by the engine'), 'an empty engine answer is printed as such');
}

function verifiesLoadingAndRefusalAreShownNotGuessed() {
  const loading = renderToStaticMarkup(
    createElement(LevelGainsPanel, { heldClasses: MONK_2_FIGHTER_2, intelligenceModifier: 1, isHuman: false, classFeatures: { kind: 'loading' } }),
  );
  assert(loading.includes(LEVEL_FEATURES_LOADING), 'loading is said');
  assert(!loading.includes('class features:'), 'no feature text before the engine answers');
  const refused = renderToStaticMarkup(
    createElement(LevelGainsPanel, {
      heldClasses: MONK_2_FIGHTER_2,
      intelligenceModifier: 1,
      isHuman: false,
      classFeatures: { kind: 'error', message: 'Failed to load the draft level-up preview: no race' },
    }),
  );
  assert(refused.includes('role="note"') && refused.includes('Failed to load the draft level-up preview: no race'), 'the refusal is shown');
  assert(refused.includes('Monk 1: +1 feat, +5 skills<'), 'the universal gains still print');
}

async function verifiesTheLoaderAsksOnceperLineWithThePreviousClassLevel() {
  const calls: string[] = [];
  const byLevel = await loadLevelFeatures(buildLevelEntries(MONK_2_FIGHTER_2), async (classId, fromLevel) => {
    calls.push(`${classId}@${fromLevel}`);
    return { fromLevel, toLevel: fromLevel + 1, automaticFeatures: [`${classId} ${fromLevel + 1}`] };
  });
  assertEqual(calls.join(','), 'class:monk@0,class:monk@1,class:fighter@0,class:fighter@1', 'one call per line, from the class level before');
  assertEqual(byLevel[3].join(), 'class:fighter 1', 'keyed by character level');
  let thrown: unknown;
  try {
    await loadLevelFeatures(buildLevelEntries(MONK_2_FIGHTER_2), async () => {
      throw new Error('refused');
    });
  } catch (cause) {
    thrown = cause;
  }
  assert(thrown instanceof Error && thrown.message === 'refused', 'a refused call refuses the whole answer');
}

verifiesEachLevelReadsFeatSkillsAndAbilityIncrease();
verifiesEachLineAppendsTheEnginesClassFeatures();
verifiesLoadingAndRefusalAreShownNotGuessed();
verifiesHumanBonusAndFloor();
verifiesAnUnknownClassSaysUnknownSkills();
verifiesThePanelListsOneLinePerLevelInOrder();
verifiesThePanelIsEmptyWithoutLevels();
verifiesTheLoaderAsksOnceperLineWithThePreviousClassLevel()
  .then(() => console.log('levelGains.test.ts: all assertions passed'))
  .catch((error: unknown) => {
    console.error(error);
    process.exitCode = 1;
  });
