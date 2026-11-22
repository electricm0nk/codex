import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { DiagnosticDto } from './loadCreateCharacter';
import type { ExplanationDto } from './loadSavedCharacterDetail';
import type { SfCreateResponse, SfCreationSlotDto, SfOptionDto, SfPickDto } from './starfinderCreation';

/**
 * Desktop boundary over a Starfinder 1e character's feats, spells known and gear (SD-37 E6.5a):
 * `preview_starfinder_choices`, `save_starfinder_choices` and
 * `list_starfinder_equipment_options` (`apps/desktop/src-tauri/src/sf_choices.rs`). The same
 * `SfChoicesDto` rides on the creation and level-up requests. Every list, prerequisite verdict,
 * spells-known total and sheet total is the engine's, so no surface carries a Starfinder table
 * of its own. Mirrors the Rust DTOs field for field (`serde(rename_all = "camelCase")`).
 */

export interface SfSpellPickDto {
  classId: string;
  spellId: string;
}

export interface SfGearDto {
  itemId: string;
  /** Worn armour, a wielded weapon, an installed augmentation; otherwise carried. */
  equipped: boolean;
  /** Upgrades, fusions and special materials applied to this item. */
  modifiers: string[];
}

export interface SfChoicesDto {
  feats: string[];
  featPicks: SfPickDto[];
  spells: SfSpellPickDto[];
  gear: SfGearDto[];
}

export interface SfFeatOptionDto {
  id: string;
  label: string;
  /** The engine's verdict on the feat's prerequisite for this character. */
  eligible: boolean;
  repeatable: boolean;
}

export interface SfSpellLevelChoiceDto {
  classId: string;
  classLabel: string;
  level: number;
  /** The engine's spells-known total at this level. */
  known: number;
  knownTerms: string[];
  chosen: SfOptionDto[];
  options: SfOptionDto[];
}

export interface SfGearLineDto {
  position: number;
  itemId: string;
  label: string;
  equipped: boolean;
  modifiers: SfOptionDto[];
  modifierOptions: SfOptionDto[];
}

export interface SfEquipmentOptionDto {
  id: string;
  label: string;
  /** The record's own `Price` row (credits), or `null`. */
  price: string | null;
}

export interface SfChoicesPreviewDto {
  chosen: SfChoicesDto;
  featRules: string[];
  feats: SfOptionDto[];
  featOptions: SfFeatOptionDto[];
  featSlots: SfCreationSlotDto[];
  spellLevels: SfSpellLevelChoiceDto[];
  gear: SfGearLineDto[];
  totals: ExplanationDto[];
  problems: DiagnosticDto[];
}

export interface SfChoicesRequest {
  characterId: string;
  savedAt: string;
  /** `null`: the saved character's own choices. */
  choices: SfChoicesDto | null;
}

export async function previewStarfinderChoices(request: SfChoicesRequest): Promise<SfChoicesPreviewDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for Starfinder feats, spells and gear');
  }
  try {
    return await invoke<SfChoicesPreviewDto>('preview_starfinder_choices', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Starfinder feats, spells and gear: ${formatError(cause)}`);
  }
}

export async function saveStarfinderChoices(request: SfChoicesRequest): Promise<SfCreateResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for Starfinder feats, spells and gear');
  }
  try {
    return await invoke<SfCreateResponse>('save_starfinder_choices', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to save the Starfinder feats, spells and gear: ${formatError(cause)}`);
  }
}

export async function listStarfinderEquipmentOptions(): Promise<SfEquipmentOptionDto[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the Starfinder equipment list');
  }
  try {
    return await invoke<SfEquipmentOptionDto[]>('list_starfinder_equipment_options');
  } catch (cause: unknown) {
    throw new Error(`Failed to list the Starfinder equipment: ${formatError(cause)}`);
  }
}
