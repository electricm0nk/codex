import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ClassOption } from './characterHubModel';
import { LevelsPanel } from './LevelsPanel';
import { addLevel, rerollLevel, type CreationLevel } from './levelsModel';

const option = (id: string, label: string, hitDie: number | null, max = 20): ClassOption => ({
  id,
  label,
  supportLevel: 'full',
  levelOptions: Array.from({ length: max }, (_, i) => i + 1),
  hitDie,
});
const FIGHTER = option('class:fighter', 'Fighter', 10);
const WIZARD = option('class:wizard', 'Wizard', 6);

const render = (levels: CreationLevel[], con = 2, options: ClassOption[] = [FIGHTER, WIZARD]) =>
  renderToStaticMarkup(
    createElement(LevelsPanel, { classOptions: options, levels, constitutionModifier: con, onAdd: () => {}, onReroll: () => {}, onRemove: () => {} }),
  );

function verifiesAnEmptyListSaysSoAndStillOffersTheClasses() {
  const html = render([]);
  assert(html.includes('No levels yet'), 'an empty list says so');
  assert(html.includes('id="character-class"'), 'the class dropdown is present');
  assert(html.includes('Fighter') && html.includes('Wizard'), 'the classes are offered');
  assert(html.includes('Add level'), 'there is an Add level button');
}

function verifiesTheHeaderShowsLevelAndHitPointTotals() {
  const levels = addLevel(addLevel([], 'class:fighter', 10), 'class:wizard', 6); // 10 and 4
  const html = render(levels, 2);
  assert(html.includes('Level 2'), 'the character level');
  assert(html.includes('HP 18'), 'the hit point total: (10+2) + (4+2)');
}

function verifiesEachRowShowsItsClassDieAndHitPoints() {
  const levels = addLevel(addLevel([], 'class:fighter', 10), 'class:wizard', 6);
  const html = render(levels, 2);
  assertEqual((html.match(/data-level-row/g) ?? []).length, 2, 'one row per level');
  assert(html.includes('d10') && html.includes('d6'), 'each row names its die');
  assert(html.includes('10 + 2') && html.includes('4 + 2'), 'each row shows die result plus Constitution');
}

function verifiesTagsAndRerollAvailability() {
  const base = addLevel(addLevel(addLevel([], 'class:fighter', 10), 'class:fighter', 10), 'class:fighter', 10);
  const levels = rerollLevel(base, 2, () => 0.2);
  const html = render(levels, 0);
  assert(html.includes('>Max<'), 'level 1 is tagged Max');
  assert(html.includes('>Average<'), 'an untouched later level is tagged Average');
  assert(html.includes('>Rolled<'), 'a rerolled level is tagged Rolled');
  assert(!html.includes('aria-label="Reroll level 1"'), 'level 1 has no reroll');
  assert(html.includes('aria-label="Reroll level 2"') && html.includes('aria-label="Reroll level 3"'), 'later levels can be rerolled');
  assert(html.includes('aria-label="Remove level 1"'), 'any level can be removed');
}

function verifiesAddIsDisabledWithTheReasonWhenTheClassIsMaxedOut() {
  const only = option('class:fighter', 'Fighter', 10, 2);
  const levels = addLevel(addLevel([], 'class:fighter', 10), 'class:fighter', 10);
  const html = render(levels, 0, [only]);
  assert(/id="add-level-button"[^>]*disabled/.test(html) || /disabled=""[^>]*id="add-level-button"/.test(html), 'Add level is disabled');
  assert(html.includes('highest level'), 'the reason is shown');
}

function verifiesTheRulesAreExplainedBriefly() {
  const html = render(addLevel([], 'class:fighter', 10));
  assert(html.includes('full hit die'), 'level 1 rule is stated');
  assert(html.includes('average'), 'the later-level default is stated');
}

verifiesAnEmptyListSaysSoAndStillOffersTheClasses();
verifiesTheHeaderShowsLevelAndHitPointTotals();
verifiesEachRowShowsItsClassDieAndHitPoints();
verifiesTagsAndRerollAvailability();
verifiesAddIsDisabledWithTheReasonWhenTheClassIsMaxedOut();
verifiesTheRulesAreExplainedBriefly();
console.log('LevelsPanel.test.ts: all assertions passed');
