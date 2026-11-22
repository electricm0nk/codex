import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Write desktop boundary for deleting a saved character.
 *
 * Invokes the `delete_character` Tauri command, which removes the
 * character's on-disk directory tree. Unlike every other character-hub
 * command, a failure is carried inside the response payload
 * (`{ ok: false, error }`) rather than raised as a rejected Tauri IPC call
 * — see `DeleteCharacterResponse`'s own doc comment in `character_hub.rs`
 * — so this only throws for a genuine IPC-layer failure, never for an
 * ordinary "could not delete" outcome.
 */

export interface DeleteCharacterResponse {
  ok: boolean;
  error?: string;
}

export async function loadDeleteCharacter(characterId: string): Promise<DeleteCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for deleting a character');
  }

  try {
    return await invoke<DeleteCharacterResponse>('delete_character', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to delete character: ${formatError(cause)}`);
  }
}
