import {
  EMPTY_STARFINDER_DRAFT,
  adjustPoints,
  blockingProblems,
  buildStarfinderCreationRequest,
  keyAbilityAfterClassChange,
  picksForSlot,
  reconcilePicks,
  setSlotPick,
  starfinderOutcomeTiles,
  type StarfinderCreationDraft,
} from './starfinderCreationModel';
import { characterCreationGate } from './characterHubRuntime';
import type { SfCreationPreviewDto, SfCreationSlotDto } from '../boundary/starfinderCreation';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-37 E6.2: the Starfinder creation flow (race -> theme -> class -> point buy). Every
 * number the form shows comes from the engine's preview (`preview_starfinder_character`):
 * the point budget, the creation cap, each ability score and its terms. These tests pin the
 * pure pieces the form composes: the request it sends, the point buttons (bounded by the
 * engine's budget, not a desktop table), the picks it keeps across preview rounds, and the
 * outcome tiles (only `sf.*` rows the engine returned; nothing fabricated).
 */

const RULES = { budget: 10, maxScoreAtCreation: 18, source: 'SRD Buying Ability Scores' };

function slot(partial: Partial<SfCreationSlotDto> & { slotId: string }): SfCreationSlotDto {
  return {
    label: partial.slotId,
    kind: 'option',
    count: 1,
    repeatable: false,
    options: [],
    defaultOptionId: null,
    required: false,
    ...partial,
  };
}

function testStarfinderCreationOpensTheStarfinderForm() {
  const gate = characterCreationGate('starfinder-1e');
  assertEqual(gate.enabled, true, 'New Character opens for Starfinder 1e');
  assertEqual(gate.form, 'starfinder', 'Starfinder opens the Starfinder flow, never the Pathfinder form');
  assertEqual(characterCreationGate('pathfinder-1e').form, 'pathfinder', 'Pathfinder keeps its own form');
  assertEqual(characterCreationGate('traveller').enabled, false, 'a rule set with no adapter stays closed');
}

function testTheRequestCarriesEveryChoiceAndAFreshIdentity() {
  const draft: StarfinderCreationDraft = {
    ...EMPTY_STARFINDER_DRAFT,
    displayLabel: '  Vex  ',
    raceId: 'core:race:human',
    themeId: 'core:ability:mercenary',
    classId: 'core:class:soldier',
    keyAbility: 'STR',
    pointBuy: { ...EMPTY_STARFINDER_DRAFT.pointBuy, strength: 3, dexterity: 4 },
    picks: [{ slotId: 'core:ability:2_racial_stat_bonus', optionId: 'STR' }],
    choices: {
      feats: ['core:feat:quick_draw'],
      featPicks: [],
      spells: [],
      gear: [{ itemId: 'core:equipment:baton_tactical', equipped: false, modifiers: [] }],
    },
  };
  const request = buildStarfinderCreationRequest(draft, { generateId: () => 'id-1', now: () => '2026-10-05T00:00:00Z' });
  assertEqual(request.choices?.feats.join(','), 'core:feat:quick_draw', 'the feats chosen at creation (SD-37 E6.5a)');
  assertEqual(request.choices?.gear[0]?.itemId, 'core:equipment:baton_tactical', 'the gear chosen at creation');
  assert(request.choices !== draft.choices, 'the request carries a copy, not the draft itself');
  assertEqual(request.characterId, 'id-1', 'a fresh character id');
  assertEqual(request.savedAt, '2026-10-05T00:00:00Z', 'the save time');
  assertEqual(request.displayLabel, 'Vex', 'the name is trimmed');
  assertEqual(request.raceId, 'core:race:human', 'race');
  assertEqual(request.themeId, 'core:ability:mercenary', 'theme');
  assertEqual(request.classId, 'core:class:soldier', 'class');
  assertEqual(request.keyAbility, 'STR', 'key ability');
  assertEqual(request.pointBuy.strength + request.pointBuy.dexterity, 7, 'points spent');
  assertEqual(request.picks.length, 1, 'picks');
  const unset = buildStarfinderCreationRequest(EMPTY_STARFINDER_DRAFT, { generateId: () => 'id-2', now: () => 'now' });
  assertEqual(unset.raceId, null, 'an unchosen race is sent as null, never a guessed default');
}

function testThePointButtonsStayInsideTheEnginesBudget() {
  const spent = { ...EMPTY_STARFINDER_DRAFT.pointBuy, strength: 9 };
  const one = adjustPoints(spent, 'dexterity', 1, RULES);
  assertEqual(one.dexterity, 1, 'the 10th point is spent');
  assertEqual(adjustPoints(one, 'charisma', 1, RULES).charisma, 0, 'an 11th point is not: the budget is the engine\'s 10');
  assertEqual(adjustPoints(spent, 'wisdom', -1, RULES).wisdom, 0, 'no ability goes below 0 points');
  assertEqual(adjustPoints(spent, 'strength', -1, RULES).strength, 8, 'a spent point comes back');
  const bigger = adjustPoints(spent, 'charisma', 2, { ...RULES, budget: 12 });
  assertEqual(bigger.charisma, 2, 'a different engine budget moves the bound (no desktop constant)');
}

function testAClassWithOneKeyAbilitySettlesItAndAChoiceWaits() {
  const classes = [
    { id: 'core:class:envoy', label: 'Envoy', keyAbilityOptions: [{ id: 'CHA', label: 'Charisma' }] },
    { id: 'core:class:soldier', label: 'Soldier', keyAbilityOptions: [{ id: 'STR', label: 'Strength' }, { id: 'DEX', label: 'Dexterity' }] },
  ];
  assertEqual(keyAbilityAfterClassChange(classes, 'core:class:envoy', null), 'CHA', 'a fixed key ability is set');
  assertEqual(keyAbilityAfterClassChange(classes, 'core:class:soldier', null), null, 'a choice waits for the player');
  assertEqual(keyAbilityAfterClassChange(classes, 'core:class:soldier', 'DEX'), 'DEX', 'a legal earlier choice is kept');
  assertEqual(keyAbilityAfterClassChange(classes, 'core:class:soldier', 'CHA'), null, 'an illegal earlier choice is dropped');
}

function testPicksAreKeptPrunedAndDefaulted() {
  const slots = [
    slot({ slotId: 'core:ability:human', options: [{ id: 'core:ability:human_no_variant_ability', label: 'No Variant Ability' }], defaultOptionId: 'core:ability:human_no_variant_ability' }),
    slot({ slotId: 'core:ability:2_racial_bonus_to_skill', kind: 'skill', count: 2, options: [{ id: 'diplomacy', label: 'Diplomacy' }, { id: 'medicine', label: 'Medicine' }] }),
  ];
  let picks = setSlotPick([], slots[1], 0, 'diplomacy');
  picks = setSlotPick(picks, slots[1], 1, 'medicine');
  assertEqual(picksForSlot(picks, slots[1].slotId).join(','), 'diplomacy,medicine', 'two picks in one slot, by position');
  picks = setSlotPick(picks, slots[1], 0, '');
  assertEqual(picksForSlot(picks, slots[1].slotId).join(','), 'medicine', 'clearing a position removes only that pick');
  const stale = [...picks, { slotId: 'core:ability:lashunta_default_dimorphic', optionId: 'core:ability:lashunta_subrace_damaya' }];
  const reconciled = reconcilePicks(stale, slots);
  assert(!reconciled.some((p) => p.slotId === 'core:ability:lashunta_default_dimorphic'), 'a pick for a slot no longer offered (race changed) is dropped');
  assertEqual(picksForSlot(reconciled, 'core:ability:human')[0], 'core:ability:human_no_variant_ability', 'a slot the package marks "No ..." starts on that option');
  assertEqual(reconcilePicks(reconciled, slots).length, reconciled.length, 'reconciling twice changes nothing (no preview loop)');
}

function testOnlyBlockingProblemsKeepTheButtonClosed() {
  const preview = {
    problems: [
      { id: 'sf_creation.choice_unfilled', message: '+2 Racial Stat Bonus: choose 1 (0 chosen)', claimBlocking: true },
      { id: 'sf_creation.not_chosen_yet', message: 'UPGRADE SLOT: choose 1 (0 chosen)', claimBlocking: false },
    ],
  } as Pick<SfCreationPreviewDto, 'problems'>;
  const blocking = blockingProblems(preview);
  assertEqual(blocking.length, 1, 'an optional open pick does not block');
  assertEqual(blocking[0].id, 'sf_creation.choice_unfilled', 'the open ability-score pick does');
}

function testOutcomeTilesComeFromTheEnginesRowsOnly() {
  const tiles = starfinderOutcomeTiles([
    { id: 'sf.hit_points', value: 11, detail: '' },
    { id: 'sf.stamina', value: 8, detail: '' },
    { id: 'sf.resolve', value: 4, detail: '' },
    { id: 'sf.eac', value: 12, detail: '' },
    { id: 'sf.kac', value: 12, detail: '' },
    { id: 'sf.fortitude', value: 3, detail: '' },
    { id: 'sf.reflex', value: -1, detail: '' },
    { id: 'sf.skill.athletics', value: 4, detail: '' },
  ]);
  const labels = tiles.map((t) => t.label);
  assertEqual(labels.join('|'), 'Hit Points|Stamina Points|Resolve Points|Energy AC|Kinetic AC|Fortitude|Reflex', 'one tile per engine row, in sheet order');
  assertEqual(tiles.find((t) => t.label === 'Hit Points')?.value, '11', 'a pool prints as a number');
  assertEqual(tiles.find((t) => t.label === 'Reflex')?.value, '-1', 'a negative bonus keeps its sign');
  assertEqual(tiles.find((t) => t.label === 'Fortitude')?.value, '+3', 'a positive bonus prints its sign');
  assert(!labels.includes('Will'), 'a row the engine did not return gets no tile (no fabricated 0)');
  assert(!labels.some((l) => /CMB|CMD|Touch|Flat-Footed/.test(l)), 'no Pathfinder tile on a Starfinder outcome');
}

testStarfinderCreationOpensTheStarfinderForm();
testTheRequestCarriesEveryChoiceAndAFreshIdentity();
testThePointButtonsStayInsideTheEnginesBudget();
testAClassWithOneKeyAbilitySettlesItAndAChoiceWaits();
testPicksAreKeptPrunedAndDefaulted();
testOnlyBlockingProblemsKeepTheButtonClosed();
testOutcomeTilesComeFromTheEnginesRowsOnly();
