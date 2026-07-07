import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over the saved-character listing.
 *
 * Invokes the `list_saved_characters` Tauri command and returns the summary
 * list verbatim; performs no filtering, sorting, or presentation mapping
 * (that lives in `characterHub/buildCharacterHubListSurface.ts`).
 */

export interface CharacterSummaryDto {
  characterId: string;
  displayLabel: string;
  gameSystem: string;
  schemaVersion: number;
  savedAt: string;
  raceId: string;
  classSummary: string;
}

export interface ListSavedCharactersResponse {
  characters: CharacterSummaryDto[];
  unreadableCount: number;
}

export async function loadListSavedCharacters(): Promise<ListSavedCharactersResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for listing saved characters');
  }

  try {
    return await invoke<ListSavedCharactersResponse>('list_saved_characters');
  } catch (cause: unknown) {
    throw new Error(`Failed to list saved characters: ${formatError(cause)}`);
  }
}
