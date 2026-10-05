import type {
  SfClassOptionDto,
  SfCreationPreviewDto,
  SfCreationRequest,
  SfCreationSlotDto,
  SfPickDto,
  SfPointBuyDto,
  SfPointBuyRulesDto,
} from '../boundary/starfinderCreation';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';

/**
 * Pure state for the Starfinder 1e creation flow (SD-37 E6.2): race -> theme
 * -> class -> point buy. The form holds only the player's choices; every rule
 * number it shows (the point budget, the creation cap, each ability score and
 * its terms, the picks a race or theme asks for) is the engine's, read from
 * `preview_starfinder_character`. Nothing here is a Starfinder rules table.
 */

export type SfAbilityKey = keyof SfPointBuyDto;

export const SF_ABILITY_KEYS: readonly SfAbilityKey[] = [
  'strength',
  'dexterity',
  'constitution',
  'intelligence',
  'wisdom',
  'charisma',
];

/** Short names for the point buttons' own text (`+1 Str`). */
export const SF_ABILITY_ABBREVIATIONS: Record<SfAbilityKey, string> = {
  strength: 'Str',
  dexterity: 'Dex',
  constitution: 'Con',
  intelligence: 'Int',
  wisdom: 'Wis',
  charisma: 'Cha',
};

export interface StarfinderCreationDraft {
  displayLabel: string;
  raceId: string | null;
  themeId: string | null;
  classId: string | null;
  keyAbility: string | null;
  pointBuy: SfPointBuyDto;
  picks: SfPickDto[];
}

export const EMPTY_STARFINDER_DRAFT: StarfinderCreationDraft = {
  displayLabel: '',
  raceId: null,
  themeId: null,
  classId: null,
  keyAbility: null,
  pointBuy: { strength: 0, dexterity: 0, constitution: 0, intelligence: 0, wisdom: 0, charisma: 0 },
  picks: [],
};

/** The request for the draft (`create_starfinder_character`, or the preview with an empty identity). */
export function buildStarfinderCreationRequest(
  draft: StarfinderCreationDraft,
  deps: { generateId: () => string; now: () => string },
): SfCreationRequest {
  return {
    characterId: deps.generateId(),
    displayLabel: draft.displayLabel.trim(),
    savedAt: deps.now(),
    raceId: draft.raceId,
    themeId: draft.themeId,
    classId: draft.classId,
    keyAbility: draft.keyAbility,
    pointBuy: { ...draft.pointBuy },
    picks: draft.picks.map((pick) => ({ ...pick })),
  };
}

/** The preview request: the same choices, no identity (nothing is saved). */
export function buildStarfinderPreviewRequest(draft: StarfinderCreationDraft): SfCreationRequest {
  return buildStarfinderCreationRequest(draft, { generateId: () => '', now: () => '' });
}

/**
 * One point up or down on `ability`, inside the engine's budget (`rules.budget`,
 * from the preview): never below 0 points, never past the budget. The 18 cap at
 * creation depends on race and theme, so it is the engine's to refuse
 * (`sf_abilities.score_over_18_at_creation`), shown as a problem, not guessed here.
 */
export function adjustPoints(
  pointBuy: SfPointBuyDto,
  ability: SfAbilityKey,
  delta: number,
  rules: SfPointBuyRulesDto,
): SfPointBuyDto {
  const next = pointBuy[ability] + delta;
  const spent = SF_ABILITY_KEYS.reduce((sum, key) => sum + pointBuy[key], 0) + delta;
  if (next < 0 || spent > rules.budget) {
    return pointBuy;
  }
  return { ...pointBuy, [ability]: next };
}

/** The key ability after a class change: a fixed one is set; a legal earlier choice is kept. */
export function keyAbilityAfterClassChange(
  classes: readonly SfClassOptionDto[],
  classId: string | null,
  current: string | null,
): string | null {
  const options = classes.find((candidate) => candidate.id === classId)?.keyAbilityOptions ?? [];
  if (options.length === 1) {
    return options[0].id;
  }
  return options.some((option) => option.id === current) ? current : null;
}

