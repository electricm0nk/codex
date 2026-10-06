import type {
  SfChoicesDto,
  SfChoicesPreviewDto,
  SfEquipmentOptionDto,
  SfFeatOptionDto,
  SfSpellLevelChoiceDto,
} from '../boundary/starfinderChoices';
import type { SfCreationSlotDto } from '../boundary/starfinderCreation';
import type { DiagnosticDto } from '../boundary/loadCreateCharacter';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import { setSlotPick } from './starfinderCreationModel';
import { starfinderRowLabel } from './starfinderSheetModel';

/**
 * The pure edits of the Starfinder feats, spells known and gear panel (SD-37 E6.5a). The panel
 * holds the player's choices; every list, verdict and number it shows is the engine's preview
 * (`preview_starfinder_choices`, or the creation and level-up previews' `choices`). Nothing here
 * knows a Starfinder rule: a feat's prerequisite, a level's spells-known total, an upgrade that
 * fits an item and every total are the engine's, and its refusals come back as problems.
 */

export const EMPTY_STARFINDER_CHOICES: SfChoicesDto = { feats: [], featPicks: [], spells: [], gear: [] };

/** The rows the totals line prints, in sheet order. */
const TOTALS_LINE_ROWS = ['sf.eac', 'sf.kac', 'sf.credits.remaining', 'sf.bulk'];

export function addFeat(choices: SfChoicesDto, featId: string): SfChoicesDto {
  if (featId === '') {
    return choices;
  }
  return { ...choices, feats: [...choices.feats, featId] };
}

/** Removes the feat at `index`; a feat no longer held drops its own picks. */
export function removeFeatAt(choices: SfChoicesDto, index: number): SfChoicesDto {
  const feats = choices.feats.filter((_, i) => i !== index);
  return { ...choices, feats, featPicks: choices.featPicks.filter((pick) => feats.includes(pick.slotId)) };
}

/** A chosen feat's own pick (Weapon Focus's weapon type), the creation flow's slot edit. */
export function setFeatPick(choices: SfChoicesDto, slot: SfCreationSlotDto, position: number, optionId: string): SfChoicesDto {
  return { ...choices, featPicks: setSlotPick(choices.featPicks, slot, position, optionId) };
}

export function addSpell(choices: SfChoicesDto, classId: string, spellId: string): SfChoicesDto {
  if (spellId === '' || choices.spells.some((s) => s.classId === classId && s.spellId === spellId)) {
    return choices;
  }
  return { ...choices, spells: [...choices.spells, { classId, spellId }] };
}

export function removeSpell(choices: SfChoicesDto, classId: string, spellId: string): SfChoicesDto {
  return { ...choices, spells: choices.spells.filter((s) => !(s.classId === classId && s.spellId === spellId)) };
}

/** The picks of `level`'s class that are on that level's list. */
export function spellPicksAt(choices: SfChoicesDto, level: SfSpellLevelChoiceDto): string[] {
  return choices.spells
    .filter((s) => s.classId === level.classId && level.options.some((o) => o.id === s.spellId))
    .map((s) => s.spellId);
}

export function addGear(choices: SfChoicesDto, itemId: string, equipped: boolean): SfChoicesDto {
  if (itemId === '') {
    return choices;
  }
  return { ...choices, gear: [...choices.gear, { itemId, equipped, modifiers: [] }] };
}

export function removeGearAt(choices: SfChoicesDto, position: number): SfChoicesDto {
  return { ...choices, gear: choices.gear.filter((_, i) => i !== position) };
}

export function setGearEquipped(choices: SfChoicesDto, position: number, equipped: boolean): SfChoicesDto {
  return { ...choices, gear: choices.gear.map((g, i) => (i === position ? { ...g, equipped } : g)) };
}

export function addModifier(choices: SfChoicesDto, position: number, modifierId: string): SfChoicesDto {
  if (modifierId === '') {
    return choices;
  }
  return { ...choices, gear: choices.gear.map((g, i) => (i === position ? { ...g, modifiers: [...g.modifiers, modifierId] } : g)) };
}

export function removeModifierAt(choices: SfChoicesDto, position: number, index: number): SfChoicesDto {
  return {
    ...choices,
    gear: choices.gear.map((g, i) => (i === position ? { ...g, modifiers: g.modifiers.filter((_, m) => m !== index) } : g)),
  };
}

/** A feat option's text: its name, and the engine's verdict when the prerequisite is not met. */
export function featOptionLabel(option: SfFeatOptionDto): string {
  return option.eligible ? option.label : `${option.label} (prerequisite not met)`;
}

/** An equipment option's text: its name and its own printed price. */
export function equipmentOptionLabel(option: SfEquipmentOptionDto): string {
  return option.price === null ? option.label : `${option.label} (${option.price} credits)`;
}

/** The engine's EAC, KAC, credits remaining and bulk, by the sheet's labels: `Energy Armor Class (EAC) 16 · …`. */
export function choicesTotalsLine(totals: readonly ExplanationDto[]): string {
  return TOTALS_LINE_ROWS.flatMap((id) => {
    const row = totals.find((t) => t.id === id);
    return row === undefined ? [] : [`${starfinderRowLabel(id)} ${row.value}`];
  }).join(' · ');
}

export function blockingChoiceProblems(preview: Pick<SfChoicesPreviewDto, 'problems'>): DiagnosticDto[] {
  return preview.problems.filter((problem) => problem.claimBlocking);
}
