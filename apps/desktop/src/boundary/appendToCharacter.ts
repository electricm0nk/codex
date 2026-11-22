import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the `append_to_character` Tauri command
 * (`apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`): appends equipment to a saved
 * character through the active rule system's adapter (`ruleSystemId`: `"pf1"` -> `Pf1Adapter`,
 * `"starfinder-1e"` -> `StarfinderAdapter`). The adapter checks every item against its own
 * package, recomputes with the items and re-saves, or saves nothing and returns the error (an
 * unknown item, a loadout over the character's credits). Mirrors the Rust DTOs.
 */

export type ActiveStateDto = 'EquippedActive' | 'SelectedInactive' | 'Absent';

export interface ItemToAppendDto {
  itemId: string;
  activeState: ActiveStateDto;
}

export interface AppendToCharacterRequest {
  characterId: string;
  itemsToAppend: ItemToAppendDto[];
  savedAt: string;
  ruleSystemId: string;
}

export interface AppendToCharacterResponse {
  success: boolean;
  /** The re-saved character's projection; this boundary's callers re-read the character instead. */
  character: unknown;
  error: string | null;
}

export async function appendToCharacter(request: AppendToCharacterRequest): Promise<AppendToCharacterResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for appending equipment');
  }
  try {
    return await invoke<AppendToCharacterResponse>('append_to_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to append equipment: ${formatError(cause)}`);
  }
}
