import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CharacterCustom } from '../characterHub/customModel';

/**
 * Custom item exchange (SD-43 `decisions.md §21`): one JSON file holds many custom equipment
 * records and names its game system. `export_custom_items` writes the picked items to a path (its
 * folder made if missing); `import_custom_items` adds a file's items to a character, editing an item
 * of the same name instead of adding a second, and refuses a file from another game system.
 * `custom_items_folder` is a campaign's `Custom Items/` folder, made if an older campaign lacks it.
 */

export interface ImportedCustomItems {
  custom: CharacterCustom;
  added: number;
  edited: number;
}

function requireRuntime(what: string) {
  if (!hasTauriRuntime()) {
    throw new Error(`Tauri runtime not available for ${what}`);
  }
}

export async function exportCustomItems(request: { characterId: string; itemIds: string[]; path: string }): Promise<number> {
  requireRuntime('exporting custom items');
  try {
    return await invoke<number>('export_custom_items', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to export the custom items: ${formatError(cause)}`);
  }
}

export async function importCustomItems(request: { characterId: string; path: string; savedAt: string }): Promise<ImportedCustomItems> {
  requireRuntime('importing custom items');
  try {
    return await invoke<ImportedCustomItems>('import_custom_items', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to import the custom items: ${formatError(cause)}`);
  }
}

export async function customItemsFolder(campaignDir: string): Promise<string> {
  requireRuntime("finding the campaign's Custom Items folder");
  try {
    return await invoke<string>('custom_items_folder', { campaignDir });
  } catch (cause: unknown) {
    throw new Error(`Failed to open the campaign's Custom Items folder: ${formatError(cause)}`);
  }
}
