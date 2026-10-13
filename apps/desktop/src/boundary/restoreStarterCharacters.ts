import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Desktop boundary over the starter characters (`character_hub.rs`, Brief 3).
 *
 * `restore_starter_characters` rebuilds every starter seed whose character
 * folder is missing (a seed whose folder exists is never touched) and returns
 * the labels it restored plus the error of every seed that did not compute.
 * `list_starter_seed_errors` returns the errors of the latest seeding pass
 * (the launch, or the last Restore), so a failed seed is visible in the app,
 * not only on stderr.
 */

export interface RestoreStarterCharactersResponse {
  restored: string[];
  errors: string[];
}

export async function restoreStarterCharacters(): Promise<RestoreStarterCharactersResponse> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for restoring starter characters');
  }

  try {
    return await invoke<RestoreStarterCharactersResponse>('restore_starter_characters');
  } catch (cause: unknown) {
    throw new Error(`Failed to restore starter characters: ${formatError(cause)}`);
  }
}

export async function listStarterSeedErrors(): Promise<string[]> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for reading starter seed errors');
  }

  try {
    return await invoke<string[]>('list_starter_seed_errors');
  } catch (cause: unknown) {
    throw new Error(`Failed to read starter seed errors: ${formatError(cause)}`);
  }
}
