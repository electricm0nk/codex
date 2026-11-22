import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterCustom } from '../characterHub/customModel';

/**
 * Read/write boundary over a character's Custom data (`custom.json`): GM grants and house-rule
 * records. Saving applies ability grants to the saved ability scores (a new revision), so callers
 * reload the character afterwards.
 */
export async function loadCharacterCustom(characterId: string): Promise<CharacterCustom> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for loading custom data');
  }
  try {
    return await invoke<CharacterCustom>('load_character_custom', { request: { characterId } });
  } catch (cause: unknown) {
    throw new Error(`Failed to load this character's custom data: ${formatError(cause)}`);
  }
}

export async function saveCharacterCustom(characterId: string, custom: CharacterCustom, savedAt: string): Promise<CharacterCustom> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for saving custom data');
  }
  try {
    return await invoke<CharacterCustom>('save_character_custom', { request: { characterId, custom, savedAt } });
  } catch (cause: unknown) {
    throw new Error(`Failed to save this character's custom data: ${formatError(cause)}`);
  }
}
