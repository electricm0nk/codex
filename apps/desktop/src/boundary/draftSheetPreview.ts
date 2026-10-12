import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterRequest, EncumbranceDto } from './loadCreateCharacter';
import type { ExplanationDto } from './loadSavedCharacterDetail';

/**
 * The sheet numbers the Create screen previews for a character that is not saved yet
 * (`draft_sheet_preview`): the engine's `initiative.total` row and the draft's carried load, from
 * the same engine run the sheet reads once the character is saved. Mirrors `DraftSheetPreviewDto`
 * in `character_hub.rs`.
 */
export interface DraftSheetPreviewDto {
  initiative: ExplanationDto;
  encumbrance: EncumbranceDto;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function requireField(owner: Record<string, unknown>, path: string, key: string, type: 'string' | 'number' | 'array'): void {
  const value = owner[key];
  const ok = type === 'array' ? Array.isArray(value) : typeof value === type;
  if (!ok) {
    throw new Error(`draft sheet preview: ${path}.${key} is missing or not a ${type}`);
  }
}

/** Checks the wire payload has every field the Create boxes render; a malformed payload is refused, never shown. */
export function readDraftSheetPreview(raw: unknown): DraftSheetPreviewDto {
  if (!isRecord(raw)) {
    throw new Error('draft sheet preview: the payload is not an object');
  }
  const { initiative, encumbrance } = raw;
  if (!isRecord(initiative)) {
    throw new Error('draft sheet preview: initiative is missing');
  }
  if (!isRecord(encumbrance)) {
    throw new Error('draft sheet preview: encumbrance is missing');
  }
  requireField(initiative, 'initiative', 'id', 'string');
  requireField(initiative, 'initiative', 'value', 'number');
  requireField(initiative, 'initiative', 'detail', 'string');
  for (const key of ['totalCarriedWeightLbs', 'totalCarriedCostGp', 'lightMaxLbs', 'mediumMaxLbs', 'heavyMaxLbs', 'loadArmorCheckPenalty']) {
    requireField(encumbrance, 'encumbrance', key, 'number');
  }
  requireField(encumbrance, 'encumbrance', 'level', 'string');
  requireField(encumbrance, 'encumbrance', 'perItem', 'array');
  requireField(encumbrance, 'encumbrance', 'unresolvedItemIds', 'array');
  return raw as unknown as DraftSheetPreviewDto;
}

export async function loadDraftSheetPreview(draft: CreateCharacterRequest): Promise<DraftSheetPreviewDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading the draft sheet preview');
  }
  let raw: unknown;
  try {
    raw = await invoke<unknown>('draft_sheet_preview', { request: draft });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the draft character's sheet preview: ${formatError(cause)}`);
  }
  return readDraftSheetPreview(raw);
}
