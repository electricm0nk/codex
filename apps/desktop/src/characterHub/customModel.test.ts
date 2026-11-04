import { assert, assertEqual } from '../testSupport/asserts';
import { sellableCopies } from '../boundary/listHeldEquipment';
import { SKILLS, skillIdFor } from './skillsModel';
import {
  BONUS_TARGETS,
  BONUS_TYPES,
  EMPTY_CUSTOM,
  bonusLine,
  customItemPlace,
  customIsEmpty,
  customOverrides,
  NO_OVERRIDES,
  OVERRIDE_TOGGLES,
  withOverride,
  customProblems,
  customSummaryLines,
  grantTargetLabel,
  grantTotals,
  newBonus,
  newGrant,
  newRecord,
  withCustomHitPoints,
  withItemHitPoints,
  savedIntelligenceModifier,
  withItemSpeed,
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
  assertEqual(EMPTY_CUSTOM.conditions.length, 0, 'a new character has no conditions (the sheet shows Normal)');
  const custom: CharacterCustom = {
    ...withGrants({ ...grant('g', 'ability:wisdom', 1), label: 'Boon of Pharasma' }),
    feats: [{ ...newRecord('f'), name: 'Racial Weapon Focus' }],
  };
  assertEqual(customIsEmpty(custom), false, 'not empty');
  assertEqual(customSummaryLines(custom).join('|'), 'Boon of Pharasma: +1 Wisdom score|Feat: Racial Weapon Focus', 'one line per entry');
}


function verifiesTheBonusFormOffersTheRowsVocabulary() {
  const targets = BONUS_TARGETS.map((option) => option.value);
  for (const flat of ['attack', 'damage', 'ac', 'speed', 'initiative', 'hit_points', 'cmb', 'cmd', 'save:fortitude', 'save:reflex', 'save:will', 'ability:charisma']) {
    assert(targets.includes(flat), `${flat} is offered`);
  }
  for (const skill of SKILLS) {
    assert(targets.includes(skillIdFor(skill.name)), `${skill.name} is offered`);
  }
  assertEqual(new Set(targets).size, targets.length, 'no target twice');
  assert(BONUS_TYPES.includes('Deflection') && BONUS_TYPES.includes('Untyped'), 'the card names deflection');
  assertEqual(newBonus().value, 1, 'a new bonus is a number the form can save');
}

function verifiesABonusReadsAsThePlayerSaysIt() {
  assertEqual(bonusLine({ target: 'ac', bonusType: 'Deflection', value: 1 }), '+1 deflection to AC', 'a typed bonus');
  assertEqual(bonusLine({ target: 'skill:sense_motive', bonusType: 'Untyped', value: -2 }), '-2 to Sense Motive', 'untyped says no type');
  assertEqual(bonusLine({ target: 'ability:strength', bonusType: 'NaturalArmor', value: 2 }), '+2 natural armor to Strength score', 'two words');
}

function verifiesWhereACustomItemIs() {
  const blank = newRecord('a');
  assertEqual(customItemPlace(blank), null, 'a text-only record has no place: it is unchanged');
  assertEqual(customItemPlace({ ...blank, bonuses: [newBonus()] }), 'carried', 'it starts carried');
  assertEqual(customItemPlace({ ...blank, slot: 'head' }), 'carried', 'a slot alone is held');
  assertEqual(customItemPlace({ ...blank, slot: 'head', worn: true }), 'worn', 'worn says worn');
}

function verifiesTheBonusProblemsMatchTheBackend() {
  const item = (bonus: { target: string; bonusType: string; value: number }) => ({ ...EMPTY_CUSTOM, equipment: [{ ...newRecord('a'), name: 'Hat', bonuses: [bonus] }] });
  assertEqual(customProblems(item({ target: 'ac', bonusType: 'Deflection', value: 1 })).length, 0, 'the card\'s item saves');
  assert(customProblems(item({ target: 'ac', bonusType: 'Deflection', value: 0 }))[0].includes('Hat'), 'a zero is named');
  assertEqual(customProblems(item({ target: 'ac', bonusType: 'Deflection', value: 101 })).length, 1, 'past the range');
  assertEqual(customProblems(item({ target: 'ac', bonusType: 'Deflection', value: 1.5 })).length, 1, 'not whole');
}

