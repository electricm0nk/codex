import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterSummaryDto } from './loadListSavedCharacters';
import type { CorpusDerivedDto, DiagnosticDto, PilotSnapshotDto } from './loadCreateCharacter';

/**
 * Read-only desktop boundary over a single saved character's detail.
 *
 * Invokes the `load_saved_character` Tauri command, which re-computes the
 * saved build via the real rules-core engine on every load (the receipt is
 * never itself persisted) and returns the summary, snapshot (when
 * `Computed`), and diagnostics verbatim.
 */

export interface LoadSavedCharacterRequest {
  characterId: string;
}

export interface LoadSavedCharacterResponse {
  summary: CharacterSummaryDto;
  snapshot: PilotSnapshotDto | null;
  diagnostics: DiagnosticDto[];
  corpusDerived: CorpusDerivedDto;
  /** The character's full persisted `chosen.selected_feats`, verbatim — not just feats added this session. */
  selectedFeats: string[];
}

export async function loadSavedCharacterDetail(
  request: LoadSavedCharacterRequest
): Promise<LoadSavedCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading a saved character');
  }

  try {
    return await invoke<LoadSavedCharacterResponse>('load_saved_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to load saved character: ${formatError(cause)}`);
  }
}
