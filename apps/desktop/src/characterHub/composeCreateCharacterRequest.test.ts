import {
  applyFloatingAbilityAllocation,
  applyRacialAbilityAdjustments,
  composeCreateCharacterRequest,
} from './composeCreateCharacterRequest';
import { assertEqual } from '../testSupport/asserts';

async function main() {
  verifiesRequestShapeFromFormFields();
  verifiesRacialAdjustmentsAreBakedIntoSubmittedScores();
  verifiesHumanEmptyAdjustmentsLeaveScoresUnchanged();
  verifiesDwarfAdjustmentsAreBakedIntoSubmittedScores();
  verifiesGnomeAdjustmentsAreBakedIntoSubmittedScores();
  verifiesHalflingAdjustmentsAreBakedIntoSubmittedScores();
  verifiesTheFloatingAbilityAllocationReachesTheSubmittedScoresForNonHumanRaces();
  verifiesHumansFloatingAllocationIsLeftToTheBackendSoItIsNotAppliedTwice();
  verifiesTraitSkillChoicesDefaultsToEmptyAndPassesThroughWhenProvided();
}

const ZERO_ALLOCATION = { strength: 0, dexterity: 0, constitution: 0, intelligence: 0, wisdom: 0, charisma: 0 };

/**
 * The sibling of `verifiesRacialAdjustmentsAreBakedIntoSubmittedScores`,
 * and the same defect one field over.
 *
 * PF1's "+2 to one ability score" races are Human, Half-Elf and Half-Orc —
 * derived by command from the corpus, not recalled: only their
 * `Racial Ability Scores` rows carry `BONUS:ABILITYPOOL|Ability Bonus|1`.
 * The creation form has always offered a stepper to distribute those points
 * and shown them in the on-screen calculated score.
 *
 * They reached the engine for **Human only**. `compose_character_input`
 * (`pf1_adapter.rs`) pushes the `choice:human_ability_bonus` slot when
 * `race_id == "race:human"` and for no other race, and
 * `apply_human_ability_bonus` (`pilot_compute.rs`) returns the scores
 * untouched for every other race. Verified by running the engine rather than
 * by reading it: with a submitted Strength of 16 and `abilityBonusTarget:
 * "strength"`, Human computes at 18 and Half-Elf and Half-Orc both compute
 * at 16. So the stepper was a live control that silently changed nothing for
 * two of the three races that have it.
 *
 * `pilot_compute.rs` is off-limits to this cycle (`decisions.md §8`), so the
 * fix is the one the frontend already applies to *fixed* racial adjustments:
 * bake it into the submitted score for every race the backend does not
 * handle itself.
 */
function verifiesTheFloatingAbilityAllocationReachesTheSubmittedScoresForNonHumanRaces() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const allocation = { ...ZERO_ALLOCATION, charisma: 2 };

  const halfElf = applyFloatingAbilityAllocation(raw, allocation, 'race:half-elf');
  assertEqual(halfElf.charisma, 10, 'Half-Elf spends its +2 on Charisma and submits 10, not 8');
  assertEqual(halfElf.strength, 16, 'and nothing else moves');

  const halfOrc = applyFloatingAbilityAllocation(raw, { ...ZERO_ALLOCATION, strength: 2 }, 'race:half-orc');
  assertEqual(halfOrc.strength, 18, 'Half-Orc spends its +2 on Strength');

  // A race with no floating pool allocates nothing, so this is a no-op for
  // the other 15 races whatever it is handed.
  const goblin = applyFloatingAbilityAllocation(raw, ZERO_ALLOCATION, 'race:goblin');
  assertEqual(goblin.dexterity, 14, 'a race with no floating pool submits its scores unchanged');
}

/**
 * Human is the deliberate exception, and must stay one: its floating +2 is
 * applied server-side from `abilityBonusTarget`
 * (`race.human.ability_bonus_applied`). Baking it in here as well would
 * award +4.
 */
function verifiesHumansFloatingAllocationIsLeftToTheBackendSoItIsNotAppliedTwice() {
  const raw = { strength: 16, dexterity: 14, constitution: 14, intelligence: 10, wisdom: 12, charisma: 8 };
  const human = applyFloatingAbilityAllocation(raw, { ...ZERO_ALLOCATION, strength: 2 }, 'race:human');
  assertEqual(human.strength, 16, 'Human submits the PRE-bonus base score; the engine adds the +2');
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
 * AT-34-E4-002 (second slice): an omitted `traitSkillChoices` composes to
 * an empty array (every pre-existing caller keeps working unchanged), and
 * a provided one passes through verbatim -- the same "trusted wire list"
 * shape `selectedTraits` already follows.
 */
function verifiesTraitSkillChoicesDefaultsToEmptyAndPassesThroughWhenProvided() {
  const baseFields = {
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
  };
  const deps = { generateId: () => 'char-fixed-id', now: () => '2026-07-08T00:00:00Z' };

  const withoutChoices = composeCreateCharacterRequest(baseFields, deps);
  assertEqual(withoutChoices.traitSkillChoices.length, 0, 'omitted traitSkillChoices composes empty');

  const withChoices = composeCreateCharacterRequest(
    {
      ...baseFields,
      selectedTraits: ['trait:trait_criminal'],
      traitSkillChoices: [{ choiceSetId: 'trait_choice:trait:trait_criminal', selectionId: 'skill:intimidate' }],
    },
    deps
  );
  assertEqual(withChoices.traitSkillChoices.length, 1, 'provided traitSkillChoices passes through');
  assertEqual(withChoices.traitSkillChoices[0]!.choiceSetId, 'trait_choice:trait:trait_criminal', 'choiceSetId');
  assertEqual(withChoices.traitSkillChoices[0]!.selectionId, 'skill:intimidate', 'selectionId');
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
