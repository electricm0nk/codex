import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { DiagnosticDto } from './loadCreateCharacter';
import type { SfCreateResponse, SfCreationSlotDto, SfOptionDto, SfPickDto } from './starfinderCreation';
import type { SfChoicesDto, SfChoicesPreviewDto } from './starfinderChoices';

/**
 * Desktop boundary over the Starfinder 1e level-up (SD-37 E6.5):
 * `preview_starfinder_level_up` and `level_up_starfinder_character`
 * (`apps/desktop/src-tauri/src/sf_level_up.rs`). The preview is the engine's
 * answer for the dialog's current state -- the classes, the ability increase,
 * each skill's ranks and cap, the picks the level opens and every sheet total
 * the level changes -- so the dialog carries no Starfinder table of its own.
 * Mirrors the Rust DTOs field for field (`serde(rename_all = "camelCase")`).
 */

export interface SfSkillRanksDto {
  skill: string;
  ranks: number;
}

export interface SfLevelUpRequest {
  characterId: string;
  savedAt: string;
  classId: string | null;
  /** `STR` .. `CHA`, for a new class whose key ability is a choice. */
  keyAbility: string | null;
  /** The four scores the increase raises, when the new level has one. */
  abilityIncreases: string[];
  picks: SfPickDto[];
  skillRanks: SfSkillRanksDto[];
  /** Feats, spells known and gear at the new level (SD-37 E6.5a); `null` keeps the character's own. */
  choices: SfChoicesDto | null;
}

export interface SfLevelUpClassOptionDto {
  id: string;
  label: string;
  /** The class's level now; 0 for a class the character does not hold. */
  currentLevel: number;
  keyAbilityOptions: SfOptionDto[];
}

export interface SfLevelUpAbilityDto {
  ability: string;
  label: string;
  score: number;
  newScore: number;
  increase: string | null;
}

export interface SfLevelUpSkillDto {
  skill: string;
  label: string;
  ranks: number;
  added: number;
  maxRanks: number;
}

export interface SfLevelUpChangeDto {
  id: string;
  before: number | null;
  after: number | null;
}

export interface SfLevelUpPreviewDto {
  characterLevel: number;
  classes: SfLevelUpClassOptionDto[];
  levelLine: string | null;
  classLines: string[];
  ruleLines: string[];
  increaseDue: boolean;
  /** How many different scores the increase raises (the engine's count); 0 with no increase. */
  increaseScores: number;
  abilities: SfLevelUpAbilityDto[];
  slots: SfCreationSlotDto[];
  chosenOnTheSheet: string[];
  skills: SfLevelUpSkillDto[];
  skillRule: string;
  changes: SfLevelUpChangeDto[];
  problems: DiagnosticDto[];
  /** The feats, spells known and gear at the new level, once the level composes. */
  choices: SfChoicesPreviewDto | null;
}

export async function previewStarfinderLevelUp(request: SfLevelUpRequest): Promise<SfLevelUpPreviewDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the Starfinder level-up');
  }
  try {
    return await invoke<SfLevelUpPreviewDto>('preview_starfinder_level_up', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to preview the Starfinder level-up: ${formatError(cause)}`);
  }
}

export async function levelUpStarfinderCharacter(request: SfLevelUpRequest): Promise<SfCreateResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for the Starfinder level-up');
  }
  try {
    return await invoke<SfCreateResponse>('level_up_starfinder_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to level up the Starfinder character: ${formatError(cause)}`);
  }
}