/** The options chosen for one slot, in position order. */
export function picksForSlot(picks: readonly SfPickDto[], slotId: string): string[] {
  return picks.filter((pick) => pick.slotId === slotId).map((pick) => pick.optionId);
}

/** Sets (or, with `''`, clears) the pick at `position` of `slot`; other slots are untouched. */
export function setSlotPick(picks: readonly SfPickDto[], slot: SfCreationSlotDto, position: number, optionId: string): SfPickDto[] {
  const chosen: (string | undefined)[] = picksForSlot(picks, slot.slotId);
  while (chosen.length <= position) {
    chosen.push(undefined);
  }
  chosen[position] = optionId === '' ? undefined : optionId;
  const others = picks.filter((pick) => pick.slotId !== slot.slotId);
  return [
    ...others,
    ...chosen.filter((id): id is string => id !== undefined).map((id) => ({ slotId: slot.slotId, optionId: id })),
  ];
}

/**
 * The picks after a preview round: a pick for a slot no longer offered (the race
 * or class changed) is dropped, and an empty slot the package marks "No ..."
 * starts on that option. Idempotent, so applying it never loops the preview.
 */
export function reconcilePicks(picks: readonly SfPickDto[], slots: readonly SfCreationSlotDto[]): SfPickDto[] {
  const offered = new Set(slots.map((slot) => slot.slotId));
  const kept = picks.filter((pick) => offered.has(pick.slotId));
  for (const slot of slots) {
    if (slot.defaultOptionId !== null && picksForSlot(kept, slot.slotId).length === 0) {
      kept.push({ slotId: slot.slotId, optionId: slot.defaultOptionId });
    }
  }
  return kept;
}

/** Whether two pick lists are the same picks in the same order. */
export function samePicks(a: readonly SfPickDto[], b: readonly SfPickDto[]): boolean {
  return a.length === b.length && a.every((pick, i) => pick.slotId === b[i].slotId && pick.optionId === b[i].optionId);
}

/** The problems that keep the character from being created (an open optional pick does not). */
export function blockingProblems(preview: Pick<SfCreationPreviewDto, 'problems'>): DiagnosticDto[] {
  return preview.problems.filter((problem) => problem.claimBlocking);
}

/** The optional picks still open: listed, never blocking. */
export function openOptionalPicks(preview: Pick<SfCreationPreviewDto, 'problems'>): DiagnosticDto[] {
  return preview.problems.filter((problem) => !problem.claimBlocking);
}

export interface StarfinderOutcomeTile {
  label: string;
  value: string;
}

/** The outcome's tiles, in sheet order: engine row id -> its label, and whether it is a bonus (signed). */
const OUTCOME_ROWS: readonly { id: string; label: string; signed: boolean }[] = [
  { id: 'sf.hit_points', label: 'Hit Points', signed: false },
  { id: 'sf.stamina', label: 'Stamina Points', signed: false },
  { id: 'sf.resolve', label: 'Resolve Points', signed: false },
  { id: 'sf.eac', label: 'Energy AC', signed: false },
  { id: 'sf.kac', label: 'Kinetic AC', signed: false },
  { id: 'sf.fortitude', label: 'Fortitude', signed: true },
  { id: 'sf.reflex', label: 'Reflex', signed: true },
  { id: 'sf.will', label: 'Will', signed: true },
  { id: 'sf.initiative', label: 'Initiative', signed: true },
  { id: 'sf.base_attack_bonus', label: 'Base Attack Bonus', signed: true },
];

/** One tile per `sf.*` row the engine returned; a row it did not return gets no tile. */
export function starfinderOutcomeTiles(explanations: readonly ExplanationDto[]): StarfinderOutcomeTile[] {
  return OUTCOME_ROWS.flatMap((row) => {
    const found = explanations.find((explanation) => explanation.id === row.id);
    if (!found) {
      return [];
    }
    const value = row.signed && found.value >= 0 ? `+${found.value}` : String(found.value);
    return [{ label: row.label, value }];
  });
}