function verifiesTheTwoTotalsTheSheetHoldsTakeTheEngineRow() {
  const rows = [
    { id: 'custom_item.speed', value: 10, detail: '' },
    { id: 'custom_item.hit_points', value: 7, detail: '' },
  ];
  assertEqual(savedIntelligenceModifier(5, [{ id: 'ability_modifier.intelligence.saved', value: 4, detail: '' }]), 4, 'a worn item does not move the budget');
  assertEqual(savedIntelligenceModifier(4, []), 4, 'no row, the sheet\'s modifier');
  assertEqual(withItemSpeed('30 ft.', rows), '40 ft.', 'the row moves it');
  assertEqual(withItemSpeed('30 ft.', []), '30 ft.', 'no row, no change');
  assertEqual(withItemSpeed('Unknown', rows), 'Unknown', 'an unknown speed stays unknown');
  assertEqual(withItemSpeed('5 ft.', [{ id: 'custom_item.speed', value: -10, detail: '' }]), '0 ft.', 'never below 0');
  const halved = (feet: number) => [{ id: 'custom_item.speed_percent', value: 50, detail: '' }, { id: 'custom_item.speed', value: feet, detail: '' }];
  assertEqual(withItemSpeed('30 ft.', halved(0)), '15 ft.', 'a Loadstone halves the land speed');
  assertEqual(withItemSpeed('30 ft.', halved(10)), '20 ft.', 'and what an item adds to it');
  assertEqual(withItemSpeed('25 ft.', halved(0)), '12 ft.', 'whole feet, rounded down');
  assertEqual(withItemSpeed('Unknown', halved(10)), 'Unknown', 'an unknown speed stays unknown');
  assertEqual(withItemHitPoints(20, rows), 27, 'the row moves it');
  assertEqual(withItemHitPoints(null, rows), null, 'an unknown total stays unknown');
  assertEqual(withItemHitPoints(20, []), 20, 'no row, no change');
}

function verifiesTheMarketCannotSellACustomItem() {
  const copy = (itemId: string) => ({ itemId, name: itemId, bonus: 0 });
  assertEqual(sellableCopies([copy('item:longsword'), copy('custom:a')]).map((row) => row.itemId).join(), 'item:longsword', 'the custom row is off the Sell list');
}

function verifiesTheOverridesGroupHoldsTheBackendsEightFlags() {
  assertEqual(OVERRIDE_TOGGLES.length, 8, 'eight toggles');
  assertEqual(new Set(OVERRIDE_TOGGLES.map((toggle) => toggle.key)).size, 8, 'each flag once');
  assertEqual(OVERRIDE_TOGGLES.map((toggle) => toggle.key).sort().join(), Object.keys(NO_OVERRIDES).sort().join(), 'every flag has its toggle');
  assertEqual(customOverrides(EMPTY_CUSTOM).ignoreSlotLimits, false, 'data saved before the flags reads all off');
  const on = withOverride(EMPTY_CUSTOM, 'ignoreSlotLimits', true);
  assertEqual(customOverrides(on).ignoreSlotLimits, true, 'the flag is set');
  assertEqual(customOverrides(on).ignoreEncumbrance, false, 'and no other');
  assertEqual(customOverrides(withOverride(on, 'ignoreSlotLimits', false)).ignoreSlotLimits, false, 'and clears');
  assertEqual(customIsEmpty(on), true, 'a flag alone prints no Custom page');
}

verifiesGrantTotalsSumPerTarget();
verifiesTheSheetNumbersFollowTheGrants();
verifiesBlankEntriesAreNamedProblemsNotSilentlyDropped();
verifiesLabelsAndSummaries();
verifiesTheBonusFormOffersTheRowsVocabulary();
verifiesABonusReadsAsThePlayerSaysIt();
verifiesWhereACustomItemIs();
verifiesTheBonusProblemsMatchTheBackend();
verifiesTheTwoTotalsTheSheetHoldsTakeTheEngineRow();
verifiesTheMarketCannotSellACustomItem();
verifiesTheOverridesGroupHoldsTheBackendsEightFlags();
console.log('customModel.test.ts: all assertions passed');
