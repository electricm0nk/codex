import { assert, assertEqual } from '../testSupport/asserts';
import {
  EMPTY_CUSTOM,
  customIsEmpty,
  customProblems,
  customSummaryLines,
  grantTargetLabel,
  grantTotals,
  newGrant,
  newRecord,
  withCustomHitPoints,
  withCustomSkillPoints,
  type CharacterCustom,
} from './customModel';

const withGrants = (...grants: CharacterCustom['grants']): CharacterCustom => ({ ...EMPTY_CUSTOM, grants });
const grant = (id: string, target: string, value: number) => ({ ...newGrant(id), label: `g-${id}`, target, value });

function verifiesGrantTotalsSumPerTarget() {
  const totals = grantTotals(withGrants(grant('a', 'ability:wisdom', 1), grant('b', 'ability:wisdom', 2), grant('c', 'hit_points', 5), grant('d', 'skill_points', 3), grant('e', 'hit_points', -1)));
  assertEqual(totals.abilities.wisdom, 3, 'ability grants add');
  assertEqual(totals.abilities.strength, 0, 'untouched abilities stay zero');
  assertEqual(totals.hitPoints, 4, 'hit point grants add, including a penalty');
  assertEqual(totals.skillPoints, 3, 'skill point grants add');
}

function verifiesTheSheetNumbersFollowTheGrants() {
  const custom = withGrants(grant('c', 'hit_points', 5), grant('d', 'skill_points', 3));
  assertEqual(withCustomHitPoints(20, custom), 25, 'max hit points include the grant');
  assertEqual(withCustomHitPoints(null, custom), null, 'an unknown total stays unknown');
  assertEqual(withCustomSkillPoints(8, custom), 11, 'the skill pool includes the grant');
  assertEqual(withCustomSkillPoints(null, custom), null, 'an unknown pool stays unknown');
  assertEqual(withCustomHitPoints(20, EMPTY_CUSTOM), 20, 'no grants, no change');
  assertEqual(withCustomSkillPoints(3, withGrants(grant('x', 'skill_points', -9))), 0, 'the pool never goes below zero');
}

function verifiesBlankEntriesAreNamedProblemsNotSilentlyDropped() {
  assertEqual(customProblems(EMPTY_CUSTOM).length, 0, 'empty is fine');
  assertEqual(customProblems({ ...EMPTY_CUSTOM, feats: [newRecord('f1')] }).length, 1, 'a record with no name is a problem');
  assert(customProblems({ ...EMPTY_CUSTOM, feats: [newRecord('f1')] })[0]!.includes('feat'), 'and says which kind');
  assertEqual(customProblems(withGrants({ ...newGrant('g'), value: 1 })).length, 1, 'a grant with no label is a problem');
  assertEqual(customProblems(withGrants({ ...grant('g', 'ability:wisdom', 99) })).length, 1, 'a grant over the limit is a problem');
  const stat = { ...newRecord('e1'), name: 'Thing', stats: [{ label: '', value: '1d8' }] };
  assertEqual(customProblems({ ...EMPTY_CUSTOM, equipment: [stat] }).length, 1, 'a stat line with no label is a problem');
}

function verifiesLabelsAndSummaries() {
  assertEqual(grantTargetLabel('ability:wisdom'), 'Wisdom score', 'ability targets read as scores');
  assertEqual(grantTargetLabel('hit_points'), 'Hit points', 'hit points');
  assertEqual(grantTargetLabel('skill_points'), 'Skill points', 'skill points');
  assertEqual(customIsEmpty(EMPTY_CUSTOM), true, 'empty is empty');
  const custom: CharacterCustom = {
    ...withGrants({ ...grant('g', 'ability:wisdom', 1), label: 'Boon of Pharasma' }),
    feats: [{ ...newRecord('f'), name: 'Racial Weapon Focus' }],
  };
  assertEqual(customIsEmpty(custom), false, 'not empty');
  assertEqual(customSummaryLines(custom).join('|'), 'Boon of Pharasma: +1 Wisdom score|Feat: Racial Weapon Focus', 'one line per entry');
}

verifiesGrantTotalsSumPerTarget();
verifiesTheSheetNumbersFollowTheGrants();
verifiesBlankEntriesAreNamedProblemsNotSilentlyDropped();
verifiesLabelsAndSummaries();
console.log('customModel.test.ts: all assertions passed');
