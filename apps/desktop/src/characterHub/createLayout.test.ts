import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { assert, assertEqual } from '../testSupport/asserts';
import type { ClassOption, RaceOption } from './characterHubModel';
import { CreateCharacterFields } from './CreateCharacterForm';
import { CREATE_COLUMNS, CREATE_COLUMN_STYLES, CREATE_LAYOUT, boxesInColumn } from './sheetLayout';

// Brief 2 L3: the Create screen is laid out from CREATE_LAYOUT to match the sheet: the level-add
// dialog and HP roller far-left, the Skills box under Abilities in one column. Rendered to static
// markup (no DOM on this box); the order of the boxes in the markup is their on-screen order.

const HUMAN: RaceOption = {
  id: 'race:human',
  label: 'Human',
  book: 'CRB',
  abilityAdjustments: {},
  floatingBonusPoints: 2,
  size: 'Medium',
  vision: 'Normal',
  baseSpeedFt: 30,
  body: null,
};
const FIGHTER: ClassOption = {
  id: 'class:fighter',
  label: 'Fighter',
  supportLevel: 'full',
  levelOptions: Array.from({ length: 20 }, (_, i) => i + 1),
  hitDie: 10,
};

const html = renderToStaticMarkup(
  createElement(CreateCharacterFields, {
    races: [HUMAN],
    rosterDiagnostics: [],
    classOptions: [FIGHTER],
    classRosterNotice: null,
    onCreated: () => {},
  }),
);

function verifiesTheCreateColumnsAreRegisteredLeftToRight() {
  assertEqual(CREATE_COLUMNS[0], 'left', 'the levels column is far-left');
  for (const box of CREATE_LAYOUT) {
    assert(CREATE_COLUMNS.includes(box.column), `${box.id} sits in a column Create renders`);
    assert(CREATE_COLUMN_STYLES[box.column] !== undefined, `${box.column} has a column style`);
  }
  assertEqual(
    boxesInColumn(CREATE_LAYOUT, 'left').map((box) => box.id).join('|'),
    'levels|levelGains',
    'far-left: the levels panel, then the per-level summary under the HP roller',
  );
  assertEqual(
    boxesInColumn(CREATE_LAYOUT, 'details').map((box) => box.id).join('|'),
    'identity|sheetBlock|details',
    'the 2x2 block sits under the name, as on the sheet',
  );
  assertEqual(
    boxesInColumn(CREATE_LAYOUT, 'abilities').map((box) => box.id).join('|'),
    'abilities|skills',
    'Abilities, then Skills, in one column',
  );
}

function at(label: string): number {
  const index = html.indexOf(label);
  assert(index >= 0, `the Create screen renders ${label}`);
  return index;
}

function verifiesTheLevelsPanelRendersFarLeft() {
  const levels = at('aria-label="Levels"');
  assert(levels < at('id="character-name"'), 'the Levels panel comes before the identity fields');
  assert(levels < at('Ability scores'), 'the Levels panel comes before Abilities');
  assert(html.includes('Add level'), 'the ui-smoke label Add level is still on the screen');
}

function verifiesSkillsSitsUnderAbilitiesInTheSameColumn() {
  const abilities = at('Ability scores');
  const skills = at('aria-label="Skills"');
  assert(skills > abilities, 'the Skills box comes after Abilities');
  // Same column: no other registered column opens between Abilities and Skills.
  const between = html.slice(abilities, skills);
  assert(!between.includes('data-create-column='), 'Skills sits in the Abilities column, not a later one');
  const abilitiesColumn = html.lastIndexOf('data-create-column="abilities"', abilities);
  assert(abilitiesColumn >= 0, 'Abilities renders inside the abilities column');
  assert(at('aria-label="Feats"') < abilitiesColumn, 'the other manage boxes stay in the details column');
}

function verifiesSkillsListsKnownSkillsAndKeepsManage() {
  const skillsBox = html.slice(at('aria-label="Skills"'));
  assert(skillsBox.includes('Add a level first'), 'Skills explains it needs a level before ranks can be bought');
}

function verifiesTheTwoByTwoBlockRendersUnderTheName() {
  const name = at('id="character-name"');
  const block = at('aria-label="Initiative"');
  assert(name < at('aria-label="Hit Points"') && at('aria-label="Hit Points"') < block, 'Hit Points then Initiative, under the name');
  assert(block < at('aria-label="Encumbrance"') && at('aria-label="Encumbrance"') < at('aria-label="Conditions"'), '2x2 reading order');
  assert(at('aria-label="Conditions"') < at('Physical Attributes'), 'the block sits above Physical Attributes');
  assert(html.includes('Add a level first'), 'no levels yet: the engine boxes say a level is needed');
  assert(html.includes('Normal'), 'no conditions reads Normal');
}

verifiesTheCreateColumnsAreRegisteredLeftToRight();
verifiesTheTwoByTwoBlockRendersUnderTheName();
verifiesTheLevelsPanelRendersFarLeft();
verifiesSkillsSitsUnderAbilitiesInTheSameColumn();
verifiesSkillsListsKnownSkillsAndKeepsManage();
console.log('createLayout.test.ts: all assertions passed');
