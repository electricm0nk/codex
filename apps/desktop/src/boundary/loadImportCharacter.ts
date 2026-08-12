import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';
import type { CreateCharacterOutcome } from './loadCreateCharacter';

/**
 * Write desktop boundary for importing a character from a JSON file the
 * user picked via the native open dialog.
 *
 * Invokes the `import_character` Tauri command, which reads `filePath`
 * itself, parses it as an `ImportedCharacterFileDto` (a `displayLabel` +
 * `characterInput` — the same shape `exportCharacter` writes), mints a
 * fresh character id, recomputes via the real rules-core engine, and either
 * saves and returns `Saved` or leaves nothing persisted and returns
 * `Blocked` with real diagnostics — mirrors `create_character`'s "never
 * persist an unproven build" invariant. Returns the same tagged
 * `CreateCharacterOutcome` union every other character mutation command
 * returns.
 */

export interface ImportCharacterRequest {
  filePath: string;
  savedAt: string;
}

export async function loadImportCharacter(request: ImportCharacterRequest): Promise<CreateCharacterOutcome> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for importing a character');
  }

  try {
    return await invoke<CreateCharacterOutcome>('import_character', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to import character: ${formatError(cause)}`);
  }
}
