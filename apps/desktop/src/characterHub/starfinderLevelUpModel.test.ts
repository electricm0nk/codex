import {
  EMPTY_LEVEL_UP_DRAFT,
  adjustSkillRanks,
  buildStarfinderLevelUpRequest,
  canAcceptLevelUp,
  classChoiceAfterPreview,
  levelUpChangeLines,
  levelUpNeedsKeyAbility,
  toggleIncrease,
  type StarfinderLevelUpDraft,
} from './starfinderLevelUpModel';
import { starfinderRowLabel } from './starfinderSheetModel';
import type { SfLevelUpPreviewDto } from '../boundary/starfinderLevelUp';
import { assert, assertEqual } from '../testSupport/asserts';

/**
 * SD-37 E6.5: the Starfinder level-up. Every number the dialog shows is the engine's
 * (`preview_starfinder_level_up`): the class levels, the scores before and after the
 * increase, each skill's ranks and its cap, and the totals the level changes. These tests pin
 * the pure pieces the dialog composes: the request it sends, the increase and rank buttons
 * (bounded by the engine's preview, not a desktop table), the class it starts on and the
 * change lines (engine values, labelled by the sheet's own labels).
 */

function preview(partial: Partial<SfLevelUpPreviewDto> = {}): SfLevelUpPreviewDto {
  return {
    characterLevel: 1,
    classes: [
      { id: 'core:class:soldier', label: 'Soldier', currentLevel: 1, keyAbilityOptions: [{ id: 'STR', label: 'Strength' }, { id: 'DEX', label: 'Dexterity' }] },
      { id: 'core:class:operative', label: 'Operative', currentLevel: 0, keyAbilityOptions: [{ id: 'DEX', label: 'Dexterity' }] },
      { id: 'core:class:solarian', label: 'Solarian', currentLevel: 0, keyAbilityOptions: [{ id: 'CHA', label: 'Charisma' }, { id: 'STR', label: 'Strength' }] },
    ],
    levelLine: 'Soldier 2 — character level 2',
    classLines: ['Skill ranks per level: 4'],
    ruleLines: [],
    increaseDue: false,
    increaseScores: 0,
    abilities: [],
    slots: [],
    chosenOnTheSheet: [],
    skills: [
      { skill: 'athletics', label: 'Athletics', ranks: 1, added: 0, maxRanks: 2 },
      { skill: 'medicine', label: 'Medicine', ranks: 0, added: 0, maxRanks: 2 },
    ],
    skillRule: 'Ranks in one skill: at most the character level, 2',
    changes: [],
    problems: [],
    choices: null,
    ...partial,
  };
}

function testTheRequestCarriesEveryChoice() {
  const draft: StarfinderLevelUpDraft = {
    ...EMPTY_LEVEL_UP_DRAFT,
    classId: 'core:class:soldier',
    abilityIncreases: ['STR', 'DEX', 'CON', 'WIS'],
    picks: [{ slotId: 'core:ability:soldier_class_feature_gear_boost', optionId: 'core:ability:laser_accuracy' }],
    skillRanks: { athletics: 1, medicine: 0 },
    choices: { feats: ['core:feat:coordinated_shot'], featPicks: [], spells: [], gear: [] },
  };
  const request = buildStarfinderLevelUpRequest('sf-1', draft, () => '2026-10-05T00:00:00Z');
  assertEqual(request.choices?.feats.join(','), 'core:feat:coordinated_shot', 'the feat the level owes, chosen in the dialog (SD-37 E6.5a)');
  assertEqual(buildStarfinderLevelUpRequest('sf-1', EMPTY_LEVEL_UP_DRAFT, () => '').choices, null, 'untouched choices: the character keeps its own');
  assertEqual(request.characterId, 'sf-1', 'the character');
  assertEqual(request.classId, 'core:class:soldier', 'the class');
  assertEqual(request.keyAbility, null, 'no key ability for an advanced class');
  assertEqual(request.abilityIncreases.join(','), 'STR,DEX,CON,WIS', 'the increase');
  assertEqual(request.skillRanks.length, 1, 'a skill with no added rank is not sent');
  assertEqual(request.skillRanks[0].skill, 'athletics', 'the ranked skill');
  assertEqual(request.skillRanks[0].ranks, 1, 'its added ranks');
  assertEqual(request.picks.length, 1, 'the picks');
  assertEqual(request.savedAt, '2026-10-05T00:00:00Z', 'the save time');
}

