import { save } from '@tauri-apps/plugin-dialog';
import { exportCharacter, type ExportCharacterRequest } from '../boundary/exportCharacter';
import { hasTauriRuntime } from '../boundary/runtime';

/**
 * The character-export flow behind the sheet's ☰ → Export item (v0.8 F-8),
 * factored so the decision logic is testable without a native dialog.
 * Mirrors `LoadCharacterScreen.tsx`'s `handleExport`: the file's contents
 * are built server-side by `export_character` from the on-disk build, so
 * what this exports is always something the Load screen can re-import.
 */

export interface CharacterExportDeps {
  hasRuntime: () => boolean;
  /** Native save dialog; resolves `null` when the player dismisses it. */
  save: (defaultFileName: string) => Promise<string | null>;
  exportCharacter: (request: ExportCharacterRequest) => Promise<void>;
}

export type CharacterExportOutcome =
  | { kind: 'Exported'; message: string }
  | { kind: 'Cancelled' }
  | { kind: 'Failed'; message: string };

/** Filesystem-safe default name, same rule the Load screen uses. */
export function exportFileName(displayLabel: string): string {
  const safeName = displayLabel.replace(/[^a-z0-9-_ ]/gi, '').trim() || 'character';
  return `${safeName}.json`;
}

export async function runCharacterExport(
  target: { characterId: string; displayLabel: string },
  deps: CharacterExportDeps,
): Promise<CharacterExportOutcome> {
  // Without the Tauri runtime (browser preview) there is no native save
  // dialog and no on-disk build for the command to read.
  if (!deps.hasRuntime()) {
    return { kind: 'Failed', message: 'Exporting requires the desktop runtime.' };
  }
  try {
    const filePath = await deps.save(exportFileName(target.displayLabel));
    if (!filePath) {
      return { kind: 'Cancelled' };
    }
    await deps.exportCharacter({ characterId: target.characterId, filePath });
    return { kind: 'Exported', message: `Exported ${target.displayLabel} to ${filePath}.` };
  } catch (cause: unknown) {
    return { kind: 'Failed', message: cause instanceof Error ? cause.message : 'Could not export the character.' };
  }
}

/** Production wiring: real dialog, real runtime check, real boundary call. */
export const DESKTOP_EXPORT_DEPS: CharacterExportDeps = {
  hasRuntime: hasTauriRuntime,
  save: (defaultFileName) =>
    save({ defaultPath: defaultFileName, filters: [{ name: 'Character JSON', extensions: ['json'] }] }),
  exportCharacter,
};
