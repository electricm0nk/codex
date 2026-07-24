import { applyRacialAbilityAdjustments, composeCreateCharacterRequest } from './composeCreateCharacterRequest';
import { assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesRequestShapeFromFormFields();
  verifiesRacialAdjustmentsAreBakedIntoSubmittedScores();
  verifiesHumanEmptyAdjustmentsLeaveScoresUnchanged();
  verifiesDwarfAdjustmentsAreBakedIntoSubmittedScores();
  verifiesGnomeAdjustmentsAreBakedIntoSubmittedScores();
  verifiesHalflingAdjustmentsAreBakedIntoSubmittedScores();
}

function verifiesRequestShapeFromFormFields() {
  const request = composeCreateCharacterRequest(
    {
      displayLabel: 'Aldric',
      raceId: 'race:human',
      classId: 'class:fighter',
      level: 1,
      abilityScores: {
        strength: 16,
        dexterity: 14,
        constitution: 14,
        intelligence: 10,
        wisdom: 12,
        charisma: 8,
      },
      abilityBonusTarget: 'dexterity',
    },
    {
      generateId: () => 'char-fixed-id',
      now: () => '2026-07-08T00:00:00Z',
    }
  );

  assertEqual(request.characterId, 'char-fixed-id', 'characterId');
  assertEqual(request.displayLabel, 'Aldric', 'displayLabel');
  assertEqual(request.raceId, 'race:human', 'raceId');
  assertEqual(request.classId, 'class:fighter', 'classId');
  assertEqual(request.level, 1, 'level');
  assertEqual(request.savedAt, '2026-07-08T00:00:00Z', 'savedAt');
  assertEqual(request.abilityScores.strength, 16, 'ability strength');
  assertEqual(request.abilityScores.charisma, 8, 'ability charisma');
  assertEqual(request.abilityBonusTarget, 'dexterity', 'abilityBonusTarget');
}

/**
 * Regression guard for a real bug found by live-driving the app (not caught
 * by any prior test): `CreateCharacterForm.tsx` was submitting the raw
 * entered/rolled ability scores unmodified, silently dropping every
 * non-Human race's fixed ability adjustment (Elf +2 DEX/-2 CON/+2 INT, Dwarf
 * +2 CON/+2 WIS/-2 CHA, Gnome +2 CON/+2 CHA/-2 STR, Halfling +2 DEX/+2 CHA/-2
 * STR) even though the compute engine's own contract expects those baked in
 * before submission (only Human is the pre-bonus-base exception). Live-
 * verified on disk: an Elf character created before this fix had raw
 * ability=intelligence:10 saved, not the expected 12.
 */
function verifiesRacialAdjustmentsAreBakedIntoSubmittedScores() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const elfAdjustments = { dexterity: 2, intelligence: 2, constitution: -2 };

  const adjusted = applyRacialAbilityAdjustments(raw, elfAdjustments);

  assertEqual(adjusted.dexterity, 16, 'Elf +2 Dexterity should be baked in');
  assertEqual(adjusted.intelligence, 12, 'Elf +2 Intelligence should be baked in');
  assertEqual(adjusted.constitution, 12, 'Elf -2 Constitution should be baked in');
  assertEqual(adjusted.strength, 16, 'Strength has no Elf adjustment and should be untouched');
}

function verifiesHumanEmptyAdjustmentsLeaveScoresUnchanged() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };

  const adjusted = applyRacialAbilityAdjustments(raw, {});

  assertEqual(adjusted.strength, 16, 'strength');
  assertEqual(adjusted.dexterity, 14, 'dexterity');
  assertEqual(adjusted.constitution, 14, 'constitution');
  assertEqual(adjusted.intelligence, 10, 'intelligence');
  assertEqual(adjusted.wisdom, 12, 'wisdom');
  assertEqual(adjusted.charisma, 8, 'charisma');
}

/**
 * QA found this file's original Dwarf coverage only asserted the
 * *untouched* abilities, never the actual adjusted CON/WIS/CHA values --
 * meaning a regression that broke Dwarf's adjustment (e.g. applying the
 * wrong magnitude, or applying it to the wrong ability) would have passed
 * silently. Asserts the full adjusted triple, matching Elf's coverage shape.
 */
function verifiesDwarfAdjustmentsAreBakedIntoSubmittedScores() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const dwarfAdjustments = { constitution: 2, wisdom: 2, charisma: -2 };

  const adjusted = applyRacialAbilityAdjustments(raw, dwarfAdjustments);

  assertEqual(adjusted.constitution, 16, 'Dwarf +2 Constitution should be baked in');
  assertEqual(adjusted.wisdom, 14, 'Dwarf +2 Wisdom should be baked in');
  assertEqual(adjusted.charisma, 6, 'Dwarf -2 Charisma should be baked in');
  assertEqual(adjusted.strength, 16, 'Strength has no Dwarf adjustment and should be untouched');
  assertEqual(adjusted.dexterity, 14, 'Dexterity has no Dwarf adjustment and should be untouched');
  assertEqual(adjusted.intelligence, 10, 'Intelligence has no Dwarf adjustment and should be untouched');
}

/** Gnome had zero coverage at all before this -- same gap QA flagged for Dwarf, one step further. */
function verifiesGnomeAdjustmentsAreBakedIntoSubmittedScores() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const gnomeAdjustments = { constitution: 2, charisma: 2, strength: -2 };

  const adjusted = applyRacialAbilityAdjustments(raw, gnomeAdjustments);

  assertEqual(adjusted.constitution, 16, 'Gnome +2 Constitution should be baked in');
  assertEqual(adjusted.charisma, 10, 'Gnome +2 Charisma should be baked in');
  assertEqual(adjusted.strength, 14, 'Gnome -2 Strength should be baked in');
  assertEqual(adjusted.dexterity, 14, 'Dexterity has no Gnome adjustment and should be untouched');
  assertEqual(adjusted.intelligence, 10, 'Intelligence has no Gnome adjustment and should be untouched');
  assertEqual(adjusted.wisdom, 12, 'Wisdom has no Gnome adjustment and should be untouched');
}

/** Halfling had zero coverage at all before this -- same gap QA flagged for Dwarf, one step further. */
function verifiesHalflingAdjustmentsAreBakedIntoSubmittedScores() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const halflingAdjustments = { dexterity: 2, charisma: 2, strength: -2 };

  const adjusted = applyRacialAbilityAdjustments(raw, halflingAdjustments);

  assertEqual(adjusted.dexterity, 16, 'Halfling +2 Dexterity should be baked in');
  assertEqual(adjusted.charisma, 10, 'Halfling +2 Charisma should be baked in');
  assertEqual(adjusted.strength, 14, 'Halfling -2 Strength should be baked in');
  assertEqual(adjusted.constitution, 14, 'Constitution has no Halfling adjustment and should be untouched');
  assertEqual(adjusted.intelligence, 10, 'Intelligence has no Halfling adjustment and should be untouched');
  assertEqual(adjusted.wisdom, 12, 'Wisdom has no Halfling adjustment and should be untouched');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
