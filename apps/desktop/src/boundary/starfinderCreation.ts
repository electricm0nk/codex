import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { DiagnosticDto } from './loadCreateCharacter';
import type { ExplanationDto } from './loadSavedCharacterDetail';
import type { SfChoicesDto, SfChoicesPreviewDto } from './starfinderChoices';

/**
 * Desktop boundary over Starfinder 1e character creation (SD-37 E6.2):
 * `preview_starfinder_character` and `create_starfinder_character`
 * (`apps/desktop/src-tauri/src/sf_creation.rs`). The preview is the engine's
 * answer for the form's current state -- the race/theme/class lists, the picks
 * the held set asks for, the point-buy rules and every ability score with its
 * terms -- so the form carries no Starfinder table of its own. Mirrors the Rust
 * DTOs field for field (`serde(rename_all = "camelCase")`).
 */

export interface SfPointBuyDto {
  strength: number;
  dexterity: number;
  constitution: number;
  intelligence: number;
  wisdom: number;
  charisma: number;
}

export interface SfPickDto {
  slotId: string;
  optionId: string;
}

export interface SfCreationRequest {
  characterId: string;
  displayLabel: string;
  savedAt: string;
  raceId: string | null;
  themeId: string | null;
  classId: string | null;
  /** `STR` .. `CHA`, for a class whose key ability is a choice. */
  keyAbility: string | null;
  pointBuy: SfPointBuyDto;
  picks: SfPickDto[];
  /** Feats, spells known and gear chosen at creation (SD-37 E6.5a). */
  choices: SfChoicesDto | null;
}

export interface SfOptionDto {
  id: string;
  label: string;
}

export interface SfClassOptionDto {
  id: string;
  label: string;
  keyAbilityOptions: SfOptionDto[];
}

export interface SfPointBuyRulesDto {
  budget: number;
  maxScoreAtCreation: number;
  source: string;
}

export interface SfCreationSlotDto {
  slotId: string;
  label: string;
  /** `option` (a record), `skill` or `ability`. */
  kind: string;
  count: number;
  repeatable: boolean;
  options: SfOptionDto[];
  /** The package's "No ..." option, preselected; `null` when there is none. */
  defaultOptionId: string | null;
  /** Whether the creation scores depend on it (only these block creation while open). */
  required: boolean;
}

export interface SfScoreTermDto {
  label: string;
  value: number;
}

export interface SfAbilityScoreDto {
  ability: string;
  label: string;
  score: number;
  modifier: number;
  terms: SfScoreTermDto[];
}

export interface SfCreationPreviewDto {
  races: SfOptionDto[];
  themes: SfOptionDto[];
  classes: SfClassOptionDto[];
  pointBuyRules: SfPointBuyRulesDto;
  slots: SfCreationSlotDto[];
  chosenOnTheSheet: string[];
  abilityScores: SfAbilityScoreDto[];
  pointsSpent: number;
  pointsUnspent: number;
  problems: DiagnosticDto[];
  /** The 1st-level character's feats, spells known and gear, once its scores compute. */
  choices: SfChoicesPreviewDto | null;
}

export type SfCreateResponse =
  | { kind: 'Saved'; summary: CharacterSummaryDto; explanations: ExplanationDto[] }
  | { kind: 'Blocked'; diagnostics: DiagnosticDto[] };

export async function previewStarfinderCharacter(request: SfCreationRequest): Promise<SfCreationPreviewDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for Starfinder character creation');
  }
  try {
    return await invoke<SfCreationPreviewDto>('preview_starfinder_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Starfinder character: ${formatError(cause)}`);
  }
}

export async function createStarfinderCharacter(request: SfCreationRequest): Promise<SfCreateResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for Starfinder character creation');
  }
  try {
    return await invoke<SfCreateResponse>('create_starfinder_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to create the Starfinder character: ${formatError(cause)}`);
  }
}
