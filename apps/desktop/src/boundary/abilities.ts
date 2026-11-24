import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';
import type { ExplanationDto } from './loadSavedCharacterDetail';

/**
 * Desktop boundary for the Abilities sub-tabs (SD-43 E3.2): the engine lists a sub-tab's options
 * with `qualified` and the unmet reasons, and its `abilities.budget.<subtab>.<pool>.*` rows; add
 * and remove go through the engine's budget and prerequisite refusals, then the same
 * recompute-before-save gate as every other add.
 */

/** A sub-tab id the engine serves: `ability_budget::SubTab`, and the race's alternate racial traits. */
export type AbilitySubTabId = 'traits' | 'class_features' | 'archetype' | 'favored_class' | 'background' | 'misc' | 'racial_abilities';

export interface AbilityOptionDto {
  id: string;
  label: string;
  pool: string;
  category: string | null;
  qualified: boolean;
  unmet: string[];
  /** The record's rule text when the engine carries it (Class Features). */
  description?: string | null;
  /** The held pick this option replaces instead of taking room (Class Features: a bloodline renames the creation choice). */
  replaces?: string | null;
}

export interface AbilityOptionsDto {
  budget: ExplanationDto[];
  options: AbilityOptionDto[];
  /** The option ids the character holds, in saved order. */
  selected: string[];
}

export interface AbilitySelectionRequest {
  characterId: string;
  subtab: AbilitySubTabId;
  /** An option's `id` from `listAbilityOptions`. */
  optionId: string;
  savedAt: string;
}

export async function listAbilityOptions(characterId: string, subtab: AbilitySubTabId): Promise<AbilityOptionsDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing ability options');
  }
  try {
    return await invoke<AbilityOptionsDto>('list_ability_options', { request: { characterId, subtab } });
  } catch (cause: unknown) {
    throw new Error(`Failed to list ability options: ${formatError(cause)}`);
  }
}

export async function addAbilitySelection(request: AbilitySelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for adding an ability');
  }
  try {
    return await invoke<CreateCharacterOutcome>('add_ability_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to add the ability: ${formatError(cause)}`);
  }
}

export async function removeAbilitySelection(request: AbilitySelectionRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for removing an ability');
  }
  try {
    return await invoke<CreateCharacterOutcome>('remove_ability_selection', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to remove the ability: ${formatError(cause)}`);
  }
}
