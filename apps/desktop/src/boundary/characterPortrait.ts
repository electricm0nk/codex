import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only-ish desktop boundary over a saved character's portrait image.
 * The image itself is stored as `portrait.png` alongside the character's
 * existing envelope/input files (see `save_character_portrait` in
 * character_hub.rs) and always crosses the IPC boundary as a full
 * `data:image/png;base64,...` URL — no asset-protocol/file-path access from
 * the frontend, consistent with every other command in this app.
 */

export async function loadCharacterPortrait(characterId: string): Promise<string | null> {
  if (!hasTauriRuntime()) {
    return null;
  }

  try {
    return await invoke<string | null>('load_character_portrait', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load character portrait: ${formatError(cause)}`);
  }
}

/** `imageDataUrl` must be a `data:image/png;base64,...` URL — callers crop/resize client-side first. */
export async function saveCharacterPortrait(characterId: string, imageDataUrl: string): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for saving a character portrait');
  }

  const base64Marker = ';base64,';
  const markerIndex = imageDataUrl.indexOf(base64Marker);
  if (markerIndex === -1) {
    throw new Error('Portrait image must be a base64 data URL');
  }
  const imageBase64 = imageDataUrl.slice(markerIndex + base64Marker.length);

  try {
    await invoke('save_character_portrait', { request: { characterId, imageBase64 } });
  } catch (cause: unknown) {
    throw new Error(`Failed to save character portrait: ${formatError(cause)}`);
  }
}

export async function deleteCharacterPortrait(characterId: string): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for deleting a character portrait');
  }

  try {
    await invoke('delete_character_portrait', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to delete character portrait: ${formatError(cause)}`);
  }
}
