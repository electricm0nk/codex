import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/** Writes `contents` to `filePath`, a destination the caller already picked via the dialog plugin's `save()`. */
export async function exportCharacterJson(filePath: string, contents: string): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for exporting a character');
  }

  try {
    await invoke('export_character_json', { request: { filePath, contents } });
  } catch (cause: unknown) {
    throw new Error(`Failed to export character: ${formatError(cause)}`);
  }
}
