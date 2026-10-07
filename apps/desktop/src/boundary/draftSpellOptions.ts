import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterRequest } from './loadCreateCharacter';
import type { ExplanationDto } from './loadSavedCharacterDetail';

/**
 * What the Create screen's spells dialog needs for a character that is not saved yet
 * (`draft_spell_options`): the engine's spells-per-day rows for the draft, and the spell names the
 * race grants as spell-like abilities.
 */
export interface DraftSpellOptionsDto {
  perDay: ExplanationDto[];
  innate: string[];
}

export async function loadDraftSpellOptions(draft: CreateCharacterRequest): Promise<DraftSpellOptionsDto> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading spell options');
  }
  try {
    return await invoke<DraftSpellOptionsDto>('draft_spell_options', { request: draft });
  } catch (cause: unknown) {
    throw new Error(`Failed to load the draft character's spell options: ${formatError(cause)}`);
  }
}
