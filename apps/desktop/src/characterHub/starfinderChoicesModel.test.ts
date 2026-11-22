import {
  EMPTY_STARFINDER_CHOICES,
  addFeat,
  addGear,
  addModifier,
  addSpell,
  blockingChoiceProblems,
  choicesTotalsLine,
  equipmentOptionLabel,
  featOptionLabel,
  removeFeatAt,
  removeGearAt,
  removeModifierAt,
  removeSpell,
  setFeatPick,
  setGearEquipped,
  spellPicksAt,
} from './starfinderChoicesModel';
import type { SfChoicesDto, SfChoicesPreviewDto } from '../boundary/starfinderChoices';
import type { SfCreationSlotDto } from '../boundary/starfinderCreation';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-37 E6.5a: feats, spells known and gear chosen for a Starfinder character. The panel holds
 * only the player's choices (`SfChoicesDto`); every option list, prerequisite verdict, spells-
 * known total and sheet total it shows is the engine's (`preview_starfinder_choices`). These
 * tests pin the pure edits the three affordances make, and the totals line the sheet prints
 * after a save (the engine's EAC, KAC and credits rows, labelled by the sheet's own labels).
 */

function preview(partial: Partial<SfChoicesPreviewDto> = {}): SfChoicesPreviewDto {
  return {
    chosen: EMPTY_STARFINDER_CHOICES,
    featRules: [],
    feats: [],
    featOptions: [],
    featSlots: [],
    spellLevels: [],
    gear: [],
    totals: [],
    problems: [],
    ...partial,
  };
}

function testFeatsAreAddedAndRemovedWithTheirOwnPicks() {
  let choices: SfChoicesDto = addFeat(EMPTY_STARFINDER_CHOICES, 'core:feat:weapon_focus');
  choices = addFeat(choices, 'core:feat:quick_draw');
  assertEqual(choices.feats.join(','), 'core:feat:weapon_focus,core:feat:quick_draw', 'feats in the order chosen');
  assertEqual(addFeat(choices, '').feats.length, 2, 'the empty option adds nothing');
  const slot: SfCreationSlotDto = {
    slotId: 'core:feat:weapon_focus',
    label: 'Weapon Focus',
    kind: 'option',
    count: 1,
    repeatable: false,
    options: [{ id: 'core:ability:weapon_prof_longarms', label: 'Longarms' }],
    defaultOptionId: null,
    required: false,
  };
  choices = setFeatPick(choices, slot, 0, 'core:ability:weapon_prof_longarms');
  assertEqual(choices.featPicks.length, 1, 'the feat’s own pick is held');
  const without = removeFeatAt(choices, 0);
  assertEqual(without.feats.join(','), 'core:feat:quick_draw', 'the feat is removed');
  assertEqual(without.featPicks.length, 0, 'and its own pick with it');
  assert(EMPTY_STARFINDER_CHOICES.feats.length === 0, 'the edits never mutate their input');
}

function testSpellsKnownAreKeptPerClass() {
  let choices = addSpell(EMPTY_STARFINDER_CHOICES, 'core:class:mystic', 'core:spell:command');
  choices = addSpell(choices, 'core:class:mystic', 'core:spell:command');
  assertEqual(choices.spells.length, 1, 'a spell is known once per class');
  choices = addSpell(choices, 'core:class:mystic', 'core:spell:detect_thoughts');
  const level = {
    classId: 'core:class:mystic',
    classLabel: 'Mystic',
    level: 1,
    known: 5,
    knownTerms: [],
    chosen: [],
    options: [
      { id: 'core:spell:command', label: 'Command' },
      { id: 'core:spell:detect_thoughts', label: 'Detect Thoughts' },
    ],
  };
  assertEqual(spellPicksAt(choices, level).length, 2, 'the picks at a level are the class’s picks on that level’s list');
  assertEqual(removeSpell(choices, 'core:class:mystic', 'core:spell:command').spells.length, 1, 'a spell is removed');
}

function testGearIsAddedEquippedOrCarriedWithUpgrades() {
  let choices = addGear(EMPTY_STARFINDER_CHOICES, 'core:equipment:defiance_series_squad', true);
  choices = addGear(choices, 'core:equipment:battery', false);
  choices = addGear(choices, 'core:equipment:battery', false);
  assertEqual(choices.gear.length, 3, 'one selection per item, the same item twice');
  assert(choices.gear[0]!.equipped && !choices.gear[1]!.equipped, 'equipped or carried as added');
  choices = setGearEquipped(choices, 1, true);
  assert(choices.gear[1]!.equipped, 'a carried item is equipped');
  choices = addModifier(choices, 0, 'core:equipment_modifier:armor_infrared_sensors');
  choices = addModifier(choices, 0, '');
  assertEqual(choices.gear[0]!.modifiers.join(','), 'core:equipment_modifier:armor_infrared_sensors', 'an upgrade on its item');
  assertEqual(removeModifierAt(choices, 0, 0).gear[0]!.modifiers.length, 0, 'an upgrade is removed');
  const removed = removeGearAt(choices, 1);
  assertEqual(removed.gear.map((g) => g.itemId).join(','), 'core:equipment:defiance_series_squad,core:equipment:battery', 'the item at that position goes');
}

function testLabelsAndTheTotalsLineAreTheEngines() {
  assertEqual(featOptionLabel({ id: 'a', label: 'Quick Draw', eligible: true, repeatable: false }), 'Quick Draw', 'an open feat');
  assertEqual(
    featOptionLabel({ id: 'b', label: 'Deflect Projectiles', eligible: false, repeatable: false }),
    'Deflect Projectiles (prerequisite not met)',
    'the engine’s verdict printed',
  );
  assertEqual(equipmentOptionLabel({ id: 'c', label: 'Defiance Series, Squad', price: '1220' }), 'Defiance Series, Squad (1220 credits)', 'the record’s own price row');
  assertEqual(equipmentOptionLabel({ id: 'd', label: 'Battery', price: null }), 'Battery', 'no price row, no price');
  const line = choicesTotalsLine([
    { id: 'sf.eac', value: 16, detail: '' },
    { id: 'sf.kac', value: 19, detail: '' },
    { id: 'sf.credits.starting', value: 4000, detail: '' },
    { id: 'sf.credits.remaining', value: 2045, detail: '' },
    { id: 'sf.bulk', value: 4, detail: '' },
  ]);
  assertEqual(line, 'Energy Armor Class (EAC) 16 · Kinetic Armor Class (KAC) 19 · Credits remaining 2045 · Bulk carried 4', 'the engine’s rows, the sheet’s labels');
  assertEqual(choicesTotalsLine([]), '', 'no rows, no line');
  const blocking = blockingChoiceProblems(
    preview({
      problems: [
        { id: 'sf_loadout.over_budget', message: 'over', claimBlocking: true },
        { id: 'sf_creation.not_chosen_yet', message: 'later', claimBlocking: false },
      ],
    }),
  );
  assertEqual(blocking.map((p) => p.id).join(','), 'sf_loadout.over_budget', 'only the blocking problems block');
}

testFeatsAreAddedAndRemovedWithTheirOwnPicks();
testSpellsKnownAreKeptPerClass();
testGearIsAddedEquippedOrCarriedWithUpgrades();
testLabelsAndTheTotalsLineAreTheEngines();