function testTheDialogStartsOnAHeldClass() {
  const draft = classChoiceAfterPreview(EMPTY_LEVEL_UP_DRAFT, preview());
  assertEqual(draft.classId, 'core:class:soldier', 'the first held class is preselected');
  const kept = classChoiceAfterPreview({ ...EMPTY_LEVEL_UP_DRAFT, classId: 'core:class:operative' }, preview());
  assertEqual(kept.classId, 'core:class:operative', 'a chosen class is kept');
  assert(!levelUpNeedsKeyAbility(preview(), 'core:class:soldier'), 'an advanced class keeps its key ability');
  assert(!levelUpNeedsKeyAbility(preview(), 'core:class:operative'), 'a new class with one key ability settles it');
  assert(levelUpNeedsKeyAbility(preview(), 'core:class:solarian'), 'a new class with a choice asks for it');
}

function testTheIncreaseTakesFourDifferentScores() {
  const shown = preview({ increaseDue: true, increaseScores: 4 });
  let draft = EMPTY_LEVEL_UP_DRAFT;
  for (const code of ['STR', 'DEX', 'CON', 'INT', 'WIS']) {
    draft = toggleIncrease(draft, code, shown);
  }
  assertEqual(draft.abilityIncreases.join(','), 'STR,DEX,CON,INT', 'a fifth score is not taken');
  draft = toggleIncrease(draft, 'DEX', shown);
  assertEqual(draft.abilityIncreases.join(','), 'STR,CON,INT', 'a chosen score toggles off');
}

function testTheIncreaseCountIsTheEnginesNotTheDesktops() {
  // SD-37 E6.MC: the count of scores an increase raises is the preview's (the engine's
  // `sf_abilities::SCORES_PER_INCREASE`), never a desktop constant (R2).
  let draft = EMPTY_LEVEL_UP_DRAFT;
  for (const code of ['STR', 'DEX', 'CON']) {
    draft = toggleIncrease(draft, code, preview({ increaseDue: true, increaseScores: 2 }));
  }
  assertEqual(draft.abilityIncreases.join(','), 'STR,DEX', 'the preview asks for two: a third is not taken');
  const none = toggleIncrease(EMPTY_LEVEL_UP_DRAFT, 'STR', preview({ increaseDue: false, increaseScores: 0 }));
  assertEqual(none.abilityIncreases.length, 0, 'a level with no increase takes no score');
}

function testRankButtonsStayInsideTheEnginesCap() {
  const shown = preview();
  let draft = adjustSkillRanks(EMPTY_LEVEL_UP_DRAFT, shown, 'athletics', 1);
  assertEqual(draft.skillRanks.athletics, 1, 'one rank added');
  draft = adjustSkillRanks(draft, shown, 'athletics', 1);
  assertEqual(draft.skillRanks.athletics, 1, 'Athletics 1 + 1 = the cap 2: no more');
  draft = adjustSkillRanks(draft, shown, 'athletics', -1);
  draft = adjustSkillRanks(draft, shown, 'athletics', -1);
  assertEqual(draft.skillRanks.athletics, 0, 'never below the ranks held');
  draft = adjustSkillRanks(draft, shown, 'piloting', 1);
  assertEqual(draft.skillRanks.piloting, undefined, 'a skill the engine did not list is not ranked');
}

function testChangeLinesAreTheEnginesValuesWithTheSheetsLabels() {
  const lines = levelUpChangeLines(
    preview({
      changes: [
        { id: 'sf.hit_points', before: 11, after: 18 },
        { id: 'sf.base_attack_bonus', before: 1, after: 2 },
        { id: 'sf.class_level.soldier', before: 1, after: 2 },
        { id: 'sf.skill.athletics', before: null, after: 7 },
      ],
    }),
  );
  assertEqual(lines.join('|'), 'Hit Points: 11 → 18|Base Attack Bonus: +1 → +2|Soldier level: 1 → 2|Athletics: — → +7', 'one line per changed engine row');
  assertEqual(starfinderRowLabel('sf.ability_score.wisdom'), 'Wisdom', 'an ability score row');
  assertEqual(starfinderRowLabel('sf.made_up'), 'sf.made_up', 'an unknown row keeps its id');
}

function testOnlyAPreviewWithNoBlockingProblemAccepts() {
  assert(canAcceptLevelUp(preview()), 'a clean preview accepts');
  assert(!canAcceptLevelUp(preview({ levelLine: null })), 'no class chosen');
  assert(
    !canAcceptLevelUp(preview({ problems: [{ id: 'sf_level_up.increase_not_chosen', message: 'choose', claimBlocking: true }] })),
    'a blocking problem keeps Accept closed',
  );
  assert(
    canAcceptLevelUp(preview({ problems: [{ id: 'sf_creation.not_chosen_yet', message: 'later', claimBlocking: false }] })),
    'an optional pick left open does not',
  );
}

testTheRequestCarriesEveryChoice();
testTheDialogStartsOnAHeldClass();
testTheIncreaseTakesFourDifferentScores();
testTheIncreaseCountIsTheEnginesNotTheDesktops();
testRankButtonsStayInsideTheEnginesCap();
testChangeLinesAreTheEnginesValuesWithTheSheetsLabels();
testOnlyAPreviewWithNoBlockingProblemAccepts();
