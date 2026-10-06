import type { SfLevelUpPreviewDto, SfLevelUpRequest } from '../boundary/starfinderLevelUp';
import type { SfPickDto } from '../boundary/starfinderCreation';
import type { SfChoicesDto } from '../boundary/starfinderChoices';
import { starfinderRowLabel, starfinderRowSigned } from './starfinderSheetModel';

/**
 * The Starfinder level-up dialog's pure pieces (SD-37 E6.5). The dialog holds the player's
 * choices; every list, cap and number it shows is the engine's preview
 * (`preview_starfinder_level_up`). Nothing here knows a Starfinder rule: the increase takes
 * four scores because the engine's preview asks for four, and a skill's cap is the preview's
 * `maxRanks`.
 */

export interface StarfinderLevelUpDraft {
  classId: string | null;
  keyAbility: string | null;
  abilityIncreases: string[];
  picks: SfPickDto[];
  /** Package skill id -> ranks this level adds. */
  skillRanks: Record<string, number>;
  /** Feats, spells known and gear at the new level (SD-37 E6.5a); `null` until the player edits them. */
  choices: SfChoicesDto | null;
}

export const EMPTY_LEVEL_UP_DRAFT: StarfinderLevelUpDraft = {
  classId: null,
  keyAbility: null,
  abilityIncreases: [],
  picks: [],
  skillRanks: {},
  choices: null,
};

export function buildStarfinderLevelUpRequest(characterId: string, draft: StarfinderLevelUpDraft, now: () => string): SfLevelUpRequest {
  return {
    characterId,
    savedAt: now(),
    classId: draft.classId,
    keyAbility: draft.keyAbility,
    abilityIncreases: [...draft.abilityIncreases],
    picks: [...draft.picks],
    skillRanks: Object.entries(draft.skillRanks)
      .filter(([, ranks]) => ranks > 0)
      .map(([skill, ranks]) => ({ skill, ranks })),
    choices: draft.choices,
  };
}

/** The draft once a preview arrives: with no class chosen yet, the first class the character holds. */
export function classChoiceAfterPreview(draft: StarfinderLevelUpDraft, preview: SfLevelUpPreviewDto): StarfinderLevelUpDraft {
  if (draft.classId !== null) {
    return draft;
  }
  const held = preview.classes.find((option) => option.currentLevel > 0);
  return held === undefined ? draft : { ...draft, classId: held.id };
}

/** A new class whose key ability is a choice asks for it; a held class keeps its own. */
export function levelUpNeedsKeyAbility(preview: SfLevelUpPreviewDto, classId: string | null): boolean {
  const option = preview.classes.find((candidate) => candidate.id === classId);
  return option !== undefined && option.currentLevel === 0 && option.keyAbilityOptions.length > 1;
}

/** Picking another class starts its key ability and picks over. */
export function draftAfterClassChange(draft: StarfinderLevelUpDraft, classId: string | null): StarfinderLevelUpDraft {
  return { ...draft, classId, keyAbility: null, picks: [] };
}

/** Toggles one score of the increase; the preview's `increaseScores` (the engine's count) bounds it. */
export function toggleIncrease(draft: StarfinderLevelUpDraft, ability: string, preview: SfLevelUpPreviewDto): StarfinderLevelUpDraft {
  if (draft.abilityIncreases.includes(ability)) {
    return { ...draft, abilityIncreases: draft.abilityIncreases.filter((code) => code !== ability) };
  }
  if (draft.abilityIncreases.length >= preview.increaseScores) {
    return draft;
  }
  return { ...draft, abilityIncreases: [...draft.abilityIncreases, ability] };
}

/** Adds or removes one rank this level, inside the preview's cap for that skill. */
export function adjustSkillRanks(draft: StarfinderLevelUpDraft, preview: SfLevelUpPreviewDto, skill: string, delta: number): StarfinderLevelUpDraft {
  const listed = preview.skills.find((candidate) => candidate.skill === skill);
  if (listed === undefined) {
    return draft;
  }
  const added = (draft.skillRanks[skill] ?? 0) + delta;
  if (added < 0 || listed.ranks + added > listed.maxRanks) {
    return draft;
  }
  return { ...draft, skillRanks: { ...draft.skillRanks, [skill]: added } };
}

function formatChange(value: number | null, signed: boolean): string {
  if (value === null) {
    return '—';
  }
  return signed && value >= 0 ? `+${value}` : String(value);
}

/** One line per sheet total the level changes: the engine's values, the sheet's labels. */
export function levelUpChangeLines(preview: SfLevelUpPreviewDto): string[] {
  return preview.changes.map((change) => {
    const signed = starfinderRowSigned(change.id);
    return `${starfinderRowLabel(change.id)}: ${formatChange(change.before, signed)} → ${formatChange(change.after, signed)}`;
  });
}

export function canAcceptLevelUp(preview: SfLevelUpPreviewDto): boolean {
  return preview.levelLine !== null && preview.problems.every((problem) => !problem.claimBlocking);
}
