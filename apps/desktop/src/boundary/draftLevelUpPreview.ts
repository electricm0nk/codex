import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterRequest } from './loadCreateCharacter';

/**
 * What class level `fromLevel + 1` of a class grants a character that is not saved yet
 * (`draft_level_up_preview`): the engine's automatic grant names, verbatim, from the same
 * `compute_level_up_grants_for_class` call `preview_level_up` makes for a saved character.
 * Mirrors `DraftLevelUpPreviewDto` in `character_hub.rs`. An empty list is a real answer.
 */
export interface DraftLevelUpPreviewDto {
  fromLevel: number;
  toLevel: number;
  automaticFeatures: string[];
}

/** Checks the wire payload; a malformed payload is refused, never shown. */
export function readDraftLevelUpPreview(raw: unknown): DraftLevelUpPreviewDto {
  if (typeof raw !== 'object' || raw === null) {
    throw new Error('draft level-up preview: the payload is not an object');
  }
  const { fromLevel, toLevel, automaticFeatures } = raw as Record<string, unknown>;
  if (typeof fromLevel !== 'number' || typeof toLevel !== 'number') {
    throw new Error('draft level-up preview: fromLevel or toLevel is missing or not a number');
  }
  if (!Array.isArray(automaticFeatures) || !automaticFeatures.every((name) => typeof name === 'string')) {
    throw new Error('draft level-up preview: automaticFeatures is missing or not a list of names');
  }
  return { fromLevel, toLevel, automaticFeatures: automaticFeatures as string[] };
}

export async function loadDraftLevelUpPreview(
  draft: CreateCharacterRequest,
  classId: string,
  fromLevel: number,
): Promise<DraftLevelUpPreviewDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the draft level-up preview');
  }
  let raw: unknown;
  try {
    raw = await invoke<unknown>('draft_level_up_preview', { request: draft, classId, fromLevel });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the draft level-up preview: ${formatError(cause)}`);
  }
  return readDraftLevelUpPreview(raw);
}
