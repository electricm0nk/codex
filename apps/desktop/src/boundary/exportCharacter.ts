import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Write desktop boundary for exporting a saved character to a file the user
 * picked via the native save dialog.
 *
 * Invokes the `export_character` Tauri command, which loads the character's
 * real saved build and serializes it into the exact `{displayLabel,
 * characterInput}` shape `import_character` (and `loadImportCharacter`)
 * expects — unlike the older `exportCharacterJson` boundary, which wrote
 * whatever string the caller already built, this command builds the export
 * payload itself from the on-disk envelope, so the file it writes is always
 * re-importable.
 */

export interface ExportCharacterRequest {
  characterId: string;
  filePath: string;
}

export async function exportCharacter(request: ExportCharacterRequest): Promise<void> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for exporting a character');
  }

  try {
    await invoke('export_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to export character: ${formatError(cause)}`);
  }
}
